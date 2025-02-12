use std::{
    fmt::{self, Display},
    str::FromStr,
};

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, PartialOrd)]
pub struct UpdateDateTime {
    datetime: i64,
}

impl UpdateDateTime {
    pub fn from_epoch(epoch: i64) -> Self {
        Self { datetime: epoch }
    }
}

impl FromStr for UpdateDateTime {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_epoch(s.parse()?))
    }
}

impl Display for UpdateDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let utc_time = DateTime::<Utc>::from_timestamp(self.datetime, 0).ok_or(fmt::Error)?;

        write!(f, "{}", utc_time)
    }
}
