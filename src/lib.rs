use std::fmt::{Debug, Display, Formatter};
use std::fmt;

pub mod clr;
use clr::Color;

pub trait AnsiEscape {
    fn ansi(&self) -> String;
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Color(r={}, g={}, b={})", self.0, self.1, self.2)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl AnsiEscape for Color {
    fn ansi(&self) -> String {
        format!("\x1B[38;2;{};{};{}m", self.0, self.1, self.2)
    }
}

pub enum Ansi {
    ColorEscape(Color),
    Bold,
    NotBold,
    Reset,
}

impl Display for Ansi {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Ansi::*;
        match self {
            ColorEscape(color) => write!(f, "{}", color.ansi()),
            Bold => write!(f, "\x1B[1m"),
            NotBold => write!(f, "\x1B[22m"),
            Reset => write!(f, "\x1B[0m"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn are_there_colors() {
        assert_eq!(format!("{}", clr::Color(255, 0, 0)), "(255, 0, 0)");
    }

    #[test]
    fn debug_working() {
        assert_eq!(format!("{:?}", clr::Color(255, 0, 0)), "Color(r=255, g=0, b=0)");
    }

    #[test]
    fn check_basic_colors() {
        assert_eq!(format!("{}", clr::R), "(255, 0, 0)");
        assert_eq!(format!("{}", clr::B), "(0, 0, 255)");
        assert_eq!(format!("{}", clr::M), "(191, 0, 191)");
    }

    #[test]
    fn check_css_colors() {
        assert_eq!(format!("{}", clr::CHARTREUSE), "(127, 255, 0)");
        assert_eq!(format!("{}", clr::GOLDENROD), "(218, 165, 32)");
        assert_eq!(format!("{}", clr::FIREBRICK), "(178, 34, 34)");
    }

    #[test]
    fn test_ansi_colors() {

        assert_eq!(format!("{}", Ansi::ColorEscape(clr::R)), "\x1B[38;2;255;0;0m");
        assert_eq!(format!("{}", Ansi::ColorEscape(clr::GOLDENROD)), "\x1B[38;2;218;165;32m");
        assert_eq!(format!("{}", Ansi::ColorEscape(Color(255,0,0))), "\x1B[38;2;255;0;0m");
    }
}
