//! SBI console driver, for text output

use log::{Metadata, Record};

use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// print string macro
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// println string macro
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

/// wrap args with ansi escape code: `\x1b[{code}m{args}\x1b[0m`
macro_rules! ansi_escape {
    ($args: expr, $code: expr) => {{
        format_args!("\x1b[{}m{}\x1b[0m", $code, $args)
    }};
}

/// map log::Level to ansi color code
fn level_to_color_code(level: log::Level) -> u8 {
    match level {
        log::Level::Error => 31, // red
        log::Level::Warn => 93,  // yellow
        log::Level::Info => 34,  // blue
        log::Level::Debug => 32, // green
        log::Level::Trace => 90, // gray
    }
}

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn flush(&self) {}

    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        crate::console::print(ansi_escape!(
            format_args!("[{:>5}] {}\n", record.level(), record.args()),
            level_to_color_code(record.level())
        ))
    }
}

/// init logger
pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger {};
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
}
