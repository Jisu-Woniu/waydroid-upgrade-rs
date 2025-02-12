//! Helper for working with Waydroid.
//!
//! Currently contains the config loader.

mod config;
pub use config::{load as load_config, PREINSTALLED_IMAGES_PATHS};

mod deserialize;
pub use deserialize::deserialize_max;

mod datetime;
pub use datetime::UpdateDateTime;
