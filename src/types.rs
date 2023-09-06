use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TripTime {
    pub hour: u8,
    pub minute: u8
}

#[derive(Debug)]
pub enum TimeError {
    MissingSeparator,
    InvalidComponent
}

impl std::error::Error for TimeError {}

impl std::fmt::Display for TimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::MissingSeparator => "Missing component separator (':')",
            Self::InvalidComponent => "Invalid component (cannot be parsed as a number)"
        })
    }
}

impl TryFrom<String> for TripTime {
    type Error = TimeError;

    fn try_from(val: String) -> Result<Self, Self::Error> {
        let (h, m) = val
            .split_once(':')
            .ok_or(Self::Error::MissingSeparator)?;

        let hour: u8 = h.parse().map_err(|_| Self::Error::InvalidComponent)?;
        let minute: u8 = m.parse().map_err(|_| Self::Error::InvalidComponent)?;

        Ok(Self { hour, minute })
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Trip {
    pub headsign: String,
    pub time: TripTime,
    pub track: u8
}
