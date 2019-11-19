//! # rcolours
//! CSS and [XKCD](https://blog.xkcd.com/2010/05/03/color-survey-results/) colour package for rust.
//!
//! Makes CSS and XKCD colours (copied shamelessly from [matplotlib](https://matplotlib.org/)'s
//! [color module](https://matplotlib.org/2.0.2/api/colors_api.html)) available as `u8`-tuples and
//! provides basic conversion functions into different representations.
//!
//! An emphasis is on transforming the colour tuple to [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code) for colouring console output.
//! For this goal the crate provides the
//!
//! ## Colour representation
//!
//! For each colour there is detailled information on how to represent it in different ways (for now hex,
//! int/float tuple in RGB space). Other colour spaces (HUV, L\*AB, maybe also XYZ and spectrum samples)
//! to come).
//!
//! ## Basic Usage
//!
//! ```
//! use colors::clr::RED;
//! use colors::Ansi;
//!
//!
//! println!("Change colour and font face of console output:");
//! println!("Make the output {}red{}, {}bold{}", Ansi::ColorEscape(RED), Ansi::Reset, Ansi::Bold, Ansi::Reset);
//!
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub mod clr;
use clr::Color;

/// A trait that transforms an objects into an ANSI escape sequence.
pub trait AnsiEscape {
    fn ansi(&self) -> String;
}

impl AnsiEscape for Color {
    fn ansi(&self) -> String {
        format!("\x1B[38;2;{};{};{}m", self.0, self.1, self.2)
    }
}

/// Different kinds of escape codes. The `ColorEscape` variant converts a `Color` struct into an escape
/// code.
///
/// # Examples
/// - Color the console output
/// ```
/// use colors::clr::Color;
/// use colors::Ansi;
///
/// let red = Ansi::ColorEscape(Color(255,0,0));
/// assert_eq!(format!("{}", red), "\x1B[38;2;255;0;0m");
/// ```
/// - make it bold
/// ```
/// use colors::Ansi;
///
/// assert_eq!(format!("{}", Ansi::Bold), "\x1B[1m");
/// ```
pub enum Ansi {
    /// Transforms the `Color` struct into an escape code of the form `ESC[38;2;R;G;Bm`
    ColorEscape(Color),
    /// Sets the foreground color to one of 256 predefined colours from the colour palette
    ForegroundColor(u8),
    /// Sets the background color to one of 256 predefined colours from the colour palette
    BackgroundColor(u8),
    /// Sets the font to bold
    Bold,
    /// Unsets bold
    NotBold,
    /// Resets all changes to font and color
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
            ForegroundColor(c) => write!(f, "\x1B[38;5;{}m", c),
            BackgroundColor(c) => write!(f, "\x1B[48;5;{}m", c),
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
    fn convert_to_hex() {
        assert_eq!(clr::Color(255, 0, 0).to_hex(), "#FF0000");
    }

    #[test]
    fn debug_working() {
        assert_eq!(
            format!("{:?}", clr::Color(255, 0, 0)),
            "Color(r=255, g=0, b=0)"
        );
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
        assert_eq!(
            format!("{}", Ansi::ColorEscape(clr::R)),
            "\x1B[38;2;255;0;0m"
        );
        assert_eq!(
            format!("{}", Ansi::ColorEscape(clr::GOLDENROD)),
            "\x1B[38;2;218;165;32m"
        );
        assert_eq!(
            format!("{}", Ansi::ColorEscape(Color(255, 0, 0))),
            "\x1B[38;2;255;0;0m"
        );
    }
}
