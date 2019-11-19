use std::fmt;
use std::fmt::{Debug, Display, Formatter};

/// An RGB24 color representation. Each channel has a depth of 8 bit.
#[derive(Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    /// Prints out a hex representation `#RRGGBB` of the color.
    /// Example
    /// ```
    /// # use colors::clr::Color;
    /// # fn main() {
    /// let red = Color(255, 0, 0);
    /// assert_eq!(red.to_hex(), "#FF0000");
    /// # }
    /// ```
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

/// Debug formatting for Color
impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Color(r={}, g={}, b={})", self.0, self.1, self.2)
    }
}

/// Display formatting for Color, will print in the format `(r, g, b)` where `r`, `g` and `b` are the
/// color values as `unsigned int8`.
impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}


/// Colour `(r = 0, g = 0, b = 255)`
///
/// Colour `B` from the set `BASE_COLORS`. (Colour number `0`)
/// ## Representations:
/// - int tuple `(0, 0, 255)`
/// - float tuple `(0.0, 0, 1)`
/// - hex: `#0000FF`
pub const B: Color = Color(0, 0, 255);

/// Colour `(r = 0, g = 127, b = 0)`
///
/// Colour `G` from the set `BASE_COLORS`. (Colour number `1`)
/// ## Representations:
/// - int tuple `(0, 127, 0)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#007F00`
pub const G: Color = Color(0, 127, 0);

/// Colour `(r = 255, g = 0, b = 0)`
///
/// Colour `R` from the set `BASE_COLORS`. (Colour number `2`)
/// ## Representations:
/// - int tuple `(255, 0, 0)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FF0000`
pub const R: Color = Color(255, 0, 0);

/// Colour `(r = 0, g = 191, b = 191)`
///
/// Colour `C` from the set `BASE_COLORS`. (Colour number `3`)
/// ## Representations:
/// - int tuple `(0, 191, 191)`
/// - float tuple `(0.0, 1, 1)`
/// - hex: `#00BFBF`
pub const C: Color = Color(0, 191, 191);

/// Colour `(r = 191, g = 0, b = 191)`
///
/// Colour `M` from the set `BASE_COLORS`. (Colour number `4`)
/// ## Representations:
/// - int tuple `(191, 0, 191)`
/// - float tuple `(0.75, 0, 1)`
/// - hex: `#BF00BF`
pub const M: Color = Color(191, 0, 191);

/// Colour `(r = 191, g = 191, b = 0)`
///
/// Colour `Y` from the set `BASE_COLORS`. (Colour number `5`)
/// ## Representations:
/// - int tuple `(191, 191, 0)`
/// - float tuple `(0.75, 1, 0)`
/// - hex: `#BFBF00`
pub const Y: Color = Color(191, 191, 0);

/// Colour `(r = 0, g = 0, b = 0)`
///
/// Colour `K` from the set `BASE_COLORS`. (Colour number `6`)
/// ## Representations:
/// - int tuple `(0, 0, 0)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#000000`
pub const K: Color = Color(0, 0, 0);

/// Colour `(r = 255, g = 255, b = 255)`
///
/// Colour `W` from the set `BASE_COLORS`. (Colour number `7`)
/// ## Representations:
/// - int tuple `(255, 255, 255)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFFFFF`
pub const W: Color = Color(255, 255, 255);

/// Colour `(r = 240, g = 248, b = 255)`
///
/// Colour `ALICEBLUE` from the set `CSS4_COLORS`. (Colour number `8`)
/// ## Representations:
/// - int tuple `(240, 248, 255)`
/// - float tuple `(0.94, 1, 1)`
/// - hex: `#F0F8FF`
pub const ALICEBLUE: Color = Color(240, 248, 255);

/// Colour `(r = 250, g = 235, b = 215)`
///
/// Colour `ANTIQUEWHITE` from the set `CSS4_COLORS`. (Colour number `9`)
/// ## Representations:
/// - int tuple `(250, 235, 215)`
/// - float tuple `(0.98, 1, 1)`
/// - hex: `#FAEBD7`
pub const ANTIQUEWHITE: Color = Color(250, 235, 215);

/// Colour `(r = 0, g = 255, b = 255)`
///
/// Colour `AQUA` from the set `CSS4_COLORS`. (Colour number `10`)
/// ## Representations:
/// - int tuple `(0, 255, 255)`
/// - float tuple `(0.0, 1, 1)`
/// - hex: `#00FFFF`
pub const AQUA: Color = Color(0, 255, 255);

/// Colour `(r = 127, g = 255, b = 212)`
///
/// Colour `AQUAMARINE` from the set `CSS4_COLORS`. (Colour number `11`)
/// ## Representations:
/// - int tuple `(127, 255, 212)`
/// - float tuple `(0.5, 1, 1)`
/// - hex: `#7FFFD4`
pub const AQUAMARINE: Color = Color(127, 255, 212);

/// Colour `(r = 240, g = 255, b = 255)`
///
/// Colour `AZURE` from the set `CSS4_COLORS`. (Colour number `12`)
/// ## Representations:
/// - int tuple `(240, 255, 255)`
/// - float tuple `(0.94, 1, 1)`
/// - hex: `#F0FFFF`
pub const AZURE: Color = Color(240, 255, 255);

/// Colour `(r = 245, g = 245, b = 220)`
///
/// Colour `BEIGE` from the set `CSS4_COLORS`. (Colour number `13`)
/// ## Representations:
/// - int tuple `(245, 245, 220)`
/// - float tuple `(0.96, 1, 1)`
/// - hex: `#F5F5DC`
pub const BEIGE: Color = Color(245, 245, 220);

/// Colour `(r = 255, g = 228, b = 196)`
///
/// Colour `BISQUE` from the set `CSS4_COLORS`. (Colour number `14`)
/// ## Representations:
/// - int tuple `(255, 228, 196)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFE4C4`
pub const BISQUE: Color = Color(255, 228, 196);

/// Colour `(r = 0, g = 0, b = 0)`
///
/// Colour `BLACK` from the set `CSS4_COLORS`. (Colour number `15`)
/// ## Representations:
/// - int tuple `(0, 0, 0)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#000000`
pub const BLACK: Color = Color(0, 0, 0);

/// Colour `(r = 255, g = 235, b = 205)`
///
/// Colour `BLANCHEDALMOND` from the set `CSS4_COLORS`. (Colour number `16`)
/// ## Representations:
/// - int tuple `(255, 235, 205)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFEBCD`
pub const BLANCHEDALMOND: Color = Color(255, 235, 205);

/// Colour `(r = 0, g = 0, b = 255)`
///
/// Colour `BLUE` from the set `CSS4_COLORS`. (Colour number `17`)
/// ## Representations:
/// - int tuple `(0, 0, 255)`
/// - float tuple `(0.0, 0, 1)`
/// - hex: `#0000FF`
pub const BLUE: Color = Color(0, 0, 255);

/// Colour `(r = 138, g = 43, b = 226)`
///
/// Colour `BLUEVIOLET` from the set `CSS4_COLORS`. (Colour number `18`)
/// ## Representations:
/// - int tuple `(138, 43, 226)`
/// - float tuple `(0.54, 0, 1)`
/// - hex: `#8A2BE2`
pub const BLUEVIOLET: Color = Color(138, 43, 226);

/// Colour `(r = 165, g = 42, b = 42)`
///
/// Colour `BROWN` from the set `CSS4_COLORS`. (Colour number `19`)
/// ## Representations:
/// - int tuple `(165, 42, 42)`
/// - float tuple `(0.65, 0, 0)`
/// - hex: `#A52A2A`
pub const BROWN: Color = Color(165, 42, 42);

/// Colour `(r = 222, g = 184, b = 135)`
///
/// Colour `BURLYWOOD` from the set `CSS4_COLORS`. (Colour number `20`)
/// ## Representations:
/// - int tuple `(222, 184, 135)`
/// - float tuple `(0.87, 1, 1)`
/// - hex: `#DEB887`
pub const BURLYWOOD: Color = Color(222, 184, 135);

/// Colour `(r = 95, g = 158, b = 160)`
///
/// Colour `CADETBLUE` from the set `CSS4_COLORS`. (Colour number `21`)
/// ## Representations:
/// - int tuple `(95, 158, 160)`
/// - float tuple `(0.37, 1, 1)`
/// - hex: `#5F9EA0`
pub const CADETBLUE: Color = Color(95, 158, 160);

/// Colour `(r = 127, g = 255, b = 0)`
///
/// Colour `CHARTREUSE` from the set `CSS4_COLORS`. (Colour number `22`)
/// ## Representations:
/// - int tuple `(127, 255, 0)`
/// - float tuple `(0.5, 1, 0)`
/// - hex: `#7FFF00`
pub const CHARTREUSE: Color = Color(127, 255, 0);

/// Colour `(r = 210, g = 105, b = 30)`
///
/// Colour `CHOCOLATE` from the set `CSS4_COLORS`. (Colour number `23`)
/// ## Representations:
/// - int tuple `(210, 105, 30)`
/// - float tuple `(0.82, 0, 0)`
/// - hex: `#D2691E`
pub const CHOCOLATE: Color = Color(210, 105, 30);

/// Colour `(r = 255, g = 127, b = 80)`
///
/// Colour `CORAL` from the set `CSS4_COLORS`. (Colour number `24`)
/// ## Representations:
/// - int tuple `(255, 127, 80)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FF7F50`
pub const CORAL: Color = Color(255, 127, 80);

/// Colour `(r = 100, g = 149, b = 237)`
///
/// Colour `CORNFLOWERBLUE` from the set `CSS4_COLORS`. (Colour number `25`)
/// ## Representations:
/// - int tuple `(100, 149, 237)`
/// - float tuple `(0.39, 1, 1)`
/// - hex: `#6495ED`
pub const CORNFLOWERBLUE: Color = Color(100, 149, 237);

/// Colour `(r = 255, g = 248, b = 220)`
///
/// Colour `CORNSILK` from the set `CSS4_COLORS`. (Colour number `26`)
/// ## Representations:
/// - int tuple `(255, 248, 220)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFF8DC`
pub const CORNSILK: Color = Color(255, 248, 220);

/// Colour `(r = 220, g = 20, b = 60)`
///
/// Colour `CRIMSON` from the set `CSS4_COLORS`. (Colour number `27`)
/// ## Representations:
/// - int tuple `(220, 20, 60)`
/// - float tuple `(0.86, 0, 0)`
/// - hex: `#DC143C`
pub const CRIMSON: Color = Color(220, 20, 60);

/// Colour `(r = 0, g = 255, b = 255)`
///
/// Colour `CYAN` from the set `CSS4_COLORS`. (Colour number `28`)
/// ## Representations:
/// - int tuple `(0, 255, 255)`
/// - float tuple `(0.0, 1, 1)`
/// - hex: `#00FFFF`
pub const CYAN: Color = Color(0, 255, 255);

/// Colour `(r = 0, g = 0, b = 139)`
///
/// Colour `DARKBLUE` from the set `CSS4_COLORS`. (Colour number `29`)
/// ## Representations:
/// - int tuple `(0, 0, 139)`
/// - float tuple `(0.0, 0, 1)`
/// - hex: `#00008B`
pub const DARKBLUE: Color = Color(0, 0, 139);

/// Colour `(r = 0, g = 139, b = 139)`
///
/// Colour `DARKCYAN` from the set `CSS4_COLORS`. (Colour number `30`)
/// ## Representations:
/// - int tuple `(0, 139, 139)`
/// - float tuple `(0.0, 1, 1)`
/// - hex: `#008B8B`
pub const DARKCYAN: Color = Color(0, 139, 139);

/// Colour `(r = 184, g = 134, b = 11)`
///
/// Colour `DARKGOLDENROD` from the set `CSS4_COLORS`. (Colour number `31`)
/// ## Representations:
/// - int tuple `(184, 134, 11)`
/// - float tuple `(0.72, 1, 0)`
/// - hex: `#B8860B`
pub const DARKGOLDENROD: Color = Color(184, 134, 11);

/// Colour `(r = 169, g = 169, b = 169)`
///
/// Colour `DARKGRAY` from the set `CSS4_COLORS`. (Colour number `32`)
/// ## Representations:
/// - int tuple `(169, 169, 169)`
/// - float tuple `(0.66, 1, 1)`
/// - hex: `#A9A9A9`
pub const DARKGRAY: Color = Color(169, 169, 169);

/// Colour `(r = 0, g = 100, b = 0)`
///
/// Colour `DARKGREEN` from the set `CSS4_COLORS`. (Colour number `33`)
/// ## Representations:
/// - int tuple `(0, 100, 0)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#006400`
pub const DARKGREEN: Color = Color(0, 100, 0);

/// Colour `(r = 169, g = 169, b = 169)`
///
/// Colour `DARKGREY` from the set `CSS4_COLORS`. (Colour number `34`)
/// ## Representations:
/// - int tuple `(169, 169, 169)`
/// - float tuple `(0.66, 1, 1)`
/// - hex: `#A9A9A9`
pub const DARKGREY: Color = Color(169, 169, 169);

/// Colour `(r = 189, g = 183, b = 107)`
///
/// Colour `DARKKHAKI` from the set `CSS4_COLORS`. (Colour number `35`)
/// ## Representations:
/// - int tuple `(189, 183, 107)`
/// - float tuple `(0.74, 1, 0)`
/// - hex: `#BDB76B`
pub const DARKKHAKI: Color = Color(189, 183, 107);

/// Colour `(r = 139, g = 0, b = 139)`
///
/// Colour `DARKMAGENTA` from the set `CSS4_COLORS`. (Colour number `36`)
/// ## Representations:
/// - int tuple `(139, 0, 139)`
/// - float tuple `(0.55, 0, 1)`
/// - hex: `#8B008B`
pub const DARKMAGENTA: Color = Color(139, 0, 139);

/// Colour `(r = 85, g = 107, b = 47)`
///
/// Colour `DARKOLIVEGREEN` from the set `CSS4_COLORS`. (Colour number `37`)
/// ## Representations:
/// - int tuple `(85, 107, 47)`
/// - float tuple `(0.33, 0, 0)`
/// - hex: `#556B2F`
pub const DARKOLIVEGREEN: Color = Color(85, 107, 47);

/// Colour `(r = 255, g = 140, b = 0)`
///
/// Colour `DARKORANGE` from the set `CSS4_COLORS`. (Colour number `38`)
/// ## Representations:
/// - int tuple `(255, 140, 0)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FF8C00`
pub const DARKORANGE: Color = Color(255, 140, 0);

/// Colour `(r = 153, g = 50, b = 204)`
///
/// Colour `DARKORCHID` from the set `CSS4_COLORS`. (Colour number `39`)
/// ## Representations:
/// - int tuple `(153, 50, 204)`
/// - float tuple `(0.6, 0, 1)`
/// - hex: `#9932CC`
pub const DARKORCHID: Color = Color(153, 50, 204);

/// Colour `(r = 139, g = 0, b = 0)`
///
/// Colour `DARKRED` from the set `CSS4_COLORS`. (Colour number `40`)
/// ## Representations:
/// - int tuple `(139, 0, 0)`
/// - float tuple `(0.55, 0, 0)`
/// - hex: `#8B0000`
pub const DARKRED: Color = Color(139, 0, 0);

/// Colour `(r = 233, g = 150, b = 122)`
///
/// Colour `DARKSALMON` from the set `CSS4_COLORS`. (Colour number `41`)
/// ## Representations:
/// - int tuple `(233, 150, 122)`
/// - float tuple `(0.91, 1, 0)`
/// - hex: `#E9967A`
pub const DARKSALMON: Color = Color(233, 150, 122);

/// Colour `(r = 143, g = 188, b = 143)`
///
/// Colour `DARKSEAGREEN` from the set `CSS4_COLORS`. (Colour number `42`)
/// ## Representations:
/// - int tuple `(143, 188, 143)`
/// - float tuple `(0.56, 1, 1)`
/// - hex: `#8FBC8F`
pub const DARKSEAGREEN: Color = Color(143, 188, 143);

/// Colour `(r = 72, g = 61, b = 139)`
///
/// Colour `DARKSLATEBLUE` from the set `CSS4_COLORS`. (Colour number `43`)
/// ## Representations:
/// - int tuple `(72, 61, 139)`
/// - float tuple `(0.28, 0, 1)`
/// - hex: `#483D8B`
pub const DARKSLATEBLUE: Color = Color(72, 61, 139);

/// Colour `(r = 47, g = 79, b = 79)`
///
/// Colour `DARKSLATEGRAY` from the set `CSS4_COLORS`. (Colour number `44`)
/// ## Representations:
/// - int tuple `(47, 79, 79)`
/// - float tuple `(0.18, 0, 0)`
/// - hex: `#2F4F4F`
pub const DARKSLATEGRAY: Color = Color(47, 79, 79);

/// Colour `(r = 47, g = 79, b = 79)`
///
/// Colour `DARKSLATEGREY` from the set `CSS4_COLORS`. (Colour number `45`)
/// ## Representations:
/// - int tuple `(47, 79, 79)`
/// - float tuple `(0.18, 0, 0)`
/// - hex: `#2F4F4F`
pub const DARKSLATEGREY: Color = Color(47, 79, 79);

/// Colour `(r = 0, g = 206, b = 209)`
///
/// Colour `DARKTURQUOISE` from the set `CSS4_COLORS`. (Colour number `46`)
/// ## Representations:
/// - int tuple `(0, 206, 209)`
/// - float tuple `(0.0, 1, 1)`
/// - hex: `#00CED1`
pub const DARKTURQUOISE: Color = Color(0, 206, 209);

/// Colour `(r = 148, g = 0, b = 211)`
///
/// Colour `DARKVIOLET` from the set `CSS4_COLORS`. (Colour number `47`)
/// ## Representations:
/// - int tuple `(148, 0, 211)`
/// - float tuple `(0.58, 0, 1)`
/// - hex: `#9400D3`
pub const DARKVIOLET: Color = Color(148, 0, 211);

/// Colour `(r = 255, g = 20, b = 147)`
///
/// Colour `DEEPPINK` from the set `CSS4_COLORS`. (Colour number `48`)
/// ## Representations:
/// - int tuple `(255, 20, 147)`
/// - float tuple `(1.0, 0, 1)`
/// - hex: `#FF1493`
pub const DEEPPINK: Color = Color(255, 20, 147);

/// Colour `(r = 0, g = 191, b = 255)`
///
/// Colour `DEEPSKYBLUE` from the set `CSS4_COLORS`. (Colour number `49`)
/// ## Representations:
/// - int tuple `(0, 191, 255)`
/// - float tuple `(0.0, 1, 1)`
/// - hex: `#00BFFF`
pub const DEEPSKYBLUE: Color = Color(0, 191, 255);

/// Colour `(r = 105, g = 105, b = 105)`
///
/// Colour `DIMGRAY` from the set `CSS4_COLORS`. (Colour number `50`)
/// ## Representations:
/// - int tuple `(105, 105, 105)`
/// - float tuple `(0.41, 0, 0)`
/// - hex: `#696969`
pub const DIMGRAY: Color = Color(105, 105, 105);

/// Colour `(r = 105, g = 105, b = 105)`
///
/// Colour `DIMGREY` from the set `CSS4_COLORS`. (Colour number `51`)
/// ## Representations:
/// - int tuple `(105, 105, 105)`
/// - float tuple `(0.41, 0, 0)`
/// - hex: `#696969`
pub const DIMGREY: Color = Color(105, 105, 105);

/// Colour `(r = 30, g = 144, b = 255)`
///
/// Colour `DODGERBLUE` from the set `CSS4_COLORS`. (Colour number `52`)
/// ## Representations:
/// - int tuple `(30, 144, 255)`
/// - float tuple `(0.12, 1, 1)`
/// - hex: `#1E90FF`
pub const DODGERBLUE: Color = Color(30, 144, 255);

/// Colour `(r = 178, g = 34, b = 34)`
///
/// Colour `FIREBRICK` from the set `CSS4_COLORS`. (Colour number `53`)
/// ## Representations:
/// - int tuple `(178, 34, 34)`
/// - float tuple `(0.7, 0, 0)`
/// - hex: `#B22222`
pub const FIREBRICK: Color = Color(178, 34, 34);

/// Colour `(r = 255, g = 250, b = 240)`
///
/// Colour `FLORALWHITE` from the set `CSS4_COLORS`. (Colour number `54`)
/// ## Representations:
/// - int tuple `(255, 250, 240)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFFAF0`
pub const FLORALWHITE: Color = Color(255, 250, 240);

/// Colour `(r = 34, g = 139, b = 34)`
///
/// Colour `FORESTGREEN` from the set `CSS4_COLORS`. (Colour number `55`)
/// ## Representations:
/// - int tuple `(34, 139, 34)`
/// - float tuple `(0.13, 1, 0)`
/// - hex: `#228B22`
pub const FORESTGREEN: Color = Color(34, 139, 34);

/// Colour `(r = 255, g = 0, b = 255)`
///
/// Colour `FUCHSIA` from the set `CSS4_COLORS`. (Colour number `56`)
/// ## Representations:
/// - int tuple `(255, 0, 255)`
/// - float tuple `(1.0, 0, 1)`
/// - hex: `#FF00FF`
pub const FUCHSIA: Color = Color(255, 0, 255);

/// Colour `(r = 220, g = 220, b = 220)`
///
/// Colour `GAINSBORO` from the set `CSS4_COLORS`. (Colour number `57`)
/// ## Representations:
/// - int tuple `(220, 220, 220)`
/// - float tuple `(0.86, 1, 1)`
/// - hex: `#DCDCDC`
pub const GAINSBORO: Color = Color(220, 220, 220);

/// Colour `(r = 248, g = 248, b = 255)`
///
/// Colour `GHOSTWHITE` from the set `CSS4_COLORS`. (Colour number `58`)
/// ## Representations:
/// - int tuple `(248, 248, 255)`
/// - float tuple `(0.97, 1, 1)`
/// - hex: `#F8F8FF`
pub const GHOSTWHITE: Color = Color(248, 248, 255);

/// Colour `(r = 255, g = 215, b = 0)`
///
/// Colour `GOLD` from the set `CSS4_COLORS`. (Colour number `59`)
/// ## Representations:
/// - int tuple `(255, 215, 0)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFD700`
pub const GOLD: Color = Color(255, 215, 0);

/// Colour `(r = 218, g = 165, b = 32)`
///
/// Colour `GOLDENROD` from the set `CSS4_COLORS`. (Colour number `60`)
/// ## Representations:
/// - int tuple `(218, 165, 32)`
/// - float tuple `(0.85, 1, 0)`
/// - hex: `#DAA520`
pub const GOLDENROD: Color = Color(218, 165, 32);

/// Colour `(r = 128, g = 128, b = 128)`
///
/// Colour `GRAY` from the set `CSS4_COLORS`. (Colour number `61`)
/// ## Representations:
/// - int tuple `(128, 128, 128)`
/// - float tuple `(0.5, 1, 1)`
/// - hex: `#808080`
pub const GRAY: Color = Color(128, 128, 128);

/// Colour `(r = 0, g = 128, b = 0)`
///
/// Colour `GREEN` from the set `CSS4_COLORS`. (Colour number `62`)
/// ## Representations:
/// - int tuple `(0, 128, 0)`
/// - float tuple `(0.0, 1, 0)`
/// - hex: `#008000`
pub const GREEN: Color = Color(0, 128, 0);

/// Colour `(r = 173, g = 255, b = 47)`
///
/// Colour `GREENYELLOW` from the set `CSS4_COLORS`. (Colour number `63`)
/// ## Representations:
/// - int tuple `(173, 255, 47)`
/// - float tuple `(0.68, 1, 0)`
/// - hex: `#ADFF2F`
pub const GREENYELLOW: Color = Color(173, 255, 47);

/// Colour `(r = 128, g = 128, b = 128)`
///
/// Colour `GREY` from the set `CSS4_COLORS`. (Colour number `64`)
/// ## Representations:
/// - int tuple `(128, 128, 128)`
/// - float tuple `(0.5, 1, 1)`
/// - hex: `#808080`
pub const GREY: Color = Color(128, 128, 128);

/// Colour `(r = 240, g = 255, b = 240)`
///
/// Colour `HONEYDEW` from the set `CSS4_COLORS`. (Colour number `65`)
/// ## Representations:
/// - int tuple `(240, 255, 240)`
/// - float tuple `(0.94, 1, 1)`
/// - hex: `#F0FFF0`
pub const HONEYDEW: Color = Color(240, 255, 240);

/// Colour `(r = 255, g = 105, b = 180)`
///
/// Colour `HOTPINK` from the set `CSS4_COLORS`. (Colour number `66`)
/// ## Representations:
/// - int tuple `(255, 105, 180)`
/// - float tuple `(1.0, 0, 1)`
/// - hex: `#FF69B4`
pub const HOTPINK: Color = Color(255, 105, 180);

/// Colour `(r = 205, g = 92, b = 92)`
///
/// Colour `INDIANRED` from the set `CSS4_COLORS`. (Colour number `67`)
/// ## Representations:
/// - int tuple `(205, 92, 92)`
/// - float tuple `(0.8, 0, 0)`
/// - hex: `#CD5C5C`
pub const INDIANRED: Color = Color(205, 92, 92);

/// Colour `(r = 75, g = 0, b = 130)`
///
/// Colour `INDIGO` from the set `CSS4_COLORS`. (Colour number `68`)
/// ## Representations:
/// - int tuple `(75, 0, 130)`
/// - float tuple `(0.29, 0, 1)`
/// - hex: `#4B0082`
pub const INDIGO: Color = Color(75, 0, 130);

/// Colour `(r = 255, g = 255, b = 240)`
///
/// Colour `IVORY` from the set `CSS4_COLORS`. (Colour number `69`)
/// ## Representations:
/// - int tuple `(255, 255, 240)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFFFF0`
pub const IVORY: Color = Color(255, 255, 240);

/// Colour `(r = 240, g = 230, b = 140)`
///
/// Colour `KHAKI` from the set `CSS4_COLORS`. (Colour number `70`)
/// ## Representations:
/// - int tuple `(240, 230, 140)`
/// - float tuple `(0.94, 1, 1)`
/// - hex: `#F0E68C`
pub const KHAKI: Color = Color(240, 230, 140);

/// Colour `(r = 230, g = 230, b = 250)`
///
/// Colour `LAVENDER` from the set `CSS4_COLORS`. (Colour number `71`)
/// ## Representations:
/// - int tuple `(230, 230, 250)`
/// - float tuple `(0.9, 1, 1)`
/// - hex: `#E6E6FA`
pub const LAVENDER: Color = Color(230, 230, 250);

/// Colour `(r = 255, g = 240, b = 245)`
///
/// Colour `LAVENDERBLUSH` from the set `CSS4_COLORS`. (Colour number `72`)
/// ## Representations:
/// - int tuple `(255, 240, 245)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFF0F5`
pub const LAVENDERBLUSH: Color = Color(255, 240, 245);

/// Colour `(r = 124, g = 252, b = 0)`
///
/// Colour `LAWNGREEN` from the set `CSS4_COLORS`. (Colour number `73`)
/// ## Representations:
/// - int tuple `(124, 252, 0)`
/// - float tuple `(0.49, 1, 0)`
/// - hex: `#7CFC00`
pub const LAWNGREEN: Color = Color(124, 252, 0);

/// Colour `(r = 255, g = 250, b = 205)`
///
/// Colour `LEMONCHIFFON` from the set `CSS4_COLORS`. (Colour number `74`)
/// ## Representations:
/// - int tuple `(255, 250, 205)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFFACD`
pub const LEMONCHIFFON: Color = Color(255, 250, 205);

/// Colour `(r = 173, g = 216, b = 230)`
///
/// Colour `LIGHTBLUE` from the set `CSS4_COLORS`. (Colour number `75`)
/// ## Representations:
/// - int tuple `(173, 216, 230)`
/// - float tuple `(0.68, 1, 1)`
/// - hex: `#ADD8E6`
pub const LIGHTBLUE: Color = Color(173, 216, 230);

/// Colour `(r = 240, g = 128, b = 128)`
///
/// Colour `LIGHTCORAL` from the set `CSS4_COLORS`. (Colour number `76`)
/// ## Representations:
/// - int tuple `(240, 128, 128)`
/// - float tuple `(0.94, 1, 1)`
/// - hex: `#F08080`
pub const LIGHTCORAL: Color = Color(240, 128, 128);

/// Colour `(r = 224, g = 255, b = 255)`
///
/// Colour `LIGHTCYAN` from the set `CSS4_COLORS`. (Colour number `77`)
/// ## Representations:
/// - int tuple `(224, 255, 255)`
/// - float tuple `(0.88, 1, 1)`
/// - hex: `#E0FFFF`
pub const LIGHTCYAN: Color = Color(224, 255, 255);

/// Colour `(r = 250, g = 250, b = 210)`
///
/// Colour `LIGHTGOLDENRODYELLOW` from the set `CSS4_COLORS`. (Colour number `78`)
/// ## Representations:
/// - int tuple `(250, 250, 210)`
/// - float tuple `(0.98, 1, 1)`
/// - hex: `#FAFAD2`
pub const LIGHTGOLDENRODYELLOW: Color = Color(250, 250, 210);

/// Colour `(r = 211, g = 211, b = 211)`
///
/// Colour `LIGHTGRAY` from the set `CSS4_COLORS`. (Colour number `79`)
/// ## Representations:
/// - int tuple `(211, 211, 211)`
/// - float tuple `(0.83, 1, 1)`
/// - hex: `#D3D3D3`
pub const LIGHTGRAY: Color = Color(211, 211, 211);

/// Colour `(r = 144, g = 238, b = 144)`
///
/// Colour `LIGHTGREEN` from the set `CSS4_COLORS`. (Colour number `80`)
/// ## Representations:
/// - int tuple `(144, 238, 144)`
/// - float tuple `(0.56, 1, 1)`
/// - hex: `#90EE90`
pub const LIGHTGREEN: Color = Color(144, 238, 144);

/// Colour `(r = 211, g = 211, b = 211)`
///
/// Colour `LIGHTGREY` from the set `CSS4_COLORS`. (Colour number `81`)
/// ## Representations:
/// - int tuple `(211, 211, 211)`
/// - float tuple `(0.83, 1, 1)`
/// - hex: `#D3D3D3`
pub const LIGHTGREY: Color = Color(211, 211, 211);

/// Colour `(r = 255, g = 182, b = 193)`
///
/// Colour `LIGHTPINK` from the set `CSS4_COLORS`. (Colour number `82`)
/// ## Representations:
/// - int tuple `(255, 182, 193)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFB6C1`
pub const LIGHTPINK: Color = Color(255, 182, 193);

/// Colour `(r = 255, g = 160, b = 122)`
///
/// Colour `LIGHTSALMON` from the set `CSS4_COLORS`. (Colour number `83`)
/// ## Representations:
/// - int tuple `(255, 160, 122)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFA07A`
pub const LIGHTSALMON: Color = Color(255, 160, 122);

/// Colour `(r = 32, g = 178, b = 170)`
///
/// Colour `LIGHTSEAGREEN` from the set `CSS4_COLORS`. (Colour number `84`)
/// ## Representations:
/// - int tuple `(32, 178, 170)`
/// - float tuple `(0.13, 1, 1)`
/// - hex: `#20B2AA`
pub const LIGHTSEAGREEN: Color = Color(32, 178, 170);

/// Colour `(r = 135, g = 206, b = 250)`
///
/// Colour `LIGHTSKYBLUE` from the set `CSS4_COLORS`. (Colour number `85`)
/// ## Representations:
/// - int tuple `(135, 206, 250)`
/// - float tuple `(0.53, 1, 1)`
/// - hex: `#87CEFA`
pub const LIGHTSKYBLUE: Color = Color(135, 206, 250);

/// Colour `(r = 119, g = 136, b = 153)`
///
/// Colour `LIGHTSLATEGRAY` from the set `CSS4_COLORS`. (Colour number `86`)
/// ## Representations:
/// - int tuple `(119, 136, 153)`
/// - float tuple `(0.47, 1, 1)`
/// - hex: `#778899`
pub const LIGHTSLATEGRAY: Color = Color(119, 136, 153);

/// Colour `(r = 119, g = 136, b = 153)`
///
/// Colour `LIGHTSLATEGREY` from the set `CSS4_COLORS`. (Colour number `87`)
/// ## Representations:
/// - int tuple `(119, 136, 153)`
/// - float tuple `(0.47, 1, 1)`
/// - hex: `#778899`
pub const LIGHTSLATEGREY: Color = Color(119, 136, 153);

/// Colour `(r = 176, g = 196, b = 222)`
///
/// Colour `LIGHTSTEELBLUE` from the set `CSS4_COLORS`. (Colour number `88`)
/// ## Representations:
/// - int tuple `(176, 196, 222)`
/// - float tuple `(0.69, 1, 1)`
/// - hex: `#B0C4DE`
pub const LIGHTSTEELBLUE: Color = Color(176, 196, 222);

/// Colour `(r = 255, g = 255, b = 224)`
///
/// Colour `LIGHTYELLOW` from the set `CSS4_COLORS`. (Colour number `89`)
/// ## Representations:
/// - int tuple `(255, 255, 224)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFFFE0`
pub const LIGHTYELLOW: Color = Color(255, 255, 224);

/// Colour `(r = 0, g = 255, b = 0)`
///
/// Colour `LIME` from the set `CSS4_COLORS`. (Colour number `90`)
/// ## Representations:
/// - int tuple `(0, 255, 0)`
/// - float tuple `(0.0, 1, 0)`
/// - hex: `#00FF00`
pub const LIME: Color = Color(0, 255, 0);

/// Colour `(r = 50, g = 205, b = 50)`
///
/// Colour `LIMEGREEN` from the set `CSS4_COLORS`. (Colour number `91`)
/// ## Representations:
/// - int tuple `(50, 205, 50)`
/// - float tuple `(0.2, 1, 0)`
/// - hex: `#32CD32`
pub const LIMEGREEN: Color = Color(50, 205, 50);

/// Colour `(r = 250, g = 240, b = 230)`
///
/// Colour `LINEN` from the set `CSS4_COLORS`. (Colour number `92`)
/// ## Representations:
/// - int tuple `(250, 240, 230)`
/// - float tuple `(0.98, 1, 1)`
/// - hex: `#FAF0E6`
pub const LINEN: Color = Color(250, 240, 230);

/// Colour `(r = 255, g = 0, b = 255)`
///
/// Colour `MAGENTA` from the set `CSS4_COLORS`. (Colour number `93`)
/// ## Representations:
/// - int tuple `(255, 0, 255)`
/// - float tuple `(1.0, 0, 1)`
/// - hex: `#FF00FF`
pub const MAGENTA: Color = Color(255, 0, 255);

/// Colour `(r = 128, g = 0, b = 0)`
///
/// Colour `MAROON` from the set `CSS4_COLORS`. (Colour number `94`)
/// ## Representations:
/// - int tuple `(128, 0, 0)`
/// - float tuple `(0.5, 0, 0)`
/// - hex: `#800000`
pub const MAROON: Color = Color(128, 0, 0);

/// Colour `(r = 102, g = 205, b = 170)`
///
/// Colour `MEDIUMAQUAMARINE` from the set `CSS4_COLORS`. (Colour number `95`)
/// ## Representations:
/// - int tuple `(102, 205, 170)`
/// - float tuple `(0.4, 1, 1)`
/// - hex: `#66CDAA`
pub const MEDIUMAQUAMARINE: Color = Color(102, 205, 170);

/// Colour `(r = 0, g = 0, b = 205)`
///
/// Colour `MEDIUMBLUE` from the set `CSS4_COLORS`. (Colour number `96`)
/// ## Representations:
/// - int tuple `(0, 0, 205)`
/// - float tuple `(0.0, 0, 1)`
/// - hex: `#0000CD`
pub const MEDIUMBLUE: Color = Color(0, 0, 205);

/// Colour `(r = 186, g = 85, b = 211)`
///
/// Colour `MEDIUMORCHID` from the set `CSS4_COLORS`. (Colour number `97`)
/// ## Representations:
/// - int tuple `(186, 85, 211)`
/// - float tuple `(0.73, 0, 1)`
/// - hex: `#BA55D3`
pub const MEDIUMORCHID: Color = Color(186, 85, 211);

/// Colour `(r = 147, g = 112, b = 219)`
///
/// Colour `MEDIUMPURPLE` from the set `CSS4_COLORS`. (Colour number `98`)
/// ## Representations:
/// - int tuple `(147, 112, 219)`
/// - float tuple `(0.58, 0, 1)`
/// - hex: `#9370DB`
pub const MEDIUMPURPLE: Color = Color(147, 112, 219);

/// Colour `(r = 60, g = 179, b = 113)`
///
/// Colour `MEDIUMSEAGREEN` from the set `CSS4_COLORS`. (Colour number `99`)
/// ## Representations:
/// - int tuple `(60, 179, 113)`
/// - float tuple `(0.24, 1, 0)`
/// - hex: `#3CB371`
pub const MEDIUMSEAGREEN: Color = Color(60, 179, 113);

/// Colour `(r = 123, g = 104, b = 238)`
///
/// Colour `MEDIUMSLATEBLUE` from the set `CSS4_COLORS`. (Colour number `100`)
/// ## Representations:
/// - int tuple `(123, 104, 238)`
/// - float tuple `(0.48, 0, 1)`
/// - hex: `#7B68EE`
pub const MEDIUMSLATEBLUE: Color = Color(123, 104, 238);

/// Colour `(r = 0, g = 250, b = 154)`
///
/// Colour `MEDIUMSPRINGGREEN` from the set `CSS4_COLORS`. (Colour number `101`)
/// ## Representations:
/// - int tuple `(0, 250, 154)`
/// - float tuple `(0.0, 1, 1)`
/// - hex: `#00FA9A`
pub const MEDIUMSPRINGGREEN: Color = Color(0, 250, 154);

/// Colour `(r = 72, g = 209, b = 204)`
///
/// Colour `MEDIUMTURQUOISE` from the set `CSS4_COLORS`. (Colour number `102`)
/// ## Representations:
/// - int tuple `(72, 209, 204)`
/// - float tuple `(0.28, 1, 1)`
/// - hex: `#48D1CC`
pub const MEDIUMTURQUOISE: Color = Color(72, 209, 204);

/// Colour `(r = 199, g = 21, b = 133)`
///
/// Colour `MEDIUMVIOLETRED` from the set `CSS4_COLORS`. (Colour number `103`)
/// ## Representations:
/// - int tuple `(199, 21, 133)`
/// - float tuple `(0.78, 0, 1)`
/// - hex: `#C71585`
pub const MEDIUMVIOLETRED: Color = Color(199, 21, 133);

/// Colour `(r = 25, g = 25, b = 112)`
///
/// Colour `MIDNIGHTBLUE` from the set `CSS4_COLORS`. (Colour number `104`)
/// ## Representations:
/// - int tuple `(25, 25, 112)`
/// - float tuple `(0.1, 0, 0)`
/// - hex: `#191970`
pub const MIDNIGHTBLUE: Color = Color(25, 25, 112);

/// Colour `(r = 245, g = 255, b = 250)`
///
/// Colour `MINTCREAM` from the set `CSS4_COLORS`. (Colour number `105`)
/// ## Representations:
/// - int tuple `(245, 255, 250)`
/// - float tuple `(0.96, 1, 1)`
/// - hex: `#F5FFFA`
pub const MINTCREAM: Color = Color(245, 255, 250);

/// Colour `(r = 255, g = 228, b = 225)`
///
/// Colour `MISTYROSE` from the set `CSS4_COLORS`. (Colour number `106`)
/// ## Representations:
/// - int tuple `(255, 228, 225)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFE4E1`
pub const MISTYROSE: Color = Color(255, 228, 225);

/// Colour `(r = 255, g = 228, b = 181)`
///
/// Colour `MOCCASIN` from the set `CSS4_COLORS`. (Colour number `107`)
/// ## Representations:
/// - int tuple `(255, 228, 181)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFE4B5`
pub const MOCCASIN: Color = Color(255, 228, 181);

/// Colour `(r = 255, g = 222, b = 173)`
///
/// Colour `NAVAJOWHITE` from the set `CSS4_COLORS`. (Colour number `108`)
/// ## Representations:
/// - int tuple `(255, 222, 173)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFDEAD`
pub const NAVAJOWHITE: Color = Color(255, 222, 173);

/// Colour `(r = 0, g = 0, b = 128)`
///
/// Colour `NAVY` from the set `CSS4_COLORS`. (Colour number `109`)
/// ## Representations:
/// - int tuple `(0, 0, 128)`
/// - float tuple `(0.0, 0, 1)`
/// - hex: `#000080`
pub const NAVY: Color = Color(0, 0, 128);

/// Colour `(r = 253, g = 245, b = 230)`
///
/// Colour `OLDLACE` from the set `CSS4_COLORS`. (Colour number `110`)
/// ## Representations:
/// - int tuple `(253, 245, 230)`
/// - float tuple `(0.99, 1, 1)`
/// - hex: `#FDF5E6`
pub const OLDLACE: Color = Color(253, 245, 230);

/// Colour `(r = 128, g = 128, b = 0)`
///
/// Colour `OLIVE` from the set `CSS4_COLORS`. (Colour number `111`)
/// ## Representations:
/// - int tuple `(128, 128, 0)`
/// - float tuple `(0.5, 1, 0)`
/// - hex: `#808000`
pub const OLIVE: Color = Color(128, 128, 0);

/// Colour `(r = 107, g = 142, b = 35)`
///
/// Colour `OLIVEDRAB` from the set `CSS4_COLORS`. (Colour number `112`)
/// ## Representations:
/// - int tuple `(107, 142, 35)`
/// - float tuple `(0.42, 1, 0)`
/// - hex: `#6B8E23`
pub const OLIVEDRAB: Color = Color(107, 142, 35);

/// Colour `(r = 255, g = 165, b = 0)`
///
/// Colour `ORANGE` from the set `CSS4_COLORS`. (Colour number `113`)
/// ## Representations:
/// - int tuple `(255, 165, 0)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFA500`
pub const ORANGE: Color = Color(255, 165, 0);

/// Colour `(r = 255, g = 69, b = 0)`
///
/// Colour `ORANGERED` from the set `CSS4_COLORS`. (Colour number `114`)
/// ## Representations:
/// - int tuple `(255, 69, 0)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FF4500`
pub const ORANGERED: Color = Color(255, 69, 0);

/// Colour `(r = 218, g = 112, b = 214)`
///
/// Colour `ORCHID` from the set `CSS4_COLORS`. (Colour number `115`)
/// ## Representations:
/// - int tuple `(218, 112, 214)`
/// - float tuple `(0.85, 0, 1)`
/// - hex: `#DA70D6`
pub const ORCHID: Color = Color(218, 112, 214);

/// Colour `(r = 238, g = 232, b = 170)`
///
/// Colour `PALEGOLDENROD` from the set `CSS4_COLORS`. (Colour number `116`)
/// ## Representations:
/// - int tuple `(238, 232, 170)`
/// - float tuple `(0.93, 1, 1)`
/// - hex: `#EEE8AA`
pub const PALEGOLDENROD: Color = Color(238, 232, 170);

/// Colour `(r = 152, g = 251, b = 152)`
///
/// Colour `PALEGREEN` from the set `CSS4_COLORS`. (Colour number `117`)
/// ## Representations:
/// - int tuple `(152, 251, 152)`
/// - float tuple `(0.6, 1, 1)`
/// - hex: `#98FB98`
pub const PALEGREEN: Color = Color(152, 251, 152);

/// Colour `(r = 175, g = 238, b = 238)`
///
/// Colour `PALETURQUOISE` from the set `CSS4_COLORS`. (Colour number `118`)
/// ## Representations:
/// - int tuple `(175, 238, 238)`
/// - float tuple `(0.69, 1, 1)`
/// - hex: `#AFEEEE`
pub const PALETURQUOISE: Color = Color(175, 238, 238);

/// Colour `(r = 219, g = 112, b = 147)`
///
/// Colour `PALEVIOLETRED` from the set `CSS4_COLORS`. (Colour number `119`)
/// ## Representations:
/// - int tuple `(219, 112, 147)`
/// - float tuple `(0.86, 0, 1)`
/// - hex: `#DB7093`
pub const PALEVIOLETRED: Color = Color(219, 112, 147);

/// Colour `(r = 255, g = 239, b = 213)`
///
/// Colour `PAPAYAWHIP` from the set `CSS4_COLORS`. (Colour number `120`)
/// ## Representations:
/// - int tuple `(255, 239, 213)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFEFD5`
pub const PAPAYAWHIP: Color = Color(255, 239, 213);

/// Colour `(r = 255, g = 218, b = 185)`
///
/// Colour `PEACHPUFF` from the set `CSS4_COLORS`. (Colour number `121`)
/// ## Representations:
/// - int tuple `(255, 218, 185)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFDAB9`
pub const PEACHPUFF: Color = Color(255, 218, 185);

/// Colour `(r = 205, g = 133, b = 63)`
///
/// Colour `PERU` from the set `CSS4_COLORS`. (Colour number `122`)
/// ## Representations:
/// - int tuple `(205, 133, 63)`
/// - float tuple `(0.8, 1, 0)`
/// - hex: `#CD853F`
pub const PERU: Color = Color(205, 133, 63);

/// Colour `(r = 255, g = 192, b = 203)`
///
/// Colour `PINK` from the set `CSS4_COLORS`. (Colour number `123`)
/// ## Representations:
/// - int tuple `(255, 192, 203)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFC0CB`
pub const PINK: Color = Color(255, 192, 203);

/// Colour `(r = 221, g = 160, b = 221)`
///
/// Colour `PLUM` from the set `CSS4_COLORS`. (Colour number `124`)
/// ## Representations:
/// - int tuple `(221, 160, 221)`
/// - float tuple `(0.87, 1, 1)`
/// - hex: `#DDA0DD`
pub const PLUM: Color = Color(221, 160, 221);

/// Colour `(r = 176, g = 224, b = 230)`
///
/// Colour `POWDERBLUE` from the set `CSS4_COLORS`. (Colour number `125`)
/// ## Representations:
/// - int tuple `(176, 224, 230)`
/// - float tuple `(0.69, 1, 1)`
/// - hex: `#B0E0E6`
pub const POWDERBLUE: Color = Color(176, 224, 230);

/// Colour `(r = 128, g = 0, b = 128)`
///
/// Colour `PURPLE` from the set `CSS4_COLORS`. (Colour number `126`)
/// ## Representations:
/// - int tuple `(128, 0, 128)`
/// - float tuple `(0.5, 0, 1)`
/// - hex: `#800080`
pub const PURPLE: Color = Color(128, 0, 128);

/// Colour `(r = 102, g = 51, b = 153)`
///
/// Colour `REBECCAPURPLE` from the set `CSS4_COLORS`. (Colour number `127`)
/// ## Representations:
/// - int tuple `(102, 51, 153)`
/// - float tuple `(0.4, 0, 1)`
/// - hex: `#663399`
pub const REBECCAPURPLE: Color = Color(102, 51, 153);

/// Colour `(r = 255, g = 0, b = 0)`
///
/// Colour `RED` from the set `CSS4_COLORS`. (Colour number `128`)
/// ## Representations:
/// - int tuple `(255, 0, 0)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FF0000`
pub const RED: Color = Color(255, 0, 0);

/// Colour `(r = 188, g = 143, b = 143)`
///
/// Colour `ROSYBROWN` from the set `CSS4_COLORS`. (Colour number `129`)
/// ## Representations:
/// - int tuple `(188, 143, 143)`
/// - float tuple `(0.74, 1, 1)`
/// - hex: `#BC8F8F`
pub const ROSYBROWN: Color = Color(188, 143, 143);

/// Colour `(r = 65, g = 105, b = 225)`
///
/// Colour `ROYALBLUE` from the set `CSS4_COLORS`. (Colour number `130`)
/// ## Representations:
/// - int tuple `(65, 105, 225)`
/// - float tuple `(0.25, 0, 1)`
/// - hex: `#4169E1`
pub const ROYALBLUE: Color = Color(65, 105, 225);

/// Colour `(r = 139, g = 69, b = 19)`
///
/// Colour `SADDLEBROWN` from the set `CSS4_COLORS`. (Colour number `131`)
/// ## Representations:
/// - int tuple `(139, 69, 19)`
/// - float tuple `(0.55, 0, 0)`
/// - hex: `#8B4513`
pub const SADDLEBROWN: Color = Color(139, 69, 19);

/// Colour `(r = 250, g = 128, b = 114)`
///
/// Colour `SALMON` from the set `CSS4_COLORS`. (Colour number `132`)
/// ## Representations:
/// - int tuple `(250, 128, 114)`
/// - float tuple `(0.98, 1, 0)`
/// - hex: `#FA8072`
pub const SALMON: Color = Color(250, 128, 114);

/// Colour `(r = 244, g = 164, b = 96)`
///
/// Colour `SANDYBROWN` from the set `CSS4_COLORS`. (Colour number `133`)
/// ## Representations:
/// - int tuple `(244, 164, 96)`
/// - float tuple `(0.96, 1, 0)`
/// - hex: `#F4A460`
pub const SANDYBROWN: Color = Color(244, 164, 96);

/// Colour `(r = 46, g = 139, b = 87)`
///
/// Colour `SEAGREEN` from the set `CSS4_COLORS`. (Colour number `134`)
/// ## Representations:
/// - int tuple `(46, 139, 87)`
/// - float tuple `(0.18, 1, 0)`
/// - hex: `#2E8B57`
pub const SEAGREEN: Color = Color(46, 139, 87);

/// Colour `(r = 255, g = 245, b = 238)`
///
/// Colour `SEASHELL` from the set `CSS4_COLORS`. (Colour number `135`)
/// ## Representations:
/// - int tuple `(255, 245, 238)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFF5EE`
pub const SEASHELL: Color = Color(255, 245, 238);

/// Colour `(r = 160, g = 82, b = 45)`
///
/// Colour `SIENNA` from the set `CSS4_COLORS`. (Colour number `136`)
/// ## Representations:
/// - int tuple `(160, 82, 45)`
/// - float tuple `(0.63, 0, 0)`
/// - hex: `#A0522D`
pub const SIENNA: Color = Color(160, 82, 45);

/// Colour `(r = 192, g = 192, b = 192)`
///
/// Colour `SILVER` from the set `CSS4_COLORS`. (Colour number `137`)
/// ## Representations:
/// - int tuple `(192, 192, 192)`
/// - float tuple `(0.75, 1, 1)`
/// - hex: `#C0C0C0`
pub const SILVER: Color = Color(192, 192, 192);

/// Colour `(r = 135, g = 206, b = 235)`
///
/// Colour `SKYBLUE` from the set `CSS4_COLORS`. (Colour number `138`)
/// ## Representations:
/// - int tuple `(135, 206, 235)`
/// - float tuple `(0.53, 1, 1)`
/// - hex: `#87CEEB`
pub const SKYBLUE: Color = Color(135, 206, 235);

/// Colour `(r = 106, g = 90, b = 205)`
///
/// Colour `SLATEBLUE` from the set `CSS4_COLORS`. (Colour number `139`)
/// ## Representations:
/// - int tuple `(106, 90, 205)`
/// - float tuple `(0.42, 0, 1)`
/// - hex: `#6A5ACD`
pub const SLATEBLUE: Color = Color(106, 90, 205);

/// Colour `(r = 112, g = 128, b = 144)`
///
/// Colour `SLATEGRAY` from the set `CSS4_COLORS`. (Colour number `140`)
/// ## Representations:
/// - int tuple `(112, 128, 144)`
/// - float tuple `(0.44, 1, 1)`
/// - hex: `#708090`
pub const SLATEGRAY: Color = Color(112, 128, 144);

/// Colour `(r = 112, g = 128, b = 144)`
///
/// Colour `SLATEGREY` from the set `CSS4_COLORS`. (Colour number `141`)
/// ## Representations:
/// - int tuple `(112, 128, 144)`
/// - float tuple `(0.44, 1, 1)`
/// - hex: `#708090`
pub const SLATEGREY: Color = Color(112, 128, 144);

/// Colour `(r = 255, g = 250, b = 250)`
///
/// Colour `SNOW` from the set `CSS4_COLORS`. (Colour number `142`)
/// ## Representations:
/// - int tuple `(255, 250, 250)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFFAFA`
pub const SNOW: Color = Color(255, 250, 250);

/// Colour `(r = 0, g = 255, b = 127)`
///
/// Colour `SPRINGGREEN` from the set `CSS4_COLORS`. (Colour number `143`)
/// ## Representations:
/// - int tuple `(0, 255, 127)`
/// - float tuple `(0.0, 1, 0)`
/// - hex: `#00FF7F`
pub const SPRINGGREEN: Color = Color(0, 255, 127);

/// Colour `(r = 70, g = 130, b = 180)`
///
/// Colour `STEELBLUE` from the set `CSS4_COLORS`. (Colour number `144`)
/// ## Representations:
/// - int tuple `(70, 130, 180)`
/// - float tuple `(0.27, 1, 1)`
/// - hex: `#4682B4`
pub const STEELBLUE: Color = Color(70, 130, 180);

/// Colour `(r = 210, g = 180, b = 140)`
///
/// Colour `TAN` from the set `CSS4_COLORS`. (Colour number `145`)
/// ## Representations:
/// - int tuple `(210, 180, 140)`
/// - float tuple `(0.82, 1, 1)`
/// - hex: `#D2B48C`
pub const TAN: Color = Color(210, 180, 140);

/// Colour `(r = 0, g = 128, b = 128)`
///
/// Colour `TEAL` from the set `CSS4_COLORS`. (Colour number `146`)
/// ## Representations:
/// - int tuple `(0, 128, 128)`
/// - float tuple `(0.0, 1, 1)`
/// - hex: `#008080`
pub const TEAL: Color = Color(0, 128, 128);

/// Colour `(r = 216, g = 191, b = 216)`
///
/// Colour `THISTLE` from the set `CSS4_COLORS`. (Colour number `147`)
/// ## Representations:
/// - int tuple `(216, 191, 216)`
/// - float tuple `(0.85, 1, 1)`
/// - hex: `#D8BFD8`
pub const THISTLE: Color = Color(216, 191, 216);

/// Colour `(r = 255, g = 99, b = 71)`
///
/// Colour `TOMATO` from the set `CSS4_COLORS`. (Colour number `148`)
/// ## Representations:
/// - int tuple `(255, 99, 71)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FF6347`
pub const TOMATO: Color = Color(255, 99, 71);

/// Colour `(r = 64, g = 224, b = 208)`
///
/// Colour `TURQUOISE` from the set `CSS4_COLORS`. (Colour number `149`)
/// ## Representations:
/// - int tuple `(64, 224, 208)`
/// - float tuple `(0.25, 1, 1)`
/// - hex: `#40E0D0`
pub const TURQUOISE: Color = Color(64, 224, 208);

/// Colour `(r = 238, g = 130, b = 238)`
///
/// Colour `VIOLET` from the set `CSS4_COLORS`. (Colour number `150`)
/// ## Representations:
/// - int tuple `(238, 130, 238)`
/// - float tuple `(0.93, 1, 1)`
/// - hex: `#EE82EE`
pub const VIOLET: Color = Color(238, 130, 238);

/// Colour `(r = 245, g = 222, b = 179)`
///
/// Colour `WHEAT` from the set `CSS4_COLORS`. (Colour number `151`)
/// ## Representations:
/// - int tuple `(245, 222, 179)`
/// - float tuple `(0.96, 1, 1)`
/// - hex: `#F5DEB3`
pub const WHEAT: Color = Color(245, 222, 179);

/// Colour `(r = 255, g = 255, b = 255)`
///
/// Colour `WHITE` from the set `CSS4_COLORS`. (Colour number `152`)
/// ## Representations:
/// - int tuple `(255, 255, 255)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFFFFF`
pub const WHITE: Color = Color(255, 255, 255);

/// Colour `(r = 245, g = 245, b = 245)`
///
/// Colour `WHITESMOKE` from the set `CSS4_COLORS`. (Colour number `153`)
/// ## Representations:
/// - int tuple `(245, 245, 245)`
/// - float tuple `(0.96, 1, 1)`
/// - hex: `#F5F5F5`
pub const WHITESMOKE: Color = Color(245, 245, 245);

/// Colour `(r = 255, g = 255, b = 0)`
///
/// Colour `YELLOW` from the set `CSS4_COLORS`. (Colour number `154`)
/// ## Representations:
/// - int tuple `(255, 255, 0)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFFF00`
pub const YELLOW: Color = Color(255, 255, 0);

/// Colour `(r = 154, g = 205, b = 50)`
///
/// Colour `YELLOWGREEN` from the set `CSS4_COLORS`. (Colour number `155`)
/// ## Representations:
/// - int tuple `(154, 205, 50)`
/// - float tuple `(0.6, 1, 0)`
/// - hex: `#9ACD32`
pub const YELLOWGREEN: Color = Color(154, 205, 50);

/// Colour `(r = 172, g = 194, b = 217)`
///
/// Colour `XKCD_CLOUDY_BLUE` from the set `CSS4_COLORS`. (Colour number `156`)
/// ## Representations:
/// - int tuple `(172, 194, 217)`
/// - float tuple `(0.67, 1, 1)`
/// - hex: `#ACC2D9`
pub const XKCD_CLOUDY_BLUE: Color = Color(172, 194, 217);

/// Colour `(r = 86, g = 174, b = 87)`
///
/// Colour `XKCD_DARK_PASTEL_GREEN` from the set `CSS4_COLORS`. (Colour number `157`)
/// ## Representations:
/// - int tuple `(86, 174, 87)`
/// - float tuple `(0.34, 1, 0)`
/// - hex: `#56AE57`
pub const XKCD_DARK_PASTEL_GREEN: Color = Color(86, 174, 87);

/// Colour `(r = 178, g = 153, b = 110)`
///
/// Colour `XKCD_DUST` from the set `CSS4_COLORS`. (Colour number `158`)
/// ## Representations:
/// - int tuple `(178, 153, 110)`
/// - float tuple `(0.7, 1, 0)`
/// - hex: `#B2996E`
pub const XKCD_DUST: Color = Color(178, 153, 110);

/// Colour `(r = 168, g = 255, b = 4)`
///
/// Colour `XKCD_ELECTRIC_LIME` from the set `CSS4_COLORS`. (Colour number `159`)
/// ## Representations:
/// - int tuple `(168, 255, 4)`
/// - float tuple `(0.66, 1, 0)`
/// - hex: `#A8FF04`
pub const XKCD_ELECTRIC_LIME: Color = Color(168, 255, 4);

/// Colour `(r = 105, g = 216, b = 79)`
///
/// Colour `XKCD_FRESH_GREEN` from the set `CSS4_COLORS`. (Colour number `160`)
/// ## Representations:
/// - int tuple `(105, 216, 79)`
/// - float tuple `(0.41, 1, 0)`
/// - hex: `#69D84F`
pub const XKCD_FRESH_GREEN: Color = Color(105, 216, 79);

/// Colour `(r = 137, g = 69, b = 133)`
///
/// Colour `XKCD_LIGHT_EGGPLANT` from the set `CSS4_COLORS`. (Colour number `161`)
/// ## Representations:
/// - int tuple `(137, 69, 133)`
/// - float tuple `(0.54, 0, 1)`
/// - hex: `#894585`
pub const XKCD_LIGHT_EGGPLANT: Color = Color(137, 69, 133);

/// Colour `(r = 112, g = 178, b = 63)`
///
/// Colour `XKCD_NASTY_GREEN` from the set `CSS4_COLORS`. (Colour number `162`)
/// ## Representations:
/// - int tuple `(112, 178, 63)`
/// - float tuple `(0.44, 1, 0)`
/// - hex: `#70B23F`
pub const XKCD_NASTY_GREEN: Color = Color(112, 178, 63);

/// Colour `(r = 212, g = 255, b = 255)`
///
/// Colour `XKCD_REALLY_LIGHT_BLUE` from the set `CSS4_COLORS`. (Colour number `163`)
/// ## Representations:
/// - int tuple `(212, 255, 255)`
/// - float tuple `(0.83, 1, 1)`
/// - hex: `#D4FFFF`
pub const XKCD_REALLY_LIGHT_BLUE: Color = Color(212, 255, 255);

/// Colour `(r = 101, g = 171, b = 124)`
///
/// Colour `XKCD_TEA` from the set `CSS4_COLORS`. (Colour number `164`)
/// ## Representations:
/// - int tuple `(101, 171, 124)`
/// - float tuple `(0.4, 1, 0)`
/// - hex: `#65AB7C`
pub const XKCD_TEA: Color = Color(101, 171, 124);

/// Colour `(r = 149, g = 46, b = 143)`
///
/// Colour `XKCD_WARM_PURPLE` from the set `CSS4_COLORS`. (Colour number `165`)
/// ## Representations:
/// - int tuple `(149, 46, 143)`
/// - float tuple `(0.58, 0, 1)`
/// - hex: `#952E8F`
pub const XKCD_WARM_PURPLE: Color = Color(149, 46, 143);

/// Colour `(r = 252, g = 252, b = 129)`
///
/// Colour `XKCD_YELLOWISH_TAN` from the set `CSS4_COLORS`. (Colour number `166`)
/// ## Representations:
/// - int tuple `(252, 252, 129)`
/// - float tuple `(0.99, 1, 1)`
/// - hex: `#FCFC81`
pub const XKCD_YELLOWISH_TAN: Color = Color(252, 252, 129);

/// Colour `(r = 165, g = 163, b = 145)`
///
/// Colour `XKCD_CEMENT` from the set `CSS4_COLORS`. (Colour number `167`)
/// ## Representations:
/// - int tuple `(165, 163, 145)`
/// - float tuple `(0.65, 1, 1)`
/// - hex: `#A5A391`
pub const XKCD_CEMENT: Color = Color(165, 163, 145);

/// Colour `(r = 56, g = 128, b = 4)`
///
/// Colour `XKCD_DARK_GRASS_GREEN` from the set `CSS4_COLORS`. (Colour number `168`)
/// ## Representations:
/// - int tuple `(56, 128, 4)`
/// - float tuple `(0.22, 1, 0)`
/// - hex: `#388004`
pub const XKCD_DARK_GRASS_GREEN: Color = Color(56, 128, 4);

/// Colour `(r = 76, g = 144, b = 133)`
///
/// Colour `XKCD_DUSTY_TEAL` from the set `CSS4_COLORS`. (Colour number `169`)
/// ## Representations:
/// - int tuple `(76, 144, 133)`
/// - float tuple `(0.3, 1, 1)`
/// - hex: `#4C9085`
pub const XKCD_DUSTY_TEAL: Color = Color(76, 144, 133);

/// Colour `(r = 94, g = 155, b = 138)`
///
/// Colour `XKCD_GREY_TEAL` from the set `CSS4_COLORS`. (Colour number `170`)
/// ## Representations:
/// - int tuple `(94, 155, 138)`
/// - float tuple `(0.37, 1, 1)`
/// - hex: `#5E9B8A`
pub const XKCD_GREY_TEAL: Color = Color(94, 155, 138);

/// Colour `(r = 239, g = 180, b = 53)`
///
/// Colour `XKCD_MACARONI_AND_CHEESE` from the set `CSS4_COLORS`. (Colour number `171`)
/// ## Representations:
/// - int tuple `(239, 180, 53)`
/// - float tuple `(0.94, 1, 0)`
/// - hex: `#EFB435`
pub const XKCD_MACARONI_AND_CHEESE: Color = Color(239, 180, 53);

/// Colour `(r = 217, g = 155, b = 130)`
///
/// Colour `XKCD_PINKISH_TAN` from the set `CSS4_COLORS`. (Colour number `172`)
/// ## Representations:
/// - int tuple `(217, 155, 130)`
/// - float tuple `(0.85, 1, 1)`
/// - hex: `#D99B82`
pub const XKCD_PINKISH_TAN: Color = Color(217, 155, 130);

/// Colour `(r = 10, g = 95, b = 56)`
///
/// Colour `XKCD_SPRUCE` from the set `CSS4_COLORS`. (Colour number `173`)
/// ## Representations:
/// - int tuple `(10, 95, 56)`
/// - float tuple `(0.04, 0, 0)`
/// - hex: `#0A5F38`
pub const XKCD_SPRUCE: Color = Color(10, 95, 56);

/// Colour `(r = 12, g = 6, b = 247)`
///
/// Colour `XKCD_STRONG_BLUE` from the set `CSS4_COLORS`. (Colour number `174`)
/// ## Representations:
/// - int tuple `(12, 6, 247)`
/// - float tuple `(0.05, 0, 1)`
/// - hex: `#0C06F7`
pub const XKCD_STRONG_BLUE: Color = Color(12, 6, 247);

/// Colour `(r = 97, g = 222, b = 42)`
///
/// Colour `XKCD_TOXIC_GREEN` from the set `CSS4_COLORS`. (Colour number `175`)
/// ## Representations:
/// - int tuple `(97, 222, 42)`
/// - float tuple `(0.38, 1, 0)`
/// - hex: `#61DE2A`
pub const XKCD_TOXIC_GREEN: Color = Color(97, 222, 42);

/// Colour `(r = 55, g = 120, b = 191)`
///
/// Colour `XKCD_WINDOWS_BLUE` from the set `CSS4_COLORS`. (Colour number `176`)
/// ## Representations:
/// - int tuple `(55, 120, 191)`
/// - float tuple `(0.22, 0, 1)`
/// - hex: `#3778BF`
pub const XKCD_WINDOWS_BLUE: Color = Color(55, 120, 191);

/// Colour `(r = 34, g = 66, b = 199)`
///
/// Colour `XKCD_BLUE_BLUE` from the set `CSS4_COLORS`. (Colour number `177`)
/// ## Representations:
/// - int tuple `(34, 66, 199)`
/// - float tuple `(0.13, 0, 1)`
/// - hex: `#2242C7`
pub const XKCD_BLUE_BLUE: Color = Color(34, 66, 199);

/// Colour `(r = 83, g = 60, b = 198)`
///
/// Colour `XKCD_BLUE_WITH_A_HINT_OF_PURPLE` from the set `CSS4_COLORS`. (Colour number `178`)
/// ## Representations:
/// - int tuple `(83, 60, 198)`
/// - float tuple `(0.33, 0, 1)`
/// - hex: `#533CC6`
pub const XKCD_BLUE_WITH_A_HINT_OF_PURPLE: Color = Color(83, 60, 198);

/// Colour `(r = 155, g = 181, b = 60)`
///
/// Colour `XKCD_BOOGER` from the set `CSS4_COLORS`. (Colour number `179`)
/// ## Representations:
/// - int tuple `(155, 181, 60)`
/// - float tuple `(0.61, 1, 0)`
/// - hex: `#9BB53C`
pub const XKCD_BOOGER: Color = Color(155, 181, 60);

/// Colour `(r = 5, g = 255, b = 166)`
///
/// Colour `XKCD_BRIGHT_SEA_GREEN` from the set `CSS4_COLORS`. (Colour number `180`)
/// ## Representations:
/// - int tuple `(5, 255, 166)`
/// - float tuple `(0.02, 1, 1)`
/// - hex: `#05FFA6`
pub const XKCD_BRIGHT_SEA_GREEN: Color = Color(5, 255, 166);

/// Colour `(r = 31, g = 99, b = 87)`
///
/// Colour `XKCD_DARK_GREEN_BLUE` from the set `CSS4_COLORS`. (Colour number `181`)
/// ## Representations:
/// - int tuple `(31, 99, 87)`
/// - float tuple `(0.12, 0, 0)`
/// - hex: `#1F6357`
pub const XKCD_DARK_GREEN_BLUE: Color = Color(31, 99, 87);

/// Colour `(r = 1, g = 115, b = 116)`
///
/// Colour `XKCD_DEEP_TURQUOISE` from the set `CSS4_COLORS`. (Colour number `182`)
/// ## Representations:
/// - int tuple `(1, 115, 116)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#017374`
pub const XKCD_DEEP_TURQUOISE: Color = Color(1, 115, 116);

/// Colour `(r = 12, g = 181, b = 119)`
///
/// Colour `XKCD_GREEN_TEAL` from the set `CSS4_COLORS`. (Colour number `183`)
/// ## Representations:
/// - int tuple `(12, 181, 119)`
/// - float tuple `(0.05, 1, 0)`
/// - hex: `#0CB577`
pub const XKCD_GREEN_TEAL: Color = Color(12, 181, 119);

/// Colour `(r = 255, g = 7, b = 137)`
///
/// Colour `XKCD_STRONG_PINK` from the set `CSS4_COLORS`. (Colour number `184`)
/// ## Representations:
/// - int tuple `(255, 7, 137)`
/// - float tuple `(1.0, 0, 1)`
/// - hex: `#FF0789`
pub const XKCD_STRONG_PINK: Color = Color(255, 7, 137);

/// Colour `(r = 175, g = 168, b = 139)`
///
/// Colour `XKCD_BLAND` from the set `CSS4_COLORS`. (Colour number `185`)
/// ## Representations:
/// - int tuple `(175, 168, 139)`
/// - float tuple `(0.69, 1, 1)`
/// - hex: `#AFA88B`
pub const XKCD_BLAND: Color = Color(175, 168, 139);

/// Colour `(r = 8, g = 120, b = 127)`
///
/// Colour `XKCD_DEEP_AQUA` from the set `CSS4_COLORS`. (Colour number `186`)
/// ## Representations:
/// - int tuple `(8, 120, 127)`
/// - float tuple `(0.03, 0, 0)`
/// - hex: `#08787F`
pub const XKCD_DEEP_AQUA: Color = Color(8, 120, 127);

/// Colour `(r = 221, g = 133, b = 215)`
///
/// Colour `XKCD_LAVENDER_PINK` from the set `CSS4_COLORS`. (Colour number `187`)
/// ## Representations:
/// - int tuple `(221, 133, 215)`
/// - float tuple `(0.87, 1, 1)`
/// - hex: `#DD85D7`
pub const XKCD_LAVENDER_PINK: Color = Color(221, 133, 215);

/// Colour `(r = 166, g = 200, b = 117)`
///
/// Colour `XKCD_LIGHT_MOSS_GREEN` from the set `CSS4_COLORS`. (Colour number `188`)
/// ## Representations:
/// - int tuple `(166, 200, 117)`
/// - float tuple `(0.65, 1, 0)`
/// - hex: `#A6C875`
pub const XKCD_LIGHT_MOSS_GREEN: Color = Color(166, 200, 117);

/// Colour `(r = 167, g = 255, b = 181)`
///
/// Colour `XKCD_LIGHT_SEAFOAM_GREEN` from the set `CSS4_COLORS`. (Colour number `189`)
/// ## Representations:
/// - int tuple `(167, 255, 181)`
/// - float tuple `(0.65, 1, 1)`
/// - hex: `#A7FFB5`
pub const XKCD_LIGHT_SEAFOAM_GREEN: Color = Color(167, 255, 181);

/// Colour `(r = 194, g = 183, b = 9)`
///
/// Colour `XKCD_OLIVE_YELLOW` from the set `CSS4_COLORS`. (Colour number `190`)
/// ## Representations:
/// - int tuple `(194, 183, 9)`
/// - float tuple `(0.76, 1, 0)`
/// - hex: `#C2B709`
pub const XKCD_OLIVE_YELLOW: Color = Color(194, 183, 9);

/// Colour `(r = 231, g = 142, b = 165)`
///
/// Colour `XKCD_PIG_PINK` from the set `CSS4_COLORS`. (Colour number `191`)
/// ## Representations:
/// - int tuple `(231, 142, 165)`
/// - float tuple `(0.91, 1, 1)`
/// - hex: `#E78EA5`
pub const XKCD_PIG_PINK: Color = Color(231, 142, 165);

/// Colour `(r = 150, g = 110, b = 189)`
///
/// Colour `XKCD_DEEP_LILAC` from the set `CSS4_COLORS`. (Colour number `192`)
/// ## Representations:
/// - int tuple `(150, 110, 189)`
/// - float tuple `(0.59, 0, 1)`
/// - hex: `#966EBD`
pub const XKCD_DEEP_LILAC: Color = Color(150, 110, 189);

/// Colour `(r = 204, g = 173, b = 96)`
///
/// Colour `XKCD_DESERT` from the set `CSS4_COLORS`. (Colour number `193`)
/// ## Representations:
/// - int tuple `(204, 173, 96)`
/// - float tuple `(0.8, 1, 0)`
/// - hex: `#CCAD60`
pub const XKCD_DESERT: Color = Color(204, 173, 96);

/// Colour `(r = 172, g = 134, b = 168)`
///
/// Colour `XKCD_DUSTY_LAVENDER` from the set `CSS4_COLORS`. (Colour number `194`)
/// ## Representations:
/// - int tuple `(172, 134, 168)`
/// - float tuple `(0.67, 1, 1)`
/// - hex: `#AC86A8`
pub const XKCD_DUSTY_LAVENDER: Color = Color(172, 134, 168);

/// Colour `(r = 148, g = 126, b = 148)`
///
/// Colour `XKCD_PURPLEY_GREY` from the set `CSS4_COLORS`. (Colour number `195`)
/// ## Representations:
/// - int tuple `(148, 126, 148)`
/// - float tuple `(0.58, 0, 1)`
/// - hex: `#947E94`
pub const XKCD_PURPLEY_GREY: Color = Color(148, 126, 148);

/// Colour `(r = 152, g = 63, b = 178)`
///
/// Colour `XKCD_PURPLY` from the set `CSS4_COLORS`. (Colour number `196`)
/// ## Representations:
/// - int tuple `(152, 63, 178)`
/// - float tuple `(0.6, 0, 1)`
/// - hex: `#983FB2`
pub const XKCD_PURPLY: Color = Color(152, 63, 178);

/// Colour `(r = 255, g = 99, b = 233)`
///
/// Colour `XKCD_CANDY_PINK` from the set `CSS4_COLORS`. (Colour number `197`)
/// ## Representations:
/// - int tuple `(255, 99, 233)`
/// - float tuple `(1.0, 0, 1)`
/// - hex: `#FF63E9`
pub const XKCD_CANDY_PINK: Color = Color(255, 99, 233);

/// Colour `(r = 178, g = 251, b = 165)`
///
/// Colour `XKCD_LIGHT_PASTEL_GREEN` from the set `CSS4_COLORS`. (Colour number `198`)
/// ## Representations:
/// - int tuple `(178, 251, 165)`
/// - float tuple `(0.7, 1, 1)`
/// - hex: `#B2FBA5`
pub const XKCD_LIGHT_PASTEL_GREEN: Color = Color(178, 251, 165);

/// Colour `(r = 99, g = 179, b = 101)`
///
/// Colour `XKCD_BORING_GREEN` from the set `CSS4_COLORS`. (Colour number `199`)
/// ## Representations:
/// - int tuple `(99, 179, 101)`
/// - float tuple `(0.39, 1, 0)`
/// - hex: `#63B365`
pub const XKCD_BORING_GREEN: Color = Color(99, 179, 101);

/// Colour `(r = 142, g = 229, b = 63)`
///
/// Colour `XKCD_KIWI_GREEN` from the set `CSS4_COLORS`. (Colour number `200`)
/// ## Representations:
/// - int tuple `(142, 229, 63)`
/// - float tuple `(0.56, 1, 0)`
/// - hex: `#8EE53F`
pub const XKCD_KIWI_GREEN: Color = Color(142, 229, 63);

/// Colour `(r = 183, g = 225, b = 161)`
///
/// Colour `XKCD_LIGHT_GREY_GREEN` from the set `CSS4_COLORS`. (Colour number `201`)
/// ## Representations:
/// - int tuple `(183, 225, 161)`
/// - float tuple `(0.72, 1, 1)`
/// - hex: `#B7E1A1`
pub const XKCD_LIGHT_GREY_GREEN: Color = Color(183, 225, 161);

/// Colour `(r = 255, g = 111, b = 82)`
///
/// Colour `XKCD_ORANGE_PINK` from the set `CSS4_COLORS`. (Colour number `202`)
/// ## Representations:
/// - int tuple `(255, 111, 82)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FF6F52`
pub const XKCD_ORANGE_PINK: Color = Color(255, 111, 82);

/// Colour `(r = 189, g = 248, b = 163)`
///
/// Colour `XKCD_TEA_GREEN` from the set `CSS4_COLORS`. (Colour number `203`)
/// ## Representations:
/// - int tuple `(189, 248, 163)`
/// - float tuple `(0.74, 1, 1)`
/// - hex: `#BDF8A3`
pub const XKCD_TEA_GREEN: Color = Color(189, 248, 163);

/// Colour `(r = 211, g = 182, b = 131)`
///
/// Colour `XKCD_VERY_LIGHT_BROWN` from the set `CSS4_COLORS`. (Colour number `204`)
/// ## Representations:
/// - int tuple `(211, 182, 131)`
/// - float tuple `(0.83, 1, 1)`
/// - hex: `#D3B683`
pub const XKCD_VERY_LIGHT_BROWN: Color = Color(211, 182, 131);

/// Colour `(r = 255, g = 252, b = 196)`
///
/// Colour `XKCD_EGG_SHELL` from the set `CSS4_COLORS`. (Colour number `205`)
/// ## Representations:
/// - int tuple `(255, 252, 196)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFFCC4`
pub const XKCD_EGG_SHELL: Color = Color(255, 252, 196);

/// Colour `(r = 67, g = 5, b = 65)`
///
/// Colour `XKCD_EGGPLANT_PURPLE` from the set `CSS4_COLORS`. (Colour number `206`)
/// ## Representations:
/// - int tuple `(67, 5, 65)`
/// - float tuple `(0.26, 0, 0)`
/// - hex: `#430541`
pub const XKCD_EGGPLANT_PURPLE: Color = Color(67, 5, 65);

/// Colour `(r = 255, g = 178, b = 208)`
///
/// Colour `XKCD_POWDER_PINK` from the set `CSS4_COLORS`. (Colour number `207`)
/// ## Representations:
/// - int tuple `(255, 178, 208)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFB2D0`
pub const XKCD_POWDER_PINK: Color = Color(255, 178, 208);

/// Colour `(r = 153, g = 117, b = 112)`
///
/// Colour `XKCD_REDDISH_GREY` from the set `CSS4_COLORS`. (Colour number `208`)
/// ## Representations:
/// - int tuple `(153, 117, 112)`
/// - float tuple `(0.6, 0, 0)`
/// - hex: `#997570`
pub const XKCD_REDDISH_GREY: Color = Color(153, 117, 112);

/// Colour `(r = 173, g = 144, b = 13)`
///
/// Colour `XKCD_BABY_SHIT_BROWN` from the set `CSS4_COLORS`. (Colour number `209`)
/// ## Representations:
/// - int tuple `(173, 144, 13)`
/// - float tuple `(0.68, 1, 0)`
/// - hex: `#AD900D`
pub const XKCD_BABY_SHIT_BROWN: Color = Color(173, 144, 13);

/// Colour `(r = 196, g = 142, b = 253)`
///
/// Colour `XKCD_LILIAC` from the set `CSS4_COLORS`. (Colour number `210`)
/// ## Representations:
/// - int tuple `(196, 142, 253)`
/// - float tuple `(0.77, 1, 1)`
/// - hex: `#C48EFD`
pub const XKCD_LILIAC: Color = Color(196, 142, 253);

/// Colour `(r = 80, g = 123, b = 156)`
///
/// Colour `XKCD_STORMY_BLUE` from the set `CSS4_COLORS`. (Colour number `211`)
/// ## Representations:
/// - int tuple `(80, 123, 156)`
/// - float tuple `(0.31, 0, 1)`
/// - hex: `#507B9C`
pub const XKCD_STORMY_BLUE: Color = Color(80, 123, 156);

/// Colour `(r = 125, g = 113, b = 3)`
///
/// Colour `XKCD_UGLY_BROWN` from the set `CSS4_COLORS`. (Colour number `212`)
/// ## Representations:
/// - int tuple `(125, 113, 3)`
/// - float tuple `(0.49, 0, 0)`
/// - hex: `#7D7103`
pub const XKCD_UGLY_BROWN: Color = Color(125, 113, 3);

/// Colour `(r = 255, g = 253, b = 120)`
///
/// Colour `XKCD_CUSTARD` from the set `CSS4_COLORS`. (Colour number `213`)
/// ## Representations:
/// - int tuple `(255, 253, 120)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFFD78`
pub const XKCD_CUSTARD: Color = Color(255, 253, 120);

/// Colour `(r = 218, g = 70, b = 125)`
///
/// Colour `XKCD_DARKISH_PINK` from the set `CSS4_COLORS`. (Colour number `214`)
/// ## Representations:
/// - int tuple `(218, 70, 125)`
/// - float tuple `(0.85, 0, 0)`
/// - hex: `#DA467D`
pub const XKCD_DARKISH_PINK: Color = Color(218, 70, 125);

/// Colour `(r = 65, g = 2, b = 0)`
///
/// Colour `XKCD_DEEP_BROWN` from the set `CSS4_COLORS`. (Colour number `215`)
/// ## Representations:
/// - int tuple `(65, 2, 0)`
/// - float tuple `(0.25, 0, 0)`
/// - hex: `#410200`
pub const XKCD_DEEP_BROWN: Color = Color(65, 2, 0);

/// Colour `(r = 201, g = 209, b = 121)`
///
/// Colour `XKCD_GREENISH_BEIGE` from the set `CSS4_COLORS`. (Colour number `216`)
/// ## Representations:
/// - int tuple `(201, 209, 121)`
/// - float tuple `(0.79, 1, 0)`
/// - hex: `#C9D179`
pub const XKCD_GREENISH_BEIGE: Color = Color(201, 209, 121);

/// Colour `(r = 255, g = 250, b = 134)`
///
/// Colour `XKCD_MANILLA` from the set `CSS4_COLORS`. (Colour number `217`)
/// ## Representations:
/// - int tuple `(255, 250, 134)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFFA86`
pub const XKCD_MANILLA: Color = Color(255, 250, 134);

/// Colour `(r = 86, g = 132, b = 174)`
///
/// Colour `XKCD_OFF_BLUE` from the set `CSS4_COLORS`. (Colour number `218`)
/// ## Representations:
/// - int tuple `(86, 132, 174)`
/// - float tuple `(0.34, 1, 1)`
/// - hex: `#5684AE`
pub const XKCD_OFF_BLUE: Color = Color(86, 132, 174);

/// Colour `(r = 107, g = 124, b = 133)`
///
/// Colour `XKCD_BATTLESHIP_GREY` from the set `CSS4_COLORS`. (Colour number `219`)
/// ## Representations:
/// - int tuple `(107, 124, 133)`
/// - float tuple `(0.42, 0, 1)`
/// - hex: `#6B7C85`
pub const XKCD_BATTLESHIP_GREY: Color = Color(107, 124, 133);

/// Colour `(r = 111, g = 108, b = 10)`
///
/// Colour `XKCD_BROWNY_GREEN` from the set `CSS4_COLORS`. (Colour number `220`)
/// ## Representations:
/// - int tuple `(111, 108, 10)`
/// - float tuple `(0.44, 0, 0)`
/// - hex: `#6F6C0A`
pub const XKCD_BROWNY_GREEN: Color = Color(111, 108, 10);

/// Colour `(r = 126, g = 64, b = 113)`
///
/// Colour `XKCD_BRUISE` from the set `CSS4_COLORS`. (Colour number `221`)
/// ## Representations:
/// - int tuple `(126, 64, 113)`
/// - float tuple `(0.49, 0, 0)`
/// - hex: `#7E4071`
pub const XKCD_BRUISE: Color = Color(126, 64, 113);

/// Colour `(r = 0, g = 147, b = 55)`
///
/// Colour `XKCD_KELLEY_GREEN` from the set `CSS4_COLORS`. (Colour number `222`)
/// ## Representations:
/// - int tuple `(0, 147, 55)`
/// - float tuple `(0.0, 1, 0)`
/// - hex: `#009337`
pub const XKCD_KELLEY_GREEN: Color = Color(0, 147, 55);

/// Colour `(r = 208, g = 228, b = 41)`
///
/// Colour `XKCD_SICKLY_YELLOW` from the set `CSS4_COLORS`. (Colour number `223`)
/// ## Representations:
/// - int tuple `(208, 228, 41)`
/// - float tuple `(0.82, 1, 0)`
/// - hex: `#D0E429`
pub const XKCD_SICKLY_YELLOW: Color = Color(208, 228, 41);

/// Colour `(r = 255, g = 249, b = 23)`
///
/// Colour `XKCD_SUNNY_YELLOW` from the set `CSS4_COLORS`. (Colour number `224`)
/// ## Representations:
/// - int tuple `(255, 249, 23)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFF917`
pub const XKCD_SUNNY_YELLOW: Color = Color(255, 249, 23);

/// Colour `(r = 29, g = 93, b = 236)`
///
/// Colour `XKCD_AZUL` from the set `CSS4_COLORS`. (Colour number `225`)
/// ## Representations:
/// - int tuple `(29, 93, 236)`
/// - float tuple `(0.11, 0, 1)`
/// - hex: `#1D5DEC`
pub const XKCD_AZUL: Color = Color(29, 93, 236);

/// Colour `(r = 5, g = 73, b = 7)`
///
/// Colour `XKCD_DARKGREEN` from the set `CSS4_COLORS`. (Colour number `226`)
/// ## Representations:
/// - int tuple `(5, 73, 7)`
/// - float tuple `(0.02, 0, 0)`
/// - hex: `#054907`
pub const XKCD_DARKGREEN: Color = Color(5, 73, 7);

/// Colour `(r = 181, g = 206, b = 8)`
///
/// Colour `XKCD_GREEN_SLASH_YELLOW` from the set `CSS4_COLORS`. (Colour number `227`)
/// ## Representations:
/// - int tuple `(181, 206, 8)`
/// - float tuple `(0.71, 1, 0)`
/// - hex: `#B5CE08`
pub const XKCD_GREEN_SLASH_YELLOW: Color = Color(181, 206, 8);

/// Colour `(r = 143, g = 182, b = 123)`
///
/// Colour `XKCD_LICHEN` from the set `CSS4_COLORS`. (Colour number `228`)
/// ## Representations:
/// - int tuple `(143, 182, 123)`
/// - float tuple `(0.56, 1, 0)`
/// - hex: `#8FB67B`
pub const XKCD_LICHEN: Color = Color(143, 182, 123);

/// Colour `(r = 200, g = 255, b = 176)`
///
/// Colour `XKCD_LIGHT_LIGHT_GREEN` from the set `CSS4_COLORS`. (Colour number `229`)
/// ## Representations:
/// - int tuple `(200, 255, 176)`
/// - float tuple `(0.78, 1, 1)`
/// - hex: `#C8FFB0`
pub const XKCD_LIGHT_LIGHT_GREEN: Color = Color(200, 255, 176);

/// Colour `(r = 253, g = 222, b = 108)`
///
/// Colour `XKCD_PALE_GOLD` from the set `CSS4_COLORS`. (Colour number `230`)
/// ## Representations:
/// - int tuple `(253, 222, 108)`
/// - float tuple `(0.99, 1, 0)`
/// - hex: `#FDDE6C`
pub const XKCD_PALE_GOLD: Color = Color(253, 222, 108);

/// Colour `(r = 255, g = 223, b = 34)`
///
/// Colour `XKCD_SUN_YELLOW` from the set `CSS4_COLORS`. (Colour number `231`)
/// ## Representations:
/// - int tuple `(255, 223, 34)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFDF22`
pub const XKCD_SUN_YELLOW: Color = Color(255, 223, 34);

/// Colour `(r = 169, g = 190, b = 112)`
///
/// Colour `XKCD_TAN_GREEN` from the set `CSS4_COLORS`. (Colour number `232`)
/// ## Representations:
/// - int tuple `(169, 190, 112)`
/// - float tuple `(0.66, 1, 0)`
/// - hex: `#A9BE70`
pub const XKCD_TAN_GREEN: Color = Color(169, 190, 112);

/// Colour `(r = 104, g = 50, b = 227)`
///
/// Colour `XKCD_BURPLE` from the set `CSS4_COLORS`. (Colour number `233`)
/// ## Representations:
/// - int tuple `(104, 50, 227)`
/// - float tuple `(0.41, 0, 1)`
/// - hex: `#6832E3`
pub const XKCD_BURPLE: Color = Color(104, 50, 227);

/// Colour `(r = 253, g = 177, b = 71)`
///
/// Colour `XKCD_BUTTERSCOTCH` from the set `CSS4_COLORS`. (Colour number `234`)
/// ## Representations:
/// - int tuple `(253, 177, 71)`
/// - float tuple `(0.99, 1, 0)`
/// - hex: `#FDB147`
pub const XKCD_BUTTERSCOTCH: Color = Color(253, 177, 71);

/// Colour `(r = 199, g = 172, b = 125)`
///
/// Colour `XKCD_TOUPE` from the set `CSS4_COLORS`. (Colour number `235`)
/// ## Representations:
/// - int tuple `(199, 172, 125)`
/// - float tuple `(0.78, 1, 0)`
/// - hex: `#C7AC7D`
pub const XKCD_TOUPE: Color = Color(199, 172, 125);

/// Colour `(r = 255, g = 243, b = 154)`
///
/// Colour `XKCD_DARK_CREAM` from the set `CSS4_COLORS`. (Colour number `236`)
/// ## Representations:
/// - int tuple `(255, 243, 154)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFF39A`
pub const XKCD_DARK_CREAM: Color = Color(255, 243, 154);

/// Colour `(r = 133, g = 14, b = 4)`
///
/// Colour `XKCD_INDIAN_RED` from the set `CSS4_COLORS`. (Colour number `237`)
/// ## Representations:
/// - int tuple `(133, 14, 4)`
/// - float tuple `(0.52, 0, 0)`
/// - hex: `#850E04`
pub const XKCD_INDIAN_RED: Color = Color(133, 14, 4);

/// Colour `(r = 239, g = 192, b = 254)`
///
/// Colour `XKCD_LIGHT_LAVENDAR` from the set `CSS4_COLORS`. (Colour number `238`)
/// ## Representations:
/// - int tuple `(239, 192, 254)`
/// - float tuple `(0.94, 1, 1)`
/// - hex: `#EFC0FE`
pub const XKCD_LIGHT_LAVENDAR: Color = Color(239, 192, 254);

/// Colour `(r = 64, g = 253, b = 20)`
///
/// Colour `XKCD_POISON_GREEN` from the set `CSS4_COLORS`. (Colour number `239`)
/// ## Representations:
/// - int tuple `(64, 253, 20)`
/// - float tuple `(0.25, 1, 0)`
/// - hex: `#40FD14`
pub const XKCD_POISON_GREEN: Color = Color(64, 253, 20);

/// Colour `(r = 182, g = 196, b = 6)`
///
/// Colour `XKCD_BABY_PUKE_GREEN` from the set `CSS4_COLORS`. (Colour number `240`)
/// ## Representations:
/// - int tuple `(182, 196, 6)`
/// - float tuple `(0.71, 1, 0)`
/// - hex: `#B6C406`
pub const XKCD_BABY_PUKE_GREEN: Color = Color(182, 196, 6);

/// Colour `(r = 157, g = 255, b = 0)`
///
/// Colour `XKCD_BRIGHT_YELLOW_GREEN` from the set `CSS4_COLORS`. (Colour number `241`)
/// ## Representations:
/// - int tuple `(157, 255, 0)`
/// - float tuple `(0.62, 1, 0)`
/// - hex: `#9DFF00`
pub const XKCD_BRIGHT_YELLOW_GREEN: Color = Color(157, 255, 0);

/// Colour `(r = 60, g = 65, b = 66)`
///
/// Colour `XKCD_CHARCOAL_GREY` from the set `CSS4_COLORS`. (Colour number `242`)
/// ## Representations:
/// - int tuple `(60, 65, 66)`
/// - float tuple `(0.24, 0, 0)`
/// - hex: `#3C4142`
pub const XKCD_CHARCOAL_GREY: Color = Color(60, 65, 66);

/// Colour `(r = 242, g = 171, b = 21)`
///
/// Colour `XKCD_SQUASH` from the set `CSS4_COLORS`. (Colour number `243`)
/// ## Representations:
/// - int tuple `(242, 171, 21)`
/// - float tuple `(0.95, 1, 0)`
/// - hex: `#F2AB15`
pub const XKCD_SQUASH: Color = Color(242, 171, 21);

/// Colour `(r = 172, g = 79, b = 6)`
///
/// Colour `XKCD_CINNAMON` from the set `CSS4_COLORS`. (Colour number `244`)
/// ## Representations:
/// - int tuple `(172, 79, 6)`
/// - float tuple `(0.67, 0, 0)`
/// - hex: `#AC4F06`
pub const XKCD_CINNAMON: Color = Color(172, 79, 6);

/// Colour `(r = 196, g = 254, b = 130)`
///
/// Colour `XKCD_LIGHT_PEA_GREEN` from the set `CSS4_COLORS`. (Colour number `245`)
/// ## Representations:
/// - int tuple `(196, 254, 130)`
/// - float tuple `(0.77, 1, 1)`
/// - hex: `#C4FE82`
pub const XKCD_LIGHT_PEA_GREEN: Color = Color(196, 254, 130);

/// Colour `(r = 44, g = 250, b = 31)`
///
/// Colour `XKCD_RADIOACTIVE_GREEN` from the set `CSS4_COLORS`. (Colour number `246`)
/// ## Representations:
/// - int tuple `(44, 250, 31)`
/// - float tuple `(0.17, 1, 0)`
/// - hex: `#2CFA1F`
pub const XKCD_RADIOACTIVE_GREEN: Color = Color(44, 250, 31);

/// Colour `(r = 154, g = 98, b = 0)`
///
/// Colour `XKCD_RAW_SIENNA` from the set `CSS4_COLORS`. (Colour number `247`)
/// ## Representations:
/// - int tuple `(154, 98, 0)`
/// - float tuple `(0.6, 0, 0)`
/// - hex: `#9A6200`
pub const XKCD_RAW_SIENNA: Color = Color(154, 98, 0);

/// Colour `(r = 202, g = 155, b = 247)`
///
/// Colour `XKCD_BABY_PURPLE` from the set `CSS4_COLORS`. (Colour number `248`)
/// ## Representations:
/// - int tuple `(202, 155, 247)`
/// - float tuple `(0.79, 1, 1)`
/// - hex: `#CA9BF7`
pub const XKCD_BABY_PURPLE: Color = Color(202, 155, 247);

/// Colour `(r = 135, g = 95, b = 66)`
///
/// Colour `XKCD_COCOA` from the set `CSS4_COLORS`. (Colour number `249`)
/// ## Representations:
/// - int tuple `(135, 95, 66)`
/// - float tuple `(0.53, 0, 0)`
/// - hex: `#875F42`
pub const XKCD_COCOA: Color = Color(135, 95, 66);

/// Colour `(r = 58, g = 46, b = 254)`
///
/// Colour `XKCD_LIGHT_ROYAL_BLUE` from the set `CSS4_COLORS`. (Colour number `250`)
/// ## Representations:
/// - int tuple `(58, 46, 254)`
/// - float tuple `(0.23, 0, 1)`
/// - hex: `#3A2EFE`
pub const XKCD_LIGHT_ROYAL_BLUE: Color = Color(58, 46, 254);

/// Colour `(r = 253, g = 141, b = 73)`
///
/// Colour `XKCD_ORANGEISH` from the set `CSS4_COLORS`. (Colour number `251`)
/// ## Representations:
/// - int tuple `(253, 141, 73)`
/// - float tuple `(0.99, 1, 0)`
/// - hex: `#FD8D49`
pub const XKCD_ORANGEISH: Color = Color(253, 141, 73);

/// Colour `(r = 139, g = 49, b = 3)`
///
/// Colour `XKCD_RUST_BROWN` from the set `CSS4_COLORS`. (Colour number `252`)
/// ## Representations:
/// - int tuple `(139, 49, 3)`
/// - float tuple `(0.55, 0, 0)`
/// - hex: `#8B3103`
pub const XKCD_RUST_BROWN: Color = Color(139, 49, 3);

/// Colour `(r = 203, g = 165, b = 96)`
///
/// Colour `XKCD_SAND_BROWN` from the set `CSS4_COLORS`. (Colour number `253`)
/// ## Representations:
/// - int tuple `(203, 165, 96)`
/// - float tuple `(0.8, 1, 0)`
/// - hex: `#CBA560`
pub const XKCD_SAND_BROWN: Color = Color(203, 165, 96);

/// Colour `(r = 105, g = 131, b = 57)`
///
/// Colour `XKCD_SWAMP` from the set `CSS4_COLORS`. (Colour number `254`)
/// ## Representations:
/// - int tuple `(105, 131, 57)`
/// - float tuple `(0.41, 1, 0)`
/// - hex: `#698339`
pub const XKCD_SWAMP: Color = Color(105, 131, 57);

/// Colour `(r = 12, g = 220, b = 115)`
///
/// Colour `XKCD_TEALISH_GREEN` from the set `CSS4_COLORS`. (Colour number `255`)
/// ## Representations:
/// - int tuple `(12, 220, 115)`
/// - float tuple `(0.05, 1, 0)`
/// - hex: `#0CDC73`
pub const XKCD_TEALISH_GREEN: Color = Color(12, 220, 115);

/// Colour `(r = 183, g = 82, b = 3)`
///
/// Colour `XKCD_BURNT_SIENA` from the set `CSS4_COLORS`. (Colour number `256`)
/// ## Representations:
/// - int tuple `(183, 82, 3)`
/// - float tuple `(0.72, 0, 0)`
/// - hex: `#B75203`
pub const XKCD_BURNT_SIENA: Color = Color(183, 82, 3);

/// Colour `(r = 127, g = 143, b = 78)`
///
/// Colour `XKCD_CAMO` from the set `CSS4_COLORS`. (Colour number `257`)
/// ## Representations:
/// - int tuple `(127, 143, 78)`
/// - float tuple `(0.5, 1, 0)`
/// - hex: `#7F8F4E`
pub const XKCD_CAMO: Color = Color(127, 143, 78);

/// Colour `(r = 38, g = 83, b = 141)`
///
/// Colour `XKCD_DUSK_BLUE` from the set `CSS4_COLORS`. (Colour number `258`)
/// ## Representations:
/// - int tuple `(38, 83, 141)`
/// - float tuple `(0.15, 0, 1)`
/// - hex: `#26538D`
pub const XKCD_DUSK_BLUE: Color = Color(38, 83, 141);

/// Colour `(r = 99, g = 169, b = 80)`
///
/// Colour `XKCD_FERN` from the set `CSS4_COLORS`. (Colour number `259`)
/// ## Representations:
/// - int tuple `(99, 169, 80)`
/// - float tuple `(0.39, 1, 0)`
/// - hex: `#63A950`
pub const XKCD_FERN: Color = Color(99, 169, 80);

/// Colour `(r = 200, g = 127, b = 137)`
///
/// Colour `XKCD_OLD_ROSE` from the set `CSS4_COLORS`. (Colour number `260`)
/// ## Representations:
/// - int tuple `(200, 127, 137)`
/// - float tuple `(0.78, 0, 1)`
/// - hex: `#C87F89`
pub const XKCD_OLD_ROSE: Color = Color(200, 127, 137);

/// Colour `(r = 177, g = 252, b = 153)`
///
/// Colour `XKCD_PALE_LIGHT_GREEN` from the set `CSS4_COLORS`. (Colour number `261`)
/// ## Representations:
/// - int tuple `(177, 252, 153)`
/// - float tuple `(0.69, 1, 1)`
/// - hex: `#B1FC99`
pub const XKCD_PALE_LIGHT_GREEN: Color = Color(177, 252, 153);

/// Colour `(r = 255, g = 154, b = 138)`
///
/// Colour `XKCD_PEACHY_PINK` from the set `CSS4_COLORS`. (Colour number `262`)
/// ## Representations:
/// - int tuple `(255, 154, 138)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FF9A8A`
pub const XKCD_PEACHY_PINK: Color = Color(255, 154, 138);

/// Colour `(r = 246, g = 104, b = 142)`
///
/// Colour `XKCD_ROSY_PINK` from the set `CSS4_COLORS`. (Colour number `263`)
/// ## Representations:
/// - int tuple `(246, 104, 142)`
/// - float tuple `(0.96, 0, 1)`
/// - hex: `#F6688E`
pub const XKCD_ROSY_PINK: Color = Color(246, 104, 142);

/// Colour `(r = 118, g = 253, b = 168)`
///
/// Colour `XKCD_LIGHT_BLUISH_GREEN` from the set `CSS4_COLORS`. (Colour number `264`)
/// ## Representations:
/// - int tuple `(118, 253, 168)`
/// - float tuple `(0.46, 1, 1)`
/// - hex: `#76FDA8`
pub const XKCD_LIGHT_BLUISH_GREEN: Color = Color(118, 253, 168);

/// Colour `(r = 83, g = 254, b = 92)`
///
/// Colour `XKCD_LIGHT_BRIGHT_GREEN` from the set `CSS4_COLORS`. (Colour number `265`)
/// ## Representations:
/// - int tuple `(83, 254, 92)`
/// - float tuple `(0.33, 1, 0)`
/// - hex: `#53FE5C`
pub const XKCD_LIGHT_BRIGHT_GREEN: Color = Color(83, 254, 92);

/// Colour `(r = 78, g = 253, b = 84)`
///
/// Colour `XKCD_LIGHT_NEON_GREEN` from the set `CSS4_COLORS`. (Colour number `266`)
/// ## Representations:
/// - int tuple `(78, 253, 84)`
/// - float tuple `(0.31, 1, 0)`
/// - hex: `#4EFD54`
pub const XKCD_LIGHT_NEON_GREEN: Color = Color(78, 253, 84);

/// Colour `(r = 160, g = 254, b = 191)`
///
/// Colour `XKCD_LIGHT_SEAFOAM` from the set `CSS4_COLORS`. (Colour number `267`)
/// ## Representations:
/// - int tuple `(160, 254, 191)`
/// - float tuple `(0.63, 1, 1)`
/// - hex: `#A0FEBF`
pub const XKCD_LIGHT_SEAFOAM: Color = Color(160, 254, 191);

/// Colour `(r = 123, g = 242, b = 218)`
///
/// Colour `XKCD_TIFFANY_BLUE` from the set `CSS4_COLORS`. (Colour number `268`)
/// ## Representations:
/// - int tuple `(123, 242, 218)`
/// - float tuple `(0.48, 1, 1)`
/// - hex: `#7BF2DA`
pub const XKCD_TIFFANY_BLUE: Color = Color(123, 242, 218);

/// Colour `(r = 188, g = 245, b = 166)`
///
/// Colour `XKCD_WASHED_OUT_GREEN` from the set `CSS4_COLORS`. (Colour number `269`)
/// ## Representations:
/// - int tuple `(188, 245, 166)`
/// - float tuple `(0.74, 1, 1)`
/// - hex: `#BCF5A6`
pub const XKCD_WASHED_OUT_GREEN: Color = Color(188, 245, 166);

/// Colour `(r = 202, g = 107, b = 2)`
///
/// Colour `XKCD_BROWNY_ORANGE` from the set `CSS4_COLORS`. (Colour number `270`)
/// ## Representations:
/// - int tuple `(202, 107, 2)`
/// - float tuple `(0.79, 0, 0)`
/// - hex: `#CA6B02`
pub const XKCD_BROWNY_ORANGE: Color = Color(202, 107, 2);

/// Colour `(r = 16, g = 122, b = 176)`
///
/// Colour `XKCD_NICE_BLUE` from the set `CSS4_COLORS`. (Colour number `271`)
/// ## Representations:
/// - int tuple `(16, 122, 176)`
/// - float tuple `(0.06, 0, 1)`
/// - hex: `#107AB0`
pub const XKCD_NICE_BLUE: Color = Color(16, 122, 176);

/// Colour `(r = 33, g = 56, b = 171)`
///
/// Colour `XKCD_SAPPHIRE` from the set `CSS4_COLORS`. (Colour number `272`)
/// ## Representations:
/// - int tuple `(33, 56, 171)`
/// - float tuple `(0.13, 0, 1)`
/// - hex: `#2138AB`
pub const XKCD_SAPPHIRE: Color = Color(33, 56, 171);

/// Colour `(r = 113, g = 159, b = 145)`
///
/// Colour `XKCD_GREYISH_TEAL` from the set `CSS4_COLORS`. (Colour number `273`)
/// ## Representations:
/// - int tuple `(113, 159, 145)`
/// - float tuple `(0.44, 1, 1)`
/// - hex: `#719F91`
pub const XKCD_GREYISH_TEAL: Color = Color(113, 159, 145);

/// Colour `(r = 253, g = 185, b = 21)`
///
/// Colour `XKCD_ORANGEY_YELLOW` from the set `CSS4_COLORS`. (Colour number `274`)
/// ## Representations:
/// - int tuple `(253, 185, 21)`
/// - float tuple `(0.99, 1, 0)`
/// - hex: `#FDB915`
pub const XKCD_ORANGEY_YELLOW: Color = Color(253, 185, 21);

/// Colour `(r = 254, g = 252, b = 175)`
///
/// Colour `XKCD_PARCHMENT` from the set `CSS4_COLORS`. (Colour number `275`)
/// ## Representations:
/// - int tuple `(254, 252, 175)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FEFCAF`
pub const XKCD_PARCHMENT: Color = Color(254, 252, 175);

/// Colour `(r = 252, g = 246, b = 121)`
///
/// Colour `XKCD_STRAW` from the set `CSS4_COLORS`. (Colour number `276`)
/// ## Representations:
/// - int tuple `(252, 246, 121)`
/// - float tuple `(0.99, 1, 0)`
/// - hex: `#FCF679`
pub const XKCD_STRAW: Color = Color(252, 246, 121);

/// Colour `(r = 29, g = 2, b = 0)`
///
/// Colour `XKCD_VERY_DARK_BROWN` from the set `CSS4_COLORS`. (Colour number `277`)
/// ## Representations:
/// - int tuple `(29, 2, 0)`
/// - float tuple `(0.11, 0, 0)`
/// - hex: `#1D0200`
pub const XKCD_VERY_DARK_BROWN: Color = Color(29, 2, 0);

/// Colour `(r = 203, g = 104, b = 67)`
///
/// Colour `XKCD_TERRACOTA` from the set `CSS4_COLORS`. (Colour number `278`)
/// ## Representations:
/// - int tuple `(203, 104, 67)`
/// - float tuple `(0.8, 0, 0)`
/// - hex: `#CB6843`
pub const XKCD_TERRACOTA: Color = Color(203, 104, 67);

/// Colour `(r = 49, g = 102, b = 138)`
///
/// Colour `XKCD_UGLY_BLUE` from the set `CSS4_COLORS`. (Colour number `279`)
/// ## Representations:
/// - int tuple `(49, 102, 138)`
/// - float tuple `(0.19, 0, 1)`
/// - hex: `#31668A`
pub const XKCD_UGLY_BLUE: Color = Color(49, 102, 138);

/// Colour `(r = 36, g = 122, b = 253)`
///
/// Colour `XKCD_CLEAR_BLUE` from the set `CSS4_COLORS`. (Colour number `280`)
/// ## Representations:
/// - int tuple `(36, 122, 253)`
/// - float tuple `(0.14, 0, 1)`
/// - hex: `#247AFD`
pub const XKCD_CLEAR_BLUE: Color = Color(36, 122, 253);

/// Colour `(r = 255, g = 255, b = 182)`
///
/// Colour `XKCD_CREME` from the set `CSS4_COLORS`. (Colour number `281`)
/// ## Representations:
/// - int tuple `(255, 255, 182)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFFFB6`
pub const XKCD_CREME: Color = Color(255, 255, 182);

/// Colour `(r = 144, g = 253, b = 169)`
///
/// Colour `XKCD_FOAM_GREEN` from the set `CSS4_COLORS`. (Colour number `282`)
/// ## Representations:
/// - int tuple `(144, 253, 169)`
/// - float tuple `(0.56, 1, 1)`
/// - hex: `#90FDA9`
pub const XKCD_FOAM_GREEN: Color = Color(144, 253, 169);

/// Colour `(r = 134, g = 161, b = 125)`
///
/// Colour `XKCD_GREY_SLASH_GREEN` from the set `CSS4_COLORS`. (Colour number `283`)
/// ## Representations:
/// - int tuple `(134, 161, 125)`
/// - float tuple `(0.53, 1, 0)`
/// - hex: `#86A17D`
pub const XKCD_GREY_SLASH_GREEN: Color = Color(134, 161, 125);

/// Colour `(r = 253, g = 220, b = 92)`
///
/// Colour `XKCD_LIGHT_GOLD` from the set `CSS4_COLORS`. (Colour number `284`)
/// ## Representations:
/// - int tuple `(253, 220, 92)`
/// - float tuple `(0.99, 1, 0)`
/// - hex: `#FDDC5C`
pub const XKCD_LIGHT_GOLD: Color = Color(253, 220, 92);

/// Colour `(r = 120, g = 209, b = 182)`
///
/// Colour `XKCD_SEAFOAM_BLUE` from the set `CSS4_COLORS`. (Colour number `285`)
/// ## Representations:
/// - int tuple `(120, 209, 182)`
/// - float tuple `(0.47, 1, 1)`
/// - hex: `#78D1B6`
pub const XKCD_SEAFOAM_BLUE: Color = Color(120, 209, 182);

/// Colour `(r = 19, g = 187, b = 175)`
///
/// Colour `XKCD_TOPAZ` from the set `CSS4_COLORS`. (Colour number `286`)
/// ## Representations:
/// - int tuple `(19, 187, 175)`
/// - float tuple `(0.07, 1, 1)`
/// - hex: `#13BBAF`
pub const XKCD_TOPAZ: Color = Color(19, 187, 175);

/// Colour `(r = 251, g = 95, b = 252)`
///
/// Colour `XKCD_VIOLET_PINK` from the set `CSS4_COLORS`. (Colour number `287`)
/// ## Representations:
/// - int tuple `(251, 95, 252)`
/// - float tuple `(0.98, 0, 1)`
/// - hex: `#FB5FFC`
pub const XKCD_VIOLET_PINK: Color = Color(251, 95, 252);

/// Colour `(r = 32, g = 249, b = 134)`
///
/// Colour `XKCD_WINTERGREEN` from the set `CSS4_COLORS`. (Colour number `288`)
/// ## Representations:
/// - int tuple `(32, 249, 134)`
/// - float tuple `(0.13, 1, 1)`
/// - hex: `#20F986`
pub const XKCD_WINTERGREEN: Color = Color(32, 249, 134);

/// Colour `(r = 255, g = 227, b = 110)`
///
/// Colour `XKCD_YELLOW_TAN` from the set `CSS4_COLORS`. (Colour number `289`)
/// ## Representations:
/// - int tuple `(255, 227, 110)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFE36E`
pub const XKCD_YELLOW_TAN: Color = Color(255, 227, 110);

/// Colour `(r = 157, g = 7, b = 89)`
///
/// Colour `XKCD_DARK_FUCHSIA` from the set `CSS4_COLORS`. (Colour number `290`)
/// ## Representations:
/// - int tuple `(157, 7, 89)`
/// - float tuple `(0.62, 0, 0)`
/// - hex: `#9D0759`
pub const XKCD_DARK_FUCHSIA: Color = Color(157, 7, 89);

/// Colour `(r = 58, g = 24, b = 177)`
///
/// Colour `XKCD_INDIGO_BLUE` from the set `CSS4_COLORS`. (Colour number `291`)
/// ## Representations:
/// - int tuple `(58, 24, 177)`
/// - float tuple `(0.23, 0, 1)`
/// - hex: `#3A18B1`
pub const XKCD_INDIGO_BLUE: Color = Color(58, 24, 177);

/// Colour `(r = 194, g = 255, b = 137)`
///
/// Colour `XKCD_LIGHT_YELLOWISH_GREEN` from the set `CSS4_COLORS`. (Colour number `292`)
/// ## Representations:
/// - int tuple `(194, 255, 137)`
/// - float tuple `(0.76, 1, 1)`
/// - hex: `#C2FF89`
pub const XKCD_LIGHT_YELLOWISH_GREEN: Color = Color(194, 255, 137);

/// Colour `(r = 215, g = 103, b = 173)`
///
/// Colour `XKCD_PALE_MAGENTA` from the set `CSS4_COLORS`. (Colour number `293`)
/// ## Representations:
/// - int tuple `(215, 103, 173)`
/// - float tuple `(0.84, 0, 1)`
/// - hex: `#D767AD`
pub const XKCD_PALE_MAGENTA: Color = Color(215, 103, 173);

/// Colour `(r = 114, g = 0, b = 88)`
///
/// Colour `XKCD_RICH_PURPLE` from the set `CSS4_COLORS`. (Colour number `294`)
/// ## Representations:
/// - int tuple `(114, 0, 88)`
/// - float tuple `(0.45, 0, 0)`
/// - hex: `#720058`
pub const XKCD_RICH_PURPLE: Color = Color(114, 0, 88);

/// Colour `(r = 255, g = 218, b = 3)`
///
/// Colour `XKCD_SUNFLOWER_YELLOW` from the set `CSS4_COLORS`. (Colour number `295`)
/// ## Representations:
/// - int tuple `(255, 218, 3)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFDA03`
pub const XKCD_SUNFLOWER_YELLOW: Color = Color(255, 218, 3);

/// Colour `(r = 1, g = 192, b = 141)`
///
/// Colour `XKCD_GREEN_SLASH_BLUE` from the set `CSS4_COLORS`. (Colour number `296`)
/// ## Representations:
/// - int tuple `(1, 192, 141)`
/// - float tuple `(0.0, 1, 1)`
/// - hex: `#01C08D`
pub const XKCD_GREEN_SLASH_BLUE: Color = Color(1, 192, 141);

/// Colour `(r = 172, g = 116, b = 52)`
///
/// Colour `XKCD_LEATHER` from the set `CSS4_COLORS`. (Colour number `297`)
/// ## Representations:
/// - int tuple `(172, 116, 52)`
/// - float tuple `(0.67, 0, 0)`
/// - hex: `#AC7434`
pub const XKCD_LEATHER: Color = Color(172, 116, 52);

/// Colour `(r = 1, g = 70, b = 0)`
///
/// Colour `XKCD_RACING_GREEN` from the set `CSS4_COLORS`. (Colour number `298`)
/// ## Representations:
/// - int tuple `(1, 70, 0)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#014600`
pub const XKCD_RACING_GREEN: Color = Color(1, 70, 0);

/// Colour `(r = 153, g = 0, b = 250)`
///
/// Colour `XKCD_VIVID_PURPLE` from the set `CSS4_COLORS`. (Colour number `299`)
/// ## Representations:
/// - int tuple `(153, 0, 250)`
/// - float tuple `(0.6, 0, 1)`
/// - hex: `#9900FA`
pub const XKCD_VIVID_PURPLE: Color = Color(153, 0, 250);

/// Colour `(r = 2, g = 6, b = 111)`
///
/// Colour `XKCD_DARK_ROYAL_BLUE` from the set `CSS4_COLORS`. (Colour number `300`)
/// ## Representations:
/// - int tuple `(2, 6, 111)`
/// - float tuple `(0.01, 0, 0)`
/// - hex: `#02066F`
pub const XKCD_DARK_ROYAL_BLUE: Color = Color(2, 6, 111);

/// Colour `(r = 142, g = 118, b = 24)`
///
/// Colour `XKCD_HAZEL` from the set `CSS4_COLORS`. (Colour number `301`)
/// ## Representations:
/// - int tuple `(142, 118, 24)`
/// - float tuple `(0.56, 0, 0)`
/// - hex: `#8E7618`
pub const XKCD_HAZEL: Color = Color(142, 118, 24);

/// Colour `(r = 209, g = 118, b = 143)`
///
/// Colour `XKCD_MUTED_PINK` from the set `CSS4_COLORS`. (Colour number `302`)
/// ## Representations:
/// - int tuple `(209, 118, 143)`
/// - float tuple `(0.82, 0, 1)`
/// - hex: `#D1768F`
pub const XKCD_MUTED_PINK: Color = Color(209, 118, 143);

/// Colour `(r = 150, g = 180, b = 3)`
///
/// Colour `XKCD_BOOGER_GREEN` from the set `CSS4_COLORS`. (Colour number `303`)
/// ## Representations:
/// - int tuple `(150, 180, 3)`
/// - float tuple `(0.59, 1, 0)`
/// - hex: `#96B403`
pub const XKCD_BOOGER_GREEN: Color = Color(150, 180, 3);

/// Colour `(r = 253, g = 255, b = 99)`
///
/// Colour `XKCD_CANARY` from the set `CSS4_COLORS`. (Colour number `304`)
/// ## Representations:
/// - int tuple `(253, 255, 99)`
/// - float tuple `(0.99, 1, 0)`
/// - hex: `#FDFF63`
pub const XKCD_CANARY: Color = Color(253, 255, 99);

/// Colour `(r = 149, g = 163, b = 166)`
///
/// Colour `XKCD_COOL_GREY` from the set `CSS4_COLORS`. (Colour number `305`)
/// ## Representations:
/// - int tuple `(149, 163, 166)`
/// - float tuple `(0.58, 1, 1)`
/// - hex: `#95A3A6`
pub const XKCD_COOL_GREY: Color = Color(149, 163, 166);

/// Colour `(r = 127, g = 104, b = 78)`
///
/// Colour `XKCD_DARK_TAUPE` from the set `CSS4_COLORS`. (Colour number `306`)
/// ## Representations:
/// - int tuple `(127, 104, 78)`
/// - float tuple `(0.5, 0, 0)`
/// - hex: `#7F684E`
pub const XKCD_DARK_TAUPE: Color = Color(127, 104, 78);

/// Colour `(r = 117, g = 25, b = 115)`
///
/// Colour `XKCD_DARKISH_PURPLE` from the set `CSS4_COLORS`. (Colour number `307`)
/// ## Representations:
/// - int tuple `(117, 25, 115)`
/// - float tuple `(0.46, 0, 0)`
/// - hex: `#751973`
pub const XKCD_DARKISH_PURPLE: Color = Color(117, 25, 115);

/// Colour `(r = 8, g = 148, b = 4)`
///
/// Colour `XKCD_TRUE_GREEN` from the set `CSS4_COLORS`. (Colour number `308`)
/// ## Representations:
/// - int tuple `(8, 148, 4)`
/// - float tuple `(0.03, 1, 0)`
/// - hex: `#089404`
pub const XKCD_TRUE_GREEN: Color = Color(8, 148, 4);

/// Colour `(r = 255, g = 97, b = 99)`
///
/// Colour `XKCD_CORAL_PINK` from the set `CSS4_COLORS`. (Colour number `309`)
/// ## Representations:
/// - int tuple `(255, 97, 99)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FF6163`
pub const XKCD_CORAL_PINK: Color = Color(255, 97, 99);

/// Colour `(r = 89, g = 133, b = 86)`
///
/// Colour `XKCD_DARK_SAGE` from the set `CSS4_COLORS`. (Colour number `310`)
/// ## Representations:
/// - int tuple `(89, 133, 86)`
/// - float tuple `(0.35, 1, 0)`
/// - hex: `#598556`
pub const XKCD_DARK_SAGE: Color = Color(89, 133, 86);

/// Colour `(r = 33, g = 71, b = 97)`
///
/// Colour `XKCD_DARK_SLATE_BLUE` from the set `CSS4_COLORS`. (Colour number `311`)
/// ## Representations:
/// - int tuple `(33, 71, 97)`
/// - float tuple `(0.13, 0, 0)`
/// - hex: `#214761`
pub const XKCD_DARK_SLATE_BLUE: Color = Color(33, 71, 97);

/// Colour `(r = 60, g = 115, b = 168)`
///
/// Colour `XKCD_FLAT_BLUE` from the set `CSS4_COLORS`. (Colour number `312`)
/// ## Representations:
/// - int tuple `(60, 115, 168)`
/// - float tuple `(0.24, 0, 1)`
/// - hex: `#3C73A8`
pub const XKCD_FLAT_BLUE: Color = Color(60, 115, 168);

/// Colour `(r = 186, g = 158, b = 136)`
///
/// Colour `XKCD_MUSHROOM` from the set `CSS4_COLORS`. (Colour number `313`)
/// ## Representations:
/// - int tuple `(186, 158, 136)`
/// - float tuple `(0.73, 1, 1)`
/// - hex: `#BA9E88`
pub const XKCD_MUSHROOM: Color = Color(186, 158, 136);

/// Colour `(r = 2, g = 27, b = 249)`
///
/// Colour `XKCD_RICH_BLUE` from the set `CSS4_COLORS`. (Colour number `314`)
/// ## Representations:
/// - int tuple `(2, 27, 249)`
/// - float tuple `(0.01, 0, 1)`
/// - hex: `#021BF9`
pub const XKCD_RICH_BLUE: Color = Color(2, 27, 249);

/// Colour `(r = 115, g = 74, b = 101)`
///
/// Colour `XKCD_DIRTY_PURPLE` from the set `CSS4_COLORS`. (Colour number `315`)
/// ## Representations:
/// - int tuple `(115, 74, 101)`
/// - float tuple `(0.45, 0, 0)`
/// - hex: `#734A65`
pub const XKCD_DIRTY_PURPLE: Color = Color(115, 74, 101);

/// Colour `(r = 35, g = 196, b = 139)`
///
/// Colour `XKCD_GREENBLUE` from the set `CSS4_COLORS`. (Colour number `316`)
/// ## Representations:
/// - int tuple `(35, 196, 139)`
/// - float tuple `(0.14, 1, 1)`
/// - hex: `#23C48B`
pub const XKCD_GREENBLUE: Color = Color(35, 196, 139);

/// Colour `(r = 143, g = 174, b = 34)`
///
/// Colour `XKCD_ICKY_GREEN` from the set `CSS4_COLORS`. (Colour number `317`)
/// ## Representations:
/// - int tuple `(143, 174, 34)`
/// - float tuple `(0.56, 1, 0)`
/// - hex: `#8FAE22`
pub const XKCD_ICKY_GREEN: Color = Color(143, 174, 34);

/// Colour `(r = 230, g = 242, b = 162)`
///
/// Colour `XKCD_LIGHT_KHAKI` from the set `CSS4_COLORS`. (Colour number `318`)
/// ## Representations:
/// - int tuple `(230, 242, 162)`
/// - float tuple `(0.9, 1, 1)`
/// - hex: `#E6F2A2`
pub const XKCD_LIGHT_KHAKI: Color = Color(230, 242, 162);

/// Colour `(r = 75, g = 87, b = 219)`
///
/// Colour `XKCD_WARM_BLUE` from the set `CSS4_COLORS`. (Colour number `319`)
/// ## Representations:
/// - int tuple `(75, 87, 219)`
/// - float tuple `(0.29, 0, 1)`
/// - hex: `#4B57DB`
pub const XKCD_WARM_BLUE: Color = Color(75, 87, 219);

/// Colour `(r = 217, g = 1, b = 102)`
///
/// Colour `XKCD_DARK_HOT_PINK` from the set `CSS4_COLORS`. (Colour number `320`)
/// ## Representations:
/// - int tuple `(217, 1, 102)`
/// - float tuple `(0.85, 0, 0)`
/// - hex: `#D90166`
pub const XKCD_DARK_HOT_PINK: Color = Color(217, 1, 102);

/// Colour `(r = 1, g = 84, b = 130)`
///
/// Colour `XKCD_DEEP_SEA_BLUE` from the set `CSS4_COLORS`. (Colour number `321`)
/// ## Representations:
/// - int tuple `(1, 84, 130)`
/// - float tuple `(0.0, 0, 1)`
/// - hex: `#015482`
pub const XKCD_DEEP_SEA_BLUE: Color = Color(1, 84, 130);

/// Colour `(r = 157, g = 2, b = 22)`
///
/// Colour `XKCD_CARMINE` from the set `CSS4_COLORS`. (Colour number `322`)
/// ## Representations:
/// - int tuple `(157, 2, 22)`
/// - float tuple `(0.62, 0, 0)`
/// - hex: `#9D0216`
pub const XKCD_CARMINE: Color = Color(157, 2, 22);

/// Colour `(r = 114, g = 143, b = 2)`
///
/// Colour `XKCD_DARK_YELLOW_GREEN` from the set `CSS4_COLORS`. (Colour number `323`)
/// ## Representations:
/// - int tuple `(114, 143, 2)`
/// - float tuple `(0.45, 1, 0)`
/// - hex: `#728F02`
pub const XKCD_DARK_YELLOW_GREEN: Color = Color(114, 143, 2);

/// Colour `(r = 255, g = 229, b = 173)`
///
/// Colour `XKCD_PALE_PEACH` from the set `CSS4_COLORS`. (Colour number `324`)
/// ## Representations:
/// - int tuple `(255, 229, 173)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFE5AD`
pub const XKCD_PALE_PEACH: Color = Color(255, 229, 173);

/// Colour `(r = 78, g = 5, b = 80)`
///
/// Colour `XKCD_PLUM_PURPLE` from the set `CSS4_COLORS`. (Colour number `325`)
/// ## Representations:
/// - int tuple `(78, 5, 80)`
/// - float tuple `(0.31, 0, 0)`
/// - hex: `#4E0550`
pub const XKCD_PLUM_PURPLE: Color = Color(78, 5, 80);

/// Colour `(r = 249, g = 188, b = 8)`
///
/// Colour `XKCD_GOLDEN_ROD` from the set `CSS4_COLORS`. (Colour number `326`)
/// ## Representations:
/// - int tuple `(249, 188, 8)`
/// - float tuple `(0.98, 1, 0)`
/// - hex: `#F9BC08`
pub const XKCD_GOLDEN_ROD: Color = Color(249, 188, 8);

/// Colour `(r = 255, g = 7, b = 58)`
///
/// Colour `XKCD_NEON_RED` from the set `CSS4_COLORS`. (Colour number `327`)
/// ## Representations:
/// - int tuple `(255, 7, 58)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FF073A`
pub const XKCD_NEON_RED: Color = Color(255, 7, 58);

/// Colour `(r = 199, g = 121, b = 134)`
///
/// Colour `XKCD_OLD_PINK` from the set `CSS4_COLORS`. (Colour number `328`)
/// ## Representations:
/// - int tuple `(199, 121, 134)`
/// - float tuple `(0.78, 0, 1)`
/// - hex: `#C77986`
pub const XKCD_OLD_PINK: Color = Color(199, 121, 134);

/// Colour `(r = 214, g = 255, b = 254)`
///
/// Colour `XKCD_VERY_PALE_BLUE` from the set `CSS4_COLORS`. (Colour number `329`)
/// ## Representations:
/// - int tuple `(214, 255, 254)`
/// - float tuple `(0.84, 1, 1)`
/// - hex: `#D6FFFE`
pub const XKCD_VERY_PALE_BLUE: Color = Color(214, 255, 254);

/// Colour `(r = 254, g = 75, b = 3)`
///
/// Colour `XKCD_BLOOD_ORANGE` from the set `CSS4_COLORS`. (Colour number `330`)
/// ## Representations:
/// - int tuple `(254, 75, 3)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FE4B03`
pub const XKCD_BLOOD_ORANGE: Color = Color(254, 75, 3);

/// Colour `(r = 253, g = 89, b = 86)`
///
/// Colour `XKCD_GRAPEFRUIT` from the set `CSS4_COLORS`. (Colour number `331`)
/// ## Representations:
/// - int tuple `(253, 89, 86)`
/// - float tuple `(0.99, 0, 0)`
/// - hex: `#FD5956`
pub const XKCD_GRAPEFRUIT: Color = Color(253, 89, 86);

/// Colour `(r = 252, g = 225, b = 102)`
///
/// Colour `XKCD_SAND_YELLOW` from the set `CSS4_COLORS`. (Colour number `332`)
/// ## Representations:
/// - int tuple `(252, 225, 102)`
/// - float tuple `(0.99, 1, 0)`
/// - hex: `#FCE166`
pub const XKCD_SAND_YELLOW: Color = Color(252, 225, 102);

/// Colour `(r = 178, g = 113, b = 61)`
///
/// Colour `XKCD_CLAY_BROWN` from the set `CSS4_COLORS`. (Colour number `333`)
/// ## Representations:
/// - int tuple `(178, 113, 61)`
/// - float tuple `(0.7, 0, 0)`
/// - hex: `#B2713D`
pub const XKCD_CLAY_BROWN: Color = Color(178, 113, 61);

/// Colour `(r = 31, g = 59, b = 77)`
///
/// Colour `XKCD_DARK_BLUE_GREY` from the set `CSS4_COLORS`. (Colour number `334`)
/// ## Representations:
/// - int tuple `(31, 59, 77)`
/// - float tuple `(0.12, 0, 0)`
/// - hex: `#1F3B4D`
pub const XKCD_DARK_BLUE_GREY: Color = Color(31, 59, 77);

/// Colour `(r = 105, g = 157, b = 76)`
///
/// Colour `XKCD_FLAT_GREEN` from the set `CSS4_COLORS`. (Colour number `335`)
/// ## Representations:
/// - int tuple `(105, 157, 76)`
/// - float tuple `(0.41, 1, 0)`
/// - hex: `#699D4C`
pub const XKCD_FLAT_GREEN: Color = Color(105, 157, 76);

/// Colour `(r = 86, g = 252, b = 162)`
///
/// Colour `XKCD_LIGHT_GREEN_BLUE` from the set `CSS4_COLORS`. (Colour number `336`)
/// ## Representations:
/// - int tuple `(86, 252, 162)`
/// - float tuple `(0.34, 1, 1)`
/// - hex: `#56FCA2`
pub const XKCD_LIGHT_GREEN_BLUE: Color = Color(86, 252, 162);

/// Colour `(r = 251, g = 85, b = 129)`
///
/// Colour `XKCD_WARM_PINK` from the set `CSS4_COLORS`. (Colour number `337`)
/// ## Representations:
/// - int tuple `(251, 85, 129)`
/// - float tuple `(0.98, 0, 1)`
/// - hex: `#FB5581`
pub const XKCD_WARM_PINK: Color = Color(251, 85, 129);

/// Colour `(r = 62, g = 130, b = 252)`
///
/// Colour `XKCD_DODGER_BLUE` from the set `CSS4_COLORS`. (Colour number `338`)
/// ## Representations:
/// - int tuple `(62, 130, 252)`
/// - float tuple `(0.24, 1, 1)`
/// - hex: `#3E82FC`
pub const XKCD_DODGER_BLUE: Color = Color(62, 130, 252);

/// Colour `(r = 160, g = 191, b = 22)`
///
/// Colour `XKCD_GROSS_GREEN` from the set `CSS4_COLORS`. (Colour number `339`)
/// ## Representations:
/// - int tuple `(160, 191, 22)`
/// - float tuple `(0.63, 1, 0)`
/// - hex: `#A0BF16`
pub const XKCD_GROSS_GREEN: Color = Color(160, 191, 22);

/// Colour `(r = 214, g = 255, b = 250)`
///
/// Colour `XKCD_ICE` from the set `CSS4_COLORS`. (Colour number `340`)
/// ## Representations:
/// - int tuple `(214, 255, 250)`
/// - float tuple `(0.84, 1, 1)`
/// - hex: `#D6FFFA`
pub const XKCD_ICE: Color = Color(214, 255, 250);

/// Colour `(r = 79, g = 115, b = 142)`
///
/// Colour `XKCD_METALLIC_BLUE` from the set `CSS4_COLORS`. (Colour number `341`)
/// ## Representations:
/// - int tuple `(79, 115, 142)`
/// - float tuple `(0.31, 0, 1)`
/// - hex: `#4F738E`
pub const XKCD_METALLIC_BLUE: Color = Color(79, 115, 142);

/// Colour `(r = 255, g = 177, b = 154)`
///
/// Colour `XKCD_PALE_SALMON` from the set `CSS4_COLORS`. (Colour number `342`)
/// ## Representations:
/// - int tuple `(255, 177, 154)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFB19A`
pub const XKCD_PALE_SALMON: Color = Color(255, 177, 154);

/// Colour `(r = 92, g = 139, b = 21)`
///
/// Colour `XKCD_SAP_GREEN` from the set `CSS4_COLORS`. (Colour number `343`)
/// ## Representations:
/// - int tuple `(92, 139, 21)`
/// - float tuple `(0.36, 1, 0)`
/// - hex: `#5C8B15`
pub const XKCD_SAP_GREEN: Color = Color(92, 139, 21);

/// Colour `(r = 84, g = 172, b = 104)`
///
/// Colour `XKCD_ALGAE` from the set `CSS4_COLORS`. (Colour number `344`)
/// ## Representations:
/// - int tuple `(84, 172, 104)`
/// - float tuple `(0.33, 1, 0)`
/// - hex: `#54AC68`
pub const XKCD_ALGAE: Color = Color(84, 172, 104);

/// Colour `(r = 137, g = 160, b = 176)`
///
/// Colour `XKCD_BLUEY_GREY` from the set `CSS4_COLORS`. (Colour number `345`)
/// ## Representations:
/// - int tuple `(137, 160, 176)`
/// - float tuple `(0.54, 1, 1)`
/// - hex: `#89A0B0`
pub const XKCD_BLUEY_GREY: Color = Color(137, 160, 176);

/// Colour `(r = 126, g = 160, b = 122)`
///
/// Colour `XKCD_GREENY_GREY` from the set `CSS4_COLORS`. (Colour number `346`)
/// ## Representations:
/// - int tuple `(126, 160, 122)`
/// - float tuple `(0.49, 1, 0)`
/// - hex: `#7EA07A`
pub const XKCD_GREENY_GREY: Color = Color(126, 160, 122);

/// Colour `(r = 27, g = 252, b = 6)`
///
/// Colour `XKCD_HIGHLIGHTER_GREEN` from the set `CSS4_COLORS`. (Colour number `347`)
/// ## Representations:
/// - int tuple `(27, 252, 6)`
/// - float tuple `(0.11, 1, 0)`
/// - hex: `#1BFC06`
pub const XKCD_HIGHLIGHTER_GREEN: Color = Color(27, 252, 6);

/// Colour `(r = 202, g = 255, b = 251)`
///
/// Colour `XKCD_LIGHT_LIGHT_BLUE` from the set `CSS4_COLORS`. (Colour number `348`)
/// ## Representations:
/// - int tuple `(202, 255, 251)`
/// - float tuple `(0.79, 1, 1)`
/// - hex: `#CAFFFB`
pub const XKCD_LIGHT_LIGHT_BLUE: Color = Color(202, 255, 251);

/// Colour `(r = 182, g = 255, b = 187)`
///
/// Colour `XKCD_LIGHT_MINT` from the set `CSS4_COLORS`. (Colour number `349`)
/// ## Representations:
/// - int tuple `(182, 255, 187)`
/// - float tuple `(0.71, 1, 1)`
/// - hex: `#B6FFBB`
pub const XKCD_LIGHT_MINT: Color = Color(182, 255, 187);

/// Colour `(r = 167, g = 94, b = 9)`
///
/// Colour `XKCD_RAW_UMBER` from the set `CSS4_COLORS`. (Colour number `350`)
/// ## Representations:
/// - int tuple `(167, 94, 9)`
/// - float tuple `(0.65, 0, 0)`
/// - hex: `#A75E09`
pub const XKCD_RAW_UMBER: Color = Color(167, 94, 9);

/// Colour `(r = 21, g = 46, b = 255)`
///
/// Colour `XKCD_VIVID_BLUE` from the set `CSS4_COLORS`. (Colour number `351`)
/// ## Representations:
/// - int tuple `(21, 46, 255)`
/// - float tuple `(0.08, 0, 1)`
/// - hex: `#152EFF`
pub const XKCD_VIVID_BLUE: Color = Color(21, 46, 255);

/// Colour `(r = 141, g = 94, b = 183)`
///
/// Colour `XKCD_DEEP_LAVENDER` from the set `CSS4_COLORS`. (Colour number `352`)
/// ## Representations:
/// - int tuple `(141, 94, 183)`
/// - float tuple `(0.55, 0, 1)`
/// - hex: `#8D5EB7`
pub const XKCD_DEEP_LAVENDER: Color = Color(141, 94, 183);

/// Colour `(r = 95, g = 158, b = 143)`
///
/// Colour `XKCD_DULL_TEAL` from the set `CSS4_COLORS`. (Colour number `353`)
/// ## Representations:
/// - int tuple `(95, 158, 143)`
/// - float tuple `(0.37, 1, 1)`
/// - hex: `#5F9E8F`
pub const XKCD_DULL_TEAL: Color = Color(95, 158, 143);

/// Colour `(r = 99, g = 247, b = 180)`
///
/// Colour `XKCD_LIGHT_GREENISH_BLUE` from the set `CSS4_COLORS`. (Colour number `354`)
/// ## Representations:
/// - int tuple `(99, 247, 180)`
/// - float tuple `(0.39, 1, 1)`
/// - hex: `#63F7B4`
pub const XKCD_LIGHT_GREENISH_BLUE: Color = Color(99, 247, 180);

/// Colour `(r = 96, g = 102, b = 2)`
///
/// Colour `XKCD_MUD_GREEN` from the set `CSS4_COLORS`. (Colour number `355`)
/// ## Representations:
/// - int tuple `(96, 102, 2)`
/// - float tuple `(0.38, 0, 0)`
/// - hex: `#606602`
pub const XKCD_MUD_GREEN: Color = Color(96, 102, 2);

/// Colour `(r = 252, g = 134, b = 170)`
///
/// Colour `XKCD_PINKY` from the set `CSS4_COLORS`. (Colour number `356`)
/// ## Representations:
/// - int tuple `(252, 134, 170)`
/// - float tuple `(0.99, 1, 1)`
/// - hex: `#FC86AA`
pub const XKCD_PINKY: Color = Color(252, 134, 170);

/// Colour `(r = 140, g = 0, b = 52)`
///
/// Colour `XKCD_RED_WINE` from the set `CSS4_COLORS`. (Colour number `357`)
/// ## Representations:
/// - int tuple `(140, 0, 52)`
/// - float tuple `(0.55, 0, 0)`
/// - hex: `#8C0034`
pub const XKCD_RED_WINE: Color = Color(140, 0, 52);

/// Colour `(r = 117, g = 128, b = 0)`
///
/// Colour `XKCD_SHIT_GREEN` from the set `CSS4_COLORS`. (Colour number `358`)
/// ## Representations:
/// - int tuple `(117, 128, 0)`
/// - float tuple `(0.46, 1, 0)`
/// - hex: `#758000`
pub const XKCD_SHIT_GREEN: Color = Color(117, 128, 0);

/// Colour `(r = 171, g = 126, b = 76)`
///
/// Colour `XKCD_TAN_BROWN` from the set `CSS4_COLORS`. (Colour number `359`)
/// ## Representations:
/// - int tuple `(171, 126, 76)`
/// - float tuple `(0.67, 0, 0)`
/// - hex: `#AB7E4C`
pub const XKCD_TAN_BROWN: Color = Color(171, 126, 76);

/// Colour `(r = 3, g = 7, b = 100)`
///
/// Colour `XKCD_DARKBLUE` from the set `CSS4_COLORS`. (Colour number `360`)
/// ## Representations:
/// - int tuple `(3, 7, 100)`
/// - float tuple `(0.01, 0, 0)`
/// - hex: `#030764`
pub const XKCD_DARKBLUE: Color = Color(3, 7, 100);

/// Colour `(r = 254, g = 134, b = 164)`
///
/// Colour `XKCD_ROSA` from the set `CSS4_COLORS`. (Colour number `361`)
/// ## Representations:
/// - int tuple `(254, 134, 164)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FE86A4`
pub const XKCD_ROSA: Color = Color(254, 134, 164);

/// Colour `(r = 213, g = 23, b = 78)`
///
/// Colour `XKCD_LIPSTICK` from the set `CSS4_COLORS`. (Colour number `362`)
/// ## Representations:
/// - int tuple `(213, 23, 78)`
/// - float tuple `(0.84, 0, 0)`
/// - hex: `#D5174E`
pub const XKCD_LIPSTICK: Color = Color(213, 23, 78);

/// Colour `(r = 254, g = 208, b = 252)`
///
/// Colour `XKCD_PALE_MAUVE` from the set `CSS4_COLORS`. (Colour number `363`)
/// ## Representations:
/// - int tuple `(254, 208, 252)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FED0FC`
pub const XKCD_PALE_MAUVE: Color = Color(254, 208, 252);

/// Colour `(r = 104, g = 0, b = 24)`
///
/// Colour `XKCD_CLARET` from the set `CSS4_COLORS`. (Colour number `364`)
/// ## Representations:
/// - int tuple `(104, 0, 24)`
/// - float tuple `(0.41, 0, 0)`
/// - hex: `#680018`
pub const XKCD_CLARET: Color = Color(104, 0, 24);

/// Colour `(r = 254, g = 223, b = 8)`
///
/// Colour `XKCD_DANDELION` from the set `CSS4_COLORS`. (Colour number `365`)
/// ## Representations:
/// - int tuple `(254, 223, 8)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FEDF08`
pub const XKCD_DANDELION: Color = Color(254, 223, 8);

/// Colour `(r = 254, g = 66, b = 15)`
///
/// Colour `XKCD_ORANGERED` from the set `CSS4_COLORS`. (Colour number `366`)
/// ## Representations:
/// - int tuple `(254, 66, 15)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FE420F`
pub const XKCD_ORANGERED: Color = Color(254, 66, 15);

/// Colour `(r = 111, g = 124, b = 0)`
///
/// Colour `XKCD_POOP_GREEN` from the set `CSS4_COLORS`. (Colour number `367`)
/// ## Representations:
/// - int tuple `(111, 124, 0)`
/// - float tuple `(0.44, 0, 0)`
/// - hex: `#6F7C00`
pub const XKCD_POOP_GREEN: Color = Color(111, 124, 0);

/// Colour `(r = 202, g = 1, b = 71)`
///
/// Colour `XKCD_RUBY` from the set `CSS4_COLORS`. (Colour number `368`)
/// ## Representations:
/// - int tuple `(202, 1, 71)`
/// - float tuple `(0.79, 0, 0)`
/// - hex: `#CA0147`
pub const XKCD_RUBY: Color = Color(202, 1, 71);

/// Colour `(r = 27, g = 36, b = 49)`
///
/// Colour `XKCD_DARK` from the set `CSS4_COLORS`. (Colour number `369`)
/// ## Representations:
/// - int tuple `(27, 36, 49)`
/// - float tuple `(0.11, 0, 0)`
/// - hex: `#1B2431`
pub const XKCD_DARK: Color = Color(27, 36, 49);

/// Colour `(r = 0, g = 251, b = 176)`
///
/// Colour `XKCD_GREENISH_TURQUOISE` from the set `CSS4_COLORS`. (Colour number `370`)
/// ## Representations:
/// - int tuple `(0, 251, 176)`
/// - float tuple `(0.0, 1, 1)`
/// - hex: `#00FBB0`
pub const XKCD_GREENISH_TURQUOISE: Color = Color(0, 251, 176);

/// Colour `(r = 219, g = 88, b = 86)`
///
/// Colour `XKCD_PASTEL_RED` from the set `CSS4_COLORS`. (Colour number `371`)
/// ## Representations:
/// - int tuple `(219, 88, 86)`
/// - float tuple `(0.86, 0, 0)`
/// - hex: `#DB5856`
pub const XKCD_PASTEL_RED: Color = Color(219, 88, 86);

/// Colour `(r = 221, g = 214, b = 24)`
///
/// Colour `XKCD_PISS_YELLOW` from the set `CSS4_COLORS`. (Colour number `372`)
/// ## Representations:
/// - int tuple `(221, 214, 24)`
/// - float tuple `(0.87, 1, 0)`
/// - hex: `#DDD618`
pub const XKCD_PISS_YELLOW: Color = Color(221, 214, 24);

/// Colour `(r = 65, g = 253, b = 254)`
///
/// Colour `XKCD_BRIGHT_CYAN` from the set `CSS4_COLORS`. (Colour number `373`)
/// ## Representations:
/// - int tuple `(65, 253, 254)`
/// - float tuple `(0.25, 1, 1)`
/// - hex: `#41FDFE`
pub const XKCD_BRIGHT_CYAN: Color = Color(65, 253, 254);

/// Colour `(r = 207, g = 82, b = 78)`
///
/// Colour `XKCD_DARK_CORAL` from the set `CSS4_COLORS`. (Colour number `374`)
/// ## Representations:
/// - int tuple `(207, 82, 78)`
/// - float tuple `(0.81, 0, 0)`
/// - hex: `#CF524E`
pub const XKCD_DARK_CORAL: Color = Color(207, 82, 78);

/// Colour `(r = 33, g = 195, b = 111)`
///
/// Colour `XKCD_ALGAE_GREEN` from the set `CSS4_COLORS`. (Colour number `375`)
/// ## Representations:
/// - int tuple `(33, 195, 111)`
/// - float tuple `(0.13, 1, 0)`
/// - hex: `#21C36F`
pub const XKCD_ALGAE_GREEN: Color = Color(33, 195, 111);

/// Colour `(r = 169, g = 3, b = 8)`
///
/// Colour `XKCD_DARKISH_RED` from the set `CSS4_COLORS`. (Colour number `376`)
/// ## Representations:
/// - int tuple `(169, 3, 8)`
/// - float tuple `(0.66, 0, 0)`
/// - hex: `#A90308`
pub const XKCD_DARKISH_RED: Color = Color(169, 3, 8);

/// Colour `(r = 110, g = 16, b = 5)`
///
/// Colour `XKCD_REDDY_BROWN` from the set `CSS4_COLORS`. (Colour number `377`)
/// ## Representations:
/// - int tuple `(110, 16, 5)`
/// - float tuple `(0.43, 0, 0)`
/// - hex: `#6E1005`
pub const XKCD_REDDY_BROWN: Color = Color(110, 16, 5);

/// Colour `(r = 254, g = 130, b = 140)`
///
/// Colour `XKCD_BLUSH_PINK` from the set `CSS4_COLORS`. (Colour number `378`)
/// ## Representations:
/// - int tuple `(254, 130, 140)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FE828C`
pub const XKCD_BLUSH_PINK: Color = Color(254, 130, 140);

/// Colour `(r = 75, g = 97, b = 19)`
///
/// Colour `XKCD_CAMOUFLAGE_GREEN` from the set `CSS4_COLORS`. (Colour number `379`)
/// ## Representations:
/// - int tuple `(75, 97, 19)`
/// - float tuple `(0.29, 0, 0)`
/// - hex: `#4B6113`
pub const XKCD_CAMOUFLAGE_GREEN: Color = Color(75, 97, 19);

/// Colour `(r = 77, g = 164, b = 9)`
///
/// Colour `XKCD_LAWN_GREEN` from the set `CSS4_COLORS`. (Colour number `380`)
/// ## Representations:
/// - int tuple `(77, 164, 9)`
/// - float tuple `(0.3, 1, 0)`
/// - hex: `#4DA409`
pub const XKCD_LAWN_GREEN: Color = Color(77, 164, 9);

/// Colour `(r = 190, g = 174, b = 138)`
///
/// Colour `XKCD_PUTTY` from the set `CSS4_COLORS`. (Colour number `381`)
/// ## Representations:
/// - int tuple `(190, 174, 138)`
/// - float tuple `(0.75, 1, 1)`
/// - hex: `#BEAE8A`
pub const XKCD_PUTTY: Color = Color(190, 174, 138);

/// Colour `(r = 3, g = 57, b = 248)`
///
/// Colour `XKCD_VIBRANT_BLUE` from the set `CSS4_COLORS`. (Colour number `382`)
/// ## Representations:
/// - int tuple `(3, 57, 248)`
/// - float tuple `(0.01, 0, 1)`
/// - hex: `#0339F8`
pub const XKCD_VIBRANT_BLUE: Color = Color(3, 57, 248);

/// Colour `(r = 168, g = 143, b = 89)`
///
/// Colour `XKCD_DARK_SAND` from the set `CSS4_COLORS`. (Colour number `383`)
/// ## Representations:
/// - int tuple `(168, 143, 89)`
/// - float tuple `(0.66, 1, 0)`
/// - hex: `#A88F59`
pub const XKCD_DARK_SAND: Color = Color(168, 143, 89);

/// Colour `(r = 93, g = 33, b = 208)`
///
/// Colour `XKCD_PURPLE_SLASH_BLUE` from the set `CSS4_COLORS`. (Colour number `384`)
/// ## Representations:
/// - int tuple `(93, 33, 208)`
/// - float tuple `(0.36, 0, 1)`
/// - hex: `#5D21D0`
pub const XKCD_PURPLE_SLASH_BLUE: Color = Color(93, 33, 208);

/// Colour `(r = 254, g = 178, b = 9)`
///
/// Colour `XKCD_SAFFRON` from the set `CSS4_COLORS`. (Colour number `385`)
/// ## Representations:
/// - int tuple `(254, 178, 9)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FEB209`
pub const XKCD_SAFFRON: Color = Color(254, 178, 9);

/// Colour `(r = 78, g = 81, b = 139)`
///
/// Colour `XKCD_TWILIGHT` from the set `CSS4_COLORS`. (Colour number `386`)
/// ## Representations:
/// - int tuple `(78, 81, 139)`
/// - float tuple `(0.31, 0, 1)`
/// - hex: `#4E518B`
pub const XKCD_TWILIGHT: Color = Color(78, 81, 139);

/// Colour `(r = 150, g = 78, b = 2)`
///
/// Colour `XKCD_WARM_BROWN` from the set `CSS4_COLORS`. (Colour number `387`)
/// ## Representations:
/// - int tuple `(150, 78, 2)`
/// - float tuple `(0.59, 0, 0)`
/// - hex: `#964E02`
pub const XKCD_WARM_BROWN: Color = Color(150, 78, 2);

/// Colour `(r = 133, g = 163, b = 178)`
///
/// Colour `XKCD_BLUEGREY` from the set `CSS4_COLORS`. (Colour number `388`)
/// ## Representations:
/// - int tuple `(133, 163, 178)`
/// - float tuple `(0.52, 1, 1)`
/// - hex: `#85A3B2`
pub const XKCD_BLUEGREY: Color = Color(133, 163, 178);

/// Colour `(r = 255, g = 105, b = 175)`
///
/// Colour `XKCD_BUBBLE_GUM_PINK` from the set `CSS4_COLORS`. (Colour number `389`)
/// ## Representations:
/// - int tuple `(255, 105, 175)`
/// - float tuple `(1.0, 0, 1)`
/// - hex: `#FF69AF`
pub const XKCD_BUBBLE_GUM_PINK: Color = Color(255, 105, 175);

/// Colour `(r = 195, g = 251, b = 244)`
///
/// Colour `XKCD_DUCK_EGG_BLUE` from the set `CSS4_COLORS`. (Colour number `390`)
/// ## Representations:
/// - int tuple `(195, 251, 244)`
/// - float tuple `(0.76, 1, 1)`
/// - hex: `#C3FBF4`
pub const XKCD_DUCK_EGG_BLUE: Color = Color(195, 251, 244);

/// Colour `(r = 42, g = 254, b = 183)`
///
/// Colour `XKCD_GREENISH_CYAN` from the set `CSS4_COLORS`. (Colour number `391`)
/// ## Representations:
/// - int tuple `(42, 254, 183)`
/// - float tuple `(0.16, 1, 1)`
/// - hex: `#2AFEB7`
pub const XKCD_GREENISH_CYAN: Color = Color(42, 254, 183);

/// Colour `(r = 0, g = 95, b = 106)`
///
/// Colour `XKCD_PETROL` from the set `CSS4_COLORS`. (Colour number `392`)
/// ## Representations:
/// - int tuple `(0, 95, 106)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#005F6A`
pub const XKCD_PETROL: Color = Color(0, 95, 106);

/// Colour `(r = 12, g = 23, b = 147)`
///
/// Colour `XKCD_ROYAL` from the set `CSS4_COLORS`. (Colour number `393`)
/// ## Representations:
/// - int tuple `(12, 23, 147)`
/// - float tuple `(0.05, 0, 1)`
/// - hex: `#0C1793`
pub const XKCD_ROYAL: Color = Color(12, 23, 147);

/// Colour `(r = 255, g = 255, b = 129)`
///
/// Colour `XKCD_BUTTER` from the set `CSS4_COLORS`. (Colour number `394`)
/// ## Representations:
/// - int tuple `(255, 255, 129)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFFF81`
pub const XKCD_BUTTER: Color = Color(255, 255, 129);

/// Colour `(r = 240, g = 131, b = 58)`
///
/// Colour `XKCD_DUSTY_ORANGE` from the set `CSS4_COLORS`. (Colour number `395`)
/// ## Representations:
/// - int tuple `(240, 131, 58)`
/// - float tuple `(0.94, 1, 0)`
/// - hex: `#F0833A`
pub const XKCD_DUSTY_ORANGE: Color = Color(240, 131, 58);

/// Colour `(r = 241, g = 243, b = 63)`
///
/// Colour `XKCD_OFF_YELLOW` from the set `CSS4_COLORS`. (Colour number `396`)
/// ## Representations:
/// - int tuple `(241, 243, 63)`
/// - float tuple `(0.95, 1, 0)`
/// - hex: `#F1F33F`
pub const XKCD_OFF_YELLOW: Color = Color(241, 243, 63);

/// Colour `(r = 177, g = 210, b = 123)`
///
/// Colour `XKCD_PALE_OLIVE_GREEN` from the set `CSS4_COLORS`. (Colour number `397`)
/// ## Representations:
/// - int tuple `(177, 210, 123)`
/// - float tuple `(0.69, 1, 0)`
/// - hex: `#B1D27B`
pub const XKCD_PALE_OLIVE_GREEN: Color = Color(177, 210, 123);

/// Colour `(r = 252, g = 130, b = 74)`
///
/// Colour `XKCD_ORANGISH` from the set `CSS4_COLORS`. (Colour number `398`)
/// ## Representations:
/// - int tuple `(252, 130, 74)`
/// - float tuple `(0.99, 1, 0)`
/// - hex: `#FC824A`
pub const XKCD_ORANGISH: Color = Color(252, 130, 74);

/// Colour `(r = 113, g = 170, b = 52)`
///
/// Colour `XKCD_LEAF` from the set `CSS4_COLORS`. (Colour number `399`)
/// ## Representations:
/// - int tuple `(113, 170, 52)`
/// - float tuple `(0.44, 1, 0)`
/// - hex: `#71AA34`
pub const XKCD_LEAF: Color = Color(113, 170, 52);

/// Colour `(r = 183, g = 201, b = 226)`
///
/// Colour `XKCD_LIGHT_BLUE_GREY` from the set `CSS4_COLORS`. (Colour number `400`)
/// ## Representations:
/// - int tuple `(183, 201, 226)`
/// - float tuple `(0.72, 1, 1)`
/// - hex: `#B7C9E2`
pub const XKCD_LIGHT_BLUE_GREY: Color = Color(183, 201, 226);

/// Colour `(r = 75, g = 1, b = 1)`
///
/// Colour `XKCD_DRIED_BLOOD` from the set `CSS4_COLORS`. (Colour number `401`)
/// ## Representations:
/// - int tuple `(75, 1, 1)`
/// - float tuple `(0.29, 0, 0)`
/// - hex: `#4B0101`
pub const XKCD_DRIED_BLOOD: Color = Color(75, 1, 1);

/// Colour `(r = 165, g = 82, b = 230)`
///
/// Colour `XKCD_LIGHTISH_PURPLE` from the set `CSS4_COLORS`. (Colour number `402`)
/// ## Representations:
/// - int tuple `(165, 82, 230)`
/// - float tuple `(0.65, 0, 1)`
/// - hex: `#A552E6`
pub const XKCD_LIGHTISH_PURPLE: Color = Color(165, 82, 230);

/// Colour `(r = 175, g = 47, b = 13)`
///
/// Colour `XKCD_RUSTY_RED` from the set `CSS4_COLORS`. (Colour number `403`)
/// ## Representations:
/// - int tuple `(175, 47, 13)`
/// - float tuple `(0.69, 0, 0)`
/// - hex: `#AF2F0D`
pub const XKCD_RUSTY_RED: Color = Color(175, 47, 13);

/// Colour `(r = 139, g = 136, b = 248)`
///
/// Colour `XKCD_LAVENDER_BLUE` from the set `CSS4_COLORS`. (Colour number `404`)
/// ## Representations:
/// - int tuple `(139, 136, 248)`
/// - float tuple `(0.55, 1, 1)`
/// - hex: `#8B88F8`
pub const XKCD_LAVENDER_BLUE: Color = Color(139, 136, 248);

/// Colour `(r = 154, g = 247, b = 100)`
///
/// Colour `XKCD_LIGHT_GRASS_GREEN` from the set `CSS4_COLORS`. (Colour number `405`)
/// ## Representations:
/// - int tuple `(154, 247, 100)`
/// - float tuple `(0.6, 1, 0)`
/// - hex: `#9AF764`
pub const XKCD_LIGHT_GRASS_GREEN: Color = Color(154, 247, 100);

/// Colour `(r = 166, g = 251, b = 178)`
///
/// Colour `XKCD_LIGHT_MINT_GREEN` from the set `CSS4_COLORS`. (Colour number `406`)
/// ## Representations:
/// - int tuple `(166, 251, 178)`
/// - float tuple `(0.65, 1, 1)`
/// - hex: `#A6FBB2`
pub const XKCD_LIGHT_MINT_GREEN: Color = Color(166, 251, 178);

/// Colour `(r = 255, g = 197, b = 18)`
///
/// Colour `XKCD_SUNFLOWER` from the set `CSS4_COLORS`. (Colour number `407`)
/// ## Representations:
/// - int tuple `(255, 197, 18)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFC512`
pub const XKCD_SUNFLOWER: Color = Color(255, 197, 18);

/// Colour `(r = 117, g = 8, b = 81)`
///
/// Colour `XKCD_VELVET` from the set `CSS4_COLORS`. (Colour number `408`)
/// ## Representations:
/// - int tuple `(117, 8, 81)`
/// - float tuple `(0.46, 0, 0)`
/// - hex: `#750851`
pub const XKCD_VELVET: Color = Color(117, 8, 81);

/// Colour `(r = 193, g = 74, b = 9)`
///
/// Colour `XKCD_BRICK_ORANGE` from the set `CSS4_COLORS`. (Colour number `409`)
/// ## Representations:
/// - int tuple `(193, 74, 9)`
/// - float tuple `(0.76, 0, 0)`
/// - hex: `#C14A09`
pub const XKCD_BRICK_ORANGE: Color = Color(193, 74, 9);

/// Colour `(r = 254, g = 47, b = 74)`
///
/// Colour `XKCD_LIGHTISH_RED` from the set `CSS4_COLORS`. (Colour number `410`)
/// ## Representations:
/// - int tuple `(254, 47, 74)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FE2F4A`
pub const XKCD_LIGHTISH_RED: Color = Color(254, 47, 74);

/// Colour `(r = 2, g = 3, b = 226)`
///
/// Colour `XKCD_PURE_BLUE` from the set `CSS4_COLORS`. (Colour number `411`)
/// ## Representations:
/// - int tuple `(2, 3, 226)`
/// - float tuple `(0.01, 0, 1)`
/// - hex: `#0203E2`
pub const XKCD_PURE_BLUE: Color = Color(2, 3, 226);

/// Colour `(r = 10, g = 67, b = 122)`
///
/// Colour `XKCD_TWILIGHT_BLUE` from the set `CSS4_COLORS`. (Colour number `412`)
/// ## Representations:
/// - int tuple `(10, 67, 122)`
/// - float tuple `(0.04, 0, 0)`
/// - hex: `#0A437A`
pub const XKCD_TWILIGHT_BLUE: Color = Color(10, 67, 122);

/// Colour `(r = 165, g = 0, b = 85)`
///
/// Colour `XKCD_VIOLET_RED` from the set `CSS4_COLORS`. (Colour number `413`)
/// ## Representations:
/// - int tuple `(165, 0, 85)`
/// - float tuple `(0.65, 0, 0)`
/// - hex: `#A50055`
pub const XKCD_VIOLET_RED: Color = Color(165, 0, 85);

/// Colour `(r = 174, g = 139, b = 12)`
///
/// Colour `XKCD_YELLOWY_BROWN` from the set `CSS4_COLORS`. (Colour number `414`)
/// ## Representations:
/// - int tuple `(174, 139, 12)`
/// - float tuple `(0.68, 1, 0)`
/// - hex: `#AE8B0C`
pub const XKCD_YELLOWY_BROWN: Color = Color(174, 139, 12);

/// Colour `(r = 253, g = 121, b = 143)`
///
/// Colour `XKCD_CARNATION` from the set `CSS4_COLORS`. (Colour number `415`)
/// ## Representations:
/// - int tuple `(253, 121, 143)`
/// - float tuple `(0.99, 0, 1)`
/// - hex: `#FD798F`
pub const XKCD_CARNATION: Color = Color(253, 121, 143);

/// Colour `(r = 191, g = 172, b = 5)`
///
/// Colour `XKCD_MUDDY_YELLOW` from the set `CSS4_COLORS`. (Colour number `416`)
/// ## Representations:
/// - int tuple `(191, 172, 5)`
/// - float tuple `(0.75, 1, 0)`
/// - hex: `#BFAC05`
pub const XKCD_MUDDY_YELLOW: Color = Color(191, 172, 5);

/// Colour `(r = 62, g = 175, b = 118)`
///
/// Colour `XKCD_DARK_SEAFOAM_GREEN` from the set `CSS4_COLORS`. (Colour number `417`)
/// ## Representations:
/// - int tuple `(62, 175, 118)`
/// - float tuple `(0.24, 1, 0)`
/// - hex: `#3EAF76`
pub const XKCD_DARK_SEAFOAM_GREEN: Color = Color(62, 175, 118);

/// Colour `(r = 199, g = 71, b = 103)`
///
/// Colour `XKCD_DEEP_ROSE` from the set `CSS4_COLORS`. (Colour number `418`)
/// ## Representations:
/// - int tuple `(199, 71, 103)`
/// - float tuple `(0.78, 0, 0)`
/// - hex: `#C74767`
pub const XKCD_DEEP_ROSE: Color = Color(199, 71, 103);

/// Colour `(r = 185, g = 72, b = 78)`
///
/// Colour `XKCD_DUSTY_RED` from the set `CSS4_COLORS`. (Colour number `419`)
/// ## Representations:
/// - int tuple `(185, 72, 78)`
/// - float tuple `(0.73, 0, 0)`
/// - hex: `#B9484E`
pub const XKCD_DUSTY_RED: Color = Color(185, 72, 78);

/// Colour `(r = 100, g = 125, b = 142)`
///
/// Colour `XKCD_GREY_SLASH_BLUE` from the set `CSS4_COLORS`. (Colour number `420`)
/// ## Representations:
/// - int tuple `(100, 125, 142)`
/// - float tuple `(0.39, 0, 1)`
/// - hex: `#647D8E`
pub const XKCD_GREY_SLASH_BLUE: Color = Color(100, 125, 142);

/// Colour `(r = 191, g = 254, b = 40)`
///
/// Colour `XKCD_LEMON_LIME` from the set `CSS4_COLORS`. (Colour number `421`)
/// ## Representations:
/// - int tuple `(191, 254, 40)`
/// - float tuple `(0.75, 1, 0)`
/// - hex: `#BFFE28`
pub const XKCD_LEMON_LIME: Color = Color(191, 254, 40);

/// Colour `(r = 215, g = 37, b = 222)`
///
/// Colour `XKCD_PURPLE_SLASH_PINK` from the set `CSS4_COLORS`. (Colour number `422`)
/// ## Representations:
/// - int tuple `(215, 37, 222)`
/// - float tuple `(0.84, 0, 1)`
/// - hex: `#D725DE`
pub const XKCD_PURPLE_SLASH_PINK: Color = Color(215, 37, 222);

/// Colour `(r = 178, g = 151, b = 5)`
///
/// Colour `XKCD_BROWN_YELLOW` from the set `CSS4_COLORS`. (Colour number `423`)
/// ## Representations:
/// - int tuple `(178, 151, 5)`
/// - float tuple `(0.7, 1, 0)`
/// - hex: `#B29705`
pub const XKCD_BROWN_YELLOW: Color = Color(178, 151, 5);

/// Colour `(r = 103, g = 58, b = 63)`
///
/// Colour `XKCD_PURPLE_BROWN` from the set `CSS4_COLORS`. (Colour number `424`)
/// ## Representations:
/// - int tuple `(103, 58, 63)`
/// - float tuple `(0.4, 0, 0)`
/// - hex: `#673A3F`
pub const XKCD_PURPLE_BROWN: Color = Color(103, 58, 63);

/// Colour `(r = 168, g = 125, b = 194)`
///
/// Colour `XKCD_WISTERIA` from the set `CSS4_COLORS`. (Colour number `425`)
/// ## Representations:
/// - int tuple `(168, 125, 194)`
/// - float tuple `(0.66, 0, 1)`
/// - hex: `#A87DC2`
pub const XKCD_WISTERIA: Color = Color(168, 125, 194);

/// Colour `(r = 250, g = 254, b = 75)`
///
/// Colour `XKCD_BANANA_YELLOW` from the set `CSS4_COLORS`. (Colour number `426`)
/// ## Representations:
/// - int tuple `(250, 254, 75)`
/// - float tuple `(0.98, 1, 0)`
/// - hex: `#FAFE4B`
pub const XKCD_BANANA_YELLOW: Color = Color(250, 254, 75);

/// Colour `(r = 192, g = 2, b = 47)`
///
/// Colour `XKCD_LIPSTICK_RED` from the set `CSS4_COLORS`. (Colour number `427`)
/// ## Representations:
/// - int tuple `(192, 2, 47)`
/// - float tuple `(0.75, 0, 0)`
/// - hex: `#C0022F`
pub const XKCD_LIPSTICK_RED: Color = Color(192, 2, 47);

/// Colour `(r = 14, g = 135, b = 204)`
///
/// Colour `XKCD_WATER_BLUE` from the set `CSS4_COLORS`. (Colour number `428`)
/// ## Representations:
/// - int tuple `(14, 135, 204)`
/// - float tuple `(0.05, 1, 1)`
/// - hex: `#0E87CC`
pub const XKCD_WATER_BLUE: Color = Color(14, 135, 204);

/// Colour `(r = 141, g = 132, b = 104)`
///
/// Colour `XKCD_BROWN_GREY` from the set `CSS4_COLORS`. (Colour number `429`)
/// ## Representations:
/// - int tuple `(141, 132, 104)`
/// - float tuple `(0.55, 1, 0)`
/// - hex: `#8D8468`
pub const XKCD_BROWN_GREY: Color = Color(141, 132, 104);

/// Colour `(r = 173, g = 3, b = 222)`
///
/// Colour `XKCD_VIBRANT_PURPLE` from the set `CSS4_COLORS`. (Colour number `430`)
/// ## Representations:
/// - int tuple `(173, 3, 222)`
/// - float tuple `(0.68, 0, 1)`
/// - hex: `#AD03DE`
pub const XKCD_VIBRANT_PURPLE: Color = Color(173, 3, 222);

/// Colour `(r = 140, g = 255, b = 158)`
///
/// Colour `XKCD_BABY_GREEN` from the set `CSS4_COLORS`. (Colour number `431`)
/// ## Representations:
/// - int tuple `(140, 255, 158)`
/// - float tuple `(0.55, 1, 1)`
/// - hex: `#8CFF9E`
pub const XKCD_BABY_GREEN: Color = Color(140, 255, 158);

/// Colour `(r = 148, g = 172, b = 2)`
///
/// Colour `XKCD_BARF_GREEN` from the set `CSS4_COLORS`. (Colour number `432`)
/// ## Representations:
/// - int tuple `(148, 172, 2)`
/// - float tuple `(0.58, 1, 0)`
/// - hex: `#94AC02`
pub const XKCD_BARF_GREEN: Color = Color(148, 172, 2);

/// Colour `(r = 196, g = 255, b = 247)`
///
/// Colour `XKCD_EGGSHELL_BLUE` from the set `CSS4_COLORS`. (Colour number `433`)
/// ## Representations:
/// - int tuple `(196, 255, 247)`
/// - float tuple `(0.77, 1, 1)`
/// - hex: `#C4FFF7`
pub const XKCD_EGGSHELL_BLUE: Color = Color(196, 255, 247);

/// Colour `(r = 253, g = 238, b = 115)`
///
/// Colour `XKCD_SANDY_YELLOW` from the set `CSS4_COLORS`. (Colour number `434`)
/// ## Representations:
/// - int tuple `(253, 238, 115)`
/// - float tuple `(0.99, 1, 0)`
/// - hex: `#FDEE73`
pub const XKCD_SANDY_YELLOW: Color = Color(253, 238, 115);

/// Colour `(r = 51, g = 184, b = 100)`
///
/// Colour `XKCD_COOL_GREEN` from the set `CSS4_COLORS`. (Colour number `435`)
/// ## Representations:
/// - int tuple `(51, 184, 100)`
/// - float tuple `(0.2, 1, 0)`
/// - hex: `#33B864`
pub const XKCD_COOL_GREEN: Color = Color(51, 184, 100);

/// Colour `(r = 255, g = 249, b = 208)`
///
/// Colour `XKCD_PALE` from the set `CSS4_COLORS`. (Colour number `436`)
/// ## Representations:
/// - int tuple `(255, 249, 208)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFF9D0`
pub const XKCD_PALE: Color = Color(255, 249, 208);

/// Colour `(r = 117, g = 141, b = 163)`
///
/// Colour `XKCD_BLUE_SLASH_GREY` from the set `CSS4_COLORS`. (Colour number `437`)
/// ## Representations:
/// - int tuple `(117, 141, 163)`
/// - float tuple `(0.46, 1, 1)`
/// - hex: `#758DA3`
pub const XKCD_BLUE_SLASH_GREY: Color = Color(117, 141, 163);

/// Colour `(r = 245, g = 4, b = 201)`
///
/// Colour `XKCD_HOT_MAGENTA` from the set `CSS4_COLORS`. (Colour number `438`)
/// ## Representations:
/// - int tuple `(245, 4, 201)`
/// - float tuple `(0.96, 0, 1)`
/// - hex: `#F504C9`
pub const XKCD_HOT_MAGENTA: Color = Color(245, 4, 201);

/// Colour `(r = 119, g = 161, b = 181)`
///
/// Colour `XKCD_GREYBLUE` from the set `CSS4_COLORS`. (Colour number `439`)
/// ## Representations:
/// - int tuple `(119, 161, 181)`
/// - float tuple `(0.47, 1, 1)`
/// - hex: `#77A1B5`
pub const XKCD_GREYBLUE: Color = Color(119, 161, 181);

/// Colour `(r = 135, g = 86, b = 228)`
///
/// Colour `XKCD_PURPLEY` from the set `CSS4_COLORS`. (Colour number `440`)
/// ## Representations:
/// - int tuple `(135, 86, 228)`
/// - float tuple `(0.53, 0, 1)`
/// - hex: `#8756E4`
pub const XKCD_PURPLEY: Color = Color(135, 86, 228);

/// Colour `(r = 136, g = 151, b = 23)`
///
/// Colour `XKCD_BABY_SHIT_GREEN` from the set `CSS4_COLORS`. (Colour number `441`)
/// ## Representations:
/// - int tuple `(136, 151, 23)`
/// - float tuple `(0.53, 1, 0)`
/// - hex: `#889717`
pub const XKCD_BABY_SHIT_GREEN: Color = Color(136, 151, 23);

/// Colour `(r = 194, g = 126, b = 121)`
///
/// Colour `XKCD_BROWNISH_PINK` from the set `CSS4_COLORS`. (Colour number `442`)
/// ## Representations:
/// - int tuple `(194, 126, 121)`
/// - float tuple `(0.76, 0, 0)`
/// - hex: `#C27E79`
pub const XKCD_BROWNISH_PINK: Color = Color(194, 126, 121);

/// Colour `(r = 1, g = 115, b = 113)`
///
/// Colour `XKCD_DARK_AQUAMARINE` from the set `CSS4_COLORS`. (Colour number `443`)
/// ## Representations:
/// - int tuple `(1, 115, 113)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#017371`
pub const XKCD_DARK_AQUAMARINE: Color = Color(1, 115, 113);

/// Colour `(r = 159, g = 131, b = 3)`
///
/// Colour `XKCD_DIARRHEA` from the set `CSS4_COLORS`. (Colour number `444`)
/// ## Representations:
/// - int tuple `(159, 131, 3)`
/// - float tuple `(0.62, 1, 0)`
/// - hex: `#9F8303`
pub const XKCD_DIARRHEA: Color = Color(159, 131, 3);

/// Colour `(r = 247, g = 213, b = 96)`
///
/// Colour `XKCD_LIGHT_MUSTARD` from the set `CSS4_COLORS`. (Colour number `445`)
/// ## Representations:
/// - int tuple `(247, 213, 96)`
/// - float tuple `(0.97, 1, 0)`
/// - hex: `#F7D560`
pub const XKCD_LIGHT_MUSTARD: Color = Color(247, 213, 96);

/// Colour `(r = 189, g = 246, b = 254)`
///
/// Colour `XKCD_PALE_SKY_BLUE` from the set `CSS4_COLORS`. (Colour number `446`)
/// ## Representations:
/// - int tuple `(189, 246, 254)`
/// - float tuple `(0.74, 1, 1)`
/// - hex: `#BDF6FE`
pub const XKCD_PALE_SKY_BLUE: Color = Color(189, 246, 254);

/// Colour `(r = 117, g = 184, b = 79)`
///
/// Colour `XKCD_TURTLE_GREEN` from the set `CSS4_COLORS`. (Colour number `447`)
/// ## Representations:
/// - int tuple `(117, 184, 79)`
/// - float tuple `(0.46, 1, 0)`
/// - hex: `#75B84F`
pub const XKCD_TURTLE_GREEN: Color = Color(117, 184, 79);

/// Colour `(r = 156, g = 187, b = 4)`
///
/// Colour `XKCD_BRIGHT_OLIVE` from the set `CSS4_COLORS`. (Colour number `448`)
/// ## Representations:
/// - int tuple `(156, 187, 4)`
/// - float tuple `(0.61, 1, 0)`
/// - hex: `#9CBB04`
pub const XKCD_BRIGHT_OLIVE: Color = Color(156, 187, 4);

/// Colour `(r = 41, g = 70, b = 91)`
///
/// Colour `XKCD_DARK_GREY_BLUE` from the set `CSS4_COLORS`. (Colour number `449`)
/// ## Representations:
/// - int tuple `(41, 70, 91)`
/// - float tuple `(0.16, 0, 0)`
/// - hex: `#29465B`
pub const XKCD_DARK_GREY_BLUE: Color = Color(41, 70, 91);

/// Colour `(r = 105, g = 96, b = 6)`
///
/// Colour `XKCD_GREENY_BROWN` from the set `CSS4_COLORS`. (Colour number `450`)
/// ## Representations:
/// - int tuple `(105, 96, 6)`
/// - float tuple `(0.41, 0, 0)`
/// - hex: `#696006`
pub const XKCD_GREENY_BROWN: Color = Color(105, 96, 6);

/// Colour `(r = 173, g = 248, b = 2)`
///
/// Colour `XKCD_LEMON_GREEN` from the set `CSS4_COLORS`. (Colour number `451`)
/// ## Representations:
/// - int tuple `(173, 248, 2)`
/// - float tuple `(0.68, 1, 0)`
/// - hex: `#ADF802`
pub const XKCD_LEMON_GREEN: Color = Color(173, 248, 2);

/// Colour `(r = 193, g = 198, b = 252)`
///
/// Colour `XKCD_LIGHT_PERIWINKLE` from the set `CSS4_COLORS`. (Colour number `452`)
/// ## Representations:
/// - int tuple `(193, 198, 252)`
/// - float tuple `(0.76, 1, 1)`
/// - hex: `#C1C6FC`
pub const XKCD_LIGHT_PERIWINKLE: Color = Color(193, 198, 252);

/// Colour `(r = 53, g = 173, b = 107)`
///
/// Colour `XKCD_SEAWEED_GREEN` from the set `CSS4_COLORS`. (Colour number `453`)
/// ## Representations:
/// - int tuple `(53, 173, 107)`
/// - float tuple `(0.21, 1, 0)`
/// - hex: `#35AD6B`
pub const XKCD_SEAWEED_GREEN: Color = Color(53, 173, 107);

/// Colour `(r = 255, g = 253, b = 55)`
///
/// Colour `XKCD_SUNSHINE_YELLOW` from the set `CSS4_COLORS`. (Colour number `454`)
/// ## Representations:
/// - int tuple `(255, 253, 55)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFFD37`
pub const XKCD_SUNSHINE_YELLOW: Color = Color(255, 253, 55);

/// Colour `(r = 164, g = 66, b = 160)`
///
/// Colour `XKCD_UGLY_PURPLE` from the set `CSS4_COLORS`. (Colour number `455`)
/// ## Representations:
/// - int tuple `(164, 66, 160)`
/// - float tuple `(0.64, 0, 1)`
/// - hex: `#A442A0`
pub const XKCD_UGLY_PURPLE: Color = Color(164, 66, 160);

/// Colour `(r = 243, g = 97, b = 150)`
///
/// Colour `XKCD_MEDIUM_PINK` from the set `CSS4_COLORS`. (Colour number `456`)
/// ## Representations:
/// - int tuple `(243, 97, 150)`
/// - float tuple `(0.95, 0, 1)`
/// - hex: `#F36196`
pub const XKCD_MEDIUM_PINK: Color = Color(243, 97, 150);

/// Colour `(r = 148, g = 119, b = 6)`
///
/// Colour `XKCD_PUKE_BROWN` from the set `CSS4_COLORS`. (Colour number `457`)
/// ## Representations:
/// - int tuple `(148, 119, 6)`
/// - float tuple `(0.58, 0, 0)`
/// - hex: `#947706`
pub const XKCD_PUKE_BROWN: Color = Color(148, 119, 6);

/// Colour `(r = 255, g = 244, b = 242)`
///
/// Colour `XKCD_VERY_LIGHT_PINK` from the set `CSS4_COLORS`. (Colour number `458`)
/// ## Representations:
/// - int tuple `(255, 244, 242)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFF4F2`
pub const XKCD_VERY_LIGHT_PINK: Color = Color(255, 244, 242);

/// Colour `(r = 30, g = 145, b = 103)`
///
/// Colour `XKCD_VIRIDIAN` from the set `CSS4_COLORS`. (Colour number `459`)
/// ## Representations:
/// - int tuple `(30, 145, 103)`
/// - float tuple `(0.12, 1, 0)`
/// - hex: `#1E9167`
pub const XKCD_VIRIDIAN: Color = Color(30, 145, 103);

/// Colour `(r = 181, g = 195, b = 6)`
///
/// Colour `XKCD_BILE` from the set `CSS4_COLORS`. (Colour number `460`)
/// ## Representations:
/// - int tuple `(181, 195, 6)`
/// - float tuple `(0.71, 1, 0)`
/// - hex: `#B5C306`
pub const XKCD_BILE: Color = Color(181, 195, 6);

/// Colour `(r = 254, g = 255, b = 127)`
///
/// Colour `XKCD_FADED_YELLOW` from the set `CSS4_COLORS`. (Colour number `461`)
/// ## Representations:
/// - int tuple `(254, 255, 127)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FEFF7F`
pub const XKCD_FADED_YELLOW: Color = Color(254, 255, 127);

/// Colour `(r = 207, g = 253, b = 188)`
///
/// Colour `XKCD_VERY_PALE_GREEN` from the set `CSS4_COLORS`. (Colour number `462`)
/// ## Representations:
/// - int tuple `(207, 253, 188)`
/// - float tuple `(0.81, 1, 1)`
/// - hex: `#CFFDBC`
pub const XKCD_VERY_PALE_GREEN: Color = Color(207, 253, 188);

/// Colour `(r = 10, g = 221, b = 8)`
///
/// Colour `XKCD_VIBRANT_GREEN` from the set `CSS4_COLORS`. (Colour number `463`)
/// ## Representations:
/// - int tuple `(10, 221, 8)`
/// - float tuple `(0.04, 1, 0)`
/// - hex: `#0ADD08`
pub const XKCD_VIBRANT_GREEN: Color = Color(10, 221, 8);

/// Colour `(r = 135, g = 253, b = 5)`
///
/// Colour `XKCD_BRIGHT_LIME` from the set `CSS4_COLORS`. (Colour number `464`)
/// ## Representations:
/// - int tuple `(135, 253, 5)`
/// - float tuple `(0.53, 1, 0)`
/// - hex: `#87FD05`
pub const XKCD_BRIGHT_LIME: Color = Color(135, 253, 5);

/// Colour `(r = 30, g = 248, b = 118)`
///
/// Colour `XKCD_SPEARMINT` from the set `CSS4_COLORS`. (Colour number `465`)
/// ## Representations:
/// - int tuple `(30, 248, 118)`
/// - float tuple `(0.12, 1, 0)`
/// - hex: `#1EF876`
pub const XKCD_SPEARMINT: Color = Color(30, 248, 118);

/// Colour `(r = 123, g = 253, b = 199)`
///
/// Colour `XKCD_LIGHT_AQUAMARINE` from the set `CSS4_COLORS`. (Colour number `466`)
/// ## Representations:
/// - int tuple `(123, 253, 199)`
/// - float tuple `(0.48, 1, 1)`
/// - hex: `#7BFDC7`
pub const XKCD_LIGHT_AQUAMARINE: Color = Color(123, 253, 199);

/// Colour `(r = 188, g = 236, b = 172)`
///
/// Colour `XKCD_LIGHT_SAGE` from the set `CSS4_COLORS`. (Colour number `467`)
/// ## Representations:
/// - int tuple `(188, 236, 172)`
/// - float tuple `(0.74, 1, 1)`
/// - hex: `#BCECAC`
pub const XKCD_LIGHT_SAGE: Color = Color(188, 236, 172);

/// Colour `(r = 187, g = 249, b = 15)`
///
/// Colour `XKCD_YELLOWGREEN` from the set `CSS4_COLORS`. (Colour number `468`)
/// ## Representations:
/// - int tuple `(187, 249, 15)`
/// - float tuple `(0.73, 1, 0)`
/// - hex: `#BBF90F`
pub const XKCD_YELLOWGREEN: Color = Color(187, 249, 15);

/// Colour `(r = 171, g = 144, b = 4)`
///
/// Colour `XKCD_BABY_POO` from the set `CSS4_COLORS`. (Colour number `469`)
/// ## Representations:
/// - int tuple `(171, 144, 4)`
/// - float tuple `(0.67, 1, 0)`
/// - hex: `#AB9004`
pub const XKCD_BABY_POO: Color = Color(171, 144, 4);

/// Colour `(r = 31, g = 181, b = 122)`
///
/// Colour `XKCD_DARK_SEAFOAM` from the set `CSS4_COLORS`. (Colour number `470`)
/// ## Representations:
/// - int tuple `(31, 181, 122)`
/// - float tuple `(0.12, 1, 0)`
/// - hex: `#1FB57A`
pub const XKCD_DARK_SEAFOAM: Color = Color(31, 181, 122);

/// Colour `(r = 0, g = 85, b = 90)`
///
/// Colour `XKCD_DEEP_TEAL` from the set `CSS4_COLORS`. (Colour number `471`)
/// ## Representations:
/// - int tuple `(0, 85, 90)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#00555A`
pub const XKCD_DEEP_TEAL: Color = Color(0, 85, 90);

/// Colour `(r = 164, g = 132, b = 172)`
///
/// Colour `XKCD_HEATHER` from the set `CSS4_COLORS`. (Colour number `472`)
/// ## Representations:
/// - int tuple `(164, 132, 172)`
/// - float tuple `(0.64, 1, 1)`
/// - hex: `#A484AC`
pub const XKCD_HEATHER: Color = Color(164, 132, 172);

/// Colour `(r = 196, g = 85, b = 8)`
///
/// Colour `XKCD_RUST_ORANGE` from the set `CSS4_COLORS`. (Colour number `473`)
/// ## Representations:
/// - int tuple `(196, 85, 8)`
/// - float tuple `(0.77, 0, 0)`
/// - hex: `#C45508`
pub const XKCD_RUST_ORANGE: Color = Color(196, 85, 8);

/// Colour `(r = 63, g = 130, b = 157)`
///
/// Colour `XKCD_DIRTY_BLUE` from the set `CSS4_COLORS`. (Colour number `474`)
/// ## Representations:
/// - int tuple `(63, 130, 157)`
/// - float tuple `(0.25, 1, 1)`
/// - hex: `#3F829D`
pub const XKCD_DIRTY_BLUE: Color = Color(63, 130, 157);

/// Colour `(r = 84, g = 141, b = 68)`
///
/// Colour `XKCD_FERN_GREEN` from the set `CSS4_COLORS`. (Colour number `475`)
/// ## Representations:
/// - int tuple `(84, 141, 68)`
/// - float tuple `(0.33, 1, 0)`
/// - hex: `#548D44`
pub const XKCD_FERN_GREEN: Color = Color(84, 141, 68);

/// Colour `(r = 201, g = 94, b = 251)`
///
/// Colour `XKCD_BRIGHT_LILAC` from the set `CSS4_COLORS`. (Colour number `476`)
/// ## Representations:
/// - int tuple `(201, 94, 251)`
/// - float tuple `(0.79, 0, 1)`
/// - hex: `#C95EFB`
pub const XKCD_BRIGHT_LILAC: Color = Color(201, 94, 251);

/// Colour `(r = 58, g = 229, b = 127)`
///
/// Colour `XKCD_WEIRD_GREEN` from the set `CSS4_COLORS`. (Colour number `477`)
/// ## Representations:
/// - int tuple `(58, 229, 127)`
/// - float tuple `(0.23, 1, 0)`
/// - hex: `#3AE57F`
pub const XKCD_WEIRD_GREEN: Color = Color(58, 229, 127);

/// Colour `(r = 1, g = 103, b = 149)`
///
/// Colour `XKCD_PEACOCK_BLUE` from the set `CSS4_COLORS`. (Colour number `478`)
/// ## Representations:
/// - int tuple `(1, 103, 149)`
/// - float tuple `(0.0, 0, 1)`
/// - hex: `#016795`
pub const XKCD_PEACOCK_BLUE: Color = Color(1, 103, 149);

/// Colour `(r = 135, g = 169, b = 34)`
///
/// Colour `XKCD_AVOCADO_GREEN` from the set `CSS4_COLORS`. (Colour number `479`)
/// ## Representations:
/// - int tuple `(135, 169, 34)`
/// - float tuple `(0.53, 1, 0)`
/// - hex: `#87A922`
pub const XKCD_AVOCADO_GREEN: Color = Color(135, 169, 34);

/// Colour `(r = 240, g = 148, b = 77)`
///
/// Colour `XKCD_FADED_ORANGE` from the set `CSS4_COLORS`. (Colour number `480`)
/// ## Representations:
/// - int tuple `(240, 148, 77)`
/// - float tuple `(0.94, 1, 0)`
/// - hex: `#F0944D`
pub const XKCD_FADED_ORANGE: Color = Color(240, 148, 77);

/// Colour `(r = 93, g = 20, b = 81)`
///
/// Colour `XKCD_GRAPE_PURPLE` from the set `CSS4_COLORS`. (Colour number `481`)
/// ## Representations:
/// - int tuple `(93, 20, 81)`
/// - float tuple `(0.36, 0, 0)`
/// - hex: `#5D1451`
pub const XKCD_GRAPE_PURPLE: Color = Color(93, 20, 81);

/// Colour `(r = 37, g = 255, b = 41)`
///
/// Colour `XKCD_HOT_GREEN` from the set `CSS4_COLORS`. (Colour number `482`)
/// ## Representations:
/// - int tuple `(37, 255, 41)`
/// - float tuple `(0.15, 1, 0)`
/// - hex: `#25FF29`
pub const XKCD_HOT_GREEN: Color = Color(37, 255, 41);

/// Colour `(r = 208, g = 254, b = 29)`
///
/// Colour `XKCD_LIME_YELLOW` from the set `CSS4_COLORS`. (Colour number `483`)
/// ## Representations:
/// - int tuple `(208, 254, 29)`
/// - float tuple `(0.82, 1, 0)`
/// - hex: `#D0FE1D`
pub const XKCD_LIME_YELLOW: Color = Color(208, 254, 29);

/// Colour `(r = 255, g = 166, b = 43)`
///
/// Colour `XKCD_MANGO` from the set `CSS4_COLORS`. (Colour number `484`)
/// ## Representations:
/// - int tuple `(255, 166, 43)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFA62B`
pub const XKCD_MANGO: Color = Color(255, 166, 43);

/// Colour `(r = 1, g = 180, b = 76)`
///
/// Colour `XKCD_SHAMROCK` from the set `CSS4_COLORS`. (Colour number `485`)
/// ## Representations:
/// - int tuple `(1, 180, 76)`
/// - float tuple `(0.0, 1, 0)`
/// - hex: `#01B44C`
pub const XKCD_SHAMROCK: Color = Color(1, 180, 76);

/// Colour `(r = 255, g = 108, b = 181)`
///
/// Colour `XKCD_BUBBLEGUM` from the set `CSS4_COLORS`. (Colour number `486`)
/// ## Representations:
/// - int tuple `(255, 108, 181)`
/// - float tuple `(1.0, 0, 1)`
/// - hex: `#FF6CB5`
pub const XKCD_BUBBLEGUM: Color = Color(255, 108, 181);

/// Colour `(r = 107, g = 66, b = 71)`
///
/// Colour `XKCD_PURPLISH_BROWN` from the set `CSS4_COLORS`. (Colour number `487`)
/// ## Representations:
/// - int tuple `(107, 66, 71)`
/// - float tuple `(0.42, 0, 0)`
/// - hex: `#6B4247`
pub const XKCD_PURPLISH_BROWN: Color = Color(107, 66, 71);

/// Colour `(r = 199, g = 193, b = 12)`
///
/// Colour `XKCD_VOMIT_YELLOW` from the set `CSS4_COLORS`. (Colour number `488`)
/// ## Representations:
/// - int tuple `(199, 193, 12)`
/// - float tuple `(0.78, 1, 0)`
/// - hex: `#C7C10C`
pub const XKCD_VOMIT_YELLOW: Color = Color(199, 193, 12);

/// Colour `(r = 183, g = 255, b = 250)`
///
/// Colour `XKCD_PALE_CYAN` from the set `CSS4_COLORS`. (Colour number `489`)
/// ## Representations:
/// - int tuple `(183, 255, 250)`
/// - float tuple `(0.72, 1, 1)`
/// - hex: `#B7FFFA`
pub const XKCD_PALE_CYAN: Color = Color(183, 255, 250);

/// Colour `(r = 174, g = 255, b = 110)`
///
/// Colour `XKCD_KEY_LIME` from the set `CSS4_COLORS`. (Colour number `490`)
/// ## Representations:
/// - int tuple `(174, 255, 110)`
/// - float tuple `(0.68, 1, 0)`
/// - hex: `#AEFF6E`
pub const XKCD_KEY_LIME: Color = Color(174, 255, 110);

/// Colour `(r = 236, g = 45, b = 1)`
///
/// Colour `XKCD_TOMATO_RED` from the set `CSS4_COLORS`. (Colour number `491`)
/// ## Representations:
/// - int tuple `(236, 45, 1)`
/// - float tuple `(0.93, 0, 0)`
/// - hex: `#EC2D01`
pub const XKCD_TOMATO_RED: Color = Color(236, 45, 1);

/// Colour `(r = 118, g = 255, b = 123)`
///
/// Colour `XKCD_LIGHTGREEN` from the set `CSS4_COLORS`. (Colour number `492`)
/// ## Representations:
/// - int tuple `(118, 255, 123)`
/// - float tuple `(0.46, 1, 0)`
/// - hex: `#76FF7B`
pub const XKCD_LIGHTGREEN: Color = Color(118, 255, 123);

/// Colour `(r = 115, g = 0, b = 57)`
///
/// Colour `XKCD_MERLOT` from the set `CSS4_COLORS`. (Colour number `493`)
/// ## Representations:
/// - int tuple `(115, 0, 57)`
/// - float tuple `(0.45, 0, 0)`
/// - hex: `#730039`
pub const XKCD_MERLOT: Color = Color(115, 0, 57);

/// Colour `(r = 4, g = 3, b = 72)`
///
/// Colour `XKCD_NIGHT_BLUE` from the set `CSS4_COLORS`. (Colour number `494`)
/// ## Representations:
/// - int tuple `(4, 3, 72)`
/// - float tuple `(0.02, 0, 0)`
/// - hex: `#040348`
pub const XKCD_NIGHT_BLUE: Color = Color(4, 3, 72);

/// Colour `(r = 223, g = 78, b = 200)`
///
/// Colour `XKCD_PURPLEISH_PINK` from the set `CSS4_COLORS`. (Colour number `495`)
/// ## Representations:
/// - int tuple `(223, 78, 200)`
/// - float tuple `(0.87, 0, 1)`
/// - hex: `#DF4EC8`
pub const XKCD_PURPLEISH_PINK: Color = Color(223, 78, 200);

/// Colour `(r = 110, g = 203, b = 60)`
///
/// Colour `XKCD_APPLE` from the set `CSS4_COLORS`. (Colour number `496`)
/// ## Representations:
/// - int tuple `(110, 203, 60)`
/// - float tuple `(0.43, 1, 0)`
/// - hex: `#6ECB3C`
pub const XKCD_APPLE: Color = Color(110, 203, 60);

/// Colour `(r = 143, g = 152, b = 5)`
///
/// Colour `XKCD_BABY_POOP_GREEN` from the set `CSS4_COLORS`. (Colour number `497`)
/// ## Representations:
/// - int tuple `(143, 152, 5)`
/// - float tuple `(0.56, 1, 0)`
/// - hex: `#8F9805`
pub const XKCD_BABY_POOP_GREEN: Color = Color(143, 152, 5);

/// Colour `(r = 94, g = 220, b = 31)`
///
/// Colour `XKCD_GREEN_APPLE` from the set `CSS4_COLORS`. (Colour number `498`)
/// ## Representations:
/// - int tuple `(94, 220, 31)`
/// - float tuple `(0.37, 1, 0)`
/// - hex: `#5EDC1F`
pub const XKCD_GREEN_APPLE: Color = Color(94, 220, 31);

/// Colour `(r = 217, g = 79, b = 245)`
///
/// Colour `XKCD_HELIOTROPE` from the set `CSS4_COLORS`. (Colour number `499`)
/// ## Representations:
/// - int tuple `(217, 79, 245)`
/// - float tuple `(0.85, 0, 1)`
/// - hex: `#D94FF5`
pub const XKCD_HELIOTROPE: Color = Color(217, 79, 245);

/// Colour `(r = 200, g = 253, b = 61)`
///
/// Colour `XKCD_YELLOW_SLASH_GREEN` from the set `CSS4_COLORS`. (Colour number `500`)
/// ## Representations:
/// - int tuple `(200, 253, 61)`
/// - float tuple `(0.78, 1, 0)`
/// - hex: `#C8FD3D`
pub const XKCD_YELLOW_SLASH_GREEN: Color = Color(200, 253, 61);

/// Colour `(r = 7, g = 13, b = 13)`
///
/// Colour `XKCD_ALMOST_BLACK` from the set `CSS4_COLORS`. (Colour number `501`)
/// ## Representations:
/// - int tuple `(7, 13, 13)`
/// - float tuple `(0.03, 0, 0)`
/// - hex: `#070D0D`
pub const XKCD_ALMOST_BLACK: Color = Color(7, 13, 13);

/// Colour `(r = 73, g = 132, b = 184)`
///
/// Colour `XKCD_COOL_BLUE` from the set `CSS4_COLORS`. (Colour number `502`)
/// ## Representations:
/// - int tuple `(73, 132, 184)`
/// - float tuple `(0.29, 1, 1)`
/// - hex: `#4984B8`
pub const XKCD_COOL_BLUE: Color = Color(73, 132, 184);

/// Colour `(r = 81, g = 183, b = 59)`
///
/// Colour `XKCD_LEAFY_GREEN` from the set `CSS4_COLORS`. (Colour number `503`)
/// ## Representations:
/// - int tuple `(81, 183, 59)`
/// - float tuple `(0.32, 1, 0)`
/// - hex: `#51B73B`
pub const XKCD_LEAFY_GREEN: Color = Color(81, 183, 59);

/// Colour `(r = 172, g = 126, b = 4)`
///
/// Colour `XKCD_MUSTARD_BROWN` from the set `CSS4_COLORS`. (Colour number `504`)
/// ## Representations:
/// - int tuple `(172, 126, 4)`
/// - float tuple `(0.67, 0, 0)`
/// - hex: `#AC7E04`
pub const XKCD_MUSTARD_BROWN: Color = Color(172, 126, 4);

/// Colour `(r = 78, g = 84, b = 129)`
///
/// Colour `XKCD_DUSK` from the set `CSS4_COLORS`. (Colour number `505`)
/// ## Representations:
/// - int tuple `(78, 84, 129)`
/// - float tuple `(0.31, 0, 1)`
/// - hex: `#4E5481`
pub const XKCD_DUSK: Color = Color(78, 84, 129);

/// Colour `(r = 135, g = 110, b = 75)`
///
/// Colour `XKCD_DULL_BROWN` from the set `CSS4_COLORS`. (Colour number `506`)
/// ## Representations:
/// - int tuple `(135, 110, 75)`
/// - float tuple `(0.53, 0, 0)`
/// - hex: `#876E4B`
pub const XKCD_DULL_BROWN: Color = Color(135, 110, 75);

/// Colour `(r = 88, g = 188, b = 8)`
///
/// Colour `XKCD_FROG_GREEN` from the set `CSS4_COLORS`. (Colour number `507`)
/// ## Representations:
/// - int tuple `(88, 188, 8)`
/// - float tuple `(0.35, 1, 0)`
/// - hex: `#58BC08`
pub const XKCD_FROG_GREEN: Color = Color(88, 188, 8);

/// Colour `(r = 47, g = 239, b = 16)`
///
/// Colour `XKCD_VIVID_GREEN` from the set `CSS4_COLORS`. (Colour number `508`)
/// ## Representations:
/// - int tuple `(47, 239, 16)`
/// - float tuple `(0.18, 1, 0)`
/// - hex: `#2FEF10`
pub const XKCD_VIVID_GREEN: Color = Color(47, 239, 16);

/// Colour `(r = 45, g = 254, b = 84)`
///
/// Colour `XKCD_BRIGHT_LIGHT_GREEN` from the set `CSS4_COLORS`. (Colour number `509`)
/// ## Representations:
/// - int tuple `(45, 254, 84)`
/// - float tuple `(0.18, 1, 0)`
/// - hex: `#2DFE54`
pub const XKCD_BRIGHT_LIGHT_GREEN: Color = Color(45, 254, 84);

/// Colour `(r = 10, g = 255, b = 2)`
///
/// Colour `XKCD_FLURO_GREEN` from the set `CSS4_COLORS`. (Colour number `510`)
/// ## Representations:
/// - int tuple `(10, 255, 2)`
/// - float tuple `(0.04, 1, 0)`
/// - hex: `#0AFF02`
pub const XKCD_FLURO_GREEN: Color = Color(10, 255, 2);

/// Colour `(r = 156, g = 239, b = 67)`
///
/// Colour `XKCD_KIWI` from the set `CSS4_COLORS`. (Colour number `511`)
/// ## Representations:
/// - int tuple `(156, 239, 67)`
/// - float tuple `(0.61, 1, 0)`
/// - hex: `#9CEF43`
pub const XKCD_KIWI: Color = Color(156, 239, 67);

/// Colour `(r = 24, g = 209, b = 123)`
///
/// Colour `XKCD_SEAWEED` from the set `CSS4_COLORS`. (Colour number `512`)
/// ## Representations:
/// - int tuple `(24, 209, 123)`
/// - float tuple `(0.09, 1, 0)`
/// - hex: `#18D17B`
pub const XKCD_SEAWEED: Color = Color(24, 209, 123);

/// Colour `(r = 53, g = 83, b = 10)`
///
/// Colour `XKCD_NAVY_GREEN` from the set `CSS4_COLORS`. (Colour number `513`)
/// ## Representations:
/// - int tuple `(53, 83, 10)`
/// - float tuple `(0.21, 0, 0)`
/// - hex: `#35530A`
pub const XKCD_NAVY_GREEN: Color = Color(53, 83, 10);

/// Colour `(r = 24, g = 5, b = 219)`
///
/// Colour `XKCD_ULTRAMARINE_BLUE` from the set `CSS4_COLORS`. (Colour number `514`)
/// ## Representations:
/// - int tuple `(24, 5, 219)`
/// - float tuple `(0.09, 0, 1)`
/// - hex: `#1805DB`
pub const XKCD_ULTRAMARINE_BLUE: Color = Color(24, 5, 219);

/// Colour `(r = 98, g = 88, b = 196)`
///
/// Colour `XKCD_IRIS` from the set `CSS4_COLORS`. (Colour number `515`)
/// ## Representations:
/// - int tuple `(98, 88, 196)`
/// - float tuple `(0.38, 0, 1)`
/// - hex: `#6258C4`
pub const XKCD_IRIS: Color = Color(98, 88, 196);

/// Colour `(r = 255, g = 150, b = 79)`
///
/// Colour `XKCD_PASTEL_ORANGE` from the set `CSS4_COLORS`. (Colour number `516`)
/// ## Representations:
/// - int tuple `(255, 150, 79)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FF964F`
pub const XKCD_PASTEL_ORANGE: Color = Color(255, 150, 79);

/// Colour `(r = 255, g = 171, b = 15)`
///
/// Colour `XKCD_YELLOWISH_ORANGE` from the set `CSS4_COLORS`. (Colour number `517`)
/// ## Representations:
/// - int tuple `(255, 171, 15)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFAB0F`
pub const XKCD_YELLOWISH_ORANGE: Color = Color(255, 171, 15);

/// Colour `(r = 143, g = 140, b = 231)`
///
/// Colour `XKCD_PERRYWINKLE` from the set `CSS4_COLORS`. (Colour number `518`)
/// ## Representations:
/// - int tuple `(143, 140, 231)`
/// - float tuple `(0.56, 1, 1)`
/// - hex: `#8F8CE7`
pub const XKCD_PERRYWINKLE: Color = Color(143, 140, 231);

/// Colour `(r = 36, g = 188, b = 168)`
///
/// Colour `XKCD_TEALISH` from the set `CSS4_COLORS`. (Colour number `519`)
/// ## Representations:
/// - int tuple `(36, 188, 168)`
/// - float tuple `(0.14, 1, 1)`
/// - hex: `#24BCA8`
pub const XKCD_TEALISH: Color = Color(36, 188, 168);

/// Colour `(r = 63, g = 1, b = 44)`
///
/// Colour `XKCD_DARK_PLUM` from the set `CSS4_COLORS`. (Colour number `520`)
/// ## Representations:
/// - int tuple `(63, 1, 44)`
/// - float tuple `(0.25, 0, 0)`
/// - hex: `#3F012C`
pub const XKCD_DARK_PLUM: Color = Color(63, 1, 44);

/// Colour `(r = 203, g = 248, b = 95)`
///
/// Colour `XKCD_PEAR` from the set `CSS4_COLORS`. (Colour number `521`)
/// ## Representations:
/// - int tuple `(203, 248, 95)`
/// - float tuple `(0.8, 1, 0)`
/// - hex: `#CBF85F`
pub const XKCD_PEAR: Color = Color(203, 248, 95);

/// Colour `(r = 255, g = 114, b = 76)`
///
/// Colour `XKCD_PINKISH_ORANGE` from the set `CSS4_COLORS`. (Colour number `522`)
/// ## Representations:
/// - int tuple `(255, 114, 76)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FF724C`
pub const XKCD_PINKISH_ORANGE: Color = Color(255, 114, 76);

/// Colour `(r = 40, g = 1, b = 55)`
///
/// Colour `XKCD_MIDNIGHT_PURPLE` from the set `CSS4_COLORS`. (Colour number `523`)
/// ## Representations:
/// - int tuple `(40, 1, 55)`
/// - float tuple `(0.16, 0, 0)`
/// - hex: `#280137`
pub const XKCD_MIDNIGHT_PURPLE: Color = Color(40, 1, 55);

/// Colour `(r = 179, g = 111, b = 246)`
///
/// Colour `XKCD_LIGHT_URPLE` from the set `CSS4_COLORS`. (Colour number `524`)
/// ## Representations:
/// - int tuple `(179, 111, 246)`
/// - float tuple `(0.7, 0, 1)`
/// - hex: `#B36FF6`
pub const XKCD_LIGHT_URPLE: Color = Color(179, 111, 246);

/// Colour `(r = 72, g = 192, b = 114)`
///
/// Colour `XKCD_DARK_MINT` from the set `CSS4_COLORS`. (Colour number `525`)
/// ## Representations:
/// - int tuple `(72, 192, 114)`
/// - float tuple `(0.28, 1, 0)`
/// - hex: `#48C072`
pub const XKCD_DARK_MINT: Color = Color(72, 192, 114);

/// Colour `(r = 188, g = 203, b = 122)`
///
/// Colour `XKCD_GREENISH_TAN` from the set `CSS4_COLORS`. (Colour number `526`)
/// ## Representations:
/// - int tuple `(188, 203, 122)`
/// - float tuple `(0.74, 1, 0)`
/// - hex: `#BCCB7A`
pub const XKCD_GREENISH_TAN: Color = Color(188, 203, 122);

/// Colour `(r = 168, g = 65, b = 91)`
///
/// Colour `XKCD_LIGHT_BURGUNDY` from the set `CSS4_COLORS`. (Colour number `527`)
/// ## Representations:
/// - int tuple `(168, 65, 91)`
/// - float tuple `(0.66, 0, 0)`
/// - hex: `#A8415B`
pub const XKCD_LIGHT_BURGUNDY: Color = Color(168, 65, 91);

/// Colour `(r = 6, g = 177, b = 196)`
///
/// Colour `XKCD_TURQUOISE_BLUE` from the set `CSS4_COLORS`. (Colour number `528`)
/// ## Representations:
/// - int tuple `(6, 177, 196)`
/// - float tuple `(0.02, 1, 1)`
/// - hex: `#06B1C4`
pub const XKCD_TURQUOISE_BLUE: Color = Color(6, 177, 196);

/// Colour `(r = 205, g = 117, b = 132)`
///
/// Colour `XKCD_UGLY_PINK` from the set `CSS4_COLORS`. (Colour number `529`)
/// ## Representations:
/// - int tuple `(205, 117, 132)`
/// - float tuple `(0.8, 0, 1)`
/// - hex: `#CD7584`
pub const XKCD_UGLY_PINK: Color = Color(205, 117, 132);

/// Colour `(r = 241, g = 218, b = 122)`
///
/// Colour `XKCD_SANDY` from the set `CSS4_COLORS`. (Colour number `530`)
/// ## Representations:
/// - int tuple `(241, 218, 122)`
/// - float tuple `(0.95, 1, 0)`
/// - hex: `#F1DA7A`
pub const XKCD_SANDY: Color = Color(241, 218, 122);

/// Colour `(r = 255, g = 4, b = 144)`
///
/// Colour `XKCD_ELECTRIC_PINK` from the set `CSS4_COLORS`. (Colour number `531`)
/// ## Representations:
/// - int tuple `(255, 4, 144)`
/// - float tuple `(1.0, 0, 1)`
/// - hex: `#FF0490`
pub const XKCD_ELECTRIC_PINK: Color = Color(255, 4, 144);

/// Colour `(r = 128, g = 91, b = 135)`
///
/// Colour `XKCD_MUTED_PURPLE` from the set `CSS4_COLORS`. (Colour number `532`)
/// ## Representations:
/// - int tuple `(128, 91, 135)`
/// - float tuple `(0.5, 0, 1)`
/// - hex: `#805B87`
pub const XKCD_MUTED_PURPLE: Color = Color(128, 91, 135);

/// Colour `(r = 80, g = 167, b = 71)`
///
/// Colour `XKCD_MID_GREEN` from the set `CSS4_COLORS`. (Colour number `533`)
/// ## Representations:
/// - int tuple `(80, 167, 71)`
/// - float tuple `(0.31, 1, 0)`
/// - hex: `#50A747`
pub const XKCD_MID_GREEN: Color = Color(80, 167, 71);

/// Colour `(r = 168, g = 164, b = 149)`
///
/// Colour `XKCD_GREYISH` from the set `CSS4_COLORS`. (Colour number `534`)
/// ## Representations:
/// - int tuple `(168, 164, 149)`
/// - float tuple `(0.66, 1, 1)`
/// - hex: `#A8A495`
pub const XKCD_GREYISH: Color = Color(168, 164, 149);

/// Colour `(r = 207, g = 255, b = 4)`
///
/// Colour `XKCD_NEON_YELLOW` from the set `CSS4_COLORS`. (Colour number `535`)
/// ## Representations:
/// - int tuple `(207, 255, 4)`
/// - float tuple `(0.81, 1, 0)`
/// - hex: `#CFFF04`
pub const XKCD_NEON_YELLOW: Color = Color(207, 255, 4);

/// Colour `(r = 255, g = 255, b = 126)`
///
/// Colour `XKCD_BANANA` from the set `CSS4_COLORS`. (Colour number `536`)
/// ## Representations:
/// - int tuple `(255, 255, 126)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFFF7E`
pub const XKCD_BANANA: Color = Color(255, 255, 126);

/// Colour `(r = 255, g = 127, b = 167)`
///
/// Colour `XKCD_CARNATION_PINK` from the set `CSS4_COLORS`. (Colour number `537`)
/// ## Representations:
/// - int tuple `(255, 127, 167)`
/// - float tuple `(1.0, 0, 1)`
/// - hex: `#FF7FA7`
pub const XKCD_CARNATION_PINK: Color = Color(255, 127, 167);

/// Colour `(r = 239, g = 64, b = 38)`
///
/// Colour `XKCD_TOMATO` from the set `CSS4_COLORS`. (Colour number `538`)
/// ## Representations:
/// - int tuple `(239, 64, 38)`
/// - float tuple `(0.94, 0, 0)`
/// - hex: `#EF4026`
pub const XKCD_TOMATO: Color = Color(239, 64, 38);

/// Colour `(r = 60, g = 153, b = 146)`
///
/// Colour `XKCD_SEA` from the set `CSS4_COLORS`. (Colour number `539`)
/// ## Representations:
/// - int tuple `(60, 153, 146)`
/// - float tuple `(0.24, 1, 1)`
/// - hex: `#3C9992`
pub const XKCD_SEA: Color = Color(60, 153, 146);

/// Colour `(r = 136, g = 104, b = 6)`
///
/// Colour `XKCD_MUDDY_BROWN` from the set `CSS4_COLORS`. (Colour number `540`)
/// ## Representations:
/// - int tuple `(136, 104, 6)`
/// - float tuple `(0.53, 0, 0)`
/// - hex: `#886806`
pub const XKCD_MUDDY_BROWN: Color = Color(136, 104, 6);

/// Colour `(r = 4, g = 244, b = 137)`
///
/// Colour `XKCD_TURQUOISE_GREEN` from the set `CSS4_COLORS`. (Colour number `541`)
/// ## Representations:
/// - int tuple `(4, 244, 137)`
/// - float tuple `(0.02, 1, 1)`
/// - hex: `#04F489`
pub const XKCD_TURQUOISE_GREEN: Color = Color(4, 244, 137);

/// Colour `(r = 254, g = 246, b = 158)`
///
/// Colour `XKCD_BUFF` from the set `CSS4_COLORS`. (Colour number `542`)
/// ## Representations:
/// - int tuple `(254, 246, 158)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FEF69E`
pub const XKCD_BUFF: Color = Color(254, 246, 158);

/// Colour `(r = 207, g = 175, b = 123)`
///
/// Colour `XKCD_FAWN` from the set `CSS4_COLORS`. (Colour number `543`)
/// ## Representations:
/// - int tuple `(207, 175, 123)`
/// - float tuple `(0.81, 1, 0)`
/// - hex: `#CFAF7B`
pub const XKCD_FAWN: Color = Color(207, 175, 123);

/// Colour `(r = 59, g = 113, b = 159)`
///
/// Colour `XKCD_MUTED_BLUE` from the set `CSS4_COLORS`. (Colour number `544`)
/// ## Representations:
/// - int tuple `(59, 113, 159)`
/// - float tuple `(0.23, 0, 1)`
/// - hex: `#3B719F`
pub const XKCD_MUTED_BLUE: Color = Color(59, 113, 159);

/// Colour `(r = 253, g = 193, b = 197)`
///
/// Colour `XKCD_PALE_ROSE` from the set `CSS4_COLORS`. (Colour number `545`)
/// ## Representations:
/// - int tuple `(253, 193, 197)`
/// - float tuple `(0.99, 1, 1)`
/// - hex: `#FDC1C5`
pub const XKCD_PALE_ROSE: Color = Color(253, 193, 197);

/// Colour `(r = 32, g = 192, b = 115)`
///
/// Colour `XKCD_DARK_MINT_GREEN` from the set `CSS4_COLORS`. (Colour number `546`)
/// ## Representations:
/// - int tuple `(32, 192, 115)`
/// - float tuple `(0.13, 1, 0)`
/// - hex: `#20C073`
pub const XKCD_DARK_MINT_GREEN: Color = Color(32, 192, 115);

/// Colour `(r = 155, g = 95, b = 192)`
///
/// Colour `XKCD_AMETHYST` from the set `CSS4_COLORS`. (Colour number `547`)
/// ## Representations:
/// - int tuple `(155, 95, 192)`
/// - float tuple `(0.61, 0, 1)`
/// - hex: `#9B5FC0`
pub const XKCD_AMETHYST: Color = Color(155, 95, 192);

/// Colour `(r = 15, g = 155, b = 142)`
///
/// Colour `XKCD_BLUE_SLASH_GREEN` from the set `CSS4_COLORS`. (Colour number `548`)
/// ## Representations:
/// - int tuple `(15, 155, 142)`
/// - float tuple `(0.06, 1, 1)`
/// - hex: `#0F9B8E`
pub const XKCD_BLUE_SLASH_GREEN: Color = Color(15, 155, 142);

/// Colour `(r = 116, g = 40, b = 2)`
///
/// Colour `XKCD_CHESTNUT` from the set `CSS4_COLORS`. (Colour number `549`)
/// ## Representations:
/// - int tuple `(116, 40, 2)`
/// - float tuple `(0.45, 0, 0)`
/// - hex: `#742802`
pub const XKCD_CHESTNUT: Color = Color(116, 40, 2);

/// Colour `(r = 157, g = 185, b = 44)`
///
/// Colour `XKCD_SICK_GREEN` from the set `CSS4_COLORS`. (Colour number `550`)
/// ## Representations:
/// - int tuple `(157, 185, 44)`
/// - float tuple `(0.62, 1, 0)`
/// - hex: `#9DB92C`
pub const XKCD_SICK_GREEN: Color = Color(157, 185, 44);

/// Colour `(r = 164, g = 191, b = 32)`
///
/// Colour `XKCD_PEA` from the set `CSS4_COLORS`. (Colour number `551`)
/// ## Representations:
/// - int tuple `(164, 191, 32)`
/// - float tuple `(0.64, 1, 0)`
/// - hex: `#A4BF20`
pub const XKCD_PEA: Color = Color(164, 191, 32);

/// Colour `(r = 205, g = 89, b = 9)`
///
/// Colour `XKCD_RUSTY_ORANGE` from the set `CSS4_COLORS`. (Colour number `552`)
/// ## Representations:
/// - int tuple `(205, 89, 9)`
/// - float tuple `(0.8, 0, 0)`
/// - hex: `#CD5909`
pub const XKCD_RUSTY_ORANGE: Color = Color(205, 89, 9);

/// Colour `(r = 173, g = 165, b = 135)`
///
/// Colour `XKCD_STONE` from the set `CSS4_COLORS`. (Colour number `553`)
/// ## Representations:
/// - int tuple `(173, 165, 135)`
/// - float tuple `(0.68, 1, 1)`
/// - hex: `#ADA587`
pub const XKCD_STONE: Color = Color(173, 165, 135);

/// Colour `(r = 190, g = 1, b = 60)`
///
/// Colour `XKCD_ROSE_RED` from the set `CSS4_COLORS`. (Colour number `554`)
/// ## Representations:
/// - int tuple `(190, 1, 60)`
/// - float tuple `(0.75, 0, 0)`
/// - hex: `#BE013C`
pub const XKCD_ROSE_RED: Color = Color(190, 1, 60);

/// Colour `(r = 184, g = 255, b = 235)`
///
/// Colour `XKCD_PALE_AQUA` from the set `CSS4_COLORS`. (Colour number `555`)
/// ## Representations:
/// - int tuple `(184, 255, 235)`
/// - float tuple `(0.72, 1, 1)`
/// - hex: `#B8FFEB`
pub const XKCD_PALE_AQUA: Color = Color(184, 255, 235);

/// Colour `(r = 220, g = 77, b = 1)`
///
/// Colour `XKCD_DEEP_ORANGE` from the set `CSS4_COLORS`. (Colour number `556`)
/// ## Representations:
/// - int tuple `(220, 77, 1)`
/// - float tuple `(0.86, 0, 0)`
/// - hex: `#DC4D01`
pub const XKCD_DEEP_ORANGE: Color = Color(220, 77, 1);

/// Colour `(r = 162, g = 101, b = 62)`
///
/// Colour `XKCD_EARTH` from the set `CSS4_COLORS`. (Colour number `557`)
/// ## Representations:
/// - int tuple `(162, 101, 62)`
/// - float tuple `(0.64, 0, 0)`
/// - hex: `#A2653E`
pub const XKCD_EARTH: Color = Color(162, 101, 62);

/// Colour `(r = 99, g = 139, b = 39)`
///
/// Colour `XKCD_MOSSY_GREEN` from the set `CSS4_COLORS`. (Colour number `558`)
/// ## Representations:
/// - int tuple `(99, 139, 39)`
/// - float tuple `(0.39, 1, 0)`
/// - hex: `#638B27`
pub const XKCD_MOSSY_GREEN: Color = Color(99, 139, 39);

/// Colour `(r = 65, g = 156, b = 3)`
///
/// Colour `XKCD_GRASSY_GREEN` from the set `CSS4_COLORS`. (Colour number `559`)
/// ## Representations:
/// - int tuple `(65, 156, 3)`
/// - float tuple `(0.25, 1, 0)`
/// - hex: `#419C03`
pub const XKCD_GRASSY_GREEN: Color = Color(65, 156, 3);

/// Colour `(r = 177, g = 255, b = 101)`
///
/// Colour `XKCD_PALE_LIME_GREEN` from the set `CSS4_COLORS`. (Colour number `560`)
/// ## Representations:
/// - int tuple `(177, 255, 101)`
/// - float tuple `(0.69, 1, 0)`
/// - hex: `#B1FF65`
pub const XKCD_PALE_LIME_GREEN: Color = Color(177, 255, 101);

/// Colour `(r = 157, g = 188, b = 212)`
///
/// Colour `XKCD_LIGHT_GREY_BLUE` from the set `CSS4_COLORS`. (Colour number `561`)
/// ## Representations:
/// - int tuple `(157, 188, 212)`
/// - float tuple `(0.62, 1, 1)`
/// - hex: `#9DBCD4`
pub const XKCD_LIGHT_GREY_BLUE: Color = Color(157, 188, 212);

/// Colour `(r = 253, g = 253, b = 254)`
///
/// Colour `XKCD_PALE_GREY` from the set `CSS4_COLORS`. (Colour number `562`)
/// ## Representations:
/// - int tuple `(253, 253, 254)`
/// - float tuple `(0.99, 1, 1)`
/// - hex: `#FDFDFE`
pub const XKCD_PALE_GREY: Color = Color(253, 253, 254);

/// Colour `(r = 119, g = 171, b = 86)`
///
/// Colour `XKCD_ASPARAGUS` from the set `CSS4_COLORS`. (Colour number `563`)
/// ## Representations:
/// - int tuple `(119, 171, 86)`
/// - float tuple `(0.47, 1, 0)`
/// - hex: `#77AB56`
pub const XKCD_ASPARAGUS: Color = Color(119, 171, 86);

/// Colour `(r = 70, g = 65, b = 150)`
///
/// Colour `XKCD_BLUEBERRY` from the set `CSS4_COLORS`. (Colour number `564`)
/// ## Representations:
/// - int tuple `(70, 65, 150)`
/// - float tuple `(0.27, 0, 1)`
/// - hex: `#464196`
pub const XKCD_BLUEBERRY: Color = Color(70, 65, 150);

/// Colour `(r = 153, g = 1, b = 71)`
///
/// Colour `XKCD_PURPLE_RED` from the set `CSS4_COLORS`. (Colour number `565`)
/// ## Representations:
/// - int tuple `(153, 1, 71)`
/// - float tuple `(0.6, 0, 0)`
/// - hex: `#990147`
pub const XKCD_PURPLE_RED: Color = Color(153, 1, 71);

/// Colour `(r = 190, g = 253, b = 115)`
///
/// Colour `XKCD_PALE_LIME` from the set `CSS4_COLORS`. (Colour number `566`)
/// ## Representations:
/// - int tuple `(190, 253, 115)`
/// - float tuple `(0.75, 1, 0)`
/// - hex: `#BEFD73`
pub const XKCD_PALE_LIME: Color = Color(190, 253, 115);

/// Colour `(r = 50, g = 191, b = 132)`
///
/// Colour `XKCD_GREENISH_TEAL` from the set `CSS4_COLORS`. (Colour number `567`)
/// ## Representations:
/// - int tuple `(50, 191, 132)`
/// - float tuple `(0.2, 1, 1)`
/// - hex: `#32BF84`
pub const XKCD_GREENISH_TEAL: Color = Color(50, 191, 132);

/// Colour `(r = 175, g = 111, b = 9)`
///
/// Colour `XKCD_CARAMEL` from the set `CSS4_COLORS`. (Colour number `568`)
/// ## Representations:
/// - int tuple `(175, 111, 9)`
/// - float tuple `(0.69, 0, 0)`
/// - hex: `#AF6F09`
pub const XKCD_CARAMEL: Color = Color(175, 111, 9);

/// Colour `(r = 160, g = 2, b = 92)`
///
/// Colour `XKCD_DEEP_MAGENTA` from the set `CSS4_COLORS`. (Colour number `569`)
/// ## Representations:
/// - int tuple `(160, 2, 92)`
/// - float tuple `(0.63, 0, 0)`
/// - hex: `#A0025C`
pub const XKCD_DEEP_MAGENTA: Color = Color(160, 2, 92);

/// Colour `(r = 255, g = 216, b = 177)`
///
/// Colour `XKCD_LIGHT_PEACH` from the set `CSS4_COLORS`. (Colour number `570`)
/// ## Representations:
/// - int tuple `(255, 216, 177)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFD8B1`
pub const XKCD_LIGHT_PEACH: Color = Color(255, 216, 177);

/// Colour `(r = 127, g = 78, b = 30)`
///
/// Colour `XKCD_MILK_CHOCOLATE` from the set `CSS4_COLORS`. (Colour number `571`)
/// ## Representations:
/// - int tuple `(127, 78, 30)`
/// - float tuple `(0.5, 0, 0)`
/// - hex: `#7F4E1E`
pub const XKCD_MILK_CHOCOLATE: Color = Color(127, 78, 30);

/// Colour `(r = 191, g = 155, b = 12)`
///
/// Colour `XKCD_OCHER` from the set `CSS4_COLORS`. (Colour number `572`)
/// ## Representations:
/// - int tuple `(191, 155, 12)`
/// - float tuple `(0.75, 1, 0)`
/// - hex: `#BF9B0C`
pub const XKCD_OCHER: Color = Color(191, 155, 12);

/// Colour `(r = 107, g = 163, b = 83)`
///
/// Colour `XKCD_OFF_GREEN` from the set `CSS4_COLORS`. (Colour number `573`)
/// ## Representations:
/// - int tuple `(107, 163, 83)`
/// - float tuple `(0.42, 1, 0)`
/// - hex: `#6BA353`
pub const XKCD_OFF_GREEN: Color = Color(107, 163, 83);

/// Colour `(r = 240, g = 117, b = 230)`
///
/// Colour `XKCD_PURPLY_PINK` from the set `CSS4_COLORS`. (Colour number `574`)
/// ## Representations:
/// - int tuple `(240, 117, 230)`
/// - float tuple `(0.94, 0, 1)`
/// - hex: `#F075E6`
pub const XKCD_PURPLY_PINK: Color = Color(240, 117, 230);

/// Colour `(r = 123, g = 200, b = 246)`
///
/// Colour `XKCD_LIGHTBLUE` from the set `CSS4_COLORS`. (Colour number `575`)
/// ## Representations:
/// - int tuple `(123, 200, 246)`
/// - float tuple `(0.48, 1, 1)`
/// - hex: `#7BC8F6`
pub const XKCD_LIGHTBLUE: Color = Color(123, 200, 246);

/// Colour `(r = 71, g = 95, b = 148)`
///
/// Colour `XKCD_DUSKY_BLUE` from the set `CSS4_COLORS`. (Colour number `576`)
/// ## Representations:
/// - int tuple `(71, 95, 148)`
/// - float tuple `(0.28, 0, 1)`
/// - hex: `#475F94`
pub const XKCD_DUSKY_BLUE: Color = Color(71, 95, 148);

/// Colour `(r = 245, g = 191, b = 3)`
///
/// Colour `XKCD_GOLDEN` from the set `CSS4_COLORS`. (Colour number `577`)
/// ## Representations:
/// - int tuple `(245, 191, 3)`
/// - float tuple `(0.96, 1, 0)`
/// - hex: `#F5BF03`
pub const XKCD_GOLDEN: Color = Color(245, 191, 3);

/// Colour `(r = 255, g = 254, b = 182)`
///
/// Colour `XKCD_LIGHT_BEIGE` from the set `CSS4_COLORS`. (Colour number `578`)
/// ## Representations:
/// - int tuple `(255, 254, 182)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFFEB6`
pub const XKCD_LIGHT_BEIGE: Color = Color(255, 254, 182);

/// Colour `(r = 255, g = 253, b = 116)`
///
/// Colour `XKCD_BUTTER_YELLOW` from the set `CSS4_COLORS`. (Colour number `579`)
/// ## Representations:
/// - int tuple `(255, 253, 116)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFFD74`
pub const XKCD_BUTTER_YELLOW: Color = Color(255, 253, 116);

/// Colour `(r = 137, g = 91, b = 123)`
///
/// Colour `XKCD_DUSKY_PURPLE` from the set `CSS4_COLORS`. (Colour number `580`)
/// ## Representations:
/// - int tuple `(137, 91, 123)`
/// - float tuple `(0.54, 0, 0)`
/// - hex: `#895B7B`
pub const XKCD_DUSKY_PURPLE: Color = Color(137, 91, 123);

/// Colour `(r = 67, g = 107, b = 173)`
///
/// Colour `XKCD_FRENCH_BLUE` from the set `CSS4_COLORS`. (Colour number `581`)
/// ## Representations:
/// - int tuple `(67, 107, 173)`
/// - float tuple `(0.26, 0, 1)`
/// - hex: `#436BAD`
pub const XKCD_FRENCH_BLUE: Color = Color(67, 107, 173);

/// Colour `(r = 208, g = 193, b = 1)`
///
/// Colour `XKCD_UGLY_YELLOW` from the set `CSS4_COLORS`. (Colour number `582`)
/// ## Representations:
/// - int tuple `(208, 193, 1)`
/// - float tuple `(0.82, 1, 0)`
/// - hex: `#D0C101`
pub const XKCD_UGLY_YELLOW: Color = Color(208, 193, 1);

/// Colour `(r = 198, g = 248, b = 8)`
///
/// Colour `XKCD_GREENY_YELLOW` from the set `CSS4_COLORS`. (Colour number `583`)
/// ## Representations:
/// - int tuple `(198, 248, 8)`
/// - float tuple `(0.78, 1, 0)`
/// - hex: `#C6F808`
pub const XKCD_GREENY_YELLOW: Color = Color(198, 248, 8);

/// Colour `(r = 244, g = 54, b = 5)`
///
/// Colour `XKCD_ORANGISH_RED` from the set `CSS4_COLORS`. (Colour number `584`)
/// ## Representations:
/// - int tuple `(244, 54, 5)`
/// - float tuple `(0.96, 0, 0)`
/// - hex: `#F43605`
pub const XKCD_ORANGISH_RED: Color = Color(244, 54, 5);

/// Colour `(r = 2, g = 193, b = 77)`
///
/// Colour `XKCD_SHAMROCK_GREEN` from the set `CSS4_COLORS`. (Colour number `585`)
/// ## Representations:
/// - int tuple `(2, 193, 77)`
/// - float tuple `(0.01, 1, 0)`
/// - hex: `#02C14D`
pub const XKCD_SHAMROCK_GREEN: Color = Color(2, 193, 77);

/// Colour `(r = 178, g = 95, b = 3)`
///
/// Colour `XKCD_ORANGISH_BROWN` from the set `CSS4_COLORS`. (Colour number `586`)
/// ## Representations:
/// - int tuple `(178, 95, 3)`
/// - float tuple `(0.7, 0, 0)`
/// - hex: `#B25F03`
pub const XKCD_ORANGISH_BROWN: Color = Color(178, 95, 3);

/// Colour `(r = 42, g = 126, b = 25)`
///
/// Colour `XKCD_TREE_GREEN` from the set `CSS4_COLORS`. (Colour number `587`)
/// ## Representations:
/// - int tuple `(42, 126, 25)`
/// - float tuple `(0.16, 0, 0)`
/// - hex: `#2A7E19`
pub const XKCD_TREE_GREEN: Color = Color(42, 126, 25);

/// Colour `(r = 73, g = 6, b = 72)`
///
/// Colour `XKCD_DEEP_VIOLET` from the set `CSS4_COLORS`. (Colour number `588`)
/// ## Representations:
/// - int tuple `(73, 6, 72)`
/// - float tuple `(0.29, 0, 0)`
/// - hex: `#490648`
pub const XKCD_DEEP_VIOLET: Color = Color(73, 6, 72);

/// Colour `(r = 83, g = 98, b = 103)`
///
/// Colour `XKCD_GUNMETAL` from the set `CSS4_COLORS`. (Colour number `589`)
/// ## Representations:
/// - int tuple `(83, 98, 103)`
/// - float tuple `(0.33, 0, 0)`
/// - hex: `#536267`
pub const XKCD_GUNMETAL: Color = Color(83, 98, 103);

/// Colour `(r = 90, g = 6, b = 239)`
///
/// Colour `XKCD_BLUE_SLASH_PURPLE` from the set `CSS4_COLORS`. (Colour number `590`)
/// ## Representations:
/// - int tuple `(90, 6, 239)`
/// - float tuple `(0.35, 0, 1)`
/// - hex: `#5A06EF`
pub const XKCD_BLUE_SLASH_PURPLE: Color = Color(90, 6, 239);

/// Colour `(r = 207, g = 2, b = 52)`
///
/// Colour `XKCD_CHERRY` from the set `CSS4_COLORS`. (Colour number `591`)
/// ## Representations:
/// - int tuple `(207, 2, 52)`
/// - float tuple `(0.81, 0, 0)`
/// - hex: `#CF0234`
pub const XKCD_CHERRY: Color = Color(207, 2, 52);

/// Colour `(r = 196, g = 166, b = 97)`
///
/// Colour `XKCD_SANDY_BROWN` from the set `CSS4_COLORS`. (Colour number `592`)
/// ## Representations:
/// - int tuple `(196, 166, 97)`
/// - float tuple `(0.77, 1, 0)`
/// - hex: `#C4A661`
pub const XKCD_SANDY_BROWN: Color = Color(196, 166, 97);

/// Colour `(r = 151, g = 138, b = 132)`
///
/// Colour `XKCD_WARM_GREY` from the set `CSS4_COLORS`. (Colour number `593`)
/// ## Representations:
/// - int tuple `(151, 138, 132)`
/// - float tuple `(0.59, 1, 1)`
/// - hex: `#978A84`
pub const XKCD_WARM_GREY: Color = Color(151, 138, 132);

/// Colour `(r = 31, g = 9, b = 84)`
///
/// Colour `XKCD_DARK_INDIGO` from the set `CSS4_COLORS`. (Colour number `594`)
/// ## Representations:
/// - int tuple `(31, 9, 84)`
/// - float tuple `(0.12, 0, 0)`
/// - hex: `#1F0954`
pub const XKCD_DARK_INDIGO: Color = Color(31, 9, 84);

/// Colour `(r = 3, g = 1, b = 45)`
///
/// Colour `XKCD_MIDNIGHT` from the set `CSS4_COLORS`. (Colour number `595`)
/// ## Representations:
/// - int tuple `(3, 1, 45)`
/// - float tuple `(0.01, 0, 0)`
/// - hex: `#03012D`
pub const XKCD_MIDNIGHT: Color = Color(3, 1, 45);

/// Colour `(r = 43, g = 177, b = 121)`
///
/// Colour `XKCD_BLUEY_GREEN` from the set `CSS4_COLORS`. (Colour number `596`)
/// ## Representations:
/// - int tuple `(43, 177, 121)`
/// - float tuple `(0.17, 1, 0)`
/// - hex: `#2BB179`
pub const XKCD_BLUEY_GREEN: Color = Color(43, 177, 121);

/// Colour `(r = 195, g = 144, b = 155)`
///
/// Colour `XKCD_GREY_PINK` from the set `CSS4_COLORS`. (Colour number `597`)
/// ## Representations:
/// - int tuple `(195, 144, 155)`
/// - float tuple `(0.76, 1, 1)`
/// - hex: `#C3909B`
pub const XKCD_GREY_PINK: Color = Color(195, 144, 155);

/// Colour `(r = 166, g = 111, b = 181)`
///
/// Colour `XKCD_SOFT_PURPLE` from the set `CSS4_COLORS`. (Colour number `598`)
/// ## Representations:
/// - int tuple `(166, 111, 181)`
/// - float tuple `(0.65, 0, 1)`
/// - hex: `#A66FB5`
pub const XKCD_SOFT_PURPLE: Color = Color(166, 111, 181);

/// Colour `(r = 119, g = 0, b = 1)`
///
/// Colour `XKCD_BLOOD` from the set `CSS4_COLORS`. (Colour number `599`)
/// ## Representations:
/// - int tuple `(119, 0, 1)`
/// - float tuple `(0.47, 0, 0)`
/// - hex: `#770001`
pub const XKCD_BLOOD: Color = Color(119, 0, 1);

/// Colour `(r = 146, g = 43, b = 5)`
///
/// Colour `XKCD_BROWN_RED` from the set `CSS4_COLORS`. (Colour number `600`)
/// ## Representations:
/// - int tuple `(146, 43, 5)`
/// - float tuple `(0.57, 0, 0)`
/// - hex: `#922B05`
pub const XKCD_BROWN_RED: Color = Color(146, 43, 5);

/// Colour `(r = 125, g = 127, b = 124)`
///
/// Colour `XKCD_MEDIUM_GREY` from the set `CSS4_COLORS`. (Colour number `601`)
/// ## Representations:
/// - int tuple `(125, 127, 124)`
/// - float tuple `(0.49, 0, 0)`
/// - hex: `#7D7F7C`
pub const XKCD_MEDIUM_GREY: Color = Color(125, 127, 124);

/// Colour `(r = 153, g = 15, b = 75)`
///
/// Colour `XKCD_BERRY` from the set `CSS4_COLORS`. (Colour number `602`)
/// ## Representations:
/// - int tuple `(153, 15, 75)`
/// - float tuple `(0.6, 0, 0)`
/// - hex: `#990F4B`
pub const XKCD_BERRY: Color = Color(153, 15, 75);

/// Colour `(r = 143, g = 115, b = 3)`
///
/// Colour `XKCD_POO` from the set `CSS4_COLORS`. (Colour number `603`)
/// ## Representations:
/// - int tuple `(143, 115, 3)`
/// - float tuple `(0.56, 0, 0)`
/// - hex: `#8F7303`
pub const XKCD_POO: Color = Color(143, 115, 3);

/// Colour `(r = 200, g = 60, b = 185)`
///
/// Colour `XKCD_PURPLEY_PINK` from the set `CSS4_COLORS`. (Colour number `604`)
/// ## Representations:
/// - int tuple `(200, 60, 185)`
/// - float tuple `(0.78, 0, 1)`
/// - hex: `#C83CB9`
pub const XKCD_PURPLEY_PINK: Color = Color(200, 60, 185);

/// Colour `(r = 254, g = 169, b = 147)`
///
/// Colour `XKCD_LIGHT_SALMON` from the set `CSS4_COLORS`. (Colour number `605`)
/// ## Representations:
/// - int tuple `(254, 169, 147)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FEA993`
pub const XKCD_LIGHT_SALMON: Color = Color(254, 169, 147);

/// Colour `(r = 172, g = 187, b = 13)`
///
/// Colour `XKCD_SNOT` from the set `CSS4_COLORS`. (Colour number `606`)
/// ## Representations:
/// - int tuple `(172, 187, 13)`
/// - float tuple `(0.67, 1, 0)`
/// - hex: `#ACBB0D`
pub const XKCD_SNOT: Color = Color(172, 187, 13);

/// Colour `(r = 192, g = 113, b = 254)`
///
/// Colour `XKCD_EASTER_PURPLE` from the set `CSS4_COLORS`. (Colour number `607`)
/// ## Representations:
/// - int tuple `(192, 113, 254)`
/// - float tuple `(0.75, 0, 1)`
/// - hex: `#C071FE`
pub const XKCD_EASTER_PURPLE: Color = Color(192, 113, 254);

/// Colour `(r = 204, g = 253, b = 127)`
///
/// Colour `XKCD_LIGHT_YELLOW_GREEN` from the set `CSS4_COLORS`. (Colour number `608`)
/// ## Representations:
/// - int tuple `(204, 253, 127)`
/// - float tuple `(0.8, 1, 0)`
/// - hex: `#CCFD7F`
pub const XKCD_LIGHT_YELLOW_GREEN: Color = Color(204, 253, 127);

/// Colour `(r = 0, g = 2, b = 46)`
///
/// Colour `XKCD_DARK_NAVY_BLUE` from the set `CSS4_COLORS`. (Colour number `609`)
/// ## Representations:
/// - int tuple `(0, 2, 46)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#00022E`
pub const XKCD_DARK_NAVY_BLUE: Color = Color(0, 2, 46);

/// Colour `(r = 130, g = 131, b = 68)`
///
/// Colour `XKCD_DRAB` from the set `CSS4_COLORS`. (Colour number `610`)
/// ## Representations:
/// - int tuple `(130, 131, 68)`
/// - float tuple `(0.51, 1, 0)`
/// - hex: `#828344`
pub const XKCD_DRAB: Color = Color(130, 131, 68);

/// Colour `(r = 255, g = 197, b = 203)`
///
/// Colour `XKCD_LIGHT_ROSE` from the set `CSS4_COLORS`. (Colour number `611`)
/// ## Representations:
/// - int tuple `(255, 197, 203)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFC5CB`
pub const XKCD_LIGHT_ROSE: Color = Color(255, 197, 203);

/// Colour `(r = 171, g = 18, b = 57)`
///
/// Colour `XKCD_ROUGE` from the set `CSS4_COLORS`. (Colour number `612`)
/// ## Representations:
/// - int tuple `(171, 18, 57)`
/// - float tuple `(0.67, 0, 0)`
/// - hex: `#AB1239`
pub const XKCD_ROUGE: Color = Color(171, 18, 57);

/// Colour `(r = 176, g = 5, b = 75)`
///
/// Colour `XKCD_PURPLISH_RED` from the set `CSS4_COLORS`. (Colour number `613`)
/// ## Representations:
/// - int tuple `(176, 5, 75)`
/// - float tuple `(0.69, 0, 0)`
/// - hex: `#B0054B`
pub const XKCD_PURPLISH_RED: Color = Color(176, 5, 75);

/// Colour `(r = 153, g = 204, b = 4)`
///
/// Colour `XKCD_SLIME_GREEN` from the set `CSS4_COLORS`. (Colour number `614`)
/// ## Representations:
/// - int tuple `(153, 204, 4)`
/// - float tuple `(0.6, 1, 0)`
/// - hex: `#99CC04`
pub const XKCD_SLIME_GREEN: Color = Color(153, 204, 4);

/// Colour `(r = 147, g = 124, b = 0)`
///
/// Colour `XKCD_BABY_POOP` from the set `CSS4_COLORS`. (Colour number `615`)
/// ## Representations:
/// - int tuple `(147, 124, 0)`
/// - float tuple `(0.58, 0, 0)`
/// - hex: `#937C00`
pub const XKCD_BABY_POOP: Color = Color(147, 124, 0);

/// Colour `(r = 1, g = 149, b = 41)`
///
/// Colour `XKCD_IRISH_GREEN` from the set `CSS4_COLORS`. (Colour number `616`)
/// ## Representations:
/// - int tuple `(1, 149, 41)`
/// - float tuple `(0.0, 1, 0)`
/// - hex: `#019529`
pub const XKCD_IRISH_GREEN: Color = Color(1, 149, 41);

/// Colour `(r = 239, g = 29, b = 231)`
///
/// Colour `XKCD_PINK_SLASH_PURPLE` from the set `CSS4_COLORS`. (Colour number `617`)
/// ## Representations:
/// - int tuple `(239, 29, 231)`
/// - float tuple `(0.94, 0, 1)`
/// - hex: `#EF1DE7`
pub const XKCD_PINK_SLASH_PURPLE: Color = Color(239, 29, 231);

/// Colour `(r = 0, g = 4, b = 53)`
///
/// Colour `XKCD_DARK_NAVY` from the set `CSS4_COLORS`. (Colour number `618`)
/// ## Representations:
/// - int tuple `(0, 4, 53)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#000435`
pub const XKCD_DARK_NAVY: Color = Color(0, 4, 53);

/// Colour `(r = 66, g = 179, b = 149)`
///
/// Colour `XKCD_GREENY_BLUE` from the set `CSS4_COLORS`. (Colour number `619`)
/// ## Representations:
/// - int tuple `(66, 179, 149)`
/// - float tuple `(0.26, 1, 1)`
/// - hex: `#42B395`
pub const XKCD_GREENY_BLUE: Color = Color(66, 179, 149);

/// Colour `(r = 157, g = 87, b = 131)`
///
/// Colour `XKCD_LIGHT_PLUM` from the set `CSS4_COLORS`. (Colour number `620`)
/// ## Representations:
/// - int tuple `(157, 87, 131)`
/// - float tuple `(0.62, 0, 1)`
/// - hex: `#9D5783`
pub const XKCD_LIGHT_PLUM: Color = Color(157, 87, 131);

/// Colour `(r = 200, g = 172, b = 169)`
///
/// Colour `XKCD_PINKISH_GREY` from the set `CSS4_COLORS`. (Colour number `621`)
/// ## Representations:
/// - int tuple `(200, 172, 169)`
/// - float tuple `(0.78, 1, 1)`
/// - hex: `#C8ACA9`
pub const XKCD_PINKISH_GREY: Color = Color(200, 172, 169);

/// Colour `(r = 200, g = 118, b = 6)`
///
/// Colour `XKCD_DIRTY_ORANGE` from the set `CSS4_COLORS`. (Colour number `622`)
/// ## Representations:
/// - int tuple `(200, 118, 6)`
/// - float tuple `(0.78, 0, 0)`
/// - hex: `#C87606`
pub const XKCD_DIRTY_ORANGE: Color = Color(200, 118, 6);

/// Colour `(r = 170, g = 39, b = 4)`
///
/// Colour `XKCD_RUST_RED` from the set `CSS4_COLORS`. (Colour number `623`)
/// ## Representations:
/// - int tuple `(170, 39, 4)`
/// - float tuple `(0.67, 0, 0)`
/// - hex: `#AA2704`
pub const XKCD_RUST_RED: Color = Color(170, 39, 4);

/// Colour `(r = 228, g = 203, b = 255)`
///
/// Colour `XKCD_PALE_LILAC` from the set `CSS4_COLORS`. (Colour number `624`)
/// ## Representations:
/// - int tuple `(228, 203, 255)`
/// - float tuple `(0.89, 1, 1)`
/// - hex: `#E4CBFF`
pub const XKCD_PALE_LILAC: Color = Color(228, 203, 255);

/// Colour `(r = 250, g = 66, b = 36)`
///
/// Colour `XKCD_ORANGEY_RED` from the set `CSS4_COLORS`. (Colour number `625`)
/// ## Representations:
/// - int tuple `(250, 66, 36)`
/// - float tuple `(0.98, 0, 0)`
/// - hex: `#FA4224`
pub const XKCD_ORANGEY_RED: Color = Color(250, 66, 36);

/// Colour `(r = 8, g = 4, b = 249)`
///
/// Colour `XKCD_PRIMARY_BLUE` from the set `CSS4_COLORS`. (Colour number `626`)
/// ## Representations:
/// - int tuple `(8, 4, 249)`
/// - float tuple `(0.03, 0, 1)`
/// - hex: `#0804F9`
pub const XKCD_PRIMARY_BLUE: Color = Color(8, 4, 249);

/// Colour `(r = 92, g = 178, b = 0)`
///
/// Colour `XKCD_KERMIT_GREEN` from the set `CSS4_COLORS`. (Colour number `627`)
/// ## Representations:
/// - int tuple `(92, 178, 0)`
/// - float tuple `(0.36, 1, 0)`
/// - hex: `#5CB200`
pub const XKCD_KERMIT_GREEN: Color = Color(92, 178, 0);

/// Colour `(r = 118, g = 66, b = 78)`
///
/// Colour `XKCD_BROWNISH_PURPLE` from the set `CSS4_COLORS`. (Colour number `628`)
/// ## Representations:
/// - int tuple `(118, 66, 78)`
/// - float tuple `(0.46, 0, 0)`
/// - hex: `#76424E`
pub const XKCD_BROWNISH_PURPLE: Color = Color(118, 66, 78);

/// Colour `(r = 108, g = 122, b = 14)`
///
/// Colour `XKCD_MURKY_GREEN` from the set `CSS4_COLORS`. (Colour number `629`)
/// ## Representations:
/// - int tuple `(108, 122, 14)`
/// - float tuple `(0.42, 0, 0)`
/// - hex: `#6C7A0E`
pub const XKCD_MURKY_GREEN: Color = Color(108, 122, 14);

/// Colour `(r = 251, g = 221, b = 126)`
///
/// Colour `XKCD_WHEAT` from the set `CSS4_COLORS`. (Colour number `630`)
/// ## Representations:
/// - int tuple `(251, 221, 126)`
/// - float tuple `(0.98, 1, 0)`
/// - hex: `#FBDD7E`
pub const XKCD_WHEAT: Color = Color(251, 221, 126);

/// Colour `(r = 42, g = 1, b = 52)`
///
/// Colour `XKCD_VERY_DARK_PURPLE` from the set `CSS4_COLORS`. (Colour number `631`)
/// ## Representations:
/// - int tuple `(42, 1, 52)`
/// - float tuple `(0.16, 0, 0)`
/// - hex: `#2A0134`
pub const XKCD_VERY_DARK_PURPLE: Color = Color(42, 1, 52);

/// Colour `(r = 4, g = 74, b = 5)`
///
/// Colour `XKCD_BOTTLE_GREEN` from the set `CSS4_COLORS`. (Colour number `632`)
/// ## Representations:
/// - int tuple `(4, 74, 5)`
/// - float tuple `(0.02, 0, 0)`
/// - hex: `#044A05`
pub const XKCD_BOTTLE_GREEN: Color = Color(4, 74, 5);

/// Colour `(r = 253, g = 70, b = 89)`
///
/// Colour `XKCD_WATERMELON` from the set `CSS4_COLORS`. (Colour number `633`)
/// ## Representations:
/// - int tuple `(253, 70, 89)`
/// - float tuple `(0.99, 0, 0)`
/// - hex: `#FD4659`
pub const XKCD_WATERMELON: Color = Color(253, 70, 89);

/// Colour `(r = 13, g = 117, b = 248)`
///
/// Colour `XKCD_DEEP_SKY_BLUE` from the set `CSS4_COLORS`. (Colour number `634`)
/// ## Representations:
/// - int tuple `(13, 117, 248)`
/// - float tuple `(0.05, 0, 1)`
/// - hex: `#0D75F8`
pub const XKCD_DEEP_SKY_BLUE: Color = Color(13, 117, 248);

/// Colour `(r = 254, g = 0, b = 2)`
///
/// Colour `XKCD_FIRE_ENGINE_RED` from the set `CSS4_COLORS`. (Colour number `635`)
/// ## Representations:
/// - int tuple `(254, 0, 2)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FE0002`
pub const XKCD_FIRE_ENGINE_RED: Color = Color(254, 0, 2);

/// Colour `(r = 203, g = 157, b = 6)`
///
/// Colour `XKCD_YELLOW_OCHRE` from the set `CSS4_COLORS`. (Colour number `636`)
/// ## Representations:
/// - int tuple `(203, 157, 6)`
/// - float tuple `(0.8, 1, 0)`
/// - hex: `#CB9D06`
pub const XKCD_YELLOW_OCHRE: Color = Color(203, 157, 6);

/// Colour `(r = 251, g = 125, b = 7)`
///
/// Colour `XKCD_PUMPKIN_ORANGE` from the set `CSS4_COLORS`. (Colour number `637`)
/// ## Representations:
/// - int tuple `(251, 125, 7)`
/// - float tuple `(0.98, 0, 0)`
/// - hex: `#FB7D07`
pub const XKCD_PUMPKIN_ORANGE: Color = Color(251, 125, 7);

/// Colour `(r = 185, g = 204, b = 129)`
///
/// Colour `XKCD_PALE_OLIVE` from the set `CSS4_COLORS`. (Colour number `638`)
/// ## Representations:
/// - int tuple `(185, 204, 129)`
/// - float tuple `(0.73, 1, 1)`
/// - hex: `#B9CC81`
pub const XKCD_PALE_OLIVE: Color = Color(185, 204, 129);

/// Colour `(r = 237, g = 200, b = 255)`
///
/// Colour `XKCD_LIGHT_LILAC` from the set `CSS4_COLORS`. (Colour number `639`)
/// ## Representations:
/// - int tuple `(237, 200, 255)`
/// - float tuple `(0.93, 1, 1)`
/// - hex: `#EDC8FF`
pub const XKCD_LIGHT_LILAC: Color = Color(237, 200, 255);

/// Colour `(r = 97, g = 225, b = 96)`
///
/// Colour `XKCD_LIGHTISH_GREEN` from the set `CSS4_COLORS`. (Colour number `640`)
/// ## Representations:
/// - int tuple `(97, 225, 96)`
/// - float tuple `(0.38, 1, 0)`
/// - hex: `#61E160`
pub const XKCD_LIGHTISH_GREEN: Color = Color(97, 225, 96);

/// Colour `(r = 138, g = 184, b = 254)`
///
/// Colour `XKCD_CAROLINA_BLUE` from the set `CSS4_COLORS`. (Colour number `641`)
/// ## Representations:
/// - int tuple `(138, 184, 254)`
/// - float tuple `(0.54, 1, 1)`
/// - hex: `#8AB8FE`
pub const XKCD_CAROLINA_BLUE: Color = Color(138, 184, 254);

/// Colour `(r = 146, g = 10, b = 78)`
///
/// Colour `XKCD_MULBERRY` from the set `CSS4_COLORS`. (Colour number `642`)
/// ## Representations:
/// - int tuple `(146, 10, 78)`
/// - float tuple `(0.57, 0, 0)`
/// - hex: `#920A4E`
pub const XKCD_MULBERRY: Color = Color(146, 10, 78);

/// Colour `(r = 254, g = 2, b = 162)`
///
/// Colour `XKCD_SHOCKING_PINK` from the set `CSS4_COLORS`. (Colour number `643`)
/// ## Representations:
/// - int tuple `(254, 2, 162)`
/// - float tuple `(1.0, 0, 1)`
/// - hex: `#FE02A2`
pub const XKCD_SHOCKING_PINK: Color = Color(254, 2, 162);

/// Colour `(r = 154, g = 48, b = 1)`
///
/// Colour `XKCD_AUBURN` from the set `CSS4_COLORS`. (Colour number `644`)
/// ## Representations:
/// - int tuple `(154, 48, 1)`
/// - float tuple `(0.6, 0, 0)`
/// - hex: `#9A3001`
pub const XKCD_AUBURN: Color = Color(154, 48, 1);

/// Colour `(r = 101, g = 254, b = 8)`
///
/// Colour `XKCD_BRIGHT_LIME_GREEN` from the set `CSS4_COLORS`. (Colour number `645`)
/// ## Representations:
/// - int tuple `(101, 254, 8)`
/// - float tuple `(0.4, 1, 0)`
/// - hex: `#65FE08`
pub const XKCD_BRIGHT_LIME_GREEN: Color = Color(101, 254, 8);

/// Colour `(r = 190, g = 253, b = 183)`
///
/// Colour `XKCD_CELADON` from the set `CSS4_COLORS`. (Colour number `646`)
/// ## Representations:
/// - int tuple `(190, 253, 183)`
/// - float tuple `(0.75, 1, 1)`
/// - hex: `#BEFDB7`
pub const XKCD_CELADON: Color = Color(190, 253, 183);

/// Colour `(r = 177, g = 114, b = 97)`
///
/// Colour `XKCD_PINKISH_BROWN` from the set `CSS4_COLORS`. (Colour number `647`)
/// ## Representations:
/// - int tuple `(177, 114, 97)`
/// - float tuple `(0.69, 0, 0)`
/// - hex: `#B17261`
pub const XKCD_PINKISH_BROWN: Color = Color(177, 114, 97);

/// Colour `(r = 136, g = 95, b = 1)`
///
/// Colour `XKCD_POO_BROWN` from the set `CSS4_COLORS`. (Colour number `648`)
/// ## Representations:
/// - int tuple `(136, 95, 1)`
/// - float tuple `(0.53, 0, 0)`
/// - hex: `#885F01`
pub const XKCD_POO_BROWN: Color = Color(136, 95, 1);

/// Colour `(r = 2, g = 204, b = 254)`
///
/// Colour `XKCD_BRIGHT_SKY_BLUE` from the set `CSS4_COLORS`. (Colour number `649`)
/// ## Representations:
/// - int tuple `(2, 204, 254)`
/// - float tuple `(0.01, 1, 1)`
/// - hex: `#02CCFE`
pub const XKCD_BRIGHT_SKY_BLUE: Color = Color(2, 204, 254);

/// Colour `(r = 193, g = 253, b = 149)`
///
/// Colour `XKCD_CELERY` from the set `CSS4_COLORS`. (Colour number `650`)
/// ## Representations:
/// - int tuple `(193, 253, 149)`
/// - float tuple `(0.76, 1, 1)`
/// - hex: `#C1FD95`
pub const XKCD_CELERY: Color = Color(193, 253, 149);

/// Colour `(r = 131, g = 101, b = 57)`
///
/// Colour `XKCD_DIRT_BROWN` from the set `CSS4_COLORS`. (Colour number `651`)
/// ## Representations:
/// - int tuple `(131, 101, 57)`
/// - float tuple `(0.51, 0, 0)`
/// - hex: `#836539`
pub const XKCD_DIRT_BROWN: Color = Color(131, 101, 57);

/// Colour `(r = 251, g = 41, b = 67)`
///
/// Colour `XKCD_STRAWBERRY` from the set `CSS4_COLORS`. (Colour number `652`)
/// ## Representations:
/// - int tuple `(251, 41, 67)`
/// - float tuple `(0.98, 0, 0)`
/// - hex: `#FB2943`
pub const XKCD_STRAWBERRY: Color = Color(251, 41, 67);

/// Colour `(r = 132, g = 183, b = 1)`
///
/// Colour `XKCD_DARK_LIME` from the set `CSS4_COLORS`. (Colour number `653`)
/// ## Representations:
/// - int tuple `(132, 183, 1)`
/// - float tuple `(0.52, 1, 0)`
/// - hex: `#84B701`
pub const XKCD_DARK_LIME: Color = Color(132, 183, 1);

/// Colour `(r = 182, g = 99, b = 37)`
///
/// Colour `XKCD_COPPER` from the set `CSS4_COLORS`. (Colour number `654`)
/// ## Representations:
/// - int tuple `(182, 99, 37)`
/// - float tuple `(0.71, 0, 0)`
/// - hex: `#B66325`
pub const XKCD_COPPER: Color = Color(182, 99, 37);

/// Colour `(r = 127, g = 81, b = 18)`
///
/// Colour `XKCD_MEDIUM_BROWN` from the set `CSS4_COLORS`. (Colour number `655`)
/// ## Representations:
/// - int tuple `(127, 81, 18)`
/// - float tuple `(0.5, 0, 0)`
/// - hex: `#7F5112`
pub const XKCD_MEDIUM_BROWN: Color = Color(127, 81, 18);

/// Colour `(r = 95, g = 160, b = 82)`
///
/// Colour `XKCD_MUTED_GREEN` from the set `CSS4_COLORS`. (Colour number `656`)
/// ## Representations:
/// - int tuple `(95, 160, 82)`
/// - float tuple `(0.37, 1, 0)`
/// - hex: `#5FA052`
pub const XKCD_MUTED_GREEN: Color = Color(95, 160, 82);

/// Colour `(r = 109, g = 237, b = 253)`
///
/// Colour `XKCD_ROBIN_S_EGG` from the set `CSS4_COLORS`. (Colour number `657`)
/// ## Representations:
/// - int tuple `(109, 237, 253)`
/// - float tuple `(0.43, 1, 1)`
/// - hex: `#6DEDFD`
pub const XKCD_ROBIN_S_EGG: Color = Color(109, 237, 253);

/// Colour `(r = 11, g = 249, b = 234)`
///
/// Colour `XKCD_BRIGHT_AQUA` from the set `CSS4_COLORS`. (Colour number `658`)
/// ## Representations:
/// - int tuple `(11, 249, 234)`
/// - float tuple `(0.04, 1, 1)`
/// - hex: `#0BF9EA`
pub const XKCD_BRIGHT_AQUA: Color = Color(11, 249, 234);

/// Colour `(r = 199, g = 96, b = 255)`
///
/// Colour `XKCD_BRIGHT_LAVENDER` from the set `CSS4_COLORS`. (Colour number `659`)
/// ## Representations:
/// - int tuple `(199, 96, 255)`
/// - float tuple `(0.78, 0, 1)`
/// - hex: `#C760FF`
pub const XKCD_BRIGHT_LAVENDER: Color = Color(199, 96, 255);

/// Colour `(r = 255, g = 255, b = 203)`
///
/// Colour `XKCD_IVORY` from the set `CSS4_COLORS`. (Colour number `660`)
/// ## Representations:
/// - int tuple `(255, 255, 203)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFFFCB`
pub const XKCD_IVORY: Color = Color(255, 255, 203);

/// Colour `(r = 246, g = 206, b = 252)`
///
/// Colour `XKCD_VERY_LIGHT_PURPLE` from the set `CSS4_COLORS`. (Colour number `661`)
/// ## Representations:
/// - int tuple `(246, 206, 252)`
/// - float tuple `(0.96, 1, 1)`
/// - hex: `#F6CEFC`
pub const XKCD_VERY_LIGHT_PURPLE: Color = Color(246, 206, 252);

/// Colour `(r = 21, g = 80, b = 132)`
///
/// Colour `XKCD_LIGHT_NAVY` from the set `CSS4_COLORS`. (Colour number `662`)
/// ## Representations:
/// - int tuple `(21, 80, 132)`
/// - float tuple `(0.08, 0, 1)`
/// - hex: `#155084`
pub const XKCD_LIGHT_NAVY: Color = Color(21, 80, 132);

/// Colour `(r = 245, g = 5, b = 79)`
///
/// Colour `XKCD_PINK_RED` from the set `CSS4_COLORS`. (Colour number `663`)
/// ## Representations:
/// - int tuple `(245, 5, 79)`
/// - float tuple `(0.96, 0, 0)`
/// - hex: `#F5054F`
pub const XKCD_PINK_RED: Color = Color(245, 5, 79);

/// Colour `(r = 100, g = 84, b = 3)`
///
/// Colour `XKCD_OLIVE_BROWN` from the set `CSS4_COLORS`. (Colour number `664`)
/// ## Representations:
/// - int tuple `(100, 84, 3)`
/// - float tuple `(0.39, 0, 0)`
/// - hex: `#645403`
pub const XKCD_OLIVE_BROWN: Color = Color(100, 84, 3);

/// Colour `(r = 122, g = 89, b = 1)`
///
/// Colour `XKCD_POOP_BROWN` from the set `CSS4_COLORS`. (Colour number `665`)
/// ## Representations:
/// - int tuple `(122, 89, 1)`
/// - float tuple `(0.48, 0, 0)`
/// - hex: `#7A5901`
pub const XKCD_POOP_BROWN: Color = Color(122, 89, 1);

/// Colour `(r = 168, g = 181, b = 4)`
///
/// Colour `XKCD_MUSTARD_GREEN` from the set `CSS4_COLORS`. (Colour number `666`)
/// ## Representations:
/// - int tuple `(168, 181, 4)`
/// - float tuple `(0.66, 1, 0)`
/// - hex: `#A8B504`
pub const XKCD_MUSTARD_GREEN: Color = Color(168, 181, 4);

/// Colour `(r = 61, g = 153, b = 115)`
///
/// Colour `XKCD_OCEAN_GREEN` from the set `CSS4_COLORS`. (Colour number `667`)
/// ## Representations:
/// - int tuple `(61, 153, 115)`
/// - float tuple `(0.24, 1, 0)`
/// - hex: `#3D9973`
pub const XKCD_OCEAN_GREEN: Color = Color(61, 153, 115);

/// Colour `(r = 0, g = 1, b = 51)`
///
/// Colour `XKCD_VERY_DARK_BLUE` from the set `CSS4_COLORS`. (Colour number `668`)
/// ## Representations:
/// - int tuple `(0, 1, 51)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#000133`
pub const XKCD_VERY_DARK_BLUE: Color = Color(0, 1, 51);

/// Colour `(r = 118, g = 169, b = 115)`
///
/// Colour `XKCD_DUSTY_GREEN` from the set `CSS4_COLORS`. (Colour number `669`)
/// ## Representations:
/// - int tuple `(118, 169, 115)`
/// - float tuple `(0.46, 1, 0)`
/// - hex: `#76A973`
pub const XKCD_DUSTY_GREEN: Color = Color(118, 169, 115);

/// Colour `(r = 46, g = 90, b = 136)`
///
/// Colour `XKCD_LIGHT_NAVY_BLUE` from the set `CSS4_COLORS`. (Colour number `670`)
/// ## Representations:
/// - int tuple `(46, 90, 136)`
/// - float tuple `(0.18, 0, 1)`
/// - hex: `#2E5A88`
pub const XKCD_LIGHT_NAVY_BLUE: Color = Color(46, 90, 136);

/// Colour `(r = 11, g = 247, b = 125)`
///
/// Colour `XKCD_MINTY_GREEN` from the set `CSS4_COLORS`. (Colour number `671`)
/// ## Representations:
/// - int tuple `(11, 247, 125)`
/// - float tuple `(0.04, 1, 0)`
/// - hex: `#0BF77D`
pub const XKCD_MINTY_GREEN: Color = Color(11, 247, 125);

/// Colour `(r = 189, g = 108, b = 72)`
///
/// Colour `XKCD_ADOBE` from the set `CSS4_COLORS`. (Colour number `672`)
/// ## Representations:
/// - int tuple `(189, 108, 72)`
/// - float tuple `(0.74, 0, 0)`
/// - hex: `#BD6C48`
pub const XKCD_ADOBE: Color = Color(189, 108, 72);

/// Colour `(r = 172, g = 29, b = 184)`
///
/// Colour `XKCD_BARNEY` from the set `CSS4_COLORS`. (Colour number `673`)
/// ## Representations:
/// - int tuple `(172, 29, 184)`
/// - float tuple `(0.67, 0, 1)`
/// - hex: `#AC1DB8`
pub const XKCD_BARNEY: Color = Color(172, 29, 184);

/// Colour `(r = 43, g = 175, b = 106)`
///
/// Colour `XKCD_JADE_GREEN` from the set `CSS4_COLORS`. (Colour number `674`)
/// ## Representations:
/// - int tuple `(43, 175, 106)`
/// - float tuple `(0.17, 1, 0)`
/// - hex: `#2BAF6A`
pub const XKCD_JADE_GREEN: Color = Color(43, 175, 106);

/// Colour `(r = 38, g = 247, b = 253)`
///
/// Colour `XKCD_BRIGHT_LIGHT_BLUE` from the set `CSS4_COLORS`. (Colour number `675`)
/// ## Representations:
/// - int tuple `(38, 247, 253)`
/// - float tuple `(0.15, 1, 1)`
/// - hex: `#26F7FD`
pub const XKCD_BRIGHT_LIGHT_BLUE: Color = Color(38, 247, 253);

/// Colour `(r = 174, g = 253, b = 108)`
///
/// Colour `XKCD_LIGHT_LIME` from the set `CSS4_COLORS`. (Colour number `676`)
/// ## Representations:
/// - int tuple `(174, 253, 108)`
/// - float tuple `(0.68, 1, 0)`
/// - hex: `#AEFD6C`
pub const XKCD_LIGHT_LIME: Color = Color(174, 253, 108);

/// Colour `(r = 155, g = 143, b = 85)`
///
/// Colour `XKCD_DARK_KHAKI` from the set `CSS4_COLORS`. (Colour number `677`)
/// ## Representations:
/// - int tuple `(155, 143, 85)`
/// - float tuple `(0.61, 1, 0)`
/// - hex: `#9B8F55`
pub const XKCD_DARK_KHAKI: Color = Color(155, 143, 85);

/// Colour `(r = 255, g = 173, b = 1)`
///
/// Colour `XKCD_ORANGE_YELLOW` from the set `CSS4_COLORS`. (Colour number `678`)
/// ## Representations:
/// - int tuple `(255, 173, 1)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFAD01`
pub const XKCD_ORANGE_YELLOW: Color = Color(255, 173, 1);

/// Colour `(r = 198, g = 156, b = 4)`
///
/// Colour `XKCD_OCRE` from the set `CSS4_COLORS`. (Colour number `679`)
/// ## Representations:
/// - int tuple `(198, 156, 4)`
/// - float tuple `(0.78, 1, 0)`
/// - hex: `#C69C04`
pub const XKCD_OCRE: Color = Color(198, 156, 4);

/// Colour `(r = 244, g = 208, b = 84)`
///
/// Colour `XKCD_MAIZE` from the set `CSS4_COLORS`. (Colour number `680`)
/// ## Representations:
/// - int tuple `(244, 208, 84)`
/// - float tuple `(0.96, 1, 0)`
/// - hex: `#F4D054`
pub const XKCD_MAIZE: Color = Color(244, 208, 84);

/// Colour `(r = 222, g = 157, b = 172)`
///
/// Colour `XKCD_FADED_PINK` from the set `CSS4_COLORS`. (Colour number `681`)
/// ## Representations:
/// - int tuple `(222, 157, 172)`
/// - float tuple `(0.87, 1, 1)`
/// - hex: `#DE9DAC`
pub const XKCD_FADED_PINK: Color = Color(222, 157, 172);

/// Colour `(r = 5, g = 72, b = 13)`
///
/// Colour `XKCD_BRITISH_RACING_GREEN` from the set `CSS4_COLORS`. (Colour number `682`)
/// ## Representations:
/// - int tuple `(5, 72, 13)`
/// - float tuple `(0.02, 0, 0)`
/// - hex: `#05480D`
pub const XKCD_BRITISH_RACING_GREEN: Color = Color(5, 72, 13);

/// Colour `(r = 201, g = 174, b = 116)`
///
/// Colour `XKCD_SANDSTONE` from the set `CSS4_COLORS`. (Colour number `683`)
/// ## Representations:
/// - int tuple `(201, 174, 116)`
/// - float tuple `(0.79, 1, 0)`
/// - hex: `#C9AE74`
pub const XKCD_SANDSTONE: Color = Color(201, 174, 116);

/// Colour `(r = 96, g = 70, b = 15)`
///
/// Colour `XKCD_MUD_BROWN` from the set `CSS4_COLORS`. (Colour number `684`)
/// ## Representations:
/// - int tuple `(96, 70, 15)`
/// - float tuple `(0.38, 0, 0)`
/// - hex: `#60460F`
pub const XKCD_MUD_BROWN: Color = Color(96, 70, 15);

/// Colour `(r = 152, g = 246, b = 176)`
///
/// Colour `XKCD_LIGHT_SEA_GREEN` from the set `CSS4_COLORS`. (Colour number `685`)
/// ## Representations:
/// - int tuple `(152, 246, 176)`
/// - float tuple `(0.6, 1, 1)`
/// - hex: `#98F6B0`
pub const XKCD_LIGHT_SEA_GREEN: Color = Color(152, 246, 176);

/// Colour `(r = 138, g = 241, b = 254)`
///
/// Colour `XKCD_ROBIN_EGG_BLUE` from the set `CSS4_COLORS`. (Colour number `686`)
/// ## Representations:
/// - int tuple `(138, 241, 254)`
/// - float tuple `(0.54, 1, 1)`
/// - hex: `#8AF1FE`
pub const XKCD_ROBIN_EGG_BLUE: Color = Color(138, 241, 254);

/// Colour `(r = 46, g = 232, b = 187)`
///
/// Colour `XKCD_AQUA_MARINE` from the set `CSS4_COLORS`. (Colour number `687`)
/// ## Representations:
/// - int tuple `(46, 232, 187)`
/// - float tuple `(0.18, 1, 1)`
/// - hex: `#2EE8BB`
pub const XKCD_AQUA_MARINE: Color = Color(46, 232, 187);

/// Colour `(r = 17, g = 135, b = 93)`
///
/// Colour `XKCD_DARK_SEA_GREEN` from the set `CSS4_COLORS`. (Colour number `688`)
/// ## Representations:
/// - int tuple `(17, 135, 93)`
/// - float tuple `(0.07, 1, 0)`
/// - hex: `#11875D`
pub const XKCD_DARK_SEA_GREEN: Color = Color(17, 135, 93);

/// Colour `(r = 253, g = 176, b = 192)`
///
/// Colour `XKCD_SOFT_PINK` from the set `CSS4_COLORS`. (Colour number `689`)
/// ## Representations:
/// - int tuple `(253, 176, 192)`
/// - float tuple `(0.99, 1, 1)`
/// - hex: `#FDB0C0`
pub const XKCD_SOFT_PINK: Color = Color(253, 176, 192);

/// Colour `(r = 177, g = 96, b = 2)`
///
/// Colour `XKCD_ORANGEY_BROWN` from the set `CSS4_COLORS`. (Colour number `690`)
/// ## Representations:
/// - int tuple `(177, 96, 2)`
/// - float tuple `(0.69, 0, 0)`
/// - hex: `#B16002`
pub const XKCD_ORANGEY_BROWN: Color = Color(177, 96, 2);

/// Colour `(r = 247, g = 2, b = 42)`
///
/// Colour `XKCD_CHERRY_RED` from the set `CSS4_COLORS`. (Colour number `691`)
/// ## Representations:
/// - int tuple `(247, 2, 42)`
/// - float tuple `(0.97, 0, 0)`
/// - hex: `#F7022A`
pub const XKCD_CHERRY_RED: Color = Color(247, 2, 42);

/// Colour `(r = 213, g = 171, b = 9)`
///
/// Colour `XKCD_BURNT_YELLOW` from the set `CSS4_COLORS`. (Colour number `692`)
/// ## Representations:
/// - int tuple `(213, 171, 9)`
/// - float tuple `(0.84, 1, 0)`
/// - hex: `#D5AB09`
pub const XKCD_BURNT_YELLOW: Color = Color(213, 171, 9);

/// Colour `(r = 134, g = 119, b = 95)`
///
/// Colour `XKCD_BROWNISH_GREY` from the set `CSS4_COLORS`. (Colour number `693`)
/// ## Representations:
/// - int tuple `(134, 119, 95)`
/// - float tuple `(0.53, 0, 0)`
/// - hex: `#86775F`
pub const XKCD_BROWNISH_GREY: Color = Color(134, 119, 95);

/// Colour `(r = 198, g = 159, b = 89)`
///
/// Colour `XKCD_CAMEL` from the set `CSS4_COLORS`. (Colour number `694`)
/// ## Representations:
/// - int tuple `(198, 159, 89)`
/// - float tuple `(0.78, 1, 0)`
/// - hex: `#C69F59`
pub const XKCD_CAMEL: Color = Color(198, 159, 89);

/// Colour `(r = 122, g = 104, b = 127)`
///
/// Colour `XKCD_PURPLISH_GREY` from the set `CSS4_COLORS`. (Colour number `695`)
/// ## Representations:
/// - int tuple `(122, 104, 127)`
/// - float tuple `(0.48, 0, 0)`
/// - hex: `#7A687F`
pub const XKCD_PURPLISH_GREY: Color = Color(122, 104, 127);

/// Colour `(r = 4, g = 46, b = 96)`
///
/// Colour `XKCD_MARINE` from the set `CSS4_COLORS`. (Colour number `696`)
/// ## Representations:
/// - int tuple `(4, 46, 96)`
/// - float tuple `(0.02, 0, 0)`
/// - hex: `#042E60`
pub const XKCD_MARINE: Color = Color(4, 46, 96);

/// Colour `(r = 200, g = 141, b = 148)`
///
/// Colour `XKCD_GREYISH_PINK` from the set `CSS4_COLORS`. (Colour number `697`)
/// ## Representations:
/// - int tuple `(200, 141, 148)`
/// - float tuple `(0.78, 1, 1)`
/// - hex: `#C88D94`
pub const XKCD_GREYISH_PINK: Color = Color(200, 141, 148);

/// Colour `(r = 165, g = 251, b = 213)`
///
/// Colour `XKCD_PALE_TURQUOISE` from the set `CSS4_COLORS`. (Colour number `698`)
/// ## Representations:
/// - int tuple `(165, 251, 213)`
/// - float tuple `(0.65, 1, 1)`
/// - hex: `#A5FBD5`
pub const XKCD_PALE_TURQUOISE: Color = Color(165, 251, 213);

/// Colour `(r = 255, g = 254, b = 113)`
///
/// Colour `XKCD_PASTEL_YELLOW` from the set `CSS4_COLORS`. (Colour number `699`)
/// ## Representations:
/// - int tuple `(255, 254, 113)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFFE71`
pub const XKCD_PASTEL_YELLOW: Color = Color(255, 254, 113);

/// Colour `(r = 98, g = 65, b = 199)`
///
/// Colour `XKCD_BLUEY_PURPLE` from the set `CSS4_COLORS`. (Colour number `700`)
/// ## Representations:
/// - int tuple `(98, 65, 199)`
/// - float tuple `(0.38, 0, 1)`
/// - hex: `#6241C7`
pub const XKCD_BLUEY_PURPLE: Color = Color(98, 65, 199);

/// Colour `(r = 255, g = 254, b = 64)`
///
/// Colour `XKCD_CANARY_YELLOW` from the set `CSS4_COLORS`. (Colour number `701`)
/// ## Representations:
/// - int tuple `(255, 254, 64)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFFE40`
pub const XKCD_CANARY_YELLOW: Color = Color(255, 254, 64);

/// Colour `(r = 211, g = 73, b = 78)`
///
/// Colour `XKCD_FADED_RED` from the set `CSS4_COLORS`. (Colour number `702`)
/// ## Representations:
/// - int tuple `(211, 73, 78)`
/// - float tuple `(0.83, 0, 0)`
/// - hex: `#D3494E`
pub const XKCD_FADED_RED: Color = Color(211, 73, 78);

/// Colour `(r = 152, g = 94, b = 43)`
///
/// Colour `XKCD_SEPIA` from the set `CSS4_COLORS`. (Colour number `703`)
/// ## Representations:
/// - int tuple `(152, 94, 43)`
/// - float tuple `(0.6, 0, 0)`
/// - hex: `#985E2B`
pub const XKCD_SEPIA: Color = Color(152, 94, 43);

/// Colour `(r = 166, g = 129, b = 76)`
///
/// Colour `XKCD_COFFEE` from the set `CSS4_COLORS`. (Colour number `704`)
/// ## Representations:
/// - int tuple `(166, 129, 76)`
/// - float tuple `(0.65, 1, 0)`
/// - hex: `#A6814C`
pub const XKCD_COFFEE: Color = Color(166, 129, 76);

/// Colour `(r = 255, g = 8, b = 232)`
///
/// Colour `XKCD_BRIGHT_MAGENTA` from the set `CSS4_COLORS`. (Colour number `705`)
/// ## Representations:
/// - int tuple `(255, 8, 232)`
/// - float tuple `(1.0, 0, 1)`
/// - hex: `#FF08E8`
pub const XKCD_BRIGHT_MAGENTA: Color = Color(255, 8, 232);

/// Colour `(r = 157, g = 118, b = 81)`
///
/// Colour `XKCD_MOCHA` from the set `CSS4_COLORS`. (Colour number `706`)
/// ## Representations:
/// - int tuple `(157, 118, 81)`
/// - float tuple `(0.62, 0, 0)`
/// - hex: `#9D7651`
pub const XKCD_MOCHA: Color = Color(157, 118, 81);

/// Colour `(r = 254, g = 255, b = 202)`
///
/// Colour `XKCD_ECRU` from the set `CSS4_COLORS`. (Colour number `707`)
/// ## Representations:
/// - int tuple `(254, 255, 202)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FEFFCA`
pub const XKCD_ECRU: Color = Color(254, 255, 202);

/// Colour `(r = 152, g = 86, b = 141)`
///
/// Colour `XKCD_PURPLEISH` from the set `CSS4_COLORS`. (Colour number `708`)
/// ## Representations:
/// - int tuple `(152, 86, 141)`
/// - float tuple `(0.6, 0, 1)`
/// - hex: `#98568D`
pub const XKCD_PURPLEISH: Color = Color(152, 86, 141);

/// Colour `(r = 158, g = 0, b = 58)`
///
/// Colour `XKCD_CRANBERRY` from the set `CSS4_COLORS`. (Colour number `709`)
/// ## Representations:
/// - int tuple `(158, 0, 58)`
/// - float tuple `(0.62, 0, 0)`
/// - hex: `#9E003A`
pub const XKCD_CRANBERRY: Color = Color(158, 0, 58);

/// Colour `(r = 40, g = 124, b = 55)`
///
/// Colour `XKCD_DARKISH_GREEN` from the set `CSS4_COLORS`. (Colour number `710`)
/// ## Representations:
/// - int tuple `(40, 124, 55)`
/// - float tuple `(0.16, 0, 0)`
/// - hex: `#287C37`
pub const XKCD_DARKISH_GREEN: Color = Color(40, 124, 55);

/// Colour `(r = 185, g = 105, b = 2)`
///
/// Colour `XKCD_BROWN_ORANGE` from the set `CSS4_COLORS`. (Colour number `711`)
/// ## Representations:
/// - int tuple `(185, 105, 2)`
/// - float tuple `(0.73, 0, 0)`
/// - hex: `#B96902`
pub const XKCD_BROWN_ORANGE: Color = Color(185, 105, 2);

/// Colour `(r = 186, g = 104, b = 115)`
///
/// Colour `XKCD_DUSKY_ROSE` from the set `CSS4_COLORS`. (Colour number `712`)
/// ## Representations:
/// - int tuple `(186, 104, 115)`
/// - float tuple `(0.73, 0, 0)`
/// - hex: `#BA6873`
pub const XKCD_DUSKY_ROSE: Color = Color(186, 104, 115);

/// Colour `(r = 255, g = 120, b = 85)`
///
/// Colour `XKCD_MELON` from the set `CSS4_COLORS`. (Colour number `713`)
/// ## Representations:
/// - int tuple `(255, 120, 85)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FF7855`
pub const XKCD_MELON: Color = Color(255, 120, 85);

/// Colour `(r = 148, g = 178, b = 28)`
///
/// Colour `XKCD_SICKLY_GREEN` from the set `CSS4_COLORS`. (Colour number `714`)
/// ## Representations:
/// - int tuple `(148, 178, 28)`
/// - float tuple `(0.58, 1, 0)`
/// - hex: `#94B21C`
pub const XKCD_SICKLY_GREEN: Color = Color(148, 178, 28);

/// Colour `(r = 197, g = 201, b = 199)`
///
/// Colour `XKCD_SILVER` from the set `CSS4_COLORS`. (Colour number `715`)
/// ## Representations:
/// - int tuple `(197, 201, 199)`
/// - float tuple `(0.77, 1, 1)`
/// - hex: `#C5C9C7`
pub const XKCD_SILVER: Color = Color(197, 201, 199);

/// Colour `(r = 102, g = 26, b = 238)`
///
/// Colour `XKCD_PURPLY_BLUE` from the set `CSS4_COLORS`. (Colour number `716`)
/// ## Representations:
/// - int tuple `(102, 26, 238)`
/// - float tuple `(0.4, 0, 1)`
/// - hex: `#661AEE`
pub const XKCD_PURPLY_BLUE: Color = Color(102, 26, 238);

/// Colour `(r = 97, g = 64, b = 239)`
///
/// Colour `XKCD_PURPLEISH_BLUE` from the set `CSS4_COLORS`. (Colour number `717`)
/// ## Representations:
/// - int tuple `(97, 64, 239)`
/// - float tuple `(0.38, 0, 1)`
/// - hex: `#6140EF`
pub const XKCD_PURPLEISH_BLUE: Color = Color(97, 64, 239);

/// Colour `(r = 155, g = 229, b = 170)`
///
/// Colour `XKCD_HOSPITAL_GREEN` from the set `CSS4_COLORS`. (Colour number `718`)
/// ## Representations:
/// - int tuple `(155, 229, 170)`
/// - float tuple `(0.61, 1, 1)`
/// - hex: `#9BE5AA`
pub const XKCD_HOSPITAL_GREEN: Color = Color(155, 229, 170);

/// Colour `(r = 123, g = 88, b = 4)`
///
/// Colour `XKCD_SHIT_BROWN` from the set `CSS4_COLORS`. (Colour number `719`)
/// ## Representations:
/// - int tuple `(123, 88, 4)`
/// - float tuple `(0.48, 0, 0)`
/// - hex: `#7B5804`
pub const XKCD_SHIT_BROWN: Color = Color(123, 88, 4);

/// Colour `(r = 39, g = 106, b = 179)`
///
/// Colour `XKCD_MID_BLUE` from the set `CSS4_COLORS`. (Colour number `720`)
/// ## Representations:
/// - int tuple `(39, 106, 179)`
/// - float tuple `(0.15, 0, 1)`
/// - hex: `#276AB3`
pub const XKCD_MID_BLUE: Color = Color(39, 106, 179);

/// Colour `(r = 254, g = 179, b = 8)`
///
/// Colour `XKCD_AMBER` from the set `CSS4_COLORS`. (Colour number `721`)
/// ## Representations:
/// - int tuple `(254, 179, 8)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FEB308`
pub const XKCD_AMBER: Color = Color(254, 179, 8);

/// Colour `(r = 140, g = 253, b = 126)`
///
/// Colour `XKCD_EASTER_GREEN` from the set `CSS4_COLORS`. (Colour number `722`)
/// ## Representations:
/// - int tuple `(140, 253, 126)`
/// - float tuple `(0.55, 1, 0)`
/// - hex: `#8CFD7E`
pub const XKCD_EASTER_GREEN: Color = Color(140, 253, 126);

/// Colour `(r = 100, g = 136, b = 234)`
///
/// Colour `XKCD_SOFT_BLUE` from the set `CSS4_COLORS`. (Colour number `723`)
/// ## Representations:
/// - int tuple `(100, 136, 234)`
/// - float tuple `(0.39, 1, 1)`
/// - hex: `#6488EA`
pub const XKCD_SOFT_BLUE: Color = Color(100, 136, 234);

/// Colour `(r = 5, g = 110, b = 238)`
///
/// Colour `XKCD_CERULEAN_BLUE` from the set `CSS4_COLORS`. (Colour number `724`)
/// ## Representations:
/// - int tuple `(5, 110, 238)`
/// - float tuple `(0.02, 0, 1)`
/// - hex: `#056EEE`
pub const XKCD_CERULEAN_BLUE: Color = Color(5, 110, 238);

/// Colour `(r = 178, g = 122, b = 1)`
///
/// Colour `XKCD_GOLDEN_BROWN` from the set `CSS4_COLORS`. (Colour number `725`)
/// ## Representations:
/// - int tuple `(178, 122, 1)`
/// - float tuple `(0.7, 0, 0)`
/// - hex: `#B27A01`
pub const XKCD_GOLDEN_BROWN: Color = Color(178, 122, 1);

/// Colour `(r = 15, g = 254, b = 249)`
///
/// Colour `XKCD_BRIGHT_TURQUOISE` from the set `CSS4_COLORS`. (Colour number `726`)
/// ## Representations:
/// - int tuple `(15, 254, 249)`
/// - float tuple `(0.06, 1, 1)`
/// - hex: `#0FFEF9`
pub const XKCD_BRIGHT_TURQUOISE: Color = Color(15, 254, 249);

/// Colour `(r = 250, g = 42, b = 85)`
///
/// Colour `XKCD_RED_PINK` from the set `CSS4_COLORS`. (Colour number `727`)
/// ## Representations:
/// - int tuple `(250, 42, 85)`
/// - float tuple `(0.98, 0, 0)`
/// - hex: `#FA2A55`
pub const XKCD_RED_PINK: Color = Color(250, 42, 85);

/// Colour `(r = 130, g = 7, b = 71)`
///
/// Colour `XKCD_RED_PURPLE` from the set `CSS4_COLORS`. (Colour number `728`)
/// ## Representations:
/// - int tuple `(130, 7, 71)`
/// - float tuple `(0.51, 0, 0)`
/// - hex: `#820747`
pub const XKCD_RED_PURPLE: Color = Color(130, 7, 71);

/// Colour `(r = 122, g = 106, b = 79)`
///
/// Colour `XKCD_GREYISH_BROWN` from the set `CSS4_COLORS`. (Colour number `729`)
/// ## Representations:
/// - int tuple `(122, 106, 79)`
/// - float tuple `(0.48, 0, 0)`
/// - hex: `#7A6A4F`
pub const XKCD_GREYISH_BROWN: Color = Color(122, 106, 79);

/// Colour `(r = 244, g = 50, b = 12)`
///
/// Colour `XKCD_VERMILLION` from the set `CSS4_COLORS`. (Colour number `730`)
/// ## Representations:
/// - int tuple `(244, 50, 12)`
/// - float tuple `(0.96, 0, 0)`
/// - hex: `#F4320C`
pub const XKCD_VERMILLION: Color = Color(244, 50, 12);

/// Colour `(r = 161, g = 57, b = 5)`
///
/// Colour `XKCD_RUSSET` from the set `CSS4_COLORS`. (Colour number `731`)
/// ## Representations:
/// - int tuple `(161, 57, 5)`
/// - float tuple `(0.63, 0, 0)`
/// - hex: `#A13905`
pub const XKCD_RUSSET: Color = Color(161, 57, 5);

/// Colour `(r = 111, g = 130, b = 138)`
///
/// Colour `XKCD_STEEL_GREY` from the set `CSS4_COLORS`. (Colour number `732`)
/// ## Representations:
/// - int tuple `(111, 130, 138)`
/// - float tuple `(0.44, 1, 1)`
/// - hex: `#6F828A`
pub const XKCD_STEEL_GREY: Color = Color(111, 130, 138);

/// Colour `(r = 165, g = 90, b = 244)`
///
/// Colour `XKCD_LIGHTER_PURPLE` from the set `CSS4_COLORS`. (Colour number `733`)
/// ## Representations:
/// - int tuple `(165, 90, 244)`
/// - float tuple `(0.65, 0, 1)`
/// - hex: `#A55AF4`
pub const XKCD_LIGHTER_PURPLE: Color = Color(165, 90, 244);

/// Colour `(r = 173, g = 10, b = 253)`
///
/// Colour `XKCD_BRIGHT_VIOLET` from the set `CSS4_COLORS`. (Colour number `734`)
/// ## Representations:
/// - int tuple `(173, 10, 253)`
/// - float tuple `(0.68, 0, 1)`
/// - hex: `#AD0AFD`
pub const XKCD_BRIGHT_VIOLET: Color = Color(173, 10, 253);

/// Colour `(r = 0, g = 69, b = 119)`
///
/// Colour `XKCD_PRUSSIAN_BLUE` from the set `CSS4_COLORS`. (Colour number `735`)
/// ## Representations:
/// - int tuple `(0, 69, 119)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#004577`
pub const XKCD_PRUSSIAN_BLUE: Color = Color(0, 69, 119);

/// Colour `(r = 101, g = 141, b = 109)`
///
/// Colour `XKCD_SLATE_GREEN` from the set `CSS4_COLORS`. (Colour number `736`)
/// ## Representations:
/// - int tuple `(101, 141, 109)`
/// - float tuple `(0.4, 1, 0)`
/// - hex: `#658D6D`
pub const XKCD_SLATE_GREEN: Color = Color(101, 141, 109);

/// Colour `(r = 202, g = 123, b = 128)`
///
/// Colour `XKCD_DIRTY_PINK` from the set `CSS4_COLORS`. (Colour number `737`)
/// ## Representations:
/// - int tuple `(202, 123, 128)`
/// - float tuple `(0.79, 0, 1)`
/// - hex: `#CA7B80`
pub const XKCD_DIRTY_PINK: Color = Color(202, 123, 128);

/// Colour `(r = 0, g = 82, b = 73)`
///
/// Colour `XKCD_DARK_BLUE_GREEN` from the set `CSS4_COLORS`. (Colour number `738`)
/// ## Representations:
/// - int tuple `(0, 82, 73)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#005249`
pub const XKCD_DARK_BLUE_GREEN: Color = Color(0, 82, 73);

/// Colour `(r = 43, g = 93, b = 52)`
///
/// Colour `XKCD_PINE` from the set `CSS4_COLORS`. (Colour number `739`)
/// ## Representations:
/// - int tuple `(43, 93, 52)`
/// - float tuple `(0.17, 0, 0)`
/// - hex: `#2B5D34`
pub const XKCD_PINE: Color = Color(43, 93, 52);

/// Colour `(r = 191, g = 241, b = 40)`
///
/// Colour `XKCD_YELLOWY_GREEN` from the set `CSS4_COLORS`. (Colour number `740`)
/// ## Representations:
/// - int tuple `(191, 241, 40)`
/// - float tuple `(0.75, 1, 0)`
/// - hex: `#BFF128`
pub const XKCD_YELLOWY_GREEN: Color = Color(191, 241, 40);

/// Colour `(r = 181, g = 148, b = 16)`
///
/// Colour `XKCD_DARK_GOLD` from the set `CSS4_COLORS`. (Colour number `741`)
/// ## Representations:
/// - int tuple `(181, 148, 16)`
/// - float tuple `(0.71, 1, 0)`
/// - hex: `#B59410`
pub const XKCD_DARK_GOLD: Color = Color(181, 148, 16);

/// Colour `(r = 41, g = 118, b = 187)`
///
/// Colour `XKCD_BLUISH` from the set `CSS4_COLORS`. (Colour number `742`)
/// ## Representations:
/// - int tuple `(41, 118, 187)`
/// - float tuple `(0.16, 0, 1)`
/// - hex: `#2976BB`
pub const XKCD_BLUISH: Color = Color(41, 118, 187);

/// Colour `(r = 1, g = 65, b = 130)`
///
/// Colour `XKCD_DARKISH_BLUE` from the set `CSS4_COLORS`. (Colour number `743`)
/// ## Representations:
/// - int tuple `(1, 65, 130)`
/// - float tuple `(0.0, 0, 1)`
/// - hex: `#014182`
pub const XKCD_DARKISH_BLUE: Color = Color(1, 65, 130);

/// Colour `(r = 187, g = 63, b = 63)`
///
/// Colour `XKCD_DULL_RED` from the set `CSS4_COLORS`. (Colour number `744`)
/// ## Representations:
/// - int tuple `(187, 63, 63)`
/// - float tuple `(0.73, 0, 0)`
/// - hex: `#BB3F3F`
pub const XKCD_DULL_RED: Color = Color(187, 63, 63);

/// Colour `(r = 252, g = 38, b = 71)`
///
/// Colour `XKCD_PINKY_RED` from the set `CSS4_COLORS`. (Colour number `745`)
/// ## Representations:
/// - int tuple `(252, 38, 71)`
/// - float tuple `(0.99, 0, 0)`
/// - hex: `#FC2647`
pub const XKCD_PINKY_RED: Color = Color(252, 38, 71);

/// Colour `(r = 168, g = 121, b = 0)`
///
/// Colour `XKCD_BRONZE` from the set `CSS4_COLORS`. (Colour number `746`)
/// ## Representations:
/// - int tuple `(168, 121, 0)`
/// - float tuple `(0.66, 0, 0)`
/// - hex: `#A87900`
pub const XKCD_BRONZE: Color = Color(168, 121, 0);

/// Colour `(r = 130, g = 203, b = 178)`
///
/// Colour `XKCD_PALE_TEAL` from the set `CSS4_COLORS`. (Colour number `747`)
/// ## Representations:
/// - int tuple `(130, 203, 178)`
/// - float tuple `(0.51, 1, 1)`
/// - hex: `#82CBB2`
pub const XKCD_PALE_TEAL: Color = Color(130, 203, 178);

/// Colour `(r = 102, g = 124, b = 62)`
///
/// Colour `XKCD_MILITARY_GREEN` from the set `CSS4_COLORS`. (Colour number `748`)
/// ## Representations:
/// - int tuple `(102, 124, 62)`
/// - float tuple `(0.4, 0, 0)`
/// - hex: `#667C3E`
pub const XKCD_MILITARY_GREEN: Color = Color(102, 124, 62);

/// Colour `(r = 254, g = 70, b = 165)`
///
/// Colour `XKCD_BARBIE_PINK` from the set `CSS4_COLORS`. (Colour number `749`)
/// ## Representations:
/// - int tuple `(254, 70, 165)`
/// - float tuple `(1.0, 0, 1)`
/// - hex: `#FE46A5`
pub const XKCD_BARBIE_PINK: Color = Color(254, 70, 165);

/// Colour `(r = 254, g = 131, b = 204)`
///
/// Colour `XKCD_BUBBLEGUM_PINK` from the set `CSS4_COLORS`. (Colour number `750`)
/// ## Representations:
/// - int tuple `(254, 131, 204)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FE83CC`
pub const XKCD_BUBBLEGUM_PINK: Color = Color(254, 131, 204);

/// Colour `(r = 148, g = 166, b = 23)`
///
/// Colour `XKCD_PEA_SOUP_GREEN` from the set `CSS4_COLORS`. (Colour number `751`)
/// ## Representations:
/// - int tuple `(148, 166, 23)`
/// - float tuple `(0.58, 1, 0)`
/// - hex: `#94A617`
pub const XKCD_PEA_SOUP_GREEN: Color = Color(148, 166, 23);

/// Colour `(r = 168, g = 137, b = 5)`
///
/// Colour `XKCD_DARK_MUSTARD` from the set `CSS4_COLORS`. (Colour number `752`)
/// ## Representations:
/// - int tuple `(168, 137, 5)`
/// - float tuple `(0.66, 1, 0)`
/// - hex: `#A88905`
pub const XKCD_DARK_MUSTARD: Color = Color(168, 137, 5);

/// Colour `(r = 127, g = 95, b = 0)`
///
/// Colour `XKCD_SHIT` from the set `CSS4_COLORS`. (Colour number `753`)
/// ## Representations:
/// - int tuple `(127, 95, 0)`
/// - float tuple `(0.5, 0, 0)`
/// - hex: `#7F5F00`
pub const XKCD_SHIT: Color = Color(127, 95, 0);

/// Colour `(r = 158, g = 67, b = 162)`
///
/// Colour `XKCD_MEDIUM_PURPLE` from the set `CSS4_COLORS`. (Colour number `754`)
/// ## Representations:
/// - int tuple `(158, 67, 162)`
/// - float tuple `(0.62, 0, 1)`
/// - hex: `#9E43A2`
pub const XKCD_MEDIUM_PURPLE: Color = Color(158, 67, 162);

/// Colour `(r = 6, g = 46, b = 3)`
///
/// Colour `XKCD_VERY_DARK_GREEN` from the set `CSS4_COLORS`. (Colour number `755`)
/// ## Representations:
/// - int tuple `(6, 46, 3)`
/// - float tuple `(0.02, 0, 0)`
/// - hex: `#062E03`
pub const XKCD_VERY_DARK_GREEN: Color = Color(6, 46, 3);

/// Colour `(r = 138, g = 110, b = 69)`
///
/// Colour `XKCD_DIRT` from the set `CSS4_COLORS`. (Colour number `756`)
/// ## Representations:
/// - int tuple `(138, 110, 69)`
/// - float tuple `(0.54, 0, 0)`
/// - hex: `#8A6E45`
pub const XKCD_DIRT: Color = Color(138, 110, 69);

/// Colour `(r = 204, g = 122, b = 139)`
///
/// Colour `XKCD_DUSKY_PINK` from the set `CSS4_COLORS`. (Colour number `757`)
/// ## Representations:
/// - int tuple `(204, 122, 139)`
/// - float tuple `(0.8, 0, 1)`
/// - hex: `#CC7A8B`
pub const XKCD_DUSKY_PINK: Color = Color(204, 122, 139);

/// Colour `(r = 158, g = 1, b = 104)`
///
/// Colour `XKCD_RED_VIOLET` from the set `CSS4_COLORS`. (Colour number `758`)
/// ## Representations:
/// - int tuple `(158, 1, 104)`
/// - float tuple `(0.62, 0, 0)`
/// - hex: `#9E0168`
pub const XKCD_RED_VIOLET: Color = Color(158, 1, 104);

/// Colour `(r = 253, g = 255, b = 56)`
///
/// Colour `XKCD_LEMON_YELLOW` from the set `CSS4_COLORS`. (Colour number `759`)
/// ## Representations:
/// - int tuple `(253, 255, 56)`
/// - float tuple `(0.99, 1, 0)`
/// - hex: `#FDFF38`
pub const XKCD_LEMON_YELLOW: Color = Color(253, 255, 56);

/// Colour `(r = 192, g = 250, b = 139)`
///
/// Colour `XKCD_PISTACHIO` from the set `CSS4_COLORS`. (Colour number `760`)
/// ## Representations:
/// - int tuple `(192, 250, 139)`
/// - float tuple `(0.75, 1, 1)`
/// - hex: `#C0FA8B`
pub const XKCD_PISTACHIO: Color = Color(192, 250, 139);

/// Colour `(r = 238, g = 220, b = 91)`
///
/// Colour `XKCD_DULL_YELLOW` from the set `CSS4_COLORS`. (Colour number `761`)
/// ## Representations:
/// - int tuple `(238, 220, 91)`
/// - float tuple `(0.93, 1, 0)`
/// - hex: `#EEDC5B`
pub const XKCD_DULL_YELLOW: Color = Color(238, 220, 91);

/// Colour `(r = 126, g = 189, b = 1)`
///
/// Colour `XKCD_DARK_LIME_GREEN` from the set `CSS4_COLORS`. (Colour number `762`)
/// ## Representations:
/// - int tuple `(126, 189, 1)`
/// - float tuple `(0.49, 1, 0)`
/// - hex: `#7EBD01`
pub const XKCD_DARK_LIME_GREEN: Color = Color(126, 189, 1);

/// Colour `(r = 59, g = 91, b = 146)`
///
/// Colour `XKCD_DENIM_BLUE` from the set `CSS4_COLORS`. (Colour number `763`)
/// ## Representations:
/// - int tuple `(59, 91, 146)`
/// - float tuple `(0.23, 0, 1)`
/// - hex: `#3B5B92`
pub const XKCD_DENIM_BLUE: Color = Color(59, 91, 146);

/// Colour `(r = 1, g = 136, b = 159)`
///
/// Colour `XKCD_TEAL_BLUE` from the set `CSS4_COLORS`. (Colour number `764`)
/// ## Representations:
/// - int tuple `(1, 136, 159)`
/// - float tuple `(0.0, 1, 1)`
/// - hex: `#01889F`
pub const XKCD_TEAL_BLUE: Color = Color(1, 136, 159);

/// Colour `(r = 61, g = 122, b = 253)`
///
/// Colour `XKCD_LIGHTISH_BLUE` from the set `CSS4_COLORS`. (Colour number `765`)
/// ## Representations:
/// - int tuple `(61, 122, 253)`
/// - float tuple `(0.24, 0, 1)`
/// - hex: `#3D7AFD`
pub const XKCD_LIGHTISH_BLUE: Color = Color(61, 122, 253);

/// Colour `(r = 95, g = 52, b = 231)`
///
/// Colour `XKCD_PURPLEY_BLUE` from the set `CSS4_COLORS`. (Colour number `766`)
/// ## Representations:
/// - int tuple `(95, 52, 231)`
/// - float tuple `(0.37, 0, 1)`
/// - hex: `#5F34E7`
pub const XKCD_PURPLEY_BLUE: Color = Color(95, 52, 231);

/// Colour `(r = 109, g = 90, b = 207)`
///
/// Colour `XKCD_LIGHT_INDIGO` from the set `CSS4_COLORS`. (Colour number `767`)
/// ## Representations:
/// - int tuple `(109, 90, 207)`
/// - float tuple `(0.43, 0, 1)`
/// - hex: `#6D5ACF`
pub const XKCD_LIGHT_INDIGO: Color = Color(109, 90, 207);

/// Colour `(r = 116, g = 133, b = 0)`
///
/// Colour `XKCD_SWAMP_GREEN` from the set `CSS4_COLORS`. (Colour number `768`)
/// ## Representations:
/// - int tuple `(116, 133, 0)`
/// - float tuple `(0.45, 1, 0)`
/// - hex: `#748500`
pub const XKCD_SWAMP_GREEN: Color = Color(116, 133, 0);

/// Colour `(r = 112, g = 108, b = 17)`
///
/// Colour `XKCD_BROWN_GREEN` from the set `CSS4_COLORS`. (Colour number `769`)
/// ## Representations:
/// - int tuple `(112, 108, 17)`
/// - float tuple `(0.44, 0, 0)`
/// - hex: `#706C11`
pub const XKCD_BROWN_GREEN: Color = Color(112, 108, 17);

/// Colour `(r = 60, g = 0, b = 8)`
///
/// Colour `XKCD_DARK_MAROON` from the set `CSS4_COLORS`. (Colour number `770`)
/// ## Representations:
/// - int tuple `(60, 0, 8)`
/// - float tuple `(0.24, 0, 0)`
/// - hex: `#3C0008`
pub const XKCD_DARK_MAROON: Color = Color(60, 0, 8);

/// Colour `(r = 203, g = 0, b = 245)`
///
/// Colour `XKCD_HOT_PURPLE` from the set `CSS4_COLORS`. (Colour number `771`)
/// ## Representations:
/// - int tuple `(203, 0, 245)`
/// - float tuple `(0.8, 0, 1)`
/// - hex: `#CB00F5`
pub const XKCD_HOT_PURPLE: Color = Color(203, 0, 245);

/// Colour `(r = 0, g = 45, b = 4)`
///
/// Colour `XKCD_DARK_FOREST_GREEN` from the set `CSS4_COLORS`. (Colour number `772`)
/// ## Representations:
/// - int tuple `(0, 45, 4)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#002D04`
pub const XKCD_DARK_FOREST_GREEN: Color = Color(0, 45, 4);

/// Colour `(r = 101, g = 140, b = 187)`
///
/// Colour `XKCD_FADED_BLUE` from the set `CSS4_COLORS`. (Colour number `773`)
/// ## Representations:
/// - int tuple `(101, 140, 187)`
/// - float tuple `(0.4, 1, 1)`
/// - hex: `#658CBB`
pub const XKCD_FADED_BLUE: Color = Color(101, 140, 187);

/// Colour `(r = 116, g = 149, b = 81)`
///
/// Colour `XKCD_DRAB_GREEN` from the set `CSS4_COLORS`. (Colour number `774`)
/// ## Representations:
/// - int tuple `(116, 149, 81)`
/// - float tuple `(0.45, 1, 0)`
/// - hex: `#749551`
pub const XKCD_DRAB_GREEN: Color = Color(116, 149, 81);

/// Colour `(r = 185, g = 255, b = 102)`
///
/// Colour `XKCD_LIGHT_LIME_GREEN` from the set `CSS4_COLORS`. (Colour number `775`)
/// ## Representations:
/// - int tuple `(185, 255, 102)`
/// - float tuple `(0.73, 1, 0)`
/// - hex: `#B9FF66`
pub const XKCD_LIGHT_LIME_GREEN: Color = Color(185, 255, 102);

/// Colour `(r = 157, g = 193, b = 0)`
///
/// Colour `XKCD_SNOT_GREEN` from the set `CSS4_COLORS`. (Colour number `776`)
/// ## Representations:
/// - int tuple `(157, 193, 0)`
/// - float tuple `(0.62, 1, 0)`
/// - hex: `#9DC100`
pub const XKCD_SNOT_GREEN: Color = Color(157, 193, 0);

/// Colour `(r = 250, g = 238, b = 102)`
///
/// Colour `XKCD_YELLOWISH` from the set `CSS4_COLORS`. (Colour number `777`)
/// ## Representations:
/// - int tuple `(250, 238, 102)`
/// - float tuple `(0.98, 1, 0)`
/// - hex: `#FAEE66`
pub const XKCD_YELLOWISH: Color = Color(250, 238, 102);

/// Colour `(r = 126, g = 251, b = 179)`
///
/// Colour `XKCD_LIGHT_BLUE_GREEN` from the set `CSS4_COLORS`. (Colour number `778`)
/// ## Representations:
/// - int tuple `(126, 251, 179)`
/// - float tuple `(0.49, 1, 1)`
/// - hex: `#7EFBB3`
pub const XKCD_LIGHT_BLUE_GREEN: Color = Color(126, 251, 179);

/// Colour `(r = 123, g = 0, b = 44)`
///
/// Colour `XKCD_BORDEAUX` from the set `CSS4_COLORS`. (Colour number `779`)
/// ## Representations:
/// - int tuple `(123, 0, 44)`
/// - float tuple `(0.48, 0, 0)`
/// - hex: `#7B002C`
pub const XKCD_BORDEAUX: Color = Color(123, 0, 44);

/// Colour `(r = 194, g = 146, b = 161)`
///
/// Colour `XKCD_LIGHT_MAUVE` from the set `CSS4_COLORS`. (Colour number `780`)
/// ## Representations:
/// - int tuple `(194, 146, 161)`
/// - float tuple `(0.76, 1, 1)`
/// - hex: `#C292A1`
pub const XKCD_LIGHT_MAUVE: Color = Color(194, 146, 161);

/// Colour `(r = 1, g = 123, b = 146)`
///
/// Colour `XKCD_OCEAN` from the set `CSS4_COLORS`. (Colour number `781`)
/// ## Representations:
/// - int tuple `(1, 123, 146)`
/// - float tuple `(0.0, 0, 1)`
/// - hex: `#017B92`
pub const XKCD_OCEAN: Color = Color(1, 123, 146);

/// Colour `(r = 252, g = 192, b = 6)`
///
/// Colour `XKCD_MARIGOLD` from the set `CSS4_COLORS`. (Colour number `782`)
/// ## Representations:
/// - int tuple `(252, 192, 6)`
/// - float tuple `(0.99, 1, 0)`
/// - hex: `#FCC006`
pub const XKCD_MARIGOLD: Color = Color(252, 192, 6);

/// Colour `(r = 101, g = 116, b = 50)`
///
/// Colour `XKCD_MUDDY_GREEN` from the set `CSS4_COLORS`. (Colour number `783`)
/// ## Representations:
/// - int tuple `(101, 116, 50)`
/// - float tuple `(0.4, 0, 0)`
/// - hex: `#657432`
pub const XKCD_MUDDY_GREEN: Color = Color(101, 116, 50);

/// Colour `(r = 216, g = 134, b = 59)`
///
/// Colour `XKCD_DULL_ORANGE` from the set `CSS4_COLORS`. (Colour number `784`)
/// ## Representations:
/// - int tuple `(216, 134, 59)`
/// - float tuple `(0.85, 1, 0)`
/// - hex: `#D8863B`
pub const XKCD_DULL_ORANGE: Color = Color(216, 134, 59);

/// Colour `(r = 115, g = 133, b = 149)`
///
/// Colour `XKCD_STEEL` from the set `CSS4_COLORS`. (Colour number `785`)
/// ## Representations:
/// - int tuple `(115, 133, 149)`
/// - float tuple `(0.45, 1, 1)`
/// - hex: `#738595`
pub const XKCD_STEEL: Color = Color(115, 133, 149);

/// Colour `(r = 170, g = 35, b = 255)`
///
/// Colour `XKCD_ELECTRIC_PURPLE` from the set `CSS4_COLORS`. (Colour number `786`)
/// ## Representations:
/// - int tuple `(170, 35, 255)`
/// - float tuple `(0.67, 0, 1)`
/// - hex: `#AA23FF`
pub const XKCD_ELECTRIC_PURPLE: Color = Color(170, 35, 255);

/// Colour `(r = 8, g = 255, b = 8)`
///
/// Colour `XKCD_FLUORESCENT_GREEN` from the set `CSS4_COLORS`. (Colour number `787`)
/// ## Representations:
/// - int tuple `(8, 255, 8)`
/// - float tuple `(0.03, 1, 0)`
/// - hex: `#08FF08`
pub const XKCD_FLUORESCENT_GREEN: Color = Color(8, 255, 8);

/// Colour `(r = 155, g = 122, b = 1)`
///
/// Colour `XKCD_YELLOWISH_BROWN` from the set `CSS4_COLORS`. (Colour number `788`)
/// ## Representations:
/// - int tuple `(155, 122, 1)`
/// - float tuple `(0.61, 0, 0)`
/// - hex: `#9B7A01`
pub const XKCD_YELLOWISH_BROWN: Color = Color(155, 122, 1);

/// Colour `(r = 242, g = 158, b = 142)`
///
/// Colour `XKCD_BLUSH` from the set `CSS4_COLORS`. (Colour number `789`)
/// ## Representations:
/// - int tuple `(242, 158, 142)`
/// - float tuple `(0.95, 1, 1)`
/// - hex: `#F29E8E`
pub const XKCD_BLUSH: Color = Color(242, 158, 142);

/// Colour `(r = 111, g = 194, b = 118)`
///
/// Colour `XKCD_SOFT_GREEN` from the set `CSS4_COLORS`. (Colour number `790`)
/// ## Representations:
/// - int tuple `(111, 194, 118)`
/// - float tuple `(0.44, 1, 0)`
/// - hex: `#6FC276`
pub const XKCD_SOFT_GREEN: Color = Color(111, 194, 118);

/// Colour `(r = 255, g = 91, b = 0)`
///
/// Colour `XKCD_BRIGHT_ORANGE` from the set `CSS4_COLORS`. (Colour number `791`)
/// ## Representations:
/// - int tuple `(255, 91, 0)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FF5B00`
pub const XKCD_BRIGHT_ORANGE: Color = Color(255, 91, 0);

/// Colour `(r = 253, g = 255, b = 82)`
///
/// Colour `XKCD_LEMON` from the set `CSS4_COLORS`. (Colour number `792`)
/// ## Representations:
/// - int tuple `(253, 255, 82)`
/// - float tuple `(0.99, 1, 0)`
/// - hex: `#FDFF52`
pub const XKCD_LEMON: Color = Color(253, 255, 82);

/// Colour `(r = 134, g = 111, b = 133)`
///
/// Colour `XKCD_PURPLE_GREY` from the set `CSS4_COLORS`. (Colour number `793`)
/// ## Representations:
/// - int tuple `(134, 111, 133)`
/// - float tuple `(0.53, 0, 1)`
/// - hex: `#866F85`
pub const XKCD_PURPLE_GREY: Color = Color(134, 111, 133);

/// Colour `(r = 143, g = 254, b = 9)`
///
/// Colour `XKCD_ACID_GREEN` from the set `CSS4_COLORS`. (Colour number `794`)
/// ## Representations:
/// - int tuple `(143, 254, 9)`
/// - float tuple `(0.56, 1, 0)`
/// - hex: `#8FFE09`
pub const XKCD_ACID_GREEN: Color = Color(143, 254, 9);

/// Colour `(r = 238, g = 207, b = 254)`
///
/// Colour `XKCD_PALE_LAVENDER` from the set `CSS4_COLORS`. (Colour number `795`)
/// ## Representations:
/// - int tuple `(238, 207, 254)`
/// - float tuple `(0.93, 1, 1)`
/// - hex: `#EECFFE`
pub const XKCD_PALE_LAVENDER: Color = Color(238, 207, 254);

/// Colour `(r = 81, g = 10, b = 201)`
///
/// Colour `XKCD_VIOLET_BLUE` from the set `CSS4_COLORS`. (Colour number `796`)
/// ## Representations:
/// - int tuple `(81, 10, 201)`
/// - float tuple `(0.32, 0, 1)`
/// - hex: `#510AC9`
pub const XKCD_VIOLET_BLUE: Color = Color(81, 10, 201);

/// Colour `(r = 79, g = 145, b = 83)`
///
/// Colour `XKCD_LIGHT_FOREST_GREEN` from the set `CSS4_COLORS`. (Colour number `797`)
/// ## Representations:
/// - int tuple `(79, 145, 83)`
/// - float tuple `(0.31, 1, 0)`
/// - hex: `#4F9153`
pub const XKCD_LIGHT_FOREST_GREEN: Color = Color(79, 145, 83);

/// Colour `(r = 159, g = 35, b = 5)`
///
/// Colour `XKCD_BURNT_RED` from the set `CSS4_COLORS`. (Colour number `798`)
/// ## Representations:
/// - int tuple `(159, 35, 5)`
/// - float tuple `(0.62, 0, 0)`
/// - hex: `#9F2305`
pub const XKCD_BURNT_RED: Color = Color(159, 35, 5);

/// Colour `(r = 114, g = 134, b = 57)`
///
/// Colour `XKCD_KHAKI_GREEN` from the set `CSS4_COLORS`. (Colour number `799`)
/// ## Representations:
/// - int tuple `(114, 134, 57)`
/// - float tuple `(0.45, 1, 0)`
/// - hex: `#728639`
pub const XKCD_KHAKI_GREEN: Color = Color(114, 134, 57);

/// Colour `(r = 222, g = 12, b = 98)`
///
/// Colour `XKCD_CERISE` from the set `CSS4_COLORS`. (Colour number `800`)
/// ## Representations:
/// - int tuple `(222, 12, 98)`
/// - float tuple `(0.87, 0, 0)`
/// - hex: `#DE0C62`
pub const XKCD_CERISE: Color = Color(222, 12, 98);

/// Colour `(r = 145, g = 110, b = 153)`
///
/// Colour `XKCD_FADED_PURPLE` from the set `CSS4_COLORS`. (Colour number `801`)
/// ## Representations:
/// - int tuple `(145, 110, 153)`
/// - float tuple `(0.57, 0, 1)`
/// - hex: `#916E99`
pub const XKCD_FADED_PURPLE: Color = Color(145, 110, 153);

/// Colour `(r = 255, g = 177, b = 109)`
///
/// Colour `XKCD_APRICOT` from the set `CSS4_COLORS`. (Colour number `802`)
/// ## Representations:
/// - int tuple `(255, 177, 109)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFB16D`
pub const XKCD_APRICOT: Color = Color(255, 177, 109);

/// Colour `(r = 60, g = 77, b = 3)`
///
/// Colour `XKCD_DARK_OLIVE_GREEN` from the set `CSS4_COLORS`. (Colour number `803`)
/// ## Representations:
/// - int tuple `(60, 77, 3)`
/// - float tuple `(0.24, 0, 0)`
/// - hex: `#3C4D03`
pub const XKCD_DARK_OLIVE_GREEN: Color = Color(60, 77, 3);

/// Colour `(r = 127, g = 112, b = 83)`
///
/// Colour `XKCD_GREY_BROWN` from the set `CSS4_COLORS`. (Colour number `804`)
/// ## Representations:
/// - int tuple `(127, 112, 83)`
/// - float tuple `(0.5, 0, 0)`
/// - hex: `#7F7053`
pub const XKCD_GREY_BROWN: Color = Color(127, 112, 83);

/// Colour `(r = 119, g = 146, b = 111)`
///
/// Colour `XKCD_GREEN_GREY` from the set `CSS4_COLORS`. (Colour number `805`)
/// ## Representations:
/// - int tuple `(119, 146, 111)`
/// - float tuple `(0.47, 1, 0)`
/// - hex: `#77926F`
pub const XKCD_GREEN_GREY: Color = Color(119, 146, 111);

/// Colour `(r = 1, g = 15, b = 204)`
///
/// Colour `XKCD_TRUE_BLUE` from the set `CSS4_COLORS`. (Colour number `806`)
/// ## Representations:
/// - int tuple `(1, 15, 204)`
/// - float tuple `(0.0, 0, 1)`
/// - hex: `#010FCC`
pub const XKCD_TRUE_BLUE: Color = Color(1, 15, 204);

/// Colour `(r = 206, g = 174, b = 250)`
///
/// Colour `XKCD_PALE_VIOLET` from the set `CSS4_COLORS`. (Colour number `807`)
/// ## Representations:
/// - int tuple `(206, 174, 250)`
/// - float tuple `(0.81, 1, 1)`
/// - hex: `#CEAEFA`
pub const XKCD_PALE_VIOLET: Color = Color(206, 174, 250);

/// Colour `(r = 143, g = 153, b = 251)`
///
/// Colour `XKCD_PERIWINKLE_BLUE` from the set `CSS4_COLORS`. (Colour number `808`)
/// ## Representations:
/// - int tuple `(143, 153, 251)`
/// - float tuple `(0.56, 1, 1)`
/// - hex: `#8F99FB`
pub const XKCD_PERIWINKLE_BLUE: Color = Color(143, 153, 251);

/// Colour `(r = 198, g = 252, b = 255)`
///
/// Colour `XKCD_LIGHT_SKY_BLUE` from the set `CSS4_COLORS`. (Colour number `809`)
/// ## Representations:
/// - int tuple `(198, 252, 255)`
/// - float tuple `(0.78, 1, 1)`
/// - hex: `#C6FCFF`
pub const XKCD_LIGHT_SKY_BLUE: Color = Color(198, 252, 255);

/// Colour `(r = 85, g = 57, b = 204)`
///
/// Colour `XKCD_BLURPLE` from the set `CSS4_COLORS`. (Colour number `810`)
/// ## Representations:
/// - int tuple `(85, 57, 204)`
/// - float tuple `(0.33, 0, 1)`
/// - hex: `#5539CC`
pub const XKCD_BLURPLE: Color = Color(85, 57, 204);

/// Colour `(r = 84, g = 78, b = 3)`
///
/// Colour `XKCD_GREEN_BROWN` from the set `CSS4_COLORS`. (Colour number `811`)
/// ## Representations:
/// - int tuple `(84, 78, 3)`
/// - float tuple `(0.33, 0, 0)`
/// - hex: `#544E03`
pub const XKCD_GREEN_BROWN: Color = Color(84, 78, 3);

/// Colour `(r = 1, g = 122, b = 121)`
///
/// Colour `XKCD_BLUEGREEN` from the set `CSS4_COLORS`. (Colour number `812`)
/// ## Representations:
/// - int tuple `(1, 122, 121)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#017A79`
pub const XKCD_BLUEGREEN: Color = Color(1, 122, 121);

/// Colour `(r = 1, g = 249, b = 198)`
///
/// Colour `XKCD_BRIGHT_TEAL` from the set `CSS4_COLORS`. (Colour number `813`)
/// ## Representations:
/// - int tuple `(1, 249, 198)`
/// - float tuple `(0.0, 1, 1)`
/// - hex: `#01F9C6`
pub const XKCD_BRIGHT_TEAL: Color = Color(1, 249, 198);

/// Colour `(r = 201, g = 176, b = 3)`
///
/// Colour `XKCD_BROWNISH_YELLOW` from the set `CSS4_COLORS`. (Colour number `814`)
/// ## Representations:
/// - int tuple `(201, 176, 3)`
/// - float tuple `(0.79, 1, 0)`
/// - hex: `#C9B003`
pub const XKCD_BROWNISH_YELLOW: Color = Color(201, 176, 3);

/// Colour `(r = 146, g = 153, b = 1)`
///
/// Colour `XKCD_PEA_SOUP` from the set `CSS4_COLORS`. (Colour number `815`)
/// ## Representations:
/// - int tuple `(146, 153, 1)`
/// - float tuple `(0.57, 1, 0)`
/// - hex: `#929901`
pub const XKCD_PEA_SOUP: Color = Color(146, 153, 1);

/// Colour `(r = 11, g = 85, b = 9)`
///
/// Colour `XKCD_FOREST` from the set `CSS4_COLORS`. (Colour number `816`)
/// ## Representations:
/// - int tuple `(11, 85, 9)`
/// - float tuple `(0.04, 0, 0)`
/// - hex: `#0B5509`
pub const XKCD_FOREST: Color = Color(11, 85, 9);

/// Colour `(r = 160, g = 4, b = 152)`
///
/// Colour `XKCD_BARNEY_PURPLE` from the set `CSS4_COLORS`. (Colour number `817`)
/// ## Representations:
/// - int tuple `(160, 4, 152)`
/// - float tuple `(0.63, 0, 1)`
/// - hex: `#A00498`
pub const XKCD_BARNEY_PURPLE: Color = Color(160, 4, 152);

/// Colour `(r = 32, g = 0, b = 177)`
///
/// Colour `XKCD_ULTRAMARINE` from the set `CSS4_COLORS`. (Colour number `818`)
/// ## Representations:
/// - int tuple `(32, 0, 177)`
/// - float tuple `(0.13, 0, 1)`
/// - hex: `#2000B1`
pub const XKCD_ULTRAMARINE: Color = Color(32, 0, 177);

/// Colour `(r = 148, g = 86, b = 140)`
///
/// Colour `XKCD_PURPLISH` from the set `CSS4_COLORS`. (Colour number `819`)
/// ## Representations:
/// - int tuple `(148, 86, 140)`
/// - float tuple `(0.58, 0, 1)`
/// - hex: `#94568C`
pub const XKCD_PURPLISH: Color = Color(148, 86, 140);

/// Colour `(r = 194, g = 190, b = 14)`
///
/// Colour `XKCD_PUKE_YELLOW` from the set `CSS4_COLORS`. (Colour number `820`)
/// ## Representations:
/// - int tuple `(194, 190, 14)`
/// - float tuple `(0.76, 1, 0)`
/// - hex: `#C2BE0E`
pub const XKCD_PUKE_YELLOW: Color = Color(194, 190, 14);

/// Colour `(r = 116, g = 139, b = 151)`
///
/// Colour `XKCD_BLUISH_GREY` from the set `CSS4_COLORS`. (Colour number `821`)
/// ## Representations:
/// - int tuple `(116, 139, 151)`
/// - float tuple `(0.45, 1, 1)`
/// - hex: `#748B97`
pub const XKCD_BLUISH_GREY: Color = Color(116, 139, 151);

/// Colour `(r = 102, g = 95, b = 209)`
///
/// Colour `XKCD_DARK_PERIWINKLE` from the set `CSS4_COLORS`. (Colour number `822`)
/// ## Representations:
/// - int tuple `(102, 95, 209)`
/// - float tuple `(0.4, 0, 1)`
/// - hex: `#665FD1`
pub const XKCD_DARK_PERIWINKLE: Color = Color(102, 95, 209);

/// Colour `(r = 156, g = 109, b = 165)`
///
/// Colour `XKCD_DARK_LILAC` from the set `CSS4_COLORS`. (Colour number `823`)
/// ## Representations:
/// - int tuple `(156, 109, 165)`
/// - float tuple `(0.61, 0, 1)`
/// - hex: `#9C6DA5`
pub const XKCD_DARK_LILAC: Color = Color(156, 109, 165);

/// Colour `(r = 196, g = 66, b = 64)`
///
/// Colour `XKCD_REDDISH` from the set `CSS4_COLORS`. (Colour number `824`)
/// ## Representations:
/// - int tuple `(196, 66, 64)`
/// - float tuple `(0.77, 0, 0)`
/// - hex: `#C44240`
pub const XKCD_REDDISH: Color = Color(196, 66, 64);

/// Colour `(r = 162, g = 72, b = 87)`
///
/// Colour `XKCD_LIGHT_MAROON` from the set `CSS4_COLORS`. (Colour number `825`)
/// ## Representations:
/// - int tuple `(162, 72, 87)`
/// - float tuple `(0.64, 0, 0)`
/// - hex: `#A24857`
pub const XKCD_LIGHT_MAROON: Color = Color(162, 72, 87);

/// Colour `(r = 130, g = 95, b = 135)`
///
/// Colour `XKCD_DUSTY_PURPLE` from the set `CSS4_COLORS`. (Colour number `826`)
/// ## Representations:
/// - int tuple `(130, 95, 135)`
/// - float tuple `(0.51, 0, 1)`
/// - hex: `#825F87`
pub const XKCD_DUSTY_PURPLE: Color = Color(130, 95, 135);

/// Colour `(r = 201, g = 100, b = 59)`
///
/// Colour `XKCD_TERRA_COTTA` from the set `CSS4_COLORS`. (Colour number `827`)
/// ## Representations:
/// - int tuple `(201, 100, 59)`
/// - float tuple `(0.79, 0, 0)`
/// - hex: `#C9643B`
pub const XKCD_TERRA_COTTA: Color = Color(201, 100, 59);

/// Colour `(r = 144, g = 177, b = 52)`
///
/// Colour `XKCD_AVOCADO` from the set `CSS4_COLORS`. (Colour number `828`)
/// ## Representations:
/// - int tuple `(144, 177, 52)`
/// - float tuple `(0.56, 1, 0)`
/// - hex: `#90B134`
pub const XKCD_AVOCADO: Color = Color(144, 177, 52);

/// Colour `(r = 1, g = 56, b = 106)`
///
/// Colour `XKCD_MARINE_BLUE` from the set `CSS4_COLORS`. (Colour number `829`)
/// ## Representations:
/// - int tuple `(1, 56, 106)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#01386A`
pub const XKCD_MARINE_BLUE: Color = Color(1, 56, 106);

/// Colour `(r = 37, g = 163, b = 111)`
///
/// Colour `XKCD_TEAL_GREEN` from the set `CSS4_COLORS`. (Colour number `830`)
/// ## Representations:
/// - int tuple `(37, 163, 111)`
/// - float tuple `(0.15, 1, 0)`
/// - hex: `#25A36F`
pub const XKCD_TEAL_GREEN: Color = Color(37, 163, 111);

/// Colour `(r = 89, g = 101, b = 109)`
///
/// Colour `XKCD_SLATE_GREY` from the set `CSS4_COLORS`. (Colour number `831`)
/// ## Representations:
/// - int tuple `(89, 101, 109)`
/// - float tuple `(0.35, 0, 0)`
/// - hex: `#59656D`
pub const XKCD_SLATE_GREY: Color = Color(89, 101, 109);

/// Colour `(r = 117, g = 253, b = 99)`
///
/// Colour `XKCD_LIGHTER_GREEN` from the set `CSS4_COLORS`. (Colour number `832`)
/// ## Representations:
/// - int tuple `(117, 253, 99)`
/// - float tuple `(0.46, 1, 0)`
/// - hex: `#75FD63`
pub const XKCD_LIGHTER_GREEN: Color = Color(117, 253, 99);

/// Colour `(r = 33, g = 252, b = 13)`
///
/// Colour `XKCD_ELECTRIC_GREEN` from the set `CSS4_COLORS`. (Colour number `833`)
/// ## Representations:
/// - int tuple `(33, 252, 13)`
/// - float tuple `(0.13, 1, 0)`
/// - hex: `#21FC0D`
pub const XKCD_ELECTRIC_GREEN: Color = Color(33, 252, 13);

/// Colour `(r = 90, g = 134, b = 173)`
///
/// Colour `XKCD_DUSTY_BLUE` from the set `CSS4_COLORS`. (Colour number `834`)
/// ## Representations:
/// - int tuple `(90, 134, 173)`
/// - float tuple `(0.35, 1, 1)`
/// - hex: `#5A86AD`
pub const XKCD_DUSTY_BLUE: Color = Color(90, 134, 173);

/// Colour `(r = 254, g = 198, b = 21)`
///
/// Colour `XKCD_GOLDEN_YELLOW` from the set `CSS4_COLORS`. (Colour number `835`)
/// ## Representations:
/// - int tuple `(254, 198, 21)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FEC615`
pub const XKCD_GOLDEN_YELLOW: Color = Color(254, 198, 21);

/// Colour `(r = 255, g = 253, b = 1)`
///
/// Colour `XKCD_BRIGHT_YELLOW` from the set `CSS4_COLORS`. (Colour number `836`)
/// ## Representations:
/// - int tuple `(255, 253, 1)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFFD01`
pub const XKCD_BRIGHT_YELLOW: Color = Color(255, 253, 1);

/// Colour `(r = 223, g = 197, b = 254)`
///
/// Colour `XKCD_LIGHT_LAVENDER` from the set `CSS4_COLORS`. (Colour number `837`)
/// ## Representations:
/// - int tuple `(223, 197, 254)`
/// - float tuple `(0.87, 1, 1)`
/// - hex: `#DFC5FE`
pub const XKCD_LIGHT_LAVENDER: Color = Color(223, 197, 254);

/// Colour `(r = 178, g = 100, b = 0)`
///
/// Colour `XKCD_UMBER` from the set `CSS4_COLORS`. (Colour number `838`)
/// ## Representations:
/// - int tuple `(178, 100, 0)`
/// - float tuple `(0.7, 0, 0)`
/// - hex: `#B26400`
pub const XKCD_UMBER: Color = Color(178, 100, 0);

/// Colour `(r = 127, g = 94, b = 0)`
///
/// Colour `XKCD_POOP` from the set `CSS4_COLORS`. (Colour number `839`)
/// ## Representations:
/// - int tuple `(127, 94, 0)`
/// - float tuple `(0.5, 0, 0)`
/// - hex: `#7F5E00`
pub const XKCD_POOP: Color = Color(127, 94, 0);

/// Colour `(r = 222, g = 126, b = 93)`
///
/// Colour `XKCD_DARK_PEACH` from the set `CSS4_COLORS`. (Colour number `840`)
/// ## Representations:
/// - int tuple `(222, 126, 93)`
/// - float tuple `(0.87, 0, 0)`
/// - hex: `#DE7E5D`
pub const XKCD_DARK_PEACH: Color = Color(222, 126, 93);

/// Colour `(r = 4, g = 130, b = 67)`
///
/// Colour `XKCD_JUNGLE_GREEN` from the set `CSS4_COLORS`. (Colour number `841`)
/// ## Representations:
/// - int tuple `(4, 130, 67)`
/// - float tuple `(0.02, 1, 0)`
/// - hex: `#048243`
pub const XKCD_JUNGLE_GREEN: Color = Color(4, 130, 67);

/// Colour `(r = 255, g = 255, b = 212)`
///
/// Colour `XKCD_EGGSHELL` from the set `CSS4_COLORS`. (Colour number `842`)
/// ## Representations:
/// - int tuple `(255, 255, 212)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFFFD4`
pub const XKCD_EGGSHELL: Color = Color(255, 255, 212);

/// Colour `(r = 59, g = 99, b = 140)`
///
/// Colour `XKCD_DENIM` from the set `CSS4_COLORS`. (Colour number `843`)
/// ## Representations:
/// - int tuple `(59, 99, 140)`
/// - float tuple `(0.23, 0, 1)`
/// - hex: `#3B638C`
pub const XKCD_DENIM: Color = Color(59, 99, 140);

/// Colour `(r = 183, g = 148, b = 0)`
///
/// Colour `XKCD_YELLOW_BROWN` from the set `CSS4_COLORS`. (Colour number `844`)
/// ## Representations:
/// - int tuple `(183, 148, 0)`
/// - float tuple `(0.72, 1, 0)`
/// - hex: `#B79400`
pub const XKCD_YELLOW_BROWN: Color = Color(183, 148, 0);

/// Colour `(r = 132, g = 89, b = 126)`
///
/// Colour `XKCD_DULL_PURPLE` from the set `CSS4_COLORS`. (Colour number `845`)
/// ## Representations:
/// - int tuple `(132, 89, 126)`
/// - float tuple `(0.52, 0, 0)`
/// - hex: `#84597E`
pub const XKCD_DULL_PURPLE: Color = Color(132, 89, 126);

/// Colour `(r = 65, g = 25, b = 0)`
///
/// Colour `XKCD_CHOCOLATE_BROWN` from the set `CSS4_COLORS`. (Colour number `846`)
/// ## Representations:
/// - int tuple `(65, 25, 0)`
/// - float tuple `(0.25, 0, 0)`
/// - hex: `#411900`
pub const XKCD_CHOCOLATE_BROWN: Color = Color(65, 25, 0);

/// Colour `(r = 123, g = 3, b = 35)`
///
/// Colour `XKCD_WINE_RED` from the set `CSS4_COLORS`. (Colour number `847`)
/// ## Representations:
/// - int tuple `(123, 3, 35)`
/// - float tuple `(0.48, 0, 0)`
/// - hex: `#7B0323`
pub const XKCD_WINE_RED: Color = Color(123, 3, 35);

/// Colour `(r = 4, g = 217, b = 255)`
///
/// Colour `XKCD_NEON_BLUE` from the set `CSS4_COLORS`. (Colour number `848`)
/// ## Representations:
/// - int tuple `(4, 217, 255)`
/// - float tuple `(0.02, 1, 1)`
/// - hex: `#04D9FF`
pub const XKCD_NEON_BLUE: Color = Color(4, 217, 255);

/// Colour `(r = 102, g = 126, b = 44)`
///
/// Colour `XKCD_DIRTY_GREEN` from the set `CSS4_COLORS`. (Colour number `849`)
/// ## Representations:
/// - int tuple `(102, 126, 44)`
/// - float tuple `(0.4, 0, 0)`
/// - hex: `#667E2C`
pub const XKCD_DIRTY_GREEN: Color = Color(102, 126, 44);

/// Colour `(r = 251, g = 238, b = 172)`
///
/// Colour `XKCD_LIGHT_TAN` from the set `CSS4_COLORS`. (Colour number `850`)
/// ## Representations:
/// - int tuple `(251, 238, 172)`
/// - float tuple `(0.98, 1, 1)`
/// - hex: `#FBEEAC`
pub const XKCD_LIGHT_TAN: Color = Color(251, 238, 172);

/// Colour `(r = 215, g = 255, b = 254)`
///
/// Colour `XKCD_ICE_BLUE` from the set `CSS4_COLORS`. (Colour number `851`)
/// ## Representations:
/// - int tuple `(215, 255, 254)`
/// - float tuple `(0.84, 1, 1)`
/// - hex: `#D7FFFE`
pub const XKCD_ICE_BLUE: Color = Color(215, 255, 254);

/// Colour `(r = 78, g = 116, b = 150)`
///
/// Colour `XKCD_CADET_BLUE` from the set `CSS4_COLORS`. (Colour number `852`)
/// ## Representations:
/// - int tuple `(78, 116, 150)`
/// - float tuple `(0.31, 0, 1)`
/// - hex: `#4E7496`
pub const XKCD_CADET_BLUE: Color = Color(78, 116, 150);

/// Colour `(r = 135, g = 76, b = 98)`
///
/// Colour `XKCD_DARK_MAUVE` from the set `CSS4_COLORS`. (Colour number `853`)
/// ## Representations:
/// - int tuple `(135, 76, 98)`
/// - float tuple `(0.53, 0, 0)`
/// - hex: `#874C62`
pub const XKCD_DARK_MAUVE: Color = Color(135, 76, 98);

/// Colour `(r = 213, g = 255, b = 255)`
///
/// Colour `XKCD_VERY_LIGHT_BLUE` from the set `CSS4_COLORS`. (Colour number `854`)
/// ## Representations:
/// - int tuple `(213, 255, 255)`
/// - float tuple `(0.84, 1, 1)`
/// - hex: `#D5FFFF`
pub const XKCD_VERY_LIGHT_BLUE: Color = Color(213, 255, 255);

/// Colour `(r = 130, g = 109, b = 140)`
///
/// Colour `XKCD_GREY_PURPLE` from the set `CSS4_COLORS`. (Colour number `855`)
/// ## Representations:
/// - int tuple `(130, 109, 140)`
/// - float tuple `(0.51, 0, 1)`
/// - hex: `#826D8C`
pub const XKCD_GREY_PURPLE: Color = Color(130, 109, 140);

/// Colour `(r = 255, g = 186, b = 205)`
///
/// Colour `XKCD_PASTEL_PINK` from the set `CSS4_COLORS`. (Colour number `856`)
/// ## Representations:
/// - int tuple `(255, 186, 205)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFBACD`
pub const XKCD_PASTEL_PINK: Color = Color(255, 186, 205);

/// Colour `(r = 209, g = 255, b = 189)`
///
/// Colour `XKCD_VERY_LIGHT_GREEN` from the set `CSS4_COLORS`. (Colour number `857`)
/// ## Representations:
/// - int tuple `(209, 255, 189)`
/// - float tuple `(0.82, 1, 1)`
/// - hex: `#D1FFBD`
pub const XKCD_VERY_LIGHT_GREEN: Color = Color(209, 255, 189);

/// Colour `(r = 68, g = 142, b = 228)`
///
/// Colour `XKCD_DARK_SKY_BLUE` from the set `CSS4_COLORS`. (Colour number `858`)
/// ## Representations:
/// - int tuple `(68, 142, 228)`
/// - float tuple `(0.27, 1, 1)`
/// - hex: `#448EE4`
pub const XKCD_DARK_SKY_BLUE: Color = Color(68, 142, 228);

/// Colour `(r = 5, g = 71, b = 42)`
///
/// Colour `XKCD_EVERGREEN` from the set `CSS4_COLORS`. (Colour number `859`)
/// ## Representations:
/// - int tuple `(5, 71, 42)`
/// - float tuple `(0.02, 0, 0)`
/// - hex: `#05472A`
pub const XKCD_EVERGREEN: Color = Color(5, 71, 42);

/// Colour `(r = 213, g = 134, b = 157)`
///
/// Colour `XKCD_DULL_PINK` from the set `CSS4_COLORS`. (Colour number `860`)
/// ## Representations:
/// - int tuple `(213, 134, 157)`
/// - float tuple `(0.84, 1, 1)`
/// - hex: `#D5869D`
pub const XKCD_DULL_PINK: Color = Color(213, 134, 157);

/// Colour `(r = 61, g = 7, b = 52)`
///
/// Colour `XKCD_AUBERGINE` from the set `CSS4_COLORS`. (Colour number `861`)
/// ## Representations:
/// - int tuple `(61, 7, 52)`
/// - float tuple `(0.24, 0, 0)`
/// - hex: `#3D0734`
pub const XKCD_AUBERGINE: Color = Color(61, 7, 52);

/// Colour `(r = 74, g = 1, b = 0)`
///
/// Colour `XKCD_MAHOGANY` from the set `CSS4_COLORS`. (Colour number `862`)
/// ## Representations:
/// - int tuple `(74, 1, 0)`
/// - float tuple `(0.29, 0, 0)`
/// - hex: `#4A0100`
pub const XKCD_MAHOGANY: Color = Color(74, 1, 0);

/// Colour `(r = 248, g = 72, b = 28)`
///
/// Colour `XKCD_REDDISH_ORANGE` from the set `CSS4_COLORS`. (Colour number `863`)
/// ## Representations:
/// - int tuple `(248, 72, 28)`
/// - float tuple `(0.97, 0, 0)`
/// - hex: `#F8481C`
pub const XKCD_REDDISH_ORANGE: Color = Color(248, 72, 28);

/// Colour `(r = 2, g = 89, b = 15)`
///
/// Colour `XKCD_DEEP_GREEN` from the set `CSS4_COLORS`. (Colour number `864`)
/// ## Representations:
/// - int tuple `(2, 89, 15)`
/// - float tuple `(0.01, 0, 0)`
/// - hex: `#02590F`
pub const XKCD_DEEP_GREEN: Color = Color(2, 89, 15);

/// Colour `(r = 137, g = 162, b = 3)`
///
/// Colour `XKCD_VOMIT_GREEN` from the set `CSS4_COLORS`. (Colour number `865`)
/// ## Representations:
/// - int tuple `(137, 162, 3)`
/// - float tuple `(0.54, 1, 0)`
/// - hex: `#89A203`
pub const XKCD_VOMIT_GREEN: Color = Color(137, 162, 3);

/// Colour `(r = 224, g = 63, b = 216)`
///
/// Colour `XKCD_PURPLE_PINK` from the set `CSS4_COLORS`. (Colour number `866`)
/// ## Representations:
/// - int tuple `(224, 63, 216)`
/// - float tuple `(0.88, 0, 1)`
/// - hex: `#E03FD8`
pub const XKCD_PURPLE_PINK: Color = Color(224, 63, 216);

/// Colour `(r = 213, g = 138, b = 148)`
///
/// Colour `XKCD_DUSTY_PINK` from the set `CSS4_COLORS`. (Colour number `867`)
/// ## Representations:
/// - int tuple `(213, 138, 148)`
/// - float tuple `(0.84, 1, 1)`
/// - hex: `#D58A94`
pub const XKCD_DUSTY_PINK: Color = Color(213, 138, 148);

/// Colour `(r = 123, g = 178, b = 116)`
///
/// Colour `XKCD_FADED_GREEN` from the set `CSS4_COLORS`. (Colour number `868`)
/// ## Representations:
/// - int tuple `(123, 178, 116)`
/// - float tuple `(0.48, 1, 0)`
/// - hex: `#7BB274`
pub const XKCD_FADED_GREEN: Color = Color(123, 178, 116);

/// Colour `(r = 82, g = 101, b = 37)`
///
/// Colour `XKCD_CAMO_GREEN` from the set `CSS4_COLORS`. (Colour number `869`)
/// ## Representations:
/// - int tuple `(82, 101, 37)`
/// - float tuple `(0.32, 0, 0)`
/// - hex: `#526525`
pub const XKCD_CAMO_GREEN: Color = Color(82, 101, 37);

/// Colour `(r = 201, g = 76, b = 190)`
///
/// Colour `XKCD_PINKY_PURPLE` from the set `CSS4_COLORS`. (Colour number `870`)
/// ## Representations:
/// - int tuple `(201, 76, 190)`
/// - float tuple `(0.79, 0, 1)`
/// - hex: `#C94CBE`
pub const XKCD_PINKY_PURPLE: Color = Color(201, 76, 190);

/// Colour `(r = 219, g = 75, b = 218)`
///
/// Colour `XKCD_PINK_PURPLE` from the set `CSS4_COLORS`. (Colour number `871`)
/// ## Representations:
/// - int tuple `(219, 75, 218)`
/// - float tuple `(0.86, 0, 1)`
/// - hex: `#DB4BDA`
pub const XKCD_PINK_PURPLE: Color = Color(219, 75, 218);

/// Colour `(r = 158, g = 54, b = 35)`
///
/// Colour `XKCD_BROWNISH_RED` from the set `CSS4_COLORS`. (Colour number `872`)
/// ## Representations:
/// - int tuple `(158, 54, 35)`
/// - float tuple `(0.62, 0, 0)`
/// - hex: `#9E3623`
pub const XKCD_BROWNISH_RED: Color = Color(158, 54, 35);

/// Colour `(r = 181, g = 72, b = 93)`
///
/// Colour `XKCD_DARK_ROSE` from the set `CSS4_COLORS`. (Colour number `873`)
/// ## Representations:
/// - int tuple `(181, 72, 93)`
/// - float tuple `(0.71, 0, 0)`
/// - hex: `#B5485D`
pub const XKCD_DARK_ROSE: Color = Color(181, 72, 93);

/// Colour `(r = 115, g = 92, b = 18)`
///
/// Colour `XKCD_MUD` from the set `CSS4_COLORS`. (Colour number `874`)
/// ## Representations:
/// - int tuple `(115, 92, 18)`
/// - float tuple `(0.45, 0, 0)`
/// - hex: `#735C12`
pub const XKCD_MUD: Color = Color(115, 92, 18);

/// Colour `(r = 156, g = 109, b = 87)`
///
/// Colour `XKCD_BROWNISH` from the set `CSS4_COLORS`. (Colour number `875`)
/// ## Representations:
/// - int tuple `(156, 109, 87)`
/// - float tuple `(0.61, 0, 0)`
/// - hex: `#9C6D57`
pub const XKCD_BROWNISH: Color = Color(156, 109, 87);

/// Colour `(r = 2, g = 143, b = 30)`
///
/// Colour `XKCD_EMERALD_GREEN` from the set `CSS4_COLORS`. (Colour number `876`)
/// ## Representations:
/// - int tuple `(2, 143, 30)`
/// - float tuple `(0.01, 1, 0)`
/// - hex: `#028F1E`
pub const XKCD_EMERALD_GREEN: Color = Color(2, 143, 30);

/// Colour `(r = 177, g = 145, b = 110)`
///
/// Colour `XKCD_PALE_BROWN` from the set `CSS4_COLORS`. (Colour number `877`)
/// ## Representations:
/// - int tuple `(177, 145, 110)`
/// - float tuple `(0.69, 1, 0)`
/// - hex: `#B1916E`
pub const XKCD_PALE_BROWN: Color = Color(177, 145, 110);

/// Colour `(r = 73, g = 117, b = 156)`
///
/// Colour `XKCD_DULL_BLUE` from the set `CSS4_COLORS`. (Colour number `878`)
/// ## Representations:
/// - int tuple `(73, 117, 156)`
/// - float tuple `(0.29, 0, 1)`
/// - hex: `#49759C`
pub const XKCD_DULL_BLUE: Color = Color(73, 117, 156);

/// Colour `(r = 160, g = 69, b = 14)`
///
/// Colour `XKCD_BURNT_UMBER` from the set `CSS4_COLORS`. (Colour number `879`)
/// ## Representations:
/// - int tuple `(160, 69, 14)`
/// - float tuple `(0.63, 0, 0)`
/// - hex: `#A0450E`
pub const XKCD_BURNT_UMBER: Color = Color(160, 69, 14);

/// Colour `(r = 57, g = 173, b = 72)`
///
/// Colour `XKCD_MEDIUM_GREEN` from the set `CSS4_COLORS`. (Colour number `880`)
/// ## Representations:
/// - int tuple `(57, 173, 72)`
/// - float tuple `(0.22, 1, 0)`
/// - hex: `#39AD48`
pub const XKCD_MEDIUM_GREEN: Color = Color(57, 173, 72);

/// Colour `(r = 182, g = 106, b = 80)`
///
/// Colour `XKCD_CLAY` from the set `CSS4_COLORS`. (Colour number `881`)
/// ## Representations:
/// - int tuple `(182, 106, 80)`
/// - float tuple `(0.71, 0, 0)`
/// - hex: `#B66A50`
pub const XKCD_CLAY: Color = Color(182, 106, 80);

/// Colour `(r = 140, g = 255, b = 219)`
///
/// Colour `XKCD_LIGHT_AQUA` from the set `CSS4_COLORS`. (Colour number `882`)
/// ## Representations:
/// - int tuple `(140, 255, 219)`
/// - float tuple `(0.55, 1, 1)`
/// - hex: `#8CFFDB`
pub const XKCD_LIGHT_AQUA: Color = Color(140, 255, 219);

/// Colour `(r = 164, g = 190, b = 92)`
///
/// Colour `XKCD_LIGHT_OLIVE_GREEN` from the set `CSS4_COLORS`. (Colour number `883`)
/// ## Representations:
/// - int tuple `(164, 190, 92)`
/// - float tuple `(0.64, 1, 0)`
/// - hex: `#A4BE5C`
pub const XKCD_LIGHT_OLIVE_GREEN: Color = Color(164, 190, 92);

/// Colour `(r = 203, g = 119, b = 35)`
///
/// Colour `XKCD_BROWNISH_ORANGE` from the set `CSS4_COLORS`. (Colour number `884`)
/// ## Representations:
/// - int tuple `(203, 119, 35)`
/// - float tuple `(0.8, 0, 0)`
/// - hex: `#CB7723`
pub const XKCD_BROWNISH_ORANGE: Color = Color(203, 119, 35);

/// Colour `(r = 5, g = 105, b = 107)`
///
/// Colour `XKCD_DARK_AQUA` from the set `CSS4_COLORS`. (Colour number `885`)
/// ## Representations:
/// - int tuple `(5, 105, 107)`
/// - float tuple `(0.02, 0, 0)`
/// - hex: `#05696B`
pub const XKCD_DARK_AQUA: Color = Color(5, 105, 107);

/// Colour `(r = 206, g = 93, b = 174)`
///
/// Colour `XKCD_PURPLISH_PINK` from the set `CSS4_COLORS`. (Colour number `886`)
/// ## Representations:
/// - int tuple `(206, 93, 174)`
/// - float tuple `(0.81, 0, 1)`
/// - hex: `#CE5DAE`
pub const XKCD_PURPLISH_PINK: Color = Color(206, 93, 174);

/// Colour `(r = 200, g = 90, b = 83)`
///
/// Colour `XKCD_DARK_SALMON` from the set `CSS4_COLORS`. (Colour number `887`)
/// ## Representations:
/// - int tuple `(200, 90, 83)`
/// - float tuple `(0.78, 0, 0)`
/// - hex: `#C85A53`
pub const XKCD_DARK_SALMON: Color = Color(200, 90, 83);

/// Colour `(r = 150, g = 174, b = 141)`
///
/// Colour `XKCD_GREENISH_GREY` from the set `CSS4_COLORS`. (Colour number `888`)
/// ## Representations:
/// - int tuple `(150, 174, 141)`
/// - float tuple `(0.59, 1, 1)`
/// - hex: `#96AE8D`
pub const XKCD_GREENISH_GREY: Color = Color(150, 174, 141);

/// Colour `(r = 31, g = 167, b = 116)`
///
/// Colour `XKCD_JADE` from the set `CSS4_COLORS`. (Colour number `889`)
/// ## Representations:
/// - int tuple `(31, 167, 116)`
/// - float tuple `(0.12, 1, 0)`
/// - hex: `#1FA774`
pub const XKCD_JADE: Color = Color(31, 167, 116);

/// Colour `(r = 122, g = 151, b = 3)`
///
/// Colour `XKCD_UGLY_GREEN` from the set `CSS4_COLORS`. (Colour number `890`)
/// ## Representations:
/// - int tuple `(122, 151, 3)`
/// - float tuple `(0.48, 1, 0)`
/// - hex: `#7A9703`
pub const XKCD_UGLY_GREEN: Color = Color(122, 151, 3);

/// Colour `(r = 172, g = 147, b = 98)`
///
/// Colour `XKCD_DARK_BEIGE` from the set `CSS4_COLORS`. (Colour number `891`)
/// ## Representations:
/// - int tuple `(172, 147, 98)`
/// - float tuple `(0.67, 1, 0)`
/// - hex: `#AC9362`
pub const XKCD_DARK_BEIGE: Color = Color(172, 147, 98);

/// Colour `(r = 1, g = 160, b = 73)`
///
/// Colour `XKCD_EMERALD` from the set `CSS4_COLORS`. (Colour number `892`)
/// ## Representations:
/// - int tuple `(1, 160, 73)`
/// - float tuple `(0.0, 1, 0)`
/// - hex: `#01A049`
pub const XKCD_EMERALD: Color = Color(1, 160, 73);

/// Colour `(r = 217, g = 84, b = 77)`
///
/// Colour `XKCD_PALE_RED` from the set `CSS4_COLORS`. (Colour number `893`)
/// ## Representations:
/// - int tuple `(217, 84, 77)`
/// - float tuple `(0.85, 0, 0)`
/// - hex: `#D9544D`
pub const XKCD_PALE_RED: Color = Color(217, 84, 77);

/// Colour `(r = 250, g = 95, b = 247)`
///
/// Colour `XKCD_LIGHT_MAGENTA` from the set `CSS4_COLORS`. (Colour number `894`)
/// ## Representations:
/// - int tuple `(250, 95, 247)`
/// - float tuple `(0.98, 0, 1)`
/// - hex: `#FA5FF7`
pub const XKCD_LIGHT_MAGENTA: Color = Color(250, 95, 247);

/// Colour `(r = 130, g = 202, b = 252)`
///
/// Colour `XKCD_SKY` from the set `CSS4_COLORS`. (Colour number `895`)
/// ## Representations:
/// - int tuple `(130, 202, 252)`
/// - float tuple `(0.51, 1, 1)`
/// - hex: `#82CAFC`
pub const XKCD_SKY: Color = Color(130, 202, 252);

/// Colour `(r = 172, g = 255, b = 252)`
///
/// Colour `XKCD_LIGHT_CYAN` from the set `CSS4_COLORS`. (Colour number `896`)
/// ## Representations:
/// - int tuple `(172, 255, 252)`
/// - float tuple `(0.67, 1, 1)`
/// - hex: `#ACFFFC`
pub const XKCD_LIGHT_CYAN: Color = Color(172, 255, 252);

/// Colour `(r = 252, g = 176, b = 1)`
///
/// Colour `XKCD_YELLOW_ORANGE` from the set `CSS4_COLORS`. (Colour number `897`)
/// ## Representations:
/// - int tuple `(252, 176, 1)`
/// - float tuple `(0.99, 1, 0)`
/// - hex: `#FCB001`
pub const XKCD_YELLOW_ORANGE: Color = Color(252, 176, 1);

/// Colour `(r = 145, g = 9, b = 81)`
///
/// Colour `XKCD_REDDISH_PURPLE` from the set `CSS4_COLORS`. (Colour number `898`)
/// ## Representations:
/// - int tuple `(145, 9, 81)`
/// - float tuple `(0.57, 0, 0)`
/// - hex: `#910951`
pub const XKCD_REDDISH_PURPLE: Color = Color(145, 9, 81);

/// Colour `(r = 254, g = 44, b = 84)`
///
/// Colour `XKCD_REDDISH_PINK` from the set `CSS4_COLORS`. (Colour number `899`)
/// ## Representations:
/// - int tuple `(254, 44, 84)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FE2C54`
pub const XKCD_REDDISH_PINK: Color = Color(254, 44, 84);

/// Colour `(r = 200, g = 117, b = 196)`
///
/// Colour `XKCD_ORCHID` from the set `CSS4_COLORS`. (Colour number `900`)
/// ## Representations:
/// - int tuple `(200, 117, 196)`
/// - float tuple `(0.78, 0, 1)`
/// - hex: `#C875C4`
pub const XKCD_ORCHID: Color = Color(200, 117, 196);

/// Colour `(r = 205, g = 197, b = 10)`
///
/// Colour `XKCD_DIRTY_YELLOW` from the set `CSS4_COLORS`. (Colour number `901`)
/// ## Representations:
/// - int tuple `(205, 197, 10)`
/// - float tuple `(0.8, 1, 0)`
/// - hex: `#CDC50A`
pub const XKCD_DIRTY_YELLOW: Color = Color(205, 197, 10);

/// Colour `(r = 253, g = 65, b = 30)`
///
/// Colour `XKCD_ORANGE_RED` from the set `CSS4_COLORS`. (Colour number `902`)
/// ## Representations:
/// - int tuple `(253, 65, 30)`
/// - float tuple `(0.99, 0, 0)`
/// - hex: `#FD411E`
pub const XKCD_ORANGE_RED: Color = Color(253, 65, 30);

/// Colour `(r = 154, g = 2, b = 0)`
///
/// Colour `XKCD_DEEP_RED` from the set `CSS4_COLORS`. (Colour number `903`)
/// ## Representations:
/// - int tuple `(154, 2, 0)`
/// - float tuple `(0.6, 0, 0)`
/// - hex: `#9A0200`
pub const XKCD_DEEP_RED: Color = Color(154, 2, 0);

/// Colour `(r = 190, g = 100, b = 0)`
///
/// Colour `XKCD_ORANGE_BROWN` from the set `CSS4_COLORS`. (Colour number `904`)
/// ## Representations:
/// - int tuple `(190, 100, 0)`
/// - float tuple `(0.75, 0, 0)`
/// - hex: `#BE6400`
pub const XKCD_ORANGE_BROWN: Color = Color(190, 100, 0);

/// Colour `(r = 3, g = 10, b = 167)`
///
/// Colour `XKCD_COBALT_BLUE` from the set `CSS4_COLORS`. (Colour number `905`)
/// ## Representations:
/// - int tuple `(3, 10, 167)`
/// - float tuple `(0.01, 0, 1)`
/// - hex: `#030AA7`
pub const XKCD_COBALT_BLUE: Color = Color(3, 10, 167);

/// Colour `(r = 254, g = 1, b = 154)`
///
/// Colour `XKCD_NEON_PINK` from the set `CSS4_COLORS`. (Colour number `906`)
/// ## Representations:
/// - int tuple `(254, 1, 154)`
/// - float tuple `(1.0, 0, 1)`
/// - hex: `#FE019A`
pub const XKCD_NEON_PINK: Color = Color(254, 1, 154);

/// Colour `(r = 247, g = 135, b = 154)`
///
/// Colour `XKCD_ROSE_PINK` from the set `CSS4_COLORS`. (Colour number `907`)
/// ## Representations:
/// - int tuple `(247, 135, 154)`
/// - float tuple `(0.97, 1, 1)`
/// - hex: `#F7879A`
pub const XKCD_ROSE_PINK: Color = Color(247, 135, 154);

/// Colour `(r = 136, g = 113, b = 145)`
///
/// Colour `XKCD_GREYISH_PURPLE` from the set `CSS4_COLORS`. (Colour number `908`)
/// ## Representations:
/// - int tuple `(136, 113, 145)`
/// - float tuple `(0.53, 0, 1)`
/// - hex: `#887191`
pub const XKCD_GREYISH_PURPLE: Color = Color(136, 113, 145);

/// Colour `(r = 176, g = 1, b = 73)`
///
/// Colour `XKCD_RASPBERRY` from the set `CSS4_COLORS`. (Colour number `909`)
/// ## Representations:
/// - int tuple `(176, 1, 73)`
/// - float tuple `(0.69, 0, 0)`
/// - hex: `#B00149`
pub const XKCD_RASPBERRY: Color = Color(176, 1, 73);

/// Colour `(r = 18, g = 225, b = 147)`
///
/// Colour `XKCD_AQUA_GREEN` from the set `CSS4_COLORS`. (Colour number `910`)
/// ## Representations:
/// - int tuple `(18, 225, 147)`
/// - float tuple `(0.07, 1, 1)`
/// - hex: `#12E193`
pub const XKCD_AQUA_GREEN: Color = Color(18, 225, 147);

/// Colour `(r = 254, g = 123, b = 124)`
///
/// Colour `XKCD_SALMON_PINK` from the set `CSS4_COLORS`. (Colour number `911`)
/// ## Representations:
/// - int tuple `(254, 123, 124)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FE7B7C`
pub const XKCD_SALMON_PINK: Color = Color(254, 123, 124);

/// Colour `(r = 255, g = 148, b = 8)`
///
/// Colour `XKCD_TANGERINE` from the set `CSS4_COLORS`. (Colour number `912`)
/// ## Representations:
/// - int tuple `(255, 148, 8)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FF9408`
pub const XKCD_TANGERINE: Color = Color(255, 148, 8);

/// Colour `(r = 106, g = 110, b = 9)`
///
/// Colour `XKCD_BROWNISH_GREEN` from the set `CSS4_COLORS`. (Colour number `913`)
/// ## Representations:
/// - int tuple `(106, 110, 9)`
/// - float tuple `(0.42, 0, 0)`
/// - hex: `#6A6E09`
pub const XKCD_BROWNISH_GREEN: Color = Color(106, 110, 9);

/// Colour `(r = 139, g = 46, b = 22)`
///
/// Colour `XKCD_RED_BROWN` from the set `CSS4_COLORS`. (Colour number `914`)
/// ## Representations:
/// - int tuple `(139, 46, 22)`
/// - float tuple `(0.55, 0, 0)`
/// - hex: `#8B2E16`
pub const XKCD_RED_BROWN: Color = Color(139, 46, 22);

/// Colour `(r = 105, g = 97, b = 18)`
///
/// Colour `XKCD_GREENISH_BROWN` from the set `CSS4_COLORS`. (Colour number `915`)
/// ## Representations:
/// - int tuple `(105, 97, 18)`
/// - float tuple `(0.41, 0, 0)`
/// - hex: `#696112`
pub const XKCD_GREENISH_BROWN: Color = Color(105, 97, 18);

/// Colour `(r = 225, g = 119, b = 1)`
///
/// Colour `XKCD_PUMPKIN` from the set `CSS4_COLORS`. (Colour number `916`)
/// ## Representations:
/// - int tuple `(225, 119, 1)`
/// - float tuple `(0.88, 0, 0)`
/// - hex: `#E17701`
pub const XKCD_PUMPKIN: Color = Color(225, 119, 1);

/// Colour `(r = 10, g = 72, b = 30)`
///
/// Colour `XKCD_PINE_GREEN` from the set `CSS4_COLORS`. (Colour number `917`)
/// ## Representations:
/// - int tuple `(10, 72, 30)`
/// - float tuple `(0.04, 0, 0)`
/// - hex: `#0A481E`
pub const XKCD_PINE_GREEN: Color = Color(10, 72, 30);

/// Colour `(r = 52, g = 56, b = 55)`
///
/// Colour `XKCD_CHARCOAL` from the set `CSS4_COLORS`. (Colour number `918`)
/// ## Representations:
/// - int tuple `(52, 56, 55)`
/// - float tuple `(0.2, 0, 0)`
/// - hex: `#343837`
pub const XKCD_CHARCOAL: Color = Color(52, 56, 55);

/// Colour `(r = 255, g = 183, b = 206)`
///
/// Colour `XKCD_BABY_PINK` from the set `CSS4_COLORS`. (Colour number `919`)
/// ## Representations:
/// - int tuple `(255, 183, 206)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFB7CE`
pub const XKCD_BABY_PINK: Color = Color(255, 183, 206);

/// Colour `(r = 106, g = 121, b = 247)`
///
/// Colour `XKCD_CORNFLOWER` from the set `CSS4_COLORS`. (Colour number `920`)
/// ## Representations:
/// - int tuple `(106, 121, 247)`
/// - float tuple `(0.42, 0, 1)`
/// - hex: `#6A79F7`
pub const XKCD_CORNFLOWER: Color = Color(106, 121, 247);

/// Colour `(r = 93, g = 6, b = 233)`
///
/// Colour `XKCD_BLUE_VIOLET` from the set `CSS4_COLORS`. (Colour number `921`)
/// ## Representations:
/// - int tuple `(93, 6, 233)`
/// - float tuple `(0.36, 0, 1)`
/// - hex: `#5D06E9`
pub const XKCD_BLUE_VIOLET: Color = Color(93, 6, 233);

/// Colour `(r = 61, g = 28, b = 2)`
///
/// Colour `XKCD_CHOCOLATE` from the set `CSS4_COLORS`. (Colour number `922`)
/// ## Representations:
/// - int tuple `(61, 28, 2)`
/// - float tuple `(0.24, 0, 0)`
/// - hex: `#3D1C02`
pub const XKCD_CHOCOLATE: Color = Color(61, 28, 2);

/// Colour `(r = 130, g = 166, b = 125)`
///
/// Colour `XKCD_GREYISH_GREEN` from the set `CSS4_COLORS`. (Colour number `923`)
/// ## Representations:
/// - int tuple `(130, 166, 125)`
/// - float tuple `(0.51, 1, 0)`
/// - hex: `#82A67D`
pub const XKCD_GREYISH_GREEN: Color = Color(130, 166, 125);

/// Colour `(r = 190, g = 1, b = 25)`
///
/// Colour `XKCD_SCARLET` from the set `CSS4_COLORS`. (Colour number `924`)
/// ## Representations:
/// - int tuple `(190, 1, 25)`
/// - float tuple `(0.75, 0, 0)`
/// - hex: `#BE0119`
pub const XKCD_SCARLET: Color = Color(190, 1, 25);

/// Colour `(r = 201, g = 255, b = 39)`
///
/// Colour `XKCD_GREEN_YELLOW` from the set `CSS4_COLORS`. (Colour number `925`)
/// ## Representations:
/// - int tuple `(201, 255, 39)`
/// - float tuple `(0.79, 1, 0)`
/// - hex: `#C9FF27`
pub const XKCD_GREEN_YELLOW: Color = Color(201, 255, 39);

/// Colour `(r = 55, g = 62, b = 2)`
///
/// Colour `XKCD_DARK_OLIVE` from the set `CSS4_COLORS`. (Colour number `926`)
/// ## Representations:
/// - int tuple `(55, 62, 2)`
/// - float tuple `(0.22, 0, 0)`
/// - hex: `#373E02`
pub const XKCD_DARK_OLIVE: Color = Color(55, 62, 2);

/// Colour `(r = 169, g = 86, b = 30)`
///
/// Colour `XKCD_SIENNA` from the set `CSS4_COLORS`. (Colour number `927`)
/// ## Representations:
/// - int tuple `(169, 86, 30)`
/// - float tuple `(0.66, 0, 0)`
/// - hex: `#A9561E`
pub const XKCD_SIENNA: Color = Color(169, 86, 30);

/// Colour `(r = 202, g = 160, b = 255)`
///
/// Colour `XKCD_PASTEL_PURPLE` from the set `CSS4_COLORS`. (Colour number `928`)
/// ## Representations:
/// - int tuple `(202, 160, 255)`
/// - float tuple `(0.79, 1, 1)`
/// - hex: `#CAA0FF`
pub const XKCD_PASTEL_PURPLE: Color = Color(202, 160, 255);

/// Colour `(r = 202, g = 102, b = 65)`
///
/// Colour `XKCD_TERRACOTTA` from the set `CSS4_COLORS`. (Colour number `929`)
/// ## Representations:
/// - int tuple `(202, 102, 65)`
/// - float tuple `(0.79, 0, 0)`
/// - hex: `#CA6641`
pub const XKCD_TERRACOTTA: Color = Color(202, 102, 65);

/// Colour `(r = 2, g = 216, b = 233)`
///
/// Colour `XKCD_AQUA_BLUE` from the set `CSS4_COLORS`. (Colour number `930`)
/// ## Representations:
/// - int tuple `(2, 216, 233)`
/// - float tuple `(0.01, 1, 1)`
/// - hex: `#02D8E9`
pub const XKCD_AQUA_BLUE: Color = Color(2, 216, 233);

/// Colour `(r = 136, g = 179, b = 120)`
///
/// Colour `XKCD_SAGE_GREEN` from the set `CSS4_COLORS`. (Colour number `931`)
/// ## Representations:
/// - int tuple `(136, 179, 120)`
/// - float tuple `(0.53, 1, 0)`
/// - hex: `#88B378`
pub const XKCD_SAGE_GREEN: Color = Color(136, 179, 120);

/// Colour `(r = 152, g = 0, b = 2)`
///
/// Colour `XKCD_BLOOD_RED` from the set `CSS4_COLORS`. (Colour number `932`)
/// ## Representations:
/// - int tuple `(152, 0, 2)`
/// - float tuple `(0.6, 0, 0)`
/// - hex: `#980002`
pub const XKCD_BLOOD_RED: Color = Color(152, 0, 2);

/// Colour `(r = 203, g = 1, b = 98)`
///
/// Colour `XKCD_DEEP_PINK` from the set `CSS4_COLORS`. (Colour number `933`)
/// ## Representations:
/// - int tuple `(203, 1, 98)`
/// - float tuple `(0.8, 0, 0)`
/// - hex: `#CB0162`
pub const XKCD_DEEP_PINK: Color = Color(203, 1, 98);

/// Colour `(r = 92, g = 172, b = 45)`
///
/// Colour `XKCD_GRASS` from the set `CSS4_COLORS`. (Colour number `934`)
/// ## Representations:
/// - int tuple `(92, 172, 45)`
/// - float tuple `(0.36, 1, 0)`
/// - hex: `#5CAC2D`
pub const XKCD_GRASS: Color = Color(92, 172, 45);

/// Colour `(r = 118, g = 153, b = 88)`
///
/// Colour `XKCD_MOSS` from the set `CSS4_COLORS`. (Colour number `935`)
/// ## Representations:
/// - int tuple `(118, 153, 88)`
/// - float tuple `(0.46, 1, 0)`
/// - hex: `#769958`
pub const XKCD_MOSS: Color = Color(118, 153, 88);

/// Colour `(r = 162, g = 191, b = 254)`
///
/// Colour `XKCD_PASTEL_BLUE` from the set `CSS4_COLORS`. (Colour number `936`)
/// ## Representations:
/// - int tuple `(162, 191, 254)`
/// - float tuple `(0.64, 1, 1)`
/// - hex: `#A2BFFE`
pub const XKCD_PASTEL_BLUE: Color = Color(162, 191, 254);

/// Colour `(r = 16, g = 166, b = 116)`
///
/// Colour `XKCD_BLUISH_GREEN` from the set `CSS4_COLORS`. (Colour number `937`)
/// ## Representations:
/// - int tuple `(16, 166, 116)`
/// - float tuple `(0.06, 1, 0)`
/// - hex: `#10A674`
pub const XKCD_BLUISH_GREEN: Color = Color(16, 166, 116);

/// Colour `(r = 6, g = 180, b = 139)`
///
/// Colour `XKCD_GREEN_BLUE` from the set `CSS4_COLORS`. (Colour number `938`)
/// ## Representations:
/// - int tuple `(6, 180, 139)`
/// - float tuple `(0.02, 1, 1)`
/// - hex: `#06B48B`
pub const XKCD_GREEN_BLUE: Color = Color(6, 180, 139);

/// Colour `(r = 175, g = 136, b = 74)`
///
/// Colour `XKCD_DARK_TAN` from the set `CSS4_COLORS`. (Colour number `939`)
/// ## Representations:
/// - int tuple `(175, 136, 74)`
/// - float tuple `(0.69, 1, 0)`
/// - hex: `#AF884A`
pub const XKCD_DARK_TAN: Color = Color(175, 136, 74);

/// Colour `(r = 11, g = 139, b = 135)`
///
/// Colour `XKCD_GREENISH_BLUE` from the set `CSS4_COLORS`. (Colour number `940`)
/// ## Representations:
/// - int tuple `(11, 139, 135)`
/// - float tuple `(0.04, 1, 1)`
/// - hex: `#0B8B87`
pub const XKCD_GREENISH_BLUE: Color = Color(11, 139, 135);

/// Colour `(r = 255, g = 167, b = 86)`
///
/// Colour `XKCD_PALE_ORANGE` from the set `CSS4_COLORS`. (Colour number `941`)
/// ## Representations:
/// - int tuple `(255, 167, 86)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFA756`
pub const XKCD_PALE_ORANGE: Color = Color(255, 167, 86);

/// Colour `(r = 162, g = 164, b = 21)`
///
/// Colour `XKCD_VOMIT` from the set `CSS4_COLORS`. (Colour number `942`)
/// ## Representations:
/// - int tuple `(162, 164, 21)`
/// - float tuple `(0.64, 1, 0)`
/// - hex: `#A2A415`
pub const XKCD_VOMIT: Color = Color(162, 164, 21);

/// Colour `(r = 21, g = 68, b = 6)`
///
/// Colour `XKCD_FORREST_GREEN` from the set `CSS4_COLORS`. (Colour number `943`)
/// ## Representations:
/// - int tuple `(21, 68, 6)`
/// - float tuple `(0.08, 0, 0)`
/// - hex: `#154406`
pub const XKCD_FORREST_GREEN: Color = Color(21, 68, 6);

/// Colour `(r = 133, g = 103, b = 152)`
///
/// Colour `XKCD_DARK_LAVENDER` from the set `CSS4_COLORS`. (Colour number `944`)
/// ## Representations:
/// - int tuple `(133, 103, 152)`
/// - float tuple `(0.52, 0, 1)`
/// - hex: `#856798`
pub const XKCD_DARK_LAVENDER: Color = Color(133, 103, 152);

/// Colour `(r = 52, g = 1, b = 63)`
///
/// Colour `XKCD_DARK_VIOLET` from the set `CSS4_COLORS`. (Colour number `945`)
/// ## Representations:
/// - int tuple `(52, 1, 63)`
/// - float tuple `(0.2, 0, 0)`
/// - hex: `#34013F`
pub const XKCD_DARK_VIOLET: Color = Color(52, 1, 63);

/// Colour `(r = 99, g = 45, b = 233)`
///
/// Colour `XKCD_PURPLE_BLUE` from the set `CSS4_COLORS`. (Colour number `946`)
/// ## Representations:
/// - int tuple `(99, 45, 233)`
/// - float tuple `(0.39, 0, 1)`
/// - hex: `#632DE9`
pub const XKCD_PURPLE_BLUE: Color = Color(99, 45, 233);

/// Colour `(r = 10, g = 136, b = 138)`
///
/// Colour `XKCD_DARK_CYAN` from the set `CSS4_COLORS`. (Colour number `947`)
/// ## Representations:
/// - int tuple `(10, 136, 138)`
/// - float tuple `(0.04, 1, 1)`
/// - hex: `#0A888A`
pub const XKCD_DARK_CYAN: Color = Color(10, 136, 138);

/// Colour `(r = 111, g = 118, b = 50)`
///
/// Colour `XKCD_OLIVE_DRAB` from the set `CSS4_COLORS`. (Colour number `948`)
/// ## Representations:
/// - int tuple `(111, 118, 50)`
/// - float tuple `(0.44, 0, 0)`
/// - hex: `#6F7632`
pub const XKCD_OLIVE_DRAB: Color = Color(111, 118, 50);

/// Colour `(r = 212, g = 106, b = 126)`
///
/// Colour `XKCD_PINKISH` from the set `CSS4_COLORS`. (Colour number `949`)
/// ## Representations:
/// - int tuple `(212, 106, 126)`
/// - float tuple `(0.83, 0, 0)`
/// - hex: `#D46A7E`
pub const XKCD_PINKISH: Color = Color(212, 106, 126);

/// Colour `(r = 30, g = 72, b = 143)`
///
/// Colour `XKCD_COBALT` from the set `CSS4_COLORS`. (Colour number `950`)
/// ## Representations:
/// - int tuple `(30, 72, 143)`
/// - float tuple `(0.12, 0, 1)`
/// - hex: `#1E488F`
pub const XKCD_COBALT: Color = Color(30, 72, 143);

/// Colour `(r = 188, g = 19, b = 254)`
///
/// Colour `XKCD_NEON_PURPLE` from the set `CSS4_COLORS`. (Colour number `951`)
/// ## Representations:
/// - int tuple `(188, 19, 254)`
/// - float tuple `(0.74, 0, 1)`
/// - hex: `#BC13FE`
pub const XKCD_NEON_PURPLE: Color = Color(188, 19, 254);

/// Colour `(r = 126, g = 244, b = 204)`
///
/// Colour `XKCD_LIGHT_TURQUOISE` from the set `CSS4_COLORS`. (Colour number `952`)
/// ## Representations:
/// - int tuple `(126, 244, 204)`
/// - float tuple `(0.49, 1, 1)`
/// - hex: `#7EF4CC`
pub const XKCD_LIGHT_TURQUOISE: Color = Color(126, 244, 204);

/// Colour `(r = 118, g = 205, b = 38)`
///
/// Colour `XKCD_APPLE_GREEN` from the set `CSS4_COLORS`. (Colour number `953`)
/// ## Representations:
/// - int tuple `(118, 205, 38)`
/// - float tuple `(0.46, 1, 0)`
/// - hex: `#76CD26`
pub const XKCD_APPLE_GREEN: Color = Color(118, 205, 38);

/// Colour `(r = 116, g = 166, b = 98)`
///
/// Colour `XKCD_DULL_GREEN` from the set `CSS4_COLORS`. (Colour number `954`)
/// ## Representations:
/// - int tuple `(116, 166, 98)`
/// - float tuple `(0.45, 1, 0)`
/// - hex: `#74A662`
pub const XKCD_DULL_GREEN: Color = Color(116, 166, 98);

/// Colour `(r = 128, g = 1, b = 63)`
///
/// Colour `XKCD_WINE` from the set `CSS4_COLORS`. (Colour number `955`)
/// ## Representations:
/// - int tuple `(128, 1, 63)`
/// - float tuple `(0.5, 0, 0)`
/// - hex: `#80013F`
pub const XKCD_WINE: Color = Color(128, 1, 63);

/// Colour `(r = 177, g = 209, b = 252)`
///
/// Colour `XKCD_POWDER_BLUE` from the set `CSS4_COLORS`. (Colour number `956`)
/// ## Representations:
/// - int tuple `(177, 209, 252)`
/// - float tuple `(0.69, 1, 1)`
/// - hex: `#B1D1FC`
pub const XKCD_POWDER_BLUE: Color = Color(177, 209, 252);

/// Colour `(r = 255, g = 255, b = 228)`
///
/// Colour `XKCD_OFF_WHITE` from the set `CSS4_COLORS`. (Colour number `957`)
/// ## Representations:
/// - int tuple `(255, 255, 228)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFFFE4`
pub const XKCD_OFF_WHITE: Color = Color(255, 255, 228);

/// Colour `(r = 6, g = 82, b = 255)`
///
/// Colour `XKCD_ELECTRIC_BLUE` from the set `CSS4_COLORS`. (Colour number `958`)
/// ## Representations:
/// - int tuple `(6, 82, 255)`
/// - float tuple `(0.02, 0, 1)`
/// - hex: `#0652FF`
pub const XKCD_ELECTRIC_BLUE: Color = Color(6, 82, 255);

/// Colour `(r = 4, g = 92, b = 90)`
///
/// Colour `XKCD_DARK_TURQUOISE` from the set `CSS4_COLORS`. (Colour number `959`)
/// ## Representations:
/// - int tuple `(4, 92, 90)`
/// - float tuple `(0.02, 0, 0)`
/// - hex: `#045C5A`
pub const XKCD_DARK_TURQUOISE: Color = Color(4, 92, 90);

/// Colour `(r = 87, g = 41, b = 206)`
///
/// Colour `XKCD_BLUE_PURPLE` from the set `CSS4_COLORS`. (Colour number `960`)
/// ## Representations:
/// - int tuple `(87, 41, 206)`
/// - float tuple `(0.34, 0, 1)`
/// - hex: `#5729CE`
pub const XKCD_BLUE_PURPLE: Color = Color(87, 41, 206);

/// Colour `(r = 6, g = 154, b = 243)`
///
/// Colour `XKCD_AZURE` from the set `CSS4_COLORS`. (Colour number `961`)
/// ## Representations:
/// - int tuple `(6, 154, 243)`
/// - float tuple `(0.02, 1, 1)`
/// - hex: `#069AF3`
pub const XKCD_AZURE: Color = Color(6, 154, 243);

/// Colour `(r = 255, g = 0, b = 13)`
///
/// Colour `XKCD_BRIGHT_RED` from the set `CSS4_COLORS`. (Colour number `962`)
/// ## Representations:
/// - int tuple `(255, 0, 13)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FF000D`
pub const XKCD_BRIGHT_RED: Color = Color(255, 0, 13);

/// Colour `(r = 241, g = 12, b = 69)`
///
/// Colour `XKCD_PINKISH_RED` from the set `CSS4_COLORS`. (Colour number `963`)
/// ## Representations:
/// - int tuple `(241, 12, 69)`
/// - float tuple `(0.95, 0, 0)`
/// - hex: `#F10C45`
pub const XKCD_PINKISH_RED: Color = Color(241, 12, 69);

/// Colour `(r = 81, g = 112, b = 215)`
///
/// Colour `XKCD_CORNFLOWER_BLUE` from the set `CSS4_COLORS`. (Colour number `964`)
/// ## Representations:
/// - int tuple `(81, 112, 215)`
/// - float tuple `(0.32, 0, 1)`
/// - hex: `#5170D7`
pub const XKCD_CORNFLOWER_BLUE: Color = Color(81, 112, 215);

/// Colour `(r = 172, g = 191, b = 105)`
///
/// Colour `XKCD_LIGHT_OLIVE` from the set `CSS4_COLORS`. (Colour number `965`)
/// ## Representations:
/// - int tuple `(172, 191, 105)`
/// - float tuple `(0.67, 1, 0)`
/// - hex: `#ACBF69`
pub const XKCD_LIGHT_OLIVE: Color = Color(172, 191, 105);

/// Colour `(r = 108, g = 52, b = 97)`
///
/// Colour `XKCD_GRAPE` from the set `CSS4_COLORS`. (Colour number `966`)
/// ## Representations:
/// - int tuple `(108, 52, 97)`
/// - float tuple `(0.42, 0, 0)`
/// - hex: `#6C3461`
pub const XKCD_GRAPE: Color = Color(108, 52, 97);

/// Colour `(r = 94, g = 129, b = 157)`
///
/// Colour `XKCD_GREYISH_BLUE` from the set `CSS4_COLORS`. (Colour number `967`)
/// ## Representations:
/// - int tuple `(94, 129, 157)`
/// - float tuple `(0.37, 1, 1)`
/// - hex: `#5E819D`
pub const XKCD_GREYISH_BLUE: Color = Color(94, 129, 157);

/// Colour `(r = 96, g = 30, b = 249)`
///
/// Colour `XKCD_PURPLISH_BLUE` from the set `CSS4_COLORS`. (Colour number `968`)
/// ## Representations:
/// - int tuple `(96, 30, 249)`
/// - float tuple `(0.38, 0, 1)`
/// - hex: `#601EF9`
pub const XKCD_PURPLISH_BLUE: Color = Color(96, 30, 249);

/// Colour `(r = 176, g = 221, b = 22)`
///
/// Colour `XKCD_YELLOWISH_GREEN` from the set `CSS4_COLORS`. (Colour number `969`)
/// ## Representations:
/// - int tuple `(176, 221, 22)`
/// - float tuple `(0.69, 1, 0)`
/// - hex: `#B0DD16`
pub const XKCD_YELLOWISH_GREEN: Color = Color(176, 221, 22);

/// Colour `(r = 205, g = 253, b = 2)`
///
/// Colour `XKCD_GREENISH_YELLOW` from the set `CSS4_COLORS`. (Colour number `970`)
/// ## Representations:
/// - int tuple `(205, 253, 2)`
/// - float tuple `(0.8, 1, 0)`
/// - hex: `#CDFD02`
pub const XKCD_GREENISH_YELLOW: Color = Color(205, 253, 2);

/// Colour `(r = 44, g = 111, b = 187)`
///
/// Colour `XKCD_MEDIUM_BLUE` from the set `CSS4_COLORS`. (Colour number `971`)
/// ## Representations:
/// - int tuple `(44, 111, 187)`
/// - float tuple `(0.17, 0, 1)`
/// - hex: `#2C6FBB`
pub const XKCD_MEDIUM_BLUE: Color = Color(44, 111, 187);

/// Colour `(r = 192, g = 115, b = 122)`
///
/// Colour `XKCD_DUSTY_ROSE` from the set `CSS4_COLORS`. (Colour number `972`)
/// ## Representations:
/// - int tuple `(192, 115, 122)`
/// - float tuple `(0.75, 0, 0)`
/// - hex: `#C0737A`
pub const XKCD_DUSTY_ROSE: Color = Color(192, 115, 122);

/// Colour `(r = 214, g = 180, b = 252)`
///
/// Colour `XKCD_LIGHT_VIOLET` from the set `CSS4_COLORS`. (Colour number `973`)
/// ## Representations:
/// - int tuple `(214, 180, 252)`
/// - float tuple `(0.84, 1, 1)`
/// - hex: `#D6B4FC`
pub const XKCD_LIGHT_VIOLET: Color = Color(214, 180, 252);

/// Colour `(r = 2, g = 0, b = 53)`
///
/// Colour `XKCD_MIDNIGHT_BLUE` from the set `CSS4_COLORS`. (Colour number `974`)
/// ## Representations:
/// - int tuple `(2, 0, 53)`
/// - float tuple `(0.01, 0, 0)`
/// - hex: `#020035`
pub const XKCD_MIDNIGHT_BLUE: Color = Color(2, 0, 53);

/// Colour `(r = 112, g = 59, b = 231)`
///
/// Colour `XKCD_BLUISH_PURPLE` from the set `CSS4_COLORS`. (Colour number `975`)
/// ## Representations:
/// - int tuple `(112, 59, 231)`
/// - float tuple `(0.44, 0, 1)`
/// - hex: `#703BE7`
pub const XKCD_BLUISH_PURPLE: Color = Color(112, 59, 231);

/// Colour `(r = 253, g = 60, b = 6)`
///
/// Colour `XKCD_RED_ORANGE` from the set `CSS4_COLORS`. (Colour number `976`)
/// ## Representations:
/// - int tuple `(253, 60, 6)`
/// - float tuple `(0.99, 0, 0)`
/// - hex: `#FD3C06`
pub const XKCD_RED_ORANGE: Color = Color(253, 60, 6);

/// Colour `(r = 150, g = 0, b = 86)`
///
/// Colour `XKCD_DARK_MAGENTA` from the set `CSS4_COLORS`. (Colour number `977`)
/// ## Representations:
/// - int tuple `(150, 0, 86)`
/// - float tuple `(0.59, 0, 0)`
/// - hex: `#960056`
pub const XKCD_DARK_MAGENTA: Color = Color(150, 0, 86);

/// Colour `(r = 64, g = 163, b = 104)`
///
/// Colour `XKCD_GREENISH` from the set `CSS4_COLORS`. (Colour number `978`)
/// ## Representations:
/// - int tuple `(64, 163, 104)`
/// - float tuple `(0.25, 1, 0)`
/// - hex: `#40A368`
pub const XKCD_GREENISH: Color = Color(64, 163, 104);

/// Colour `(r = 3, g = 113, b = 156)`
///
/// Colour `XKCD_OCEAN_BLUE` from the set `CSS4_COLORS`. (Colour number `979`)
/// ## Representations:
/// - int tuple `(3, 113, 156)`
/// - float tuple `(0.01, 0, 1)`
/// - hex: `#03719C`
pub const XKCD_OCEAN_BLUE: Color = Color(3, 113, 156);

/// Colour `(r = 252, g = 90, b = 80)`
///
/// Colour `XKCD_CORAL` from the set `CSS4_COLORS`. (Colour number `980`)
/// ## Representations:
/// - int tuple `(252, 90, 80)`
/// - float tuple `(0.99, 0, 0)`
/// - hex: `#FC5A50`
pub const XKCD_CORAL: Color = Color(252, 90, 80);

/// Colour `(r = 255, g = 255, b = 194)`
///
/// Colour `XKCD_CREAM` from the set `CSS4_COLORS`. (Colour number `981`)
/// ## Representations:
/// - int tuple `(255, 255, 194)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFFFC2`
pub const XKCD_CREAM: Color = Color(255, 255, 194);

/// Colour `(r = 127, g = 43, b = 10)`
///
/// Colour `XKCD_REDDISH_BROWN` from the set `CSS4_COLORS`. (Colour number `982`)
/// ## Representations:
/// - int tuple `(127, 43, 10)`
/// - float tuple `(0.5, 0, 0)`
/// - hex: `#7F2B0A`
pub const XKCD_REDDISH_BROWN: Color = Color(127, 43, 10);

/// Colour `(r = 176, g = 78, b = 15)`
///
/// Colour `XKCD_BURNT_SIENNA` from the set `CSS4_COLORS`. (Colour number `983`)
/// ## Representations:
/// - int tuple `(176, 78, 15)`
/// - float tuple `(0.69, 0, 0)`
/// - hex: `#B04E0F`
pub const XKCD_BURNT_SIENNA: Color = Color(176, 78, 15);

/// Colour `(r = 160, g = 54, b = 35)`
///
/// Colour `XKCD_BRICK` from the set `CSS4_COLORS`. (Colour number `984`)
/// ## Representations:
/// - int tuple `(160, 54, 35)`
/// - float tuple `(0.63, 0, 0)`
/// - hex: `#A03623`
pub const XKCD_BRICK: Color = Color(160, 54, 35);

/// Colour `(r = 135, g = 174, b = 115)`
///
/// Colour `XKCD_SAGE` from the set `CSS4_COLORS`. (Colour number `985`)
/// ## Representations:
/// - int tuple `(135, 174, 115)`
/// - float tuple `(0.53, 1, 0)`
/// - hex: `#87AE73`
pub const XKCD_SAGE: Color = Color(135, 174, 115);

/// Colour `(r = 120, g = 155, b = 115)`
///
/// Colour `XKCD_GREY_GREEN` from the set `CSS4_COLORS`. (Colour number `986`)
/// ## Representations:
/// - int tuple `(120, 155, 115)`
/// - float tuple `(0.47, 1, 0)`
/// - hex: `#789B73`
pub const XKCD_GREY_GREEN: Color = Color(120, 155, 115);

/// Colour `(r = 255, g = 255, b = 255)`
///
/// Colour `XKCD_WHITE` from the set `CSS4_COLORS`. (Colour number `987`)
/// ## Representations:
/// - int tuple `(255, 255, 255)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFFFFF`
pub const XKCD_WHITE: Color = Color(255, 255, 255);

/// Colour `(r = 152, g = 239, b = 249)`
///
/// Colour `XKCD_ROBIN_S_EGG_BLUE` from the set `CSS4_COLORS`. (Colour number `988`)
/// ## Representations:
/// - int tuple `(152, 239, 249)`
/// - float tuple `(0.6, 1, 1)`
/// - hex: `#98EFF9`
pub const XKCD_ROBIN_S_EGG_BLUE: Color = Color(152, 239, 249);

/// Colour `(r = 101, g = 139, b = 56)`
///
/// Colour `XKCD_MOSS_GREEN` from the set `CSS4_COLORS`. (Colour number `989`)
/// ## Representations:
/// - int tuple `(101, 139, 56)`
/// - float tuple `(0.4, 1, 0)`
/// - hex: `#658B38`
pub const XKCD_MOSS_GREEN: Color = Color(101, 139, 56);

/// Colour `(r = 90, g = 125, b = 154)`
///
/// Colour `XKCD_STEEL_BLUE` from the set `CSS4_COLORS`. (Colour number `990`)
/// ## Representations:
/// - int tuple `(90, 125, 154)`
/// - float tuple `(0.35, 0, 1)`
/// - hex: `#5A7D9A`
pub const XKCD_STEEL_BLUE: Color = Color(90, 125, 154);

/// Colour `(r = 56, g = 8, b = 53)`
///
/// Colour `XKCD_EGGPLANT` from the set `CSS4_COLORS`. (Colour number `991`)
/// ## Representations:
/// - int tuple `(56, 8, 53)`
/// - float tuple `(0.22, 0, 0)`
/// - hex: `#380835`
pub const XKCD_EGGPLANT: Color = Color(56, 8, 53);

/// Colour `(r = 255, g = 254, b = 122)`
///
/// Colour `XKCD_LIGHT_YELLOW` from the set `CSS4_COLORS`. (Colour number `992`)
/// ## Representations:
/// - int tuple `(255, 254, 122)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFFE7A`
pub const XKCD_LIGHT_YELLOW: Color = Color(255, 254, 122);

/// Colour `(r = 92, g = 169, b = 4)`
///
/// Colour `XKCD_LEAF_GREEN` from the set `CSS4_COLORS`. (Colour number `993`)
/// ## Representations:
/// - int tuple `(92, 169, 4)`
/// - float tuple `(0.36, 1, 0)`
/// - hex: `#5CA904`
pub const XKCD_LEAF_GREEN: Color = Color(92, 169, 4);

/// Colour `(r = 216, g = 220, b = 214)`
///
/// Colour `XKCD_LIGHT_GREY` from the set `CSS4_COLORS`. (Colour number `994`)
/// ## Representations:
/// - int tuple `(216, 220, 214)`
/// - float tuple `(0.85, 1, 1)`
/// - hex: `#D8DCD6`
pub const XKCD_LIGHT_GREY: Color = Color(216, 220, 214);

/// Colour `(r = 165, g = 165, b = 2)`
///
/// Colour `XKCD_PUKE` from the set `CSS4_COLORS`. (Colour number `995`)
/// ## Representations:
/// - int tuple `(165, 165, 2)`
/// - float tuple `(0.65, 1, 0)`
/// - hex: `#A5A502`
pub const XKCD_PUKE: Color = Color(165, 165, 2);

/// Colour `(r = 214, g = 72, b = 215)`
///
/// Colour `XKCD_PINKISH_PURPLE` from the set `CSS4_COLORS`. (Colour number `996`)
/// ## Representations:
/// - int tuple `(214, 72, 215)`
/// - float tuple `(0.84, 0, 1)`
/// - hex: `#D648D7`
pub const XKCD_PINKISH_PURPLE: Color = Color(214, 72, 215);

/// Colour `(r = 4, g = 116, b = 149)`
///
/// Colour `XKCD_SEA_BLUE` from the set `CSS4_COLORS`. (Colour number `997`)
/// ## Representations:
/// - int tuple `(4, 116, 149)`
/// - float tuple `(0.02, 0, 1)`
/// - hex: `#047495`
pub const XKCD_SEA_BLUE: Color = Color(4, 116, 149);

/// Colour `(r = 183, g = 144, b = 212)`
///
/// Colour `XKCD_PALE_PURPLE` from the set `CSS4_COLORS`. (Colour number `998`)
/// ## Representations:
/// - int tuple `(183, 144, 212)`
/// - float tuple `(0.72, 1, 1)`
/// - hex: `#B790D4`
pub const XKCD_PALE_PURPLE: Color = Color(183, 144, 212);

/// Colour `(r = 91, g = 124, b = 153)`
///
/// Colour `XKCD_SLATE_BLUE` from the set `CSS4_COLORS`. (Colour number `999`)
/// ## Representations:
/// - int tuple `(91, 124, 153)`
/// - float tuple `(0.36, 0, 1)`
/// - hex: `#5B7C99`
pub const XKCD_SLATE_BLUE: Color = Color(91, 124, 153);

/// Colour `(r = 96, g = 124, b = 142)`
///
/// Colour `XKCD_BLUE_GREY` from the set `CSS4_COLORS`. (Colour number `1000`)
/// ## Representations:
/// - int tuple `(96, 124, 142)`
/// - float tuple `(0.38, 0, 1)`
/// - hex: `#607C8E`
pub const XKCD_BLUE_GREY: Color = Color(96, 124, 142);

/// Colour `(r = 11, g = 64, b = 8)`
///
/// Colour `XKCD_HUNTER_GREEN` from the set `CSS4_COLORS`. (Colour number `1001`)
/// ## Representations:
/// - int tuple `(11, 64, 8)`
/// - float tuple `(0.04, 0, 0)`
/// - hex: `#0B4008`
pub const XKCD_HUNTER_GREEN: Color = Color(11, 64, 8);

/// Colour `(r = 237, g = 13, b = 217)`
///
/// Colour `XKCD_FUCHSIA` from the set `CSS4_COLORS`. (Colour number `1002`)
/// ## Representations:
/// - int tuple `(237, 13, 217)`
/// - float tuple `(0.93, 0, 1)`
/// - hex: `#ED0DD9`
pub const XKCD_FUCHSIA: Color = Color(237, 13, 217);

/// Colour `(r = 140, g = 0, b = 15)`
///
/// Colour `XKCD_CRIMSON` from the set `CSS4_COLORS`. (Colour number `1003`)
/// ## Representations:
/// - int tuple `(140, 0, 15)`
/// - float tuple `(0.55, 0, 0)`
/// - hex: `#8C000F`
pub const XKCD_CRIMSON: Color = Color(140, 0, 15);

/// Colour `(r = 255, g = 255, b = 132)`
///
/// Colour `XKCD_PALE_YELLOW` from the set `CSS4_COLORS`. (Colour number `1004`)
/// ## Representations:
/// - int tuple `(255, 255, 132)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFFF84`
pub const XKCD_PALE_YELLOW: Color = Color(255, 255, 132);

/// Colour `(r = 191, g = 144, b = 5)`
///
/// Colour `XKCD_OCHRE` from the set `CSS4_COLORS`. (Colour number `1005`)
/// ## Representations:
/// - int tuple `(191, 144, 5)`
/// - float tuple `(0.75, 1, 0)`
/// - hex: `#BF9005`
pub const XKCD_OCHRE: Color = Color(191, 144, 5);

/// Colour `(r = 210, g = 189, b = 10)`
///
/// Colour `XKCD_MUSTARD_YELLOW` from the set `CSS4_COLORS`. (Colour number `1006`)
/// ## Representations:
/// - int tuple `(210, 189, 10)`
/// - float tuple `(0.82, 1, 0)`
/// - hex: `#D2BD0A`
pub const XKCD_MUSTARD_YELLOW: Color = Color(210, 189, 10);

/// Colour `(r = 255, g = 71, b = 76)`
///
/// Colour `XKCD_LIGHT_RED` from the set `CSS4_COLORS`. (Colour number `1007`)
/// ## Representations:
/// - int tuple `(255, 71, 76)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FF474C`
pub const XKCD_LIGHT_RED: Color = Color(255, 71, 76);

/// Colour `(r = 4, g = 133, b = 209)`
///
/// Colour `XKCD_CERULEAN` from the set `CSS4_COLORS`. (Colour number `1008`)
/// ## Representations:
/// - int tuple `(4, 133, 209)`
/// - float tuple `(0.02, 1, 1)`
/// - hex: `#0485D1`
pub const XKCD_CERULEAN: Color = Color(4, 133, 209);

/// Colour `(r = 255, g = 207, b = 220)`
///
/// Colour `XKCD_PALE_PINK` from the set `CSS4_COLORS`. (Colour number `1009`)
/// ## Representations:
/// - int tuple `(255, 207, 220)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFCFDC`
pub const XKCD_PALE_PINK: Color = Color(255, 207, 220);

/// Colour `(r = 4, g = 2, b = 115)`
///
/// Colour `XKCD_DEEP_BLUE` from the set `CSS4_COLORS`. (Colour number `1010`)
/// ## Representations:
/// - int tuple `(4, 2, 115)`
/// - float tuple `(0.02, 0, 0)`
/// - hex: `#040273`
pub const XKCD_DEEP_BLUE: Color = Color(4, 2, 115);

/// Colour `(r = 168, g = 60, b = 9)`
///
/// Colour `XKCD_RUST` from the set `CSS4_COLORS`. (Colour number `1011`)
/// ## Representations:
/// - int tuple `(168, 60, 9)`
/// - float tuple `(0.66, 0, 0)`
/// - hex: `#A83C09`
pub const XKCD_RUST: Color = Color(168, 60, 9);

/// Colour `(r = 144, g = 228, b = 193)`
///
/// Colour `XKCD_LIGHT_TEAL` from the set `CSS4_COLORS`. (Colour number `1012`)
/// ## Representations:
/// - int tuple `(144, 228, 193)`
/// - float tuple `(0.56, 1, 1)`
/// - hex: `#90E4C1`
pub const XKCD_LIGHT_TEAL: Color = Color(144, 228, 193);

/// Colour `(r = 81, g = 101, b = 114)`
///
/// Colour `XKCD_SLATE` from the set `CSS4_COLORS`. (Colour number `1013`)
/// ## Representations:
/// - int tuple `(81, 101, 114)`
/// - float tuple `(0.32, 0, 0)`
/// - hex: `#516572`
pub const XKCD_SLATE: Color = Color(81, 101, 114);

/// Colour `(r = 250, g = 194, b = 5)`
///
/// Colour `XKCD_GOLDENROD` from the set `CSS4_COLORS`. (Colour number `1014`)
/// ## Representations:
/// - int tuple `(250, 194, 5)`
/// - float tuple `(0.98, 1, 0)`
/// - hex: `#FAC205`
pub const XKCD_GOLDENROD: Color = Color(250, 194, 5);

/// Colour `(r = 213, g = 182, b = 10)`
///
/// Colour `XKCD_DARK_YELLOW` from the set `CSS4_COLORS`. (Colour number `1015`)
/// ## Representations:
/// - int tuple `(213, 182, 10)`
/// - float tuple `(0.84, 1, 0)`
/// - hex: `#D5B60A`
pub const XKCD_DARK_YELLOW: Color = Color(213, 182, 10);

/// Colour `(r = 54, g = 55, b = 55)`
///
/// Colour `XKCD_DARK_GREY` from the set `CSS4_COLORS`. (Colour number `1016`)
/// ## Representations:
/// - int tuple `(54, 55, 55)`
/// - float tuple `(0.21, 0, 0)`
/// - hex: `#363737`
pub const XKCD_DARK_GREY: Color = Color(54, 55, 55);

/// Colour `(r = 75, g = 93, b = 22)`
///
/// Colour `XKCD_ARMY_GREEN` from the set `CSS4_COLORS`. (Colour number `1017`)
/// ## Representations:
/// - int tuple `(75, 93, 22)`
/// - float tuple `(0.29, 0, 0)`
/// - hex: `#4B5D16`
pub const XKCD_ARMY_GREEN: Color = Color(75, 93, 22);

/// Colour `(r = 107, g = 139, b = 164)`
///
/// Colour `XKCD_GREY_BLUE` from the set `CSS4_COLORS`. (Colour number `1018`)
/// ## Representations:
/// - int tuple `(107, 139, 164)`
/// - float tuple `(0.42, 1, 1)`
/// - hex: `#6B8BA4`
pub const XKCD_GREY_BLUE: Color = Color(107, 139, 164);

/// Colour `(r = 128, g = 249, b = 173)`
///
/// Colour `XKCD_SEAFOAM` from the set `CSS4_COLORS`. (Colour number `1019`)
/// ## Representations:
/// - int tuple `(128, 249, 173)`
/// - float tuple `(0.5, 1, 1)`
/// - hex: `#80F9AD`
pub const XKCD_SEAFOAM: Color = Color(128, 249, 173);

/// Colour `(r = 165, g = 126, b = 82)`
///
/// Colour `XKCD_PUCE` from the set `CSS4_COLORS`. (Colour number `1020`)
/// ## Representations:
/// - int tuple `(165, 126, 82)`
/// - float tuple `(0.65, 0, 0)`
/// - hex: `#A57E52`
pub const XKCD_PUCE: Color = Color(165, 126, 82);

/// Colour `(r = 169, g = 249, b = 113)`
///
/// Colour `XKCD_SPRING_GREEN` from the set `CSS4_COLORS`. (Colour number `1021`)
/// ## Representations:
/// - int tuple `(169, 249, 113)`
/// - float tuple `(0.66, 1, 0)`
/// - hex: `#A9F971`
pub const XKCD_SPRING_GREEN: Color = Color(169, 249, 113);

/// Colour `(r = 198, g = 81, b = 2)`
///
/// Colour `XKCD_DARK_ORANGE` from the set `CSS4_COLORS`. (Colour number `1022`)
/// ## Representations:
/// - int tuple `(198, 81, 2)`
/// - float tuple `(0.78, 0, 0)`
/// - hex: `#C65102`
pub const XKCD_DARK_ORANGE: Color = Color(198, 81, 2);

/// Colour `(r = 226, g = 202, b = 118)`
///
/// Colour `XKCD_SAND` from the set `CSS4_COLORS`. (Colour number `1023`)
/// ## Representations:
/// - int tuple `(226, 202, 118)`
/// - float tuple `(0.89, 1, 0)`
/// - hex: `#E2CA76`
pub const XKCD_SAND: Color = Color(226, 202, 118);

/// Colour `(r = 176, g = 255, b = 157)`
///
/// Colour `XKCD_PASTEL_GREEN` from the set `CSS4_COLORS`. (Colour number `1024`)
/// ## Representations:
/// - int tuple `(176, 255, 157)`
/// - float tuple `(0.69, 1, 1)`
/// - hex: `#B0FF9D`
pub const XKCD_PASTEL_GREEN: Color = Color(176, 255, 157);

/// Colour `(r = 159, g = 254, b = 176)`
///
/// Colour `XKCD_MINT` from the set `CSS4_COLORS`. (Colour number `1025`)
/// ## Representations:
/// - int tuple `(159, 254, 176)`
/// - float tuple `(0.62, 1, 1)`
/// - hex: `#9FFEB0`
pub const XKCD_MINT: Color = Color(159, 254, 176);

/// Colour `(r = 253, g = 170, b = 72)`
///
/// Colour `XKCD_LIGHT_ORANGE` from the set `CSS4_COLORS`. (Colour number `1026`)
/// ## Representations:
/// - int tuple `(253, 170, 72)`
/// - float tuple `(0.99, 1, 0)`
/// - hex: `#FDAA48`
pub const XKCD_LIGHT_ORANGE: Color = Color(253, 170, 72);

/// Colour `(r = 254, g = 1, b = 177)`
///
/// Colour `XKCD_BRIGHT_PINK` from the set `CSS4_COLORS`. (Colour number `1027`)
/// ## Representations:
/// - int tuple `(254, 1, 177)`
/// - float tuple `(1.0, 0, 1)`
/// - hex: `#FE01B1`
pub const XKCD_BRIGHT_PINK: Color = Color(254, 1, 177);

/// Colour `(r = 193, g = 248, b = 10)`
///
/// Colour `XKCD_CHARTREUSE` from the set `CSS4_COLORS`. (Colour number `1028`)
/// ## Representations:
/// - int tuple `(193, 248, 10)`
/// - float tuple `(0.76, 1, 0)`
/// - hex: `#C1F80A`
pub const XKCD_CHARTREUSE: Color = Color(193, 248, 10);

/// Colour `(r = 54, g = 1, b = 63)`
///
/// Colour `XKCD_DEEP_PURPLE` from the set `CSS4_COLORS`. (Colour number `1029`)
/// ## Representations:
/// - int tuple `(54, 1, 63)`
/// - float tuple `(0.21, 0, 0)`
/// - hex: `#36013F`
pub const XKCD_DEEP_PURPLE: Color = Color(54, 1, 63);

/// Colour `(r = 52, g = 28, b = 2)`
///
/// Colour `XKCD_DARK_BROWN` from the set `CSS4_COLORS`. (Colour number `1030`)
/// ## Representations:
/// - int tuple `(52, 28, 2)`
/// - float tuple `(0.2, 0, 0)`
/// - hex: `#341C02`
pub const XKCD_DARK_BROWN: Color = Color(52, 28, 2);

/// Colour `(r = 185, g = 162, b = 129)`
///
/// Colour `XKCD_TAUPE` from the set `CSS4_COLORS`. (Colour number `1031`)
/// ## Representations:
/// - int tuple `(185, 162, 129)`
/// - float tuple `(0.73, 1, 1)`
/// - hex: `#B9A281`
pub const XKCD_TAUPE: Color = Color(185, 162, 129);

/// Colour `(r = 142, g = 171, b = 18)`
///
/// Colour `XKCD_PEA_GREEN` from the set `CSS4_COLORS`. (Colour number `1032`)
/// ## Representations:
/// - int tuple `(142, 171, 18)`
/// - float tuple `(0.56, 1, 0)`
/// - hex: `#8EAB12`
pub const XKCD_PEA_GREEN: Color = Color(142, 171, 18);

/// Colour `(r = 154, g = 174, b = 7)`
///
/// Colour `XKCD_PUKE_GREEN` from the set `CSS4_COLORS`. (Colour number `1033`)
/// ## Representations:
/// - int tuple `(154, 174, 7)`
/// - float tuple `(0.6, 1, 0)`
/// - hex: `#9AAE07`
pub const XKCD_PUKE_GREEN: Color = Color(154, 174, 7);

/// Colour `(r = 2, g = 171, b = 46)`
///
/// Colour `XKCD_KELLY_GREEN` from the set `CSS4_COLORS`. (Colour number `1034`)
/// ## Representations:
/// - int tuple `(2, 171, 46)`
/// - float tuple `(0.01, 1, 0)`
/// - hex: `#02AB2E`
pub const XKCD_KELLY_GREEN: Color = Color(2, 171, 46);

/// Colour `(r = 122, g = 249, b = 171)`
///
/// Colour `XKCD_SEAFOAM_GREEN` from the set `CSS4_COLORS`. (Colour number `1035`)
/// ## Representations:
/// - int tuple `(122, 249, 171)`
/// - float tuple `(0.48, 1, 1)`
/// - hex: `#7AF9AB`
pub const XKCD_SEAFOAM_GREEN: Color = Color(122, 249, 171);

/// Colour `(r = 19, g = 126, b = 109)`
///
/// Colour `XKCD_BLUE_GREEN` from the set `CSS4_COLORS`. (Colour number `1036`)
/// ## Representations:
/// - int tuple `(19, 126, 109)`
/// - float tuple `(0.07, 0, 0)`
/// - hex: `#137E6D`
pub const XKCD_BLUE_GREEN: Color = Color(19, 126, 109);

/// Colour `(r = 170, g = 166, b = 98)`
///
/// Colour `XKCD_KHAKI` from the set `CSS4_COLORS`. (Colour number `1037`)
/// ## Representations:
/// - int tuple `(170, 166, 98)`
/// - float tuple `(0.67, 1, 0)`
/// - hex: `#AAA662`
pub const XKCD_KHAKI: Color = Color(170, 166, 98);

/// Colour `(r = 97, g = 0, b = 35)`
///
/// Colour `XKCD_BURGUNDY` from the set `CSS4_COLORS`. (Colour number `1038`)
/// ## Representations:
/// - int tuple `(97, 0, 35)`
/// - float tuple `(0.38, 0, 0)`
/// - hex: `#610023`
pub const XKCD_BURGUNDY: Color = Color(97, 0, 35);

/// Colour `(r = 1, g = 77, b = 78)`
///
/// Colour `XKCD_DARK_TEAL` from the set `CSS4_COLORS`. (Colour number `1039`)
/// ## Representations:
/// - int tuple `(1, 77, 78)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#014D4E`
pub const XKCD_DARK_TEAL: Color = Color(1, 77, 78);

/// Colour `(r = 143, g = 20, b = 2)`
///
/// Colour `XKCD_BRICK_RED` from the set `CSS4_COLORS`. (Colour number `1040`)
/// ## Representations:
/// - int tuple `(143, 20, 2)`
/// - float tuple `(0.56, 0, 0)`
/// - hex: `#8F1402`
pub const XKCD_BRICK_RED: Color = Color(143, 20, 2);

/// Colour `(r = 75, g = 0, b = 110)`
///
/// Colour `XKCD_ROYAL_PURPLE` from the set `CSS4_COLORS`. (Colour number `1041`)
/// ## Representations:
/// - int tuple `(75, 0, 110)`
/// - float tuple `(0.29, 0, 0)`
/// - hex: `#4B006E`
pub const XKCD_ROYAL_PURPLE: Color = Color(75, 0, 110);

/// Colour `(r = 88, g = 15, b = 65)`
///
/// Colour `XKCD_PLUM` from the set `CSS4_COLORS`. (Colour number `1042`)
/// ## Representations:
/// - int tuple `(88, 15, 65)`
/// - float tuple `(0.35, 0, 0)`
/// - hex: `#580F41`
pub const XKCD_PLUM: Color = Color(88, 15, 65);

/// Colour `(r = 143, g = 255, b = 159)`
///
/// Colour `XKCD_MINT_GREEN` from the set `CSS4_COLORS`. (Colour number `1043`)
/// ## Representations:
/// - int tuple `(143, 255, 159)`
/// - float tuple `(0.56, 1, 1)`
/// - hex: `#8FFF9F`
pub const XKCD_MINT_GREEN: Color = Color(143, 255, 159);

/// Colour `(r = 219, g = 180, b = 12)`
///
/// Colour `XKCD_GOLD` from the set `CSS4_COLORS`. (Colour number `1044`)
/// ## Representations:
/// - int tuple `(219, 180, 12)`
/// - float tuple `(0.86, 1, 0)`
/// - hex: `#DBB40C`
pub const XKCD_GOLD: Color = Color(219, 180, 12);

/// Colour `(r = 162, g = 207, b = 254)`
///
/// Colour `XKCD_BABY_BLUE` from the set `CSS4_COLORS`. (Colour number `1045`)
/// ## Representations:
/// - int tuple `(162, 207, 254)`
/// - float tuple `(0.64, 1, 1)`
/// - hex: `#A2CFFE`
pub const XKCD_BABY_BLUE: Color = Color(162, 207, 254);

/// Colour `(r = 192, g = 251, b = 45)`
///
/// Colour `XKCD_YELLOW_GREEN` from the set `CSS4_COLORS`. (Colour number `1046`)
/// ## Representations:
/// - int tuple `(192, 251, 45)`
/// - float tuple `(0.75, 1, 0)`
/// - hex: `#C0FB2D`
pub const XKCD_YELLOW_GREEN: Color = Color(192, 251, 45);

/// Colour `(r = 190, g = 3, b = 253)`
///
/// Colour `XKCD_BRIGHT_PURPLE` from the set `CSS4_COLORS`. (Colour number `1047`)
/// ## Representations:
/// - int tuple `(190, 3, 253)`
/// - float tuple `(0.75, 0, 1)`
/// - hex: `#BE03FD`
pub const XKCD_BRIGHT_PURPLE: Color = Color(190, 3, 253);

/// Colour `(r = 132, g = 0, b = 0)`
///
/// Colour `XKCD_DARK_RED` from the set `CSS4_COLORS`. (Colour number `1048`)
/// ## Representations:
/// - int tuple `(132, 0, 0)`
/// - float tuple `(0.52, 0, 0)`
/// - hex: `#840000`
pub const XKCD_DARK_RED: Color = Color(132, 0, 0);

/// Colour `(r = 208, g = 254, b = 254)`
///
/// Colour `XKCD_PALE_BLUE` from the set `CSS4_COLORS`. (Colour number `1049`)
/// ## Representations:
/// - int tuple `(208, 254, 254)`
/// - float tuple `(0.82, 1, 1)`
/// - hex: `#D0FEFE`
pub const XKCD_PALE_BLUE: Color = Color(208, 254, 254);

/// Colour `(r = 63, g = 155, b = 11)`
///
/// Colour `XKCD_GRASS_GREEN` from the set `CSS4_COLORS`. (Colour number `1050`)
/// ## Representations:
/// - int tuple `(63, 155, 11)`
/// - float tuple `(0.25, 1, 0)`
/// - hex: `#3F9B0B`
pub const XKCD_GRASS_GREEN: Color = Color(63, 155, 11);

/// Colour `(r = 1, g = 21, b = 62)`
///
/// Colour `XKCD_NAVY` from the set `CSS4_COLORS`. (Colour number `1051`)
/// ## Representations:
/// - int tuple `(1, 21, 62)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#01153E`
pub const XKCD_NAVY: Color = Color(1, 21, 62);

/// Colour `(r = 4, g = 216, b = 178)`
///
/// Colour `XKCD_AQUAMARINE` from the set `CSS4_COLORS`. (Colour number `1052`)
/// ## Representations:
/// - int tuple `(4, 216, 178)`
/// - float tuple `(0.02, 1, 1)`
/// - hex: `#04D8B2`
pub const XKCD_AQUAMARINE: Color = Color(4, 216, 178);

/// Colour `(r = 192, g = 78, b = 1)`
///
/// Colour `XKCD_BURNT_ORANGE` from the set `CSS4_COLORS`. (Colour number `1053`)
/// ## Representations:
/// - int tuple `(192, 78, 1)`
/// - float tuple `(0.75, 0, 0)`
/// - hex: `#C04E01`
pub const XKCD_BURNT_ORANGE: Color = Color(192, 78, 1);

/// Colour `(r = 12, g = 255, b = 12)`
///
/// Colour `XKCD_NEON_GREEN` from the set `CSS4_COLORS`. (Colour number `1054`)
/// ## Representations:
/// - int tuple `(12, 255, 12)`
/// - float tuple `(0.05, 1, 0)`
/// - hex: `#0CFF0C`
pub const XKCD_NEON_GREEN: Color = Color(12, 255, 12);

/// Colour `(r = 1, g = 101, b = 252)`
///
/// Colour `XKCD_BRIGHT_BLUE` from the set `CSS4_COLORS`. (Colour number `1055`)
/// ## Representations:
/// - int tuple `(1, 101, 252)`
/// - float tuple `(0.0, 0, 1)`
/// - hex: `#0165FC`
pub const XKCD_BRIGHT_BLUE: Color = Color(1, 101, 252);

/// Colour `(r = 207, g = 98, b = 117)`
///
/// Colour `XKCD_ROSE` from the set `CSS4_COLORS`. (Colour number `1056`)
/// ## Representations:
/// - int tuple `(207, 98, 117)`
/// - float tuple `(0.81, 0, 0)`
/// - hex: `#CF6275`
pub const XKCD_ROSE: Color = Color(207, 98, 117);

/// Colour `(r = 255, g = 209, b = 223)`
///
/// Colour `XKCD_LIGHT_PINK` from the set `CSS4_COLORS`. (Colour number `1057`)
/// ## Representations:
/// - int tuple `(255, 209, 223)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FFD1DF`
pub const XKCD_LIGHT_PINK: Color = Color(255, 209, 223);

/// Colour `(r = 206, g = 179, b = 1)`
///
/// Colour `XKCD_MUSTARD` from the set `CSS4_COLORS`. (Colour number `1058`)
/// ## Representations:
/// - int tuple `(206, 179, 1)`
/// - float tuple `(0.81, 1, 0)`
/// - hex: `#CEB301`
pub const XKCD_MUSTARD: Color = Color(206, 179, 1);

/// Colour `(r = 56, g = 2, b = 130)`
///
/// Colour `XKCD_INDIGO` from the set `CSS4_COLORS`. (Colour number `1059`)
/// ## Representations:
/// - int tuple `(56, 2, 130)`
/// - float tuple `(0.22, 0, 1)`
/// - hex: `#380282`
pub const XKCD_INDIGO: Color = Color(56, 2, 130);

/// Colour `(r = 170, g = 255, b = 50)`
///
/// Colour `XKCD_LIME` from the set `CSS4_COLORS`. (Colour number `1060`)
/// ## Representations:
/// - int tuple `(170, 255, 50)`
/// - float tuple `(0.67, 1, 0)`
/// - hex: `#AAFF32`
pub const XKCD_LIME: Color = Color(170, 255, 50);

/// Colour `(r = 83, g = 252, b = 161)`
///
/// Colour `XKCD_SEA_GREEN` from the set `CSS4_COLORS`. (Colour number `1061`)
/// ## Representations:
/// - int tuple `(83, 252, 161)`
/// - float tuple `(0.33, 1, 1)`
/// - hex: `#53FCA1`
pub const XKCD_SEA_GREEN: Color = Color(83, 252, 161);

/// Colour `(r = 142, g = 130, b = 254)`
///
/// Colour `XKCD_PERIWINKLE` from the set `CSS4_COLORS`. (Colour number `1062`)
/// ## Representations:
/// - int tuple `(142, 130, 254)`
/// - float tuple `(0.56, 1, 1)`
/// - hex: `#8E82FE`
pub const XKCD_PERIWINKLE: Color = Color(142, 130, 254);

/// Colour `(r = 203, g = 65, b = 107)`
///
/// Colour `XKCD_DARK_PINK` from the set `CSS4_COLORS`. (Colour number `1063`)
/// ## Representations:
/// - int tuple `(203, 65, 107)`
/// - float tuple `(0.8, 0, 0)`
/// - hex: `#CB416B`
pub const XKCD_DARK_PINK: Color = Color(203, 65, 107);

/// Colour `(r = 103, g = 122, b = 4)`
///
/// Colour `XKCD_OLIVE_GREEN` from the set `CSS4_COLORS`. (Colour number `1064`)
/// ## Representations:
/// - int tuple `(103, 122, 4)`
/// - float tuple `(0.4, 0, 0)`
/// - hex: `#677A04`
pub const XKCD_OLIVE_GREEN: Color = Color(103, 122, 4);

/// Colour `(r = 255, g = 176, b = 124)`
///
/// Colour `XKCD_PEACH` from the set `CSS4_COLORS`. (Colour number `1065`)
/// ## Representations:
/// - int tuple `(255, 176, 124)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFB07C`
pub const XKCD_PEACH: Color = Color(255, 176, 124);

/// Colour `(r = 199, g = 253, b = 181)`
///
/// Colour `XKCD_PALE_GREEN` from the set `CSS4_COLORS`. (Colour number `1066`)
/// ## Representations:
/// - int tuple `(199, 253, 181)`
/// - float tuple `(0.78, 1, 1)`
/// - hex: `#C7FDB5`
pub const XKCD_PALE_GREEN: Color = Color(199, 253, 181);

/// Colour `(r = 173, g = 129, b = 80)`
///
/// Colour `XKCD_LIGHT_BROWN` from the set `CSS4_COLORS`. (Colour number `1067`)
/// ## Representations:
/// - int tuple `(173, 129, 80)`
/// - float tuple `(0.68, 1, 0)`
/// - hex: `#AD8150`
pub const XKCD_LIGHT_BROWN: Color = Color(173, 129, 80);

/// Colour `(r = 255, g = 2, b = 141)`
///
/// Colour `XKCD_HOT_PINK` from the set `CSS4_COLORS`. (Colour number `1068`)
/// ## Representations:
/// - int tuple `(255, 2, 141)`
/// - float tuple `(1.0, 0, 1)`
/// - hex: `#FF028D`
pub const XKCD_HOT_PINK: Color = Color(255, 2, 141);

/// Colour `(r = 0, g = 0, b = 0)`
///
/// Colour `XKCD_BLACK` from the set `CSS4_COLORS`. (Colour number `1069`)
/// ## Representations:
/// - int tuple `(0, 0, 0)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#000000`
pub const XKCD_BLACK: Color = Color(0, 0, 0);

/// Colour `(r = 206, g = 162, b = 253)`
///
/// Colour `XKCD_LILAC` from the set `CSS4_COLORS`. (Colour number `1070`)
/// ## Representations:
/// - int tuple `(206, 162, 253)`
/// - float tuple `(0.81, 1, 1)`
/// - hex: `#CEA2FD`
pub const XKCD_LILAC: Color = Color(206, 162, 253);

/// Colour `(r = 0, g = 17, b = 70)`
///
/// Colour `XKCD_NAVY_BLUE` from the set `CSS4_COLORS`. (Colour number `1071`)
/// ## Representations:
/// - int tuple `(0, 17, 70)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#001146`
pub const XKCD_NAVY_BLUE: Color = Color(0, 17, 70);

/// Colour `(r = 5, g = 4, b = 170)`
///
/// Colour `XKCD_ROYAL_BLUE` from the set `CSS4_COLORS`. (Colour number `1072`)
/// ## Representations:
/// - int tuple `(5, 4, 170)`
/// - float tuple `(0.02, 0, 1)`
/// - hex: `#0504AA`
pub const XKCD_ROYAL_BLUE: Color = Color(5, 4, 170);

/// Colour `(r = 230, g = 218, b = 166)`
///
/// Colour `XKCD_BEIGE` from the set `CSS4_COLORS`. (Colour number `1073`)
/// ## Representations:
/// - int tuple `(230, 218, 166)`
/// - float tuple `(0.9, 1, 1)`
/// - hex: `#E6DAA6`
pub const XKCD_BEIGE: Color = Color(230, 218, 166);

/// Colour `(r = 255, g = 121, b = 108)`
///
/// Colour `XKCD_SALMON` from the set `CSS4_COLORS`. (Colour number `1074`)
/// ## Representations:
/// - int tuple `(255, 121, 108)`
/// - float tuple `(1.0, 0, 0)`
/// - hex: `#FF796C`
pub const XKCD_SALMON: Color = Color(255, 121, 108);

/// Colour `(r = 110, g = 117, b = 14)`
///
/// Colour `XKCD_OLIVE` from the set `CSS4_COLORS`. (Colour number `1075`)
/// ## Representations:
/// - int tuple `(110, 117, 14)`
/// - float tuple `(0.43, 0, 0)`
/// - hex: `#6E750E`
pub const XKCD_OLIVE: Color = Color(110, 117, 14);

/// Colour `(r = 101, g = 0, b = 33)`
///
/// Colour `XKCD_MAROON` from the set `CSS4_COLORS`. (Colour number `1076`)
/// ## Representations:
/// - int tuple `(101, 0, 33)`
/// - float tuple `(0.4, 0, 0)`
/// - hex: `#650021`
pub const XKCD_MAROON: Color = Color(101, 0, 33);

/// Colour `(r = 1, g = 255, b = 7)`
///
/// Colour `XKCD_BRIGHT_GREEN` from the set `CSS4_COLORS`. (Colour number `1077`)
/// ## Representations:
/// - int tuple `(1, 255, 7)`
/// - float tuple `(0.0, 1, 0)`
/// - hex: `#01FF07`
pub const XKCD_BRIGHT_GREEN: Color = Color(1, 255, 7);

/// Colour `(r = 53, g = 6, b = 62)`
///
/// Colour `XKCD_DARK_PURPLE` from the set `CSS4_COLORS`. (Colour number `1078`)
/// ## Representations:
/// - int tuple `(53, 6, 62)`
/// - float tuple `(0.21, 0, 0)`
/// - hex: `#35063E`
pub const XKCD_DARK_PURPLE: Color = Color(53, 6, 62);

/// Colour `(r = 174, g = 113, b = 129)`
///
/// Colour `XKCD_MAUVE` from the set `CSS4_COLORS`. (Colour number `1079`)
/// ## Representations:
/// - int tuple `(174, 113, 129)`
/// - float tuple `(0.68, 0, 1)`
/// - hex: `#AE7181`
pub const XKCD_MAUVE: Color = Color(174, 113, 129);

/// Colour `(r = 6, g = 71, b = 12)`
///
/// Colour `XKCD_FOREST_GREEN` from the set `CSS4_COLORS`. (Colour number `1080`)
/// ## Representations:
/// - int tuple `(6, 71, 12)`
/// - float tuple `(0.02, 0, 0)`
/// - hex: `#06470C`
pub const XKCD_FOREST_GREEN: Color = Color(6, 71, 12);

/// Colour `(r = 19, g = 234, b = 201)`
///
/// Colour `XKCD_AQUA` from the set `CSS4_COLORS`. (Colour number `1081`)
/// ## Representations:
/// - int tuple `(19, 234, 201)`
/// - float tuple `(0.07, 1, 1)`
/// - hex: `#13EAC9`
pub const XKCD_AQUA: Color = Color(19, 234, 201);

/// Colour `(r = 0, g = 255, b = 255)`
///
/// Colour `XKCD_CYAN` from the set `CSS4_COLORS`. (Colour number `1082`)
/// ## Representations:
/// - int tuple `(0, 255, 255)`
/// - float tuple `(0.0, 1, 1)`
/// - hex: `#00FFFF`
pub const XKCD_CYAN: Color = Color(0, 255, 255);

/// Colour `(r = 209, g = 178, b = 111)`
///
/// Colour `XKCD_TAN` from the set `CSS4_COLORS`. (Colour number `1083`)
/// ## Representations:
/// - int tuple `(209, 178, 111)`
/// - float tuple `(0.82, 1, 0)`
/// - hex: `#D1B26F`
pub const XKCD_TAN: Color = Color(209, 178, 111);

/// Colour `(r = 0, g = 3, b = 91)`
///
/// Colour `XKCD_DARK_BLUE` from the set `CSS4_COLORS`. (Colour number `1084`)
/// ## Representations:
/// - int tuple `(0, 3, 91)`
/// - float tuple `(0.0, 0, 0)`
/// - hex: `#00035B`
pub const XKCD_DARK_BLUE: Color = Color(0, 3, 91);

/// Colour `(r = 199, g = 159, b = 239)`
///
/// Colour `XKCD_LAVENDER` from the set `CSS4_COLORS`. (Colour number `1085`)
/// ## Representations:
/// - int tuple `(199, 159, 239)`
/// - float tuple `(0.78, 1, 1)`
/// - hex: `#C79FEF`
pub const XKCD_LAVENDER: Color = Color(199, 159, 239);

/// Colour `(r = 6, g = 194, b = 172)`
///
/// Colour `XKCD_TURQUOISE` from the set `CSS4_COLORS`. (Colour number `1086`)
/// ## Representations:
/// - int tuple `(6, 194, 172)`
/// - float tuple `(0.02, 1, 1)`
/// - hex: `#06C2AC`
pub const XKCD_TURQUOISE: Color = Color(6, 194, 172);

/// Colour `(r = 3, g = 53, b = 0)`
///
/// Colour `XKCD_DARK_GREEN` from the set `CSS4_COLORS`. (Colour number `1087`)
/// ## Representations:
/// - int tuple `(3, 53, 0)`
/// - float tuple `(0.01, 0, 0)`
/// - hex: `#033500`
pub const XKCD_DARK_GREEN: Color = Color(3, 53, 0);

/// Colour `(r = 154, g = 14, b = 234)`
///
/// Colour `XKCD_VIOLET` from the set `CSS4_COLORS`. (Colour number `1088`)
/// ## Representations:
/// - int tuple `(154, 14, 234)`
/// - float tuple `(0.6, 0, 1)`
/// - hex: `#9A0EEA`
pub const XKCD_VIOLET: Color = Color(154, 14, 234);

/// Colour `(r = 191, g = 119, b = 246)`
///
/// Colour `XKCD_LIGHT_PURPLE` from the set `CSS4_COLORS`. (Colour number `1089`)
/// ## Representations:
/// - int tuple `(191, 119, 246)`
/// - float tuple `(0.75, 0, 1)`
/// - hex: `#BF77F6`
pub const XKCD_LIGHT_PURPLE: Color = Color(191, 119, 246);

/// Colour `(r = 137, g = 254, b = 5)`
///
/// Colour `XKCD_LIME_GREEN` from the set `CSS4_COLORS`. (Colour number `1090`)
/// ## Representations:
/// - int tuple `(137, 254, 5)`
/// - float tuple `(0.54, 1, 0)`
/// - hex: `#89FE05`
pub const XKCD_LIME_GREEN: Color = Color(137, 254, 5);

/// Colour `(r = 146, g = 149, b = 145)`
///
/// Colour `XKCD_GREY` from the set `CSS4_COLORS`. (Colour number `1091`)
/// ## Representations:
/// - int tuple `(146, 149, 145)`
/// - float tuple `(0.57, 1, 1)`
/// - hex: `#929591`
pub const XKCD_GREY: Color = Color(146, 149, 145);

/// Colour `(r = 117, g = 187, b = 253)`
///
/// Colour `XKCD_SKY_BLUE` from the set `CSS4_COLORS`. (Colour number `1092`)
/// ## Representations:
/// - int tuple `(117, 187, 253)`
/// - float tuple `(0.46, 1, 1)`
/// - hex: `#75BBFD`
pub const XKCD_SKY_BLUE: Color = Color(117, 187, 253);

/// Colour `(r = 255, g = 255, b = 20)`
///
/// Colour `XKCD_YELLOW` from the set `CSS4_COLORS`. (Colour number `1093`)
/// ## Representations:
/// - int tuple `(255, 255, 20)`
/// - float tuple `(1.0, 1, 0)`
/// - hex: `#FFFF14`
pub const XKCD_YELLOW: Color = Color(255, 255, 20);

/// Colour `(r = 194, g = 0, b = 120)`
///
/// Colour `XKCD_MAGENTA` from the set `CSS4_COLORS`. (Colour number `1094`)
/// ## Representations:
/// - int tuple `(194, 0, 120)`
/// - float tuple `(0.76, 0, 0)`
/// - hex: `#C20078`
pub const XKCD_MAGENTA: Color = Color(194, 0, 120);

/// Colour `(r = 150, g = 249, b = 123)`
///
/// Colour `XKCD_LIGHT_GREEN` from the set `CSS4_COLORS`. (Colour number `1095`)
/// ## Representations:
/// - int tuple `(150, 249, 123)`
/// - float tuple `(0.59, 1, 0)`
/// - hex: `#96F97B`
pub const XKCD_LIGHT_GREEN: Color = Color(150, 249, 123);

/// Colour `(r = 249, g = 115, b = 6)`
///
/// Colour `XKCD_ORANGE` from the set `CSS4_COLORS`. (Colour number `1096`)
/// ## Representations:
/// - int tuple `(249, 115, 6)`
/// - float tuple `(0.98, 0, 0)`
/// - hex: `#F97306`
pub const XKCD_ORANGE: Color = Color(249, 115, 6);

/// Colour `(r = 2, g = 147, b = 134)`
///
/// Colour `XKCD_TEAL` from the set `CSS4_COLORS`. (Colour number `1097`)
/// ## Representations:
/// - int tuple `(2, 147, 134)`
/// - float tuple `(0.01, 1, 1)`
/// - hex: `#029386`
pub const XKCD_TEAL: Color = Color(2, 147, 134);

/// Colour `(r = 149, g = 208, b = 252)`
///
/// Colour `XKCD_LIGHT_BLUE` from the set `CSS4_COLORS`. (Colour number `1098`)
/// ## Representations:
/// - int tuple `(149, 208, 252)`
/// - float tuple `(0.58, 1, 1)`
/// - hex: `#95D0FC`
pub const XKCD_LIGHT_BLUE: Color = Color(149, 208, 252);

/// Colour `(r = 229, g = 0, b = 0)`
///
/// Colour `XKCD_RED` from the set `CSS4_COLORS`. (Colour number `1099`)
/// ## Representations:
/// - int tuple `(229, 0, 0)`
/// - float tuple `(0.9, 0, 0)`
/// - hex: `#E50000`
pub const XKCD_RED: Color = Color(229, 0, 0);

/// Colour `(r = 101, g = 55, b = 0)`
///
/// Colour `XKCD_BROWN` from the set `CSS4_COLORS`. (Colour number `1100`)
/// ## Representations:
/// - int tuple `(101, 55, 0)`
/// - float tuple `(0.4, 0, 0)`
/// - hex: `#653700`
pub const XKCD_BROWN: Color = Color(101, 55, 0);

/// Colour `(r = 255, g = 129, b = 192)`
///
/// Colour `XKCD_PINK` from the set `CSS4_COLORS`. (Colour number `1101`)
/// ## Representations:
/// - int tuple `(255, 129, 192)`
/// - float tuple `(1.0, 1, 1)`
/// - hex: `#FF81C0`
pub const XKCD_PINK: Color = Color(255, 129, 192);

/// Colour `(r = 3, g = 67, b = 223)`
///
/// Colour `XKCD_BLUE` from the set `CSS4_COLORS`. (Colour number `1102`)
/// ## Representations:
/// - int tuple `(3, 67, 223)`
/// - float tuple `(0.01, 0, 1)`
/// - hex: `#0343DF`
pub const XKCD_BLUE: Color = Color(3, 67, 223);

/// Colour `(r = 21, g = 176, b = 26)`
///
/// Colour `XKCD_GREEN` from the set `CSS4_COLORS`. (Colour number `1103`)
/// ## Representations:
/// - int tuple `(21, 176, 26)`
/// - float tuple `(0.08, 1, 0)`
/// - hex: `#15B01A`
pub const XKCD_GREEN: Color = Color(21, 176, 26);

/// Colour `(r = 126, g = 30, b = 156)`
///
/// Colour `XKCD_PURPLE` from the set `CSS4_COLORS`. (Colour number `1104`)
/// ## Representations:
/// - int tuple `(126, 30, 156)`
/// - float tuple `(0.49, 0, 1)`
/// - hex: `#7E1E9C`
pub const XKCD_PURPLE: Color = Color(126, 30, 156);
