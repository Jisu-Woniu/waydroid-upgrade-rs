// SPDX-License-Identifier: GPL-3.0-or-later
use std::io::Write;

use chrono::{Local, SubsecRound};
use env_logger::{
    fmt::style::{AnsiColor, Style},
    Builder,
};
use log::LevelFilter;

pub fn setup_logger(level: LevelFilter) {
    Builder::new()
        .format(|buf, record| {
            let subtle = Style::new().fg_color(Some(AnsiColor::BrightBlack.into()));
            let level_style = buf.default_level_style(record.level());

            writeln!(
                buf,
                "{subtle}[{subtle:#}{} {level_style}{:<5}{level_style:#} {}{subtle}]{subtle:#} {}",
                Local::now().trunc_subsecs(3), // Like `2001-07-08 00:34:60.026 +09:30`.
                record.level(),
                record.target(),
                record.args()
            )
        })
        .filter_level(level)
        .parse_default_env()
        .init();
}
