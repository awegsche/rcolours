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

/// Color `B` from the set BASE_COLORS. (Color number `0`)
pub const B: Color = Color(0, 0, 255);
/// Color `G` from the set BASE_COLORS. (Color number `1`)
pub const G: Color = Color(0, 127, 0);
/// Color `R` from the set BASE_COLORS. (Color number `2`)
pub const R: Color = Color(255, 0, 0);
/// Color `C` from the set BASE_COLORS. (Color number `3`)
pub const C: Color = Color(0, 191, 191);
/// Color `M` from the set BASE_COLORS. (Color number `4`)
pub const M: Color = Color(191, 0, 191);
/// Color `Y` from the set BASE_COLORS. (Color number `5`)
pub const Y: Color = Color(191, 191, 0);
/// Color `K` from the set BASE_COLORS. (Color number `6`)
pub const K: Color = Color(0, 0, 0);
/// Color `W` from the set BASE_COLORS. (Color number `7`)
pub const W: Color = Color(255, 255, 255);
/// Color `ALICEBLUE` from the set CSS4_COLORS. (Color number `8`)
pub const ALICEBLUE: Color = Color(240, 248, 255);
/// Color `ANTIQUEWHITE` from the set CSS4_COLORS. (Color number `9`)
pub const ANTIQUEWHITE: Color = Color(250, 235, 215);
/// Color `AQUA` from the set CSS4_COLORS. (Color number `10`)
pub const AQUA: Color = Color(0, 255, 255);
/// Color `AQUAMARINE` from the set CSS4_COLORS. (Color number `11`)
pub const AQUAMARINE: Color = Color(127, 255, 212);
/// Color `AZURE` from the set CSS4_COLORS. (Color number `12`)
pub const AZURE: Color = Color(240, 255, 255);
/// Color `BEIGE` from the set CSS4_COLORS. (Color number `13`)
pub const BEIGE: Color = Color(245, 245, 220);
/// Color `BISQUE` from the set CSS4_COLORS. (Color number `14`)
pub const BISQUE: Color = Color(255, 228, 196);
/// Color `BLACK` from the set CSS4_COLORS. (Color number `15`)
pub const BLACK: Color = Color(0, 0, 0);
/// Color `BLANCHEDALMOND` from the set CSS4_COLORS. (Color number `16`)
pub const BLANCHEDALMOND: Color = Color(255, 235, 205);
/// Color `BLUE` from the set CSS4_COLORS. (Color number `17`)
pub const BLUE: Color = Color(0, 0, 255);
/// Color `BLUEVIOLET` from the set CSS4_COLORS. (Color number `18`)
pub const BLUEVIOLET: Color = Color(138, 43, 226);
/// Color `BROWN` from the set CSS4_COLORS. (Color number `19`)
pub const BROWN: Color = Color(165, 42, 42);
/// Color `BURLYWOOD` from the set CSS4_COLORS. (Color number `20`)
pub const BURLYWOOD: Color = Color(222, 184, 135);
/// Color `CADETBLUE` from the set CSS4_COLORS. (Color number `21`)
pub const CADETBLUE: Color = Color(95, 158, 160);
/// Color `CHARTREUSE` from the set CSS4_COLORS. (Color number `22`)
pub const CHARTREUSE: Color = Color(127, 255, 0);
/// Color `CHOCOLATE` from the set CSS4_COLORS. (Color number `23`)
pub const CHOCOLATE: Color = Color(210, 105, 30);
/// Color `CORAL` from the set CSS4_COLORS. (Color number `24`)
pub const CORAL: Color = Color(255, 127, 80);
/// Color `CORNFLOWERBLUE` from the set CSS4_COLORS. (Color number `25`)
pub const CORNFLOWERBLUE: Color = Color(100, 149, 237);
/// Color `CORNSILK` from the set CSS4_COLORS. (Color number `26`)
pub const CORNSILK: Color = Color(255, 248, 220);
/// Color `CRIMSON` from the set CSS4_COLORS. (Color number `27`)
pub const CRIMSON: Color = Color(220, 20, 60);
/// Color `CYAN` from the set CSS4_COLORS. (Color number `28`)
pub const CYAN: Color = Color(0, 255, 255);
/// Color `DARKBLUE` from the set CSS4_COLORS. (Color number `29`)
pub const DARKBLUE: Color = Color(0, 0, 139);
/// Color `DARKCYAN` from the set CSS4_COLORS. (Color number `30`)
pub const DARKCYAN: Color = Color(0, 139, 139);
/// Color `DARKGOLDENROD` from the set CSS4_COLORS. (Color number `31`)
pub const DARKGOLDENROD: Color = Color(184, 134, 11);
/// Color `DARKGRAY` from the set CSS4_COLORS. (Color number `32`)
pub const DARKGRAY: Color = Color(169, 169, 169);
/// Color `DARKGREEN` from the set CSS4_COLORS. (Color number `33`)
pub const DARKGREEN: Color = Color(0, 100, 0);
/// Color `DARKGREY` from the set CSS4_COLORS. (Color number `34`)
pub const DARKGREY: Color = Color(169, 169, 169);
/// Color `DARKKHAKI` from the set CSS4_COLORS. (Color number `35`)
pub const DARKKHAKI: Color = Color(189, 183, 107);
/// Color `DARKMAGENTA` from the set CSS4_COLORS. (Color number `36`)
pub const DARKMAGENTA: Color = Color(139, 0, 139);
/// Color `DARKOLIVEGREEN` from the set CSS4_COLORS. (Color number `37`)
pub const DARKOLIVEGREEN: Color = Color(85, 107, 47);
/// Color `DARKORANGE` from the set CSS4_COLORS. (Color number `38`)
pub const DARKORANGE: Color = Color(255, 140, 0);
/// Color `DARKORCHID` from the set CSS4_COLORS. (Color number `39`)
pub const DARKORCHID: Color = Color(153, 50, 204);
/// Color `DARKRED` from the set CSS4_COLORS. (Color number `40`)
pub const DARKRED: Color = Color(139, 0, 0);
/// Color `DARKSALMON` from the set CSS4_COLORS. (Color number `41`)
pub const DARKSALMON: Color = Color(233, 150, 122);
/// Color `DARKSEAGREEN` from the set CSS4_COLORS. (Color number `42`)
pub const DARKSEAGREEN: Color = Color(143, 188, 143);
/// Color `DARKSLATEBLUE` from the set CSS4_COLORS. (Color number `43`)
pub const DARKSLATEBLUE: Color = Color(72, 61, 139);
/// Color `DARKSLATEGRAY` from the set CSS4_COLORS. (Color number `44`)
pub const DARKSLATEGRAY: Color = Color(47, 79, 79);
/// Color `DARKSLATEGREY` from the set CSS4_COLORS. (Color number `45`)
pub const DARKSLATEGREY: Color = Color(47, 79, 79);
/// Color `DARKTURQUOISE` from the set CSS4_COLORS. (Color number `46`)
pub const DARKTURQUOISE: Color = Color(0, 206, 209);
/// Color `DARKVIOLET` from the set CSS4_COLORS. (Color number `47`)
pub const DARKVIOLET: Color = Color(148, 0, 211);
/// Color `DEEPPINK` from the set CSS4_COLORS. (Color number `48`)
pub const DEEPPINK: Color = Color(255, 20, 147);
/// Color `DEEPSKYBLUE` from the set CSS4_COLORS. (Color number `49`)
pub const DEEPSKYBLUE: Color = Color(0, 191, 255);
/// Color `DIMGRAY` from the set CSS4_COLORS. (Color number `50`)
pub const DIMGRAY: Color = Color(105, 105, 105);
/// Color `DIMGREY` from the set CSS4_COLORS. (Color number `51`)
pub const DIMGREY: Color = Color(105, 105, 105);
/// Color `DODGERBLUE` from the set CSS4_COLORS. (Color number `52`)
pub const DODGERBLUE: Color = Color(30, 144, 255);
/// Color `FIREBRICK` from the set CSS4_COLORS. (Color number `53`)
pub const FIREBRICK: Color = Color(178, 34, 34);
/// Color `FLORALWHITE` from the set CSS4_COLORS. (Color number `54`)
pub const FLORALWHITE: Color = Color(255, 250, 240);
/// Color `FORESTGREEN` from the set CSS4_COLORS. (Color number `55`)
pub const FORESTGREEN: Color = Color(34, 139, 34);
/// Color `FUCHSIA` from the set CSS4_COLORS. (Color number `56`)
pub const FUCHSIA: Color = Color(255, 0, 255);
/// Color `GAINSBORO` from the set CSS4_COLORS. (Color number `57`)
pub const GAINSBORO: Color = Color(220, 220, 220);
/// Color `GHOSTWHITE` from the set CSS4_COLORS. (Color number `58`)
pub const GHOSTWHITE: Color = Color(248, 248, 255);
/// Color `GOLD` from the set CSS4_COLORS. (Color number `59`)
pub const GOLD: Color = Color(255, 215, 0);
/// Color `GOLDENROD` from the set CSS4_COLORS. (Color number `60`)
pub const GOLDENROD: Color = Color(218, 165, 32);
/// Color `GRAY` from the set CSS4_COLORS. (Color number `61`)
pub const GRAY: Color = Color(128, 128, 128);
/// Color `GREEN` from the set CSS4_COLORS. (Color number `62`)
pub const GREEN: Color = Color(0, 128, 0);
/// Color `GREENYELLOW` from the set CSS4_COLORS. (Color number `63`)
pub const GREENYELLOW: Color = Color(173, 255, 47);
/// Color `GREY` from the set CSS4_COLORS. (Color number `64`)
pub const GREY: Color = Color(128, 128, 128);
/// Color `HONEYDEW` from the set CSS4_COLORS. (Color number `65`)
pub const HONEYDEW: Color = Color(240, 255, 240);
/// Color `HOTPINK` from the set CSS4_COLORS. (Color number `66`)
pub const HOTPINK: Color = Color(255, 105, 180);
/// Color `INDIANRED` from the set CSS4_COLORS. (Color number `67`)
pub const INDIANRED: Color = Color(205, 92, 92);
/// Color `INDIGO` from the set CSS4_COLORS. (Color number `68`)
pub const INDIGO: Color = Color(75, 0, 130);
/// Color `IVORY` from the set CSS4_COLORS. (Color number `69`)
pub const IVORY: Color = Color(255, 255, 240);
/// Color `KHAKI` from the set CSS4_COLORS. (Color number `70`)
pub const KHAKI: Color = Color(240, 230, 140);
/// Color `LAVENDER` from the set CSS4_COLORS. (Color number `71`)
pub const LAVENDER: Color = Color(230, 230, 250);
/// Color `LAVENDERBLUSH` from the set CSS4_COLORS. (Color number `72`)
pub const LAVENDERBLUSH: Color = Color(255, 240, 245);
/// Color `LAWNGREEN` from the set CSS4_COLORS. (Color number `73`)
pub const LAWNGREEN: Color = Color(124, 252, 0);
/// Color `LEMONCHIFFON` from the set CSS4_COLORS. (Color number `74`)
pub const LEMONCHIFFON: Color = Color(255, 250, 205);
/// Color `LIGHTBLUE` from the set CSS4_COLORS. (Color number `75`)
pub const LIGHTBLUE: Color = Color(173, 216, 230);
/// Color `LIGHTCORAL` from the set CSS4_COLORS. (Color number `76`)
pub const LIGHTCORAL: Color = Color(240, 128, 128);
/// Color `LIGHTCYAN` from the set CSS4_COLORS. (Color number `77`)
pub const LIGHTCYAN: Color = Color(224, 255, 255);
/// Color `LIGHTGOLDENRODYELLOW` from the set CSS4_COLORS. (Color number `78`)
pub const LIGHTGOLDENRODYELLOW: Color = Color(250, 250, 210);
/// Color `LIGHTGRAY` from the set CSS4_COLORS. (Color number `79`)
pub const LIGHTGRAY: Color = Color(211, 211, 211);
/// Color `LIGHTGREEN` from the set CSS4_COLORS. (Color number `80`)
pub const LIGHTGREEN: Color = Color(144, 238, 144);
/// Color `LIGHTGREY` from the set CSS4_COLORS. (Color number `81`)
pub const LIGHTGREY: Color = Color(211, 211, 211);
/// Color `LIGHTPINK` from the set CSS4_COLORS. (Color number `82`)
pub const LIGHTPINK: Color = Color(255, 182, 193);
/// Color `LIGHTSALMON` from the set CSS4_COLORS. (Color number `83`)
pub const LIGHTSALMON: Color = Color(255, 160, 122);
/// Color `LIGHTSEAGREEN` from the set CSS4_COLORS. (Color number `84`)
pub const LIGHTSEAGREEN: Color = Color(32, 178, 170);
/// Color `LIGHTSKYBLUE` from the set CSS4_COLORS. (Color number `85`)
pub const LIGHTSKYBLUE: Color = Color(135, 206, 250);
/// Color `LIGHTSLATEGRAY` from the set CSS4_COLORS. (Color number `86`)
pub const LIGHTSLATEGRAY: Color = Color(119, 136, 153);
/// Color `LIGHTSLATEGREY` from the set CSS4_COLORS. (Color number `87`)
pub const LIGHTSLATEGREY: Color = Color(119, 136, 153);
/// Color `LIGHTSTEELBLUE` from the set CSS4_COLORS. (Color number `88`)
pub const LIGHTSTEELBLUE: Color = Color(176, 196, 222);
/// Color `LIGHTYELLOW` from the set CSS4_COLORS. (Color number `89`)
pub const LIGHTYELLOW: Color = Color(255, 255, 224);
/// Color `LIME` from the set CSS4_COLORS. (Color number `90`)
pub const LIME: Color = Color(0, 255, 0);
/// Color `LIMEGREEN` from the set CSS4_COLORS. (Color number `91`)
pub const LIMEGREEN: Color = Color(50, 205, 50);
/// Color `LINEN` from the set CSS4_COLORS. (Color number `92`)
pub const LINEN: Color = Color(250, 240, 230);
/// Color `MAGENTA` from the set CSS4_COLORS. (Color number `93`)
pub const MAGENTA: Color = Color(255, 0, 255);
/// Color `MAROON` from the set CSS4_COLORS. (Color number `94`)
pub const MAROON: Color = Color(128, 0, 0);
/// Color `MEDIUMAQUAMARINE` from the set CSS4_COLORS. (Color number `95`)
pub const MEDIUMAQUAMARINE: Color = Color(102, 205, 170);
/// Color `MEDIUMBLUE` from the set CSS4_COLORS. (Color number `96`)
pub const MEDIUMBLUE: Color = Color(0, 0, 205);
/// Color `MEDIUMORCHID` from the set CSS4_COLORS. (Color number `97`)
pub const MEDIUMORCHID: Color = Color(186, 85, 211);
/// Color `MEDIUMPURPLE` from the set CSS4_COLORS. (Color number `98`)
pub const MEDIUMPURPLE: Color = Color(147, 112, 219);
/// Color `MEDIUMSEAGREEN` from the set CSS4_COLORS. (Color number `99`)
pub const MEDIUMSEAGREEN: Color = Color(60, 179, 113);
/// Color `MEDIUMSLATEBLUE` from the set CSS4_COLORS. (Color number `100`)
pub const MEDIUMSLATEBLUE: Color = Color(123, 104, 238);
/// Color `MEDIUMSPRINGGREEN` from the set CSS4_COLORS. (Color number `101`)
pub const MEDIUMSPRINGGREEN: Color = Color(0, 250, 154);
/// Color `MEDIUMTURQUOISE` from the set CSS4_COLORS. (Color number `102`)
pub const MEDIUMTURQUOISE: Color = Color(72, 209, 204);
/// Color `MEDIUMVIOLETRED` from the set CSS4_COLORS. (Color number `103`)
pub const MEDIUMVIOLETRED: Color = Color(199, 21, 133);
/// Color `MIDNIGHTBLUE` from the set CSS4_COLORS. (Color number `104`)
pub const MIDNIGHTBLUE: Color = Color(25, 25, 112);
/// Color `MINTCREAM` from the set CSS4_COLORS. (Color number `105`)
pub const MINTCREAM: Color = Color(245, 255, 250);
/// Color `MISTYROSE` from the set CSS4_COLORS. (Color number `106`)
pub const MISTYROSE: Color = Color(255, 228, 225);
/// Color `MOCCASIN` from the set CSS4_COLORS. (Color number `107`)
pub const MOCCASIN: Color = Color(255, 228, 181);
/// Color `NAVAJOWHITE` from the set CSS4_COLORS. (Color number `108`)
pub const NAVAJOWHITE: Color = Color(255, 222, 173);
/// Color `NAVY` from the set CSS4_COLORS. (Color number `109`)
pub const NAVY: Color = Color(0, 0, 128);
/// Color `OLDLACE` from the set CSS4_COLORS. (Color number `110`)
pub const OLDLACE: Color = Color(253, 245, 230);
/// Color `OLIVE` from the set CSS4_COLORS. (Color number `111`)
pub const OLIVE: Color = Color(128, 128, 0);
/// Color `OLIVEDRAB` from the set CSS4_COLORS. (Color number `112`)
pub const OLIVEDRAB: Color = Color(107, 142, 35);
/// Color `ORANGE` from the set CSS4_COLORS. (Color number `113`)
pub const ORANGE: Color = Color(255, 165, 0);
/// Color `ORANGERED` from the set CSS4_COLORS. (Color number `114`)
pub const ORANGERED: Color = Color(255, 69, 0);
/// Color `ORCHID` from the set CSS4_COLORS. (Color number `115`)
pub const ORCHID: Color = Color(218, 112, 214);
/// Color `PALEGOLDENROD` from the set CSS4_COLORS. (Color number `116`)
pub const PALEGOLDENROD: Color = Color(238, 232, 170);
/// Color `PALEGREEN` from the set CSS4_COLORS. (Color number `117`)
pub const PALEGREEN: Color = Color(152, 251, 152);
/// Color `PALETURQUOISE` from the set CSS4_COLORS. (Color number `118`)
pub const PALETURQUOISE: Color = Color(175, 238, 238);
/// Color `PALEVIOLETRED` from the set CSS4_COLORS. (Color number `119`)
pub const PALEVIOLETRED: Color = Color(219, 112, 147);
/// Color `PAPAYAWHIP` from the set CSS4_COLORS. (Color number `120`)
pub const PAPAYAWHIP: Color = Color(255, 239, 213);
/// Color `PEACHPUFF` from the set CSS4_COLORS. (Color number `121`)
pub const PEACHPUFF: Color = Color(255, 218, 185);
/// Color `PERU` from the set CSS4_COLORS. (Color number `122`)
pub const PERU: Color = Color(205, 133, 63);
/// Color `PINK` from the set CSS4_COLORS. (Color number `123`)
pub const PINK: Color = Color(255, 192, 203);
/// Color `PLUM` from the set CSS4_COLORS. (Color number `124`)
pub const PLUM: Color = Color(221, 160, 221);
/// Color `POWDERBLUE` from the set CSS4_COLORS. (Color number `125`)
pub const POWDERBLUE: Color = Color(176, 224, 230);
/// Color `PURPLE` from the set CSS4_COLORS. (Color number `126`)
pub const PURPLE: Color = Color(128, 0, 128);
/// Color `REBECCAPURPLE` from the set CSS4_COLORS. (Color number `127`)
pub const REBECCAPURPLE: Color = Color(102, 51, 153);
/// Color `RED` from the set CSS4_COLORS. (Color number `128`)
pub const RED: Color = Color(255, 0, 0);
/// Color `ROSYBROWN` from the set CSS4_COLORS. (Color number `129`)
pub const ROSYBROWN: Color = Color(188, 143, 143);
/// Color `ROYALBLUE` from the set CSS4_COLORS. (Color number `130`)
pub const ROYALBLUE: Color = Color(65, 105, 225);
/// Color `SADDLEBROWN` from the set CSS4_COLORS. (Color number `131`)
pub const SADDLEBROWN: Color = Color(139, 69, 19);
/// Color `SALMON` from the set CSS4_COLORS. (Color number `132`)
pub const SALMON: Color = Color(250, 128, 114);
/// Color `SANDYBROWN` from the set CSS4_COLORS. (Color number `133`)
pub const SANDYBROWN: Color = Color(244, 164, 96);
/// Color `SEAGREEN` from the set CSS4_COLORS. (Color number `134`)
pub const SEAGREEN: Color = Color(46, 139, 87);
/// Color `SEASHELL` from the set CSS4_COLORS. (Color number `135`)
pub const SEASHELL: Color = Color(255, 245, 238);
/// Color `SIENNA` from the set CSS4_COLORS. (Color number `136`)
pub const SIENNA: Color = Color(160, 82, 45);
/// Color `SILVER` from the set CSS4_COLORS. (Color number `137`)
pub const SILVER: Color = Color(192, 192, 192);
/// Color `SKYBLUE` from the set CSS4_COLORS. (Color number `138`)
pub const SKYBLUE: Color = Color(135, 206, 235);
/// Color `SLATEBLUE` from the set CSS4_COLORS. (Color number `139`)
pub const SLATEBLUE: Color = Color(106, 90, 205);
/// Color `SLATEGRAY` from the set CSS4_COLORS. (Color number `140`)
pub const SLATEGRAY: Color = Color(112, 128, 144);
/// Color `SLATEGREY` from the set CSS4_COLORS. (Color number `141`)
pub const SLATEGREY: Color = Color(112, 128, 144);
/// Color `SNOW` from the set CSS4_COLORS. (Color number `142`)
pub const SNOW: Color = Color(255, 250, 250);
/// Color `SPRINGGREEN` from the set CSS4_COLORS. (Color number `143`)
pub const SPRINGGREEN: Color = Color(0, 255, 127);
/// Color `STEELBLUE` from the set CSS4_COLORS. (Color number `144`)
pub const STEELBLUE: Color = Color(70, 130, 180);
/// Color `TAN` from the set CSS4_COLORS. (Color number `145`)
pub const TAN: Color = Color(210, 180, 140);
/// Color `TEAL` from the set CSS4_COLORS. (Color number `146`)
pub const TEAL: Color = Color(0, 128, 128);
/// Color `THISTLE` from the set CSS4_COLORS. (Color number `147`)
pub const THISTLE: Color = Color(216, 191, 216);
/// Color `TOMATO` from the set CSS4_COLORS. (Color number `148`)
pub const TOMATO: Color = Color(255, 99, 71);
/// Color `TURQUOISE` from the set CSS4_COLORS. (Color number `149`)
pub const TURQUOISE: Color = Color(64, 224, 208);
/// Color `VIOLET` from the set CSS4_COLORS. (Color number `150`)
pub const VIOLET: Color = Color(238, 130, 238);
/// Color `WHEAT` from the set CSS4_COLORS. (Color number `151`)
pub const WHEAT: Color = Color(245, 222, 179);
/// Color `WHITE` from the set CSS4_COLORS. (Color number `152`)
pub const WHITE: Color = Color(255, 255, 255);
/// Color `WHITESMOKE` from the set CSS4_COLORS. (Color number `153`)
pub const WHITESMOKE: Color = Color(245, 245, 245);
/// Color `YELLOW` from the set CSS4_COLORS. (Color number `154`)
pub const YELLOW: Color = Color(255, 255, 0);
/// Color `YELLOWGREEN` from the set CSS4_COLORS. (Color number `155`)
pub const YELLOWGREEN: Color = Color(154, 205, 50);
/// Color `XKCD_CLOUDY_BLUE` from the set CSS4_COLORS. (Color number `156`)
pub const XKCD_CLOUDY_BLUE: Color = Color(172, 194, 217);
/// Color `XKCD_DARK_PASTEL_GREEN` from the set CSS4_COLORS. (Color number `157`)
pub const XKCD_DARK_PASTEL_GREEN: Color = Color(86, 174, 87);
/// Color `XKCD_DUST` from the set CSS4_COLORS. (Color number `158`)
pub const XKCD_DUST: Color = Color(178, 153, 110);
/// Color `XKCD_ELECTRIC_LIME` from the set CSS4_COLORS. (Color number `159`)
pub const XKCD_ELECTRIC_LIME: Color = Color(168, 255, 4);
/// Color `XKCD_FRESH_GREEN` from the set CSS4_COLORS. (Color number `160`)
pub const XKCD_FRESH_GREEN: Color = Color(105, 216, 79);
/// Color `XKCD_LIGHT_EGGPLANT` from the set CSS4_COLORS. (Color number `161`)
pub const XKCD_LIGHT_EGGPLANT: Color = Color(137, 69, 133);
/// Color `XKCD_NASTY_GREEN` from the set CSS4_COLORS. (Color number `162`)
pub const XKCD_NASTY_GREEN: Color = Color(112, 178, 63);
/// Color `XKCD_REALLY_LIGHT_BLUE` from the set CSS4_COLORS. (Color number `163`)
pub const XKCD_REALLY_LIGHT_BLUE: Color = Color(212, 255, 255);
/// Color `XKCD_TEA` from the set CSS4_COLORS. (Color number `164`)
pub const XKCD_TEA: Color = Color(101, 171, 124);
/// Color `XKCD_WARM_PURPLE` from the set CSS4_COLORS. (Color number `165`)
pub const XKCD_WARM_PURPLE: Color = Color(149, 46, 143);
/// Color `XKCD_YELLOWISH_TAN` from the set CSS4_COLORS. (Color number `166`)
pub const XKCD_YELLOWISH_TAN: Color = Color(252, 252, 129);
/// Color `XKCD_CEMENT` from the set CSS4_COLORS. (Color number `167`)
pub const XKCD_CEMENT: Color = Color(165, 163, 145);
/// Color `XKCD_DARK_GRASS_GREEN` from the set CSS4_COLORS. (Color number `168`)
pub const XKCD_DARK_GRASS_GREEN: Color = Color(56, 128, 4);
/// Color `XKCD_DUSTY_TEAL` from the set CSS4_COLORS. (Color number `169`)
pub const XKCD_DUSTY_TEAL: Color = Color(76, 144, 133);
/// Color `XKCD_GREY_TEAL` from the set CSS4_COLORS. (Color number `170`)
pub const XKCD_GREY_TEAL: Color = Color(94, 155, 138);
/// Color `XKCD_MACARONI_AND_CHEESE` from the set CSS4_COLORS. (Color number `171`)
pub const XKCD_MACARONI_AND_CHEESE: Color = Color(239, 180, 53);
/// Color `XKCD_PINKISH_TAN` from the set CSS4_COLORS. (Color number `172`)
pub const XKCD_PINKISH_TAN: Color = Color(217, 155, 130);
/// Color `XKCD_SPRUCE` from the set CSS4_COLORS. (Color number `173`)
pub const XKCD_SPRUCE: Color = Color(10, 95, 56);
/// Color `XKCD_STRONG_BLUE` from the set CSS4_COLORS. (Color number `174`)
pub const XKCD_STRONG_BLUE: Color = Color(12, 6, 247);
/// Color `XKCD_TOXIC_GREEN` from the set CSS4_COLORS. (Color number `175`)
pub const XKCD_TOXIC_GREEN: Color = Color(97, 222, 42);
/// Color `XKCD_WINDOWS_BLUE` from the set CSS4_COLORS. (Color number `176`)
pub const XKCD_WINDOWS_BLUE: Color = Color(55, 120, 191);
/// Color `XKCD_BLUE_BLUE` from the set CSS4_COLORS. (Color number `177`)
pub const XKCD_BLUE_BLUE: Color = Color(34, 66, 199);
/// Color `XKCD_BLUE_WITH_A_HINT_OF_PURPLE` from the set CSS4_COLORS. (Color number `178`)
pub const XKCD_BLUE_WITH_A_HINT_OF_PURPLE: Color = Color(83, 60, 198);
/// Color `XKCD_BOOGER` from the set CSS4_COLORS. (Color number `179`)
pub const XKCD_BOOGER: Color = Color(155, 181, 60);
/// Color `XKCD_BRIGHT_SEA_GREEN` from the set CSS4_COLORS. (Color number `180`)
pub const XKCD_BRIGHT_SEA_GREEN: Color = Color(5, 255, 166);
/// Color `XKCD_DARK_GREEN_BLUE` from the set CSS4_COLORS. (Color number `181`)
pub const XKCD_DARK_GREEN_BLUE: Color = Color(31, 99, 87);
/// Color `XKCD_DEEP_TURQUOISE` from the set CSS4_COLORS. (Color number `182`)
pub const XKCD_DEEP_TURQUOISE: Color = Color(1, 115, 116);
/// Color `XKCD_GREEN_TEAL` from the set CSS4_COLORS. (Color number `183`)
pub const XKCD_GREEN_TEAL: Color = Color(12, 181, 119);
/// Color `XKCD_STRONG_PINK` from the set CSS4_COLORS. (Color number `184`)
pub const XKCD_STRONG_PINK: Color = Color(255, 7, 137);
/// Color `XKCD_BLAND` from the set CSS4_COLORS. (Color number `185`)
pub const XKCD_BLAND: Color = Color(175, 168, 139);
/// Color `XKCD_DEEP_AQUA` from the set CSS4_COLORS. (Color number `186`)
pub const XKCD_DEEP_AQUA: Color = Color(8, 120, 127);
/// Color `XKCD_LAVENDER_PINK` from the set CSS4_COLORS. (Color number `187`)
pub const XKCD_LAVENDER_PINK: Color = Color(221, 133, 215);
/// Color `XKCD_LIGHT_MOSS_GREEN` from the set CSS4_COLORS. (Color number `188`)
pub const XKCD_LIGHT_MOSS_GREEN: Color = Color(166, 200, 117);
/// Color `XKCD_LIGHT_SEAFOAM_GREEN` from the set CSS4_COLORS. (Color number `189`)
pub const XKCD_LIGHT_SEAFOAM_GREEN: Color = Color(167, 255, 181);
/// Color `XKCD_OLIVE_YELLOW` from the set CSS4_COLORS. (Color number `190`)
pub const XKCD_OLIVE_YELLOW: Color = Color(194, 183, 9);
/// Color `XKCD_PIG_PINK` from the set CSS4_COLORS. (Color number `191`)
pub const XKCD_PIG_PINK: Color = Color(231, 142, 165);
/// Color `XKCD_DEEP_LILAC` from the set CSS4_COLORS. (Color number `192`)
pub const XKCD_DEEP_LILAC: Color = Color(150, 110, 189);
/// Color `XKCD_DESERT` from the set CSS4_COLORS. (Color number `193`)
pub const XKCD_DESERT: Color = Color(204, 173, 96);
/// Color `XKCD_DUSTY_LAVENDER` from the set CSS4_COLORS. (Color number `194`)
pub const XKCD_DUSTY_LAVENDER: Color = Color(172, 134, 168);
/// Color `XKCD_PURPLEY_GREY` from the set CSS4_COLORS. (Color number `195`)
pub const XKCD_PURPLEY_GREY: Color = Color(148, 126, 148);
/// Color `XKCD_PURPLY` from the set CSS4_COLORS. (Color number `196`)
pub const XKCD_PURPLY: Color = Color(152, 63, 178);
/// Color `XKCD_CANDY_PINK` from the set CSS4_COLORS. (Color number `197`)
pub const XKCD_CANDY_PINK: Color = Color(255, 99, 233);
/// Color `XKCD_LIGHT_PASTEL_GREEN` from the set CSS4_COLORS. (Color number `198`)
pub const XKCD_LIGHT_PASTEL_GREEN: Color = Color(178, 251, 165);
/// Color `XKCD_BORING_GREEN` from the set CSS4_COLORS. (Color number `199`)
pub const XKCD_BORING_GREEN: Color = Color(99, 179, 101);
/// Color `XKCD_KIWI_GREEN` from the set CSS4_COLORS. (Color number `200`)
pub const XKCD_KIWI_GREEN: Color = Color(142, 229, 63);
/// Color `XKCD_LIGHT_GREY_GREEN` from the set CSS4_COLORS. (Color number `201`)
pub const XKCD_LIGHT_GREY_GREEN: Color = Color(183, 225, 161);
/// Color `XKCD_ORANGE_PINK` from the set CSS4_COLORS. (Color number `202`)
pub const XKCD_ORANGE_PINK: Color = Color(255, 111, 82);
/// Color `XKCD_TEA_GREEN` from the set CSS4_COLORS. (Color number `203`)
pub const XKCD_TEA_GREEN: Color = Color(189, 248, 163);
/// Color `XKCD_VERY_LIGHT_BROWN` from the set CSS4_COLORS. (Color number `204`)
pub const XKCD_VERY_LIGHT_BROWN: Color = Color(211, 182, 131);
/// Color `XKCD_EGG_SHELL` from the set CSS4_COLORS. (Color number `205`)
pub const XKCD_EGG_SHELL: Color = Color(255, 252, 196);
/// Color `XKCD_EGGPLANT_PURPLE` from the set CSS4_COLORS. (Color number `206`)
pub const XKCD_EGGPLANT_PURPLE: Color = Color(67, 5, 65);
/// Color `XKCD_POWDER_PINK` from the set CSS4_COLORS. (Color number `207`)
pub const XKCD_POWDER_PINK: Color = Color(255, 178, 208);
/// Color `XKCD_REDDISH_GREY` from the set CSS4_COLORS. (Color number `208`)
pub const XKCD_REDDISH_GREY: Color = Color(153, 117, 112);
/// Color `XKCD_BABY_SHIT_BROWN` from the set CSS4_COLORS. (Color number `209`)
pub const XKCD_BABY_SHIT_BROWN: Color = Color(173, 144, 13);
/// Color `XKCD_LILIAC` from the set CSS4_COLORS. (Color number `210`)
pub const XKCD_LILIAC: Color = Color(196, 142, 253);
/// Color `XKCD_STORMY_BLUE` from the set CSS4_COLORS. (Color number `211`)
pub const XKCD_STORMY_BLUE: Color = Color(80, 123, 156);
/// Color `XKCD_UGLY_BROWN` from the set CSS4_COLORS. (Color number `212`)
pub const XKCD_UGLY_BROWN: Color = Color(125, 113, 3);
/// Color `XKCD_CUSTARD` from the set CSS4_COLORS. (Color number `213`)
pub const XKCD_CUSTARD: Color = Color(255, 253, 120);
/// Color `XKCD_DARKISH_PINK` from the set CSS4_COLORS. (Color number `214`)
pub const XKCD_DARKISH_PINK: Color = Color(218, 70, 125);
/// Color `XKCD_DEEP_BROWN` from the set CSS4_COLORS. (Color number `215`)
pub const XKCD_DEEP_BROWN: Color = Color(65, 2, 0);
/// Color `XKCD_GREENISH_BEIGE` from the set CSS4_COLORS. (Color number `216`)
pub const XKCD_GREENISH_BEIGE: Color = Color(201, 209, 121);
/// Color `XKCD_MANILLA` from the set CSS4_COLORS. (Color number `217`)
pub const XKCD_MANILLA: Color = Color(255, 250, 134);
/// Color `XKCD_OFF_BLUE` from the set CSS4_COLORS. (Color number `218`)
pub const XKCD_OFF_BLUE: Color = Color(86, 132, 174);
/// Color `XKCD_BATTLESHIP_GREY` from the set CSS4_COLORS. (Color number `219`)
pub const XKCD_BATTLESHIP_GREY: Color = Color(107, 124, 133);
/// Color `XKCD_BROWNY_GREEN` from the set CSS4_COLORS. (Color number `220`)
pub const XKCD_BROWNY_GREEN: Color = Color(111, 108, 10);
/// Color `XKCD_BRUISE` from the set CSS4_COLORS. (Color number `221`)
pub const XKCD_BRUISE: Color = Color(126, 64, 113);
/// Color `XKCD_KELLEY_GREEN` from the set CSS4_COLORS. (Color number `222`)
pub const XKCD_KELLEY_GREEN: Color = Color(0, 147, 55);
/// Color `XKCD_SICKLY_YELLOW` from the set CSS4_COLORS. (Color number `223`)
pub const XKCD_SICKLY_YELLOW: Color = Color(208, 228, 41);
/// Color `XKCD_SUNNY_YELLOW` from the set CSS4_COLORS. (Color number `224`)
pub const XKCD_SUNNY_YELLOW: Color = Color(255, 249, 23);
/// Color `XKCD_AZUL` from the set CSS4_COLORS. (Color number `225`)
pub const XKCD_AZUL: Color = Color(29, 93, 236);
/// Color `XKCD_DARKGREEN` from the set CSS4_COLORS. (Color number `226`)
pub const XKCD_DARKGREEN: Color = Color(5, 73, 7);
/// Color `XKCD_GREEN_YELLOW` from the set CSS4_COLORS. (Color number `227`)
pub const XKCD_GREEN_YELLOW: Color = Color(181, 206, 8);
/// Color `XKCD_LICHEN` from the set CSS4_COLORS. (Color number `228`)
pub const XKCD_LICHEN: Color = Color(143, 182, 123);
/// Color `XKCD_LIGHT_LIGHT_GREEN` from the set CSS4_COLORS. (Color number `229`)
pub const XKCD_LIGHT_LIGHT_GREEN: Color = Color(200, 255, 176);
/// Color `XKCD_PALE_GOLD` from the set CSS4_COLORS. (Color number `230`)
pub const XKCD_PALE_GOLD: Color = Color(253, 222, 108);
/// Color `XKCD_SUN_YELLOW` from the set CSS4_COLORS. (Color number `231`)
pub const XKCD_SUN_YELLOW: Color = Color(255, 223, 34);
/// Color `XKCD_TAN_GREEN` from the set CSS4_COLORS. (Color number `232`)
pub const XKCD_TAN_GREEN: Color = Color(169, 190, 112);
/// Color `XKCD_BURPLE` from the set CSS4_COLORS. (Color number `233`)
pub const XKCD_BURPLE: Color = Color(104, 50, 227);
/// Color `XKCD_BUTTERSCOTCH` from the set CSS4_COLORS. (Color number `234`)
pub const XKCD_BUTTERSCOTCH: Color = Color(253, 177, 71);
/// Color `XKCD_TOUPE` from the set CSS4_COLORS. (Color number `235`)
pub const XKCD_TOUPE: Color = Color(199, 172, 125);
/// Color `XKCD_DARK_CREAM` from the set CSS4_COLORS. (Color number `236`)
pub const XKCD_DARK_CREAM: Color = Color(255, 243, 154);
/// Color `XKCD_INDIAN_RED` from the set CSS4_COLORS. (Color number `237`)
pub const XKCD_INDIAN_RED: Color = Color(133, 14, 4);
/// Color `XKCD_LIGHT_LAVENDAR` from the set CSS4_COLORS. (Color number `238`)
pub const XKCD_LIGHT_LAVENDAR: Color = Color(239, 192, 254);
/// Color `XKCD_POISON_GREEN` from the set CSS4_COLORS. (Color number `239`)
pub const XKCD_POISON_GREEN: Color = Color(64, 253, 20);
/// Color `XKCD_BABY_PUKE_GREEN` from the set CSS4_COLORS. (Color number `240`)
pub const XKCD_BABY_PUKE_GREEN: Color = Color(182, 196, 6);
/// Color `XKCD_BRIGHT_YELLOW_GREEN` from the set CSS4_COLORS. (Color number `241`)
pub const XKCD_BRIGHT_YELLOW_GREEN: Color = Color(157, 255, 0);
/// Color `XKCD_CHARCOAL_GREY` from the set CSS4_COLORS. (Color number `242`)
pub const XKCD_CHARCOAL_GREY: Color = Color(60, 65, 66);
/// Color `XKCD_SQUASH` from the set CSS4_COLORS. (Color number `243`)
pub const XKCD_SQUASH: Color = Color(242, 171, 21);
/// Color `XKCD_CINNAMON` from the set CSS4_COLORS. (Color number `244`)
pub const XKCD_CINNAMON: Color = Color(172, 79, 6);
/// Color `XKCD_LIGHT_PEA_GREEN` from the set CSS4_COLORS. (Color number `245`)
pub const XKCD_LIGHT_PEA_GREEN: Color = Color(196, 254, 130);
/// Color `XKCD_RADIOACTIVE_GREEN` from the set CSS4_COLORS. (Color number `246`)
pub const XKCD_RADIOACTIVE_GREEN: Color = Color(44, 250, 31);
/// Color `XKCD_RAW_SIENNA` from the set CSS4_COLORS. (Color number `247`)
pub const XKCD_RAW_SIENNA: Color = Color(154, 98, 0);
/// Color `XKCD_BABY_PURPLE` from the set CSS4_COLORS. (Color number `248`)
pub const XKCD_BABY_PURPLE: Color = Color(202, 155, 247);
/// Color `XKCD_COCOA` from the set CSS4_COLORS. (Color number `249`)
pub const XKCD_COCOA: Color = Color(135, 95, 66);
/// Color `XKCD_LIGHT_ROYAL_BLUE` from the set CSS4_COLORS. (Color number `250`)
pub const XKCD_LIGHT_ROYAL_BLUE: Color = Color(58, 46, 254);
/// Color `XKCD_ORANGEISH` from the set CSS4_COLORS. (Color number `251`)
pub const XKCD_ORANGEISH: Color = Color(253, 141, 73);
/// Color `XKCD_RUST_BROWN` from the set CSS4_COLORS. (Color number `252`)
pub const XKCD_RUST_BROWN: Color = Color(139, 49, 3);
/// Color `XKCD_SAND_BROWN` from the set CSS4_COLORS. (Color number `253`)
pub const XKCD_SAND_BROWN: Color = Color(203, 165, 96);
/// Color `XKCD_SWAMP` from the set CSS4_COLORS. (Color number `254`)
pub const XKCD_SWAMP: Color = Color(105, 131, 57);
/// Color `XKCD_TEALISH_GREEN` from the set CSS4_COLORS. (Color number `255`)
pub const XKCD_TEALISH_GREEN: Color = Color(12, 220, 115);
/// Color `XKCD_BURNT_SIENA` from the set CSS4_COLORS. (Color number `256`)
pub const XKCD_BURNT_SIENA: Color = Color(183, 82, 3);
/// Color `XKCD_CAMO` from the set CSS4_COLORS. (Color number `257`)
pub const XKCD_CAMO: Color = Color(127, 143, 78);
/// Color `XKCD_DUSK_BLUE` from the set CSS4_COLORS. (Color number `258`)
pub const XKCD_DUSK_BLUE: Color = Color(38, 83, 141);
/// Color `XKCD_FERN` from the set CSS4_COLORS. (Color number `259`)
pub const XKCD_FERN: Color = Color(99, 169, 80);
/// Color `XKCD_OLD_ROSE` from the set CSS4_COLORS. (Color number `260`)
pub const XKCD_OLD_ROSE: Color = Color(200, 127, 137);
/// Color `XKCD_PALE_LIGHT_GREEN` from the set CSS4_COLORS. (Color number `261`)
pub const XKCD_PALE_LIGHT_GREEN: Color = Color(177, 252, 153);
/// Color `XKCD_PEACHY_PINK` from the set CSS4_COLORS. (Color number `262`)
pub const XKCD_PEACHY_PINK: Color = Color(255, 154, 138);
/// Color `XKCD_ROSY_PINK` from the set CSS4_COLORS. (Color number `263`)
pub const XKCD_ROSY_PINK: Color = Color(246, 104, 142);
/// Color `XKCD_LIGHT_BLUISH_GREEN` from the set CSS4_COLORS. (Color number `264`)
pub const XKCD_LIGHT_BLUISH_GREEN: Color = Color(118, 253, 168);
/// Color `XKCD_LIGHT_BRIGHT_GREEN` from the set CSS4_COLORS. (Color number `265`)
pub const XKCD_LIGHT_BRIGHT_GREEN: Color = Color(83, 254, 92);
/// Color `XKCD_LIGHT_NEON_GREEN` from the set CSS4_COLORS. (Color number `266`)
pub const XKCD_LIGHT_NEON_GREEN: Color = Color(78, 253, 84);
/// Color `XKCD_LIGHT_SEAFOAM` from the set CSS4_COLORS. (Color number `267`)
pub const XKCD_LIGHT_SEAFOAM: Color = Color(160, 254, 191);
/// Color `XKCD_TIFFANY_BLUE` from the set CSS4_COLORS. (Color number `268`)
pub const XKCD_TIFFANY_BLUE: Color = Color(123, 242, 218);
/// Color `XKCD_WASHED_OUT_GREEN` from the set CSS4_COLORS. (Color number `269`)
pub const XKCD_WASHED_OUT_GREEN: Color = Color(188, 245, 166);
/// Color `XKCD_BROWNY_ORANGE` from the set CSS4_COLORS. (Color number `270`)
pub const XKCD_BROWNY_ORANGE: Color = Color(202, 107, 2);
/// Color `XKCD_NICE_BLUE` from the set CSS4_COLORS. (Color number `271`)
pub const XKCD_NICE_BLUE: Color = Color(16, 122, 176);
/// Color `XKCD_SAPPHIRE` from the set CSS4_COLORS. (Color number `272`)
pub const XKCD_SAPPHIRE: Color = Color(33, 56, 171);
/// Color `XKCD_GREYISH_TEAL` from the set CSS4_COLORS. (Color number `273`)
pub const XKCD_GREYISH_TEAL: Color = Color(113, 159, 145);
/// Color `XKCD_ORANGEY_YELLOW` from the set CSS4_COLORS. (Color number `274`)
pub const XKCD_ORANGEY_YELLOW: Color = Color(253, 185, 21);
/// Color `XKCD_PARCHMENT` from the set CSS4_COLORS. (Color number `275`)
pub const XKCD_PARCHMENT: Color = Color(254, 252, 175);
/// Color `XKCD_STRAW` from the set CSS4_COLORS. (Color number `276`)
pub const XKCD_STRAW: Color = Color(252, 246, 121);
/// Color `XKCD_VERY_DARK_BROWN` from the set CSS4_COLORS. (Color number `277`)
pub const XKCD_VERY_DARK_BROWN: Color = Color(29, 2, 0);
/// Color `XKCD_TERRACOTA` from the set CSS4_COLORS. (Color number `278`)
pub const XKCD_TERRACOTA: Color = Color(203, 104, 67);
/// Color `XKCD_UGLY_BLUE` from the set CSS4_COLORS. (Color number `279`)
pub const XKCD_UGLY_BLUE: Color = Color(49, 102, 138);
/// Color `XKCD_CLEAR_BLUE` from the set CSS4_COLORS. (Color number `280`)
pub const XKCD_CLEAR_BLUE: Color = Color(36, 122, 253);
/// Color `XKCD_CREME` from the set CSS4_COLORS. (Color number `281`)
pub const XKCD_CREME: Color = Color(255, 255, 182);
/// Color `XKCD_FOAM_GREEN` from the set CSS4_COLORS. (Color number `282`)
pub const XKCD_FOAM_GREEN: Color = Color(144, 253, 169);
/// Color `XKCD_GREY_GREEN` from the set CSS4_COLORS. (Color number `283`)
pub const XKCD_GREY_GREEN: Color = Color(134, 161, 125);
/// Color `XKCD_LIGHT_GOLD` from the set CSS4_COLORS. (Color number `284`)
pub const XKCD_LIGHT_GOLD: Color = Color(253, 220, 92);
/// Color `XKCD_SEAFOAM_BLUE` from the set CSS4_COLORS. (Color number `285`)
pub const XKCD_SEAFOAM_BLUE: Color = Color(120, 209, 182);
/// Color `XKCD_TOPAZ` from the set CSS4_COLORS. (Color number `286`)
pub const XKCD_TOPAZ: Color = Color(19, 187, 175);
/// Color `XKCD_VIOLET_PINK` from the set CSS4_COLORS. (Color number `287`)
pub const XKCD_VIOLET_PINK: Color = Color(251, 95, 252);
/// Color `XKCD_WINTERGREEN` from the set CSS4_COLORS. (Color number `288`)
pub const XKCD_WINTERGREEN: Color = Color(32, 249, 134);
/// Color `XKCD_YELLOW_TAN` from the set CSS4_COLORS. (Color number `289`)
pub const XKCD_YELLOW_TAN: Color = Color(255, 227, 110);
/// Color `XKCD_DARK_FUCHSIA` from the set CSS4_COLORS. (Color number `290`)
pub const XKCD_DARK_FUCHSIA: Color = Color(157, 7, 89);
/// Color `XKCD_INDIGO_BLUE` from the set CSS4_COLORS. (Color number `291`)
pub const XKCD_INDIGO_BLUE: Color = Color(58, 24, 177);
/// Color `XKCD_LIGHT_YELLOWISH_GREEN` from the set CSS4_COLORS. (Color number `292`)
pub const XKCD_LIGHT_YELLOWISH_GREEN: Color = Color(194, 255, 137);
/// Color `XKCD_PALE_MAGENTA` from the set CSS4_COLORS. (Color number `293`)
pub const XKCD_PALE_MAGENTA: Color = Color(215, 103, 173);
/// Color `XKCD_RICH_PURPLE` from the set CSS4_COLORS. (Color number `294`)
pub const XKCD_RICH_PURPLE: Color = Color(114, 0, 88);
/// Color `XKCD_SUNFLOWER_YELLOW` from the set CSS4_COLORS. (Color number `295`)
pub const XKCD_SUNFLOWER_YELLOW: Color = Color(255, 218, 3);
/// Color `XKCD_GREEN_BLUE` from the set CSS4_COLORS. (Color number `296`)
pub const XKCD_GREEN_BLUE: Color = Color(1, 192, 141);
/// Color `XKCD_LEATHER` from the set CSS4_COLORS. (Color number `297`)
pub const XKCD_LEATHER: Color = Color(172, 116, 52);
/// Color `XKCD_RACING_GREEN` from the set CSS4_COLORS. (Color number `298`)
pub const XKCD_RACING_GREEN: Color = Color(1, 70, 0);
/// Color `XKCD_VIVID_PURPLE` from the set CSS4_COLORS. (Color number `299`)
pub const XKCD_VIVID_PURPLE: Color = Color(153, 0, 250);
/// Color `XKCD_DARK_ROYAL_BLUE` from the set CSS4_COLORS. (Color number `300`)
pub const XKCD_DARK_ROYAL_BLUE: Color = Color(2, 6, 111);
/// Color `XKCD_HAZEL` from the set CSS4_COLORS. (Color number `301`)
pub const XKCD_HAZEL: Color = Color(142, 118, 24);
/// Color `XKCD_MUTED_PINK` from the set CSS4_COLORS. (Color number `302`)
pub const XKCD_MUTED_PINK: Color = Color(209, 118, 143);
/// Color `XKCD_BOOGER_GREEN` from the set CSS4_COLORS. (Color number `303`)
pub const XKCD_BOOGER_GREEN: Color = Color(150, 180, 3);
/// Color `XKCD_CANARY` from the set CSS4_COLORS. (Color number `304`)
pub const XKCD_CANARY: Color = Color(253, 255, 99);
/// Color `XKCD_COOL_GREY` from the set CSS4_COLORS. (Color number `305`)
pub const XKCD_COOL_GREY: Color = Color(149, 163, 166);
/// Color `XKCD_DARK_TAUPE` from the set CSS4_COLORS. (Color number `306`)
pub const XKCD_DARK_TAUPE: Color = Color(127, 104, 78);
/// Color `XKCD_DARKISH_PURPLE` from the set CSS4_COLORS. (Color number `307`)
pub const XKCD_DARKISH_PURPLE: Color = Color(117, 25, 115);
/// Color `XKCD_TRUE_GREEN` from the set CSS4_COLORS. (Color number `308`)
pub const XKCD_TRUE_GREEN: Color = Color(8, 148, 4);
/// Color `XKCD_CORAL_PINK` from the set CSS4_COLORS. (Color number `309`)
pub const XKCD_CORAL_PINK: Color = Color(255, 97, 99);
/// Color `XKCD_DARK_SAGE` from the set CSS4_COLORS. (Color number `310`)
pub const XKCD_DARK_SAGE: Color = Color(89, 133, 86);
/// Color `XKCD_DARK_SLATE_BLUE` from the set CSS4_COLORS. (Color number `311`)
pub const XKCD_DARK_SLATE_BLUE: Color = Color(33, 71, 97);
/// Color `XKCD_FLAT_BLUE` from the set CSS4_COLORS. (Color number `312`)
pub const XKCD_FLAT_BLUE: Color = Color(60, 115, 168);
/// Color `XKCD_MUSHROOM` from the set CSS4_COLORS. (Color number `313`)
pub const XKCD_MUSHROOM: Color = Color(186, 158, 136);
/// Color `XKCD_RICH_BLUE` from the set CSS4_COLORS. (Color number `314`)
pub const XKCD_RICH_BLUE: Color = Color(2, 27, 249);
/// Color `XKCD_DIRTY_PURPLE` from the set CSS4_COLORS. (Color number `315`)
pub const XKCD_DIRTY_PURPLE: Color = Color(115, 74, 101);
/// Color `XKCD_GREENBLUE` from the set CSS4_COLORS. (Color number `316`)
pub const XKCD_GREENBLUE: Color = Color(35, 196, 139);
/// Color `XKCD_ICKY_GREEN` from the set CSS4_COLORS. (Color number `317`)
pub const XKCD_ICKY_GREEN: Color = Color(143, 174, 34);
/// Color `XKCD_LIGHT_KHAKI` from the set CSS4_COLORS. (Color number `318`)
pub const XKCD_LIGHT_KHAKI: Color = Color(230, 242, 162);
/// Color `XKCD_WARM_BLUE` from the set CSS4_COLORS. (Color number `319`)
pub const XKCD_WARM_BLUE: Color = Color(75, 87, 219);
/// Color `XKCD_DARK_HOT_PINK` from the set CSS4_COLORS. (Color number `320`)
pub const XKCD_DARK_HOT_PINK: Color = Color(217, 1, 102);
/// Color `XKCD_DEEP_SEA_BLUE` from the set CSS4_COLORS. (Color number `321`)
pub const XKCD_DEEP_SEA_BLUE: Color = Color(1, 84, 130);
/// Color `XKCD_CARMINE` from the set CSS4_COLORS. (Color number `322`)
pub const XKCD_CARMINE: Color = Color(157, 2, 22);
/// Color `XKCD_DARK_YELLOW_GREEN` from the set CSS4_COLORS. (Color number `323`)
pub const XKCD_DARK_YELLOW_GREEN: Color = Color(114, 143, 2);
/// Color `XKCD_PALE_PEACH` from the set CSS4_COLORS. (Color number `324`)
pub const XKCD_PALE_PEACH: Color = Color(255, 229, 173);
/// Color `XKCD_PLUM_PURPLE` from the set CSS4_COLORS. (Color number `325`)
pub const XKCD_PLUM_PURPLE: Color = Color(78, 5, 80);
/// Color `XKCD_GOLDEN_ROD` from the set CSS4_COLORS. (Color number `326`)
pub const XKCD_GOLDEN_ROD: Color = Color(249, 188, 8);
/// Color `XKCD_NEON_RED` from the set CSS4_COLORS. (Color number `327`)
pub const XKCD_NEON_RED: Color = Color(255, 7, 58);
/// Color `XKCD_OLD_PINK` from the set CSS4_COLORS. (Color number `328`)
pub const XKCD_OLD_PINK: Color = Color(199, 121, 134);
/// Color `XKCD_VERY_PALE_BLUE` from the set CSS4_COLORS. (Color number `329`)
pub const XKCD_VERY_PALE_BLUE: Color = Color(214, 255, 254);
/// Color `XKCD_BLOOD_ORANGE` from the set CSS4_COLORS. (Color number `330`)
pub const XKCD_BLOOD_ORANGE: Color = Color(254, 75, 3);
/// Color `XKCD_GRAPEFRUIT` from the set CSS4_COLORS. (Color number `331`)
pub const XKCD_GRAPEFRUIT: Color = Color(253, 89, 86);
/// Color `XKCD_SAND_YELLOW` from the set CSS4_COLORS. (Color number `332`)
pub const XKCD_SAND_YELLOW: Color = Color(252, 225, 102);
/// Color `XKCD_CLAY_BROWN` from the set CSS4_COLORS. (Color number `333`)
pub const XKCD_CLAY_BROWN: Color = Color(178, 113, 61);
/// Color `XKCD_DARK_BLUE_GREY` from the set CSS4_COLORS. (Color number `334`)
pub const XKCD_DARK_BLUE_GREY: Color = Color(31, 59, 77);
/// Color `XKCD_FLAT_GREEN` from the set CSS4_COLORS. (Color number `335`)
pub const XKCD_FLAT_GREEN: Color = Color(105, 157, 76);
/// Color `XKCD_LIGHT_GREEN_BLUE` from the set CSS4_COLORS. (Color number `336`)
pub const XKCD_LIGHT_GREEN_BLUE: Color = Color(86, 252, 162);
/// Color `XKCD_WARM_PINK` from the set CSS4_COLORS. (Color number `337`)
pub const XKCD_WARM_PINK: Color = Color(251, 85, 129);
/// Color `XKCD_DODGER_BLUE` from the set CSS4_COLORS. (Color number `338`)
pub const XKCD_DODGER_BLUE: Color = Color(62, 130, 252);
/// Color `XKCD_GROSS_GREEN` from the set CSS4_COLORS. (Color number `339`)
pub const XKCD_GROSS_GREEN: Color = Color(160, 191, 22);
/// Color `XKCD_ICE` from the set CSS4_COLORS. (Color number `340`)
pub const XKCD_ICE: Color = Color(214, 255, 250);
/// Color `XKCD_METALLIC_BLUE` from the set CSS4_COLORS. (Color number `341`)
pub const XKCD_METALLIC_BLUE: Color = Color(79, 115, 142);
/// Color `XKCD_PALE_SALMON` from the set CSS4_COLORS. (Color number `342`)
pub const XKCD_PALE_SALMON: Color = Color(255, 177, 154);
/// Color `XKCD_SAP_GREEN` from the set CSS4_COLORS. (Color number `343`)
pub const XKCD_SAP_GREEN: Color = Color(92, 139, 21);
/// Color `XKCD_ALGAE` from the set CSS4_COLORS. (Color number `344`)
pub const XKCD_ALGAE: Color = Color(84, 172, 104);
/// Color `XKCD_BLUEY_GREY` from the set CSS4_COLORS. (Color number `345`)
pub const XKCD_BLUEY_GREY: Color = Color(137, 160, 176);
/// Color `XKCD_GREENY_GREY` from the set CSS4_COLORS. (Color number `346`)
pub const XKCD_GREENY_GREY: Color = Color(126, 160, 122);
/// Color `XKCD_HIGHLIGHTER_GREEN` from the set CSS4_COLORS. (Color number `347`)
pub const XKCD_HIGHLIGHTER_GREEN: Color = Color(27, 252, 6);
/// Color `XKCD_LIGHT_LIGHT_BLUE` from the set CSS4_COLORS. (Color number `348`)
pub const XKCD_LIGHT_LIGHT_BLUE: Color = Color(202, 255, 251);
/// Color `XKCD_LIGHT_MINT` from the set CSS4_COLORS. (Color number `349`)
pub const XKCD_LIGHT_MINT: Color = Color(182, 255, 187);
/// Color `XKCD_RAW_UMBER` from the set CSS4_COLORS. (Color number `350`)
pub const XKCD_RAW_UMBER: Color = Color(167, 94, 9);
/// Color `XKCD_VIVID_BLUE` from the set CSS4_COLORS. (Color number `351`)
pub const XKCD_VIVID_BLUE: Color = Color(21, 46, 255);
/// Color `XKCD_DEEP_LAVENDER` from the set CSS4_COLORS. (Color number `352`)
pub const XKCD_DEEP_LAVENDER: Color = Color(141, 94, 183);
/// Color `XKCD_DULL_TEAL` from the set CSS4_COLORS. (Color number `353`)
pub const XKCD_DULL_TEAL: Color = Color(95, 158, 143);
/// Color `XKCD_LIGHT_GREENISH_BLUE` from the set CSS4_COLORS. (Color number `354`)
pub const XKCD_LIGHT_GREENISH_BLUE: Color = Color(99, 247, 180);
/// Color `XKCD_MUD_GREEN` from the set CSS4_COLORS. (Color number `355`)
pub const XKCD_MUD_GREEN: Color = Color(96, 102, 2);
/// Color `XKCD_PINKY` from the set CSS4_COLORS. (Color number `356`)
pub const XKCD_PINKY: Color = Color(252, 134, 170);
/// Color `XKCD_RED_WINE` from the set CSS4_COLORS. (Color number `357`)
pub const XKCD_RED_WINE: Color = Color(140, 0, 52);
/// Color `XKCD_SHIT_GREEN` from the set CSS4_COLORS. (Color number `358`)
pub const XKCD_SHIT_GREEN: Color = Color(117, 128, 0);
/// Color `XKCD_TAN_BROWN` from the set CSS4_COLORS. (Color number `359`)
pub const XKCD_TAN_BROWN: Color = Color(171, 126, 76);
/// Color `XKCD_DARKBLUE` from the set CSS4_COLORS. (Color number `360`)
pub const XKCD_DARKBLUE: Color = Color(3, 7, 100);
/// Color `XKCD_ROSA` from the set CSS4_COLORS. (Color number `361`)
pub const XKCD_ROSA: Color = Color(254, 134, 164);
/// Color `XKCD_LIPSTICK` from the set CSS4_COLORS. (Color number `362`)
pub const XKCD_LIPSTICK: Color = Color(213, 23, 78);
/// Color `XKCD_PALE_MAUVE` from the set CSS4_COLORS. (Color number `363`)
pub const XKCD_PALE_MAUVE: Color = Color(254, 208, 252);
/// Color `XKCD_CLARET` from the set CSS4_COLORS. (Color number `364`)
pub const XKCD_CLARET: Color = Color(104, 0, 24);
/// Color `XKCD_DANDELION` from the set CSS4_COLORS. (Color number `365`)
pub const XKCD_DANDELION: Color = Color(254, 223, 8);
/// Color `XKCD_ORANGERED` from the set CSS4_COLORS. (Color number `366`)
pub const XKCD_ORANGERED: Color = Color(254, 66, 15);
/// Color `XKCD_POOP_GREEN` from the set CSS4_COLORS. (Color number `367`)
pub const XKCD_POOP_GREEN: Color = Color(111, 124, 0);
/// Color `XKCD_RUBY` from the set CSS4_COLORS. (Color number `368`)
pub const XKCD_RUBY: Color = Color(202, 1, 71);
/// Color `XKCD_DARK` from the set CSS4_COLORS. (Color number `369`)
pub const XKCD_DARK: Color = Color(27, 36, 49);
/// Color `XKCD_GREENISH_TURQUOISE` from the set CSS4_COLORS. (Color number `370`)
pub const XKCD_GREENISH_TURQUOISE: Color = Color(0, 251, 176);
/// Color `XKCD_PASTEL_RED` from the set CSS4_COLORS. (Color number `371`)
pub const XKCD_PASTEL_RED: Color = Color(219, 88, 86);
/// Color `XKCD_PISS_YELLOW` from the set CSS4_COLORS. (Color number `372`)
pub const XKCD_PISS_YELLOW: Color = Color(221, 214, 24);
/// Color `XKCD_BRIGHT_CYAN` from the set CSS4_COLORS. (Color number `373`)
pub const XKCD_BRIGHT_CYAN: Color = Color(65, 253, 254);
/// Color `XKCD_DARK_CORAL` from the set CSS4_COLORS. (Color number `374`)
pub const XKCD_DARK_CORAL: Color = Color(207, 82, 78);
/// Color `XKCD_ALGAE_GREEN` from the set CSS4_COLORS. (Color number `375`)
pub const XKCD_ALGAE_GREEN: Color = Color(33, 195, 111);
/// Color `XKCD_DARKISH_RED` from the set CSS4_COLORS. (Color number `376`)
pub const XKCD_DARKISH_RED: Color = Color(169, 3, 8);
/// Color `XKCD_REDDY_BROWN` from the set CSS4_COLORS. (Color number `377`)
pub const XKCD_REDDY_BROWN: Color = Color(110, 16, 5);
/// Color `XKCD_BLUSH_PINK` from the set CSS4_COLORS. (Color number `378`)
pub const XKCD_BLUSH_PINK: Color = Color(254, 130, 140);
/// Color `XKCD_CAMOUFLAGE_GREEN` from the set CSS4_COLORS. (Color number `379`)
pub const XKCD_CAMOUFLAGE_GREEN: Color = Color(75, 97, 19);
/// Color `XKCD_LAWN_GREEN` from the set CSS4_COLORS. (Color number `380`)
pub const XKCD_LAWN_GREEN: Color = Color(77, 164, 9);
/// Color `XKCD_PUTTY` from the set CSS4_COLORS. (Color number `381`)
pub const XKCD_PUTTY: Color = Color(190, 174, 138);
/// Color `XKCD_VIBRANT_BLUE` from the set CSS4_COLORS. (Color number `382`)
pub const XKCD_VIBRANT_BLUE: Color = Color(3, 57, 248);
/// Color `XKCD_DARK_SAND` from the set CSS4_COLORS. (Color number `383`)
pub const XKCD_DARK_SAND: Color = Color(168, 143, 89);
/// Color `XKCD_PURPLE_BLUE` from the set CSS4_COLORS. (Color number `384`)
pub const XKCD_PURPLE_BLUE: Color = Color(93, 33, 208);
/// Color `XKCD_SAFFRON` from the set CSS4_COLORS. (Color number `385`)
pub const XKCD_SAFFRON: Color = Color(254, 178, 9);
/// Color `XKCD_TWILIGHT` from the set CSS4_COLORS. (Color number `386`)
pub const XKCD_TWILIGHT: Color = Color(78, 81, 139);
/// Color `XKCD_WARM_BROWN` from the set CSS4_COLORS. (Color number `387`)
pub const XKCD_WARM_BROWN: Color = Color(150, 78, 2);
/// Color `XKCD_BLUEGREY` from the set CSS4_COLORS. (Color number `388`)
pub const XKCD_BLUEGREY: Color = Color(133, 163, 178);
/// Color `XKCD_BUBBLE_GUM_PINK` from the set CSS4_COLORS. (Color number `389`)
pub const XKCD_BUBBLE_GUM_PINK: Color = Color(255, 105, 175);
/// Color `XKCD_DUCK_EGG_BLUE` from the set CSS4_COLORS. (Color number `390`)
pub const XKCD_DUCK_EGG_BLUE: Color = Color(195, 251, 244);
/// Color `XKCD_GREENISH_CYAN` from the set CSS4_COLORS. (Color number `391`)
pub const XKCD_GREENISH_CYAN: Color = Color(42, 254, 183);
/// Color `XKCD_PETROL` from the set CSS4_COLORS. (Color number `392`)
pub const XKCD_PETROL: Color = Color(0, 95, 106);
/// Color `XKCD_ROYAL` from the set CSS4_COLORS. (Color number `393`)
pub const XKCD_ROYAL: Color = Color(12, 23, 147);
/// Color `XKCD_BUTTER` from the set CSS4_COLORS. (Color number `394`)
pub const XKCD_BUTTER: Color = Color(255, 255, 129);
/// Color `XKCD_DUSTY_ORANGE` from the set CSS4_COLORS. (Color number `395`)
pub const XKCD_DUSTY_ORANGE: Color = Color(240, 131, 58);
/// Color `XKCD_OFF_YELLOW` from the set CSS4_COLORS. (Color number `396`)
pub const XKCD_OFF_YELLOW: Color = Color(241, 243, 63);
/// Color `XKCD_PALE_OLIVE_GREEN` from the set CSS4_COLORS. (Color number `397`)
pub const XKCD_PALE_OLIVE_GREEN: Color = Color(177, 210, 123);
/// Color `XKCD_ORANGISH` from the set CSS4_COLORS. (Color number `398`)
pub const XKCD_ORANGISH: Color = Color(252, 130, 74);
/// Color `XKCD_LEAF` from the set CSS4_COLORS. (Color number `399`)
pub const XKCD_LEAF: Color = Color(113, 170, 52);
/// Color `XKCD_LIGHT_BLUE_GREY` from the set CSS4_COLORS. (Color number `400`)
pub const XKCD_LIGHT_BLUE_GREY: Color = Color(183, 201, 226);
/// Color `XKCD_DRIED_BLOOD` from the set CSS4_COLORS. (Color number `401`)
pub const XKCD_DRIED_BLOOD: Color = Color(75, 1, 1);
/// Color `XKCD_LIGHTISH_PURPLE` from the set CSS4_COLORS. (Color number `402`)
pub const XKCD_LIGHTISH_PURPLE: Color = Color(165, 82, 230);
/// Color `XKCD_RUSTY_RED` from the set CSS4_COLORS. (Color number `403`)
pub const XKCD_RUSTY_RED: Color = Color(175, 47, 13);
/// Color `XKCD_LAVENDER_BLUE` from the set CSS4_COLORS. (Color number `404`)
pub const XKCD_LAVENDER_BLUE: Color = Color(139, 136, 248);
/// Color `XKCD_LIGHT_GRASS_GREEN` from the set CSS4_COLORS. (Color number `405`)
pub const XKCD_LIGHT_GRASS_GREEN: Color = Color(154, 247, 100);
/// Color `XKCD_LIGHT_MINT_GREEN` from the set CSS4_COLORS. (Color number `406`)
pub const XKCD_LIGHT_MINT_GREEN: Color = Color(166, 251, 178);
/// Color `XKCD_SUNFLOWER` from the set CSS4_COLORS. (Color number `407`)
pub const XKCD_SUNFLOWER: Color = Color(255, 197, 18);
/// Color `XKCD_VELVET` from the set CSS4_COLORS. (Color number `408`)
pub const XKCD_VELVET: Color = Color(117, 8, 81);
/// Color `XKCD_BRICK_ORANGE` from the set CSS4_COLORS. (Color number `409`)
pub const XKCD_BRICK_ORANGE: Color = Color(193, 74, 9);
/// Color `XKCD_LIGHTISH_RED` from the set CSS4_COLORS. (Color number `410`)
pub const XKCD_LIGHTISH_RED: Color = Color(254, 47, 74);
/// Color `XKCD_PURE_BLUE` from the set CSS4_COLORS. (Color number `411`)
pub const XKCD_PURE_BLUE: Color = Color(2, 3, 226);
/// Color `XKCD_TWILIGHT_BLUE` from the set CSS4_COLORS. (Color number `412`)
pub const XKCD_TWILIGHT_BLUE: Color = Color(10, 67, 122);
/// Color `XKCD_VIOLET_RED` from the set CSS4_COLORS. (Color number `413`)
pub const XKCD_VIOLET_RED: Color = Color(165, 0, 85);
/// Color `XKCD_YELLOWY_BROWN` from the set CSS4_COLORS. (Color number `414`)
pub const XKCD_YELLOWY_BROWN: Color = Color(174, 139, 12);
/// Color `XKCD_CARNATION` from the set CSS4_COLORS. (Color number `415`)
pub const XKCD_CARNATION: Color = Color(253, 121, 143);
/// Color `XKCD_MUDDY_YELLOW` from the set CSS4_COLORS. (Color number `416`)
pub const XKCD_MUDDY_YELLOW: Color = Color(191, 172, 5);
/// Color `XKCD_DARK_SEAFOAM_GREEN` from the set CSS4_COLORS. (Color number `417`)
pub const XKCD_DARK_SEAFOAM_GREEN: Color = Color(62, 175, 118);
/// Color `XKCD_DEEP_ROSE` from the set CSS4_COLORS. (Color number `418`)
pub const XKCD_DEEP_ROSE: Color = Color(199, 71, 103);
/// Color `XKCD_DUSTY_RED` from the set CSS4_COLORS. (Color number `419`)
pub const XKCD_DUSTY_RED: Color = Color(185, 72, 78);
/// Color `XKCD_GREY_BLUE` from the set CSS4_COLORS. (Color number `420`)
pub const XKCD_GREY_BLUE: Color = Color(100, 125, 142);
/// Color `XKCD_LEMON_LIME` from the set CSS4_COLORS. (Color number `421`)
pub const XKCD_LEMON_LIME: Color = Color(191, 254, 40);
/// Color `XKCD_PURPLE_PINK` from the set CSS4_COLORS. (Color number `422`)
pub const XKCD_PURPLE_PINK: Color = Color(215, 37, 222);
/// Color `XKCD_BROWN_YELLOW` from the set CSS4_COLORS. (Color number `423`)
pub const XKCD_BROWN_YELLOW: Color = Color(178, 151, 5);
/// Color `XKCD_PURPLE_BROWN` from the set CSS4_COLORS. (Color number `424`)
pub const XKCD_PURPLE_BROWN: Color = Color(103, 58, 63);
/// Color `XKCD_WISTERIA` from the set CSS4_COLORS. (Color number `425`)
pub const XKCD_WISTERIA: Color = Color(168, 125, 194);
/// Color `XKCD_BANANA_YELLOW` from the set CSS4_COLORS. (Color number `426`)
pub const XKCD_BANANA_YELLOW: Color = Color(250, 254, 75);
/// Color `XKCD_LIPSTICK_RED` from the set CSS4_COLORS. (Color number `427`)
pub const XKCD_LIPSTICK_RED: Color = Color(192, 2, 47);
/// Color `XKCD_WATER_BLUE` from the set CSS4_COLORS. (Color number `428`)
pub const XKCD_WATER_BLUE: Color = Color(14, 135, 204);
/// Color `XKCD_BROWN_GREY` from the set CSS4_COLORS. (Color number `429`)
pub const XKCD_BROWN_GREY: Color = Color(141, 132, 104);
/// Color `XKCD_VIBRANT_PURPLE` from the set CSS4_COLORS. (Color number `430`)
pub const XKCD_VIBRANT_PURPLE: Color = Color(173, 3, 222);
/// Color `XKCD_BABY_GREEN` from the set CSS4_COLORS. (Color number `431`)
pub const XKCD_BABY_GREEN: Color = Color(140, 255, 158);
/// Color `XKCD_BARF_GREEN` from the set CSS4_COLORS. (Color number `432`)
pub const XKCD_BARF_GREEN: Color = Color(148, 172, 2);
/// Color `XKCD_EGGSHELL_BLUE` from the set CSS4_COLORS. (Color number `433`)
pub const XKCD_EGGSHELL_BLUE: Color = Color(196, 255, 247);
/// Color `XKCD_SANDY_YELLOW` from the set CSS4_COLORS. (Color number `434`)
pub const XKCD_SANDY_YELLOW: Color = Color(253, 238, 115);
/// Color `XKCD_COOL_GREEN` from the set CSS4_COLORS. (Color number `435`)
pub const XKCD_COOL_GREEN: Color = Color(51, 184, 100);
/// Color `XKCD_PALE` from the set CSS4_COLORS. (Color number `436`)
pub const XKCD_PALE: Color = Color(255, 249, 208);
/// Color `XKCD_BLUE_GREY` from the set CSS4_COLORS. (Color number `437`)
pub const XKCD_BLUE_GREY: Color = Color(117, 141, 163);
/// Color `XKCD_HOT_MAGENTA` from the set CSS4_COLORS. (Color number `438`)
pub const XKCD_HOT_MAGENTA: Color = Color(245, 4, 201);
/// Color `XKCD_GREYBLUE` from the set CSS4_COLORS. (Color number `439`)
pub const XKCD_GREYBLUE: Color = Color(119, 161, 181);
/// Color `XKCD_PURPLEY` from the set CSS4_COLORS. (Color number `440`)
pub const XKCD_PURPLEY: Color = Color(135, 86, 228);
/// Color `XKCD_BABY_SHIT_GREEN` from the set CSS4_COLORS. (Color number `441`)
pub const XKCD_BABY_SHIT_GREEN: Color = Color(136, 151, 23);
/// Color `XKCD_BROWNISH_PINK` from the set CSS4_COLORS. (Color number `442`)
pub const XKCD_BROWNISH_PINK: Color = Color(194, 126, 121);
/// Color `XKCD_DARK_AQUAMARINE` from the set CSS4_COLORS. (Color number `443`)
pub const XKCD_DARK_AQUAMARINE: Color = Color(1, 115, 113);
/// Color `XKCD_DIARRHEA` from the set CSS4_COLORS. (Color number `444`)
pub const XKCD_DIARRHEA: Color = Color(159, 131, 3);
/// Color `XKCD_LIGHT_MUSTARD` from the set CSS4_COLORS. (Color number `445`)
pub const XKCD_LIGHT_MUSTARD: Color = Color(247, 213, 96);
/// Color `XKCD_PALE_SKY_BLUE` from the set CSS4_COLORS. (Color number `446`)
pub const XKCD_PALE_SKY_BLUE: Color = Color(189, 246, 254);
/// Color `XKCD_TURTLE_GREEN` from the set CSS4_COLORS. (Color number `447`)
pub const XKCD_TURTLE_GREEN: Color = Color(117, 184, 79);
/// Color `XKCD_BRIGHT_OLIVE` from the set CSS4_COLORS. (Color number `448`)
pub const XKCD_BRIGHT_OLIVE: Color = Color(156, 187, 4);
/// Color `XKCD_DARK_GREY_BLUE` from the set CSS4_COLORS. (Color number `449`)
pub const XKCD_DARK_GREY_BLUE: Color = Color(41, 70, 91);
/// Color `XKCD_GREENY_BROWN` from the set CSS4_COLORS. (Color number `450`)
pub const XKCD_GREENY_BROWN: Color = Color(105, 96, 6);
/// Color `XKCD_LEMON_GREEN` from the set CSS4_COLORS. (Color number `451`)
pub const XKCD_LEMON_GREEN: Color = Color(173, 248, 2);
/// Color `XKCD_LIGHT_PERIWINKLE` from the set CSS4_COLORS. (Color number `452`)
pub const XKCD_LIGHT_PERIWINKLE: Color = Color(193, 198, 252);
/// Color `XKCD_SEAWEED_GREEN` from the set CSS4_COLORS. (Color number `453`)
pub const XKCD_SEAWEED_GREEN: Color = Color(53, 173, 107);
/// Color `XKCD_SUNSHINE_YELLOW` from the set CSS4_COLORS. (Color number `454`)
pub const XKCD_SUNSHINE_YELLOW: Color = Color(255, 253, 55);
/// Color `XKCD_UGLY_PURPLE` from the set CSS4_COLORS. (Color number `455`)
pub const XKCD_UGLY_PURPLE: Color = Color(164, 66, 160);
/// Color `XKCD_MEDIUM_PINK` from the set CSS4_COLORS. (Color number `456`)
pub const XKCD_MEDIUM_PINK: Color = Color(243, 97, 150);
/// Color `XKCD_PUKE_BROWN` from the set CSS4_COLORS. (Color number `457`)
pub const XKCD_PUKE_BROWN: Color = Color(148, 119, 6);
/// Color `XKCD_VERY_LIGHT_PINK` from the set CSS4_COLORS. (Color number `458`)
pub const XKCD_VERY_LIGHT_PINK: Color = Color(255, 244, 242);
/// Color `XKCD_VIRIDIAN` from the set CSS4_COLORS. (Color number `459`)
pub const XKCD_VIRIDIAN: Color = Color(30, 145, 103);
/// Color `XKCD_BILE` from the set CSS4_COLORS. (Color number `460`)
pub const XKCD_BILE: Color = Color(181, 195, 6);
/// Color `XKCD_FADED_YELLOW` from the set CSS4_COLORS. (Color number `461`)
pub const XKCD_FADED_YELLOW: Color = Color(254, 255, 127);
/// Color `XKCD_VERY_PALE_GREEN` from the set CSS4_COLORS. (Color number `462`)
pub const XKCD_VERY_PALE_GREEN: Color = Color(207, 253, 188);
/// Color `XKCD_VIBRANT_GREEN` from the set CSS4_COLORS. (Color number `463`)
pub const XKCD_VIBRANT_GREEN: Color = Color(10, 221, 8);
/// Color `XKCD_BRIGHT_LIME` from the set CSS4_COLORS. (Color number `464`)
pub const XKCD_BRIGHT_LIME: Color = Color(135, 253, 5);
/// Color `XKCD_SPEARMINT` from the set CSS4_COLORS. (Color number `465`)
pub const XKCD_SPEARMINT: Color = Color(30, 248, 118);
/// Color `XKCD_LIGHT_AQUAMARINE` from the set CSS4_COLORS. (Color number `466`)
pub const XKCD_LIGHT_AQUAMARINE: Color = Color(123, 253, 199);
/// Color `XKCD_LIGHT_SAGE` from the set CSS4_COLORS. (Color number `467`)
pub const XKCD_LIGHT_SAGE: Color = Color(188, 236, 172);
/// Color `XKCD_YELLOWGREEN` from the set CSS4_COLORS. (Color number `468`)
pub const XKCD_YELLOWGREEN: Color = Color(187, 249, 15);
/// Color `XKCD_BABY_POO` from the set CSS4_COLORS. (Color number `469`)
pub const XKCD_BABY_POO: Color = Color(171, 144, 4);
/// Color `XKCD_DARK_SEAFOAM` from the set CSS4_COLORS. (Color number `470`)
pub const XKCD_DARK_SEAFOAM: Color = Color(31, 181, 122);
/// Color `XKCD_DEEP_TEAL` from the set CSS4_COLORS. (Color number `471`)
pub const XKCD_DEEP_TEAL: Color = Color(0, 85, 90);
/// Color `XKCD_HEATHER` from the set CSS4_COLORS. (Color number `472`)
pub const XKCD_HEATHER: Color = Color(164, 132, 172);
/// Color `XKCD_RUST_ORANGE` from the set CSS4_COLORS. (Color number `473`)
pub const XKCD_RUST_ORANGE: Color = Color(196, 85, 8);
/// Color `XKCD_DIRTY_BLUE` from the set CSS4_COLORS. (Color number `474`)
pub const XKCD_DIRTY_BLUE: Color = Color(63, 130, 157);
/// Color `XKCD_FERN_GREEN` from the set CSS4_COLORS. (Color number `475`)
pub const XKCD_FERN_GREEN: Color = Color(84, 141, 68);
/// Color `XKCD_BRIGHT_LILAC` from the set CSS4_COLORS. (Color number `476`)
pub const XKCD_BRIGHT_LILAC: Color = Color(201, 94, 251);
/// Color `XKCD_WEIRD_GREEN` from the set CSS4_COLORS. (Color number `477`)
pub const XKCD_WEIRD_GREEN: Color = Color(58, 229, 127);
/// Color `XKCD_PEACOCK_BLUE` from the set CSS4_COLORS. (Color number `478`)
pub const XKCD_PEACOCK_BLUE: Color = Color(1, 103, 149);
/// Color `XKCD_AVOCADO_GREEN` from the set CSS4_COLORS. (Color number `479`)
pub const XKCD_AVOCADO_GREEN: Color = Color(135, 169, 34);
/// Color `XKCD_FADED_ORANGE` from the set CSS4_COLORS. (Color number `480`)
pub const XKCD_FADED_ORANGE: Color = Color(240, 148, 77);
/// Color `XKCD_GRAPE_PURPLE` from the set CSS4_COLORS. (Color number `481`)
pub const XKCD_GRAPE_PURPLE: Color = Color(93, 20, 81);
/// Color `XKCD_HOT_GREEN` from the set CSS4_COLORS. (Color number `482`)
pub const XKCD_HOT_GREEN: Color = Color(37, 255, 41);
/// Color `XKCD_LIME_YELLOW` from the set CSS4_COLORS. (Color number `483`)
pub const XKCD_LIME_YELLOW: Color = Color(208, 254, 29);
/// Color `XKCD_MANGO` from the set CSS4_COLORS. (Color number `484`)
pub const XKCD_MANGO: Color = Color(255, 166, 43);
/// Color `XKCD_SHAMROCK` from the set CSS4_COLORS. (Color number `485`)
pub const XKCD_SHAMROCK: Color = Color(1, 180, 76);
/// Color `XKCD_BUBBLEGUM` from the set CSS4_COLORS. (Color number `486`)
pub const XKCD_BUBBLEGUM: Color = Color(255, 108, 181);
/// Color `XKCD_PURPLISH_BROWN` from the set CSS4_COLORS. (Color number `487`)
pub const XKCD_PURPLISH_BROWN: Color = Color(107, 66, 71);
/// Color `XKCD_VOMIT_YELLOW` from the set CSS4_COLORS. (Color number `488`)
pub const XKCD_VOMIT_YELLOW: Color = Color(199, 193, 12);
/// Color `XKCD_PALE_CYAN` from the set CSS4_COLORS. (Color number `489`)
pub const XKCD_PALE_CYAN: Color = Color(183, 255, 250);
/// Color `XKCD_KEY_LIME` from the set CSS4_COLORS. (Color number `490`)
pub const XKCD_KEY_LIME: Color = Color(174, 255, 110);
/// Color `XKCD_TOMATO_RED` from the set CSS4_COLORS. (Color number `491`)
pub const XKCD_TOMATO_RED: Color = Color(236, 45, 1);
/// Color `XKCD_LIGHTGREEN` from the set CSS4_COLORS. (Color number `492`)
pub const XKCD_LIGHTGREEN: Color = Color(118, 255, 123);
/// Color `XKCD_MERLOT` from the set CSS4_COLORS. (Color number `493`)
pub const XKCD_MERLOT: Color = Color(115, 0, 57);
/// Color `XKCD_NIGHT_BLUE` from the set CSS4_COLORS. (Color number `494`)
pub const XKCD_NIGHT_BLUE: Color = Color(4, 3, 72);
/// Color `XKCD_PURPLEISH_PINK` from the set CSS4_COLORS. (Color number `495`)
pub const XKCD_PURPLEISH_PINK: Color = Color(223, 78, 200);
/// Color `XKCD_APPLE` from the set CSS4_COLORS. (Color number `496`)
pub const XKCD_APPLE: Color = Color(110, 203, 60);
/// Color `XKCD_BABY_POOP_GREEN` from the set CSS4_COLORS. (Color number `497`)
pub const XKCD_BABY_POOP_GREEN: Color = Color(143, 152, 5);
/// Color `XKCD_GREEN_APPLE` from the set CSS4_COLORS. (Color number `498`)
pub const XKCD_GREEN_APPLE: Color = Color(94, 220, 31);
/// Color `XKCD_HELIOTROPE` from the set CSS4_COLORS. (Color number `499`)
pub const XKCD_HELIOTROPE: Color = Color(217, 79, 245);
/// Color `XKCD_YELLOW_GREEN` from the set CSS4_COLORS. (Color number `500`)
pub const XKCD_YELLOW_GREEN: Color = Color(200, 253, 61);
/// Color `XKCD_ALMOST_BLACK` from the set CSS4_COLORS. (Color number `501`)
pub const XKCD_ALMOST_BLACK: Color = Color(7, 13, 13);
/// Color `XKCD_COOL_BLUE` from the set CSS4_COLORS. (Color number `502`)
pub const XKCD_COOL_BLUE: Color = Color(73, 132, 184);
/// Color `XKCD_LEAFY_GREEN` from the set CSS4_COLORS. (Color number `503`)
pub const XKCD_LEAFY_GREEN: Color = Color(81, 183, 59);
/// Color `XKCD_MUSTARD_BROWN` from the set CSS4_COLORS. (Color number `504`)
pub const XKCD_MUSTARD_BROWN: Color = Color(172, 126, 4);
/// Color `XKCD_DUSK` from the set CSS4_COLORS. (Color number `505`)
pub const XKCD_DUSK: Color = Color(78, 84, 129);
/// Color `XKCD_DULL_BROWN` from the set CSS4_COLORS. (Color number `506`)
pub const XKCD_DULL_BROWN: Color = Color(135, 110, 75);
/// Color `XKCD_FROG_GREEN` from the set CSS4_COLORS. (Color number `507`)
pub const XKCD_FROG_GREEN: Color = Color(88, 188, 8);
/// Color `XKCD_VIVID_GREEN` from the set CSS4_COLORS. (Color number `508`)
pub const XKCD_VIVID_GREEN: Color = Color(47, 239, 16);
/// Color `XKCD_BRIGHT_LIGHT_GREEN` from the set CSS4_COLORS. (Color number `509`)
pub const XKCD_BRIGHT_LIGHT_GREEN: Color = Color(45, 254, 84);
/// Color `XKCD_FLURO_GREEN` from the set CSS4_COLORS. (Color number `510`)
pub const XKCD_FLURO_GREEN: Color = Color(10, 255, 2);
/// Color `XKCD_KIWI` from the set CSS4_COLORS. (Color number `511`)
pub const XKCD_KIWI: Color = Color(156, 239, 67);
/// Color `XKCD_SEAWEED` from the set CSS4_COLORS. (Color number `512`)
pub const XKCD_SEAWEED: Color = Color(24, 209, 123);
/// Color `XKCD_NAVY_GREEN` from the set CSS4_COLORS. (Color number `513`)
pub const XKCD_NAVY_GREEN: Color = Color(53, 83, 10);
/// Color `XKCD_ULTRAMARINE_BLUE` from the set CSS4_COLORS. (Color number `514`)
pub const XKCD_ULTRAMARINE_BLUE: Color = Color(24, 5, 219);
/// Color `XKCD_IRIS` from the set CSS4_COLORS. (Color number `515`)
pub const XKCD_IRIS: Color = Color(98, 88, 196);
/// Color `XKCD_PASTEL_ORANGE` from the set CSS4_COLORS. (Color number `516`)
pub const XKCD_PASTEL_ORANGE: Color = Color(255, 150, 79);
/// Color `XKCD_YELLOWISH_ORANGE` from the set CSS4_COLORS. (Color number `517`)
pub const XKCD_YELLOWISH_ORANGE: Color = Color(255, 171, 15);
/// Color `XKCD_PERRYWINKLE` from the set CSS4_COLORS. (Color number `518`)
pub const XKCD_PERRYWINKLE: Color = Color(143, 140, 231);
/// Color `XKCD_TEALISH` from the set CSS4_COLORS. (Color number `519`)
pub const XKCD_TEALISH: Color = Color(36, 188, 168);
/// Color `XKCD_DARK_PLUM` from the set CSS4_COLORS. (Color number `520`)
pub const XKCD_DARK_PLUM: Color = Color(63, 1, 44);
/// Color `XKCD_PEAR` from the set CSS4_COLORS. (Color number `521`)
pub const XKCD_PEAR: Color = Color(203, 248, 95);
/// Color `XKCD_PINKISH_ORANGE` from the set CSS4_COLORS. (Color number `522`)
pub const XKCD_PINKISH_ORANGE: Color = Color(255, 114, 76);
/// Color `XKCD_MIDNIGHT_PURPLE` from the set CSS4_COLORS. (Color number `523`)
pub const XKCD_MIDNIGHT_PURPLE: Color = Color(40, 1, 55);
/// Color `XKCD_LIGHT_URPLE` from the set CSS4_COLORS. (Color number `524`)
pub const XKCD_LIGHT_URPLE: Color = Color(179, 111, 246);
/// Color `XKCD_DARK_MINT` from the set CSS4_COLORS. (Color number `525`)
pub const XKCD_DARK_MINT: Color = Color(72, 192, 114);
/// Color `XKCD_GREENISH_TAN` from the set CSS4_COLORS. (Color number `526`)
pub const XKCD_GREENISH_TAN: Color = Color(188, 203, 122);
/// Color `XKCD_LIGHT_BURGUNDY` from the set CSS4_COLORS. (Color number `527`)
pub const XKCD_LIGHT_BURGUNDY: Color = Color(168, 65, 91);
/// Color `XKCD_TURQUOISE_BLUE` from the set CSS4_COLORS. (Color number `528`)
pub const XKCD_TURQUOISE_BLUE: Color = Color(6, 177, 196);
/// Color `XKCD_UGLY_PINK` from the set CSS4_COLORS. (Color number `529`)
pub const XKCD_UGLY_PINK: Color = Color(205, 117, 132);
/// Color `XKCD_SANDY` from the set CSS4_COLORS. (Color number `530`)
pub const XKCD_SANDY: Color = Color(241, 218, 122);
/// Color `XKCD_ELECTRIC_PINK` from the set CSS4_COLORS. (Color number `531`)
pub const XKCD_ELECTRIC_PINK: Color = Color(255, 4, 144);
/// Color `XKCD_MUTED_PURPLE` from the set CSS4_COLORS. (Color number `532`)
pub const XKCD_MUTED_PURPLE: Color = Color(128, 91, 135);
/// Color `XKCD_MID_GREEN` from the set CSS4_COLORS. (Color number `533`)
pub const XKCD_MID_GREEN: Color = Color(80, 167, 71);
/// Color `XKCD_GREYISH` from the set CSS4_COLORS. (Color number `534`)
pub const XKCD_GREYISH: Color = Color(168, 164, 149);
/// Color `XKCD_NEON_YELLOW` from the set CSS4_COLORS. (Color number `535`)
pub const XKCD_NEON_YELLOW: Color = Color(207, 255, 4);
/// Color `XKCD_BANANA` from the set CSS4_COLORS. (Color number `536`)
pub const XKCD_BANANA: Color = Color(255, 255, 126);
/// Color `XKCD_CARNATION_PINK` from the set CSS4_COLORS. (Color number `537`)
pub const XKCD_CARNATION_PINK: Color = Color(255, 127, 167);
/// Color `XKCD_TOMATO` from the set CSS4_COLORS. (Color number `538`)
pub const XKCD_TOMATO: Color = Color(239, 64, 38);
/// Color `XKCD_SEA` from the set CSS4_COLORS. (Color number `539`)
pub const XKCD_SEA: Color = Color(60, 153, 146);
/// Color `XKCD_MUDDY_BROWN` from the set CSS4_COLORS. (Color number `540`)
pub const XKCD_MUDDY_BROWN: Color = Color(136, 104, 6);
/// Color `XKCD_TURQUOISE_GREEN` from the set CSS4_COLORS. (Color number `541`)
pub const XKCD_TURQUOISE_GREEN: Color = Color(4, 244, 137);
/// Color `XKCD_BUFF` from the set CSS4_COLORS. (Color number `542`)
pub const XKCD_BUFF: Color = Color(254, 246, 158);
/// Color `XKCD_FAWN` from the set CSS4_COLORS. (Color number `543`)
pub const XKCD_FAWN: Color = Color(207, 175, 123);
/// Color `XKCD_MUTED_BLUE` from the set CSS4_COLORS. (Color number `544`)
pub const XKCD_MUTED_BLUE: Color = Color(59, 113, 159);
/// Color `XKCD_PALE_ROSE` from the set CSS4_COLORS. (Color number `545`)
pub const XKCD_PALE_ROSE: Color = Color(253, 193, 197);
/// Color `XKCD_DARK_MINT_GREEN` from the set CSS4_COLORS. (Color number `546`)
pub const XKCD_DARK_MINT_GREEN: Color = Color(32, 192, 115);
/// Color `XKCD_AMETHYST` from the set CSS4_COLORS. (Color number `547`)
pub const XKCD_AMETHYST: Color = Color(155, 95, 192);
/// Color `XKCD_BLUE_GREEN` from the set CSS4_COLORS. (Color number `548`)
pub const XKCD_BLUE_GREEN: Color = Color(15, 155, 142);
/// Color `XKCD_CHESTNUT` from the set CSS4_COLORS. (Color number `549`)
pub const XKCD_CHESTNUT: Color = Color(116, 40, 2);
/// Color `XKCD_SICK_GREEN` from the set CSS4_COLORS. (Color number `550`)
pub const XKCD_SICK_GREEN: Color = Color(157, 185, 44);
/// Color `XKCD_PEA` from the set CSS4_COLORS. (Color number `551`)
pub const XKCD_PEA: Color = Color(164, 191, 32);
/// Color `XKCD_RUSTY_ORANGE` from the set CSS4_COLORS. (Color number `552`)
pub const XKCD_RUSTY_ORANGE: Color = Color(205, 89, 9);
/// Color `XKCD_STONE` from the set CSS4_COLORS. (Color number `553`)
pub const XKCD_STONE: Color = Color(173, 165, 135);
/// Color `XKCD_ROSE_RED` from the set CSS4_COLORS. (Color number `554`)
pub const XKCD_ROSE_RED: Color = Color(190, 1, 60);
/// Color `XKCD_PALE_AQUA` from the set CSS4_COLORS. (Color number `555`)
pub const XKCD_PALE_AQUA: Color = Color(184, 255, 235);
/// Color `XKCD_DEEP_ORANGE` from the set CSS4_COLORS. (Color number `556`)
pub const XKCD_DEEP_ORANGE: Color = Color(220, 77, 1);
/// Color `XKCD_EARTH` from the set CSS4_COLORS. (Color number `557`)
pub const XKCD_EARTH: Color = Color(162, 101, 62);
/// Color `XKCD_MOSSY_GREEN` from the set CSS4_COLORS. (Color number `558`)
pub const XKCD_MOSSY_GREEN: Color = Color(99, 139, 39);
/// Color `XKCD_GRASSY_GREEN` from the set CSS4_COLORS. (Color number `559`)
pub const XKCD_GRASSY_GREEN: Color = Color(65, 156, 3);
/// Color `XKCD_PALE_LIME_GREEN` from the set CSS4_COLORS. (Color number `560`)
pub const XKCD_PALE_LIME_GREEN: Color = Color(177, 255, 101);
/// Color `XKCD_LIGHT_GREY_BLUE` from the set CSS4_COLORS. (Color number `561`)
pub const XKCD_LIGHT_GREY_BLUE: Color = Color(157, 188, 212);
/// Color `XKCD_PALE_GREY` from the set CSS4_COLORS. (Color number `562`)
pub const XKCD_PALE_GREY: Color = Color(253, 253, 254);
/// Color `XKCD_ASPARAGUS` from the set CSS4_COLORS. (Color number `563`)
pub const XKCD_ASPARAGUS: Color = Color(119, 171, 86);
/// Color `XKCD_BLUEBERRY` from the set CSS4_COLORS. (Color number `564`)
pub const XKCD_BLUEBERRY: Color = Color(70, 65, 150);
/// Color `XKCD_PURPLE_RED` from the set CSS4_COLORS. (Color number `565`)
pub const XKCD_PURPLE_RED: Color = Color(153, 1, 71);
/// Color `XKCD_PALE_LIME` from the set CSS4_COLORS. (Color number `566`)
pub const XKCD_PALE_LIME: Color = Color(190, 253, 115);
/// Color `XKCD_GREENISH_TEAL` from the set CSS4_COLORS. (Color number `567`)
pub const XKCD_GREENISH_TEAL: Color = Color(50, 191, 132);
/// Color `XKCD_CARAMEL` from the set CSS4_COLORS. (Color number `568`)
pub const XKCD_CARAMEL: Color = Color(175, 111, 9);
/// Color `XKCD_DEEP_MAGENTA` from the set CSS4_COLORS. (Color number `569`)
pub const XKCD_DEEP_MAGENTA: Color = Color(160, 2, 92);
/// Color `XKCD_LIGHT_PEACH` from the set CSS4_COLORS. (Color number `570`)
pub const XKCD_LIGHT_PEACH: Color = Color(255, 216, 177);
/// Color `XKCD_MILK_CHOCOLATE` from the set CSS4_COLORS. (Color number `571`)
pub const XKCD_MILK_CHOCOLATE: Color = Color(127, 78, 30);
/// Color `XKCD_OCHER` from the set CSS4_COLORS. (Color number `572`)
pub const XKCD_OCHER: Color = Color(191, 155, 12);
/// Color `XKCD_OFF_GREEN` from the set CSS4_COLORS. (Color number `573`)
pub const XKCD_OFF_GREEN: Color = Color(107, 163, 83);
/// Color `XKCD_PURPLY_PINK` from the set CSS4_COLORS. (Color number `574`)
pub const XKCD_PURPLY_PINK: Color = Color(240, 117, 230);
/// Color `XKCD_LIGHTBLUE` from the set CSS4_COLORS. (Color number `575`)
pub const XKCD_LIGHTBLUE: Color = Color(123, 200, 246);
/// Color `XKCD_DUSKY_BLUE` from the set CSS4_COLORS. (Color number `576`)
pub const XKCD_DUSKY_BLUE: Color = Color(71, 95, 148);
/// Color `XKCD_GOLDEN` from the set CSS4_COLORS. (Color number `577`)
pub const XKCD_GOLDEN: Color = Color(245, 191, 3);
/// Color `XKCD_LIGHT_BEIGE` from the set CSS4_COLORS. (Color number `578`)
pub const XKCD_LIGHT_BEIGE: Color = Color(255, 254, 182);
/// Color `XKCD_BUTTER_YELLOW` from the set CSS4_COLORS. (Color number `579`)
pub const XKCD_BUTTER_YELLOW: Color = Color(255, 253, 116);
/// Color `XKCD_DUSKY_PURPLE` from the set CSS4_COLORS. (Color number `580`)
pub const XKCD_DUSKY_PURPLE: Color = Color(137, 91, 123);
/// Color `XKCD_FRENCH_BLUE` from the set CSS4_COLORS. (Color number `581`)
pub const XKCD_FRENCH_BLUE: Color = Color(67, 107, 173);
/// Color `XKCD_UGLY_YELLOW` from the set CSS4_COLORS. (Color number `582`)
pub const XKCD_UGLY_YELLOW: Color = Color(208, 193, 1);
/// Color `XKCD_GREENY_YELLOW` from the set CSS4_COLORS. (Color number `583`)
pub const XKCD_GREENY_YELLOW: Color = Color(198, 248, 8);
/// Color `XKCD_ORANGISH_RED` from the set CSS4_COLORS. (Color number `584`)
pub const XKCD_ORANGISH_RED: Color = Color(244, 54, 5);
/// Color `XKCD_SHAMROCK_GREEN` from the set CSS4_COLORS. (Color number `585`)
pub const XKCD_SHAMROCK_GREEN: Color = Color(2, 193, 77);
/// Color `XKCD_ORANGISH_BROWN` from the set CSS4_COLORS. (Color number `586`)
pub const XKCD_ORANGISH_BROWN: Color = Color(178, 95, 3);
/// Color `XKCD_TREE_GREEN` from the set CSS4_COLORS. (Color number `587`)
pub const XKCD_TREE_GREEN: Color = Color(42, 126, 25);
/// Color `XKCD_DEEP_VIOLET` from the set CSS4_COLORS. (Color number `588`)
pub const XKCD_DEEP_VIOLET: Color = Color(73, 6, 72);
/// Color `XKCD_GUNMETAL` from the set CSS4_COLORS. (Color number `589`)
pub const XKCD_GUNMETAL: Color = Color(83, 98, 103);
/// Color `XKCD_BLUE_PURPLE` from the set CSS4_COLORS. (Color number `590`)
pub const XKCD_BLUE_PURPLE: Color = Color(90, 6, 239);
/// Color `XKCD_CHERRY` from the set CSS4_COLORS. (Color number `591`)
pub const XKCD_CHERRY: Color = Color(207, 2, 52);
/// Color `XKCD_SANDY_BROWN` from the set CSS4_COLORS. (Color number `592`)
pub const XKCD_SANDY_BROWN: Color = Color(196, 166, 97);
/// Color `XKCD_WARM_GREY` from the set CSS4_COLORS. (Color number `593`)
pub const XKCD_WARM_GREY: Color = Color(151, 138, 132);
/// Color `XKCD_DARK_INDIGO` from the set CSS4_COLORS. (Color number `594`)
pub const XKCD_DARK_INDIGO: Color = Color(31, 9, 84);
/// Color `XKCD_MIDNIGHT` from the set CSS4_COLORS. (Color number `595`)
pub const XKCD_MIDNIGHT: Color = Color(3, 1, 45);
/// Color `XKCD_BLUEY_GREEN` from the set CSS4_COLORS. (Color number `596`)
pub const XKCD_BLUEY_GREEN: Color = Color(43, 177, 121);
/// Color `XKCD_GREY_PINK` from the set CSS4_COLORS. (Color number `597`)
pub const XKCD_GREY_PINK: Color = Color(195, 144, 155);
/// Color `XKCD_SOFT_PURPLE` from the set CSS4_COLORS. (Color number `598`)
pub const XKCD_SOFT_PURPLE: Color = Color(166, 111, 181);
/// Color `XKCD_BLOOD` from the set CSS4_COLORS. (Color number `599`)
pub const XKCD_BLOOD: Color = Color(119, 0, 1);
/// Color `XKCD_BROWN_RED` from the set CSS4_COLORS. (Color number `600`)
pub const XKCD_BROWN_RED: Color = Color(146, 43, 5);
/// Color `XKCD_MEDIUM_GREY` from the set CSS4_COLORS. (Color number `601`)
pub const XKCD_MEDIUM_GREY: Color = Color(125, 127, 124);
/// Color `XKCD_BERRY` from the set CSS4_COLORS. (Color number `602`)
pub const XKCD_BERRY: Color = Color(153, 15, 75);
/// Color `XKCD_POO` from the set CSS4_COLORS. (Color number `603`)
pub const XKCD_POO: Color = Color(143, 115, 3);
/// Color `XKCD_PURPLEY_PINK` from the set CSS4_COLORS. (Color number `604`)
pub const XKCD_PURPLEY_PINK: Color = Color(200, 60, 185);
/// Color `XKCD_LIGHT_SALMON` from the set CSS4_COLORS. (Color number `605`)
pub const XKCD_LIGHT_SALMON: Color = Color(254, 169, 147);
/// Color `XKCD_SNOT` from the set CSS4_COLORS. (Color number `606`)
pub const XKCD_SNOT: Color = Color(172, 187, 13);
/// Color `XKCD_EASTER_PURPLE` from the set CSS4_COLORS. (Color number `607`)
pub const XKCD_EASTER_PURPLE: Color = Color(192, 113, 254);
/// Color `XKCD_LIGHT_YELLOW_GREEN` from the set CSS4_COLORS. (Color number `608`)
pub const XKCD_LIGHT_YELLOW_GREEN: Color = Color(204, 253, 127);
/// Color `XKCD_DARK_NAVY_BLUE` from the set CSS4_COLORS. (Color number `609`)
pub const XKCD_DARK_NAVY_BLUE: Color = Color(0, 2, 46);
/// Color `XKCD_DRAB` from the set CSS4_COLORS. (Color number `610`)
pub const XKCD_DRAB: Color = Color(130, 131, 68);
/// Color `XKCD_LIGHT_ROSE` from the set CSS4_COLORS. (Color number `611`)
pub const XKCD_LIGHT_ROSE: Color = Color(255, 197, 203);
/// Color `XKCD_ROUGE` from the set CSS4_COLORS. (Color number `612`)
pub const XKCD_ROUGE: Color = Color(171, 18, 57);
/// Color `XKCD_PURPLISH_RED` from the set CSS4_COLORS. (Color number `613`)
pub const XKCD_PURPLISH_RED: Color = Color(176, 5, 75);
/// Color `XKCD_SLIME_GREEN` from the set CSS4_COLORS. (Color number `614`)
pub const XKCD_SLIME_GREEN: Color = Color(153, 204, 4);
/// Color `XKCD_BABY_POOP` from the set CSS4_COLORS. (Color number `615`)
pub const XKCD_BABY_POOP: Color = Color(147, 124, 0);
/// Color `XKCD_IRISH_GREEN` from the set CSS4_COLORS. (Color number `616`)
pub const XKCD_IRISH_GREEN: Color = Color(1, 149, 41);
/// Color `XKCD_PINK_PURPLE` from the set CSS4_COLORS. (Color number `617`)
pub const XKCD_PINK_PURPLE: Color = Color(239, 29, 231);
/// Color `XKCD_DARK_NAVY` from the set CSS4_COLORS. (Color number `618`)
pub const XKCD_DARK_NAVY: Color = Color(0, 4, 53);
/// Color `XKCD_GREENY_BLUE` from the set CSS4_COLORS. (Color number `619`)
pub const XKCD_GREENY_BLUE: Color = Color(66, 179, 149);
/// Color `XKCD_LIGHT_PLUM` from the set CSS4_COLORS. (Color number `620`)
pub const XKCD_LIGHT_PLUM: Color = Color(157, 87, 131);
/// Color `XKCD_PINKISH_GREY` from the set CSS4_COLORS. (Color number `621`)
pub const XKCD_PINKISH_GREY: Color = Color(200, 172, 169);
/// Color `XKCD_DIRTY_ORANGE` from the set CSS4_COLORS. (Color number `622`)
pub const XKCD_DIRTY_ORANGE: Color = Color(200, 118, 6);
/// Color `XKCD_RUST_RED` from the set CSS4_COLORS. (Color number `623`)
pub const XKCD_RUST_RED: Color = Color(170, 39, 4);
/// Color `XKCD_PALE_LILAC` from the set CSS4_COLORS. (Color number `624`)
pub const XKCD_PALE_LILAC: Color = Color(228, 203, 255);
/// Color `XKCD_ORANGEY_RED` from the set CSS4_COLORS. (Color number `625`)
pub const XKCD_ORANGEY_RED: Color = Color(250, 66, 36);
/// Color `XKCD_PRIMARY_BLUE` from the set CSS4_COLORS. (Color number `626`)
pub const XKCD_PRIMARY_BLUE: Color = Color(8, 4, 249);
/// Color `XKCD_KERMIT_GREEN` from the set CSS4_COLORS. (Color number `627`)
pub const XKCD_KERMIT_GREEN: Color = Color(92, 178, 0);
/// Color `XKCD_BROWNISH_PURPLE` from the set CSS4_COLORS. (Color number `628`)
pub const XKCD_BROWNISH_PURPLE: Color = Color(118, 66, 78);
/// Color `XKCD_MURKY_GREEN` from the set CSS4_COLORS. (Color number `629`)
pub const XKCD_MURKY_GREEN: Color = Color(108, 122, 14);
/// Color `XKCD_WHEAT` from the set CSS4_COLORS. (Color number `630`)
pub const XKCD_WHEAT: Color = Color(251, 221, 126);
/// Color `XKCD_VERY_DARK_PURPLE` from the set CSS4_COLORS. (Color number `631`)
pub const XKCD_VERY_DARK_PURPLE: Color = Color(42, 1, 52);
/// Color `XKCD_BOTTLE_GREEN` from the set CSS4_COLORS. (Color number `632`)
pub const XKCD_BOTTLE_GREEN: Color = Color(4, 74, 5);
/// Color `XKCD_WATERMELON` from the set CSS4_COLORS. (Color number `633`)
pub const XKCD_WATERMELON: Color = Color(253, 70, 89);
/// Color `XKCD_DEEP_SKY_BLUE` from the set CSS4_COLORS. (Color number `634`)
pub const XKCD_DEEP_SKY_BLUE: Color = Color(13, 117, 248);
/// Color `XKCD_FIRE_ENGINE_RED` from the set CSS4_COLORS. (Color number `635`)
pub const XKCD_FIRE_ENGINE_RED: Color = Color(254, 0, 2);
/// Color `XKCD_YELLOW_OCHRE` from the set CSS4_COLORS. (Color number `636`)
pub const XKCD_YELLOW_OCHRE: Color = Color(203, 157, 6);
/// Color `XKCD_PUMPKIN_ORANGE` from the set CSS4_COLORS. (Color number `637`)
pub const XKCD_PUMPKIN_ORANGE: Color = Color(251, 125, 7);
/// Color `XKCD_PALE_OLIVE` from the set CSS4_COLORS. (Color number `638`)
pub const XKCD_PALE_OLIVE: Color = Color(185, 204, 129);
/// Color `XKCD_LIGHT_LILAC` from the set CSS4_COLORS. (Color number `639`)
pub const XKCD_LIGHT_LILAC: Color = Color(237, 200, 255);
/// Color `XKCD_LIGHTISH_GREEN` from the set CSS4_COLORS. (Color number `640`)
pub const XKCD_LIGHTISH_GREEN: Color = Color(97, 225, 96);
/// Color `XKCD_CAROLINA_BLUE` from the set CSS4_COLORS. (Color number `641`)
pub const XKCD_CAROLINA_BLUE: Color = Color(138, 184, 254);
/// Color `XKCD_MULBERRY` from the set CSS4_COLORS. (Color number `642`)
pub const XKCD_MULBERRY: Color = Color(146, 10, 78);
/// Color `XKCD_SHOCKING_PINK` from the set CSS4_COLORS. (Color number `643`)
pub const XKCD_SHOCKING_PINK: Color = Color(254, 2, 162);
/// Color `XKCD_AUBURN` from the set CSS4_COLORS. (Color number `644`)
pub const XKCD_AUBURN: Color = Color(154, 48, 1);
/// Color `XKCD_BRIGHT_LIME_GREEN` from the set CSS4_COLORS. (Color number `645`)
pub const XKCD_BRIGHT_LIME_GREEN: Color = Color(101, 254, 8);
/// Color `XKCD_CELADON` from the set CSS4_COLORS. (Color number `646`)
pub const XKCD_CELADON: Color = Color(190, 253, 183);
/// Color `XKCD_PINKISH_BROWN` from the set CSS4_COLORS. (Color number `647`)
pub const XKCD_PINKISH_BROWN: Color = Color(177, 114, 97);
/// Color `XKCD_POO_BROWN` from the set CSS4_COLORS. (Color number `648`)
pub const XKCD_POO_BROWN: Color = Color(136, 95, 1);
/// Color `XKCD_BRIGHT_SKY_BLUE` from the set CSS4_COLORS. (Color number `649`)
pub const XKCD_BRIGHT_SKY_BLUE: Color = Color(2, 204, 254);
/// Color `XKCD_CELERY` from the set CSS4_COLORS. (Color number `650`)
pub const XKCD_CELERY: Color = Color(193, 253, 149);
/// Color `XKCD_DIRT_BROWN` from the set CSS4_COLORS. (Color number `651`)
pub const XKCD_DIRT_BROWN: Color = Color(131, 101, 57);
/// Color `XKCD_STRAWBERRY` from the set CSS4_COLORS. (Color number `652`)
pub const XKCD_STRAWBERRY: Color = Color(251, 41, 67);
/// Color `XKCD_DARK_LIME` from the set CSS4_COLORS. (Color number `653`)
pub const XKCD_DARK_LIME: Color = Color(132, 183, 1);
/// Color `XKCD_COPPER` from the set CSS4_COLORS. (Color number `654`)
pub const XKCD_COPPER: Color = Color(182, 99, 37);
/// Color `XKCD_MEDIUM_BROWN` from the set CSS4_COLORS. (Color number `655`)
pub const XKCD_MEDIUM_BROWN: Color = Color(127, 81, 18);
/// Color `XKCD_MUTED_GREEN` from the set CSS4_COLORS. (Color number `656`)
pub const XKCD_MUTED_GREEN: Color = Color(95, 160, 82);
/// Color `XKCD_ROBIN_S_EGG` from the set CSS4_COLORS. (Color number `657`)
pub const XKCD_ROBIN_S_EGG: Color = Color(109, 237, 253);
/// Color `XKCD_BRIGHT_AQUA` from the set CSS4_COLORS. (Color number `658`)
pub const XKCD_BRIGHT_AQUA: Color = Color(11, 249, 234);
/// Color `XKCD_BRIGHT_LAVENDER` from the set CSS4_COLORS. (Color number `659`)
pub const XKCD_BRIGHT_LAVENDER: Color = Color(199, 96, 255);
/// Color `XKCD_IVORY` from the set CSS4_COLORS. (Color number `660`)
pub const XKCD_IVORY: Color = Color(255, 255, 203);
/// Color `XKCD_VERY_LIGHT_PURPLE` from the set CSS4_COLORS. (Color number `661`)
pub const XKCD_VERY_LIGHT_PURPLE: Color = Color(246, 206, 252);
/// Color `XKCD_LIGHT_NAVY` from the set CSS4_COLORS. (Color number `662`)
pub const XKCD_LIGHT_NAVY: Color = Color(21, 80, 132);
/// Color `XKCD_PINK_RED` from the set CSS4_COLORS. (Color number `663`)
pub const XKCD_PINK_RED: Color = Color(245, 5, 79);
/// Color `XKCD_OLIVE_BROWN` from the set CSS4_COLORS. (Color number `664`)
pub const XKCD_OLIVE_BROWN: Color = Color(100, 84, 3);
/// Color `XKCD_POOP_BROWN` from the set CSS4_COLORS. (Color number `665`)
pub const XKCD_POOP_BROWN: Color = Color(122, 89, 1);
/// Color `XKCD_MUSTARD_GREEN` from the set CSS4_COLORS. (Color number `666`)
pub const XKCD_MUSTARD_GREEN: Color = Color(168, 181, 4);
/// Color `XKCD_OCEAN_GREEN` from the set CSS4_COLORS. (Color number `667`)
pub const XKCD_OCEAN_GREEN: Color = Color(61, 153, 115);
/// Color `XKCD_VERY_DARK_BLUE` from the set CSS4_COLORS. (Color number `668`)
pub const XKCD_VERY_DARK_BLUE: Color = Color(0, 1, 51);
/// Color `XKCD_DUSTY_GREEN` from the set CSS4_COLORS. (Color number `669`)
pub const XKCD_DUSTY_GREEN: Color = Color(118, 169, 115);
/// Color `XKCD_LIGHT_NAVY_BLUE` from the set CSS4_COLORS. (Color number `670`)
pub const XKCD_LIGHT_NAVY_BLUE: Color = Color(46, 90, 136);
/// Color `XKCD_MINTY_GREEN` from the set CSS4_COLORS. (Color number `671`)
pub const XKCD_MINTY_GREEN: Color = Color(11, 247, 125);
/// Color `XKCD_ADOBE` from the set CSS4_COLORS. (Color number `672`)
pub const XKCD_ADOBE: Color = Color(189, 108, 72);
/// Color `XKCD_BARNEY` from the set CSS4_COLORS. (Color number `673`)
pub const XKCD_BARNEY: Color = Color(172, 29, 184);
/// Color `XKCD_JADE_GREEN` from the set CSS4_COLORS. (Color number `674`)
pub const XKCD_JADE_GREEN: Color = Color(43, 175, 106);
/// Color `XKCD_BRIGHT_LIGHT_BLUE` from the set CSS4_COLORS. (Color number `675`)
pub const XKCD_BRIGHT_LIGHT_BLUE: Color = Color(38, 247, 253);
/// Color `XKCD_LIGHT_LIME` from the set CSS4_COLORS. (Color number `676`)
pub const XKCD_LIGHT_LIME: Color = Color(174, 253, 108);
/// Color `XKCD_DARK_KHAKI` from the set CSS4_COLORS. (Color number `677`)
pub const XKCD_DARK_KHAKI: Color = Color(155, 143, 85);
/// Color `XKCD_ORANGE_YELLOW` from the set CSS4_COLORS. (Color number `678`)
pub const XKCD_ORANGE_YELLOW: Color = Color(255, 173, 1);
/// Color `XKCD_OCRE` from the set CSS4_COLORS. (Color number `679`)
pub const XKCD_OCRE: Color = Color(198, 156, 4);
/// Color `XKCD_MAIZE` from the set CSS4_COLORS. (Color number `680`)
pub const XKCD_MAIZE: Color = Color(244, 208, 84);
/// Color `XKCD_FADED_PINK` from the set CSS4_COLORS. (Color number `681`)
pub const XKCD_FADED_PINK: Color = Color(222, 157, 172);
/// Color `XKCD_BRITISH_RACING_GREEN` from the set CSS4_COLORS. (Color number `682`)
pub const XKCD_BRITISH_RACING_GREEN: Color = Color(5, 72, 13);
/// Color `XKCD_SANDSTONE` from the set CSS4_COLORS. (Color number `683`)
pub const XKCD_SANDSTONE: Color = Color(201, 174, 116);
/// Color `XKCD_MUD_BROWN` from the set CSS4_COLORS. (Color number `684`)
pub const XKCD_MUD_BROWN: Color = Color(96, 70, 15);
/// Color `XKCD_LIGHT_SEA_GREEN` from the set CSS4_COLORS. (Color number `685`)
pub const XKCD_LIGHT_SEA_GREEN: Color = Color(152, 246, 176);
/// Color `XKCD_ROBIN_EGG_BLUE` from the set CSS4_COLORS. (Color number `686`)
pub const XKCD_ROBIN_EGG_BLUE: Color = Color(138, 241, 254);
/// Color `XKCD_AQUA_MARINE` from the set CSS4_COLORS. (Color number `687`)
pub const XKCD_AQUA_MARINE: Color = Color(46, 232, 187);
/// Color `XKCD_DARK_SEA_GREEN` from the set CSS4_COLORS. (Color number `688`)
pub const XKCD_DARK_SEA_GREEN: Color = Color(17, 135, 93);
/// Color `XKCD_SOFT_PINK` from the set CSS4_COLORS. (Color number `689`)
pub const XKCD_SOFT_PINK: Color = Color(253, 176, 192);
/// Color `XKCD_ORANGEY_BROWN` from the set CSS4_COLORS. (Color number `690`)
pub const XKCD_ORANGEY_BROWN: Color = Color(177, 96, 2);
/// Color `XKCD_CHERRY_RED` from the set CSS4_COLORS. (Color number `691`)
pub const XKCD_CHERRY_RED: Color = Color(247, 2, 42);
/// Color `XKCD_BURNT_YELLOW` from the set CSS4_COLORS. (Color number `692`)
pub const XKCD_BURNT_YELLOW: Color = Color(213, 171, 9);
/// Color `XKCD_BROWNISH_GREY` from the set CSS4_COLORS. (Color number `693`)
pub const XKCD_BROWNISH_GREY: Color = Color(134, 119, 95);
/// Color `XKCD_CAMEL` from the set CSS4_COLORS. (Color number `694`)
pub const XKCD_CAMEL: Color = Color(198, 159, 89);
/// Color `XKCD_PURPLISH_GREY` from the set CSS4_COLORS. (Color number `695`)
pub const XKCD_PURPLISH_GREY: Color = Color(122, 104, 127);
/// Color `XKCD_MARINE` from the set CSS4_COLORS. (Color number `696`)
pub const XKCD_MARINE: Color = Color(4, 46, 96);
/// Color `XKCD_GREYISH_PINK` from the set CSS4_COLORS. (Color number `697`)
pub const XKCD_GREYISH_PINK: Color = Color(200, 141, 148);
/// Color `XKCD_PALE_TURQUOISE` from the set CSS4_COLORS. (Color number `698`)
pub const XKCD_PALE_TURQUOISE: Color = Color(165, 251, 213);
/// Color `XKCD_PASTEL_YELLOW` from the set CSS4_COLORS. (Color number `699`)
pub const XKCD_PASTEL_YELLOW: Color = Color(255, 254, 113);
/// Color `XKCD_BLUEY_PURPLE` from the set CSS4_COLORS. (Color number `700`)
pub const XKCD_BLUEY_PURPLE: Color = Color(98, 65, 199);
/// Color `XKCD_CANARY_YELLOW` from the set CSS4_COLORS. (Color number `701`)
pub const XKCD_CANARY_YELLOW: Color = Color(255, 254, 64);
/// Color `XKCD_FADED_RED` from the set CSS4_COLORS. (Color number `702`)
pub const XKCD_FADED_RED: Color = Color(211, 73, 78);
/// Color `XKCD_SEPIA` from the set CSS4_COLORS. (Color number `703`)
pub const XKCD_SEPIA: Color = Color(152, 94, 43);
/// Color `XKCD_COFFEE` from the set CSS4_COLORS. (Color number `704`)
pub const XKCD_COFFEE: Color = Color(166, 129, 76);
/// Color `XKCD_BRIGHT_MAGENTA` from the set CSS4_COLORS. (Color number `705`)
pub const XKCD_BRIGHT_MAGENTA: Color = Color(255, 8, 232);
/// Color `XKCD_MOCHA` from the set CSS4_COLORS. (Color number `706`)
pub const XKCD_MOCHA: Color = Color(157, 118, 81);
/// Color `XKCD_ECRU` from the set CSS4_COLORS. (Color number `707`)
pub const XKCD_ECRU: Color = Color(254, 255, 202);
/// Color `XKCD_PURPLEISH` from the set CSS4_COLORS. (Color number `708`)
pub const XKCD_PURPLEISH: Color = Color(152, 86, 141);
/// Color `XKCD_CRANBERRY` from the set CSS4_COLORS. (Color number `709`)
pub const XKCD_CRANBERRY: Color = Color(158, 0, 58);
/// Color `XKCD_DARKISH_GREEN` from the set CSS4_COLORS. (Color number `710`)
pub const XKCD_DARKISH_GREEN: Color = Color(40, 124, 55);
/// Color `XKCD_BROWN_ORANGE` from the set CSS4_COLORS. (Color number `711`)
pub const XKCD_BROWN_ORANGE: Color = Color(185, 105, 2);
/// Color `XKCD_DUSKY_ROSE` from the set CSS4_COLORS. (Color number `712`)
pub const XKCD_DUSKY_ROSE: Color = Color(186, 104, 115);
/// Color `XKCD_MELON` from the set CSS4_COLORS. (Color number `713`)
pub const XKCD_MELON: Color = Color(255, 120, 85);
/// Color `XKCD_SICKLY_GREEN` from the set CSS4_COLORS. (Color number `714`)
pub const XKCD_SICKLY_GREEN: Color = Color(148, 178, 28);
/// Color `XKCD_SILVER` from the set CSS4_COLORS. (Color number `715`)
pub const XKCD_SILVER: Color = Color(197, 201, 199);
/// Color `XKCD_PURPLY_BLUE` from the set CSS4_COLORS. (Color number `716`)
pub const XKCD_PURPLY_BLUE: Color = Color(102, 26, 238);
/// Color `XKCD_PURPLEISH_BLUE` from the set CSS4_COLORS. (Color number `717`)
pub const XKCD_PURPLEISH_BLUE: Color = Color(97, 64, 239);
/// Color `XKCD_HOSPITAL_GREEN` from the set CSS4_COLORS. (Color number `718`)
pub const XKCD_HOSPITAL_GREEN: Color = Color(155, 229, 170);
/// Color `XKCD_SHIT_BROWN` from the set CSS4_COLORS. (Color number `719`)
pub const XKCD_SHIT_BROWN: Color = Color(123, 88, 4);
/// Color `XKCD_MID_BLUE` from the set CSS4_COLORS. (Color number `720`)
pub const XKCD_MID_BLUE: Color = Color(39, 106, 179);
/// Color `XKCD_AMBER` from the set CSS4_COLORS. (Color number `721`)
pub const XKCD_AMBER: Color = Color(254, 179, 8);
/// Color `XKCD_EASTER_GREEN` from the set CSS4_COLORS. (Color number `722`)
pub const XKCD_EASTER_GREEN: Color = Color(140, 253, 126);
/// Color `XKCD_SOFT_BLUE` from the set CSS4_COLORS. (Color number `723`)
pub const XKCD_SOFT_BLUE: Color = Color(100, 136, 234);
/// Color `XKCD_CERULEAN_BLUE` from the set CSS4_COLORS. (Color number `724`)
pub const XKCD_CERULEAN_BLUE: Color = Color(5, 110, 238);
/// Color `XKCD_GOLDEN_BROWN` from the set CSS4_COLORS. (Color number `725`)
pub const XKCD_GOLDEN_BROWN: Color = Color(178, 122, 1);
/// Color `XKCD_BRIGHT_TURQUOISE` from the set CSS4_COLORS. (Color number `726`)
pub const XKCD_BRIGHT_TURQUOISE: Color = Color(15, 254, 249);
/// Color `XKCD_RED_PINK` from the set CSS4_COLORS. (Color number `727`)
pub const XKCD_RED_PINK: Color = Color(250, 42, 85);
/// Color `XKCD_RED_PURPLE` from the set CSS4_COLORS. (Color number `728`)
pub const XKCD_RED_PURPLE: Color = Color(130, 7, 71);
/// Color `XKCD_GREYISH_BROWN` from the set CSS4_COLORS. (Color number `729`)
pub const XKCD_GREYISH_BROWN: Color = Color(122, 106, 79);
/// Color `XKCD_VERMILLION` from the set CSS4_COLORS. (Color number `730`)
pub const XKCD_VERMILLION: Color = Color(244, 50, 12);
/// Color `XKCD_RUSSET` from the set CSS4_COLORS. (Color number `731`)
pub const XKCD_RUSSET: Color = Color(161, 57, 5);
/// Color `XKCD_STEEL_GREY` from the set CSS4_COLORS. (Color number `732`)
pub const XKCD_STEEL_GREY: Color = Color(111, 130, 138);
/// Color `XKCD_LIGHTER_PURPLE` from the set CSS4_COLORS. (Color number `733`)
pub const XKCD_LIGHTER_PURPLE: Color = Color(165, 90, 244);
/// Color `XKCD_BRIGHT_VIOLET` from the set CSS4_COLORS. (Color number `734`)
pub const XKCD_BRIGHT_VIOLET: Color = Color(173, 10, 253);
/// Color `XKCD_PRUSSIAN_BLUE` from the set CSS4_COLORS. (Color number `735`)
pub const XKCD_PRUSSIAN_BLUE: Color = Color(0, 69, 119);
/// Color `XKCD_SLATE_GREEN` from the set CSS4_COLORS. (Color number `736`)
pub const XKCD_SLATE_GREEN: Color = Color(101, 141, 109);
/// Color `XKCD_DIRTY_PINK` from the set CSS4_COLORS. (Color number `737`)
pub const XKCD_DIRTY_PINK: Color = Color(202, 123, 128);
/// Color `XKCD_DARK_BLUE_GREEN` from the set CSS4_COLORS. (Color number `738`)
pub const XKCD_DARK_BLUE_GREEN: Color = Color(0, 82, 73);
/// Color `XKCD_PINE` from the set CSS4_COLORS. (Color number `739`)
pub const XKCD_PINE: Color = Color(43, 93, 52);
/// Color `XKCD_YELLOWY_GREEN` from the set CSS4_COLORS. (Color number `740`)
pub const XKCD_YELLOWY_GREEN: Color = Color(191, 241, 40);
/// Color `XKCD_DARK_GOLD` from the set CSS4_COLORS. (Color number `741`)
pub const XKCD_DARK_GOLD: Color = Color(181, 148, 16);
/// Color `XKCD_BLUISH` from the set CSS4_COLORS. (Color number `742`)
pub const XKCD_BLUISH: Color = Color(41, 118, 187);
/// Color `XKCD_DARKISH_BLUE` from the set CSS4_COLORS. (Color number `743`)
pub const XKCD_DARKISH_BLUE: Color = Color(1, 65, 130);
/// Color `XKCD_DULL_RED` from the set CSS4_COLORS. (Color number `744`)
pub const XKCD_DULL_RED: Color = Color(187, 63, 63);
/// Color `XKCD_PINKY_RED` from the set CSS4_COLORS. (Color number `745`)
pub const XKCD_PINKY_RED: Color = Color(252, 38, 71);
/// Color `XKCD_BRONZE` from the set CSS4_COLORS. (Color number `746`)
pub const XKCD_BRONZE: Color = Color(168, 121, 0);
/// Color `XKCD_PALE_TEAL` from the set CSS4_COLORS. (Color number `747`)
pub const XKCD_PALE_TEAL: Color = Color(130, 203, 178);
/// Color `XKCD_MILITARY_GREEN` from the set CSS4_COLORS. (Color number `748`)
pub const XKCD_MILITARY_GREEN: Color = Color(102, 124, 62);
/// Color `XKCD_BARBIE_PINK` from the set CSS4_COLORS. (Color number `749`)
pub const XKCD_BARBIE_PINK: Color = Color(254, 70, 165);
/// Color `XKCD_BUBBLEGUM_PINK` from the set CSS4_COLORS. (Color number `750`)
pub const XKCD_BUBBLEGUM_PINK: Color = Color(254, 131, 204);
/// Color `XKCD_PEA_SOUP_GREEN` from the set CSS4_COLORS. (Color number `751`)
pub const XKCD_PEA_SOUP_GREEN: Color = Color(148, 166, 23);
/// Color `XKCD_DARK_MUSTARD` from the set CSS4_COLORS. (Color number `752`)
pub const XKCD_DARK_MUSTARD: Color = Color(168, 137, 5);
/// Color `XKCD_SHIT` from the set CSS4_COLORS. (Color number `753`)
pub const XKCD_SHIT: Color = Color(127, 95, 0);
/// Color `XKCD_MEDIUM_PURPLE` from the set CSS4_COLORS. (Color number `754`)
pub const XKCD_MEDIUM_PURPLE: Color = Color(158, 67, 162);
/// Color `XKCD_VERY_DARK_GREEN` from the set CSS4_COLORS. (Color number `755`)
pub const XKCD_VERY_DARK_GREEN: Color = Color(6, 46, 3);
/// Color `XKCD_DIRT` from the set CSS4_COLORS. (Color number `756`)
pub const XKCD_DIRT: Color = Color(138, 110, 69);
/// Color `XKCD_DUSKY_PINK` from the set CSS4_COLORS. (Color number `757`)
pub const XKCD_DUSKY_PINK: Color = Color(204, 122, 139);
/// Color `XKCD_RED_VIOLET` from the set CSS4_COLORS. (Color number `758`)
pub const XKCD_RED_VIOLET: Color = Color(158, 1, 104);
/// Color `XKCD_LEMON_YELLOW` from the set CSS4_COLORS. (Color number `759`)
pub const XKCD_LEMON_YELLOW: Color = Color(253, 255, 56);
/// Color `XKCD_PISTACHIO` from the set CSS4_COLORS. (Color number `760`)
pub const XKCD_PISTACHIO: Color = Color(192, 250, 139);
/// Color `XKCD_DULL_YELLOW` from the set CSS4_COLORS. (Color number `761`)
pub const XKCD_DULL_YELLOW: Color = Color(238, 220, 91);
/// Color `XKCD_DARK_LIME_GREEN` from the set CSS4_COLORS. (Color number `762`)
pub const XKCD_DARK_LIME_GREEN: Color = Color(126, 189, 1);
/// Color `XKCD_DENIM_BLUE` from the set CSS4_COLORS. (Color number `763`)
pub const XKCD_DENIM_BLUE: Color = Color(59, 91, 146);
/// Color `XKCD_TEAL_BLUE` from the set CSS4_COLORS. (Color number `764`)
pub const XKCD_TEAL_BLUE: Color = Color(1, 136, 159);
/// Color `XKCD_LIGHTISH_BLUE` from the set CSS4_COLORS. (Color number `765`)
pub const XKCD_LIGHTISH_BLUE: Color = Color(61, 122, 253);
/// Color `XKCD_PURPLEY_BLUE` from the set CSS4_COLORS. (Color number `766`)
pub const XKCD_PURPLEY_BLUE: Color = Color(95, 52, 231);
/// Color `XKCD_LIGHT_INDIGO` from the set CSS4_COLORS. (Color number `767`)
pub const XKCD_LIGHT_INDIGO: Color = Color(109, 90, 207);
/// Color `XKCD_SWAMP_GREEN` from the set CSS4_COLORS. (Color number `768`)
pub const XKCD_SWAMP_GREEN: Color = Color(116, 133, 0);
/// Color `XKCD_BROWN_GREEN` from the set CSS4_COLORS. (Color number `769`)
pub const XKCD_BROWN_GREEN: Color = Color(112, 108, 17);
/// Color `XKCD_DARK_MAROON` from the set CSS4_COLORS. (Color number `770`)
pub const XKCD_DARK_MAROON: Color = Color(60, 0, 8);
/// Color `XKCD_HOT_PURPLE` from the set CSS4_COLORS. (Color number `771`)
pub const XKCD_HOT_PURPLE: Color = Color(203, 0, 245);
/// Color `XKCD_DARK_FOREST_GREEN` from the set CSS4_COLORS. (Color number `772`)
pub const XKCD_DARK_FOREST_GREEN: Color = Color(0, 45, 4);
/// Color `XKCD_FADED_BLUE` from the set CSS4_COLORS. (Color number `773`)
pub const XKCD_FADED_BLUE: Color = Color(101, 140, 187);
/// Color `XKCD_DRAB_GREEN` from the set CSS4_COLORS. (Color number `774`)
pub const XKCD_DRAB_GREEN: Color = Color(116, 149, 81);
/// Color `XKCD_LIGHT_LIME_GREEN` from the set CSS4_COLORS. (Color number `775`)
pub const XKCD_LIGHT_LIME_GREEN: Color = Color(185, 255, 102);
/// Color `XKCD_SNOT_GREEN` from the set CSS4_COLORS. (Color number `776`)
pub const XKCD_SNOT_GREEN: Color = Color(157, 193, 0);
/// Color `XKCD_YELLOWISH` from the set CSS4_COLORS. (Color number `777`)
pub const XKCD_YELLOWISH: Color = Color(250, 238, 102);
/// Color `XKCD_LIGHT_BLUE_GREEN` from the set CSS4_COLORS. (Color number `778`)
pub const XKCD_LIGHT_BLUE_GREEN: Color = Color(126, 251, 179);
/// Color `XKCD_BORDEAUX` from the set CSS4_COLORS. (Color number `779`)
pub const XKCD_BORDEAUX: Color = Color(123, 0, 44);
/// Color `XKCD_LIGHT_MAUVE` from the set CSS4_COLORS. (Color number `780`)
pub const XKCD_LIGHT_MAUVE: Color = Color(194, 146, 161);
/// Color `XKCD_OCEAN` from the set CSS4_COLORS. (Color number `781`)
pub const XKCD_OCEAN: Color = Color(1, 123, 146);
/// Color `XKCD_MARIGOLD` from the set CSS4_COLORS. (Color number `782`)
pub const XKCD_MARIGOLD: Color = Color(252, 192, 6);
/// Color `XKCD_MUDDY_GREEN` from the set CSS4_COLORS. (Color number `783`)
pub const XKCD_MUDDY_GREEN: Color = Color(101, 116, 50);
/// Color `XKCD_DULL_ORANGE` from the set CSS4_COLORS. (Color number `784`)
pub const XKCD_DULL_ORANGE: Color = Color(216, 134, 59);
/// Color `XKCD_STEEL` from the set CSS4_COLORS. (Color number `785`)
pub const XKCD_STEEL: Color = Color(115, 133, 149);
/// Color `XKCD_ELECTRIC_PURPLE` from the set CSS4_COLORS. (Color number `786`)
pub const XKCD_ELECTRIC_PURPLE: Color = Color(170, 35, 255);
/// Color `XKCD_FLUORESCENT_GREEN` from the set CSS4_COLORS. (Color number `787`)
pub const XKCD_FLUORESCENT_GREEN: Color = Color(8, 255, 8);
/// Color `XKCD_YELLOWISH_BROWN` from the set CSS4_COLORS. (Color number `788`)
pub const XKCD_YELLOWISH_BROWN: Color = Color(155, 122, 1);
/// Color `XKCD_BLUSH` from the set CSS4_COLORS. (Color number `789`)
pub const XKCD_BLUSH: Color = Color(242, 158, 142);
/// Color `XKCD_SOFT_GREEN` from the set CSS4_COLORS. (Color number `790`)
pub const XKCD_SOFT_GREEN: Color = Color(111, 194, 118);
/// Color `XKCD_BRIGHT_ORANGE` from the set CSS4_COLORS. (Color number `791`)
pub const XKCD_BRIGHT_ORANGE: Color = Color(255, 91, 0);
/// Color `XKCD_LEMON` from the set CSS4_COLORS. (Color number `792`)
pub const XKCD_LEMON: Color = Color(253, 255, 82);
/// Color `XKCD_PURPLE_GREY` from the set CSS4_COLORS. (Color number `793`)
pub const XKCD_PURPLE_GREY: Color = Color(134, 111, 133);
/// Color `XKCD_ACID_GREEN` from the set CSS4_COLORS. (Color number `794`)
pub const XKCD_ACID_GREEN: Color = Color(143, 254, 9);
/// Color `XKCD_PALE_LAVENDER` from the set CSS4_COLORS. (Color number `795`)
pub const XKCD_PALE_LAVENDER: Color = Color(238, 207, 254);
/// Color `XKCD_VIOLET_BLUE` from the set CSS4_COLORS. (Color number `796`)
pub const XKCD_VIOLET_BLUE: Color = Color(81, 10, 201);
/// Color `XKCD_LIGHT_FOREST_GREEN` from the set CSS4_COLORS. (Color number `797`)
pub const XKCD_LIGHT_FOREST_GREEN: Color = Color(79, 145, 83);
/// Color `XKCD_BURNT_RED` from the set CSS4_COLORS. (Color number `798`)
pub const XKCD_BURNT_RED: Color = Color(159, 35, 5);
/// Color `XKCD_KHAKI_GREEN` from the set CSS4_COLORS. (Color number `799`)
pub const XKCD_KHAKI_GREEN: Color = Color(114, 134, 57);
/// Color `XKCD_CERISE` from the set CSS4_COLORS. (Color number `800`)
pub const XKCD_CERISE: Color = Color(222, 12, 98);
/// Color `XKCD_FADED_PURPLE` from the set CSS4_COLORS. (Color number `801`)
pub const XKCD_FADED_PURPLE: Color = Color(145, 110, 153);
/// Color `XKCD_APRICOT` from the set CSS4_COLORS. (Color number `802`)
pub const XKCD_APRICOT: Color = Color(255, 177, 109);
/// Color `XKCD_DARK_OLIVE_GREEN` from the set CSS4_COLORS. (Color number `803`)
pub const XKCD_DARK_OLIVE_GREEN: Color = Color(60, 77, 3);
/// Color `XKCD_GREY_BROWN` from the set CSS4_COLORS. (Color number `804`)
pub const XKCD_GREY_BROWN: Color = Color(127, 112, 83);
/// Color `XKCD_GREEN_GREY` from the set CSS4_COLORS. (Color number `805`)
pub const XKCD_GREEN_GREY: Color = Color(119, 146, 111);
/// Color `XKCD_TRUE_BLUE` from the set CSS4_COLORS. (Color number `806`)
pub const XKCD_TRUE_BLUE: Color = Color(1, 15, 204);
/// Color `XKCD_PALE_VIOLET` from the set CSS4_COLORS. (Color number `807`)
pub const XKCD_PALE_VIOLET: Color = Color(206, 174, 250);
/// Color `XKCD_PERIWINKLE_BLUE` from the set CSS4_COLORS. (Color number `808`)
pub const XKCD_PERIWINKLE_BLUE: Color = Color(143, 153, 251);
/// Color `XKCD_LIGHT_SKY_BLUE` from the set CSS4_COLORS. (Color number `809`)
pub const XKCD_LIGHT_SKY_BLUE: Color = Color(198, 252, 255);
/// Color `XKCD_BLURPLE` from the set CSS4_COLORS. (Color number `810`)
pub const XKCD_BLURPLE: Color = Color(85, 57, 204);
/// Color `XKCD_GREEN_BROWN` from the set CSS4_COLORS. (Color number `811`)
pub const XKCD_GREEN_BROWN: Color = Color(84, 78, 3);
/// Color `XKCD_BLUEGREEN` from the set CSS4_COLORS. (Color number `812`)
pub const XKCD_BLUEGREEN: Color = Color(1, 122, 121);
/// Color `XKCD_BRIGHT_TEAL` from the set CSS4_COLORS. (Color number `813`)
pub const XKCD_BRIGHT_TEAL: Color = Color(1, 249, 198);
/// Color `XKCD_BROWNISH_YELLOW` from the set CSS4_COLORS. (Color number `814`)
pub const XKCD_BROWNISH_YELLOW: Color = Color(201, 176, 3);
/// Color `XKCD_PEA_SOUP` from the set CSS4_COLORS. (Color number `815`)
pub const XKCD_PEA_SOUP: Color = Color(146, 153, 1);
/// Color `XKCD_FOREST` from the set CSS4_COLORS. (Color number `816`)
pub const XKCD_FOREST: Color = Color(11, 85, 9);
/// Color `XKCD_BARNEY_PURPLE` from the set CSS4_COLORS. (Color number `817`)
pub const XKCD_BARNEY_PURPLE: Color = Color(160, 4, 152);
/// Color `XKCD_ULTRAMARINE` from the set CSS4_COLORS. (Color number `818`)
pub const XKCD_ULTRAMARINE: Color = Color(32, 0, 177);
/// Color `XKCD_PURPLISH` from the set CSS4_COLORS. (Color number `819`)
pub const XKCD_PURPLISH: Color = Color(148, 86, 140);
/// Color `XKCD_PUKE_YELLOW` from the set CSS4_COLORS. (Color number `820`)
pub const XKCD_PUKE_YELLOW: Color = Color(194, 190, 14);
/// Color `XKCD_BLUISH_GREY` from the set CSS4_COLORS. (Color number `821`)
pub const XKCD_BLUISH_GREY: Color = Color(116, 139, 151);
/// Color `XKCD_DARK_PERIWINKLE` from the set CSS4_COLORS. (Color number `822`)
pub const XKCD_DARK_PERIWINKLE: Color = Color(102, 95, 209);
/// Color `XKCD_DARK_LILAC` from the set CSS4_COLORS. (Color number `823`)
pub const XKCD_DARK_LILAC: Color = Color(156, 109, 165);
/// Color `XKCD_REDDISH` from the set CSS4_COLORS. (Color number `824`)
pub const XKCD_REDDISH: Color = Color(196, 66, 64);
/// Color `XKCD_LIGHT_MAROON` from the set CSS4_COLORS. (Color number `825`)
pub const XKCD_LIGHT_MAROON: Color = Color(162, 72, 87);
/// Color `XKCD_DUSTY_PURPLE` from the set CSS4_COLORS. (Color number `826`)
pub const XKCD_DUSTY_PURPLE: Color = Color(130, 95, 135);
/// Color `XKCD_TERRA_COTTA` from the set CSS4_COLORS. (Color number `827`)
pub const XKCD_TERRA_COTTA: Color = Color(201, 100, 59);
/// Color `XKCD_AVOCADO` from the set CSS4_COLORS. (Color number `828`)
pub const XKCD_AVOCADO: Color = Color(144, 177, 52);
/// Color `XKCD_MARINE_BLUE` from the set CSS4_COLORS. (Color number `829`)
pub const XKCD_MARINE_BLUE: Color = Color(1, 56, 106);
/// Color `XKCD_TEAL_GREEN` from the set CSS4_COLORS. (Color number `830`)
pub const XKCD_TEAL_GREEN: Color = Color(37, 163, 111);
/// Color `XKCD_SLATE_GREY` from the set CSS4_COLORS. (Color number `831`)
pub const XKCD_SLATE_GREY: Color = Color(89, 101, 109);
/// Color `XKCD_LIGHTER_GREEN` from the set CSS4_COLORS. (Color number `832`)
pub const XKCD_LIGHTER_GREEN: Color = Color(117, 253, 99);
/// Color `XKCD_ELECTRIC_GREEN` from the set CSS4_COLORS. (Color number `833`)
pub const XKCD_ELECTRIC_GREEN: Color = Color(33, 252, 13);
/// Color `XKCD_DUSTY_BLUE` from the set CSS4_COLORS. (Color number `834`)
pub const XKCD_DUSTY_BLUE: Color = Color(90, 134, 173);
/// Color `XKCD_GOLDEN_YELLOW` from the set CSS4_COLORS. (Color number `835`)
pub const XKCD_GOLDEN_YELLOW: Color = Color(254, 198, 21);
/// Color `XKCD_BRIGHT_YELLOW` from the set CSS4_COLORS. (Color number `836`)
pub const XKCD_BRIGHT_YELLOW: Color = Color(255, 253, 1);
/// Color `XKCD_LIGHT_LAVENDER` from the set CSS4_COLORS. (Color number `837`)
pub const XKCD_LIGHT_LAVENDER: Color = Color(223, 197, 254);
/// Color `XKCD_UMBER` from the set CSS4_COLORS. (Color number `838`)
pub const XKCD_UMBER: Color = Color(178, 100, 0);
/// Color `XKCD_POOP` from the set CSS4_COLORS. (Color number `839`)
pub const XKCD_POOP: Color = Color(127, 94, 0);
/// Color `XKCD_DARK_PEACH` from the set CSS4_COLORS. (Color number `840`)
pub const XKCD_DARK_PEACH: Color = Color(222, 126, 93);
/// Color `XKCD_JUNGLE_GREEN` from the set CSS4_COLORS. (Color number `841`)
pub const XKCD_JUNGLE_GREEN: Color = Color(4, 130, 67);
/// Color `XKCD_EGGSHELL` from the set CSS4_COLORS. (Color number `842`)
pub const XKCD_EGGSHELL: Color = Color(255, 255, 212);
/// Color `XKCD_DENIM` from the set CSS4_COLORS. (Color number `843`)
pub const XKCD_DENIM: Color = Color(59, 99, 140);
/// Color `XKCD_YELLOW_BROWN` from the set CSS4_COLORS. (Color number `844`)
pub const XKCD_YELLOW_BROWN: Color = Color(183, 148, 0);
/// Color `XKCD_DULL_PURPLE` from the set CSS4_COLORS. (Color number `845`)
pub const XKCD_DULL_PURPLE: Color = Color(132, 89, 126);
/// Color `XKCD_CHOCOLATE_BROWN` from the set CSS4_COLORS. (Color number `846`)
pub const XKCD_CHOCOLATE_BROWN: Color = Color(65, 25, 0);
/// Color `XKCD_WINE_RED` from the set CSS4_COLORS. (Color number `847`)
pub const XKCD_WINE_RED: Color = Color(123, 3, 35);
/// Color `XKCD_NEON_BLUE` from the set CSS4_COLORS. (Color number `848`)
pub const XKCD_NEON_BLUE: Color = Color(4, 217, 255);
/// Color `XKCD_DIRTY_GREEN` from the set CSS4_COLORS. (Color number `849`)
pub const XKCD_DIRTY_GREEN: Color = Color(102, 126, 44);
/// Color `XKCD_LIGHT_TAN` from the set CSS4_COLORS. (Color number `850`)
pub const XKCD_LIGHT_TAN: Color = Color(251, 238, 172);
/// Color `XKCD_ICE_BLUE` from the set CSS4_COLORS. (Color number `851`)
pub const XKCD_ICE_BLUE: Color = Color(215, 255, 254);
/// Color `XKCD_CADET_BLUE` from the set CSS4_COLORS. (Color number `852`)
pub const XKCD_CADET_BLUE: Color = Color(78, 116, 150);
/// Color `XKCD_DARK_MAUVE` from the set CSS4_COLORS. (Color number `853`)
pub const XKCD_DARK_MAUVE: Color = Color(135, 76, 98);
/// Color `XKCD_VERY_LIGHT_BLUE` from the set CSS4_COLORS. (Color number `854`)
pub const XKCD_VERY_LIGHT_BLUE: Color = Color(213, 255, 255);
/// Color `XKCD_GREY_PURPLE` from the set CSS4_COLORS. (Color number `855`)
pub const XKCD_GREY_PURPLE: Color = Color(130, 109, 140);
/// Color `XKCD_PASTEL_PINK` from the set CSS4_COLORS. (Color number `856`)
pub const XKCD_PASTEL_PINK: Color = Color(255, 186, 205);
/// Color `XKCD_VERY_LIGHT_GREEN` from the set CSS4_COLORS. (Color number `857`)
pub const XKCD_VERY_LIGHT_GREEN: Color = Color(209, 255, 189);
/// Color `XKCD_DARK_SKY_BLUE` from the set CSS4_COLORS. (Color number `858`)
pub const XKCD_DARK_SKY_BLUE: Color = Color(68, 142, 228);
/// Color `XKCD_EVERGREEN` from the set CSS4_COLORS. (Color number `859`)
pub const XKCD_EVERGREEN: Color = Color(5, 71, 42);
/// Color `XKCD_DULL_PINK` from the set CSS4_COLORS. (Color number `860`)
pub const XKCD_DULL_PINK: Color = Color(213, 134, 157);
/// Color `XKCD_AUBERGINE` from the set CSS4_COLORS. (Color number `861`)
pub const XKCD_AUBERGINE: Color = Color(61, 7, 52);
/// Color `XKCD_MAHOGANY` from the set CSS4_COLORS. (Color number `862`)
pub const XKCD_MAHOGANY: Color = Color(74, 1, 0);
/// Color `XKCD_REDDISH_ORANGE` from the set CSS4_COLORS. (Color number `863`)
pub const XKCD_REDDISH_ORANGE: Color = Color(248, 72, 28);
/// Color `XKCD_DEEP_GREEN` from the set CSS4_COLORS. (Color number `864`)
pub const XKCD_DEEP_GREEN: Color = Color(2, 89, 15);
/// Color `XKCD_VOMIT_GREEN` from the set CSS4_COLORS. (Color number `865`)
pub const XKCD_VOMIT_GREEN: Color = Color(137, 162, 3);
/// Color `XKCD_PURPLE_PINK` from the set CSS4_COLORS. (Color number `866`)
pub const XKCD_PURPLE_PINK: Color = Color(224, 63, 216);
/// Color `XKCD_DUSTY_PINK` from the set CSS4_COLORS. (Color number `867`)
pub const XKCD_DUSTY_PINK: Color = Color(213, 138, 148);
/// Color `XKCD_FADED_GREEN` from the set CSS4_COLORS. (Color number `868`)
pub const XKCD_FADED_GREEN: Color = Color(123, 178, 116);
/// Color `XKCD_CAMO_GREEN` from the set CSS4_COLORS. (Color number `869`)
pub const XKCD_CAMO_GREEN: Color = Color(82, 101, 37);
/// Color `XKCD_PINKY_PURPLE` from the set CSS4_COLORS. (Color number `870`)
pub const XKCD_PINKY_PURPLE: Color = Color(201, 76, 190);
/// Color `XKCD_PINK_PURPLE` from the set CSS4_COLORS. (Color number `871`)
pub const XKCD_PINK_PURPLE: Color = Color(219, 75, 218);
/// Color `XKCD_BROWNISH_RED` from the set CSS4_COLORS. (Color number `872`)
pub const XKCD_BROWNISH_RED: Color = Color(158, 54, 35);
/// Color `XKCD_DARK_ROSE` from the set CSS4_COLORS. (Color number `873`)
pub const XKCD_DARK_ROSE: Color = Color(181, 72, 93);
/// Color `XKCD_MUD` from the set CSS4_COLORS. (Color number `874`)
pub const XKCD_MUD: Color = Color(115, 92, 18);
/// Color `XKCD_BROWNISH` from the set CSS4_COLORS. (Color number `875`)
pub const XKCD_BROWNISH: Color = Color(156, 109, 87);
/// Color `XKCD_EMERALD_GREEN` from the set CSS4_COLORS. (Color number `876`)
pub const XKCD_EMERALD_GREEN: Color = Color(2, 143, 30);
/// Color `XKCD_PALE_BROWN` from the set CSS4_COLORS. (Color number `877`)
pub const XKCD_PALE_BROWN: Color = Color(177, 145, 110);
/// Color `XKCD_DULL_BLUE` from the set CSS4_COLORS. (Color number `878`)
pub const XKCD_DULL_BLUE: Color = Color(73, 117, 156);
/// Color `XKCD_BURNT_UMBER` from the set CSS4_COLORS. (Color number `879`)
pub const XKCD_BURNT_UMBER: Color = Color(160, 69, 14);
/// Color `XKCD_MEDIUM_GREEN` from the set CSS4_COLORS. (Color number `880`)
pub const XKCD_MEDIUM_GREEN: Color = Color(57, 173, 72);
/// Color `XKCD_CLAY` from the set CSS4_COLORS. (Color number `881`)
pub const XKCD_CLAY: Color = Color(182, 106, 80);
/// Color `XKCD_LIGHT_AQUA` from the set CSS4_COLORS. (Color number `882`)
pub const XKCD_LIGHT_AQUA: Color = Color(140, 255, 219);
/// Color `XKCD_LIGHT_OLIVE_GREEN` from the set CSS4_COLORS. (Color number `883`)
pub const XKCD_LIGHT_OLIVE_GREEN: Color = Color(164, 190, 92);
/// Color `XKCD_BROWNISH_ORANGE` from the set CSS4_COLORS. (Color number `884`)
pub const XKCD_BROWNISH_ORANGE: Color = Color(203, 119, 35);
/// Color `XKCD_DARK_AQUA` from the set CSS4_COLORS. (Color number `885`)
pub const XKCD_DARK_AQUA: Color = Color(5, 105, 107);
/// Color `XKCD_PURPLISH_PINK` from the set CSS4_COLORS. (Color number `886`)
pub const XKCD_PURPLISH_PINK: Color = Color(206, 93, 174);
/// Color `XKCD_DARK_SALMON` from the set CSS4_COLORS. (Color number `887`)
pub const XKCD_DARK_SALMON: Color = Color(200, 90, 83);
/// Color `XKCD_GREENISH_GREY` from the set CSS4_COLORS. (Color number `888`)
pub const XKCD_GREENISH_GREY: Color = Color(150, 174, 141);
/// Color `XKCD_JADE` from the set CSS4_COLORS. (Color number `889`)
pub const XKCD_JADE: Color = Color(31, 167, 116);
/// Color `XKCD_UGLY_GREEN` from the set CSS4_COLORS. (Color number `890`)
pub const XKCD_UGLY_GREEN: Color = Color(122, 151, 3);
/// Color `XKCD_DARK_BEIGE` from the set CSS4_COLORS. (Color number `891`)
pub const XKCD_DARK_BEIGE: Color = Color(172, 147, 98);
/// Color `XKCD_EMERALD` from the set CSS4_COLORS. (Color number `892`)
pub const XKCD_EMERALD: Color = Color(1, 160, 73);
/// Color `XKCD_PALE_RED` from the set CSS4_COLORS. (Color number `893`)
pub const XKCD_PALE_RED: Color = Color(217, 84, 77);
/// Color `XKCD_LIGHT_MAGENTA` from the set CSS4_COLORS. (Color number `894`)
pub const XKCD_LIGHT_MAGENTA: Color = Color(250, 95, 247);
/// Color `XKCD_SKY` from the set CSS4_COLORS. (Color number `895`)
pub const XKCD_SKY: Color = Color(130, 202, 252);
/// Color `XKCD_LIGHT_CYAN` from the set CSS4_COLORS. (Color number `896`)
pub const XKCD_LIGHT_CYAN: Color = Color(172, 255, 252);
/// Color `XKCD_YELLOW_ORANGE` from the set CSS4_COLORS. (Color number `897`)
pub const XKCD_YELLOW_ORANGE: Color = Color(252, 176, 1);
/// Color `XKCD_REDDISH_PURPLE` from the set CSS4_COLORS. (Color number `898`)
pub const XKCD_REDDISH_PURPLE: Color = Color(145, 9, 81);
/// Color `XKCD_REDDISH_PINK` from the set CSS4_COLORS. (Color number `899`)
pub const XKCD_REDDISH_PINK: Color = Color(254, 44, 84);
/// Color `XKCD_ORCHID` from the set CSS4_COLORS. (Color number `900`)
pub const XKCD_ORCHID: Color = Color(200, 117, 196);
/// Color `XKCD_DIRTY_YELLOW` from the set CSS4_COLORS. (Color number `901`)
pub const XKCD_DIRTY_YELLOW: Color = Color(205, 197, 10);
/// Color `XKCD_ORANGE_RED` from the set CSS4_COLORS. (Color number `902`)
pub const XKCD_ORANGE_RED: Color = Color(253, 65, 30);
/// Color `XKCD_DEEP_RED` from the set CSS4_COLORS. (Color number `903`)
pub const XKCD_DEEP_RED: Color = Color(154, 2, 0);
/// Color `XKCD_ORANGE_BROWN` from the set CSS4_COLORS. (Color number `904`)
pub const XKCD_ORANGE_BROWN: Color = Color(190, 100, 0);
/// Color `XKCD_COBALT_BLUE` from the set CSS4_COLORS. (Color number `905`)
pub const XKCD_COBALT_BLUE: Color = Color(3, 10, 167);
/// Color `XKCD_NEON_PINK` from the set CSS4_COLORS. (Color number `906`)
pub const XKCD_NEON_PINK: Color = Color(254, 1, 154);
/// Color `XKCD_ROSE_PINK` from the set CSS4_COLORS. (Color number `907`)
pub const XKCD_ROSE_PINK: Color = Color(247, 135, 154);
/// Color `XKCD_GREYISH_PURPLE` from the set CSS4_COLORS. (Color number `908`)
pub const XKCD_GREYISH_PURPLE: Color = Color(136, 113, 145);
/// Color `XKCD_RASPBERRY` from the set CSS4_COLORS. (Color number `909`)
pub const XKCD_RASPBERRY: Color = Color(176, 1, 73);
/// Color `XKCD_AQUA_GREEN` from the set CSS4_COLORS. (Color number `910`)
pub const XKCD_AQUA_GREEN: Color = Color(18, 225, 147);
/// Color `XKCD_SALMON_PINK` from the set CSS4_COLORS. (Color number `911`)
pub const XKCD_SALMON_PINK: Color = Color(254, 123, 124);
/// Color `XKCD_TANGERINE` from the set CSS4_COLORS. (Color number `912`)
pub const XKCD_TANGERINE: Color = Color(255, 148, 8);
/// Color `XKCD_BROWNISH_GREEN` from the set CSS4_COLORS. (Color number `913`)
pub const XKCD_BROWNISH_GREEN: Color = Color(106, 110, 9);
/// Color `XKCD_RED_BROWN` from the set CSS4_COLORS. (Color number `914`)
pub const XKCD_RED_BROWN: Color = Color(139, 46, 22);
/// Color `XKCD_GREENISH_BROWN` from the set CSS4_COLORS. (Color number `915`)
pub const XKCD_GREENISH_BROWN: Color = Color(105, 97, 18);
/// Color `XKCD_PUMPKIN` from the set CSS4_COLORS. (Color number `916`)
pub const XKCD_PUMPKIN: Color = Color(225, 119, 1);
/// Color `XKCD_PINE_GREEN` from the set CSS4_COLORS. (Color number `917`)
pub const XKCD_PINE_GREEN: Color = Color(10, 72, 30);
/// Color `XKCD_CHARCOAL` from the set CSS4_COLORS. (Color number `918`)
pub const XKCD_CHARCOAL: Color = Color(52, 56, 55);
/// Color `XKCD_BABY_PINK` from the set CSS4_COLORS. (Color number `919`)
pub const XKCD_BABY_PINK: Color = Color(255, 183, 206);
/// Color `XKCD_CORNFLOWER` from the set CSS4_COLORS. (Color number `920`)
pub const XKCD_CORNFLOWER: Color = Color(106, 121, 247);
/// Color `XKCD_BLUE_VIOLET` from the set CSS4_COLORS. (Color number `921`)
pub const XKCD_BLUE_VIOLET: Color = Color(93, 6, 233);
/// Color `XKCD_CHOCOLATE` from the set CSS4_COLORS. (Color number `922`)
pub const XKCD_CHOCOLATE: Color = Color(61, 28, 2);
/// Color `XKCD_GREYISH_GREEN` from the set CSS4_COLORS. (Color number `923`)
pub const XKCD_GREYISH_GREEN: Color = Color(130, 166, 125);
/// Color `XKCD_SCARLET` from the set CSS4_COLORS. (Color number `924`)
pub const XKCD_SCARLET: Color = Color(190, 1, 25);
/// Color `XKCD_GREEN_YELLOW` from the set CSS4_COLORS. (Color number `925`)
pub const XKCD_GREEN_YELLOW: Color = Color(201, 255, 39);
/// Color `XKCD_DARK_OLIVE` from the set CSS4_COLORS. (Color number `926`)
pub const XKCD_DARK_OLIVE: Color = Color(55, 62, 2);
/// Color `XKCD_SIENNA` from the set CSS4_COLORS. (Color number `927`)
pub const XKCD_SIENNA: Color = Color(169, 86, 30);
/// Color `XKCD_PASTEL_PURPLE` from the set CSS4_COLORS. (Color number `928`)
pub const XKCD_PASTEL_PURPLE: Color = Color(202, 160, 255);
/// Color `XKCD_TERRACOTTA` from the set CSS4_COLORS. (Color number `929`)
pub const XKCD_TERRACOTTA: Color = Color(202, 102, 65);
/// Color `XKCD_AQUA_BLUE` from the set CSS4_COLORS. (Color number `930`)
pub const XKCD_AQUA_BLUE: Color = Color(2, 216, 233);
/// Color `XKCD_SAGE_GREEN` from the set CSS4_COLORS. (Color number `931`)
pub const XKCD_SAGE_GREEN: Color = Color(136, 179, 120);
/// Color `XKCD_BLOOD_RED` from the set CSS4_COLORS. (Color number `932`)
pub const XKCD_BLOOD_RED: Color = Color(152, 0, 2);
/// Color `XKCD_DEEP_PINK` from the set CSS4_COLORS. (Color number `933`)
pub const XKCD_DEEP_PINK: Color = Color(203, 1, 98);
/// Color `XKCD_GRASS` from the set CSS4_COLORS. (Color number `934`)
pub const XKCD_GRASS: Color = Color(92, 172, 45);
/// Color `XKCD_MOSS` from the set CSS4_COLORS. (Color number `935`)
pub const XKCD_MOSS: Color = Color(118, 153, 88);
/// Color `XKCD_PASTEL_BLUE` from the set CSS4_COLORS. (Color number `936`)
pub const XKCD_PASTEL_BLUE: Color = Color(162, 191, 254);
/// Color `XKCD_BLUISH_GREEN` from the set CSS4_COLORS. (Color number `937`)
pub const XKCD_BLUISH_GREEN: Color = Color(16, 166, 116);
/// Color `XKCD_GREEN_BLUE` from the set CSS4_COLORS. (Color number `938`)
pub const XKCD_GREEN_BLUE: Color = Color(6, 180, 139);
/// Color `XKCD_DARK_TAN` from the set CSS4_COLORS. (Color number `939`)
pub const XKCD_DARK_TAN: Color = Color(175, 136, 74);
/// Color `XKCD_GREENISH_BLUE` from the set CSS4_COLORS. (Color number `940`)
pub const XKCD_GREENISH_BLUE: Color = Color(11, 139, 135);
/// Color `XKCD_PALE_ORANGE` from the set CSS4_COLORS. (Color number `941`)
pub const XKCD_PALE_ORANGE: Color = Color(255, 167, 86);
/// Color `XKCD_VOMIT` from the set CSS4_COLORS. (Color number `942`)
pub const XKCD_VOMIT: Color = Color(162, 164, 21);
/// Color `XKCD_FORREST_GREEN` from the set CSS4_COLORS. (Color number `943`)
pub const XKCD_FORREST_GREEN: Color = Color(21, 68, 6);
/// Color `XKCD_DARK_LAVENDER` from the set CSS4_COLORS. (Color number `944`)
pub const XKCD_DARK_LAVENDER: Color = Color(133, 103, 152);
/// Color `XKCD_DARK_VIOLET` from the set CSS4_COLORS. (Color number `945`)
pub const XKCD_DARK_VIOLET: Color = Color(52, 1, 63);
/// Color `XKCD_PURPLE_BLUE` from the set CSS4_COLORS. (Color number `946`)
pub const XKCD_PURPLE_BLUE: Color = Color(99, 45, 233);
/// Color `XKCD_DARK_CYAN` from the set CSS4_COLORS. (Color number `947`)
pub const XKCD_DARK_CYAN: Color = Color(10, 136, 138);
/// Color `XKCD_OLIVE_DRAB` from the set CSS4_COLORS. (Color number `948`)
pub const XKCD_OLIVE_DRAB: Color = Color(111, 118, 50);
/// Color `XKCD_PINKISH` from the set CSS4_COLORS. (Color number `949`)
pub const XKCD_PINKISH: Color = Color(212, 106, 126);
/// Color `XKCD_COBALT` from the set CSS4_COLORS. (Color number `950`)
pub const XKCD_COBALT: Color = Color(30, 72, 143);
/// Color `XKCD_NEON_PURPLE` from the set CSS4_COLORS. (Color number `951`)
pub const XKCD_NEON_PURPLE: Color = Color(188, 19, 254);
/// Color `XKCD_LIGHT_TURQUOISE` from the set CSS4_COLORS. (Color number `952`)
pub const XKCD_LIGHT_TURQUOISE: Color = Color(126, 244, 204);
/// Color `XKCD_APPLE_GREEN` from the set CSS4_COLORS. (Color number `953`)
pub const XKCD_APPLE_GREEN: Color = Color(118, 205, 38);
/// Color `XKCD_DULL_GREEN` from the set CSS4_COLORS. (Color number `954`)
pub const XKCD_DULL_GREEN: Color = Color(116, 166, 98);
/// Color `XKCD_WINE` from the set CSS4_COLORS. (Color number `955`)
pub const XKCD_WINE: Color = Color(128, 1, 63);
/// Color `XKCD_POWDER_BLUE` from the set CSS4_COLORS. (Color number `956`)
pub const XKCD_POWDER_BLUE: Color = Color(177, 209, 252);
/// Color `XKCD_OFF_WHITE` from the set CSS4_COLORS. (Color number `957`)
pub const XKCD_OFF_WHITE: Color = Color(255, 255, 228);
/// Color `XKCD_ELECTRIC_BLUE` from the set CSS4_COLORS. (Color number `958`)
pub const XKCD_ELECTRIC_BLUE: Color = Color(6, 82, 255);
/// Color `XKCD_DARK_TURQUOISE` from the set CSS4_COLORS. (Color number `959`)
pub const XKCD_DARK_TURQUOISE: Color = Color(4, 92, 90);
/// Color `XKCD_BLUE_PURPLE` from the set CSS4_COLORS. (Color number `960`)
pub const XKCD_BLUE_PURPLE: Color = Color(87, 41, 206);
/// Color `XKCD_AZURE` from the set CSS4_COLORS. (Color number `961`)
pub const XKCD_AZURE: Color = Color(6, 154, 243);
/// Color `XKCD_BRIGHT_RED` from the set CSS4_COLORS. (Color number `962`)
pub const XKCD_BRIGHT_RED: Color = Color(255, 0, 13);
/// Color `XKCD_PINKISH_RED` from the set CSS4_COLORS. (Color number `963`)
pub const XKCD_PINKISH_RED: Color = Color(241, 12, 69);
/// Color `XKCD_CORNFLOWER_BLUE` from the set CSS4_COLORS. (Color number `964`)
pub const XKCD_CORNFLOWER_BLUE: Color = Color(81, 112, 215);
/// Color `XKCD_LIGHT_OLIVE` from the set CSS4_COLORS. (Color number `965`)
pub const XKCD_LIGHT_OLIVE: Color = Color(172, 191, 105);
/// Color `XKCD_GRAPE` from the set CSS4_COLORS. (Color number `966`)
pub const XKCD_GRAPE: Color = Color(108, 52, 97);
/// Color `XKCD_GREYISH_BLUE` from the set CSS4_COLORS. (Color number `967`)
pub const XKCD_GREYISH_BLUE: Color = Color(94, 129, 157);
/// Color `XKCD_PURPLISH_BLUE` from the set CSS4_COLORS. (Color number `968`)
pub const XKCD_PURPLISH_BLUE: Color = Color(96, 30, 249);
/// Color `XKCD_YELLOWISH_GREEN` from the set CSS4_COLORS. (Color number `969`)
pub const XKCD_YELLOWISH_GREEN: Color = Color(176, 221, 22);
/// Color `XKCD_GREENISH_YELLOW` from the set CSS4_COLORS. (Color number `970`)
pub const XKCD_GREENISH_YELLOW: Color = Color(205, 253, 2);
/// Color `XKCD_MEDIUM_BLUE` from the set CSS4_COLORS. (Color number `971`)
pub const XKCD_MEDIUM_BLUE: Color = Color(44, 111, 187);
/// Color `XKCD_DUSTY_ROSE` from the set CSS4_COLORS. (Color number `972`)
pub const XKCD_DUSTY_ROSE: Color = Color(192, 115, 122);
/// Color `XKCD_LIGHT_VIOLET` from the set CSS4_COLORS. (Color number `973`)
pub const XKCD_LIGHT_VIOLET: Color = Color(214, 180, 252);
/// Color `XKCD_MIDNIGHT_BLUE` from the set CSS4_COLORS. (Color number `974`)
pub const XKCD_MIDNIGHT_BLUE: Color = Color(2, 0, 53);
/// Color `XKCD_BLUISH_PURPLE` from the set CSS4_COLORS. (Color number `975`)
pub const XKCD_BLUISH_PURPLE: Color = Color(112, 59, 231);
/// Color `XKCD_RED_ORANGE` from the set CSS4_COLORS. (Color number `976`)
pub const XKCD_RED_ORANGE: Color = Color(253, 60, 6);
/// Color `XKCD_DARK_MAGENTA` from the set CSS4_COLORS. (Color number `977`)
pub const XKCD_DARK_MAGENTA: Color = Color(150, 0, 86);
/// Color `XKCD_GREENISH` from the set CSS4_COLORS. (Color number `978`)
pub const XKCD_GREENISH: Color = Color(64, 163, 104);
/// Color `XKCD_OCEAN_BLUE` from the set CSS4_COLORS. (Color number `979`)
pub const XKCD_OCEAN_BLUE: Color = Color(3, 113, 156);
/// Color `XKCD_CORAL` from the set CSS4_COLORS. (Color number `980`)
pub const XKCD_CORAL: Color = Color(252, 90, 80);
/// Color `XKCD_CREAM` from the set CSS4_COLORS. (Color number `981`)
pub const XKCD_CREAM: Color = Color(255, 255, 194);
/// Color `XKCD_REDDISH_BROWN` from the set CSS4_COLORS. (Color number `982`)
pub const XKCD_REDDISH_BROWN: Color = Color(127, 43, 10);
/// Color `XKCD_BURNT_SIENNA` from the set CSS4_COLORS. (Color number `983`)
pub const XKCD_BURNT_SIENNA: Color = Color(176, 78, 15);
/// Color `XKCD_BRICK` from the set CSS4_COLORS. (Color number `984`)
pub const XKCD_BRICK: Color = Color(160, 54, 35);
/// Color `XKCD_SAGE` from the set CSS4_COLORS. (Color number `985`)
pub const XKCD_SAGE: Color = Color(135, 174, 115);
/// Color `XKCD_GREY_GREEN` from the set CSS4_COLORS. (Color number `986`)
pub const XKCD_GREY_GREEN: Color = Color(120, 155, 115);
/// Color `XKCD_WHITE` from the set CSS4_COLORS. (Color number `987`)
pub const XKCD_WHITE: Color = Color(255, 255, 255);
/// Color `XKCD_ROBIN_S_EGG_BLUE` from the set CSS4_COLORS. (Color number `988`)
pub const XKCD_ROBIN_S_EGG_BLUE: Color = Color(152, 239, 249);
/// Color `XKCD_MOSS_GREEN` from the set CSS4_COLORS. (Color number `989`)
pub const XKCD_MOSS_GREEN: Color = Color(101, 139, 56);
/// Color `XKCD_STEEL_BLUE` from the set CSS4_COLORS. (Color number `990`)
pub const XKCD_STEEL_BLUE: Color = Color(90, 125, 154);
/// Color `XKCD_EGGPLANT` from the set CSS4_COLORS. (Color number `991`)
pub const XKCD_EGGPLANT: Color = Color(56, 8, 53);
/// Color `XKCD_LIGHT_YELLOW` from the set CSS4_COLORS. (Color number `992`)
pub const XKCD_LIGHT_YELLOW: Color = Color(255, 254, 122);
/// Color `XKCD_LEAF_GREEN` from the set CSS4_COLORS. (Color number `993`)
pub const XKCD_LEAF_GREEN: Color = Color(92, 169, 4);
/// Color `XKCD_LIGHT_GREY` from the set CSS4_COLORS. (Color number `994`)
pub const XKCD_LIGHT_GREY: Color = Color(216, 220, 214);
/// Color `XKCD_PUKE` from the set CSS4_COLORS. (Color number `995`)
pub const XKCD_PUKE: Color = Color(165, 165, 2);
/// Color `XKCD_PINKISH_PURPLE` from the set CSS4_COLORS. (Color number `996`)
pub const XKCD_PINKISH_PURPLE: Color = Color(214, 72, 215);
/// Color `XKCD_SEA_BLUE` from the set CSS4_COLORS. (Color number `997`)
pub const XKCD_SEA_BLUE: Color = Color(4, 116, 149);
/// Color `XKCD_PALE_PURPLE` from the set CSS4_COLORS. (Color number `998`)
pub const XKCD_PALE_PURPLE: Color = Color(183, 144, 212);
/// Color `XKCD_SLATE_BLUE` from the set CSS4_COLORS. (Color number `999`)
pub const XKCD_SLATE_BLUE: Color = Color(91, 124, 153);
/// Color `XKCD_BLUE_GREY` from the set CSS4_COLORS. (Color number `1000`)
pub const XKCD_BLUE_GREY: Color = Color(96, 124, 142);
/// Color `XKCD_HUNTER_GREEN` from the set CSS4_COLORS. (Color number `1001`)
pub const XKCD_HUNTER_GREEN: Color = Color(11, 64, 8);
/// Color `XKCD_FUCHSIA` from the set CSS4_COLORS. (Color number `1002`)
pub const XKCD_FUCHSIA: Color = Color(237, 13, 217);
/// Color `XKCD_CRIMSON` from the set CSS4_COLORS. (Color number `1003`)
pub const XKCD_CRIMSON: Color = Color(140, 0, 15);
/// Color `XKCD_PALE_YELLOW` from the set CSS4_COLORS. (Color number `1004`)
pub const XKCD_PALE_YELLOW: Color = Color(255, 255, 132);
/// Color `XKCD_OCHRE` from the set CSS4_COLORS. (Color number `1005`)
pub const XKCD_OCHRE: Color = Color(191, 144, 5);
/// Color `XKCD_MUSTARD_YELLOW` from the set CSS4_COLORS. (Color number `1006`)
pub const XKCD_MUSTARD_YELLOW: Color = Color(210, 189, 10);
/// Color `XKCD_LIGHT_RED` from the set CSS4_COLORS. (Color number `1007`)
pub const XKCD_LIGHT_RED: Color = Color(255, 71, 76);
/// Color `XKCD_CERULEAN` from the set CSS4_COLORS. (Color number `1008`)
pub const XKCD_CERULEAN: Color = Color(4, 133, 209);
/// Color `XKCD_PALE_PINK` from the set CSS4_COLORS. (Color number `1009`)
pub const XKCD_PALE_PINK: Color = Color(255, 207, 220);
/// Color `XKCD_DEEP_BLUE` from the set CSS4_COLORS. (Color number `1010`)
pub const XKCD_DEEP_BLUE: Color = Color(4, 2, 115);
/// Color `XKCD_RUST` from the set CSS4_COLORS. (Color number `1011`)
pub const XKCD_RUST: Color = Color(168, 60, 9);
/// Color `XKCD_LIGHT_TEAL` from the set CSS4_COLORS. (Color number `1012`)
pub const XKCD_LIGHT_TEAL: Color = Color(144, 228, 193);
/// Color `XKCD_SLATE` from the set CSS4_COLORS. (Color number `1013`)
pub const XKCD_SLATE: Color = Color(81, 101, 114);
/// Color `XKCD_GOLDENROD` from the set CSS4_COLORS. (Color number `1014`)
pub const XKCD_GOLDENROD: Color = Color(250, 194, 5);
/// Color `XKCD_DARK_YELLOW` from the set CSS4_COLORS. (Color number `1015`)
pub const XKCD_DARK_YELLOW: Color = Color(213, 182, 10);
/// Color `XKCD_DARK_GREY` from the set CSS4_COLORS. (Color number `1016`)
pub const XKCD_DARK_GREY: Color = Color(54, 55, 55);
/// Color `XKCD_ARMY_GREEN` from the set CSS4_COLORS. (Color number `1017`)
pub const XKCD_ARMY_GREEN: Color = Color(75, 93, 22);
/// Color `XKCD_GREY_BLUE` from the set CSS4_COLORS. (Color number `1018`)
pub const XKCD_GREY_BLUE: Color = Color(107, 139, 164);
/// Color `XKCD_SEAFOAM` from the set CSS4_COLORS. (Color number `1019`)
pub const XKCD_SEAFOAM: Color = Color(128, 249, 173);
/// Color `XKCD_PUCE` from the set CSS4_COLORS. (Color number `1020`)
pub const XKCD_PUCE: Color = Color(165, 126, 82);
/// Color `XKCD_SPRING_GREEN` from the set CSS4_COLORS. (Color number `1021`)
pub const XKCD_SPRING_GREEN: Color = Color(169, 249, 113);
/// Color `XKCD_DARK_ORANGE` from the set CSS4_COLORS. (Color number `1022`)
pub const XKCD_DARK_ORANGE: Color = Color(198, 81, 2);
/// Color `XKCD_SAND` from the set CSS4_COLORS. (Color number `1023`)
pub const XKCD_SAND: Color = Color(226, 202, 118);
/// Color `XKCD_PASTEL_GREEN` from the set CSS4_COLORS. (Color number `1024`)
pub const XKCD_PASTEL_GREEN: Color = Color(176, 255, 157);
/// Color `XKCD_MINT` from the set CSS4_COLORS. (Color number `1025`)
pub const XKCD_MINT: Color = Color(159, 254, 176);
/// Color `XKCD_LIGHT_ORANGE` from the set CSS4_COLORS. (Color number `1026`)
pub const XKCD_LIGHT_ORANGE: Color = Color(253, 170, 72);
/// Color `XKCD_BRIGHT_PINK` from the set CSS4_COLORS. (Color number `1027`)
pub const XKCD_BRIGHT_PINK: Color = Color(254, 1, 177);
/// Color `XKCD_CHARTREUSE` from the set CSS4_COLORS. (Color number `1028`)
pub const XKCD_CHARTREUSE: Color = Color(193, 248, 10);
/// Color `XKCD_DEEP_PURPLE` from the set CSS4_COLORS. (Color number `1029`)
pub const XKCD_DEEP_PURPLE: Color = Color(54, 1, 63);
/// Color `XKCD_DARK_BROWN` from the set CSS4_COLORS. (Color number `1030`)
pub const XKCD_DARK_BROWN: Color = Color(52, 28, 2);
/// Color `XKCD_TAUPE` from the set CSS4_COLORS. (Color number `1031`)
pub const XKCD_TAUPE: Color = Color(185, 162, 129);
/// Color `XKCD_PEA_GREEN` from the set CSS4_COLORS. (Color number `1032`)
pub const XKCD_PEA_GREEN: Color = Color(142, 171, 18);
/// Color `XKCD_PUKE_GREEN` from the set CSS4_COLORS. (Color number `1033`)
pub const XKCD_PUKE_GREEN: Color = Color(154, 174, 7);
/// Color `XKCD_KELLY_GREEN` from the set CSS4_COLORS. (Color number `1034`)
pub const XKCD_KELLY_GREEN: Color = Color(2, 171, 46);
/// Color `XKCD_SEAFOAM_GREEN` from the set CSS4_COLORS. (Color number `1035`)
pub const XKCD_SEAFOAM_GREEN: Color = Color(122, 249, 171);
/// Color `XKCD_BLUE_GREEN` from the set CSS4_COLORS. (Color number `1036`)
pub const XKCD_BLUE_GREEN: Color = Color(19, 126, 109);
/// Color `XKCD_KHAKI` from the set CSS4_COLORS. (Color number `1037`)
pub const XKCD_KHAKI: Color = Color(170, 166, 98);
/// Color `XKCD_BURGUNDY` from the set CSS4_COLORS. (Color number `1038`)
pub const XKCD_BURGUNDY: Color = Color(97, 0, 35);
/// Color `XKCD_DARK_TEAL` from the set CSS4_COLORS. (Color number `1039`)
pub const XKCD_DARK_TEAL: Color = Color(1, 77, 78);
/// Color `XKCD_BRICK_RED` from the set CSS4_COLORS. (Color number `1040`)
pub const XKCD_BRICK_RED: Color = Color(143, 20, 2);
/// Color `XKCD_ROYAL_PURPLE` from the set CSS4_COLORS. (Color number `1041`)
pub const XKCD_ROYAL_PURPLE: Color = Color(75, 0, 110);
/// Color `XKCD_PLUM` from the set CSS4_COLORS. (Color number `1042`)
pub const XKCD_PLUM: Color = Color(88, 15, 65);
/// Color `XKCD_MINT_GREEN` from the set CSS4_COLORS. (Color number `1043`)
pub const XKCD_MINT_GREEN: Color = Color(143, 255, 159);
/// Color `XKCD_GOLD` from the set CSS4_COLORS. (Color number `1044`)
pub const XKCD_GOLD: Color = Color(219, 180, 12);
/// Color `XKCD_BABY_BLUE` from the set CSS4_COLORS. (Color number `1045`)
pub const XKCD_BABY_BLUE: Color = Color(162, 207, 254);
/// Color `XKCD_YELLOW_GREEN` from the set CSS4_COLORS. (Color number `1046`)
pub const XKCD_YELLOW_GREEN: Color = Color(192, 251, 45);
/// Color `XKCD_BRIGHT_PURPLE` from the set CSS4_COLORS. (Color number `1047`)
pub const XKCD_BRIGHT_PURPLE: Color = Color(190, 3, 253);
/// Color `XKCD_DARK_RED` from the set CSS4_COLORS. (Color number `1048`)
pub const XKCD_DARK_RED: Color = Color(132, 0, 0);
/// Color `XKCD_PALE_BLUE` from the set CSS4_COLORS. (Color number `1049`)
pub const XKCD_PALE_BLUE: Color = Color(208, 254, 254);
/// Color `XKCD_GRASS_GREEN` from the set CSS4_COLORS. (Color number `1050`)
pub const XKCD_GRASS_GREEN: Color = Color(63, 155, 11);
/// Color `XKCD_NAVY` from the set CSS4_COLORS. (Color number `1051`)
pub const XKCD_NAVY: Color = Color(1, 21, 62);
/// Color `XKCD_AQUAMARINE` from the set CSS4_COLORS. (Color number `1052`)
pub const XKCD_AQUAMARINE: Color = Color(4, 216, 178);
/// Color `XKCD_BURNT_ORANGE` from the set CSS4_COLORS. (Color number `1053`)
pub const XKCD_BURNT_ORANGE: Color = Color(192, 78, 1);
/// Color `XKCD_NEON_GREEN` from the set CSS4_COLORS. (Color number `1054`)
pub const XKCD_NEON_GREEN: Color = Color(12, 255, 12);
/// Color `XKCD_BRIGHT_BLUE` from the set CSS4_COLORS. (Color number `1055`)
pub const XKCD_BRIGHT_BLUE: Color = Color(1, 101, 252);
/// Color `XKCD_ROSE` from the set CSS4_COLORS. (Color number `1056`)
pub const XKCD_ROSE: Color = Color(207, 98, 117);
/// Color `XKCD_LIGHT_PINK` from the set CSS4_COLORS. (Color number `1057`)
pub const XKCD_LIGHT_PINK: Color = Color(255, 209, 223);
/// Color `XKCD_MUSTARD` from the set CSS4_COLORS. (Color number `1058`)
pub const XKCD_MUSTARD: Color = Color(206, 179, 1);
/// Color `XKCD_INDIGO` from the set CSS4_COLORS. (Color number `1059`)
pub const XKCD_INDIGO: Color = Color(56, 2, 130);
/// Color `XKCD_LIME` from the set CSS4_COLORS. (Color number `1060`)
pub const XKCD_LIME: Color = Color(170, 255, 50);
/// Color `XKCD_SEA_GREEN` from the set CSS4_COLORS. (Color number `1061`)
pub const XKCD_SEA_GREEN: Color = Color(83, 252, 161);
/// Color `XKCD_PERIWINKLE` from the set CSS4_COLORS. (Color number `1062`)
pub const XKCD_PERIWINKLE: Color = Color(142, 130, 254);
/// Color `XKCD_DARK_PINK` from the set CSS4_COLORS. (Color number `1063`)
pub const XKCD_DARK_PINK: Color = Color(203, 65, 107);
/// Color `XKCD_OLIVE_GREEN` from the set CSS4_COLORS. (Color number `1064`)
pub const XKCD_OLIVE_GREEN: Color = Color(103, 122, 4);
/// Color `XKCD_PEACH` from the set CSS4_COLORS. (Color number `1065`)
pub const XKCD_PEACH: Color = Color(255, 176, 124);
/// Color `XKCD_PALE_GREEN` from the set CSS4_COLORS. (Color number `1066`)
pub const XKCD_PALE_GREEN: Color = Color(199, 253, 181);
/// Color `XKCD_LIGHT_BROWN` from the set CSS4_COLORS. (Color number `1067`)
pub const XKCD_LIGHT_BROWN: Color = Color(173, 129, 80);
/// Color `XKCD_HOT_PINK` from the set CSS4_COLORS. (Color number `1068`)
pub const XKCD_HOT_PINK: Color = Color(255, 2, 141);
/// Color `XKCD_BLACK` from the set CSS4_COLORS. (Color number `1069`)
pub const XKCD_BLACK: Color = Color(0, 0, 0);
/// Color `XKCD_LILAC` from the set CSS4_COLORS. (Color number `1070`)
pub const XKCD_LILAC: Color = Color(206, 162, 253);
/// Color `XKCD_NAVY_BLUE` from the set CSS4_COLORS. (Color number `1071`)
pub const XKCD_NAVY_BLUE: Color = Color(0, 17, 70);
/// Color `XKCD_ROYAL_BLUE` from the set CSS4_COLORS. (Color number `1072`)
pub const XKCD_ROYAL_BLUE: Color = Color(5, 4, 170);
/// Color `XKCD_BEIGE` from the set CSS4_COLORS. (Color number `1073`)
pub const XKCD_BEIGE: Color = Color(230, 218, 166);
/// Color `XKCD_SALMON` from the set CSS4_COLORS. (Color number `1074`)
pub const XKCD_SALMON: Color = Color(255, 121, 108);
/// Color `XKCD_OLIVE` from the set CSS4_COLORS. (Color number `1075`)
pub const XKCD_OLIVE: Color = Color(110, 117, 14);
/// Color `XKCD_MAROON` from the set CSS4_COLORS. (Color number `1076`)
pub const XKCD_MAROON: Color = Color(101, 0, 33);
/// Color `XKCD_BRIGHT_GREEN` from the set CSS4_COLORS. (Color number `1077`)
pub const XKCD_BRIGHT_GREEN: Color = Color(1, 255, 7);
/// Color `XKCD_DARK_PURPLE` from the set CSS4_COLORS. (Color number `1078`)
pub const XKCD_DARK_PURPLE: Color = Color(53, 6, 62);
/// Color `XKCD_MAUVE` from the set CSS4_COLORS. (Color number `1079`)
pub const XKCD_MAUVE: Color = Color(174, 113, 129);
/// Color `XKCD_FOREST_GREEN` from the set CSS4_COLORS. (Color number `1080`)
pub const XKCD_FOREST_GREEN: Color = Color(6, 71, 12);
/// Color `XKCD_AQUA` from the set CSS4_COLORS. (Color number `1081`)
pub const XKCD_AQUA: Color = Color(19, 234, 201);
/// Color `XKCD_CYAN` from the set CSS4_COLORS. (Color number `1082`)
pub const XKCD_CYAN: Color = Color(0, 255, 255);
/// Color `XKCD_TAN` from the set CSS4_COLORS. (Color number `1083`)
pub const XKCD_TAN: Color = Color(209, 178, 111);
/// Color `XKCD_DARK_BLUE` from the set CSS4_COLORS. (Color number `1084`)
pub const XKCD_DARK_BLUE: Color = Color(0, 3, 91);
/// Color `XKCD_LAVENDER` from the set CSS4_COLORS. (Color number `1085`)
pub const XKCD_LAVENDER: Color = Color(199, 159, 239);
/// Color `XKCD_TURQUOISE` from the set CSS4_COLORS. (Color number `1086`)
pub const XKCD_TURQUOISE: Color = Color(6, 194, 172);
/// Color `XKCD_DARK_GREEN` from the set CSS4_COLORS. (Color number `1087`)
pub const XKCD_DARK_GREEN: Color = Color(3, 53, 0);
/// Color `XKCD_VIOLET` from the set CSS4_COLORS. (Color number `1088`)
pub const XKCD_VIOLET: Color = Color(154, 14, 234);
/// Color `XKCD_LIGHT_PURPLE` from the set CSS4_COLORS. (Color number `1089`)
pub const XKCD_LIGHT_PURPLE: Color = Color(191, 119, 246);
/// Color `XKCD_LIME_GREEN` from the set CSS4_COLORS. (Color number `1090`)
pub const XKCD_LIME_GREEN: Color = Color(137, 254, 5);
/// Color `XKCD_GREY` from the set CSS4_COLORS. (Color number `1091`)
pub const XKCD_GREY: Color = Color(146, 149, 145);
/// Color `XKCD_SKY_BLUE` from the set CSS4_COLORS. (Color number `1092`)
pub const XKCD_SKY_BLUE: Color = Color(117, 187, 253);
/// Color `XKCD_YELLOW` from the set CSS4_COLORS. (Color number `1093`)
pub const XKCD_YELLOW: Color = Color(255, 255, 20);
/// Color `XKCD_MAGENTA` from the set CSS4_COLORS. (Color number `1094`)
pub const XKCD_MAGENTA: Color = Color(194, 0, 120);
/// Color `XKCD_LIGHT_GREEN` from the set CSS4_COLORS. (Color number `1095`)
pub const XKCD_LIGHT_GREEN: Color = Color(150, 249, 123);
/// Color `XKCD_ORANGE` from the set CSS4_COLORS. (Color number `1096`)
pub const XKCD_ORANGE: Color = Color(249, 115, 6);
/// Color `XKCD_TEAL` from the set CSS4_COLORS. (Color number `1097`)
pub const XKCD_TEAL: Color = Color(2, 147, 134);
/// Color `XKCD_LIGHT_BLUE` from the set CSS4_COLORS. (Color number `1098`)
pub const XKCD_LIGHT_BLUE: Color = Color(149, 208, 252);
/// Color `XKCD_RED` from the set CSS4_COLORS. (Color number `1099`)
pub const XKCD_RED: Color = Color(229, 0, 0);
/// Color `XKCD_BROWN` from the set CSS4_COLORS. (Color number `1100`)
pub const XKCD_BROWN: Color = Color(101, 55, 0);
/// Color `XKCD_PINK` from the set CSS4_COLORS. (Color number `1101`)
pub const XKCD_PINK: Color = Color(255, 129, 192);
/// Color `XKCD_BLUE` from the set CSS4_COLORS. (Color number `1102`)
pub const XKCD_BLUE: Color = Color(3, 67, 223);
/// Color `XKCD_GREEN` from the set CSS4_COLORS. (Color number `1103`)
pub const XKCD_GREEN: Color = Color(21, 176, 26);
/// Color `XKCD_PURPLE` from the set CSS4_COLORS. (Color number `1104`)
pub const XKCD_PURPLE: Color = Color(126, 30, 156);
