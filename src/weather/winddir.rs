use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum Winddir16Point {
    N,
    NNE,
    NE,
    ENE,
    E,
    ESE,
    SE,
    SSE,
    S,
    SSW,
    SW,
    WSW,
    W,
    WNW,
    NW,
    NNW,
}

impl Winddir16Point {
    pub(crate) fn into_symbol(&self) -> String {
        let arrow: char = match self {
            Winddir16Point::N => '↑',
            Winddir16Point::NNE => '↗',
            Winddir16Point::NE => '↗',
            Winddir16Point::ENE => '↗',
            Winddir16Point::E => '→',
            Winddir16Point::ESE => '↘',
            Winddir16Point::SE => '↘',
            Winddir16Point::SSE => '↘',
            Winddir16Point::S => '↓',
            Winddir16Point::SSW => '↙',
            Winddir16Point::SW => '↙',
            Winddir16Point::WSW => '↙',
            Winddir16Point::W => '←',
            Winddir16Point::WNW => '↖',
            Winddir16Point::NW => '↖',
            Winddir16Point::NNW => '↖',
        };
        format!("{}", arrow)
    }
    pub(crate) fn into_text(&self) -> String {
        format!("{:?}", self)
    }
}
