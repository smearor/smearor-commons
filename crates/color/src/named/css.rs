use crate::Color;
use crate::HexColor;
use crate::ParseColorError;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

/// A CSS `<named-color>` keyword from the W3C
/// [CSS Color Module Level 4](https://www.w3.org/TR/css-color-4/#named-colors)
/// spec (the former "CSS3 color names").
///
/// The enum covers all 148 keywords - including the Level 4 addition
/// `rebeccapurple` and the `gray`/`grey` spelling variants - plus the
/// `transparent` keyword. `currentcolor` is context-dependent and
/// intentionally not represented.
///
/// Keywords parse ASCII case-insensitively via [`FromStr`], like in CSS;
/// [`Display`] and the wire format use the canonical lowercase keyword.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(rename_all = "lowercase"))]
pub enum CssNamedColor {
    /// `aliceblue` — `#F0F8FF`.
    AliceBlue,
    /// `antiquewhite` — `#FAEBD7`.
    AntiqueWhite,
    /// `aqua` — `#00FFFF`.
    Aqua,
    /// `aquamarine` — `#7FFFD4`.
    Aquamarine,
    /// `azure` — `#F0FFFF`.
    Azure,
    /// `beige` — `#F5F5DC`.
    Beige,
    /// `bisque` — `#FFE4C4`.
    Bisque,
    /// `black` — `#000000`.
    Black,
    /// `blanchedalmond` — `#FFEBCD`.
    BlanchedAlmond,
    /// `blue` — `#0000FF`.
    Blue,
    /// `blueviolet` — `#8A2BE2`.
    BlueViolet,
    /// `brown` — `#A52A2A`.
    Brown,
    /// `burlywood` — `#DEB887`.
    BurlyWood,
    /// `cadetblue` — `#5F9EA0`.
    CadetBlue,
    /// `chartreuse` — `#7FFF00`.
    Chartreuse,
    /// `chocolate` — `#D2691E`.
    Chocolate,
    /// `coral` — `#FF7F50`.
    Coral,
    /// `cornflowerblue` — `#6495ED`.
    CornflowerBlue,
    /// `cornsilk` — `#FFF8DC`.
    Cornsilk,
    /// `crimson` — `#DC143C`.
    Crimson,
    /// `cyan` — `#00FFFF`.
    Cyan,
    /// `darkblue` — `#00008B`.
    DarkBlue,
    /// `darkcyan` — `#008B8B`.
    DarkCyan,
    /// `darkgoldenrod` — `#B8860B`.
    DarkGoldenrod,
    /// `darkgray` — `#A9A9A9`.
    DarkGray,
    /// `darkgreen` — `#006400`.
    DarkGreen,
    /// `darkgrey` — `#A9A9A9`.
    DarkGrey,
    /// `darkkhaki` — `#BDB76B`.
    DarkKhaki,
    /// `darkmagenta` — `#8B008B`.
    DarkMagenta,
    /// `darkolivegreen` — `#556B2F`.
    DarkOliveGreen,
    /// `darkorange` — `#FF8C00`.
    DarkOrange,
    /// `darkorchid` — `#9932CC`.
    DarkOrchid,
    /// `darkred` — `#8B0000`.
    DarkRed,
    /// `darksalmon` — `#E9967A`.
    DarkSalmon,
    /// `darkseagreen` — `#8FBC8F`.
    DarkSeaGreen,
    /// `darkslateblue` — `#483D8B`.
    DarkSlateBlue,
    /// `darkslategray` — `#2F4F4F`.
    DarkSlateGray,
    /// `darkslategrey` — `#2F4F4F`.
    DarkSlateGrey,
    /// `darkturquoise` — `#00CED1`.
    DarkTurquoise,
    /// `darkviolet` — `#9400D3`.
    DarkViolet,
    /// `deeppink` — `#FF1493`.
    DeepPink,
    /// `deepskyblue` — `#00BFFF`.
    DeepSkyBlue,
    /// `dimgray` — `#696969`.
    DimGray,
    /// `dimgrey` — `#696969`.
    DimGrey,
    /// `dodgerblue` — `#1E90FF`.
    DodgerBlue,
    /// `firebrick` — `#B22222`.
    Firebrick,
    /// `floralwhite` — `#FFFAF0`.
    FloralWhite,
    /// `forestgreen` — `#228B22`.
    ForestGreen,
    /// `fuchsia` — `#FF00FF`.
    Fuchsia,
    /// `gainsboro` — `#DCDCDC`.
    Gainsboro,
    /// `ghostwhite` — `#F8F8FF`.
    GhostWhite,
    /// `gold` — `#FFD700`.
    Gold,
    /// `goldenrod` — `#DAA520`.
    Goldenrod,
    /// `gray` — `#808080`.
    Gray,
    /// `green` — `#008000`.
    Green,
    /// `greenyellow` — `#ADFF2F`.
    GreenYellow,
    /// `grey` — `#808080`.
    Grey,
    /// `honeydew` — `#F0FFF0`.
    Honeydew,
    /// `hotpink` — `#FF69B4`.
    HotPink,
    /// `indianred` — `#CD5C5C`.
    IndianRed,
    /// `indigo` — `#4B0082`.
    Indigo,
    /// `ivory` — `#FFFFF0`.
    Ivory,
    /// `khaki` — `#F0E68C`.
    Khaki,
    /// `lavender` — `#E6E6FA`.
    Lavender,
    /// `lavenderblush` — `#FFF0F5`.
    LavenderBlush,
    /// `lawngreen` — `#7CFC00`.
    LawnGreen,
    /// `lemonchiffon` — `#FFFACD`.
    LemonChiffon,
    /// `lightblue` — `#ADD8E6`.
    LightBlue,
    /// `lightcoral` — `#F08080`.
    LightCoral,
    /// `lightcyan` — `#E0FFFF`.
    LightCyan,
    /// `lightgoldenrodyellow` — `#FAFAD2`.
    LightGoldenrodYellow,
    /// `lightgray` — `#D3D3D3`.
    LightGray,
    /// `lightgreen` — `#90EE90`.
    LightGreen,
    /// `lightgrey` — `#D3D3D3`.
    LightGrey,
    /// `lightpink` — `#FFB6C1`.
    LightPink,
    /// `lightsalmon` — `#FFA07A`.
    LightSalmon,
    /// `lightseagreen` — `#20B2AA`.
    LightSeaGreen,
    /// `lightskyblue` — `#87CEFA`.
    LightSkyBlue,
    /// `lightslategray` — `#778899`.
    LightSlateGray,
    /// `lightslategrey` — `#778899`.
    LightSlateGrey,
    /// `lightsteelblue` — `#B0C4DE`.
    LightSteelBlue,
    /// `lightyellow` — `#FFFFE0`.
    LightYellow,
    /// `lime` — `#00FF00`.
    Lime,
    /// `limegreen` — `#32CD32`.
    LimeGreen,
    /// `linen` — `#FAF0E6`.
    Linen,
    /// `magenta` — `#FF00FF`.
    Magenta,
    /// `maroon` — `#800000`.
    Maroon,
    /// `mediumaquamarine` — `#66CDAA`.
    MediumAquamarine,
    /// `mediumblue` — `#0000CD`.
    MediumBlue,
    /// `mediumorchid` — `#BA55D3`.
    MediumOrchid,
    /// `mediumpurple` — `#9370DB`.
    MediumPurple,
    /// `mediumseagreen` — `#3CB371`.
    MediumSeaGreen,
    /// `mediumslateblue` — `#7B68EE`.
    MediumSlateBlue,
    /// `mediumspringgreen` — `#00FA9A`.
    MediumSpringGreen,
    /// `mediumturquoise` — `#48D1CC`.
    MediumTurquoise,
    /// `mediumvioletred` — `#C71585`.
    MediumVioletRed,
    /// `midnightblue` — `#191970`.
    MidnightBlue,
    /// `mintcream` — `#F5FFFA`.
    MintCream,
    /// `mistyrose` — `#FFE4E1`.
    MistyRose,
    /// `moccasin` — `#FFE4B5`.
    Moccasin,
    /// `navajowhite` — `#FFDEAD`.
    NavajoWhite,
    /// `navy` — `#000080`.
    Navy,
    /// `oldlace` — `#FDF5E6`.
    OldLace,
    /// `olive` — `#808000`.
    Olive,
    /// `olivedrab` — `#6B8E23`.
    OliveDrab,
    /// `orange` — `#FFA500`.
    Orange,
    /// `orangered` — `#FF4500`.
    OrangeRed,
    /// `orchid` — `#DA70D6`.
    Orchid,
    /// `palegoldenrod` — `#EEE8AA`.
    PaleGoldenrod,
    /// `palegreen` — `#98FB98`.
    PaleGreen,
    /// `paleturquoise` — `#AFEEEE`.
    PaleTurquoise,
    /// `palevioletred` — `#DB7093`.
    PaleVioletRed,
    /// `papayawhip` — `#FFEFD5`.
    PapayaWhip,
    /// `peachpuff` — `#FFDAB9`.
    PeachPuff,
    /// `peru` — `#CD853F`.
    Peru,
    /// `pink` — `#FFC0CB`.
    Pink,
    /// `plum` — `#DDA0DD`.
    Plum,
    /// `powderblue` — `#B0E0E6`.
    PowderBlue,
    /// `purple` — `#800080`.
    Purple,
    /// `rebeccapurple` — `#663399`.
    RebeccaPurple,
    /// `red` — `#FF0000`.
    Red,
    /// `rosybrown` — `#BC8F8F`.
    RosyBrown,
    /// `royalblue` — `#4169E1`.
    RoyalBlue,
    /// `saddlebrown` — `#8B4513`.
    SaddleBrown,
    /// `salmon` — `#FA8072`.
    Salmon,
    /// `sandybrown` — `#F4A460`.
    SandyBrown,
    /// `seagreen` — `#2E8B57`.
    SeaGreen,
    /// `seashell` — `#FFF5EE`.
    Seashell,
    /// `sienna` — `#A0522D`.
    Sienna,
    /// `silver` — `#C0C0C0`.
    Silver,
    /// `skyblue` — `#87CEEB`.
    SkyBlue,
    /// `slateblue` — `#6A5ACD`.
    SlateBlue,
    /// `slategray` — `#708090`.
    SlateGray,
    /// `slategrey` — `#708090`.
    SlateGrey,
    /// `snow` — `#FFFAFA`.
    Snow,
    /// `springgreen` — `#00FF7F`.
    SpringGreen,
    /// `steelblue` — `#4682B4`.
    SteelBlue,
    /// `tan` — `#D2B48C`.
    Tan,
    /// `teal` — `#008080`.
    Teal,
    /// `thistle` — `#D8BFD8`.
    Thistle,
    /// `tomato` — `#FF6347`.
    Tomato,
    /// `transparent` — `rgba(0, 0, 0, 0)`.
    Transparent,
    /// `turquoise` — `#40E0D0`.
    Turquoise,
    /// `violet` — `#EE82EE`.
    Violet,
    /// `wheat` — `#F5DEB3`.
    Wheat,
    /// `white` — `#FFFFFF`.
    White,
    /// `whitesmoke` — `#F5F5F5`.
    WhiteSmoke,
    /// `yellow` — `#FFFF00`.
    Yellow,
    /// `yellowgreen` — `#9ACD32`.
    YellowGreen,
}

