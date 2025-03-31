use std::{
    fmt::{self, Display},
    num::ParseIntError,
    str::FromStr,
};

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, PartialOrd)]
pub struct UpdateDatetime {
    datetime: i64,
}

impl UpdateDatetime {
    pub fn from_epoch(epoch: i64) -> Self {
        Self { datetime: epoch }
    }
}

impl FromStr for UpdateDatetime {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_epoch(s.parse()?))
    }
}

impl Display for UpdateDatetime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let utc_time = DateTime::<Utc>::from_timestamp(self.datetime, 0).ok_or(fmt::Error)?;

        write!(f, "{}", utc_time)
    }
}
