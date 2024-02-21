use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub(crate) mod weathercode;
pub(crate) mod winddir;
use crate::weather::weathercode::WeatherCode;
use crate::weather::winddir::Winddir16Point;

#[derive(Debug, Deserialize, Serialize, Getters)]
pub(crate) struct Response {
    area: String,
    temp: i32,
    sens: i32,
    max: i32,
    min: i32,
    code: WeatherCode,
    #[serde(alias = "winddir16Point")]
    winddir16_point: Winddir16Point,
    windspeed: u32,
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.winddir16_point.into_symbol(),
            self.code.into_symbol()
        )
    }
}
