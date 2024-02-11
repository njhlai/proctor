use colored::{ColoredString, Colorize};

pub trait ExtColorize: Colorize {
    fn orange(self) -> ColoredString;
}

impl<'a> ExtColorize for &'a str {
    fn orange(self) -> ColoredString {
        self.truecolor(255, 165, 0)
    }
}
