// These re-exports of the log crate make it easy to use this crate without having to depend on the
// log crate directly. See <https://github.com/clap-rs/clap-verbosity-flag/issues/54> for more
// information.
pub use log::Level;
pub use log::LevelFilter;

use crate::{Filter, LogLevel};

impl From<Filter> for LevelFilter {
    fn from(filter: Filter) -> Self {
        match filter {
            Filter::Off => LevelFilter::Off,
            Filter::Error => LevelFilter::Error,
            Filter::Warn => LevelFilter::Warn,
            Filter::Info => LevelFilter::Info,
            Filter::Debug => LevelFilter::Debug,
            Filter::Trace => LevelFilter::Trace,
        }
    }
}

impl From<LevelFilter> for Filter {
    fn from(level: LevelFilter) -> Self {
        match level {
            LevelFilter::Off => Filter::Off,
            LevelFilter::Error => Filter::Error,
            LevelFilter::Warn => Filter::Warn,
            LevelFilter::Info => Filter::Info,
            LevelFilter::Debug => Filter::Debug,
            LevelFilter::Trace => Filter::Trace,
        }
    }
}

impl From<Filter> for Option<Level> {
    fn from(filter: Filter) -> Self {
        match filter {
            Filter::Off => None,
            Filter::Error => Some(Level::Error),
            Filter::Warn => Some(Level::Warn),
            Filter::Info => Some(Level::Info),
            Filter::Debug => Some(Level::Debug),
            Filter::Trace => Some(Level::Trace),
        }
    }
}

impl From<Option<Level>> for Filter {
    fn from(level: Option<Level>) -> Self {
        match level {
            None => Filter::Off,
            Some(Level::Error) => Filter::Error,
            Some(Level::Warn) => Filter::Warn,
            Some(Level::Info) => Filter::Info,
            Some(Level::Debug) => Filter::Debug,
            Some(Level::Trace) => Filter::Trace,
        }
    }
}

/// Default to [`log::Level::Error`]
#[allow(clippy::exhaustive_structs)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ErrorLevel;

impl LogLevel for ErrorLevel {
    type Level = Level;
    type LevelFilter = LevelFilter;
    fn default() -> Option<Level> {
        Some(Level::Error)
    }
}

/// Default to [`log::Level::Warn`]
#[allow(clippy::exhaustive_structs)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WarnLevel;

impl LogLevel for WarnLevel {
    type Level = Level;
    type LevelFilter = LevelFilter;
    fn default() -> Option<Level> {
        Some(Level::Warn)
    }
}

/// Default to [`log::Level::Info`]
#[allow(clippy::exhaustive_structs)]
#[derive(Copy, Clone, Debug, Default)]
pub struct InfoLevel;

impl LogLevel for InfoLevel {
    type Level = Level;
    type LevelFilter = LevelFilter;
    fn default() -> Option<Level> {
        Some(Level::Info)
    }
}

#[cfg(test)]
mod tests {
    use crate::Verbosity;

    use super::*;

    #[test]
    fn verbosity_error_level() {
        let tests = [
            // verbose, quiet, expected_level, expected_filter
            (0, 0, Some(Level::Error), LevelFilter::Error),
            (1, 0, Some(Level::Warn), LevelFilter::Warn),
            (2, 0, Some(Level::Info), LevelFilter::Info),
            (3, 0, Some(Level::Debug), LevelFilter::Debug),
            (4, 0, Some(Level::Trace), LevelFilter::Trace),
            (5, 0, Some(Level::Trace), LevelFilter::Trace),
            (255, 0, Some(Level::Trace), LevelFilter::Trace),
            (0, 1, None, LevelFilter::Off),
            (0, 2, None, LevelFilter::Off),
            (0, 255, None, LevelFilter::Off),
            (255, 255, Some(Level::Error), LevelFilter::Error),
        ];

        for (verbose, quiet, expected_level, expected_filter) in tests.iter() {
            let v = Verbosity::<ErrorLevel>::new(*verbose, *quiet);
            assert_eq!(
                v.log_level(),
                *expected_level,
                "verbose = {verbose}, quiet = {quiet}"
            );
            assert_eq!(
                v.log_level_filter(),
                *expected_filter,
                "verbose = {verbose}, quiet = {quiet}"
            );
        }
    }

    #[test]
    fn verbosity_warn_level() {
        let tests = [
            // verbose, quiet, expected_level, expected_filter
            (0, 0, Some(Level::Warn), LevelFilter::Warn),
            (1, 0, Some(Level::Info), LevelFilter::Info),
            (2, 0, Some(Level::Debug), LevelFilter::Debug),
            (3, 0, Some(Level::Trace), LevelFilter::Trace),
            (4, 0, Some(Level::Trace), LevelFilter::Trace),
            (255, 0, Some(Level::Trace), LevelFilter::Trace),
            (0, 1, Some(Level::Error), LevelFilter::Error),
            (0, 2, None, LevelFilter::Off),
            (0, 3, None, LevelFilter::Off),
            (0, 255, None, LevelFilter::Off),
            (255, 255, Some(Level::Warn), LevelFilter::Warn),
        ];

        for (verbose, quiet, expected_level, expected_filter) in tests.iter() {
            let v = Verbosity::<WarnLevel>::new(*verbose, *quiet);
            assert_eq!(
                v.log_level(),
                *expected_level,
                "verbose = {verbose}, quiet = {quiet}"
            );
            assert_eq!(
                v.log_level_filter(),
                *expected_filter,
                "verbose = {verbose}, quiet = {quiet}"
            );
        }
    }

    #[test]
    fn verbosity_info_level() {
        let tests = [
            // verbose, quiet, expected_level, expected_filter
            (0, 0, Some(Level::Info), LevelFilter::Info),
            (1, 0, Some(Level::Debug), LevelFilter::Debug),
            (2, 0, Some(Level::Trace), LevelFilter::Trace),
            (3, 0, Some(Level::Trace), LevelFilter::Trace),
            (255, 0, Some(Level::Trace), LevelFilter::Trace),
            (0, 1, Some(Level::Warn), LevelFilter::Warn),
            (0, 2, Some(Level::Error), LevelFilter::Error),
            (0, 3, None, LevelFilter::Off),
            (0, 4, None, LevelFilter::Off),
            (0, 255, None, LevelFilter::Off),
            (255, 255, Some(Level::Info), LevelFilter::Info),
        ];

        for (verbose, quiet, expected_level, expected_filter) in tests.iter() {
            let v = Verbosity::<InfoLevel>::new(*verbose, *quiet);
            assert_eq!(
                v.log_level(),
                *expected_level,
                "verbose = {verbose}, quiet = {quiet}"
            );
            assert_eq!(
                v.log_level_filter(),
                *expected_filter,
                "verbose = {verbose}, quiet = {quiet}"
            );
        }
    }
}
