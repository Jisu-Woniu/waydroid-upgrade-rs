//! # waydroid-upgrade
//!
//! Checks for upgrades for Waydroid images without restarting sessions.
//!
//! This script is intended to be run as a normal user,
//! since it will not write anything to the system.
//!
//! If an upgrade is available, it will call `sudo waydroid upgrade` to apply that.
//! You can skip that by setting the `NO_UPGRADE` environment variable.

mod error;
mod logging;
mod tools;

use std::process::ExitCode;

use log::{debug, info, warn, LevelFilter};
use reqwest::Client;
use serde::Deserialize;
use tokio::join;

use crate::{
    error::LogResult,
    logging::setup_logger,
    tools::{deserialize_max, load_config, UpdateDateTime, PREINSTALLED_IMAGES_PATHS},
};

/// Fetch Waydroid update JSON from a URL.
async fn get_update_json(client: &Client, url: &str) -> reqwest::Result<UpdateDateTime> {
    #[derive(Debug, Deserialize)]
    struct WaydroidResponse {
        #[serde(deserialize_with = "deserialize_max")]
        response: UpdateDateTime,
    }

    debug!(r#"Checking {url} for updates"#);

    let response = client.get(url).send().await?.error_for_status()?;
    debug!("Received JSON from {url}, extracting...");

    Ok(response.json::<WaydroidResponse>().await?.response)
}

#[tokio::main]
async fn main() -> LogResult<ExitCode> {
    setup_logger(if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });

    let config = load_config();

    let image_path = &config["images_path"];
    if PREINSTALLED_IMAGES_PATHS.contains(&image_path) {
        warn!("Upgrade refused because Waydroid loads pre-installed image: {image_path}");
        return Ok(ExitCode::from(255));
    }

    let system_ota_url = &config["system_ota"];
    let vendor_ota_url = &config["vendor_ota"];

    {
        let client = Client::new();

        let (system_updates, vendor_updates) = join!(
            get_update_json(&client, system_ota_url),
            get_update_json(&client, vendor_ota_url)
        );

        let system_update_datetime = system_updates?;
        let vendor_update_datetime = vendor_updates?;
        let mut upgrades = 0;
        let system_datetime = config["system_datetime"].parse()?;

        if system_update_datetime > system_datetime {
            info!(
                "System image upgrade available: {} (from {})",
                system_update_datetime, system_datetime
            );
            upgrades += 1;
        } else {
            info!("System image is up to date: {}", system_datetime);
        }

        let vendor_datetime = config["vendor_datetime"].parse()?;

        if vendor_update_datetime > vendor_datetime {
            info!(
                "Vendor image upgrade available: {} (from {})",
                vendor_update_datetime, vendor_datetime
            );
            upgrades += 1;
        } else {
            info!("Vendor image is up to date: {}", vendor_datetime);
        }

        if upgrades != 0 {
            if std::env::var_os("NO_UPGRADE").is_some() {
                info!("{upgrades} upgrade(s) available.");
                info!("Run `sudo waydroid upgrade` to apply them.");
                return Ok(ExitCode::from(upgrades));
            } else {
                info!("Upgrading with `sudo waydroid upgrade`...");
                let status = tokio::process::Command::new("sudo")
                    .arg("waydroid")
                    .arg("upgrade")
                    .status()
                    .await?
                    .code()
                    .unwrap_or(1) as u8;
                return Ok(ExitCode::from(status));
            }
        } else {
            info!("No upgrades available.");
        }
    };

    Ok(ExitCode::SUCCESS)
}
