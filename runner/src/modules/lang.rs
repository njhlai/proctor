use strum::{Display, EnumProperty, EnumString};

#[derive(Clone, Display, EnumProperty, EnumString, PartialEq)]
pub enum Lang {
    #[strum(serialize = "cpp", props(name = "C++"))]
    Cpp,
    #[strum(serialize = "py", props(name = "Python"))]
    Python,
    #[strum(serialize = "rs", props(name = "Rust"))]
    Rust,
}
