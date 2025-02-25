use env_logger::{Builder, Env};
use log::Level;
use std::io::Write;

pub fn init() {
    Builder::from_env(Env::default().default_filter_or("info"))
        .format(|formatter, record| {
            let timestamp = formatter.timestamp_seconds();

            // Choose a color based on the log level.
            let mut style = formatter.style();
            style.set_color(match record.level() {
                Level::Error => env_logger::fmt::Color::Red,
                Level::Warn => env_logger::fmt::Color::Yellow,
                Level::Info => env_logger::fmt::Color::Green,
                Level::Debug => env_logger::fmt::Color::Blue,
                Level::Trace => env_logger::fmt::Color::Magenta,
            });

            // Wrap the level with the chosen color style.
            let level = style.value(record.level());

            // Print: `YYYY-MM-DD HH:MM:SS LEVEL message`
            writeln!(formatter, "{} {} {}", timestamp, level, record.args())
        })
        .init();
}