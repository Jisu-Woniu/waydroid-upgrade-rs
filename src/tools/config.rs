//! Waydroid configuration loader.
//!
//! Derived from https://github.com/waydroid/waydroid/tree/7b31f7188a382ead687d291e5a168895efcc6747/tools/config,
//! and removed unused imports and dependencies.

use ini::{Ini, Properties};
use log::{debug, warn};

const CONFIG_KEYS: &[&str] = &["images_path", "system_datetime", "vendor_datetime"];
pub const PREINSTALLED_IMAGES_PATHS: &[&str] = &[
    "/etc/waydroid-extra/images",
    "/usr/share/waydroid-extra/images",
];

const DEFAULTS: &[(&str, &str)] = &[
    ("images_path", "/var/lib/waydroid/images"),
    ("system_datetime", "0"),
    ("vendor_datetime", "0"),
];

/// Load the Waydroid configuration.
pub fn load() -> Properties {
    debug!("Loading Waydroid configuration");

    const CONFIG_FILE: &str = "/var/lib/waydroid/waydroid.cfg";
    let mut cfg = Ini::load_from_file(CONFIG_FILE)
        .inspect_err(|e| warn!("Failed to load config file: {e}"))
        .unwrap_or_default();

    let waydroid_section = cfg
        .entry(Some("waydroid".to_string()))
        .or_insert_with(Default::default);

    for (k, v) in DEFAULTS {
        if CONFIG_KEYS.contains(k) && !waydroid_section.contains_key(k) {
            debug!(r#""{k}" unset in configuration, using "{v}" as default"#);
            waydroid_section.insert(k.to_string(), v.to_string());
        }
    }

    debug!("{waydroid_section:#?}");

    waydroid_section.to_owned()
}
#[cfg(test)]
mod tests {

    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use chrono::{DateTime, Local};
    use log::LevelFilter;

    use super::*;
    use crate::logging::setup_logger;

    #[test]
    fn test_load() {
        setup_logger(LevelFilter::Debug);

        let config = load();

        dbg!(SystemTime::now().duration_since(UNIX_EPOCH).unwrap());
        let system_datetime =
            UNIX_EPOCH + Duration::from_secs(config["system_datetime"].parse().unwrap());
        dbg!(DateTime::<Local>::from(system_datetime));

        let vendor_datetime =
            UNIX_EPOCH + Duration::from_secs(config["vendor_datetime"].parse().unwrap());
        dbg!(DateTime::<Local>::from(vendor_datetime));
    }
}
