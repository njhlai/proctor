use colored::{ColoredString, Colorize};

/// Adds more colors to the trait [`Colorize`].
#[allow(clippy::module_name_repetitions)]
pub trait MoreColorize: Colorize {
    fn orange(self) -> ColoredString;
}

impl<'a> MoreColorize for &'a str {
    fn orange(self) -> ColoredString {
        self.truecolor(255, 165, 0)
    }
}