impl CssNamedColor {
    /// All named colors, sorted alphabetically by keyword.
    pub const ALL: &'static [Self] = &[
        Self::AliceBlue,
        Self::AntiqueWhite,
        Self::Aqua,
        Self::Aquamarine,
        Self::Azure,
        Self::Beige,
        Self::Bisque,
        Self::Black,
        Self::BlanchedAlmond,
        Self::Blue,
        Self::BlueViolet,
        Self::Brown,
        Self::BurlyWood,
        Self::CadetBlue,
        Self::Chartreuse,
        Self::Chocolate,
        Self::Coral,
        Self::CornflowerBlue,
        Self::Cornsilk,
        Self::Crimson,
        Self::Cyan,
        Self::DarkBlue,
        Self::DarkCyan,
        Self::DarkGoldenrod,
        Self::DarkGray,
        Self::DarkGreen,
        Self::DarkGrey,
        Self::DarkKhaki,
        Self::DarkMagenta,
        Self::DarkOliveGreen,
        Self::DarkOrange,
        Self::DarkOrchid,
        Self::DarkRed,
        Self::DarkSalmon,
        Self::DarkSeaGreen,
        Self::DarkSlateBlue,
        Self::DarkSlateGray,
        Self::DarkSlateGrey,
        Self::DarkTurquoise,
        Self::DarkViolet,
        Self::DeepPink,
        Self::DeepSkyBlue,
        Self::DimGray,
        Self::DimGrey,
        Self::DodgerBlue,
        Self::Firebrick,
        Self::FloralWhite,
        Self::ForestGreen,
        Self::Fuchsia,
        Self::Gainsboro,
        Self::GhostWhite,
        Self::Gold,
        Self::Goldenrod,
        Self::Gray,
        Self::Green,
        Self::GreenYellow,
        Self::Grey,
        Self::Honeydew,
        Self::HotPink,
        Self::IndianRed,
        Self::Indigo,
        Self::Ivory,
        Self::Khaki,
        Self::Lavender,
        Self::LavenderBlush,
        Self::LawnGreen,
        Self::LemonChiffon,
        Self::LightBlue,
        Self::LightCoral,
        Self::LightCyan,
        Self::LightGoldenrodYellow,
        Self::LightGray,
        Self::LightGreen,
        Self::LightGrey,
        Self::LightPink,
        Self::LightSalmon,
        Self::LightSeaGreen,
        Self::LightSkyBlue,
        Self::LightSlateGray,
        Self::LightSlateGrey,
        Self::LightSteelBlue,
        Self::LightYellow,
        Self::Lime,
        Self::LimeGreen,
        Self::Linen,
        Self::Magenta,
        Self::Maroon,
        Self::MediumAquamarine,
        Self::MediumBlue,
        Self::MediumOrchid,
        Self::MediumPurple,
        Self::MediumSeaGreen,
        Self::MediumSlateBlue,
        Self::MediumSpringGreen,
        Self::MediumTurquoise,
        Self::MediumVioletRed,
        Self::MidnightBlue,
        Self::MintCream,
        Self::MistyRose,
        Self::Moccasin,
        Self::NavajoWhite,
        Self::Navy,
        Self::OldLace,
        Self::Olive,
        Self::OliveDrab,
        Self::Orange,
        Self::OrangeRed,
        Self::Orchid,
        Self::PaleGoldenrod,
        Self::PaleGreen,
        Self::PaleTurquoise,
        Self::PaleVioletRed,
        Self::PapayaWhip,
        Self::PeachPuff,
        Self::Peru,
        Self::Pink,
        Self::Plum,
        Self::PowderBlue,
        Self::Purple,
        Self::RebeccaPurple,
        Self::Red,
        Self::RosyBrown,
        Self::RoyalBlue,
        Self::SaddleBrown,
        Self::Salmon,
        Self::SandyBrown,
        Self::SeaGreen,
        Self::Seashell,
        Self::Sienna,
        Self::Silver,
        Self::SkyBlue,
        Self::SlateBlue,
        Self::SlateGray,
        Self::SlateGrey,
        Self::Snow,
        Self::SpringGreen,
        Self::SteelBlue,
        Self::Tan,
        Self::Teal,
        Self::Thistle,
        Self::Tomato,
        Self::Transparent,
        Self::Turquoise,
        Self::Violet,
        Self::Wheat,
        Self::White,
        Self::WhiteSmoke,
        Self::Yellow,
        Self::YellowGreen,
    ];

    /// The canonical lowercase CSS keyword, e.g. `"rebeccapurple"`.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::AliceBlue => "aliceblue",
            Self::AntiqueWhite => "antiquewhite",
            Self::Aqua => "aqua",
            Self::Aquamarine => "aquamarine",
            Self::Azure => "azure",
            Self::Beige => "beige",
            Self::Bisque => "bisque",
            Self::Black => "black",
            Self::BlanchedAlmond => "blanchedalmond",
            Self::Blue => "blue",
            Self::BlueViolet => "blueviolet",
            Self::Brown => "brown",
            Self::BurlyWood => "burlywood",
            Self::CadetBlue => "cadetblue",
            Self::Chartreuse => "chartreuse",
            Self::Chocolate => "chocolate",
            Self::Coral => "coral",
            Self::CornflowerBlue => "cornflowerblue",
            Self::Cornsilk => "cornsilk",
            Self::Crimson => "crimson",
            Self::Cyan => "cyan",
            Self::DarkBlue => "darkblue",
            Self::DarkCyan => "darkcyan",
            Self::DarkGoldenrod => "darkgoldenrod",
            Self::DarkGray => "darkgray",
            Self::DarkGreen => "darkgreen",
            Self::DarkGrey => "darkgrey",
            Self::DarkKhaki => "darkkhaki",
            Self::DarkMagenta => "darkmagenta",
            Self::DarkOliveGreen => "darkolivegreen",
            Self::DarkOrange => "darkorange",
            Self::DarkOrchid => "darkorchid",
            Self::DarkRed => "darkred",
            Self::DarkSalmon => "darksalmon",
            Self::DarkSeaGreen => "darkseagreen",
            Self::DarkSlateBlue => "darkslateblue",
            Self::DarkSlateGray => "darkslategray",
            Self::DarkSlateGrey => "darkslategrey",
            Self::DarkTurquoise => "darkturquoise",
            Self::DarkViolet => "darkviolet",
            Self::DeepPink => "deeppink",
            Self::DeepSkyBlue => "deepskyblue",
            Self::DimGray => "dimgray",
            Self::DimGrey => "dimgrey",
            Self::DodgerBlue => "dodgerblue",
            Self::Firebrick => "firebrick",
            Self::FloralWhite => "floralwhite",
            Self::ForestGreen => "forestgreen",
            Self::Fuchsia => "fuchsia",
            Self::Gainsboro => "gainsboro",
            Self::GhostWhite => "ghostwhite",
            Self::Gold => "gold",
            Self::Goldenrod => "goldenrod",
            Self::Gray => "gray",
            Self::Green => "green",
            Self::GreenYellow => "greenyellow",
            Self::Grey => "grey",
            Self::Honeydew => "honeydew",
            Self::HotPink => "hotpink",
            Self::IndianRed => "indianred",
            Self::Indigo => "indigo",
            Self::Ivory => "ivory",
            Self::Khaki => "khaki",
            Self::Lavender => "lavender",
            Self::LavenderBlush => "lavenderblush",
            Self::LawnGreen => "lawngreen",
            Self::LemonChiffon => "lemonchiffon",
            Self::LightBlue => "lightblue",
            Self::LightCoral => "lightcoral",
            Self::LightCyan => "lightcyan",
            Self::LightGoldenrodYellow => "lightgoldenrodyellow",
            Self::LightGray => "lightgray",
            Self::LightGreen => "lightgreen",
            Self::LightGrey => "lightgrey",
            Self::LightPink => "lightpink",
            Self::LightSalmon => "lightsalmon",
            Self::LightSeaGreen => "lightseagreen",
            Self::LightSkyBlue => "lightskyblue",
            Self::LightSlateGray => "lightslategray",
            Self::LightSlateGrey => "lightslategrey",
            Self::LightSteelBlue => "lightsteelblue",
            Self::LightYellow => "lightyellow",
            Self::Lime => "lime",
            Self::LimeGreen => "limegreen",
            Self::Linen => "linen",
            Self::Magenta => "magenta",
            Self::Maroon => "maroon",
            Self::MediumAquamarine => "mediumaquamarine",
            Self::MediumBlue => "mediumblue",
            Self::MediumOrchid => "mediumorchid",
            Self::MediumPurple => "mediumpurple",
            Self::MediumSeaGreen => "mediumseagreen",
            Self::MediumSlateBlue => "mediumslateblue",
            Self::MediumSpringGreen => "mediumspringgreen",
            Self::MediumTurquoise => "mediumturquoise",
            Self::MediumVioletRed => "mediumvioletred",
            Self::MidnightBlue => "midnightblue",
            Self::MintCream => "mintcream",
            Self::MistyRose => "mistyrose",
            Self::Moccasin => "moccasin",
            Self::NavajoWhite => "navajowhite",
            Self::Navy => "navy",
            Self::OldLace => "oldlace",
            Self::Olive => "olive",
            Self::OliveDrab => "olivedrab",
            Self::Orange => "orange",
            Self::OrangeRed => "orangered",
            Self::Orchid => "orchid",
            Self::PaleGoldenrod => "palegoldenrod",
            Self::PaleGreen => "palegreen",
            Self::PaleTurquoise => "paleturquoise",
            Self::PaleVioletRed => "palevioletred",
            Self::PapayaWhip => "papayawhip",
            Self::PeachPuff => "peachpuff",
            Self::Peru => "peru",
            Self::Pink => "pink",
            Self::Plum => "plum",
            Self::PowderBlue => "powderblue",
            Self::Purple => "purple",
            Self::RebeccaPurple => "rebeccapurple",
            Self::Red => "red",
            Self::RosyBrown => "rosybrown",
            Self::RoyalBlue => "royalblue",
            Self::SaddleBrown => "saddlebrown",
            Self::Salmon => "salmon",
            Self::SandyBrown => "sandybrown",
            Self::SeaGreen => "seagreen",
            Self::Seashell => "seashell",
            Self::Sienna => "sienna",
            Self::Silver => "silver",
            Self::SkyBlue => "skyblue",
            Self::SlateBlue => "slateblue",
            Self::SlateGray => "slategray",
            Self::SlateGrey => "slategrey",
            Self::Snow => "snow",
            Self::SpringGreen => "springgreen",
            Self::SteelBlue => "steelblue",
            Self::Tan => "tan",
            Self::Teal => "teal",
            Self::Thistle => "thistle",
            Self::Tomato => "tomato",
            Self::Transparent => "transparent",
            Self::Turquoise => "turquoise",
            Self::Violet => "violet",
            Self::Wheat => "wheat",
            Self::White => "white",
            Self::WhiteSmoke => "whitesmoke",
            Self::Yellow => "yellow",
            Self::YellowGreen => "yellowgreen",
        }
    }

    /// The color as byte components - [`Self::Transparent`] maps to
    /// `rgba(0, 0, 0, 0)`.
    pub const fn to_color(&self) -> Color<u8> {
        match self {
            Self::Transparent => Color::<u8>::new_rgba(0, 0, 0, 0),
            _ => Color::from_rgb_u32(self.packed_rgb()),
        }
    }

    /// The packed `0xRRGGBB` value of the keyword.
    const fn packed_rgb(&self) -> u32 {
        match self {
            Self::AliceBlue => 0xF0F8FF,
            Self::AntiqueWhite => 0xFAEBD7,
            Self::Aqua => 0x00FFFF,
            Self::Aquamarine => 0x7FFFD4,
            Self::Azure => 0xF0FFFF,
            Self::Beige => 0xF5F5DC,
            Self::Bisque => 0xFFE4C4,
            Self::Black => 0x000000,
            Self::BlanchedAlmond => 0xFFEBCD,
            Self::Blue => 0x0000FF,
            Self::BlueViolet => 0x8A2BE2,
            Self::Brown => 0xA52A2A,
            Self::BurlyWood => 0xDEB887,
            Self::CadetBlue => 0x5F9EA0,
            Self::Chartreuse => 0x7FFF00,
            Self::Chocolate => 0xD2691E,
            Self::Coral => 0xFF7F50,
            Self::CornflowerBlue => 0x6495ED,
            Self::Cornsilk => 0xFFF8DC,
            Self::Crimson => 0xDC143C,
            Self::Cyan => 0x00FFFF,
            Self::DarkBlue => 0x00008B,
            Self::DarkCyan => 0x008B8B,
            Self::DarkGoldenrod => 0xB8860B,
            Self::DarkGray => 0xA9A9A9,
            Self::DarkGreen => 0x006400,
            Self::DarkGrey => 0xA9A9A9,
            Self::DarkKhaki => 0xBDB76B,
            Self::DarkMagenta => 0x8B008B,
            Self::DarkOliveGreen => 0x556B2F,
            Self::DarkOrange => 0xFF8C00,
            Self::DarkOrchid => 0x9932CC,
            Self::DarkRed => 0x8B0000,
            Self::DarkSalmon => 0xE9967A,
            Self::DarkSeaGreen => 0x8FBC8F,
            Self::DarkSlateBlue => 0x483D8B,
            Self::DarkSlateGray => 0x2F4F4F,
            Self::DarkSlateGrey => 0x2F4F4F,
            Self::DarkTurquoise => 0x00CED1,
            Self::DarkViolet => 0x9400D3,
            Self::DeepPink => 0xFF1493,
            Self::DeepSkyBlue => 0x00BFFF,
            Self::DimGray => 0x696969,
            Self::DimGrey => 0x696969,
            Self::DodgerBlue => 0x1E90FF,
            Self::Firebrick => 0xB22222,
            Self::FloralWhite => 0xFFFAF0,
            Self::ForestGreen => 0x228B22,
            Self::Fuchsia => 0xFF00FF,
            Self::Gainsboro => 0xDCDCDC,
            Self::GhostWhite => 0xF8F8FF,
            Self::Gold => 0xFFD700,
            Self::Goldenrod => 0xDAA520,
            Self::Gray => 0x808080,
            Self::Green => 0x008000,
            Self::GreenYellow => 0xADFF2F,
            Self::Grey => 0x808080,
            Self::Honeydew => 0xF0FFF0,
            Self::HotPink => 0xFF69B4,
            Self::IndianRed => 0xCD5C5C,
            Self::Indigo => 0x4B0082,
            Self::Ivory => 0xFFFFF0,
            Self::Khaki => 0xF0E68C,
            Self::Lavender => 0xE6E6FA,
            Self::LavenderBlush => 0xFFF0F5,
            Self::LawnGreen => 0x7CFC00,
            Self::LemonChiffon => 0xFFFACD,
            Self::LightBlue => 0xADD8E6,
            Self::LightCoral => 0xF08080,
            Self::LightCyan => 0xE0FFFF,
            Self::LightGoldenrodYellow => 0xFAFAD2,
            Self::LightGray => 0xD3D3D3,
            Self::LightGreen => 0x90EE90,
            Self::LightGrey => 0xD3D3D3,
            Self::LightPink => 0xFFB6C1,
            Self::LightSalmon => 0xFFA07A,
            Self::LightSeaGreen => 0x20B2AA,
            Self::LightSkyBlue => 0x87CEFA,
            Self::LightSlateGray => 0x778899,
            Self::LightSlateGrey => 0x778899,
            Self::LightSteelBlue => 0xB0C4DE,
            Self::LightYellow => 0xFFFFE0,
            Self::Lime => 0x00FF00,
            Self::LimeGreen => 0x32CD32,
            Self::Linen => 0xFAF0E6,
            Self::Magenta => 0xFF00FF,
            Self::Maroon => 0x800000,
            Self::MediumAquamarine => 0x66CDAA,
            Self::MediumBlue => 0x0000CD,
            Self::MediumOrchid => 0xBA55D3,
            Self::MediumPurple => 0x9370DB,
            Self::MediumSeaGreen => 0x3CB371,
            Self::MediumSlateBlue => 0x7B68EE,
            Self::MediumSpringGreen => 0x00FA9A,
            Self::MediumTurquoise => 0x48D1CC,
            Self::MediumVioletRed => 0xC71585,
            Self::MidnightBlue => 0x191970,
            Self::MintCream => 0xF5FFFA,
            Self::MistyRose => 0xFFE4E1,
            Self::Moccasin => 0xFFE4B5,
            Self::NavajoWhite => 0xFFDEAD,
            Self::Navy => 0x000080,
            Self::OldLace => 0xFDF5E6,
            Self::Olive => 0x808000,
            Self::OliveDrab => 0x6B8E23,
            Self::Orange => 0xFFA500,
            Self::OrangeRed => 0xFF4500,
            Self::Orchid => 0xDA70D6,
            Self::PaleGoldenrod => 0xEEE8AA,
            Self::PaleGreen => 0x98FB98,
            Self::PaleTurquoise => 0xAFEEEE,
            Self::PaleVioletRed => 0xDB7093,
            Self::PapayaWhip => 0xFFEFD5,
            Self::PeachPuff => 0xFFDAB9,
            Self::Peru => 0xCD853F,
            Self::Pink => 0xFFC0CB,
            Self::Plum => 0xDDA0DD,
            Self::PowderBlue => 0xB0E0E6,
            Self::Purple => 0x800080,
            Self::RebeccaPurple => 0x663399,
            Self::Red => 0xFF0000,
            Self::RosyBrown => 0xBC8F8F,
            Self::RoyalBlue => 0x4169E1,
            Self::SaddleBrown => 0x8B4513,
            Self::Salmon => 0xFA8072,
            Self::SandyBrown => 0xF4A460,
            Self::SeaGreen => 0x2E8B57,
            Self::Seashell => 0xFFF5EE,
            Self::Sienna => 0xA0522D,
            Self::Silver => 0xC0C0C0,
            Self::SkyBlue => 0x87CEEB,
            Self::SlateBlue => 0x6A5ACD,
            Self::SlateGray => 0x708090,
            Self::SlateGrey => 0x708090,
            Self::Snow => 0xFFFAFA,
            Self::SpringGreen => 0x00FF7F,
            Self::SteelBlue => 0x4682B4,
            Self::Tan => 0xD2B48C,
            Self::Teal => 0x008080,
            Self::Thistle => 0xD8BFD8,
            Self::Tomato => 0xFF6347,
            Self::Transparent => 0x000000,
            Self::Turquoise => 0x40E0D0,
            Self::Violet => 0xEE82EE,
            Self::Wheat => 0xF5DEB3,
            Self::White => 0xFFFFFF,
            Self::WhiteSmoke => 0xF5F5F5,
            Self::Yellow => 0xFFFF00,
            Self::YellowGreen => 0x9ACD32,
        }
    }
}

