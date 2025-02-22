// SPDX-License-Identifier: GPL-3.0-or-later
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

use std::{env::var_os, process::ExitCode};

use log::{debug, info, warn, LevelFilter};
use reqwest::Client;
use serde::Deserialize;
use tokio::{process::Command, try_join};

use crate::{
    error::LogResult,
    logging::setup_logger,
    tools::{deserialize_max, load_config, UpdateDatetime, PREINSTALLED_IMAGES_PATHS},
};

/// Fetch Waydroid update JSON from a URL.
async fn get_update_datetime(client: &Client, url: &str) -> reqwest::Result<UpdateDatetime> {
    #[derive(Debug, Deserialize)]
    struct WaydroidResponse {
        #[serde(deserialize_with = "deserialize_max")]
        response: UpdateDatetime,
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

    let preinstalled = PREINSTALLED_IMAGES_PATHS.contains(&image_path);

    if preinstalled {
        warn!("Waydroid is using pre-installed image from {image_path}");
    }

    let system_ota_url = &config["system_ota"];
    let vendor_ota_url = &config["vendor_ota"];

    let (system_update_datetime, vendor_update_datetime) = {
        let client = Client::new();

        try_join!(
            get_update_datetime(&client, system_ota_url),
            get_update_datetime(&client, vendor_ota_url)
        )?
    };

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
        if preinstalled {
            info!("{upgrades} upgrade(s) available.");
            info!("You are using preinstalled image, so you need to upgrade manually.");
            Ok(ExitCode::from(upgrades))
        } else if var_os("NO_UPGRADE").is_some() {
            info!("{upgrades} upgrade(s) available.");
            info!("Run `sudo waydroid upgrade` to apply them.");
            Ok(ExitCode::from(upgrades))
        } else {
            info!("Upgrading with `sudo waydroid upgrade`...");
            let status = Command::new("sudo")
                .args(["waydroid", "upgrade"])
                .status()
                .await?
                .code()
                .unwrap_or(1) as u8;
            Ok(ExitCode::from(status))
        }
    } else {
        info!("No upgrades available.");
        Ok(ExitCode::SUCCESS)
    }
}
