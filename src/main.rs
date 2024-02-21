use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand, ValueEnum};
use jq_rs;
use std::fs;
use std::io::{Read, Write};

pub(crate) mod weather;
const TEMP: &str = "°C";
const SPEED: &str = "km/h";
const CACHE_DIR: &str = ".cache";
const CACHE_FILE: &str = "wttr.json";
const REFRESH_RATE: u64 = 3600;

#[derive(Parser)]
#[command(version="1.0", about="wttr.in querier", long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long, default_value_t = false)]
    no_cache: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Current temperature in °C
    Temperature,
    /// Current felt temperature in °C
    FeelTemperature,
    /// Description of current weather (text or emoji, requires Nerd Font)
    Description {
        #[arg(value_enum, default_value_t = DescriptionType::Text)]
        description_type: DescriptionType,
    },
    /// Wind speed in km/h
    WindSpeed,
    /// Uses a sixteen direction system (text or symbol, requires Nerd Font)
    WindDirection {
        #[arg(value_enum, default_value_t = DescriptionType::Text)]
        description_type: DescriptionType,
    },
    /// Min temperature in °C for today
    MinTemperature,
    /// Max temperature in °C for today
    MaxTemperature,
    /// Place queried
    Area,
    /// Full weather data
    Full,
}

#[derive(ValueEnum, Clone)]
enum DescriptionType {
    /// Description as text
    Text,
    /// Description as Emoji / Symbol (requires Nerd Font)
    Emoji,
}

fn new_request() -> Result<String> {
    let mut res = reqwest::blocking::get("https://wttr.in/?format=j1")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    let mut program = match jq_rs::compile(" . | { area: .nearest_area[0].areaName[0].value, temp: (.current_condition[0].temp_C)|tonumber, sens: (.current_condition[0].FeelsLikeC)|tonumber, max: (.weather[0].maxtempC)|tonumber, min: (.weather[0].mintempC)|tonumber, code: (.current_condition[0].weatherCode)|tonumber, winddir16Point: .current_condition[0].winddir16Point, windspeed: (.current_condition[0].windspeedKmph)|tonumber } ") {
        Ok(v) => v,
        Err(e) => return Err(anyhow!("Jq Error: {}", e)),

    };
    match program.run(&body) {
        Ok(v) => Ok(v),
        Err(e) => Err(anyhow!("Jq Error: {}", e)),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let home_dir = match home::home_dir() {
        Some(d) => d,
        None => return Err(anyhow!("Unable to find home directory")),
    };

    let body: String;

    if !cli.no_cache {
        let cache_file = home_dir.join(CACHE_DIR).join(CACHE_FILE);

        body = match fs::metadata(&cache_file) {
            Ok(f) => {
                if f.modified()?.elapsed()?.as_secs() > REFRESH_RATE {
                    fs::remove_file(&cache_file)?;
                    let a = new_request()?;
                    let mut output = fs::File::create(cache_file)?;
                    write!(output, "{}", &a)?;
                    a
                } else {
                    fs::read_to_string(cache_file)?
                }
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    let a = new_request()?;
                    let mut output = fs::File::create(cache_file)?;
                    write!(output, "{}", &a)?;
                    a
                }
                _ => todo!(),
            },
        };
    } else {
        body = new_request()?;
    }

    let response = serde_json::from_str::<crate::weather::Response>(&body)?;
    match &cli.command {
        Commands::Area => println!("{}", response.area()),
        Commands::Temperature => println!("{}{}", response.temp(), TEMP),
        Commands::MaxTemperature => println!("{}{}", response.max(), TEMP),
        Commands::MinTemperature => println!("{}{}", response.min(), TEMP),
        Commands::FeelTemperature => println!("{}{}", response.sens(), TEMP),
        Commands::WindSpeed => println!("{}{}", response.windspeed(), SPEED),
        Commands::Description { description_type } => {
            let value = match description_type {
                DescriptionType::Text => response.code().into_text(),
                DescriptionType::Emoji => response.code().into_symbol(),
            };
            println!("{}", value)
        }
        Commands::WindDirection { description_type } => {
            let value = match description_type {
                DescriptionType::Text => response.winddir16_point().into_text(),
                DescriptionType::Emoji => response.winddir16_point().into_symbol(),
            };
            println!("{}", value)
        }
        Commands::Full => {
            println!(
                "{}: {} {}{} ({}{}) | {} {}{} | max:{}{} | min:{}{}",
                response.area(),
                response.code().into_symbol(),
                response.temp(),
                TEMP,
                response.sens(),
                TEMP,
                response.winddir16_point().into_symbol(),
                response.windspeed(),
                SPEED,
                response.max(),
                TEMP,
                response.min(),
                TEMP
            )
        }
    }
    Ok(())
}
