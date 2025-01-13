use core::fmt;
use log::{max_level, set_logger, set_max_level, Level, LevelFilter, Log};

pub(crate) fn init() {
    if let Err(err) = set_logger(&SimpleLogger) {
        panic!("Failed to set logger: {}", err);
    }
    set_max_level(match option_env!("LOG") {
        Some("debug") => LevelFilter::Debug,
        Some("info") => LevelFilter::Info,
        Some("warn") => LevelFilter::Warn,
        Some("error") => LevelFilter::Error,
        Some("trace") => LevelFilter::Trace,
        Some(_) | None => LevelFilter::Off,
    });
}

struct SimpleLogger;

fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // Yellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // Gray
    }
}

/// Add escape sequence to print with color in Linux console
macro_rules! format_with_color {
    ($args: ident, $color_code: ident) => {{
        format_args!("\u{1B}[{}m{}\u{1B}[0m", $color_code as u8, $args)
    }};
}

fn log_with_color(args: fmt::Arguments, color_code: u8) {
    crate::console::print(format_with_color!(args, color_code));
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= max_level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            log_with_color(
                format_args!(
                    "[{:>5}][{},-] {}\n",
                    record.level(),
                    crate::arch::cpu::cpu_id(),
                    record.args()
                ),
                level_to_color_code(record.level()),
            );
        }
    }

    fn flush(&self) {}
}