impl FromStr for CssNamedColor {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .iter()
            .find(|color| s.eq_ignore_ascii_case(color.as_str()))
            .copied()
            .ok_or_else(|| ParseColorError::UnknownNamedColor { input: s.into() })
    }
}

impl Display for CssNamedColor {
    /// Formats the canonical lowercase CSS keyword.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<CssNamedColor> for Color<u8> {
    fn from(color: CssNamedColor) -> Self {
        color.to_color()
    }
}

impl From<CssNamedColor> for Color<f64> {
    fn from(color: CssNamedColor) -> Self {
        color.to_color().into()
    }
}

impl From<CssNamedColor> for Color<f32> {
    fn from(color: CssNamedColor) -> Self {
        color.to_color().into()
    }
}

impl From<CssNamedColor> for HexColor {
    fn from(color: CssNamedColor) -> Self {
        Self::from(color.to_color())
    }
}

/// The wire format is the lowercase CSS keyword.
impl Serialize for CssNamedColor {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// Deserialization accepts the keyword ASCII case-insensitively.
impl<'de> Deserialize<'de> for CssNamedColor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "zbus")]
impl zvariant::Type for CssNamedColor {
    const SIGNATURE: &'static zvariant::Signature = <str as zvariant::Type>::SIGNATURE;
}

