//! Helper for working with Waydroid.
//!
//! Currently contains the config loader.

mod config;
pub use config::{PREINSTALLED_IMAGES_PATHS, load as load_config};

mod deserialize;
pub use deserialize::deserialize_max;

mod datetime;
pub use datetime::UpdateDatetime;
