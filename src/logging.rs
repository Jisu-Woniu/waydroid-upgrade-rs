use std::io::Write;

use chrono::Local;
use env_logger::{
    fmt::style::{AnsiColor, Style},
    Builder,
};
use log::LevelFilter;

pub fn setup_logger(level: LevelFilter) {
    Builder::from_default_env()
        .format(|buf, record| {
            let subtle = Style::new().fg_color(Some(AnsiColor::BrightBlack.into()));
            let level_style = buf.default_level_style(record.level());

            writeln!(
                buf,
                "{subtle}[{subtle:#}{} {level_style}{:<5}{level_style:#} {}{subtle}]{subtle:#} {}",
                Local::now().format("%FT%T%.3f%:z"), // Like `2001-07-08T00:34:60.026+09:30`.
                record.level(),
                record.target(),
                record.args()
            )
        })
        .filter_level(level)
        .init();
}
