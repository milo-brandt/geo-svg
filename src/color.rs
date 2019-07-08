use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Named(&'static str),
    Rgb(u8, u8, u8),
    Hex(u32),
}

impl Display for Color {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match self {
            Color::Named(name) => write!(fmt, "{}", name),
            Color::Rgb(r, g, b) => write!(fmt, "rgb({},{},{})", r, g, b),
            Color::Hex(hex) => write!(fmt, "{:#X}", hex),
        }
    }
}
