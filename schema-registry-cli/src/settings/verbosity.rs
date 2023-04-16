use clap::{ArgAction, Args};

/// Verbosity
///
/// - `-qq`     silences output
/// - `-q`      only shows errors
/// - (default) shows warnings and errors
/// - `-v`      only shows infos, warnings and errors
/// - `-vv`     only shows debugs, infos, warnings and errors
/// - `-vvv`    only shows traces, debugs, infos, warnings and errors
#[derive(Debug, Clone, Copy, Default, Args)]
pub struct Verbosity {
    /// More outputs per occurrence
    #[arg(long, short='v', action= ArgAction::Count, global = true)]
    verbose: u8,

    /// Less outputs per occurrence
    #[arg(long, short='q', action= ArgAction::Count, global = true)]
    quiet: u8,
}

impl From<Verbosity> for i16 {
    fn from(value: Verbosity) -> Self {
        let result = i16::from(value.verbose) - i16::from(value.quiet);
        result.clamp(-2, 3)
    }
}

impl PartialEq for Verbosity {
    fn eq(&self, other: &Self) -> bool {
        i16::from(*self) == i16::from(*other)
    }
}
impl Eq for Verbosity {}

impl Ord for Verbosity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        i16::from(*self).cmp(&i16::from(*other))
    }
}
impl PartialOrd for Verbosity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Verbosity {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    const fn new(level: i16) -> Self {
        if level == 0 {
            Self {
                verbose: 0,
                quiet: 0,
            }
        } else if level > 0 {
            Self {
                verbose: level as u8,
                quiet: 0,
            }
        } else {
            Self {
                verbose: 0,
                quiet: level.unsigned_abs() as u8,
            }
        }
    }

    /// Level OFF
    pub const OFF: Verbosity = Verbosity::new(-2);

    /// Level ERROR
    pub const ERROR: Verbosity = Verbosity::new(-1);

    /// Level WARN
    pub const WARN: Verbosity = Verbosity::new(0);

    /// Level INFO
    pub const INFO: Verbosity = Verbosity::new(1);

    /// Level DEBUG
    pub const DEBUG: Verbosity = Verbosity::new(2);

    /// Level TRACE
    pub const TRACE: Verbosity = Verbosity::new(3);
}

#[cfg(test)]
mod tests {
    use assert2::check;
    use clap::Parser;

    use super::*;

    #[derive(Debug, Parser)]
    struct JustVerbosity {
        #[clap(flatten)]
        verbosity: Verbosity,
    }

    #[rstest::rstest]
    #[case::off(Verbosity::OFF, -2)]
    #[case::error(Verbosity::ERROR, -1)]
    #[case::warn(Verbosity::WARN, 0)]
    #[case::info(Verbosity::INFO, 1)]
    #[case::debug(Verbosity::DEBUG, 2)]
    #[case::trace(Verbosity::TRACE, 3)]
    fn should_provide_number(#[case] verbosity: Verbosity, #[case] num: i16) {
        let result = i16::from(verbosity);
        check!(result == num);
        let result = Verbosity::new(num);
        check!(result == verbosity);
    }

    #[rstest::rstest]
    #[case::trace(&["bin", "-vvvvvvvvvvv"], Verbosity::TRACE)]
    #[case::trace(&["bin", "-vvv"], Verbosity::TRACE)]
    #[case::debug(&["bin", "-vv"], Verbosity::DEBUG)]
    #[case::info(&["bin", "-v"], Verbosity::INFO)]
    #[case::warn(&["bin"], Verbosity::WARN)]
    #[case::error(&["bin", "-q"], Verbosity::ERROR)]
    #[case::off(&["bin", "-qq"], Verbosity::OFF)]
    #[case::off(&["bin", "-qqqqqqqq"], Verbosity::OFF)]
    fn should_parse_verbosity(#[case] args: &[&str], #[case] expected: Verbosity) {
        let result = JustVerbosity::parse_from(args);
        check!(result.verbosity == expected);
    }

    #[test]
    fn should_be_sorted() {
        let v = &[
            Verbosity::OFF,
            Verbosity::ERROR,
            Verbosity::WARN,
            Verbosity::INFO,
            Verbosity::DEBUG,
            Verbosity::TRACE,
        ];

        for win in v.windows(2) {
            check!(win[0] < win[1]);
            check!(i16::from(win[0]) < i16::from(win[1]));
        }
    }
}