#[cfg(feature = "async-graphql")]
async_graphql::scalar!(CssNamedColor);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_covers_the_149_keywords() {
        assert_eq!(CssNamedColor::ALL.len(), 149);
    }

    #[test]
    fn all_is_sorted_by_keyword() {
        let keywords: Vec<&str> = CssNamedColor::ALL.iter().map(CssNamedColor::as_str).collect();
        let mut sorted = keywords.clone();
        sorted.sort();
        assert_eq!(keywords, sorted);
    }

    #[test]
    fn from_str_is_ascii_case_insensitive() {
        assert_eq!("rebeccapurple".parse::<CssNamedColor>().unwrap(), CssNamedColor::RebeccaPurple);
        assert_eq!("RebeccaPurple".parse::<CssNamedColor>().unwrap(), CssNamedColor::RebeccaPurple);
        assert_eq!("REBECCAPURPLE".parse::<CssNamedColor>().unwrap(), CssNamedColor::RebeccaPurple);
    }

    #[test]
    fn gray_and_grey_spellings_are_distinct_variants_with_equal_color() {
        assert_ne!(CssNamedColor::DarkSlateGray, CssNamedColor::DarkSlateGrey);
        assert_eq!(CssNamedColor::DarkSlateGray.to_color(), CssNamedColor::DarkSlateGrey.to_color());
    }

    #[test]
    fn display_and_as_str_produce_the_lowercase_keyword() {
        assert_eq!(CssNamedColor::RebeccaPurple.to_string(), "rebeccapurple");
        assert_eq!(CssNamedColor::DarkSlateGrey.as_str(), "darkslategrey");
    }

    #[test]
    fn display_from_str_round_trip_covers_all_keywords() {
        for color in CssNamedColor::ALL {
            assert_eq!(color.to_string().parse::<CssNamedColor>().unwrap(), *color);
        }
    }

    #[test]
    fn to_color_maps_keywords_to_srgb() {
        assert_eq!(CssNamedColor::RebeccaPurple.to_color(), Color::<u8>::new(0x66, 0x33, 0x99));
        assert_eq!(CssNamedColor::Transparent.to_color(), Color::<u8>::new_rgba(0, 0, 0, 0));
    }

    #[test]
    fn unknown_names_and_currentcolor_are_rejected() {
        assert!("notacolor".parse::<CssNamedColor>().is_err());
        assert!("currentcolor".parse::<CssNamedColor>().is_err());
        assert!("#ff0000".parse::<CssNamedColor>().is_err());
    }

    #[test]
    fn serde_round_trip() {
        let json = serde_json::to_string(&CssNamedColor::RebeccaPurple).unwrap();
        assert_eq!(json, "\"rebeccapurple\"");
        assert_eq!(serde_json::from_str::<CssNamedColor>("\"DarkKhaki\"").unwrap(), CssNamedColor::DarkKhaki);
    }

    #[cfg(feature = "zbus")]
    #[test]
    fn zvariant_round_trip() {
        assert_eq!(<CssNamedColor as zvariant::Type>::SIGNATURE.to_string(), "s");
        let context = zvariant::serialized::Context::new_dbus(zvariant::LE, 0);
        let bytes = zvariant::to_bytes(context, &CssNamedColor::RebeccaPurple).unwrap();
        let decoded: CssNamedColor = bytes.deserialize().unwrap().0;
        assert_eq!(decoded, CssNamedColor::RebeccaPurple);
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn json_schema_lists_the_lowercase_keywords() {
        let schema = serde_json::to_value(schemars::schema_for!(CssNamedColor)).unwrap();
        let schema = schema.to_string();
        assert!(schema.contains("\"aliceblue\""));
        assert!(schema.contains("\"rebeccapurple\""));
        assert!(schema.contains("\"transparent\""));
    }

    #[cfg(feature = "async-graphql")]
    #[test]
    fn graphql_scalar_round_trip() {
        use async_graphql::ScalarType;
        let parsed = CssNamedColor::parse(CssNamedColor::RebeccaPurple.to_value()).unwrap();
        assert_eq!(parsed, CssNamedColor::RebeccaPurple);
    }
}
