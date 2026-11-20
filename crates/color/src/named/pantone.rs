use crate::Cmyk;
use crate::Color;
use crate::HexColor;
use crate::PantoneSuffix;
use crate::ParseColorError;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

/// A Pantone Matching System (PMS) color name.
///
/// The enum covers 1342 coated (`C`) and 1335 uncoated (`U`) spot
/// colors of the Pantone system - named process colors
/// (`Pantone Process Blue C`), named mixes (`Pantone Cool Gray 1 U`)
/// and the numbered colors (`Pantone 485 C`), whose variants carry a
/// `Pms` prefix since Rust identifiers must not start with a digit.
/// The suffix letter is a [`PantoneSuffix`].
///
/// Names parse case-insensitively via [`FromStr`]; [`Display`] and the
/// wire format use the canonical `Pantone <name> <suffix>` form.
///
/// The stored sRGB values are common approximations of the print
/// colors, e.g. as published on
/// [brandible.de](https://www.brandible.de/pantone-farben) (coated)
/// and in the
/// [Pantone_finder dataset](https://github.com/picorana/Pantone_finder)
/// (uncoated) - Pantone colors are defined for print, so the values
/// are indicative rather than authoritative. The CMYK builds of
/// [`Self::to_cmyk`] follow the
/// [Pantone-CMYK-RGB-Hex dataset](https://github.com/adonald/Pantone-CMYK-RGB-Hex).
/// PANTONE is a trademark of Pantone LLC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum PantoneNamedColor {
    /// `Pantone 100 C` — `#F6EB61`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 100 C"))]
    Pms100C,
    /// `Pantone 100 U` — `#FEF380`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 100 U"))]
    Pms100U,
    /// `Pantone 101 C` — `#F7EA48`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 101 C"))]
    Pms101C,
    /// `Pantone 101 U` — `#FFF166`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 101 U"))]
    Pms101U,
    /// `Pantone 102 C` — `#FCE300`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 102 C"))]
    Pms102C,
    /// `Pantone 102 U` — `#FFEC2D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 102 U"))]
    Pms102U,
    /// `Pantone 103 C` — `#C5A900`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 103 C"))]
    Pms103C,
    /// `Pantone 103 U` — `#B49F29`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 103 U"))]
    Pms103U,
    /// `Pantone 104 C` — `#AF9800`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 104 C"))]
    Pms104C,
    /// `Pantone 104 U` — `#9B8D39`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 104 U"))]
    Pms104U,
    /// `Pantone 105 C` — `#897A27`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 105 C"))]
    Pms105C,
    /// `Pantone 105 U` — `#7D7547`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 105 U"))]
    Pms105U,
    /// `Pantone 106 C` — `#F9E547`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 106 C"))]
    Pms106C,
    /// `Pantone 106 U` — `#FFEB68`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 106 U"))]
    Pms106U,
    /// `Pantone 107 C` — `#FBE122`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 107 C"))]
    Pms107C,
    /// `Pantone 107 U` — `#FFE552`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 107 U"))]
    Pms107U,
    /// `Pantone 108 C` — `#FEDB00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 108 C"))]
    Pms108C,
    /// `Pantone 108 U` — `#FFDD35`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 108 U"))]
    Pms108U,
    /// `Pantone 109 C` — `#FFD100`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 109 C"))]
    Pms109C,
    /// `Pantone 109 U` — `#FFC700`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 109 U"))]
    Pms109U,
    /// `Pantone 110 C` — `#DAAA00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 110 C"))]
    Pms110C,
    /// `Pantone 110 U` — `#CC9F26`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 110 U"))]
    Pms110U,
    /// `Pantone 111 C` — `#AA8A00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 111 C"))]
    Pms111C,
    /// `Pantone 111 U` — `#9C873B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 111 U"))]
    Pms111U,
    /// `Pantone 112 C` — `#9C8412`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 112 C"))]
    Pms112C,
    /// `Pantone 112 U` — `#8A7A3E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 112 U"))]
    Pms112U,
    /// `Pantone 113 C` — `#FAE053`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 113 C"))]
    Pms113C,
    /// `Pantone 113 U` — `#FFE15E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 113 U"))]
    Pms113U,
    /// `Pantone 114 C` — `#FBDD40`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 114 C"))]
    Pms114C,
    /// `Pantone 114 U` — `#FFD94A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 114 U"))]
    Pms114U,
    /// `Pantone 115 C` — `#FDDA24`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 115 C"))]
    Pms115C,
    /// `Pantone 115 U` — `#FFCE2E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 115 U"))]
    Pms115U,
    /// `Pantone 116 C` — `#FFCD00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 116 C"))]
    Pms116C,
    /// `Pantone 116 U` — `#FFB511`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 116 U"))]
    Pms116U,
    /// `Pantone 117 C` — `#C99700`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 117 C"))]
    Pms117C,
    /// `Pantone 117 U` — `#B78C30`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 117 U"))]
    Pms117U,
    /// `Pantone 118 C` — `#AC8400`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 118 C"))]
    Pms118C,
    /// `Pantone 118 U` — `#9E8139`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 118 U"))]
    Pms118U,
    /// `Pantone 119 C` — `#897322`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 119 C"))]
    Pms119C,
    /// `Pantone 119 U` — `#847748`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 119 U"))]
    Pms119U,
    /// `Pantone 120 C` — `#FBDB65`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 120 C"))]
    Pms120C,
    /// `Pantone 120 U` — `#FFD96A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 120 U"))]
    Pms120U,
    /// `Pantone 1205 C` — `#F8E08E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1205 C"))]
    Pms1205C,
    /// `Pantone 1205 U` — `#FFE495`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1205 U"))]
    Pms1205U,
    /// `Pantone 121 C` — `#FDD757`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 121 C"))]
    Pms121C,
    /// `Pantone 121 U` — `#FFCC52`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 121 U"))]
    Pms121U,
    /// `Pantone 1215 C` — `#FBD872`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1215 C"))]
    Pms1215C,
    /// `Pantone 1215 U` — `#FFDA7E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1215 U"))]
    Pms1215U,
    /// `Pantone 122 C` — `#FED141`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 122 C"))]
    Pms122C,
    /// `Pantone 122 U` — `#FFC03F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 122 U"))]
    Pms122U,
    /// `Pantone 1225 C` — `#FFC845`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1225 C"))]
    Pms1225C,
    /// `Pantone 1225 U` — `#FFBB4F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1225 U"))]
    Pms1225U,
    /// `Pantone 123 C` — `#FFC72C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 123 C"))]
    Pms123C,
    /// `Pantone 123 U` — `#FFAC2A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 123 U"))]
    Pms123U,
    /// `Pantone 1235 C` — `#FFB81C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1235 C"))]
    Pms1235C,
    /// `Pantone 1235 U` — `#FFAE3B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1235 U"))]
    Pms1235U,
    /// `Pantone 124 C` — `#EAAA00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 124 C"))]
    Pms124C,
    /// `Pantone 124 U` — `#D79133`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 124 U"))]
    Pms124U,
    /// `Pantone 1245 C` — `#C69214`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1245 C"))]
    Pms1245C,
    /// `Pantone 1245 U` — `#BB8B41`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1245 U"))]
    Pms1245U,
    /// `Pantone 125 C` — `#B58500`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 125 C"))]
    Pms125C,
    /// `Pantone 125 U` — `#A6813D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 125 U"))]
    Pms125U,
    /// `Pantone 1255 C` — `#AD841F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1255 C"))]
    Pms1255C,
    /// `Pantone 1255 U` — `#A08246`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1255 U"))]
    Pms1255U,
    /// `Pantone 126 C` — `#9A7611`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 126 C"))]
    Pms126C,
    /// `Pantone 126 U` — `#8E7742`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 126 U"))]
    Pms126U,
    /// `Pantone 1265 C` — `#886B25`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1265 C"))]
    Pms1265C,
    /// `Pantone 1265 U` — `#887549`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1265 U"))]
    Pms1265U,
    /// `Pantone 127 C` — `#F3DD6D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 127 C"))]
    Pms127C,
    /// `Pantone 127 U` — `#FCE385`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 127 U"))]
    Pms127U,
    /// `Pantone 128 C` — `#F3D54E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 128 C"))]
    Pms128C,
    /// `Pantone 128 U` — `#FCCF61`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 128 U"))]
    Pms128U,
    /// `Pantone 129 C` — `#F3D03E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 129 C"))]
    Pms129C,
    /// `Pantone 129 U` — `#FAAF3F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 129 U"))]
    Pms129U,
    /// `Pantone 130 C` — `#F2A900`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 130 C"))]
    Pms130C,
    /// `Pantone 130 U` — `#F79B2E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 130 U"))]
    Pms130U,
    /// `Pantone 131 C` — `#CC8A00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 131 C"))]
    Pms131C,
    /// `Pantone 131 U` — `#BA8032`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 131 U"))]
    Pms131U,
    /// `Pantone 132 C` — `#A07400`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 132 C"))]
    Pms132C,
    /// `Pantone 132 U` — `#977A3C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 132 U"))]
    Pms132U,
    /// `Pantone 133 C` — `#6C571B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 133 C"))]
    Pms133C,
    /// `Pantone 133 U` — `#7C7047`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 133 U"))]
    Pms133U,
    /// `Pantone 134 C` — `#FDD26E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 134 C"))]
    Pms134C,
    /// `Pantone 134 U` — `#FFD378`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 134 U"))]
    Pms134U,
    /// `Pantone 1345 C` — `#FDD086`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1345 C"))]
    Pms1345C,
    /// `Pantone 1345 U` — `#FFD591`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1345 U"))]
    Pms1345U,
    /// `Pantone 135 C` — `#FFC658`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 135 C"))]
    Pms135C,
    /// `Pantone 135 U` — `#FFC267`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 135 U"))]
    Pms135U,
    /// `Pantone 1355 C` — `#FFC56E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1355 C"))]
    Pms1355C,
    /// `Pantone 1355 U` — `#FFC87E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1355 U"))]
    Pms1355U,
    /// `Pantone 136 C` — `#FFBF3F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 136 C"))]
    Pms136C,
    /// `Pantone 136 U` — `#FFAD4A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 136 U"))]
    Pms136U,
    /// `Pantone 1365 C` — `#FFB549`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1365 C"))]
    Pms1365C,
    /// `Pantone 1365 U` — `#FFAD5B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1365 U"))]
    Pms1365U,
    /// `Pantone 137 C` — `#FFA300`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 137 C"))]
    Pms137C,
    /// `Pantone 137 U` — `#FF9F37`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 137 U"))]
    Pms137U,
    /// `Pantone 1375 C` — `#FF9E1B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1375 C"))]
    Pms1375C,
    /// `Pantone 1375 U` — `#FF9940`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1375 U"))]
    Pms1375U,
    /// `Pantone 138 C` — `#DE7C00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 138 C"))]
    Pms138C,
    /// `Pantone 138 U` — `#D37C32`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 138 U"))]
    Pms138U,
    /// `Pantone 1385 C` — `#D57800`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1385 C"))]
    Pms1385C,
    /// `Pantone 1385 U` — `#CD7E3E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1385 U"))]
    Pms1385U,
    /// `Pantone 139 C` — `#AF6D04`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 139 C"))]
    Pms139C,
    /// `Pantone 139 U` — `#9D6F3C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 139 U"))]
    Pms139U,
    /// `Pantone 1395 C` — `#996017`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1395 C"))]
    Pms1395C,
    /// `Pantone 1395 U` — `#946E43`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1395 U"))]
    Pms1395U,
    /// `Pantone 140 C` — `#74531C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 140 C"))]
    Pms140C,
    /// `Pantone 140 U` — `#7F6A45`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 140 U"))]
    Pms140U,
    /// `Pantone 1405 C` — `#6E4C1E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1405 C"))]
    Pms1405C,
    /// `Pantone 1405 U` — `#796648`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1405 U"))]
    Pms1405U,
    /// `Pantone 141 C` — `#F2C75C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 141 C"))]
    Pms141C,
    /// `Pantone 141 U` — `#F9C16E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 141 U"))]
    Pms141U,
    /// `Pantone 142 C` — `#F1BE48`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 142 C"))]
    Pms142C,
    /// `Pantone 142 U` — `#F8B05D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 142 U"))]
    Pms142U,
    /// `Pantone 143 C` — `#F1B434`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 143 C"))]
    Pms143C,
    /// `Pantone 143 U` — `#F6A04D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 143 U"))]
    Pms143U,
    /// `Pantone 144 C` — `#ED8B00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 144 C"))]
    Pms144C,
    /// `Pantone 144 U` — `#F38B3C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 144 U"))]
    Pms144U,
    /// `Pantone 145 C` — `#CF7F00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 145 C"))]
    Pms145C,
    /// `Pantone 145 U` — `#C97F3A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 145 U"))]
    Pms145U,
    /// `Pantone 146 C` — `#A76D11`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 146 C"))]
    Pms146C,
    /// `Pantone 146 U` — `#9D7441`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 146 U"))]
    Pms146U,
    /// `Pantone 147 C` — `#715C2A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 147 C"))]
    Pms147C,
    /// `Pantone 147 U` — `#76694A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 147 U"))]
    Pms147U,
    /// `Pantone 148 C` — `#FECB8B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 148 C"))]
    Pms148C,
    /// `Pantone 148 U` — `#FFC688`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 148 U"))]
    Pms148U,
    /// `Pantone 1485 C` — `#FFAE62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1485 C"))]
    Pms1485C,
    /// `Pantone 1485 U` — `#FFA563`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1485 U"))]
    Pms1485U,
    /// `Pantone 149 C` — `#FFC27B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 149 C"))]
    Pms149C,
    /// `Pantone 149 U` — `#FFB575`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 149 U"))]
    Pms149U,
    /// `Pantone 1495 C` — `#FF8F1C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1495 C"))]
    Pms1495C,
    /// `Pantone 1495 U` — `#FF9351`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1495 U"))]
    Pms1495U,
    /// `Pantone 150 C` — `#FFB25B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 150 C"))]
    Pms150C,
    /// `Pantone 150 U` — `#FF9955`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 150 U"))]
    Pms150U,
    /// `Pantone 1505 C` — `#FF6900`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1505 C"))]
    Pms1505C,
    /// `Pantone 1505 U` — `#FF8038`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1505 U"))]
    Pms1505U,
    /// `Pantone 151 C` — `#FF8200`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 151 C"))]
    Pms151C,
    /// `Pantone 151 U` — `#FF883E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 151 U"))]
    Pms151U,
    /// `Pantone 152 C` — `#E57200`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 152 C"))]
    Pms152C,
    /// `Pantone 152 U` — `#E07A3F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 152 U"))]
    Pms152U,
    /// `Pantone 1525 C` — `#B94700`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1525 C"))]
    Pms1525C,
    /// `Pantone 1525 U` — `#BD6439`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1525 U"))]
    Pms1525U,
    /// `Pantone 153 C` — `#BE6A14`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 153 C"))]
    Pms153C,
    /// `Pantone 153 U` — `#AC6F42`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 153 U"))]
    Pms153U,
    /// `Pantone 1535 C` — `#94450B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1535 C"))]
    Pms1535C,
    /// `Pantone 1535 U` — `#976241`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1535 U"))]
    Pms1535U,
    /// `Pantone 154 C` — `#9B5A1A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 154 C"))]
    Pms154C,
    /// `Pantone 154 U` — `#926844`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 154 U"))]
    Pms154U,
    /// `Pantone 1545 C` — `#653819`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1545 C"))]
    Pms1545C,
    /// `Pantone 1545 U` — `#7A5F48`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1545 U"))]
    Pms1545U,
    /// `Pantone 155 C` — `#EFD19F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 155 C"))]
    Pms155C,
    /// `Pantone 155 U` — `#F7C995`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 155 U"))]
    Pms155U,
    /// `Pantone 1555 C` — `#FFB990`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1555 C"))]
    Pms1555C,
    /// `Pantone 1555 U` — `#FFC29E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1555 U"))]
    Pms1555U,
    /// `Pantone 156 C` — `#EFBE7D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 156 C"))]
    Pms156C,
    /// `Pantone 156 U` — `#F7B984`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 156 U"))]
    Pms156U,
    /// `Pantone 1565 C` — `#FFA06A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1565 C"))]
    Pms1565C,
    /// `Pantone 1565 U` — `#FFAA81`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1565 U"))]
    Pms1565U,
    /// `Pantone 157 C` — `#ECA154`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 157 C"))]
    Pms157C,
    /// `Pantone 157 U` — `#F39662`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 157 U"))]
    Pms157U,
    /// `Pantone 1575 C` — `#FF7F32`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1575 C"))]
    Pms1575C,
    /// `Pantone 1575 U` — `#FF9665`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1575 U"))]
    Pms1575U,
    /// `Pantone 158 C` — `#E87722`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 158 C"))]
    Pms158C,
    /// `Pantone 158 U` — `#EE7F4B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 158 U"))]
    Pms158U,
    /// `Pantone 1585 C` — `#FF6A13`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1585 C"))]
    Pms1585C,
    /// `Pantone 1585 U` — `#FF8450`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1585 U"))]
    Pms1585U,
    /// `Pantone 159 C` — `#CB6015`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 159 C"))]
    Pms159C,
    /// `Pantone 159 U` — `#C97146`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 159 U"))]
    Pms159U,
    /// `Pantone 1595 C` — `#D86018`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1595 C"))]
    Pms1595C,
    /// `Pantone 1595 U` — `#D37046`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1595 U"))]
    Pms1595U,
    /// `Pantone 160 C` — `#A1561C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 160 C"))]
    Pms160C,
    /// `Pantone 160 U` — `#9D6B47`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 160 U"))]
    Pms160U,
    /// `Pantone 1605 C` — `#A65523`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1605 C"))]
    Pms1605C,
    /// `Pantone 1605 U` — `#A76C4C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1605 U"))]
    Pms1605U,
    /// `Pantone 161 C` — `#603D20`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 161 C"))]
    Pms161C,
    /// `Pantone 161 U` — `#786249`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 161 U"))]
    Pms161U,
    /// `Pantone 1615 C` — `#8B4720`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1615 C"))]
    Pms1615C,
    /// `Pantone 1615 U` — `#93654A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1615 U"))]
    Pms1615U,
    /// `Pantone 162 C` — `#FFBE9F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 162 C"))]
    Pms162C,
    /// `Pantone 162 U` — `#FFBB9B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 162 U"))]
    Pms162U,
    /// `Pantone 1625 C` — `#FFA38B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1625 C"))]
    Pms1625C,
    /// `Pantone 1625 U` — `#FFA891`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1625 U"))]
    Pms1625U,
    /// `Pantone 163 C` — `#FF9D6E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 163 C"))]
    Pms163C,
    /// `Pantone 163 U` — `#FF9C76`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 163 U"))]
    Pms163U,
    /// `Pantone 1635 C` — `#FF8D6D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1635 C"))]
    Pms1635C,
    /// `Pantone 1635 U` — `#FF967E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1635 U"))]
    Pms1635U,
    /// `Pantone 164 C` — `#FF7F41`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 164 C"))]
    Pms164C,
    /// `Pantone 164 U` — `#FF865F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 164 U"))]
    Pms164U,
    /// `Pantone 1645 C` — `#FF6A39`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1645 C"))]
    Pms1645C,
    /// `Pantone 1645 U` — `#FF7F61`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1645 U"))]
    Pms1645U,
    /// `Pantone 165 C` — `#FF671F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 165 C"))]
    Pms165C,
    /// `Pantone 165 U` — `#FF7548`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 165 U"))]
    Pms165U,
    /// `Pantone 1655 C` — `#FC4C02`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1655 C"))]
    Pms1655C,
    /// `Pantone 1655 U` — `#FF6F4C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1655 U"))]
    Pms1655U,
    /// `Pantone 166 C` — `#E35205`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 166 C"))]
    Pms166C,
    /// `Pantone 166 U` — `#E16740`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 166 U"))]
    Pms166U,
    /// `Pantone 1665 C` — `#DC4405`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1665 C"))]
    Pms1665C,
    /// `Pantone 1665 U` — `#E56849`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1665 U"))]
    Pms1665U,
    /// `Pantone 167 C` — `#BE531C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 167 C"))]
    Pms167C,
    /// `Pantone 167 U` — `#B46547`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 167 U"))]
    Pms167U,
    /// `Pantone 1675 C` — `#A9431E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1675 C"))]
    Pms1675C,
    /// `Pantone 1675 U` — `#AB614A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1675 U"))]
    Pms1675U,
    /// `Pantone 168 C` — `#73381D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 168 C"))]
    Pms168C,
    /// `Pantone 168 U` — `#8A614B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 168 U"))]
    Pms168U,
    /// `Pantone 1685 C` — `#833921`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1685 C"))]
    Pms1685C,
    /// `Pantone 1685 U` — `#95614D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1685 U"))]
    Pms1685U,
    /// `Pantone 169 C` — `#FFB3AB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 169 C"))]
    Pms169C,
    /// `Pantone 169 U` — `#FFB7AE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 169 U"))]
    Pms169U,
    /// `Pantone 170 C` — `#FF8674`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 170 C"))]
    Pms170C,
    /// `Pantone 170 U` — `#FF8D7E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 170 U"))]
    Pms170U,
    /// `Pantone 171 C` — `#FF5C39`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 171 C"))]
    Pms171C,
    /// `Pantone 171 U` — `#FF7865`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 171 U"))]
    Pms171U,
    /// `Pantone 172 C` — `#FA4616`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 172 C"))]
    Pms172C,
    /// `Pantone 172 U` — `#FF674D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 172 U"))]
    Pms172U,
    /// `Pantone 173 C` — `#CF4520`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 173 C"))]
    Pms173C,
    /// `Pantone 173 U` — `#CE614A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 173 U"))]
    Pms173U,
    /// `Pantone 174 C` — `#963821`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 174 C"))]
    Pms174C,
    /// `Pantone 174 U` — `#9C5B4B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 174 U"))]
    Pms174U,
    /// `Pantone 175 C` — `#6B3529`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 175 C"))]
    Pms175C,
    /// `Pantone 175 U` — `#7D594D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 175 U"))]
    Pms175U,
    /// `Pantone 176 C` — `#FFB1BB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 176 C"))]
    Pms176C,
    /// `Pantone 176 U` — `#FFB1BA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 176 U"))]
    Pms176U,
    /// `Pantone 1765 C` — `#FFA3B5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1765 C"))]
    Pms1765C,
    /// `Pantone 1765 U` — `#FFA7B6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1765 U"))]
    Pms1765U,
    /// `Pantone 1767 C` — `#FCAFC0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1767 C"))]
    Pms1767C,
    /// `Pantone 1767 U` — `#FFBAC9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1767 U"))]
    Pms1767U,
    /// `Pantone 177 C` — `#FF808B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 177 C"))]
    Pms177C,
    /// `Pantone 177 U` — `#FF8C91`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 177 U"))]
    Pms177U,
    /// `Pantone 1775 C` — `#FF8DA1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1775 C"))]
    Pms1775C,
    /// `Pantone 1775 U` — `#FF8B9A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1775 U"))]
    Pms1775U,
    /// `Pantone 1777 C` — `#FB637E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1777 C"))]
    Pms1777C,
    /// `Pantone 1777 U` — `#FF8091`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1777 U"))]
    Pms1777U,
    /// `Pantone 178 C` — `#FF585D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 178 C"))]
    Pms178C,
    /// `Pantone 178 U` — `#FF7677`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 178 U"))]
    Pms178U,
    /// `Pantone 1785 C` — `#F8485E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1785 C"))]
    Pms1785C,
    /// `Pantone 1785 U` — `#FE6672`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1785 U"))]
    Pms1785U,
    /// `Pantone 1787 C` — `#F4364C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1787 C"))]
    Pms1787C,
    /// `Pantone 1787 U` — `#FC6371`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1787 U"))]
    Pms1787U,
    /// `Pantone 1788 C` — `#EE2737`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1788 C"))]
    Pms1788C,
    /// `Pantone 1788 U` — `#F95D62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1788 U"))]
    Pms1788U,
    /// `Pantone 179 C` — `#E03C31`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 179 C"))]
    Pms179C,
    /// `Pantone 179 U` — `#E15F55`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 179 U"))]
    Pms179U,
    /// `Pantone 1795 C` — `#D22630`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1795 C"))]
    Pms1795C,
    /// `Pantone 1795 U` — `#DA5959`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1795 U"))]
    Pms1795U,
    /// `Pantone 1797 C` — `#CB333B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1797 C"))]
    Pms1797C,
    /// `Pantone 1797 U` — `#CB4F53`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1797 U"))]
    Pms1797U,
    /// `Pantone 180 C` — `#BE3A34`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 180 C"))]
    Pms180C,
    /// `Pantone 180 U` — `#BF6059`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 180 U"))]
    Pms180U,
    /// `Pantone 1805 C` — `#AF272F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1805 C"))]
    Pms1805C,
    /// `Pantone 1805 U` — `#B45858`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1805 U"))]
    Pms1805U,
    /// `Pantone 1807 C` — `#A4343A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1807 C"))]
    Pms1807C,
    /// `Pantone 1807 U` — `#A75154`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1807 U"))]
    Pms1807U,
    /// `Pantone 181 C` — `#81312F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 181 C"))]
    Pms181C,
    /// `Pantone 181 U` — `#8B5952`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 181 U"))]
    Pms181U,
    /// `Pantone 1815 C` — `#7C2529`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1815 C"))]
    Pms1815C,
    /// `Pantone 1815 U` — `#8F5652`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1815 U"))]
    Pms1815U,
    /// `Pantone 1817 C` — `#643335`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1817 C"))]
    Pms1817C,
    /// `Pantone 1817 U` — `#795453`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1817 U"))]
    Pms1817U,
    /// `Pantone 182 C` — `#FABBCB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 182 C"))]
    Pms182C,
    /// `Pantone 182 U` — `#FFB8CA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 182 U"))]
    Pms182U,
    /// `Pantone 183 C` — `#FC9BB3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 183 C"))]
    Pms183C,
    /// `Pantone 183 U` — `#FF8DA3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 183 U"))]
    Pms183U,
    /// `Pantone 184 C` — `#F65275`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 184 C"))]
    Pms184C,
    /// `Pantone 184 U` — `#FC6E83`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 184 U"))]
    Pms184U,
    /// `Pantone 185 C` — `#E4002B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 185 C"))]
    Pms185C,
    /// `Pantone 185 U` — `#F15060`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 185 U"))]
    Pms185U,
    /// `Pantone 186 C` — `#C8102E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 186 C"))]
    Pms186C,
    /// `Pantone 186 U` — `#D2515E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 186 U"))]
    Pms186U,
    /// `Pantone 187 C` — `#A6192E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 187 C"))]
    Pms187C,
    /// `Pantone 187 U` — `#AE535C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 187 U"))]
    Pms187U,
    /// `Pantone 188 C` — `#76232F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 188 C"))]
    Pms188C,
    /// `Pantone 188 U` — `#895559`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 188 U"))]
    Pms188U,
    /// `Pantone 189 C` — `#F8A3BC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 189 C"))]
    Pms189C,
    /// `Pantone 189 U` — `#FFABC2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 189 U"))]
    Pms189U,
    /// `Pantone 1895 C` — `#F5B6CD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1895 C"))]
    Pms1895C,
    /// `Pantone 1895 U` — `#FEB3CD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1895 U"))]
    Pms1895U,
    /// `Pantone 190 C` — `#F67599`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 190 C"))]
    Pms190C,
    /// `Pantone 190 U` — `#FE87A3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 190 U"))]
    Pms190U,
    /// `Pantone 1905 C` — `#F59BBB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1905 C"))]
    Pms1905C,
    /// `Pantone 1905 U` — `#FD9CBA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1905 U"))]
    Pms1905U,
    /// `Pantone 191 C` — `#EF426F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 191 C"))]
    Pms191C,
    /// `Pantone 191 U` — `#F87089`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 191 U"))]
    Pms191U,
    /// `Pantone 1915 C` — `#EF4A81`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1915 C"))]
    Pms1915C,
    /// `Pantone 1915 U` — `#F66F92`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1915 U"))]
    Pms1915U,
    /// `Pantone 192 C` — `#E40046`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 192 C"))]
    Pms192C,
    /// `Pantone 192 U` — `#EE536B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 192 U"))]
    Pms192U,
    /// `Pantone 1925 C` — `#E0004D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1925 C"))]
    Pms1925C,
    /// `Pantone 1925 U` — `#E9506E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1925 U"))]
    Pms1925U,
    /// `Pantone 193 C` — `#BF0D3E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 193 C"))]
    Pms193C,
    /// `Pantone 193 U` — `#C55265`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 193 U"))]
    Pms193U,
    /// `Pantone 1935 C` — `#C5003E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1935 C"))]
    Pms1935C,
    /// `Pantone 1935 U` — `#C94C63`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1935 U"))]
    Pms1935U,
    /// `Pantone 194 C` — `#9B2743`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 194 C"))]
    Pms194C,
    /// `Pantone 194 U` — `#A35965`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 194 U"))]
    Pms194U,
    /// `Pantone 1945 C` — `#A6093D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1945 C"))]
    Pms1945C,
    /// `Pantone 1945 U` — `#B05468`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1945 U"))]
    Pms1945U,
    /// `Pantone 195 C` — `#782F40`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 195 C"))]
    Pms195C,
    /// `Pantone 195 U` — `#885A61`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 195 U"))]
    Pms195U,
    /// `Pantone 1955 C` — `#8A1538`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1955 C"))]
    Pms1955C,
    /// `Pantone 1955 U` — `#975361`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 1955 U"))]
    Pms1955U,
    /// `Pantone 196 C` — `#ECC7CD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 196 C"))]
    Pms196C,
    /// `Pantone 196 U` — `#F4C3CB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 196 U"))]
    Pms196U,
    /// `Pantone 197 C` — `#E89CAE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 197 C"))]
    Pms197C,
    /// `Pantone 197 U` — `#F2A1B2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 197 U"))]
    Pms197U,
    /// `Pantone 198 C` — `#DF4661`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 198 C"))]
    Pms198C,
    /// `Pantone 198 U` — `#E76B7C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 198 U"))]
    Pms198U,
    /// `Pantone 199 C` — `#D50032`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 199 C"))]
    Pms199C,
    /// `Pantone 199 U` — `#DD5061`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 199 U"))]
    Pms199U,
    /// `Pantone 200 C` — `#BA0C2F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 200 C"))]
    Pms200C,
    /// `Pantone 200 U` — `#BD4F5C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 200 U"))]
    Pms200U,
    /// `Pantone 201 C` — `#9D2235`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 201 C"))]
    Pms201C,
    /// `Pantone 201 U` — `#A15760`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 201 U"))]
    Pms201U,
    /// `Pantone 202 C` — `#862633`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 202 C"))]
    Pms202C,
    /// `Pantone 202 U` — `#90585E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 202 U"))]
    Pms202U,
    /// `Pantone 203 C` — `#ECB3CB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 203 C"))]
    Pms203C,
    /// `Pantone 203 U` — `#F5B1CC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 203 U"))]
    Pms203U,
    /// `Pantone 204 C` — `#E782A9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 204 C"))]
    Pms204C,
    /// `Pantone 204 U` — `#EE82A4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 204 U"))]
    Pms204U,
    /// `Pantone 205 C` — `#E0457B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 205 C"))]
    Pms205C,
    /// `Pantone 205 U` — `#E56284`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 205 U"))]
    Pms205U,
    /// `Pantone 206 C` — `#CE0037`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 206 C"))]
    Pms206C,
    /// `Pantone 206 U` — `#D84465`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 206 U"))]
    Pms206U,
    /// `Pantone 207 C` — `#A50034`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 207 C"))]
    Pms207C,
    /// `Pantone 207 U` — `#AF4C64`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 207 U"))]
    Pms207U,
    /// `Pantone 208 C` — `#861F41`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 208 C"))]
    Pms208C,
    /// `Pantone 208 U` — `#905567`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 208 U"))]
    Pms208U,
    /// `Pantone 209 C` — `#6F263D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 209 C"))]
    Pms209C,
    /// `Pantone 209 U` — `#825864`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 209 U"))]
    Pms209U,
    /// `Pantone 210 C` — `#F99FC9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 210 C"))]
    Pms210C,
    /// `Pantone 210 U` — `#FFA2CB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 210 U"))]
    Pms210U,
    /// `Pantone 211 C` — `#F57EB6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 211 C"))]
    Pms211C,
    /// `Pantone 211 U` — `#FC85B6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 211 U"))]
    Pms211U,
    /// `Pantone 212 C` — `#F04E98`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 212 C"))]
    Pms212C,
    /// `Pantone 212 U` — `#F56D9E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 212 U"))]
    Pms212U,
    /// `Pantone 213 C` — `#E31C79`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 213 C"))]
    Pms213C,
    /// `Pantone 213 U` — `#E95485`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 213 U"))]
    Pms213U,
    /// `Pantone 214 C` — `#CE0F69`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 214 C"))]
    Pms214C,
    /// `Pantone 214 U` — `#D1517A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 214 U"))]
    Pms214U,
    /// `Pantone 215 C` — `#AC145A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 215 C"))]
    Pms215C,
    /// `Pantone 215 U` — `#AE5171`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 215 U"))]
    Pms215U,
    /// `Pantone 216 C` — `#7D2248`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 216 C"))]
    Pms216C,
    /// `Pantone 216 U` — `#8A5566`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 216 U"))]
    Pms216U,
    /// `Pantone 217 C` — `#EABEDB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 217 C"))]
    Pms217C,
    /// `Pantone 217 U` — `#F4C1E1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 217 U"))]
    Pms217U,
    /// `Pantone 218 C` — `#E56DB1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 218 C"))]
    Pms218C,
    /// `Pantone 218 U` — `#EF84B9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 218 U"))]
    Pms218U,
    /// `Pantone 219 C` — `#DA1884`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 219 C"))]
    Pms219C,
    /// `Pantone 219 U` — `#E45C96`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 219 U"))]
    Pms219U,
    /// `Pantone 220 C` — `#A50050`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 220 C"))]
    Pms220C,
    /// `Pantone 220 U` — `#B04C75`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 220 U"))]
    Pms220U,
    /// `Pantone 221 C` — `#910048`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 221 C"))]
    Pms221C,
    /// `Pantone 221 U` — `#9E4C6E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 221 U"))]
    Pms221U,
    /// `Pantone 222 C` — `#6C1D45`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 222 C"))]
    Pms222C,
    /// `Pantone 222 U` — `#82566B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 222 U"))]
    Pms222U,
    /// `Pantone 223 C` — `#EF95CF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 223 C"))]
    Pms223C,
    /// `Pantone 223 U` — `#F9A4D8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 223 U"))]
    Pms223U,
    /// `Pantone 224 C` — `#EB6FBD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 224 C"))]
    Pms224C,
    /// `Pantone 224 U` — `#F587C7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 224 U"))]
    Pms224U,
    /// `Pantone 225 C` — `#DF1995`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 225 C"))]
    Pms225C,
    /// `Pantone 225 U` — `#EA60A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 225 U"))]
    Pms225U,
    /// `Pantone 226 C` — `#D0006F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 226 C"))]
    Pms226C,
    /// `Pantone 226 U` — `#DC4588`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 226 U"))]
    Pms226U,
    /// `Pantone 227 C` — `#AA0061`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 227 C"))]
    Pms227C,
    /// `Pantone 227 U` — `#AF4879`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 227 U"))]
    Pms227U,
    /// `Pantone 228 C` — `#890C58`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 228 C"))]
    Pms228C,
    /// `Pantone 228 U` — `#8D4D6D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 228 U"))]
    Pms228U,
    /// `Pantone 229 C` — `#672146`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 229 C"))]
    Pms229C,
    /// `Pantone 229 U` — `#7A5466`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 229 U"))]
    Pms229U,
    /// `Pantone 230 C` — `#F4A6D7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 230 C"))]
    Pms230C,
    /// `Pantone 230 U` — `#FEB2E0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 230 U"))]
    Pms230U,
    /// `Pantone 231 C` — `#F277C6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 231 C"))]
    Pms231C,
    /// `Pantone 231 U` — `#F984CA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 231 U"))]
    Pms231U,
    /// `Pantone 232 C` — `#E93CAC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 232 C"))]
    Pms232C,
    /// `Pantone 232 U` — `#F16AB7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 232 U"))]
    Pms232U,
    /// `Pantone 233 C` — `#C6007E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 233 C"))]
    Pms233C,
    /// `Pantone 233 U` — `#C84687`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 233 U"))]
    Pms233U,
    /// `Pantone 234 C` — `#A20067`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 234 C"))]
    Pms234C,
    /// `Pantone 234 U` — `#AA4C7D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 234 U"))]
    Pms234U,
    /// `Pantone 235 C` — `#840B55`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 235 C"))]
    Pms235C,
    /// `Pantone 235 U` — `#914E72`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 235 U"))]
    Pms235U,
    /// `Pantone 236 C` — `#F1A7DC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 236 C"))]
    Pms236C,
    /// `Pantone 236 U` — `#F8A6DE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 236 U"))]
    Pms236U,
    /// `Pantone 2365 C` — `#EFBAE1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2365 C"))]
    Pms2365C,
    /// `Pantone 2365 U` — `#F8BFE7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2365 U"))]
    Pms2365U,
    /// `Pantone 237 C` — `#EC86D0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 237 C"))]
    Pms237C,
    /// `Pantone 237 U` — `#F28DD2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 237 U"))]
    Pms237U,
    /// `Pantone 2375 C` — `#E277CD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2375 C"))]
    Pms2375C,
    /// `Pantone 2375 U` — `#EA88D2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2375 U"))]
    Pms2375U,
    /// `Pantone 238 C` — `#E45DBF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 238 C"))]
    Pms238C,
    /// `Pantone 238 U` — `#E971C0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 238 U"))]
    Pms238U,
    /// `Pantone 2385 C` — `#D539B5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2385 C"))]
    Pms2385C,
    /// `Pantone 2385 U` — `#E06FC2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2385 U"))]
    Pms2385U,
    /// `Pantone 239 C` — `#DB3EB1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 239 C"))]
    Pms239C,
    /// `Pantone 239 U` — `#DB55AA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 239 U"))]
    Pms239U,
    /// `Pantone 2395 C` — `#C800A1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2395 C"))]
    Pms2395C,
    /// `Pantone 2395 U` — `#D557AF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2395 U"))]
    Pms2395U,
    /// `Pantone 240 C` — `#C5299B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 240 C"))]
    Pms240C,
    /// `Pantone 240 U` — `#C15598`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 240 U"))]
    Pms240U,
    /// `Pantone 2405 C` — `#B0008E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2405 C"))]
    Pms2405C,
    /// `Pantone 2405 U` — `#BB569D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2405 U"))]
    Pms2405U,
    /// `Pantone 241 C` — `#AF1685`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 241 C"))]
    Pms241C,
    /// `Pantone 241 U` — `#AF548B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 241 U"))]
    Pms241U,
    /// `Pantone 2415 C` — `#9E007E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2415 C"))]
    Pms2415C,
    /// `Pantone 2415 U` — `#AB5491`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2415 U"))]
    Pms2415U,
    /// `Pantone 242 C` — `#80225F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 242 C"))]
    Pms242C,
    /// `Pantone 242 U` — `#8D5977`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 242 U"))]
    Pms242U,
    /// `Pantone 2425 C` — `#830065`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2425 C"))]
    Pms2425C,
    /// `Pantone 2425 U` — `#975381`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2425 U"))]
    Pms2425U,
    /// `Pantone 243 C` — `#EAB8E4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 243 C"))]
    Pms243C,
    /// `Pantone 243 U` — `#F1B2E6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 243 U"))]
    Pms243U,
    /// `Pantone 244 C` — `#E59BDC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 244 C"))]
    Pms244C,
    /// `Pantone 244 U` — `#EB9ADE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 244 U"))]
    Pms244U,
    /// `Pantone 245 C` — `#DD7FD3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 245 C"))]
    Pms245C,
    /// `Pantone 245 U` — `#E07FD0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 245 U"))]
    Pms245U,
    /// `Pantone 246 C` — `#C724B1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 246 C"))]
    Pms246C,
    /// `Pantone 246 U` — `#CE57B3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 246 U"))]
    Pms246U,
    /// `Pantone 247 C` — `#BB16A3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 247 C"))]
    Pms247C,
    /// `Pantone 247 U` — `#BD51A2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 247 U"))]
    Pms247U,
    /// `Pantone 248 C` — `#A51890`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 248 C"))]
    Pms248C,
    /// `Pantone 248 U` — `#A14F8C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 248 U"))]
    Pms248U,
    /// `Pantone 249 C` — `#80276C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 249 C"))]
    Pms249C,
    /// `Pantone 249 U` — `#885779`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 249 U"))]
    Pms249U,
    /// `Pantone 250 C` — `#E7BAE4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 250 C"))]
    Pms250C,
    /// `Pantone 250 U` — `#EEBAE9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 250 U"))]
    Pms250U,
    /// `Pantone 251 C` — `#DD9CDF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 251 C"))]
    Pms251C,
    /// `Pantone 251 U` — `#E096DF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 251 U"))]
    Pms251U,
    /// `Pantone 252 C` — `#C964CF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 252 C"))]
    Pms252C,
    /// `Pantone 252 U` — `#CE70CC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 252 U"))]
    Pms252U,
    /// `Pantone 253 C` — `#AD1AAC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 253 C"))]
    Pms253C,
    /// `Pantone 253 U` — `#B456AE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 253 U"))]
    Pms253U,
    /// `Pantone 254 C` — `#981D97`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 254 C"))]
    Pms254C,
    /// `Pantone 254 U` — `#9E549A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 254 U"))]
    Pms254U,
    /// `Pantone 255 C` — `#72246C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 255 C"))]
    Pms255C,
    /// `Pantone 255 U` — `#83577D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 255 U"))]
    Pms255U,
    /// `Pantone 256 C` — `#D6BFDD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 256 C"))]
    Pms256C,
    /// `Pantone 256 U` — `#D4B4DA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 256 U"))]
    Pms256U,
    /// `Pantone 2562 C` — `#D7A9E3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2562 C"))]
    Pms2562C,
    /// `Pantone 2562 U` — `#DDAEE9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2562 U"))]
    Pms2562U,
    /// `Pantone 2563 C` — `#CBA3D8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2563 C"))]
    Pms2563C,
    /// `Pantone 2563 U` — `#CDA2D9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2563 U"))]
    Pms2563U,
    /// `Pantone 2567 C` — `#C1A0DA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2567 C"))]
    Pms2567C,
    /// `Pantone 2567 U` — `#C09FDB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2567 U"))]
    Pms2567U,
    /// `Pantone 257 C` — `#C6A1CF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 257 C"))]
    Pms257C,
    /// `Pantone 257 U` — `#C29BCB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 257 U"))]
    Pms257U,
    /// `Pantone 2572 C` — `#C98BDB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2572 C"))]
    Pms2572C,
    /// `Pantone 2572 U` — `#CD90DE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2572 U"))]
    Pms2572U,
    /// `Pantone 2573 C` — `#B884CB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2573 C"))]
    Pms2573C,
    /// `Pantone 2573 U` — `#B989CB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2573 U"))]
    Pms2573U,
    /// `Pantone 2577 C` — `#A77BCA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2577 C"))]
    Pms2577C,
    /// `Pantone 2577 U` — `#A47EC6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2577 U"))]
    Pms2577U,
    /// `Pantone 258 C` — `#8C4799`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 258 C"))]
    Pms258C,
    /// `Pantone 258 U` — `#A276AE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 258 U"))]
    Pms258U,
    /// `Pantone 2582 C` — `#AC4FC6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2582 C"))]
    Pms2582C,
    /// `Pantone 2582 U` — `#BB76CF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2582 U"))]
    Pms2582U,
    /// `Pantone 2583 C` — `#A05EB5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2583 C"))]
    Pms2583C,
    /// `Pantone 2583 U` — `#A372B5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2583 U"))]
    Pms2583U,
    /// `Pantone 2587 C` — `#8246AF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2587 C"))]
    Pms2587C,
    /// `Pantone 2587 U` — `#936DB7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2587 U"))]
    Pms2587U,
    /// `Pantone 259 C` — `#6D2077`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 259 C"))]
    Pms259C,
    /// `Pantone 259 U` — `#94679C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 259 U"))]
    Pms259U,
    /// `Pantone 2592 C` — `#9B26B6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2592 C"))]
    Pms2592C,
    /// `Pantone 2592 U` — `#AA60BF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2592 U"))]
    Pms2592U,
    /// `Pantone 2593 C` — `#84329B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2593 C"))]
    Pms2593C,
    /// `Pantone 2593 U` — `#8E5DA0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2593 U"))]
    Pms2593U,
    /// `Pantone 2597 C` — `#5C068C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2597 C"))]
    Pms2597C,
    /// `Pantone 2597 U` — `#825AA4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2597 U"))]
    Pms2597U,
    /// `Pantone 260 C` — `#642667`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 260 C"))]
    Pms260C,
    /// `Pantone 260 U` — `#8A658D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 260 U"))]
    Pms260U,
    /// `Pantone 2602 C` — `#87189D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2602 C"))]
    Pms2602C,
    /// `Pantone 2602 U` — `#9757A6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2602 U"))]
    Pms2602U,
    /// `Pantone 2603 C` — `#702082`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2603 C"))]
    Pms2603C,
    /// `Pantone 2603 U` — `#845991`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2603 U"))]
    Pms2603U,
    /// `Pantone 2607 C` — `#500778`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2607 C"))]
    Pms2607C,
    /// `Pantone 2607 U` — `#7A5896`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2607 U"))]
    Pms2607U,
    /// `Pantone 261 C` — `#5D285F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 261 C"))]
    Pms261C,
    /// `Pantone 261 U` — `#7F6282`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 261 U"))]
    Pms261U,
    /// `Pantone 2612 C` — `#772583`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2612 C"))]
    Pms2612C,
    /// `Pantone 2612 U` — `#86578D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2612 U"))]
    Pms2612U,
    /// `Pantone 2613 C` — `#671E75`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2613 C"))]
    Pms2613C,
    /// `Pantone 2613 U` — `#7D5787`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2613 U"))]
    Pms2613U,
    /// `Pantone 2617 C` — `#470A68`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2617 C"))]
    Pms2617C,
    /// `Pantone 2617 U` — `#735489`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2617 U"))]
    Pms2617U,
    /// `Pantone 262 C` — `#51284F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 262 C"))]
    Pms262C,
    /// `Pantone 262 U` — `#755E74`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 262 U"))]
    Pms262U,
    /// `Pantone 2622 C` — `#653165`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2622 C"))]
    Pms2622C,
    /// `Pantone 2622 U` — `#7B5B7A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2622 U"))]
    Pms2622U,
    /// `Pantone 2623 C` — `#5F2167`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2623 C"))]
    Pms2623C,
    /// `Pantone 2623 U` — `#75557A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2623 U"))]
    Pms2623U,
    /// `Pantone 2627 C` — `#3C1053`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2627 C"))]
    Pms2627C,
    /// `Pantone 2627 U` — `#6E527C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2627 U"))]
    Pms2627U,
    /// `Pantone 263 C` — `#D7C6E6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 263 C"))]
    Pms263C,
    /// `Pantone 263 U` — `#DCC6EA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 263 U"))]
    Pms263U,
    /// `Pantone 2635 C` — `#C5B4E3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2635 C"))]
    Pms2635C,
    /// `Pantone 2635 U` — `#C9B7E8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2635 U"))]
    Pms2635U,
    /// `Pantone 264 C` — `#C1A7E2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 264 C"))]
    Pms264C,
    /// `Pantone 264 U` — `#BFA1E3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 264 U"))]
    Pms264U,
    /// `Pantone 2645 C` — `#AD96DC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2645 C"))]
    Pms2645C,
    /// `Pantone 2645 U` — `#B9A3E3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2645 U"))]
    Pms2645U,
    /// `Pantone 265 C` — `#9063CD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 265 C"))]
    Pms265C,
    /// `Pantone 265 U` — `#9D7AD2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 265 U"))]
    Pms265U,
    /// `Pantone 2655 C` — `#9678D3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2655 C"))]
    Pms2655C,
    /// `Pantone 2655 U` — `#A288D9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2655 U"))]
    Pms2655U,
    /// `Pantone 266 C` — `#753BBD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 266 C"))]
    Pms266C,
    /// `Pantone 266 U` — `#8A65C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 266 U"))]
    Pms266U,
    /// `Pantone 2665 C` — `#7D55C7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2665 C"))]
    Pms2665C,
    /// `Pantone 2665 U` — `#8C6FCA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2665 U"))]
    Pms2665U,
    /// `Pantone 267 C` — `#5F259F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 267 C"))]
    Pms267C,
    /// `Pantone 267 U` — `#7F5DAE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 267 U"))]
    Pms267U,
    /// `Pantone 268 C` — `#582C83`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 268 C"))]
    Pms268C,
    /// `Pantone 268 U` — `#735990`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 268 U"))]
    Pms268U,
    /// `Pantone 2685 C` — `#330072`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2685 C"))]
    Pms2685C,
    /// `Pantone 2685 U` — `#765BA7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2685 U"))]
    Pms2685U,
    /// `Pantone 269 C` — `#512D6D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 269 C"))]
    Pms269C,
    /// `Pantone 269 U` — `#6E587F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 269 U"))]
    Pms269U,
    /// `Pantone 2695 C` — `#2E1A47`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2695 C"))]
    Pms2695C,
    /// `Pantone 2695 U` — `#6C5D80`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2695 U"))]
    Pms2695U,
    /// `Pantone 270 C` — `#B4B5DF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 270 C"))]
    Pms270C,
    /// `Pantone 270 U` — `#ADAEDE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 270 U"))]
    Pms270U,
    /// `Pantone 2705 C` — `#A7A4E0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2705 C"))]
    Pms2705C,
    /// `Pantone 2705 U` — `#BAB6E9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2705 U"))]
    Pms2705U,
    /// `Pantone 2706 C` — `#CBD3EB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2706 C"))]
    Pms2706C,
    /// `Pantone 2706 U` — `#CCD4F1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2706 U"))]
    Pms2706U,
    /// `Pantone 2707 C` — `#C3D7EE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2707 C"))]
    Pms2707C,
    /// `Pantone 2707 U` — `#C7DBF4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2707 U"))]
    Pms2707U,
    /// `Pantone 2708 C` — `#B8CCEA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2708 C"))]
    Pms2708C,
    /// `Pantone 2708 U` — `#B2C9EF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2708 U"))]
    Pms2708U,
    /// `Pantone 271 C` — `#9595D2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 271 C"))]
    Pms271C,
    /// `Pantone 271 U` — `#9A9BD4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 271 U"))]
    Pms271U,
    /// `Pantone 2715 C` — `#8B84D7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2715 C"))]
    Pms2715C,
    /// `Pantone 2715 U` — `#9F98DF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2715 U"))]
    Pms2715U,
    /// `Pantone 2716 C` — `#9FAEE5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2716 C"))]
    Pms2716C,
    /// `Pantone 2716 U` — `#A2AFE9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2716 U"))]
    Pms2716U,
    /// `Pantone 2717 C` — `#A7C6ED`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2717 C"))]
    Pms2717C,
    /// `Pantone 2717 U` — `#A9C8F4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2717 U"))]
    Pms2717U,
    /// `Pantone 2718 C` — `#5C88DA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2718 C"))]
    Pms2718C,
    /// `Pantone 2718 U` — `#6A87D8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2718 U"))]
    Pms2718U,
    /// `Pantone 272 C` — `#7474C1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 272 C"))]
    Pms272C,
    /// `Pantone 272 U` — `#8586C6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 272 U"))]
    Pms272U,
    /// `Pantone 2725 C` — `#685BC7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2725 C"))]
    Pms2725C,
    /// `Pantone 2725 U` — `#8980D4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2725 U"))]
    Pms2725U,
    /// `Pantone 2726 C` — `#485CC7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2726 C"))]
    Pms2726C,
    /// `Pantone 2726 U` — `#717ACF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2726 U"))]
    Pms2726U,
    /// `Pantone 2727 C` — `#307FE2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2727 C"))]
    Pms2727C,
    /// `Pantone 2727 U` — `#6490E8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2727 U"))]
    Pms2727U,
    /// `Pantone 2728 C` — `#0047BB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2728 C"))]
    Pms2728C,
    /// `Pantone 2728 U` — `#5268C2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2728 U"))]
    Pms2728U,
    /// `Pantone 273 C` — `#24135F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 273 C"))]
    Pms273C,
    /// `Pantone 273 U` — `#645D9B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 273 U"))]
    Pms273U,
    /// `Pantone 2735 C` — `#2E008B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2735 C"))]
    Pms2735C,
    /// `Pantone 2735 U` — `#6857B2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2735 U"))]
    Pms2735U,
    /// `Pantone 2736 C` — `#1E22AA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2736 C"))]
    Pms2736C,
    /// `Pantone 2736 U` — `#5E60BA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2736 U"))]
    Pms2736U,
    /// `Pantone 2738 C` — `#06038D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2738 C"))]
    Pms2738C,
    /// `Pantone 2738 U` — `#474DA4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2738 U"))]
    Pms2738U,
    /// `Pantone 274 C` — `#211551`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 274 C"))]
    Pms274C,
    /// `Pantone 274 U` — `#5F578B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 274 U"))]
    Pms274U,
    /// `Pantone 2745 C` — `#280071`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2745 C"))]
    Pms2745C,
    /// `Pantone 2745 U` — `#61559A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2745 U"))]
    Pms2745U,
    /// `Pantone 2746 C` — `#171C8F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2746 C"))]
    Pms2746C,
    /// `Pantone 2746 U` — `#595CA5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2746 U"))]
    Pms2746U,
    /// `Pantone 2747 C` — `#001A72`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2747 C"))]
    Pms2747C,
    /// `Pantone 2747 U` — `#424F90`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2747 U"))]
    Pms2747U,
    /// `Pantone 2748 C` — `#001871`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2748 C"))]
    Pms2748C,
    /// `Pantone 2748 U` — `#484F8C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2748 U"))]
    Pms2748U,
    /// `Pantone 275 C` — `#201747`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 275 C"))]
    Pms275C,
    /// `Pantone 275 U` — `#5D5683`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 275 U"))]
    Pms275U,
    /// `Pantone 2755 C` — `#250E62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2755 C"))]
    Pms2755C,
    /// `Pantone 2755 U` — `#5D538B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2755 U"))]
    Pms2755U,
    /// `Pantone 2756 C` — `#151F6D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2756 C"))]
    Pms2756C,
    /// `Pantone 2756 U` — `#585C8C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2756 U"))]
    Pms2756U,
    /// `Pantone 2757 C` — `#001E60`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2757 C"))]
    Pms2757C,
    /// `Pantone 2757 U` — `#454F7E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2757 U"))]
    Pms2757U,
    /// `Pantone 2758 C` — `#001E62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2758 C"))]
    Pms2758C,
    /// `Pantone 2758 U` — `#484D7A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2758 U"))]
    Pms2758U,
    /// `Pantone 276 C` — `#221C35`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 276 C"))]
    Pms276C,
    /// `Pantone 276 U` — `#5C556F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 276 U"))]
    Pms276U,
    /// `Pantone 2765 C` — `#201547`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2765 C"))]
    Pms2765C,
    /// `Pantone 2765 U` — `#574F77`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2765 U"))]
    Pms2765U,
    /// `Pantone 2766 C` — `#141B4D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2766 C"))]
    Pms2766C,
    /// `Pantone 2766 U` — `#575A7C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2766 U"))]
    Pms2766U,
    /// `Pantone 2767 C` — `#13294B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2767 C"))]
    Pms2767C,
    /// `Pantone 2767 U` — `#4A4C64`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2767 U"))]
    Pms2767U,
    /// `Pantone 2768 C` — `#071D49`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2768 C"))]
    Pms2768C,
    /// `Pantone 2768 U` — `#4A4E6D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2768 U"))]
    Pms2768U,
    /// `Pantone 277 C` — `#ABCAE9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 277 C"))]
    Pms277C,
    /// `Pantone 277 U` — `#ADCFF1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 277 U"))]
    Pms277U,
    /// `Pantone 278 C` — `#8BB8E8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 278 C"))]
    Pms278C,
    /// `Pantone 278 U` — `#8DBAEE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 278 U"))]
    Pms278U,
    /// `Pantone 279 C` — `#418FDE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 279 C"))]
    Pms279C,
    /// `Pantone 279 U` — `#5B8EDB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 279 U"))]
    Pms279U,
    /// `Pantone 280 C` — `#012169`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 280 C"))]
    Pms280C,
    /// `Pantone 280 U` — `#3C4981`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 280 U"))]
    Pms280U,
    /// `Pantone 281 C` — `#00205B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 281 C"))]
    Pms281C,
    /// `Pantone 281 U` — `#3F4A75`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 281 U"))]
    Pms281U,
    /// `Pantone 282 C` — `#041E42`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 282 C"))]
    Pms282C,
    /// `Pantone 282 U` — `#434967`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 282 U"))]
    Pms282U,
    /// `Pantone 283 C` — `#92C1E9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 283 C"))]
    Pms283C,
    /// `Pantone 283 U` — `#9BC7EE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 283 U"))]
    Pms283U,
    /// `Pantone 284 C` — `#6CACE4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 284 C"))]
    Pms284C,
    /// `Pantone 284 U` — `#7AB1E8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 284 U"))]
    Pms284U,
    /// `Pantone 285 C` — `#0072CE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 285 C"))]
    Pms285C,
    /// `Pantone 285 U` — `#4982CF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 285 U"))]
    Pms285U,
    /// `Pantone 286 C` — `#0033A0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 286 C"))]
    Pms286C,
    /// `Pantone 286 U` — `#3255A4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 286 U"))]
    Pms286U,
    /// `Pantone 287 C` — `#003087`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 287 C"))]
    Pms287C,
    /// `Pantone 287 U` — `#3C5896`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 287 U"))]
    Pms287U,
    /// `Pantone 288 C` — `#002D72`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 288 C"))]
    Pms288C,
    /// `Pantone 288 U` — `#3D5588`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 288 U"))]
    Pms288U,
    /// `Pantone 289 C` — `#0C2340`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 289 C"))]
    Pms289C,
    /// `Pantone 289 U` — `#434F69`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 289 U"))]
    Pms289U,
    /// `Pantone 290 C` — `#B9D9EB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 290 C"))]
    Pms290C,
    /// `Pantone 290 U` — `#A7D4EE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 290 U"))]
    Pms290U,
    /// `Pantone 2905 C` — `#8DC8E8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2905 C"))]
    Pms2905C,
    /// `Pantone 2905 U` — `#8AC9ED`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2905 U"))]
    Pms2905U,
    /// `Pantone 291 C` — `#9BCBEB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 291 C"))]
    Pms291C,
    /// `Pantone 291 U` — `#7FBEEB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 291 U"))]
    Pms291U,
    /// `Pantone 2915 C` — `#62B5E5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2915 C"))]
    Pms2915C,
    /// `Pantone 2915 U` — `#64B3E8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2915 U"))]
    Pms2915U,
    /// `Pantone 292 C` — `#69B3E7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 292 C"))]
    Pms292C,
    /// `Pantone 292 U` — `#62A8E5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 292 U"))]
    Pms292U,
    /// `Pantone 2925 C` — `#009CDE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2925 C"))]
    Pms2925C,
    /// `Pantone 2925 U` — `#4097DB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2925 U"))]
    Pms2925U,
    /// `Pantone 293 C` — `#003DA5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 293 C"))]
    Pms293C,
    /// `Pantone 293 U` — `#235BA8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 293 U"))]
    Pms293U,
    /// `Pantone 2935 C` — `#0057B8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2935 C"))]
    Pms2935C,
    /// `Pantone 2935 U` — `#1B5FAA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2935 U"))]
    Pms2935U,
    /// `Pantone 294 C` — `#002F6C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 294 C"))]
    Pms294C,
    /// `Pantone 294 U` — `#385988`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 294 U"))]
    Pms294U,
    /// `Pantone 2945 C` — `#004C97`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2945 C"))]
    Pms2945C,
    /// `Pantone 2945 U` — `#29588C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2945 U"))]
    Pms2945U,
    /// `Pantone 295 C` — `#002855`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 295 C"))]
    Pms295C,
    /// `Pantone 295 U` — `#375172`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 295 U"))]
    Pms295U,
    /// `Pantone 2955 C` — `#003865`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2955 C"))]
    Pms2955C,
    /// `Pantone 2955 U` — `#395775`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2955 U"))]
    Pms2955U,
    /// `Pantone 296 C` — `#041C2C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 296 C"))]
    Pms296C,
    /// `Pantone 296 U` — `#435060`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 296 U"))]
    Pms296U,
    /// `Pantone 2965 C` — `#00263E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2965 C"))]
    Pms2965C,
    /// `Pantone 2965 U` — `#3D5265`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2965 U"))]
    Pms2965U,
    /// `Pantone 297 C` — `#71C5E8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 297 C"))]
    Pms297C,
    /// `Pantone 297 U` — `#7ECCEE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 297 U"))]
    Pms297U,
    /// `Pantone 2975 C` — `#99D6EA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2975 C"))]
    Pms2975C,
    /// `Pantone 2975 U` — `#8BD5EE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2975 U"))]
    Pms2975U,
    /// `Pantone 298 C` — `#41B6E6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 298 C"))]
    Pms298C,
    /// `Pantone 298 U` — `#4CB4E7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 298 U"))]
    Pms298U,
    /// `Pantone 2985 C` — `#5BC2E7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2985 C"))]
    Pms2985C,
    /// `Pantone 2985 U` — `#48B8E7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2985 U"))]
    Pms2985U,
    /// `Pantone 299 C` — `#00A3E0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 299 C"))]
    Pms299C,
    /// `Pantone 299 U` — `#1295D8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 299 U"))]
    Pms299U,
    /// `Pantone 2995 C` — `#00A9E0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2995 C"))]
    Pms2995C,
    /// `Pantone 2995 U` — `#0D9DDB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 2995 U"))]
    Pms2995U,
    /// `Pantone 300 C` — `#005EB8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 300 C"))]
    Pms300C,
    /// `Pantone 300 U` — `#006EB6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 300 U"))]
    Pms300U,
    /// `Pantone 3005 C` — `#0077C8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3005 C"))]
    Pms3005C,
    /// `Pantone 3005 U` — `#0078BF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3005 U"))]
    Pms3005U,
    /// `Pantone 301 C` — `#004B87`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 301 C"))]
    Pms301C,
    /// `Pantone 301 U` — `#28628E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 301 U"))]
    Pms301U,
    /// `Pantone 3015 C` — `#00629B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3015 C"))]
    Pms3015C,
    /// `Pantone 3015 U` — `#216A95`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3015 U"))]
    Pms3015U,
    /// `Pantone 302 C` — `#003B5C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 302 C"))]
    Pms302C,
    /// `Pantone 302 U` — `#375E77`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 302 U"))]
    Pms302U,
    /// `Pantone 3025 C` — `#004F71`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3025 C"))]
    Pms3025C,
    /// `Pantone 3025 U` — `#37617A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3025 U"))]
    Pms3025U,
    /// `Pantone 303 C` — `#002A3A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 303 C"))]
    Pms303C,
    /// `Pantone 303 U` — `#3F5666`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 303 U"))]
    Pms303U,
    /// `Pantone 3035 C` — `#003E51`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3035 C"))]
    Pms3035C,
    /// `Pantone 3035 U` — `#405866`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3035 U"))]
    Pms3035U,
    /// `Pantone 304 C` — `#9ADBE8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 304 C"))]
    Pms304C,
    /// `Pantone 304 U` — `#8EDDED`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 304 U"))]
    Pms304U,
    /// `Pantone 305 C` — `#59CBE8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 305 C"))]
    Pms305C,
    /// `Pantone 305 U` — `#53CAEC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 305 U"))]
    Pms305U,
    /// `Pantone 306 C` — `#00B5E2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 306 C"))]
    Pms306C,
    /// `Pantone 306 U` — `#00B4E4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 306 U"))]
    Pms306U,
    /// `Pantone 307 C` — `#006BA6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 307 C"))]
    Pms307C,
    /// `Pantone 307 U` — `#0074A2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 307 U"))]
    Pms307U,
    /// `Pantone 308 C` — `#00587C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 308 C"))]
    Pms308C,
    /// `Pantone 308 U` — `#266782`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 308 U"))]
    Pms308U,
    /// `Pantone 309 C` — `#003B49`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 309 C"))]
    Pms309C,
    /// `Pantone 309 U` — `#3F5864`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 309 U"))]
    Pms309U,
    /// `Pantone 310 C` — `#6AD1E3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 310 C"))]
    Pms310C,
    /// `Pantone 310 U` — `#5BD0E6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 310 U"))]
    Pms310U,
    /// `Pantone 3105 C` — `#68D2DF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3105 C"))]
    Pms3105C,
    /// `Pantone 3105 U` — `#4CCEDE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3105 U"))]
    Pms3105U,
    /// `Pantone 311 C` — `#05C3DE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 311 C"))]
    Pms311C,
    /// `Pantone 311 U` — `#01BEDB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 311 U"))]
    Pms311U,
    /// `Pantone 3115 C` — `#00C1D5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3115 C"))]
    Pms3115C,
    /// `Pantone 3115 U` — `#00BFD3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3115 U"))]
    Pms3115U,
    /// `Pantone 312 C` — `#00A9CE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 312 C"))]
    Pms312C,
    /// `Pantone 312 U` — `#00AFD0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 312 U"))]
    Pms312U,
    /// `Pantone 3125 C` — `#00AEC7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3125 C"))]
    Pms3125C,
    /// `Pantone 3125 U` — `#00ABC2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3125 U"))]
    Pms3125U,
    /// `Pantone 313 C` — `#0092BC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 313 C"))]
    Pms313C,
    /// `Pantone 313 U` — `#008DB1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 313 U"))]
    Pms313U,
    /// `Pantone 3135 C` — `#008EAA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3135 C"))]
    Pms3135C,
    /// `Pantone 3135 U` — `#008EA6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3135 U"))]
    Pms3135U,
    /// `Pantone 314 C` — `#007FA3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 314 C"))]
    Pms314C,
    /// `Pantone 314 U` — `#007D99`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 314 U"))]
    Pms314U,
    /// `Pantone 3145 C` — `#00778B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3145 C"))]
    Pms3145C,
    /// `Pantone 3145 U` — `#007788`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3145 U"))]
    Pms3145U,
    /// `Pantone 315 C` — `#00677F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 315 C"))]
    Pms315C,
    /// `Pantone 315 U` — `#206779`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 315 U"))]
    Pms315U,
    /// `Pantone 3155 C` — `#006272`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3155 C"))]
    Pms3155C,
    /// `Pantone 3155 U` — `#1B6875`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3155 U"))]
    Pms3155U,
    /// `Pantone 316 C` — `#004851`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 316 C"))]
    Pms316C,
    /// `Pantone 316 U` — `#3E5A62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 316 U"))]
    Pms316U,
    /// `Pantone 3165 C` — `#004F59`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3165 C"))]
    Pms3165C,
    /// `Pantone 3165 U` — `#2F5C66`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3165 U"))]
    Pms3165U,
    /// `Pantone 317 C` — `#B1E4E3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 317 C"))]
    Pms317C,
    /// `Pantone 317 U` — `#AEE9E8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 317 U"))]
    Pms317U,
    /// `Pantone 318 C` — `#88DBDF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 318 C"))]
    Pms318C,
    /// `Pantone 318 U` — `#6BD9DE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 318 U"))]
    Pms318U,
    /// `Pantone 319 C` — `#2DCCD3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 319 C"))]
    Pms319C,
    /// `Pantone 319 U` — `#25C9D0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 319 U"))]
    Pms319U,
    /// `Pantone 320 C` — `#009CA6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 320 C"))]
    Pms320C,
    /// `Pantone 320 U` — `#009DA5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 320 U"))]
    Pms320U,
    /// `Pantone 321 C` — `#008C95`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 321 C"))]
    Pms321C,
    /// `Pantone 321 U` — `#00838A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 321 U"))]
    Pms321U,
    /// `Pantone 322 C` — `#007377`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 322 C"))]
    Pms322C,
    /// `Pantone 322 U` — `#167378`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 322 U"))]
    Pms322U,
    /// `Pantone 323 C` — `#005F61`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 323 C"))]
    Pms323C,
    /// `Pantone 323 U` — `#2F6165`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 323 U"))]
    Pms323U,
    /// `Pantone 324 C` — `#9CDBD9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 324 C"))]
    Pms324C,
    /// `Pantone 324 U` — `#82D8D5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 324 U"))]
    Pms324U,
    /// `Pantone 3242 C` — `#71DBD4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3242 C"))]
    Pms3242C,
    /// `Pantone 3242 U` — `#5DDBD3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3242 U"))]
    Pms3242U,
    /// `Pantone 3245 C` — `#7CE0D3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3245 C"))]
    Pms3245C,
    /// `Pantone 3245 U` — `#4EDCCA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3245 U"))]
    Pms3245U,
    /// `Pantone 3248 C` — `#6DCDB8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3248 C"))]
    Pms3248C,
    /// `Pantone 3248 U` — `#6ACEB9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3248 U"))]
    Pms3248U,
    /// `Pantone 325 C` — `#64CCC9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 325 C"))]
    Pms325C,
    /// `Pantone 325 U` — `#40C1BB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 325 U"))]
    Pms325U,
    /// `Pantone 3252 C` — `#2AD2C9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3252 C"))]
    Pms3252C,
    /// `Pantone 3252 U` — `#00CDC2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3252 U"))]
    Pms3252U,
    /// `Pantone 3255 C` — `#2CD5C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3255 C"))]
    Pms3255C,
    /// `Pantone 3255 U` — `#00CFBB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3255 U"))]
    Pms3255U,
    /// `Pantone 3258 C` — `#49C5B1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3258 C"))]
    Pms3258C,
    /// `Pantone 3258 U` — `#48C1AC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3258 U"))]
    Pms3258U,
    /// `Pantone 326 C` — `#00B2A9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 326 C"))]
    Pms326C,
    /// `Pantone 326 U` — `#00AAA1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 326 U"))]
    Pms326U,
    /// `Pantone 3262 C` — `#00BFB3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3262 C"))]
    Pms3262C,
    /// `Pantone 3262 U` — `#00BAAE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3262 U"))]
    Pms3262U,
    /// `Pantone 3265 C` — `#00C7B1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3265 C"))]
    Pms3265C,
    /// `Pantone 3265 U` — `#00C0AA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3265 U"))]
    Pms3265U,
    /// `Pantone 3268 C` — `#00AB8E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3268 C"))]
    Pms3268C,
    /// `Pantone 3268 U` — `#28B29A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3268 U"))]
    Pms3268U,
    /// `Pantone 327 C` — `#008675`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 327 C"))]
    Pms327C,
    /// `Pantone 327 U` — `#008F85`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 327 U"))]
    Pms327U,
    /// `Pantone 3272 C` — `#00A499`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3272 C"))]
    Pms3272C,
    /// `Pantone 3272 U` — `#00A49A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3272 U"))]
    Pms3272U,
    /// `Pantone 3275 C` — `#00B398`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3275 C"))]
    Pms3275C,
    /// `Pantone 3275 U` — `#00AA93`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3275 U"))]
    Pms3275U,
    /// `Pantone 3278 C` — `#009B77`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3278 C"))]
    Pms3278C,
    /// `Pantone 3278 U` — `#009C82`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3278 U"))]
    Pms3278U,
    /// `Pantone 328 C` — `#007367`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 328 C"))]
    Pms328C,
    /// `Pantone 328 U` — `#137D79`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 328 U"))]
    Pms328U,
    /// `Pantone 3282 C` — `#008578`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3282 C"))]
    Pms3282C,
    /// `Pantone 3282 U` — `#008780`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3282 U"))]
    Pms3282U,
    /// `Pantone 3285 C` — `#009681`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3285 C"))]
    Pms3285C,
    /// `Pantone 3285 U` — `#009081`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3285 U"))]
    Pms3285U,
    /// `Pantone 3288 C` — `#008264`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3288 C"))]
    Pms3288C,
    /// `Pantone 3288 U` — `#208776`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3288 U"))]
    Pms3288U,
    /// `Pantone 329 C` — `#00685E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 329 C"))]
    Pms329C,
    /// `Pantone 329 U` — `#2E7572`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 329 U"))]
    Pms329U,
    /// `Pantone 3292 C` — `#00594F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3292 C"))]
    Pms3292C,
    /// `Pantone 3292 U` — `#346E6A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3292 U"))]
    Pms3292U,
    /// `Pantone 3295 C` — `#007864`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3295 C"))]
    Pms3295C,
    /// `Pantone 3295 U` — `#237E74`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3295 U"))]
    Pms3295U,
    /// `Pantone 3298 C` — `#006A52`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3298 C"))]
    Pms3298C,
    /// `Pantone 3298 U` — `#387469`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3298 U"))]
    Pms3298U,
    /// `Pantone 330 C` — `#00534C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 330 C"))]
    Pms330C,
    /// `Pantone 330 U` — `#446463`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 330 U"))]
    Pms330U,
    /// `Pantone 3302 C` — `#004C45`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3302 C"))]
    Pms3302C,
    /// `Pantone 3302 U` — `#41615F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3302 U"))]
    Pms3302U,
    /// `Pantone 3305 C` — `#004E42`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3305 C"))]
    Pms3305C,
    /// `Pantone 3305 U` — `#466964`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3305 U"))]
    Pms3305U,
    /// `Pantone 3308 C` — `#034638`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3308 C"))]
    Pms3308C,
    /// `Pantone 3308 U` — `#48615B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3308 U"))]
    Pms3308U,
    /// `Pantone 331 C` — `#A7E6D7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 331 C"))]
    Pms331C,
    /// `Pantone 331 U` — `#93E8D3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 331 U"))]
    Pms331U,
    /// `Pantone 332 C` — `#8CE2D0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 332 C"))]
    Pms332C,
    /// `Pantone 332 U` — `#70E2CB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 332 U"))]
    Pms332U,
    /// `Pantone 333 C` — `#3CDBC0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 333 C"))]
    Pms333C,
    /// `Pantone 333 U` — `#00D2B3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 333 U"))]
    Pms333U,
    /// `Pantone 334 C` — `#009775`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 334 C"))]
    Pms334C,
    /// `Pantone 334 U` — `#00957E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 334 U"))]
    Pms334U,
    /// `Pantone 335 C` — `#007B5F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 335 C"))]
    Pms335C,
    /// `Pantone 335 U` — `#247E6F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 335 U"))]
    Pms335U,
    /// `Pantone 336 C` — `#00664F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 336 C"))]
    Pms336C,
    /// `Pantone 336 U` — `#3A7267`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 336 U"))]
    Pms336U,
    /// `Pantone 337 C` — `#8FD6BD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 337 C"))]
    Pms337C,
    /// `Pantone 337 U` — `#82D5BA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 337 U"))]
    Pms337U,
    /// `Pantone 3375 C` — `#7AE1BF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3375 C"))]
    Pms3375C,
    /// `Pantone 3375 U` — `#5FE0B7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3375 U"))]
    Pms3375U,
    /// `Pantone 338 C` — `#6ECEB2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 338 C"))]
    Pms338C,
    /// `Pantone 338 U` — `#4CC1A1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 338 U"))]
    Pms338U,
    /// `Pantone 3385 C` — `#47D7AC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3385 C"))]
    Pms3385C,
    /// `Pantone 3385 U` — `#3AD4A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3385 U"))]
    Pms3385U,
    /// `Pantone 339 C` — `#00B388`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 339 C"))]
    Pms339C,
    /// `Pantone 339 U` — `#22AE8A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 339 U"))]
    Pms339U,
    /// `Pantone 3395 C` — `#00C389`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3395 C"))]
    Pms3395C,
    /// `Pantone 3395 U` — `#00C795`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3395 U"))]
    Pms3395U,
    /// `Pantone 340 C` — `#00965E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 340 C"))]
    Pms340C,
    /// `Pantone 340 U` — `#009E77`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 340 U"))]
    Pms340U,
    /// `Pantone 3405 C` — `#00AF66`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3405 C"))]
    Pms3405C,
    /// `Pantone 3405 U` — `#00B27A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3405 U"))]
    Pms3405U,
    /// `Pantone 341 C` — `#007A53`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 341 C"))]
    Pms341C,
    /// `Pantone 341 U` — `#347963`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 341 U"))]
    Pms341U,
    /// `Pantone 3415 C` — `#007749`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3415 C"))]
    Pms3415C,
    /// `Pantone 3415 U` — `#308167`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3415 U"))]
    Pms3415U,
    /// `Pantone 342 C` — `#006747`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 342 C"))]
    Pms342C,
    /// `Pantone 342 U` — `#407060`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 342 U"))]
    Pms342U,
    /// `Pantone 3425 C` — `#006341`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3425 C"))]
    Pms3425C,
    /// `Pantone 3425 U` — `#417764`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3425 U"))]
    Pms3425U,
    /// `Pantone 343 C` — `#115740`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 343 C"))]
    Pms343C,
    /// `Pantone 343 U` — `#48655B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 343 U"))]
    Pms343U,
    /// `Pantone 3435 C` — `#154734`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3435 C"))]
    Pms3435C,
    /// `Pantone 3435 U` — `#4B675B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3435 U"))]
    Pms3435U,
    /// `Pantone 344 C` — `#A0DAB3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 344 C"))]
    Pms344C,
    /// `Pantone 344 U` — `#8ED8AC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 344 U"))]
    Pms344U,
    /// `Pantone 345 C` — `#91D6AC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 345 C"))]
    Pms345C,
    /// `Pantone 345 U` — `#6ECD9C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 345 U"))]
    Pms345U,
    /// `Pantone 346 C` — `#71CC98`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 346 C"))]
    Pms346C,
    /// `Pantone 346 U` — `#4EBD88`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 346 U"))]
    Pms346U,
    /// `Pantone 347 C` — `#009A44`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 347 C"))]
    Pms347C,
    /// `Pantone 347 U` — `#169B62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 347 U"))]
    Pms347U,
    /// `Pantone 348 C` — `#00843D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 348 C"))]
    Pms348C,
    /// `Pantone 348 U` — `#34855B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 348 U"))]
    Pms348U,
    /// `Pantone 349 C` — `#046A38`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 349 C"))]
    Pms349C,
    /// `Pantone 349 U` — `#477258`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 349 U"))]
    Pms349U,
    /// `Pantone 350 C` — `#2C5234`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 350 C"))]
    Pms350C,
    /// `Pantone 350 U` — `#546758`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 350 U"))]
    Pms350U,
    /// `Pantone 351 C` — `#A2E4B8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 351 C"))]
    Pms351C,
    /// `Pantone 351 U` — `#93E6B4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 351 U"))]
    Pms351U,
    /// `Pantone 352 C` — `#8FE2B0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 352 C"))]
    Pms352C,
    /// `Pantone 352 U` — `#70E0A3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 352 U"))]
    Pms352U,
    /// `Pantone 353 C` — `#80E0A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 353 C"))]
    Pms353C,
    /// `Pantone 353 U` — `#50D691`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 353 U"))]
    Pms353U,
    /// `Pantone 354 C` — `#00B140`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 354 C"))]
    Pms354C,
    /// `Pantone 354 U` — `#00A95C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 354 U"))]
    Pms354U,
    /// `Pantone 355 C` — `#009639`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 355 C"))]
    Pms355C,
    /// `Pantone 355 U` — `#19975D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 355 U"))]
    Pms355U,
    /// `Pantone 356 C` — `#007A33`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 356 C"))]
    Pms356C,
    /// `Pantone 356 U` — `#397E58`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 356 U"))]
    Pms356U,
    /// `Pantone 357 C` — `#215732`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 357 C"))]
    Pms357C,
    /// `Pantone 357 U` — `#516E5A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 357 U"))]
    Pms357U,
    /// `Pantone 358 C` — `#ADDC91`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 358 C"))]
    Pms358C,
    /// `Pantone 358 U` — `#97D88A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 358 U"))]
    Pms358U,
    /// `Pantone 359 C` — `#A1D884`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 359 C"))]
    Pms359C,
    /// `Pantone 359 U` — `#7DCC77`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 359 U"))]
    Pms359U,
    /// `Pantone 360 C` — `#6CC24A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 360 C"))]
    Pms360C,
    /// `Pantone 360 U` — `#5DB860`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 360 U"))]
    Pms360U,
    /// `Pantone 361 C` — `#43B02A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 361 C"))]
    Pms361C,
    /// `Pantone 361 U` — `#4DAA50`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 361 U"))]
    Pms361U,
    /// `Pantone 362 C` — `#509E2F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 362 C"))]
    Pms362C,
    /// `Pantone 362 U` — `#56944F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 362 U"))]
    Pms362U,
    /// `Pantone 363 C` — `#4C8C2B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 363 C"))]
    Pms363C,
    /// `Pantone 363 U` — `#57854E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 363 U"))]
    Pms363U,
    /// `Pantone 364 C` — `#4A7729`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 364 C"))]
    Pms364C,
    /// `Pantone 364 U` — `#5B794E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 364 U"))]
    Pms364U,
    /// `Pantone 365 C` — `#C2E189`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 365 C"))]
    Pms365C,
    /// `Pantone 365 U` — `#C2E692`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 365 U"))]
    Pms365U,
    /// `Pantone 366 C` — `#B7DD79`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 366 C"))]
    Pms366C,
    /// `Pantone 366 U` — `#A8DC7D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 366 U"))]
    Pms366U,
    /// `Pantone 367 C` — `#A4D65E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 367 C"))]
    Pms367C,
    /// `Pantone 367 U` — `#8FD16A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 367 U"))]
    Pms367U,
    /// `Pantone 368 C` — `#78BE20`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 368 C"))]
    Pms368C,
    /// `Pantone 368 U` — `#67B346`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 368 U"))]
    Pms368U,
    /// `Pantone 369 C` — `#64A70B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 369 C"))]
    Pms369C,
    /// `Pantone 369 U` — `#62A044`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 369 U"))]
    Pms369U,
    /// `Pantone 370 C` — `#658D1B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 370 C"))]
    Pms370C,
    /// `Pantone 370 U` — `#698A48`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 370 U"))]
    Pms370U,
    /// `Pantone 371 C` — `#546223`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 371 C"))]
    Pms371C,
    /// `Pantone 371 U` — `#68724D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 371 U"))]
    Pms371U,
    /// `Pantone 372 C` — `#D4EB8E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 372 C"))]
    Pms372C,
    /// `Pantone 372 U` — `#D1F08F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 372 U"))]
    Pms372U,
    /// `Pantone 373 C` — `#CDEA80`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 373 C"))]
    Pms373C,
    /// `Pantone 373 U` — `#B9E972`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 373 U"))]
    Pms373U,
    /// `Pantone 374 C` — `#C5E86C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 374 C"))]
    Pms374C,
    /// `Pantone 374 U` — `#A6E35F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 374 U"))]
    Pms374U,
    /// `Pantone 375 C` — `#97D700`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 375 C"))]
    Pms375C,
    /// `Pantone 375 U` — `#73C92D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 375 U"))]
    Pms375U,
    /// `Pantone 376 C` — `#84BD00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 376 C"))]
    Pms376C,
    /// `Pantone 376 U` — `#70AB37`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 376 U"))]
    Pms376U,
    /// `Pantone 377 C` — `#7A9A01`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 377 C"))]
    Pms377C,
    /// `Pantone 377 U` — `#728F41`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 377 U"))]
    Pms377U,
    /// `Pantone 378 C` — `#59621D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 378 C"))]
    Pms378C,
    /// `Pantone 378 U` — `#747B4D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 378 U"))]
    Pms378U,
    /// `Pantone 379 C` — `#E2E868`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 379 C"))]
    Pms379C,
    /// `Pantone 379 U` — `#E7EE7B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 379 U"))]
    Pms379U,
    /// `Pantone 380 C` — `#DBE442`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 380 C"))]
    Pms380C,
    /// `Pantone 380 U` — `#D4E458`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 380 U"))]
    Pms380U,
    /// `Pantone 381 C` — `#CEDC00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 381 C"))]
    Pms381C,
    /// `Pantone 381 U` — `#BAD739`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 381 U"))]
    Pms381U,
    /// `Pantone 382 C` — `#C4D600`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 382 C"))]
    Pms382C,
    /// `Pantone 382 U` — `#99C221`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 382 U"))]
    Pms382U,
    /// `Pantone 383 C` — `#A8AD00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 383 C"))]
    Pms383C,
    /// `Pantone 383 U` — `#8E9A36`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 383 U"))]
    Pms383U,
    /// `Pantone 384 C` — `#949300`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 384 C"))]
    Pms384C,
    /// `Pantone 384 U` — `#868B3C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 384 U"))]
    Pms384U,
    /// `Pantone 385 C` — `#787121`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 385 C"))]
    Pms385C,
    /// `Pantone 385 U` — `#7E7B4B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 385 U"))]
    Pms385U,
    /// `Pantone 386 C` — `#E9EC6B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 386 C"))]
    Pms386C,
    /// `Pantone 386 U` — `#EFF178`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 386 U"))]
    Pms386U,
    /// `Pantone 387 C` — `#E3E935`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 387 C"))]
    Pms387C,
    /// `Pantone 387 U` — `#E3ED55`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 387 U"))]
    Pms387U,
    /// `Pantone 388 C` — `#E0E721`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 388 C"))]
    Pms388C,
    /// `Pantone 388 U` — `#D5E739`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 388 U"))]
    Pms388U,
    /// `Pantone 389 C` — `#D0DF00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 389 C"))]
    Pms389C,
    /// `Pantone 389 U` — `#BDDC04`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 389 U"))]
    Pms389U,
    /// `Pantone 390 C` — `#B5BD00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 390 C"))]
    Pms390C,
    /// `Pantone 390 U` — `#97A926`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 390 U"))]
    Pms390U,
    /// `Pantone 391 C` — `#9A9500`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 391 C"))]
    Pms391C,
    /// `Pantone 391 U` — `#888A3B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 391 U"))]
    Pms391U,
    /// `Pantone 392 C` — `#827A04`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 392 C"))]
    Pms392C,
    /// `Pantone 392 U` — `#7E7D41`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 392 U"))]
    Pms392U,
    /// `Pantone 393 C` — `#F0EC74`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 393 C"))]
    Pms393C,
    /// `Pantone 393 U` — `#F8F283`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 393 U"))]
    Pms393U,
    /// `Pantone 3935 C` — `#F3EA5D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3935 C"))]
    Pms3935C,
    /// `Pantone 3935 U` — `#FCF389`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3935 U"))]
    Pms3935U,
    /// `Pantone 394 C` — `#EDE939`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 394 C"))]
    Pms394C,
    /// `Pantone 394 U` — `#F2EE56`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 394 U"))]
    Pms394U,
    /// `Pantone 3945 C` — `#F3E500`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3945 C"))]
    Pms3945C,
    /// `Pantone 3945 U` — `#FCF05F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3945 U"))]
    Pms3945U,
    /// `Pantone 395 C` — `#ECE81A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 395 C"))]
    Pms395C,
    /// `Pantone 395 U` — `#EDEC3F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 395 U"))]
    Pms395U,
    /// `Pantone 3955 C` — `#EFDF00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3955 C"))]
    Pms3955C,
    /// `Pantone 3955 U` — `#FAEB37`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3955 U"))]
    Pms3955U,
    /// `Pantone 396 C` — `#E1E000`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 396 C"))]
    Pms396C,
    /// `Pantone 396 U` — `#DBE200`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 396 U"))]
    Pms396U,
    /// `Pantone 3965 C` — `#EEDC00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3965 C"))]
    Pms3965C,
    /// `Pantone 3965 U` — `#F8E815`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3965 U"))]
    Pms3965U,
    /// `Pantone 397 C` — `#BFB800`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 397 C"))]
    Pms397C,
    /// `Pantone 397 U` — `#A8AA31`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 397 U"))]
    Pms397U,
    /// `Pantone 3975 C` — `#BBA600`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3975 C"))]
    Pms3975C,
    /// `Pantone 3975 U` — `#B6A72D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3975 U"))]
    Pms3975U,
    /// `Pantone 398 C` — `#ADA400`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 398 C"))]
    Pms398C,
    /// `Pantone 398 U` — `#959539`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 398 U"))]
    Pms398U,
    /// `Pantone 3985 C` — `#9A8700`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3985 C"))]
    Pms3985C,
    /// `Pantone 3985 U` — `#9C903F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3985 U"))]
    Pms3985U,
    /// `Pantone 399 C` — `#A09200`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 399 C"))]
    Pms399C,
    /// `Pantone 399 U` — `#8A8537`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 399 U"))]
    Pms399U,
    /// `Pantone 3995 C` — `#685C20`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3995 C"))]
    Pms3995C,
    /// `Pantone 3995 U` — `#837B4A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 3995 U"))]
    Pms3995U,
    /// `Pantone 400 C` — `#C4BFB6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 400 C"))]
    Pms400C,
    /// `Pantone 400 U` — `#C7C1BA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 400 U"))]
    Pms400U,
    /// `Pantone 401 C` — `#AFA9A0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 401 C"))]
    Pms401C,
    /// `Pantone 401 U` — `#B4AEA7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 401 U"))]
    Pms401U,
    /// `Pantone 402 C` — `#9D968D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 402 C"))]
    Pms402C,
    /// `Pantone 402 U` — `#A39E98`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 402 U"))]
    Pms402U,
    /// `Pantone 403 C` — `#8C857B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 403 C"))]
    Pms403C,
    /// `Pantone 403 U` — `#928D88`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 403 U"))]
    Pms403U,
    /// `Pantone 404 C` — `#776E64`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 404 C"))]
    Pms404C,
    /// `Pantone 404 U` — `#8B8681`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 404 U"))]
    Pms404U,
    /// `Pantone 405 C` — `#696158`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 405 C"))]
    Pms405C,
    /// `Pantone 405 U` — `#77726D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 405 U"))]
    Pms405U,
    /// `Pantone 406 C` — `#C4BCB7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 406 C"))]
    Pms406C,
    /// `Pantone 406 U` — `#C7BEBA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 406 U"))]
    Pms406U,
    /// `Pantone 407 C` — `#B2A8A2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 407 C"))]
    Pms407C,
    /// `Pantone 407 U` — `#ABA2A0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 407 U"))]
    Pms407U,
    /// `Pantone 408 C` — `#978C87`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 408 C"))]
    Pms408C,
    /// `Pantone 408 U` — `#9D9594`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 408 U"))]
    Pms408U,
    /// `Pantone 409 C` — `#857874`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 409 C"))]
    Pms409C,
    /// `Pantone 409 U` — `#948B8A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 409 U"))]
    Pms409U,
    /// `Pantone 410 C` — `#746661`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 410 C"))]
    Pms410C,
    /// `Pantone 410 U` — `#8A8280`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 410 U"))]
    Pms410U,
    /// `Pantone 411 C` — `#5E514D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 411 C"))]
    Pms411C,
    /// `Pantone 411 U` — `#7A726F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 411 U"))]
    Pms411U,
    /// `Pantone 412 C` — `#382F2D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 412 C"))]
    Pms412C,
    /// `Pantone 412 U` — `#645C59`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 412 U"))]
    Pms412U,
    /// `Pantone 413 C` — `#BABBB1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 413 C"))]
    Pms413C,
    /// `Pantone 413 U` — `#BCBDB4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 413 U"))]
    Pms413U,
    /// `Pantone 414 C` — `#A8A99E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 414 C"))]
    Pms414C,
    /// `Pantone 414 U` — `#ABACA5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 414 U"))]
    Pms414U,
    /// `Pantone 415 C` — `#919388`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 415 C"))]
    Pms415C,
    /// `Pantone 415 U` — `#9D9F99`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 415 U"))]
    Pms415U,
    /// `Pantone 416 C` — `#7E7F74`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 416 C"))]
    Pms416C,
    /// `Pantone 416 U` — `#91928C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 416 U"))]
    Pms416U,
    /// `Pantone 417 C` — `#65665C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 417 C"))]
    Pms417C,
    /// `Pantone 417 U` — `#82847F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 417 U"))]
    Pms417U,
    /// `Pantone 418 C` — `#51534A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 418 C"))]
    Pms418C,
    /// `Pantone 418 U` — `#7A7B75`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 418 U"))]
    Pms418U,
    /// `Pantone 419 C` — `#212322`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 419 C"))]
    Pms419C,
    /// `Pantone 419 U` — `#595955`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 419 U"))]
    Pms419U,
    /// `Pantone 420 C` — `#C7C9C7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 420 C"))]
    Pms420C,
    /// `Pantone 420 U` — `#BDBEBD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 420 U"))]
    Pms420U,
    /// `Pantone 421 C` — `#B2B4B2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 421 C"))]
    Pms421C,
    /// `Pantone 421 U` — `#B3B5B5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 421 U"))]
    Pms421U,
    /// `Pantone 422 C` — `#9EA2A2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 422 C"))]
    Pms422C,
    /// `Pantone 422 U` — `#9EA1A2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 422 U"))]
    Pms422U,
    /// `Pantone 423 C` — `#898D8D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 423 C"))]
    Pms423C,
    /// `Pantone 423 U` — `#909496`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 423 U"))]
    Pms423U,
    /// `Pantone 424 C` — `#707372`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 424 C"))]
    Pms424C,
    /// `Pantone 424 U` — `#88898A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 424 U"))]
    Pms424U,
    /// `Pantone 425 C` — `#54585A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 425 C"))]
    Pms425C,
    /// `Pantone 425 U` — `#7C7E7F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 425 U"))]
    Pms425U,
    /// `Pantone 426 C` — `#25282A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 426 C"))]
    Pms426C,
    /// `Pantone 426 U` — `#686869`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 426 U"))]
    Pms426U,
    /// `Pantone 427 C` — `#D0D3D4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 427 C"))]
    Pms427C,
    /// `Pantone 427 U` — `#C2C7CA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 427 U"))]
    Pms427U,
    /// `Pantone 428 C` — `#C1C6C8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 428 C"))]
    Pms428C,
    /// `Pantone 428 U` — `#ACB3B8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 428 U"))]
    Pms428U,
    /// `Pantone 429 C` — `#A2AAAD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 429 C"))]
    Pms429C,
    /// `Pantone 429 U` — `#91999F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 429 U"))]
    Pms429U,
    /// `Pantone 430 C` — `#7C878E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 430 C"))]
    Pms430C,
    /// `Pantone 430 U` — `#808991`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 430 U"))]
    Pms430U,
    /// `Pantone 431 C` — `#5B6770`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 431 C"))]
    Pms431C,
    /// `Pantone 431 U` — `#737B82`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 431 U"))]
    Pms431U,
    /// `Pantone 432 C` — `#333F48`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 432 C"))]
    Pms432C,
    /// `Pantone 432 U` — `#676C73`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 432 U"))]
    Pms432U,
    /// `Pantone 433 C` — `#1D252D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 433 C"))]
    Pms433C,
    /// `Pantone 433 U` — `#5B5D62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 433 U"))]
    Pms433U,
    /// `Pantone 434 C` — `#D0C4C5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 434 C"))]
    Pms434C,
    /// `Pantone 434 U` — `#D6CACB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 434 U"))]
    Pms434U,
    /// `Pantone 435 C` — `#C1B2B6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 435 C"))]
    Pms435C,
    /// `Pantone 435 U` — `#B7A9AD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 435 U"))]
    Pms435U,
    /// `Pantone 436 C` — `#AB989D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 436 C"))]
    Pms436C,
    /// `Pantone 436 U` — `#A99B9F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 436 U"))]
    Pms436U,
    /// `Pantone 437 C` — `#7B6469`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 437 C"))]
    Pms437C,
    /// `Pantone 437 U` — `#8D7F83`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 437 U"))]
    Pms437U,
    /// `Pantone 438 C` — `#584446`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 438 C"))]
    Pms438C,
    /// `Pantone 438 U` — `#817275`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 438 U"))]
    Pms438U,
    /// `Pantone 439 C` — `#453536`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 439 C"))]
    Pms439C,
    /// `Pantone 439 U` — `#736869`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 439 U"))]
    Pms439U,
    /// `Pantone 440 C` — `#382E2C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 440 C"))]
    Pms440C,
    /// `Pantone 440 U` — `#69615F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 440 U"))]
    Pms440U,
    /// `Pantone 441 C` — `#BEC6C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 441 C"))]
    Pms441C,
    /// `Pantone 441 U` — `#B9C2C1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 441 U"))]
    Pms441U,
    /// `Pantone 442 C` — `#A2ACAB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 442 C"))]
    Pms442C,
    /// `Pantone 442 U` — `#ACB6B6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 442 U"))]
    Pms442U,
    /// `Pantone 443 C` — `#919D9D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 443 C"))]
    Pms443C,
    /// `Pantone 443 U` — `#8F9A9B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 443 U"))]
    Pms443U,
    /// `Pantone 444 C` — `#717C7D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 444 C"))]
    Pms444C,
    /// `Pantone 444 U` — `#7E8689`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 444 U"))]
    Pms444U,
    /// `Pantone 445 C` — `#505759`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 445 C"))]
    Pms445C,
    /// `Pantone 445 U` — `#6B7173`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 445 U"))]
    Pms445U,
    /// `Pantone 446 C` — `#3F4444`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 446 C"))]
    Pms446C,
    /// `Pantone 446 U` — `#636968`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 446 U"))]
    Pms446U,
    /// `Pantone 447 C` — `#373A36`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 447 C"))]
    Pms447C,
    /// `Pantone 447 U` — `#5F625F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 447 U"))]
    Pms447U,
    /// `Pantone 448 C` — `#4A412A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 448 C"))]
    Pms448C,
    /// `Pantone 448 U` — `#66614E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 448 U"))]
    Pms448U,
    /// `Pantone 4485 C` — `#614F25`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4485 C"))]
    Pms4485C,
    /// `Pantone 4485 U` — `#7B6F4B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4485 U"))]
    Pms4485U,
    /// `Pantone 449 C` — `#524727`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 449 C"))]
    Pms449C,
    /// `Pantone 449 U` — `#716B54`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 449 U"))]
    Pms449U,
    /// `Pantone 4495 C` — `#8C7732`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4495 C"))]
    Pms4495C,
    /// `Pantone 4495 U` — `#90845F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4495 U"))]
    Pms4495U,
    /// `Pantone 450 C` — `#594A25`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 450 C"))]
    Pms450C,
    /// `Pantone 450 U` — `#7E785A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 450 U"))]
    Pms450U,
    /// `Pantone 4505 C` — `#998542`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4505 C"))]
    Pms4505C,
    /// `Pantone 4505 U` — `#A39771`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4505 U"))]
    Pms4505U,
    /// `Pantone 451 C` — `#9B945F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 451 C"))]
    Pms451C,
    /// `Pantone 451 U` — `#929172`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 451 U"))]
    Pms451U,
    /// `Pantone 4515 C` — `#B3A369`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4515 C"))]
    Pms4515C,
    /// `Pantone 4515 U` — `#B1A57E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4515 U"))]
    Pms4515U,
    /// `Pantone 452 C` — `#B0AA7E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 452 C"))]
    Pms452C,
    /// `Pantone 452 U` — `#A8A687`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 452 U"))]
    Pms452U,
    /// `Pantone 4525 C` — `#C5B783`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4525 C"))]
    Pms4525C,
    /// `Pantone 4525 U` — `#C1B58C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4525 U"))]
    Pms4525U,
    /// `Pantone 453 C` — `#BFBB98`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 453 C"))]
    Pms453C,
    /// `Pantone 453 U` — `#BCB999`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 453 U"))]
    Pms453U,
    /// `Pantone 4535 C` — `#CFC493`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4535 C"))]
    Pms4535C,
    /// `Pantone 4535 U` — `#CABE96`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4535 U"))]
    Pms4535U,
    /// `Pantone 454 C` — `#CAC7A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 454 C"))]
    Pms454C,
    /// `Pantone 454 U` — `#CAC8AC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 454 U"))]
    Pms454U,
    /// `Pantone 4545 C` — `#D5CB9F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4545 C"))]
    Pms4545C,
    /// `Pantone 4545 U` — `#D6CCA6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4545 U"))]
    Pms4545U,
    /// `Pantone 455 C` — `#695B24`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 455 C"))]
    Pms455C,
    /// `Pantone 455 U` — `#776F48`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 455 U"))]
    Pms455U,
    /// `Pantone 456 C` — `#A28E2A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 456 C"))]
    Pms456C,
    /// `Pantone 456 U` — `#908145`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 456 U"))]
    Pms456U,
    /// `Pantone 457 C` — `#B89D18`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 457 C"))]
    Pms457C,
    /// `Pantone 457 U` — `#AD9844`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 457 U"))]
    Pms457U,
    /// `Pantone 458 C` — `#D9C756`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 458 C"))]
    Pms458C,
    /// `Pantone 458 U` — `#CEB964`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 458 U"))]
    Pms458U,
    /// `Pantone 459 C` — `#DECD63`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 459 C"))]
    Pms459C,
    /// `Pantone 459 U` — `#DDCB77`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 459 U"))]
    Pms459U,
    /// `Pantone 460 C` — `#E4D77E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 460 C"))]
    Pms460C,
    /// `Pantone 460 U` — `#E8D98B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 460 U"))]
    Pms460U,
    /// `Pantone 461 C` — `#E9DF97`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 461 C"))]
    Pms461C,
    /// `Pantone 461 U` — `#EEE29F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 461 U"))]
    Pms461U,
    /// `Pantone 462 C` — `#5C462B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 462 C"))]
    Pms462C,
    /// `Pantone 462 U` — `#70624D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 462 U"))]
    Pms462U,
    /// `Pantone 4625 C` — `#4F2C1D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4625 C"))]
    Pms4625C,
    /// `Pantone 4625 U` — `#715845`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4625 U"))]
    Pms4625U,
    /// `Pantone 463 C` — `#744F28`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 463 C"))]
    Pms463C,
    /// `Pantone 463 U` — `#856F51`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 463 U"))]
    Pms463U,
    /// `Pantone 4635 C` — `#946037`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4635 C"))]
    Pms4635C,
    /// `Pantone 4635 U` — `#92715E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4635 U"))]
    Pms4635U,
    /// `Pantone 464 C` — `#8B5B29`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 464 C"))]
    Pms464C,
    /// `Pantone 464 U` — `#9B7C56`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 464 U"))]
    Pms464U,
    /// `Pantone 4645 C` — `#AD7C59`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4645 C"))]
    Pms4645C,
    /// `Pantone 4645 U` — `#A07D6B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4645 U"))]
    Pms4645U,
    /// `Pantone 465 C` — `#B9975B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 465 C"))]
    Pms465C,
    /// `Pantone 465 U` — `#B0946F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 465 U"))]
    Pms465U,
    /// `Pantone 4655 C` — `#BF9474`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4655 C"))]
    Pms4655C,
    /// `Pantone 4655 U` — `#B28E7B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4655 U"))]
    Pms4655U,
    /// `Pantone 466 C` — `#C6AA76`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 466 C"))]
    Pms466C,
    /// `Pantone 466 U` — `#C1A67F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 466 U"))]
    Pms466U,
    /// `Pantone 4665 C` — `#CDA788`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4665 C"))]
    Pms4665C,
    /// `Pantone 4665 U` — `#CAA58E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4665 U"))]
    Pms4665U,
    /// `Pantone 467 C` — `#D3BC8D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 467 C"))]
    Pms467C,
    /// `Pantone 467 U` — `#D2BA92`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 467 U"))]
    Pms467U,
    /// `Pantone 4675 C` — `#DCBFA6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4675 C"))]
    Pms4675C,
    /// `Pantone 4675 U` — `#DAB9A1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4675 U"))]
    Pms4675U,
    /// `Pantone 468 C` — `#DDCBA4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 468 C"))]
    Pms468C,
    /// `Pantone 468 U` — `#E0CEAA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 468 U"))]
    Pms468U,
    /// `Pantone 4685 C` — `#E0C6AD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4685 C"))]
    Pms4685C,
    /// `Pantone 4685 U` — `#E6CAB4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4685 U"))]
    Pms4685U,
    /// `Pantone 469 C` — `#693F23`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 469 C"))]
    Pms469C,
    /// `Pantone 469 U` — `#7B604B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 469 U"))]
    Pms469U,
    /// `Pantone 4695 C` — `#5B3427`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4695 C"))]
    Pms4695C,
    /// `Pantone 4695 U` — `#775C50`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4695 U"))]
    Pms4695U,
    /// `Pantone 470 C` — `#A45A2A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 470 C"))]
    Pms470C,
    /// `Pantone 470 U` — `#A37052`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 470 U"))]
    Pms470U,
    /// `Pantone 4705 C` — `#7C4D3A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4705 C"))]
    Pms4705C,
    /// `Pantone 4705 U` — `#8B6F66`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4705 U"))]
    Pms4705U,
    /// `Pantone 471 C` — `#B86125`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 471 C"))]
    Pms471C,
    /// `Pantone 471 U` — `#BE7B54`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 471 U"))]
    Pms471U,
    /// `Pantone 4715 C` — `#956C58`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4715 C"))]
    Pms4715C,
    /// `Pantone 4715 U` — `#997D73`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4715 U"))]
    Pms4715U,
    /// `Pantone 472 C` — `#E59E6D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 472 C"))]
    Pms472C,
    /// `Pantone 472 U` — `#DB9271`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 472 U"))]
    Pms472U,
    /// `Pantone 4725 C` — `#AE8A79`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4725 C"))]
    Pms4725C,
    /// `Pantone 4725 U` — `#AE9188`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4725 U"))]
    Pms4725U,
    /// `Pantone 473 C` — `#F0BF9B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 473 C"))]
    Pms473C,
    /// `Pantone 473 U` — `#EEAB88`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 473 U"))]
    Pms473U,
    /// `Pantone 4735 C` — `#C0A392`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4735 C"))]
    Pms4735C,
    /// `Pantone 4735 U` — `#BDA195`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4735 U"))]
    Pms4735U,
    /// `Pantone 474 C` — `#F1C6A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 474 C"))]
    Pms474C,
    /// `Pantone 474 U` — `#F6BD9D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 474 U"))]
    Pms474U,
    /// `Pantone 4745 C` — `#CDB5A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4745 C"))]
    Pms4745C,
    /// `Pantone 4745 U` — `#CEB4A9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4745 U"))]
    Pms4745U,
    /// `Pantone 475 C` — `#F3CFB3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 475 C"))]
    Pms475C,
    /// `Pantone 475 U` — `#FACCAD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 475 U"))]
    Pms475U,
    /// `Pantone 4755 C` — `#D7C4B7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4755 C"))]
    Pms4755C,
    /// `Pantone 4755 U` — `#DCC6BB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4755 U"))]
    Pms4755U,
    /// `Pantone 476 C` — `#4E3629`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 476 C"))]
    Pms476C,
    /// `Pantone 476 U` — `#6E5E52`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 476 U"))]
    Pms476U,
    /// `Pantone 477 C` — `#623B2A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 477 C"))]
    Pms477C,
    /// `Pantone 477 U` — `#7D6556`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 477 U"))]
    Pms477U,
    /// `Pantone 478 C` — `#703F2A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 478 C"))]
    Pms478C,
    /// `Pantone 478 U` — `#8E6E5C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 478 U"))]
    Pms478U,
    /// `Pantone 479 C` — `#AA8066`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 479 C"))]
    Pms479C,
    /// `Pantone 479 U` — `#AC8C7E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 479 U"))]
    Pms479U,
    /// `Pantone 480 C` — `#C6A992`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 480 C"))]
    Pms480C,
    /// `Pantone 480 U` — `#C3A492`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 480 U"))]
    Pms480U,
    /// `Pantone 481 C` — `#D3BBA8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 481 C"))]
    Pms481C,
    /// `Pantone 481 U` — `#D3B7A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 481 U"))]
    Pms481U,
    /// `Pantone 482 C` — `#DBC8B6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 482 C"))]
    Pms482C,
    /// `Pantone 482 U` — `#DFC8B7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 482 U"))]
    Pms482U,
    /// `Pantone 483 C` — `#653024`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 483 C"))]
    Pms483C,
    /// `Pantone 483 U` — `#7D5B4F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 483 U"))]
    Pms483U,
    /// `Pantone 484 C` — `#9A3324`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 484 C"))]
    Pms484C,
    /// `Pantone 484 U` — `#A05B4E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 484 U"))]
    Pms484U,
    /// `Pantone 485 C` — `#DA291C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 485 C"))]
    Pms485C,
    /// `Pantone 485 U` — `#E45D50`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 485 U"))]
    Pms485U,
    /// `Pantone 486 C` — `#E8927C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 486 C"))]
    Pms486C,
    /// `Pantone 486 U` — `#F08D80`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 486 U"))]
    Pms486U,
    /// `Pantone 487 C` — `#EAA794`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 487 C"))]
    Pms487C,
    /// `Pantone 487 U` — `#F3A293`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 487 U"))]
    Pms487U,
    /// `Pantone 488 C` — `#ECBAA8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 488 C"))]
    Pms488C,
    /// `Pantone 488 U` — `#F4B9A9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 488 U"))]
    Pms488U,
    /// `Pantone 489 C` — `#ECC3B2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 489 C"))]
    Pms489C,
    /// `Pantone 489 U` — `#F5C7B8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 489 U"))]
    Pms489U,
    /// `Pantone 490 C` — `#5D2A2C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 490 C"))]
    Pms490C,
    /// `Pantone 490 U` — `#775554`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 490 U"))]
    Pms490U,
    /// `Pantone 491 C` — `#7F3035`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 491 C"))]
    Pms491C,
    /// `Pantone 491 U` — `#8E595A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 491 U"))]
    Pms491U,
    /// `Pantone 492 C` — `#8F3237`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 492 C"))]
    Pms492C,
    /// `Pantone 492 U` — `#9D5D60`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 492 U"))]
    Pms492U,
    /// `Pantone 493 C` — `#DC8699`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 493 C"))]
    Pms493C,
    /// `Pantone 493 U` — `#C77C89`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 493 U"))]
    Pms493U,
    /// `Pantone 494 C` — `#E9A2B2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 494 C"))]
    Pms494C,
    /// `Pantone 494 U` — `#E59AA8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 494 U"))]
    Pms494U,
    /// `Pantone 495 C` — `#F1BDC8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 495 C"))]
    Pms495C,
    /// `Pantone 495 U` — `#F1AFBC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 495 U"))]
    Pms495U,
    /// `Pantone 496 C` — `#F2C6CF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 496 C"))]
    Pms496C,
    /// `Pantone 496 U` — `#F9C2CC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 496 U"))]
    Pms496U,
    /// `Pantone 497 C` — `#512F2E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 497 C"))]
    Pms497C,
    /// `Pantone 497 U` — `#715956`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 497 U"))]
    Pms497U,
    /// `Pantone 4975 C` — `#3F2021`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4975 C"))]
    Pms4975C,
    /// `Pantone 4975 U` — `#745654`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4975 U"))]
    Pms4975U,
    /// `Pantone 498 C` — `#6A3735`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 498 C"))]
    Pms498C,
    /// `Pantone 498 U` — `#805D5B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 498 U"))]
    Pms498U,
    /// `Pantone 4985 C` — `#874B52`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4985 C"))]
    Pms4985C,
    /// `Pantone 4985 U` — `#8F6A6F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4985 U"))]
    Pms4985U,
    /// `Pantone 499 C` — `#7A3E3A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 499 C"))]
    Pms499C,
    /// `Pantone 499 U` — `#8D615F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 499 U"))]
    Pms499U,
    /// `Pantone 4995 C` — `#9C6169`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4995 C"))]
    Pms4995C,
    /// `Pantone 4995 U` — `#9F787F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 4995 U"))]
    Pms4995U,
    /// `Pantone 500 C` — `#C6858F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 500 C"))]
    Pms500C,
    /// `Pantone 500 U` — `#BD898F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 500 U"))]
    Pms500U,
    /// `Pantone 5005 C` — `#B07C83`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5005 C"))]
    Pms5005C,
    /// `Pantone 5005 U` — `#B28A90`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5005 U"))]
    Pms5005U,
    /// `Pantone 501 C` — `#DAA5AD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 501 C"))]
    Pms501C,
    /// `Pantone 501 U` — `#D8A5AC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 501 U"))]
    Pms501U,
    /// `Pantone 5015 C` — `#CCA1A6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5015 C"))]
    Pms5015C,
    /// `Pantone 5015 U` — `#CAA1A6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5015 U"))]
    Pms5015U,
    /// `Pantone 502 C` — `#E5BAC1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 502 C"))]
    Pms502C,
    /// `Pantone 502 U` — `#E6B7BD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 502 U"))]
    Pms502U,
    /// `Pantone 5025 C` — `#DBB7BB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5025 C"))]
    Pms5025C,
    /// `Pantone 5025 U` — `#DCB6BA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5025 U"))]
    Pms5025U,
    /// `Pantone 503 C` — `#E9C4C7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 503 C"))]
    Pms503C,
    /// `Pantone 503 U` — `#F2CDCF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 503 U"))]
    Pms503U,
    /// `Pantone 5035 C` — `#DFC2C3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5035 C"))]
    Pms5035C,
    /// `Pantone 5035 U` — `#E6C6C7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5035 U"))]
    Pms5035U,
    /// `Pantone 504 C` — `#572932`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 504 C"))]
    Pms504C,
    /// `Pantone 504 U` — `#7F5D62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 504 U"))]
    Pms504U,
    /// `Pantone 505 C` — `#6F2C3F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 505 C"))]
    Pms505C,
    /// `Pantone 505 U` — `#8A5C66`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 505 U"))]
    Pms505U,
    /// `Pantone 506 C` — `#84344E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 506 C"))]
    Pms506C,
    /// `Pantone 506 U` — `#9A6372`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 506 U"))]
    Pms506U,
    /// `Pantone 507 C` — `#D592AA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 507 C"))]
    Pms507C,
    /// `Pantone 507 U` — `#CF8FA4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 507 U"))]
    Pms507U,
    /// `Pantone 508 C` — `#E4A9BB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 508 C"))]
    Pms508C,
    /// `Pantone 508 U` — `#DF9CAE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 508 U"))]
    Pms508U,
    /// `Pantone 509 C` — `#E8B3C3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 509 C"))]
    Pms509C,
    /// `Pantone 509 U` — `#EAAEBE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 509 U"))]
    Pms509U,
    /// `Pantone 510 C` — `#EBBECB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 510 C"))]
    Pms510C,
    /// `Pantone 510 U` — `#F2BFCB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 510 U"))]
    Pms510U,
    /// `Pantone 511 C` — `#612C51`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 511 C"))]
    Pms511C,
    /// `Pantone 511 U` — `#745568`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 511 U"))]
    Pms511U,
    /// `Pantone 5115 C` — `#512A44`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5115 C"))]
    Pms5115C,
    /// `Pantone 5115 U` — `#6F5564`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5115 U"))]
    Pms5115U,
    /// `Pantone 512 C` — `#833177`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 512 C"))]
    Pms512C,
    /// `Pantone 512 U` — `#8A587F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 512 U"))]
    Pms512U,
    /// `Pantone 5125 C` — `#693C5E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5125 C"))]
    Pms5125C,
    /// `Pantone 5125 U` — `#7D6378`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5125 U"))]
    Pms5125U,
    /// `Pantone 513 C` — `#93328E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 513 C"))]
    Pms513C,
    /// `Pantone 513 U` — `#9E5F98`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 513 U"))]
    Pms513U,
    /// `Pantone 5135 C` — `#7E5475`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5135 C"))]
    Pms5135C,
    /// `Pantone 5135 U` — `#8C7287`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5135 U"))]
    Pms5135U,
    /// `Pantone 514 C` — `#D48BC8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 514 C"))]
    Pms514C,
    /// `Pantone 514 U` — `#CE89C3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 514 U"))]
    Pms514U,
    /// `Pantone 5145 C` — `#9B7793`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5145 C"))]
    Pms5145C,
    /// `Pantone 5145 U` — `#A4899F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5145 U"))]
    Pms5145U,
    /// `Pantone 515 C` — `#E2ACD7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 515 C"))]
    Pms515C,
    /// `Pantone 515 U` — `#E2A5D4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 515 U"))]
    Pms515U,
    /// `Pantone 5155 C` — `#BFA5B8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5155 C"))]
    Pms5155C,
    /// `Pantone 5155 U` — `#B99EB2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5155 U"))]
    Pms5155U,
    /// `Pantone 516 C` — `#E6BEDD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 516 C"))]
    Pms516C,
    /// `Pantone 516 U` — `#E8B6DC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 516 U"))]
    Pms516U,
    /// `Pantone 5165 C` — `#D3C0CD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5165 C"))]
    Pms5165C,
    /// `Pantone 5165 U` — `#CCB4C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5165 U"))]
    Pms5165U,
    /// `Pantone 517 C` — `#EBC6DF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 517 C"))]
    Pms517C,
    /// `Pantone 517 U` — `#F2C8E3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 517 U"))]
    Pms517U,
    /// `Pantone 5175 C` — `#D8C8D1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5175 C"))]
    Pms5175C,
    /// `Pantone 5175 U` — `#DAC5D0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5175 U"))]
    Pms5175U,
    /// `Pantone 518 C` — `#4B3048`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 518 C"))]
    Pms518C,
    /// `Pantone 518 U` — `#6C5969`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 518 U"))]
    Pms518U,
    /// `Pantone 5185 C` — `#4A3041`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5185 C"))]
    Pms5185C,
    /// `Pantone 5185 U` — `#756069`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5185 U"))]
    Pms5185U,
    /// `Pantone 519 C` — `#59315F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 519 C"))]
    Pms519C,
    /// `Pantone 519 U` — `#775D7A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 519 U"))]
    Pms519U,
    /// `Pantone 5195 C` — `#66435A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5195 C"))]
    Pms5195C,
    /// `Pantone 5195 U` — `#816C78`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5195 U"))]
    Pms5195U,
    /// `Pantone 520 C` — `#642F6C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 520 C"))]
    Pms520C,
    /// `Pantone 520 U` — `#815F87`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 520 U"))]
    Pms520U,
    /// `Pantone 5205 C` — `#86647A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5205 C"))]
    Pms5205C,
    /// `Pantone 5205 U` — `#907B8A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5205 U"))]
    Pms5205U,
    /// `Pantone 521 C` — `#A57FB2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 521 C"))]
    Pms521C,
    /// `Pantone 521 U` — `#A98BB5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 521 U"))]
    Pms521U,
    /// `Pantone 5215 C` — `#AF95A6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5215 C"))]
    Pms5215C,
    /// `Pantone 5215 U` — `#A28C9C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5215 U"))]
    Pms5215U,
    /// `Pantone 522 C` — `#BA9CC5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 522 C"))]
    Pms522C,
    /// `Pantone 522 U` — `#BC9FC6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 522 U"))]
    Pms522U,
    /// `Pantone 5225 C` — `#C6B0BC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5225 C"))]
    Pms5225C,
    /// `Pantone 5225 U` — `#C2ACB9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5225 U"))]
    Pms5225U,
    /// `Pantone 523 C` — `#C9B1D0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 523 C"))]
    Pms523C,
    /// `Pantone 523 U` — `#D0B7D5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 523 U"))]
    Pms523U,
    /// `Pantone 5235 C` — `#D0BEC7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5235 C"))]
    Pms5235C,
    /// `Pantone 5235 U` — `#D6C3CC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5235 U"))]
    Pms5235U,
    /// `Pantone 524 C` — `#D5C2D8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 524 C"))]
    Pms524C,
    /// `Pantone 524 U` — `#DAC5DC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 524 U"))]
    Pms524U,
    /// `Pantone 5245 C` — `#DBCDD3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5245 C"))]
    Pms5245C,
    /// `Pantone 5245 U` — `#E0CFD6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5245 U"))]
    Pms5245U,
    /// `Pantone 525 C` — `#572C5F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 525 C"))]
    Pms525C,
    /// `Pantone 525 U` — `#735775`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 525 U"))]
    Pms525U,
    /// `Pantone 5255 C` — `#1E1A34`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5255 C"))]
    Pms5255C,
    /// `Pantone 5255 U` — `#5D566C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5255 U"))]
    Pms5255U,
    /// `Pantone 526 C` — `#702F8A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 526 C"))]
    Pms526C,
    /// `Pantone 526 U` — `#805791`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 526 U"))]
    Pms526U,
    /// `Pantone 5265 C` — `#403A60`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5265 C"))]
    Pms5265C,
    /// `Pantone 5265 U` — `#6D6B82`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5265 U"))]
    Pms5265U,
    /// `Pantone 527 C` — `#8031A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 527 C"))]
    Pms527C,
    /// `Pantone 527 U` — `#9361B0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 527 U"))]
    Pms527U,
    /// `Pantone 5275 C` — `#595478`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5275 C"))]
    Pms5275C,
    /// `Pantone 5275 U` — `#7B7992`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5275 U"))]
    Pms5275U,
    /// `Pantone 528 C` — `#B580D1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 528 C"))]
    Pms528C,
    /// `Pantone 528 U` — `#B889D2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 528 U"))]
    Pms528U,
    /// `Pantone 5285 C` — `#8D89A5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5285 C"))]
    Pms5285C,
    /// `Pantone 5285 U` — `#9694AC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5285 U"))]
    Pms5285U,
    /// `Pantone 529 C` — `#CAA2DD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 529 C"))]
    Pms529C,
    /// `Pantone 529 U` — `#CEA5E1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 529 U"))]
    Pms529U,
    /// `Pantone 5295 C` — `#B3B0C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5295 C"))]
    Pms5295C,
    /// `Pantone 5295 U` — `#B1AFC3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5295 U"))]
    Pms5295U,
    /// `Pantone 530 C` — `#D7B9E4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 530 C"))]
    Pms530C,
    /// `Pantone 530 U` — `#DBB9E8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 530 U"))]
    Pms530U,
    /// `Pantone 5305 C` — `#C6C4D2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5305 C"))]
    Pms5305C,
    /// `Pantone 5305 U` — `#C5C2D2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5305 U"))]
    Pms5305U,
    /// `Pantone 531 C` — `#DFC8E7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 531 C"))]
    Pms531C,
    /// `Pantone 531 U` — `#E4C7EB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 531 U"))]
    Pms531U,
    /// `Pantone 5315 C` — `#D8D7DF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5315 C"))]
    Pms5315C,
    /// `Pantone 5315 U` — `#DBD8E1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5315 U"))]
    Pms5315U,
    /// `Pantone 532 C` — `#1C1F2A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 532 C"))]
    Pms532C,
    /// `Pantone 532 U` — `#5B5C64`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 532 U"))]
    Pms532U,
    /// `Pantone 533 C` — `#1F2A44`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 533 C"))]
    Pms533C,
    /// `Pantone 533 U` — `#5E6578`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 533 U"))]
    Pms533U,
    /// `Pantone 534 C` — `#1B365D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 534 C"))]
    Pms534C,
    /// `Pantone 534 U` — `#5B6983`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 534 U"))]
    Pms534U,
    /// `Pantone 535 C` — `#8E9FBC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 535 C"))]
    Pms535C,
    /// `Pantone 535 U` — `#8C9DB8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 535 U"))]
    Pms535U,
    /// `Pantone 536 C` — `#A2B2C8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 536 C"))]
    Pms536C,
    /// `Pantone 536 U` — `#A4B5CB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 536 U"))]
    Pms536U,
    /// `Pantone 537 C` — `#BBC7D6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 537 C"))]
    Pms537C,
    /// `Pantone 537 U` — `#BDCADA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 537 U"))]
    Pms537U,
    /// `Pantone 538 C` — `#C5CFDA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 538 C"))]
    Pms538C,
    /// `Pantone 538 U` — `#CDD7E1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 538 U"))]
    Pms538U,
    /// `Pantone 539 C` — `#00263A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 539 C"))]
    Pms539C,
    /// `Pantone 539 U` — `#465062`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 539 U"))]
    Pms539U,
    /// `Pantone 5395 C` — `#081F2C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5395 C"))]
    Pms5395C,
    /// `Pantone 5395 U` — `#464C59`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5395 U"))]
    Pms5395U,
    /// `Pantone 540 C` — `#003057`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 540 C"))]
    Pms540C,
    /// `Pantone 540 U` — `#3F526F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 540 U"))]
    Pms540U,
    /// `Pantone 5405 C` — `#4F758B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5405 C"))]
    Pms5405C,
    /// `Pantone 5405 U` — `#5B6F80`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5405 U"))]
    Pms5405U,
    /// `Pantone 541 C` — `#003C71`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 541 C"))]
    Pms541C,
    /// `Pantone 541 U` — `#3A557C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 541 U"))]
    Pms541U,
    /// `Pantone 5415 C` — `#5B7F95`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5415 C"))]
    Pms5415C,
    /// `Pantone 5415 U` — `#677F91`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5415 U"))]
    Pms5415U,
    /// `Pantone 542 C` — `#7BAFD4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 542 C"))]
    Pms542C,
    /// `Pantone 542 U` — `#699CC6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 542 U"))]
    Pms542U,
    /// `Pantone 5425 C` — `#7A99AC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5425 C"))]
    Pms5425C,
    /// `Pantone 5425 U` — `#7993A5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5425 U"))]
    Pms5425U,
    /// `Pantone 543 C` — `#A4C8E1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 543 C"))]
    Pms543C,
    /// `Pantone 543 U` — `#79ABD2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 543 U"))]
    Pms543U,
    /// `Pantone 5435 C` — `#A6BBC8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5435 C"))]
    Pms5435C,
    /// `Pantone 5435 U` — `#9DB4C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5435 U"))]
    Pms5435U,
    /// `Pantone 544 C` — `#BDD6E6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 544 C"))]
    Pms544C,
    /// `Pantone 544 U` — `#97C1DF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 544 U"))]
    Pms544U,
    /// `Pantone 5445 C` — `#B7C9D3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5445 C"))]
    Pms5445C,
    /// `Pantone 5445 U` — `#B4C7D3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5445 U"))]
    Pms5445U,
    /// `Pantone 545 C` — `#C6DAE7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 545 C"))]
    Pms545C,
    /// `Pantone 545 U` — `#AECFE6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 545 U"))]
    Pms545U,
    /// `Pantone 5455 C` — `#BFCED6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5455 C"))]
    Pms5455C,
    /// `Pantone 5455 U` — `#C7D7E0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5455 U"))]
    Pms5455U,
    /// `Pantone 546 C` — `#072B31`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 546 C"))]
    Pms546C,
    /// `Pantone 546 U` — `#465058`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 546 U"))]
    Pms546U,
    /// `Pantone 5463 C` — `#07272D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5463 C"))]
    Pms5463C,
    /// `Pantone 5463 U` — `#3E4E56`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5463 U"))]
    Pms5463U,
    /// `Pantone 5467 C` — `#18332F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5467 C"))]
    Pms5467C,
    /// `Pantone 5467 U` — `#545F5F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5467 U"))]
    Pms5467U,
    /// `Pantone 547 C` — `#00313C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 547 C"))]
    Pms547C,
    /// `Pantone 547 U` — `#415866`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 547 U"))]
    Pms547U,
    /// `Pantone 5473 C` — `#115E67`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5473 C"))]
    Pms5473C,
    /// `Pantone 5473 U` — `#516D75`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5473 U"))]
    Pms5473U,
    /// `Pantone 5477 C` — `#3E5D58`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5477 C"))]
    Pms5477C,
    /// `Pantone 5477 U` — `#5F7070`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5477 U"))]
    Pms5477U,
    /// `Pantone 548 C` — `#003D4C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 548 C"))]
    Pms548C,
    /// `Pantone 548 U` — `#3E5E6F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 548 U"))]
    Pms548U,
    /// `Pantone 5483 C` — `#4F868E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5483 C"))]
    Pms5483C,
    /// `Pantone 5483 U` — `#5F8289`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5483 U"))]
    Pms5483U,
    /// `Pantone 5487 C` — `#5D7975`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5487 C"))]
    Pms5487C,
    /// `Pantone 5487 U` — `#697C7D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5487 U"))]
    Pms5487U,
    /// `Pantone 549 C` — `#6BA4B8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 549 C"))]
    Pms549C,
    /// `Pantone 549 U` — `#6899AE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 549 U"))]
    Pms549U,
    /// `Pantone 5493 C` — `#7FA9AE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5493 C"))]
    Pms5493C,
    /// `Pantone 5493 U` — `#769CA3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5493 U"))]
    Pms5493U,
    /// `Pantone 5497 C` — `#829995`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5497 C"))]
    Pms5497C,
    /// `Pantone 5497 U` — `#819595`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5497 U"))]
    Pms5497U,
    /// `Pantone 550 C` — `#8DB9CA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 550 C"))]
    Pms550C,
    /// `Pantone 550 U` — `#7AAABE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 550 U"))]
    Pms550U,
    /// `Pantone 5503 C` — `#94B7BB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5503 C"))]
    Pms5503C,
    /// `Pantone 5503 U` — `#94B7BC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5503 U"))]
    Pms5503U,
    /// `Pantone 5507 C` — `#9DB0AC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5507 C"))]
    Pms5507C,
    /// `Pantone 5507 U` — `#96A9A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5507 U"))]
    Pms5507U,
    /// `Pantone 551 C` — `#A3C7D2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 551 C"))]
    Pms551C,
    /// `Pantone 551 U` — `#96C0CE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 551 U"))]
    Pms551U,
    /// `Pantone 5513 C` — `#ABC7CA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5513 C"))]
    Pms5513C,
    /// `Pantone 5513 U` — `#ACCACD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5513 U"))]
    Pms5513U,
    /// `Pantone 5517 C` — `#B1C0BC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5517 C"))]
    Pms5517C,
    /// `Pantone 5517 U` — `#A9BAB7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5517 U"))]
    Pms5517U,
    /// `Pantone 552 C` — `#B9D3DC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 552 C"))]
    Pms552C,
    /// `Pantone 552 U` — `#B0D0DC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 552 U"))]
    Pms552U,
    /// `Pantone 5523 C` — `#B6CFD0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5523 C"))]
    Pms5523C,
    /// `Pantone 5523 U` — `#BCD5D6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5523 U"))]
    Pms5523U,
    /// `Pantone 5527 C` — `#BCC9C5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5527 C"))]
    Pms5527C,
    /// `Pantone 5527 U` — `#B8C7C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5527 U"))]
    Pms5527U,
    /// `Pantone 553 C` — `#284734`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 553 C"))]
    Pms553C,
    /// `Pantone 553 U` — `#5A685D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 553 U"))]
    Pms553U,
    /// `Pantone 5535 C` — `#183028`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5535 C"))]
    Pms5535C,
    /// `Pantone 5535 U` — `#485956`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5535 U"))]
    Pms5535U,
    /// `Pantone 554 C` — `#205C40`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 554 C"))]
    Pms554C,
    /// `Pantone 554 U` — `#577565`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 554 U"))]
    Pms554U,
    /// `Pantone 5545 C` — `#43695B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5545 C"))]
    Pms5545C,
    /// `Pantone 5545 U` — `#60766F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5545 U"))]
    Pms5545U,
    /// `Pantone 555 C` — `#28724F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 555 C"))]
    Pms555C,
    /// `Pantone 555 U` — `#567E6A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 555 U"))]
    Pms555U,
    /// `Pantone 5555 C` — `#5C7F71`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5555 C"))]
    Pms5555C,
    /// `Pantone 5555 U` — `#768E87`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5555 U"))]
    Pms5555U,
    /// `Pantone 556 C` — `#6FA287`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 556 C"))]
    Pms556C,
    /// `Pantone 556 U` — `#719E8B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 556 U"))]
    Pms556U,
    /// `Pantone 5565 C` — `#7F9C90`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5565 C"))]
    Pms5565C,
    /// `Pantone 5565 U` — `#8AA29A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5565 U"))]
    Pms5565U,
    /// `Pantone 557 C` — `#85B09A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 557 C"))]
    Pms557C,
    /// `Pantone 557 U` — `#88B29F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 557 U"))]
    Pms557U,
    /// `Pantone 5575 C` — `#92ACA0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5575 C"))]
    Pms5575C,
    /// `Pantone 5575 U` — `#9DB4AB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5575 U"))]
    Pms5575U,
    /// `Pantone 558 C` — `#9ABEAA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 558 C"))]
    Pms558C,
    /// `Pantone 558 U` — `#9AC0AE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 558 U"))]
    Pms558U,
    /// `Pantone 5585 C` — `#A7BDB1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5585 C"))]
    Pms5585C,
    /// `Pantone 5585 U` — `#A2B9AF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5585 U"))]
    Pms5585U,
    /// `Pantone 559 C` — `#ADCAB8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 559 C"))]
    Pms559C,
    /// `Pantone 559 U` — `#B5D1C0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 559 U"))]
    Pms559U,
    /// `Pantone 5595 C` — `#BFCEC2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5595 C"))]
    Pms5595C,
    /// `Pantone 5595 U` — `#C1D2C7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5595 U"))]
    Pms5595U,
    /// `Pantone 560 C` — `#1D3C34`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 560 C"))]
    Pms560C,
    /// `Pantone 560 U` — `#4B5E5B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 560 U"))]
    Pms560U,
    /// `Pantone 5605 C` — `#22372B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5605 C"))]
    Pms5605C,
    /// `Pantone 5605 U` — `#5E695E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5605 U"))]
    Pms5605U,
    /// `Pantone 561 C` — `#00594C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 561 C"))]
    Pms561C,
    /// `Pantone 561 U` — `#466D69`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 561 U"))]
    Pms561U,
    /// `Pantone 5615 C` — `#5E7461`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5615 C"))]
    Pms5615C,
    /// `Pantone 5615 U` — `#6A7970`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5615 U"))]
    Pms5615U,
    /// `Pantone 562 C` — `#006F62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 562 C"))]
    Pms562C,
    /// `Pantone 562 U` — `#43807A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 562 U"))]
    Pms562U,
    /// `Pantone 5625 C` — `#708573`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5625 C"))]
    Pms5625C,
    /// `Pantone 5625 U` — `#798981`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5625 U"))]
    Pms5625U,
    /// `Pantone 563 C` — `#6BBBAE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 563 C"))]
    Pms563C,
    /// `Pantone 563 U` — `#76B9AF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 563 U"))]
    Pms563U,
    /// `Pantone 5635 C` — `#94A596`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5635 C"))]
    Pms5635C,
    /// `Pantone 5635 U` — `#8D9D94`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5635 U"))]
    Pms5635U,
    /// `Pantone 564 C` — `#86C8BC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 564 C"))]
    Pms564C,
    /// `Pantone 564 U` — `#88C6BC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 564 U"))]
    Pms564U,
    /// `Pantone 5645 C` — `#A3B2A4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5645 C"))]
    Pms5645C,
    /// `Pantone 5645 U` — `#9FAFA5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5645 U"))]
    Pms5645U,
    /// `Pantone 565 C` — `#A1D6CA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 565 C"))]
    Pms565C,
    /// `Pantone 565 U` — `#91D1C5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 565 U"))]
    Pms565U,
    /// `Pantone 5655 C` — `#B0BDB0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5655 C"))]
    Pms5655C,
    /// `Pantone 5655 U` — `#B3C0B6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5655 U"))]
    Pms5655U,
    /// `Pantone 566 C` — `#B9DCD2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 566 C"))]
    Pms566C,
    /// `Pantone 566 U` — `#B4E1D6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 566 U"))]
    Pms566U,
    /// `Pantone 5665 C` — `#BAC5B9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5665 C"))]
    Pms5665C,
    /// `Pantone 5665 U` — `#BCC8BD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5665 U"))]
    Pms5665U,
    /// `Pantone 567 C` — `#173F35`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 567 C"))]
    Pms567C,
    /// `Pantone 567 U` — `#4A635D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 567 U"))]
    Pms567U,
    /// `Pantone 568 C` — `#006C5B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 568 C"))]
    Pms568C,
    /// `Pantone 568 U` — `#458075`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 568 U"))]
    Pms568U,
    /// `Pantone 569 C` — `#00816D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 569 C"))]
    Pms569C,
    /// `Pantone 569 U` — `#2C8F82`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 569 U"))]
    Pms569U,
    /// `Pantone 570 C` — `#6BCABA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 570 C"))]
    Pms570C,
    /// `Pantone 570 U` — `#62C2B1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 570 U"))]
    Pms570U,
    /// `Pantone 571 C` — `#98DBCE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 571 C"))]
    Pms571C,
    /// `Pantone 571 U` — `#85D8C8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 571 U"))]
    Pms571U,
    /// `Pantone 572 C` — `#A5DFD3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 572 C"))]
    Pms572C,
    /// `Pantone 572 U` — `#95DECF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 572 U"))]
    Pms572U,
    /// `Pantone 573 C` — `#B5E3D8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 573 C"))]
    Pms573C,
    /// `Pantone 573 U` — `#BBE9DE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 573 U"))]
    Pms573U,
    /// `Pantone 574 C` — `#4E5B31`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 574 C"))]
    Pms574C,
    /// `Pantone 574 U` — `#6A7355`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 574 U"))]
    Pms574U,
    /// `Pantone 5743 C` — `#3E4827`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5743 C"))]
    Pms5743C,
    /// `Pantone 5743 U` — `#636A51`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5743 U"))]
    Pms5743U,
    /// `Pantone 5747 C` — `#3D441E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5747 C"))]
    Pms5747C,
    /// `Pantone 5747 U` — `#70744E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5747 U"))]
    Pms5747U,
    /// `Pantone 575 C` — `#67823A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 575 C"))]
    Pms575C,
    /// `Pantone 575 U` — `#6F8456`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 575 U"))]
    Pms575U,
    /// `Pantone 5753 C` — `#5E6738`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5753 C"))]
    Pms5753C,
    /// `Pantone 5753 U` — `#747C61`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5753 U"))]
    Pms5753U,
    /// `Pantone 5757 C` — `#6D712E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5757 C"))]
    Pms5757C,
    /// `Pantone 5757 U` — `#81855C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5757 U"))]
    Pms5757U,
    /// `Pantone 576 C` — `#789D4A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 576 C"))]
    Pms576C,
    /// `Pantone 576 U` — `#7A9B62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 576 U"))]
    Pms576U,
    /// `Pantone 5763 C` — `#737B4C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5763 C"))]
    Pms5763C,
    /// `Pantone 5763 U` — `#7E876D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5763 U"))]
    Pms5763U,
    /// `Pantone 5767 C` — `#8A8D4A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5767 C"))]
    Pms5767C,
    /// `Pantone 5767 U` — `#94986C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5767 U"))]
    Pms5767U,
    /// `Pantone 577 C` — `#A9C47F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 577 C"))]
    Pms577C,
    /// `Pantone 577 U` — `#93B479`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 577 U"))]
    Pms577U,
    /// `Pantone 5773 C` — `#899064`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5773 C"))]
    Pms5773C,
    /// `Pantone 5773 U` — `#9AA184`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5773 U"))]
    Pms5773U,
    /// `Pantone 5777 C` — `#A2A569`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5777 C"))]
    Pms5777C,
    /// `Pantone 5777 U` — `#A6A97B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5777 U"))]
    Pms5777U,
    /// `Pantone 578 C` — `#B7CE95`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 578 C"))]
    Pms578C,
    /// `Pantone 578 U` — `#A7C58E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 578 U"))]
    Pms578U,
    /// `Pantone 5783 C` — `#A3AA83`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5783 C"))]
    Pms5783C,
    /// `Pantone 5783 U` — `#B5BB9F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5783 U"))]
    Pms5783U,
    /// `Pantone 5787 C` — `#BABD8B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5787 C"))]
    Pms5787C,
    /// `Pantone 5787 U` — `#BABE8F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5787 U"))]
    Pms5787U,
    /// `Pantone 579 C` — `#BCD19B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 579 C"))]
    Pms579C,
    /// `Pantone 579 U` — `#BAD29E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 579 U"))]
    Pms579U,
    /// `Pantone 5793 C` — `#B3B995`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5793 C"))]
    Pms5793C,
    /// `Pantone 5793 U` — `#C5CAAF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5793 U"))]
    Pms5793U,
    /// `Pantone 5797 C` — `#C6C89B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5797 C"))]
    Pms5797C,
    /// `Pantone 5797 U` — `#C7CA9C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5797 U"))]
    Pms5797U,
    /// `Pantone 580 C` — `#C4D6A4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 580 C"))]
    Pms580C,
    /// `Pantone 580 U` — `#C9DCAC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 580 U"))]
    Pms580U,
    /// `Pantone 5803 C` — `#C3C6A8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5803 C"))]
    Pms5803C,
    /// `Pantone 5803 U` — `#CFD3B8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5803 U"))]
    Pms5803U,
    /// `Pantone 5807 C` — `#D0D1AB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5807 C"))]
    Pms5807C,
    /// `Pantone 5807 U` — `#D1D3A9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5807 U"))]
    Pms5807U,
    /// `Pantone 581 C` — `#625D20`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 581 C"))]
    Pms581C,
    /// `Pantone 581 U` — `#78754A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 581 U"))]
    Pms581U,
    /// `Pantone 5815 C` — `#555025`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5815 C"))]
    Pms5815C,
    /// `Pantone 5815 U` — `#736F4C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5815 U"))]
    Pms5815U,
    /// `Pantone 582 C` — `#8E8C13`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 582 C"))]
    Pms582C,
    /// `Pantone 582 U` — `#919145`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 582 U"))]
    Pms582U,
    /// `Pantone 5825 C` — `#89813D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5825 C"))]
    Pms5825C,
    /// `Pantone 5825 U` — `#8B865D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5825 U"))]
    Pms5825U,
    /// `Pantone 583 C` — `#B7BF10`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 583 C"))]
    Pms583C,
    /// `Pantone 583 U` — `#A6B340`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 583 U"))]
    Pms583U,
    /// `Pantone 5835 C` — `#A09958`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5835 C"))]
    Pms5835C,
    /// `Pantone 5835 U` — `#938F66`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5835 U"))]
    Pms5835U,
    /// `Pantone 584 C` — `#D2D755`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 584 C"))]
    Pms584C,
    /// `Pantone 584 U` — `#CED665`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 584 U"))]
    Pms584U,
    /// `Pantone 5845 C` — `#AFA96E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5845 C"))]
    Pms5845C,
    /// `Pantone 5845 U` — `#A7A375`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5845 U"))]
    Pms5845U,
    /// `Pantone 585 C` — `#DBDE70`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 585 C"))]
    Pms585C,
    /// `Pantone 585 U` — `#E3E685`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 585 U"))]
    Pms585U,
    /// `Pantone 5855 C` — `#C0BB87`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5855 C"))]
    Pms5855C,
    /// `Pantone 5855 U` — `#BDB889`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5855 U"))]
    Pms5855U,
    /// `Pantone 586 C` — `#E0E27C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 586 C"))]
    Pms586C,
    /// `Pantone 586 U` — `#E7E988`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 586 U"))]
    Pms586U,
    /// `Pantone 5865 C` — `#CBC793`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5865 C"))]
    Pms5865C,
    /// `Pantone 5865 U` — `#CCC895`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5865 U"))]
    Pms5865U,
    /// `Pantone 587 C` — `#E3E48D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 587 C"))]
    Pms587C,
    /// `Pantone 587 U` — `#EAEC97`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 587 U"))]
    Pms587U,
    /// `Pantone 5875 C` — `#D2CE9E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5875 C"))]
    Pms5875C,
    /// `Pantone 5875 U` — `#D7D3A1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 5875 U"))]
    Pms5875U,
    /// `Pantone 600 C` — `#F1EB9C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 600 C"))]
    Pms600C,
    /// `Pantone 600 U` — `#F9F29D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 600 U"))]
    Pms600U,
    /// `Pantone 601 C` — `#F0E991`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 601 C"))]
    Pms601C,
    /// `Pantone 601 U` — `#F7EF8F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 601 U"))]
    Pms601U,
    /// `Pantone 602 C` — `#F0E87B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 602 C"))]
    Pms602C,
    /// `Pantone 602 U` — `#F7EE83`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 602 U"))]
    Pms602U,
    /// `Pantone 603 C` — `#EDE04B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 603 C"))]
    Pms603C,
    /// `Pantone 603 U` — `#F3E564`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 603 U"))]
    Pms603U,
    /// `Pantone 604 C` — `#EADA24`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 604 C"))]
    Pms604C,
    /// `Pantone 604 U` — `#EDDC4D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 604 U"))]
    Pms604U,
    /// `Pantone 605 C` — `#E1CD00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 605 C"))]
    Pms605C,
    /// `Pantone 605 U` — `#E0CC39`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 605 U"))]
    Pms605U,
    /// `Pantone 606 C` — `#CFB500`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 606 C"))]
    Pms606C,
    /// `Pantone 606 U` — `#CEB829`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 606 U"))]
    Pms606U,
    /// `Pantone 607 C` — `#EBE49A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 607 C"))]
    Pms607C,
    /// `Pantone 607 U` — `#F3EAA1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 607 U"))]
    Pms607U,
    /// `Pantone 608 C` — `#E9E186`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 608 C"))]
    Pms608C,
    /// `Pantone 608 U` — `#F0E691`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 608 U"))]
    Pms608U,
    /// `Pantone 609 C` — `#E6DE77`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 609 C"))]
    Pms609C,
    /// `Pantone 609 U` — `#EBE081`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 609 U"))]
    Pms609U,
    /// `Pantone 610 C` — `#E1D555`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 610 C"))]
    Pms610C,
    /// `Pantone 610 U` — `#E0D268`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 610 U"))]
    Pms610U,
    /// `Pantone 611 C` — `#D7C826`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 611 C"))]
    Pms611C,
    /// `Pantone 611 U` — `#C8B94E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 611 U"))]
    Pms611U,
    /// `Pantone 612 C` — `#C4B000`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 612 C"))]
    Pms612C,
    /// `Pantone 612 U` — `#B7A73F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 612 U"))]
    Pms612U,
    /// `Pantone 613 C` — `#B39B00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 613 C"))]
    Pms613C,
    /// `Pantone 613 U` — `#A29438`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 613 U"))]
    Pms613U,
    /// `Pantone 614 C` — `#DCD59A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 614 C"))]
    Pms614C,
    /// `Pantone 614 U` — `#E1D9A1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 614 U"))]
    Pms614U,
    /// `Pantone 615 C` — `#D6CF8D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 615 C"))]
    Pms615C,
    /// `Pantone 615 U` — `#D8D096`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 615 U"))]
    Pms615U,
    /// `Pantone 616 C` — `#D0C883`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 616 C"))]
    Pms616C,
    /// `Pantone 616 U` — `#CDC588`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 616 U"))]
    Pms616U,
    /// `Pantone 617 C` — `#C0B561`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 617 C"))]
    Pms617C,
    /// `Pantone 617 U` — `#BBB275`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 617 U"))]
    Pms617U,
    /// `Pantone 618 C` — `#AC9F3C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 618 C"))]
    Pms618C,
    /// `Pantone 618 U` — `#A69D62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 618 U"))]
    Pms618U,
    /// `Pantone 619 C` — `#9F912A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 619 C"))]
    Pms619C,
    /// `Pantone 619 U` — `#968D54`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 619 U"))]
    Pms619U,
    /// `Pantone 620 C` — `#8A7B19`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 620 C"))]
    Pms620C,
    /// `Pantone 620 U` — `#877F49`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 620 U"))]
    Pms620U,
    /// `Pantone 621 C` — `#D1E0D7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 621 C"))]
    Pms621C,
    /// `Pantone 621 U` — `#CADFD4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 621 U"))]
    Pms621U,
    /// `Pantone 622 C` — `#B7CDC2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 622 C"))]
    Pms622C,
    /// `Pantone 622 U` — `#B3CBC1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 622 U"))]
    Pms622U,
    /// `Pantone 623 C` — `#9AB9AD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 623 C"))]
    Pms623C,
    /// `Pantone 623 U` — `#96B5AB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 623 U"))]
    Pms623U,
    /// `Pantone 624 C` — `#789F90`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 624 C"))]
    Pms624C,
    /// `Pantone 624 U` — `#82A198`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 624 U"))]
    Pms624U,
    /// `Pantone 625 C` — `#507F70`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 625 C"))]
    Pms625C,
    /// `Pantone 625 U` — `#6C8A82`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 625 U"))]
    Pms625U,
    /// `Pantone 626 C` — `#285C4D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 626 C"))]
    Pms626C,
    /// `Pantone 626 U` — `#59756F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 626 U"))]
    Pms626U,
    /// `Pantone 627 C` — `#13322B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 627 C"))]
    Pms627C,
    /// `Pantone 627 U` — `#455D58`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 627 U"))]
    Pms627U,
    /// `Pantone 628 C` — `#B8DDE1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 628 C"))]
    Pms628C,
    /// `Pantone 628 U` — `#B4E0E6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 628 U"))]
    Pms628U,
    /// `Pantone 629 C` — `#9BD3DD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 629 C"))]
    Pms629C,
    /// `Pantone 629 U` — `#8DD0DE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 629 U"))]
    Pms629U,
    /// `Pantone 630 C` — `#77C5D5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 630 C"))]
    Pms630C,
    /// `Pantone 630 U` — `#74C4D7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 630 U"))]
    Pms630U,
    /// `Pantone 631 C` — `#3EB1C8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 631 C"))]
    Pms631C,
    /// `Pantone 631 U` — `#56B2CB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 631 U"))]
    Pms631U,
    /// `Pantone 632 C` — `#0093B2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 632 C"))]
    Pms632C,
    /// `Pantone 632 U` — `#3998B5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 632 U"))]
    Pms632U,
    /// `Pantone 633 C` — `#007396`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 633 C"))]
    Pms633C,
    /// `Pantone 633 U` — `#27829E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 633 U"))]
    Pms633U,
    /// `Pantone 634 C` — `#005F83`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 634 C"))]
    Pms634C,
    /// `Pantone 634 U` — `#226E8A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 634 U"))]
    Pms634U,
    /// `Pantone 635 C` — `#A4DBE8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 635 C"))]
    Pms635C,
    /// `Pantone 635 U` — `#ACE3EF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 635 U"))]
    Pms635U,
    /// `Pantone 636 C` — `#8BD3E6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 636 C"))]
    Pms636C,
    /// `Pantone 636 U` — `#8AD8ED`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 636 U"))]
    Pms636U,
    /// `Pantone 637 C` — `#4EC3E0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 637 C"))]
    Pms637C,
    /// `Pantone 637 U` — `#5EC8E5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 637 U"))]
    Pms637U,
    /// `Pantone 638 C` — `#00AFD7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 638 C"))]
    Pms638C,
    /// `Pantone 638 U` — `#32B8DF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 638 U"))]
    Pms638U,
    /// `Pantone 639 C` — `#0095C8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 639 C"))]
    Pms639C,
    /// `Pantone 639 U` — `#009ECC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 639 U"))]
    Pms639U,
    /// `Pantone 640 C` — `#0082BA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 640 C"))]
    Pms640C,
    /// `Pantone 640 U` — `#008ABA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 640 U"))]
    Pms640U,
    /// `Pantone 641 C` — `#0067A0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 641 C"))]
    Pms641C,
    /// `Pantone 641 U` — `#0078A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 641 U"))]
    Pms641U,
    /// `Pantone 642 C` — `#D1DDE6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 642 C"))]
    Pms642C,
    /// `Pantone 642 U` — `#D3E0EA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 642 U"))]
    Pms642U,
    /// `Pantone 643 C` — `#C6D6E3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 643 C"))]
    Pms643C,
    /// `Pantone 643 U` — `#BFD2E4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 643 U"))]
    Pms643U,
    /// `Pantone 644 C` — `#9BB8D3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 644 C"))]
    Pms644C,
    /// `Pantone 644 U` — `#9DB9D5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 644 U"))]
    Pms644U,
    /// `Pantone 645 C` — `#7DA1C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 645 C"))]
    Pms645C,
    /// `Pantone 645 U` — `#84A3C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 645 U"))]
    Pms645U,
    /// `Pantone 646 C` — `#5E8AB4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 646 C"))]
    Pms646C,
    /// `Pantone 646 U` — `#6E8DB0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 646 U"))]
    Pms646U,
    /// `Pantone 647 C` — `#236192`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 647 C"))]
    Pms647C,
    /// `Pantone 647 U` — `#5F7A9D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 647 U"))]
    Pms647U,
    /// `Pantone 648 C` — `#002E5D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 648 C"))]
    Pms648C,
    /// `Pantone 648 U` — `#4E6282`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 648 U"))]
    Pms648U,
    /// `Pantone 649 C` — `#DBE2E9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 649 C"))]
    Pms649C,
    /// `Pantone 649 U` — `#E1E7EE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 649 U"))]
    Pms649U,
    /// `Pantone 650 C` — `#CED9E5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 650 C"))]
    Pms650C,
    /// `Pantone 650 U` — `#D7E0EB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 650 U"))]
    Pms650U,
    /// `Pantone 651 C` — `#A7BCD6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 651 C"))]
    Pms651C,
    /// `Pantone 651 U` — `#AFC3DC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 651 U"))]
    Pms651U,
    /// `Pantone 652 C` — `#7D9BC1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 652 C"))]
    Pms652C,
    /// `Pantone 652 U` — `#89A2C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 652 U"))]
    Pms652U,
    /// `Pantone 653 C` — `#326295`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 653 C"))]
    Pms653C,
    /// `Pantone 653 U` — `#6780A4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 653 U"))]
    Pms653U,
    /// `Pantone 654 C` — `#003A70`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 654 C"))]
    Pms654C,
    /// `Pantone 654 U` — `#55688B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 654 U"))]
    Pms654U,
    /// `Pantone 655 C` — `#002554`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 655 C"))]
    Pms655C,
    /// `Pantone 655 U` — `#4E5D7F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 655 U"))]
    Pms655U,
    /// `Pantone 656 C` — `#DDE5ED`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 656 C"))]
    Pms656C,
    /// `Pantone 656 U` — `#E2EAF2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 656 U"))]
    Pms656U,
    /// `Pantone 657 C` — `#C8D8EB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 657 C"))]
    Pms657C,
    /// `Pantone 657 U` — `#D6E2F2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 657 U"))]
    Pms657U,
    /// `Pantone 658 C` — `#B1C9E8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 658 C"))]
    Pms658C,
    /// `Pantone 658 U` — `#B8CFEE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 658 U"))]
    Pms658U,
    /// `Pantone 659 C` — `#7BA4DB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 659 C"))]
    Pms659C,
    /// `Pantone 659 U` — `#96B6E5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 659 U"))]
    Pms659U,
    /// `Pantone 660 C` — `#407EC9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 660 C"))]
    Pms660C,
    /// `Pantone 660 U` — `#658BCC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 660 U"))]
    Pms660U,
    /// `Pantone 661 C` — `#003594`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 661 C"))]
    Pms661C,
    /// `Pantone 661 U` — `#4967AA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 661 U"))]
    Pms661U,
    /// `Pantone 662 C` — `#001A70`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 662 C"))]
    Pms662C,
    /// `Pantone 662 U` — `#425390`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 662 U"))]
    Pms662U,
    /// `Pantone 663 C` — `#E5E1E6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 663 C"))]
    Pms663C,
    /// `Pantone 663 U` — `#E2D9E3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 663 U"))]
    Pms663U,
    /// `Pantone 664 C` — `#E0DBE3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 664 C"))]
    Pms664C,
    /// `Pantone 664 U` — `#DACFDD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 664 U"))]
    Pms664U,
    /// `Pantone 665 C` — `#C6BCD0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 665 C"))]
    Pms665C,
    /// `Pantone 665 U` — `#C5B8CE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 665 U"))]
    Pms665U,
    /// `Pantone 666 C` — `#A192B2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 666 C"))]
    Pms666C,
    /// `Pantone 666 U` — `#AEA1BC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 666 U"))]
    Pms666U,
    /// `Pantone 667 C` — `#7C6992`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 667 C"))]
    Pms667C,
    /// `Pantone 667 U` — `#9083A1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 667 U"))]
    Pms667U,
    /// `Pantone 668 C` — `#614B79`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 668 C"))]
    Pms668C,
    /// `Pantone 668 U` — `#817391`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 668 U"))]
    Pms668U,
    /// `Pantone 669 C` — `#3F2A56`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 669 C"))]
    Pms669C,
    /// `Pantone 669 U` — `#71637F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 669 U"))]
    Pms669U,
    /// `Pantone 670 C` — `#EAD3E2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 670 C"))]
    Pms670C,
    /// `Pantone 670 U` — `#F3D6E5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 670 U"))]
    Pms670U,
    /// `Pantone 671 C` — `#E6BCD8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 671 C"))]
    Pms671C,
    /// `Pantone 671 U` — `#EFC3DD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 671 U"))]
    Pms671U,
    /// `Pantone 672 C` — `#DFA0C9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 672 C"))]
    Pms672C,
    /// `Pantone 672 U` — `#E8A7CD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 672 U"))]
    Pms672U,
    /// `Pantone 673 C` — `#D986BA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 673 C"))]
    Pms673C,
    /// `Pantone 673 U` — `#DF8CBA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 673 U"))]
    Pms673U,
    /// `Pantone 674 C` — `#C6579A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 674 C"))]
    Pms674C,
    /// `Pantone 674 U` — `#CD72A1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 674 U"))]
    Pms674U,
    /// `Pantone 675 C` — `#AE2573`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 675 C"))]
    Pms675C,
    /// `Pantone 675 U` — `#BD618E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 675 U"))]
    Pms675U,
    /// `Pantone 676 C` — `#960051`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 676 C"))]
    Pms676C,
    /// `Pantone 676 U` — `#A85079`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 676 U"))]
    Pms676U,
    /// `Pantone 677 C` — `#E5CEDB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 677 C"))]
    Pms677C,
    /// `Pantone 677 U` — `#EDD2E0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 677 U"))]
    Pms677U,
    /// `Pantone 678 C` — `#E3C8D8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 678 C"))]
    Pms678C,
    /// `Pantone 678 U` — `#E5C0D4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 678 U"))]
    Pms678U,
    /// `Pantone 679 C` — `#DEBED2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 679 C"))]
    Pms679C,
    /// `Pantone 679 U` — `#DAACC6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 679 U"))]
    Pms679U,
    /// `Pantone 680 C` — `#C996B6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 680 C"))]
    Pms680C,
    /// `Pantone 680 U` — `#C08BA9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 680 U"))]
    Pms680U,
    /// `Pantone 681 C` — `#B06C96`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 681 C"))]
    Pms681C,
    /// `Pantone 681 U` — `#A5708D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 681 U"))]
    Pms681U,
    /// `Pantone 682 C` — `#994878`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 682 C"))]
    Pms682C,
    /// `Pantone 682 U` — `#905D78`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 682 U"))]
    Pms682U,
    /// `Pantone 683 C` — `#7C2855`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 683 C"))]
    Pms683C,
    /// `Pantone 683 U` — `#805268`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 683 U"))]
    Pms683U,
    /// `Pantone 684 C` — `#E4C6D4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 684 C"))]
    Pms684C,
    /// `Pantone 684 U` — `#ECCBD9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 684 U"))]
    Pms684U,
    /// `Pantone 685 C` — `#DCB6C9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 685 C"))]
    Pms685C,
    /// `Pantone 685 U` — `#E4BDD0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 685 U"))]
    Pms685U,
    /// `Pantone 686 C` — `#D0A1BA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 686 C"))]
    Pms686C,
    /// `Pantone 686 U` — `#D5A6BE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 686 U"))]
    Pms686U,
    /// `Pantone 687 C` — `#BE84A3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 687 C"))]
    Pms687C,
    /// `Pantone 687 U` — `#BD8CA6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 687 U"))]
    Pms687U,
    /// `Pantone 688 C` — `#A76389`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 688 C"))]
    Pms688C,
    /// `Pantone 688 U` — `#A2738C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 688 U"))]
    Pms688U,
    /// `Pantone 689 C` — `#893B67`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 689 C"))]
    Pms689C,
    /// `Pantone 689 U` — `#8E6178`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 689 U"))]
    Pms689U,
    /// `Pantone 690 C` — `#612141`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 690 C"))]
    Pms690C,
    /// `Pantone 690 U` — `#7C5060`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 690 U"))]
    Pms690U,
    /// `Pantone 691 C` — `#E9CDD0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 691 C"))]
    Pms691C,
    /// `Pantone 691 U` — `#F1D3D5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 691 U"))]
    Pms691U,
    /// `Pantone 692 C` — `#E4BEC3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 692 C"))]
    Pms692C,
    /// `Pantone 692 U` — `#E9C0C5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 692 U"))]
    Pms692U,
    /// `Pantone 693 C` — `#D7A3AB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 693 C"))]
    Pms693C,
    /// `Pantone 693 U` — `#DAA5AD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 693 U"))]
    Pms693U,
    /// `Pantone 694 C` — `#C48490`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 694 C"))]
    Pms694C,
    /// `Pantone 694 U` — `#C38D96`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 694 U"))]
    Pms694U,
    /// `Pantone 695 C` — `#B46B7A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 695 C"))]
    Pms695C,
    /// `Pantone 695 U` — `#AC7680`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 695 U"))]
    Pms695U,
    /// `Pantone 696 C` — `#984856`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 696 C"))]
    Pms696C,
    /// `Pantone 696 U` — `#98656C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 696 U"))]
    Pms696U,
    /// `Pantone 697 C` — `#893C47`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 697 C"))]
    Pms697C,
    /// `Pantone 697 U` — `#87565B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 697 U"))]
    Pms697U,
    /// `Pantone 698 C` — `#F2D4D7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 698 C"))]
    Pms698C,
    /// `Pantone 698 U` — `#FAD7DA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 698 U"))]
    Pms698U,
    /// `Pantone 699 C` — `#F4C3CC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 699 C"))]
    Pms699C,
    /// `Pantone 699 U` — `#FCC2CC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 699 U"))]
    Pms699U,
    /// `Pantone 700 C` — `#F2ACB9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 700 C"))]
    Pms700C,
    /// `Pantone 700 U` — `#F8AAB6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 700 U"))]
    Pms700U,
    /// `Pantone 701 C` — `#E68699`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 701 C"))]
    Pms701C,
    /// `Pantone 701 U` — `#E98F9D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 701 U"))]
    Pms701U,
    /// `Pantone 702 C` — `#D25B73`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 702 C"))]
    Pms702C,
    /// `Pantone 702 U` — `#D87987`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 702 U"))]
    Pms702U,
    /// `Pantone 703 C` — `#B83A4B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 703 C"))]
    Pms703C,
    /// `Pantone 703 U` — `#B45A65`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 703 U"))]
    Pms703U,
    /// `Pantone 704 C` — `#9E2A2F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 704 C"))]
    Pms704C,
    /// `Pantone 704 U` — `#A24F56`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 704 U"))]
    Pms704U,
    /// `Pantone 705 C` — `#F5DADF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 705 C"))]
    Pms705C,
    /// `Pantone 705 U` — `#FFDDE2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 705 U"))]
    Pms705U,
    /// `Pantone 706 C` — `#F7CED7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 706 C"))]
    Pms706C,
    /// `Pantone 706 U` — `#FFCAD4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 706 U"))]
    Pms706U,
    /// `Pantone 707 C` — `#F9B5C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 707 C"))]
    Pms707C,
    /// `Pantone 707 U` — `#FFB0BF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 707 U"))]
    Pms707U,
    /// `Pantone 708 C` — `#F890A5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 708 C"))]
    Pms708C,
    /// `Pantone 708 U` — `#FE99A8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 708 U"))]
    Pms708U,
    /// `Pantone 709 C` — `#EF6079`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 709 C"))]
    Pms709C,
    /// `Pantone 709 U` — `#F47C8A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 709 U"))]
    Pms709U,
    /// `Pantone 710 C` — `#E03E52`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 710 C"))]
    Pms710C,
    /// `Pantone 710 U` — `#E4636F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 710 U"))]
    Pms710U,
    /// `Pantone 711 C` — `#CB2C30`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 711 C"))]
    Pms711C,
    /// `Pantone 711 U` — `#CE5055`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 711 U"))]
    Pms711U,
    /// `Pantone 712 C` — `#FCC89B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 712 C"))]
    Pms712C,
    /// `Pantone 712 U` — `#FFCA9D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 712 U"))]
    Pms712U,
    /// `Pantone 713 C` — `#FDBE87`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 713 C"))]
    Pms713C,
    /// `Pantone 713 U` — `#FFC190`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 713 U"))]
    Pms713U,
    /// `Pantone 714 C` — `#FDAA63`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 714 C"))]
    Pms714C,
    /// `Pantone 714 U` — `#FFB07B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 714 U"))]
    Pms714U,
    /// `Pantone 715 C` — `#F68D2E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 715 C"))]
    Pms715C,
    /// `Pantone 715 U` — `#F99963`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 715 U"))]
    Pms715U,
    /// `Pantone 716 C` — `#EA7600`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 716 C"))]
    Pms716C,
    /// `Pantone 716 U` — `#EA8651`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 716 U"))]
    Pms716U,
    /// `Pantone 717 C` — `#D45D00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 717 C"))]
    Pms717C,
    /// `Pantone 717 U` — `#D77540`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 717 U"))]
    Pms717U,
    /// `Pantone 718 C` — `#BE4D00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 718 C"))]
    Pms718C,
    /// `Pantone 718 U` — `#C76C3D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 718 U"))]
    Pms718U,
    /// `Pantone 719 C` — `#EDC8A3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 719 C"))]
    Pms719C,
    /// `Pantone 719 U` — `#F2C6A2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 719 U"))]
    Pms719U,
    /// `Pantone 720 C` — `#E7B78A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 720 C"))]
    Pms720C,
    /// `Pantone 720 U` — `#ECBA93`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 720 U"))]
    Pms720U,
    /// `Pantone 721 C` — `#DDA46F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 721 C"))]
    Pms721C,
    /// `Pantone 721 U` — `#DFA880`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 721 U"))]
    Pms721U,
    /// `Pantone 722 C` — `#C88242`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 722 C"))]
    Pms722C,
    /// `Pantone 722 U` — `#CA916C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 722 U"))]
    Pms722U,
    /// `Pantone 723 C` — `#B36924`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 723 C"))]
    Pms723C,
    /// `Pantone 723 U` — `#B9825F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 723 U"))]
    Pms723U,
    /// `Pantone 724 C` — `#934D11`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 724 C"))]
    Pms724C,
    /// `Pantone 724 U` — `#A57350`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 724 U"))]
    Pms724U,
    /// `Pantone 725 C` — `#7D3F16`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 725 C"))]
    Pms725C,
    /// `Pantone 725 U` — `#946747`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 725 U"))]
    Pms725U,
    /// `Pantone 726 C` — `#E0C09F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 726 C"))]
    Pms726C,
    /// `Pantone 726 U` — `#E8C6A9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 726 U"))]
    Pms726U,
    /// `Pantone 727 C` — `#D9B48F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 727 C"))]
    Pms727C,
    /// `Pantone 727 U` — `#DCB596`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 727 U"))]
    Pms727U,
    /// `Pantone 728 C` — `#CDA077`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 728 C"))]
    Pms728C,
    /// `Pantone 728 U` — `#CAA182`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 728 U"))]
    Pms728U,
    /// `Pantone 729 C` — `#B58150`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 729 C"))]
    Pms729C,
    /// `Pantone 729 U` — `#BB9275`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 729 U"))]
    Pms729U,
    /// `Pantone 730 C` — `#9E652E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 730 C"))]
    Pms730C,
    /// `Pantone 730 U` — `#A57E62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 730 U"))]
    Pms730U,
    /// `Pantone 731 C` — `#774212`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 731 C"))]
    Pms731C,
    /// `Pantone 731 U` — `#957156`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 731 U"))]
    Pms731U,
    /// `Pantone 732 C` — `#623412`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 732 C"))]
    Pms732C,
    /// `Pantone 732 U` — `#88674D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 732 U"))]
    Pms732U,
    /// `Pantone 7401 C` — `#F5E1A4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7401 C"))]
    Pms7401C,
    /// `Pantone 7401 U` — `#FEE5A5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7401 U"))]
    Pms7401U,
    /// `Pantone 7402 C` — `#ECD898`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7402 C"))]
    Pms7402C,
    /// `Pantone 7402 U` — `#F3DA9A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7402 U"))]
    Pms7402U,
    /// `Pantone 7403 C` — `#EED484`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7403 C"))]
    Pms7403C,
    /// `Pantone 7403 U` — `#F4CF81`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7403 U"))]
    Pms7403U,
    /// `Pantone 7404 C` — `#F4DA40`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7404 C"))]
    Pms7404C,
    /// `Pantone 7404 U` — `#FDCF41`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7404 U"))]
    Pms7404U,
    /// `Pantone 7405 C` — `#F2CD00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7405 C"))]
    Pms7405C,
    /// `Pantone 7405 U` — `#E6B10E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7405 U"))]
    Pms7405U,
    /// `Pantone 7406 C` — `#F1C400`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7406 C"))]
    Pms7406C,
    /// `Pantone 7406 U` — `#F1B828`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7406 U"))]
    Pms7406U,
    /// `Pantone 7407 C` — `#CBA052`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7407 C"))]
    Pms7407C,
    /// `Pantone 7407 U` — `#CCA36E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7407 U"))]
    Pms7407U,
    /// `Pantone 7408 C` — `#F6BE00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7408 C"))]
    Pms7408C,
    /// `Pantone 7408 U` — `#EE9D26`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7408 U"))]
    Pms7408U,
    /// `Pantone 7409 C` — `#F0B323`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7409 C"))]
    Pms7409C,
    /// `Pantone 7409 U` — `#F7AD50`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7409 U"))]
    Pms7409U,
    /// `Pantone 7410 C` — `#FEAD77`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7410 C"))]
    Pms7410C,
    /// `Pantone 7410 U` — `#FFA06E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7410 U"))]
    Pms7410U,
    /// `Pantone 7411 C` — `#E6A65D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7411 C"))]
    Pms7411C,
    /// `Pantone 7411 U` — `#E9A16F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7411 U"))]
    Pms7411U,
    /// `Pantone 7412 C` — `#D38235`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7412 C"))]
    Pms7412C,
    /// `Pantone 7412 U` — `#D08C60`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7412 U"))]
    Pms7412U,
    /// `Pantone 7413 C` — `#DC8633`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7413 C"))]
    Pms7413C,
    /// `Pantone 7413 U` — `#DE8958`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7413 U"))]
    Pms7413U,
    /// `Pantone 7414 C` — `#C16C18`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7414 C"))]
    Pms7414C,
    /// `Pantone 7414 U` — `#C07F53`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7414 U"))]
    Pms7414U,
    /// `Pantone 7415 C` — `#E6BAA8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7415 C"))]
    Pms7415C,
    /// `Pantone 7415 U` — `#EBB5A5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7415 U"))]
    Pms7415U,
    /// `Pantone 7416 C` — `#E56A54`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7416 C"))]
    Pms7416C,
    /// `Pantone 7416 U` — `#EA746A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7416 U"))]
    Pms7416U,
    /// `Pantone 7417 C` — `#E04E39`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7417 C"))]
    Pms7417C,
    /// `Pantone 7417 U` — `#E76B61`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7417 U"))]
    Pms7417U,
    /// `Pantone 7418 C` — `#CD545B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7418 C"))]
    Pms7418C,
    /// `Pantone 7418 U` — `#BF6366`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7418 U"))]
    Pms7418U,
    /// `Pantone 7419 C` — `#B04A5A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7419 C"))]
    Pms7419C,
    /// `Pantone 7419 U` — `#A6626B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7419 U"))]
    Pms7419U,
    /// `Pantone 7420 C` — `#9B2242`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7420 C"))]
    Pms7420C,
    /// `Pantone 7420 U` — `#A45768`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7420 U"))]
    Pms7420U,
    /// `Pantone 7421 C` — `#651D32`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7421 C"))]
    Pms7421C,
    /// `Pantone 7421 U` — `#7D4E5B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7421 U"))]
    Pms7421U,
    /// `Pantone 7422 C` — `#F4CDD4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7422 C"))]
    Pms7422C,
    /// `Pantone 7422 U` — `#FDD4DA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7422 U"))]
    Pms7422U,
    /// `Pantone 7423 C` — `#E06287`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7423 C"))]
    Pms7423C,
    /// `Pantone 7423 U` — `#E2758F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7423 U"))]
    Pms7423U,
    /// `Pantone 7424 C` — `#E24585`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7424 C"))]
    Pms7424C,
    /// `Pantone 7424 U` — `#D9577D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7424 U"))]
    Pms7424U,
    /// `Pantone 7425 C` — `#B52555`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7425 C"))]
    Pms7425C,
    /// `Pantone 7425 U` — `#BB5A73`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7425 U"))]
    Pms7425U,
    /// `Pantone 7426 C` — `#A4123F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7426 C"))]
    Pms7426C,
    /// `Pantone 7426 U` — `#B1556A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7426 U"))]
    Pms7426U,
    /// `Pantone 7427 C` — `#971B2F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7427 C"))]
    Pms7427C,
    /// `Pantone 7427 U` — `#A5525C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7427 U"))]
    Pms7427U,
    /// `Pantone 7428 C` — `#6A2C3E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7428 C"))]
    Pms7428C,
    /// `Pantone 7428 U` — `#805C67`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7428 U"))]
    Pms7428U,
    /// `Pantone 7429 C` — `#E2BCCB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7429 C"))]
    Pms7429C,
    /// `Pantone 7429 U` — `#EECCD9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7429 U"))]
    Pms7429U,
    /// `Pantone 7430 C` — `#DCA9BF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7430 C"))]
    Pms7430C,
    /// `Pantone 7430 U` — `#E6B5C9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7430 U"))]
    Pms7430U,
    /// `Pantone 7431 C` — `#C9809E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7431 C"))]
    Pms7431C,
    /// `Pantone 7431 U` — `#DCA2B9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7431 U"))]
    Pms7431U,
    /// `Pantone 7432 C` — `#B55C80`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7432 C"))]
    Pms7432C,
    /// `Pantone 7432 U` — `#C5819A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7432 U"))]
    Pms7432U,
    /// `Pantone 7433 C` — `#A73A64`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7433 C"))]
    Pms7433C,
    /// `Pantone 7433 U` — `#B96F89`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7433 U"))]
    Pms7433U,
    /// `Pantone 7434 C` — `#9B3259`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7434 C"))]
    Pms7434C,
    /// `Pantone 7434 U` — `#A8677D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7434 U"))]
    Pms7434U,
    /// `Pantone 7435 C` — `#872651`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7435 C"))]
    Pms7435C,
    /// `Pantone 7435 U` — `#8C566B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7435 U"))]
    Pms7435U,
    /// `Pantone 7436 C` — `#EEDAEA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7436 C"))]
    Pms7436C,
    /// `Pantone 7436 U` — `#F3D5ED`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7436 U"))]
    Pms7436U,
    /// `Pantone 7437 C` — `#CCAED0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7437 C"))]
    Pms7437C,
    /// `Pantone 7437 U` — `#D4B6D6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7437 U"))]
    Pms7437U,
    /// `Pantone 7438 C` — `#D59ED7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7438 C"))]
    Pms7438C,
    /// `Pantone 7438 U` — `#D79BD7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7438 U"))]
    Pms7438U,
    /// `Pantone 7439 C` — `#B288B9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7439 C"))]
    Pms7439C,
    /// `Pantone 7439 U` — `#C09CC6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7439 U"))]
    Pms7439U,
    /// `Pantone 7440 C` — `#A277A6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7440 C"))]
    Pms7440C,
    /// `Pantone 7440 U` — `#B796BA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7440 U"))]
    Pms7440U,
    /// `Pantone 7441 C` — `#9F5CC0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7441 C"))]
    Pms7441C,
    /// `Pantone 7441 U` — `#AE7ECA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7441 U"))]
    Pms7441U,
    /// `Pantone 7442 C` — `#963CBD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7442 C"))]
    Pms7442C,
    /// `Pantone 7442 U` — `#9E62BE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7442 U"))]
    Pms7442U,
    /// `Pantone 7443 C` — `#DDDAE8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7443 C"))]
    Pms7443C,
    /// `Pantone 7443 U` — `#DDD8EB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7443 U"))]
    Pms7443U,
    /// `Pantone 7444 C` — `#B6B8DC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7444 C"))]
    Pms7444C,
    /// `Pantone 7444 U` — `#BCBDE2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7444 U"))]
    Pms7444U,
    /// `Pantone 7445 C` — `#A7A2C3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7445 C"))]
    Pms7445C,
    /// `Pantone 7445 U` — `#ACA8C7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7445 U"))]
    Pms7445U,
    /// `Pantone 7446 C` — `#8986CA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7446 C"))]
    Pms7446C,
    /// `Pantone 7446 U` — `#9493CF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7446 U"))]
    Pms7446U,
    /// `Pantone 7447 C` — `#5D4777`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7447 C"))]
    Pms7447C,
    /// `Pantone 7447 U` — `#7F7392`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7447 U"))]
    Pms7447U,
    /// `Pantone 7448 C` — `#4B384C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7448 C"))]
    Pms7448C,
    /// `Pantone 7448 U` — `#726874`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7448 U"))]
    Pms7448U,
    /// `Pantone 7449 C` — `#41273B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7449 C"))]
    Pms7449C,
    /// `Pantone 7449 U` — `#63515C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7449 U"))]
    Pms7449U,
    /// `Pantone 7450 C` — `#BDC5DB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7450 C"))]
    Pms7450C,
    /// `Pantone 7450 U` — `#C3CBE0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7450 U"))]
    Pms7450U,
    /// `Pantone 7451 C` — `#89ABE3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7451 C"))]
    Pms7451C,
    /// `Pantone 7451 U` — `#94B3EA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7451 U"))]
    Pms7451U,
    /// `Pantone 7452 C` — `#8094DD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7452 C"))]
    Pms7452C,
    /// `Pantone 7452 U` — `#91A0E4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7452 U"))]
    Pms7452U,
    /// `Pantone 7453 C` — `#7BA6DE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7453 C"))]
    Pms7453C,
    /// `Pantone 7453 U` — `#88ADE3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7453 U"))]
    Pms7453U,
    /// `Pantone 7454 C` — `#5F8FB4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7454 C"))]
    Pms7454C,
    /// `Pantone 7454 U` — `#7599BB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7454 U"))]
    Pms7454U,
    /// `Pantone 7455 C` — `#3A5DAE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7455 C"))]
    Pms7455C,
    /// `Pantone 7455 U` — `#6B7DB9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7455 U"))]
    Pms7455U,
    /// `Pantone 7456 C` — `#606EB2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7456 C"))]
    Pms7456C,
    /// `Pantone 7456 U` — `#8089C0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7456 U"))]
    Pms7456U,
    /// `Pantone 7457 C` — `#BBDDE6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7457 C"))]
    Pms7457C,
    /// `Pantone 7457 U` — `#BFE3ED`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7457 U"))]
    Pms7457U,
    /// `Pantone 7458 C` — `#71B2C9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7458 C"))]
    Pms7458C,
    /// `Pantone 7458 U` — `#67A5BF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7458 U"))]
    Pms7458U,
    /// `Pantone 7459 C` — `#4298B5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7459 C"))]
    Pms7459C,
    /// `Pantone 7459 U` — `#518EAA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7459 U"))]
    Pms7459U,
    /// `Pantone 7460 C` — `#0086BF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7460 C"))]
    Pms7460C,
    /// `Pantone 7460 U` — `#008EC0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7460 U"))]
    Pms7460U,
    /// `Pantone 7461 C` — `#007DBA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7461 C"))]
    Pms7461C,
    /// `Pantone 7461 U` — `#4492C6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7461 U"))]
    Pms7461U,
    /// `Pantone 7462 C` — `#00558C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7462 C"))]
    Pms7462C,
    /// `Pantone 7462 U` — `#547AA3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7462 U"))]
    Pms7462U,
    /// `Pantone 7463 C` — `#002B49`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7463 C"))]
    Pms7463C,
    /// `Pantone 7463 U` — `#51657D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7463 U"))]
    Pms7463U,
    /// `Pantone 7464 C` — `#A0D1CA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7464 C"))]
    Pms7464C,
    /// `Pantone 7464 U` — `#AADBD7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7464 U"))]
    Pms7464U,
    /// `Pantone 7465 C` — `#40C1AC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7465 C"))]
    Pms7465C,
    /// `Pantone 7465 U` — `#55C7B5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7465 U"))]
    Pms7465U,
    /// `Pantone 7466 C` — `#00B0B9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7466 C"))]
    Pms7466C,
    /// `Pantone 7466 U` — `#00B4BC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7466 U"))]
    Pms7466U,
    /// `Pantone 7467 C` — `#00A3AD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7467 C"))]
    Pms7467C,
    /// `Pantone 7467 U` — `#00A8B0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7467 U"))]
    Pms7467U,
    /// `Pantone 7468 C` — `#007398`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7468 C"))]
    Pms7468C,
    /// `Pantone 7468 U` — `#3F7793`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7468 U"))]
    Pms7468U,
    /// `Pantone 7469 C` — `#005F86`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7469 C"))]
    Pms7469C,
    /// `Pantone 7469 U` — `#3C6E89`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7469 U"))]
    Pms7469U,
    /// `Pantone 7470 C` — `#005A70`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7470 C"))]
    Pms7470C,
    /// `Pantone 7470 U` — `#437083`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7470 U"))]
    Pms7470U,
    /// `Pantone 7471 C` — `#7EDDD3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7471 C"))]
    Pms7471C,
    /// `Pantone 7471 U` — `#7FE2D8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7471 U"))]
    Pms7471U,
    /// `Pantone 7472 C` — `#5CB8B2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7472 C"))]
    Pms7472C,
    /// `Pantone 7472 U` — `#6BBDB9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7472 U"))]
    Pms7472U,
    /// `Pantone 7473 C` — `#279989`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7473 C"))]
    Pms7473C,
    /// `Pantone 7473 U` — `#5EA59A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7473 U"))]
    Pms7473U,
    /// `Pantone 7474 C` — `#007681`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7474 C"))]
    Pms7474C,
    /// `Pantone 7474 U` — `#458A94`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7474 U"))]
    Pms7474U,
    /// `Pantone 7475 C` — `#487A7B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7475 C"))]
    Pms7475C,
    /// `Pantone 7475 U` — `#6B8B8D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7475 U"))]
    Pms7475U,
    /// `Pantone 7476 C` — `#0D5257`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7476 C"))]
    Pms7476C,
    /// `Pantone 7476 U` — `#5A777B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7476 U"))]
    Pms7476U,
    /// `Pantone 7477 C` — `#244C5A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7477 C"))]
    Pms7477C,
    /// `Pantone 7477 U` — `#5D7079`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7477 U"))]
    Pms7477U,
    /// `Pantone 7478 C` — `#9BE3BF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7478 C"))]
    Pms7478C,
    /// `Pantone 7478 U` — `#A0EAC5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7478 U"))]
    Pms7478U,
    /// `Pantone 7479 C` — `#26D07C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7479 C"))]
    Pms7479C,
    /// `Pantone 7479 U` — `#29CB7E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7479 U"))]
    Pms7479U,
    /// `Pantone 7480 C` — `#00BF6F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7480 C"))]
    Pms7480C,
    /// `Pantone 7480 U` — `#00C281`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7480 U"))]
    Pms7480U,
    /// `Pantone 7481 C` — `#00B74F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7481 C"))]
    Pms7481C,
    /// `Pantone 7481 U` — `#00BA6E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7481 U"))]
    Pms7481U,
    /// `Pantone 7482 C` — `#009F4D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7482 C"))]
    Pms7482C,
    /// `Pantone 7482 U` — `#27A770`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7482 U"))]
    Pms7482U,
    /// `Pantone 7483 C` — `#275D38`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7483 C"))]
    Pms7483C,
    /// `Pantone 7483 U` — `#587762`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7483 U"))]
    Pms7483U,
    /// `Pantone 7484 C` — `#00573F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7484 C"))]
    Pms7484C,
    /// `Pantone 7484 U` — `#456E60`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7484 U"))]
    Pms7484U,
    /// `Pantone 7485 C` — `#D0DEBB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7485 C"))]
    Pms7485C,
    /// `Pantone 7485 U` — `#D5E4C0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7485 U"))]
    Pms7485U,
    /// `Pantone 7486 C` — `#BCE194`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7486 C"))]
    Pms7486C,
    /// `Pantone 7486 U` — `#B7E394`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7486 U"))]
    Pms7486U,
    /// `Pantone 7487 C` — `#8EDD65`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7487 C"))]
    Pms7487C,
    /// `Pantone 7487 U` — `#8BE076`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7487 U"))]
    Pms7487U,
    /// `Pantone 7488 C` — `#78D64B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7488 C"))]
    Pms7488C,
    /// `Pantone 7488 U` — `#63D05A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7488 U"))]
    Pms7488U,
    /// `Pantone 7489 C` — `#74AA50`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7489 C"))]
    Pms7489C,
    /// `Pantone 7489 U` — `#7AAB6D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7489 U"))]
    Pms7489U,
    /// `Pantone 7490 C` — `#719949`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7490 C"))]
    Pms7490C,
    /// `Pantone 7490 U` — `#7B9E6B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7490 U"))]
    Pms7490U,
    /// `Pantone 7491 C` — `#79863C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7491 C"))]
    Pms7491C,
    /// `Pantone 7491 U` — `#818C61`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7491 U"))]
    Pms7491U,
    /// `Pantone 7492 C` — `#BFCC80`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7492 C"))]
    Pms7492C,
    /// `Pantone 7492 U` — `#BCCA87`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7492 U"))]
    Pms7492U,
    /// `Pantone 7493 C` — `#BBC592`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7493 C"))]
    Pms7493C,
    /// `Pantone 7493 U` — `#B7C194`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7493 U"))]
    Pms7493U,
    /// `Pantone 7494 C` — `#9CAF88`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7494 C"))]
    Pms7494C,
    /// `Pantone 7494 U` — `#9EB091`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7494 U"))]
    Pms7494U,
    /// `Pantone 7495 C` — `#8F993E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7495 C"))]
    Pms7495C,
    /// `Pantone 7495 U` — `#838E59`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7495 U"))]
    Pms7495U,
    /// `Pantone 7496 C` — `#76881D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7496 C"))]
    Pms7496C,
    /// `Pantone 7496 U` — `#758448`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7496 U"))]
    Pms7496U,
    /// `Pantone 7497 C` — `#7A7256`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7497 C"))]
    Pms7497C,
    /// `Pantone 7497 U` — `#8C8778`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7497 U"))]
    Pms7497U,
    /// `Pantone 7498 C` — `#5B6236`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7498 C"))]
    Pms7498C,
    /// `Pantone 7498 U` — `#777D64`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7498 U"))]
    Pms7498U,
    /// `Pantone 7499 C` — `#F1E6B2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7499 C"))]
    Pms7499C,
    /// `Pantone 7499 U` — `#FAEDBC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7499 U"))]
    Pms7499U,
    /// `Pantone 7500 C` — `#DFD1A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7500 C"))]
    Pms7500C,
    /// `Pantone 7500 U` — `#E6D5AB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7500 U"))]
    Pms7500U,
    /// `Pantone 7501 C` — `#D9C89E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7501 C"))]
    Pms7501C,
    /// `Pantone 7501 U` — `#DECBA5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7501 U"))]
    Pms7501U,
    /// `Pantone 7502 C` — `#CEB888`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7502 C"))]
    Pms7502C,
    /// `Pantone 7502 U` — `#C7AF87`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7502 U"))]
    Pms7502U,
    /// `Pantone 7503 C` — `#A89968`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7503 C"))]
    Pms7503C,
    /// `Pantone 7503 U` — `#9D9272`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7503 U"))]
    Pms7503U,
    /// `Pantone 7504 C` — `#94795D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7504 C"))]
    Pms7504C,
    /// `Pantone 7504 U` — `#948272`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7504 U"))]
    Pms7504U,
    /// `Pantone 7505 C` — `#816040`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7505 C"))]
    Pms7505C,
    /// `Pantone 7505 U` — `#887566`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7505 U"))]
    Pms7505U,
    /// `Pantone 7506 C` — `#EFDBB2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7506 C"))]
    Pms7506C,
    /// `Pantone 7506 U` — `#F9E1B7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7506 U"))]
    Pms7506U,
    /// `Pantone 7507 C` — `#FCD299`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7507 C"))]
    Pms7507C,
    /// `Pantone 7507 U` — `#FFDDAA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7507 U"))]
    Pms7507U,
    /// `Pantone 7508 C` — `#E1B87F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7508 C"))]
    Pms7508C,
    /// `Pantone 7508 U` — `#E0B384`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7508 U"))]
    Pms7508U,
    /// `Pantone 7509 C` — `#D6A461`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7509 C"))]
    Pms7509C,
    /// `Pantone 7509 U` — `#D2A273`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7509 U"))]
    Pms7509U,
    /// `Pantone 7510 C` — `#C6893F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7510 C"))]
    Pms7510C,
    /// `Pantone 7510 U` — `#BE8B60`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7510 U"))]
    Pms7510U,
    /// `Pantone 7511 C` — `#B77729`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7511 C"))]
    Pms7511C,
    /// `Pantone 7511 U` — `#A77750`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7511 U"))]
    Pms7511U,
    /// `Pantone 7512 C` — `#A6631B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7512 C"))]
    Pms7512C,
    /// `Pantone 7512 U` — `#966B47`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7512 U"))]
    Pms7512U,
    /// `Pantone 7513 C` — `#E1B7A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7513 C"))]
    Pms7513C,
    /// `Pantone 7513 U` — `#E7BBB4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7513 U"))]
    Pms7513U,
    /// `Pantone 7514 C` — `#D5A286`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7514 C"))]
    Pms7514C,
    /// `Pantone 7514 U` — `#DAAB9C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7514 U"))]
    Pms7514U,
    /// `Pantone 7515 C` — `#C58B68`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7515 C"))]
    Pms7515C,
    /// `Pantone 7515 U` — `#C2907F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7515 U"))]
    Pms7515U,
    /// `Pantone 7516 C` — `#99552B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7516 C"))]
    Pms7516C,
    /// `Pantone 7516 U` — `#A07462`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7516 U"))]
    Pms7516U,
    /// `Pantone 7517 C` — `#85431E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7517 C"))]
    Pms7517C,
    /// `Pantone 7517 U` — `#936957`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7517 U"))]
    Pms7517U,
    /// `Pantone 7518 C` — `#6D4F47`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7518 C"))]
    Pms7518C,
    /// `Pantone 7518 U` — `#817172`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7518 U"))]
    Pms7518U,
    /// `Pantone 7519 C` — `#5E4B3C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7519 C"))]
    Pms7519C,
    /// `Pantone 7519 U` — `#756B66`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7519 U"))]
    Pms7519U,
    /// `Pantone 7520 C` — `#EABEB0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7520 C"))]
    Pms7520C,
    /// `Pantone 7520 U` — `#F3C4BF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7520 U"))]
    Pms7520U,
    /// `Pantone 7521 C` — `#C09C83`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7521 C"))]
    Pms7521C,
    /// `Pantone 7521 U` — `#C4A195`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7521 U"))]
    Pms7521U,
    /// `Pantone 7522 C` — `#B46A55`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7522 C"))]
    Pms7522C,
    /// `Pantone 7522 U` — `#AF7871`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7522 U"))]
    Pms7522U,
    /// `Pantone 7523 C` — `#AB5C57`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7523 C"))]
    Pms7523C,
    /// `Pantone 7523 U` — `#A97576`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7523 U"))]
    Pms7523U,
    /// `Pantone 7524 C` — `#A45248`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7524 C"))]
    Pms7524C,
    /// `Pantone 7524 U` — `#A36D6A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7524 U"))]
    Pms7524U,
    /// `Pantone 7525 C` — `#9A6A4F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7525 C"))]
    Pms7525C,
    /// `Pantone 7525 U` — `#997970`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7525 U"))]
    Pms7525U,
    /// `Pantone 7526 C` — `#8A391B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7526 C"))]
    Pms7526C,
    /// `Pantone 7526 U` — `#925F52`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7526 U"))]
    Pms7526U,
    /// `Pantone 7527 C` — `#D6D2C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7527 C"))]
    Pms7527C,
    /// `Pantone 7527 U` — `#DCD7D2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7527 U"))]
    Pms7527U,
    /// `Pantone 7528 C` — `#C5B9AC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7528 C"))]
    Pms7528C,
    /// `Pantone 7528 U` — `#D2C7C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7528 U"))]
    Pms7528U,
    /// `Pantone 7529 C` — `#B7A99A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7529 C"))]
    Pms7529C,
    /// `Pantone 7529 U` — `#BBAFA9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7529 U"))]
    Pms7529U,
    /// `Pantone 7530 C` — `#A39382`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7530 C"))]
    Pms7530C,
    /// `Pantone 7530 U` — `#A99C95`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7530 U"))]
    Pms7530U,
    /// `Pantone 7531 C` — `#7A6855`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7531 C"))]
    Pms7531C,
    /// `Pantone 7531 U` — `#8B7F79`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7531 U"))]
    Pms7531U,
    /// `Pantone 7532 C` — `#63513D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7532 C"))]
    Pms7532C,
    /// `Pantone 7532 U` — `#786E66`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7532 U"))]
    Pms7532U,
    /// `Pantone 7533 C` — `#473729`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7533 C"))]
    Pms7533C,
    /// `Pantone 7533 U` — `#6E645C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7533 U"))]
    Pms7533U,
    /// `Pantone 7534 C` — `#D1CCBD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7534 C"))]
    Pms7534C,
    /// `Pantone 7534 U` — `#D0CBC3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7534 U"))]
    Pms7534U,
    /// `Pantone 7535 C` — `#B7B09C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7535 C"))]
    Pms7535C,
    /// `Pantone 7535 U` — `#C0BAB0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7535 U"))]
    Pms7535U,
    /// `Pantone 7536 C` — `#A69F88`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7536 C"))]
    Pms7536C,
    /// `Pantone 7536 U` — `#AFA89D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7536 U"))]
    Pms7536U,
    /// `Pantone 7537 C` — `#A7ACA2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7537 C"))]
    Pms7537C,
    /// `Pantone 7537 U` — `#B4B8B6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7537 U"))]
    Pms7537U,
    /// `Pantone 7538 C` — `#949A90`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7538 C"))]
    Pms7538C,
    /// `Pantone 7538 U` — `#A5AAA8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7538 U"))]
    Pms7538U,
    /// `Pantone 7539 C` — `#8E9089`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7539 C"))]
    Pms7539C,
    /// `Pantone 7539 U` — `#9C9EA0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7539 U"))]
    Pms7539U,
    /// `Pantone 7540 C` — `#4B4F54`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7540 C"))]
    Pms7540C,
    /// `Pantone 7540 U` — `#70747C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7540 U"))]
    Pms7540U,
    /// `Pantone 7541 C` — `#D9E1E2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7541 C"))]
    Pms7541C,
    /// `Pantone 7541 U` — `#DDE4E6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7541 U"))]
    Pms7541U,
    /// `Pantone 7542 C` — `#A4BCC2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7542 C"))]
    Pms7542C,
    /// `Pantone 7542 U` — `#B0C5CC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7542 U"))]
    Pms7542U,
    /// `Pantone 7543 C` — `#98A4AE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7543 C"))]
    Pms7543C,
    /// `Pantone 7543 U` — `#AEB8C0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7543 U"))]
    Pms7543U,
    /// `Pantone 7544 C` — `#768692`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7544 C"))]
    Pms7544C,
    /// `Pantone 7544 U` — `#9AA5AF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7544 U"))]
    Pms7544U,
    /// `Pantone 7545 C` — `#425563`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7545 C"))]
    Pms7545C,
    /// `Pantone 7545 U` — `#78838E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7545 U"))]
    Pms7545U,
    /// `Pantone 7546 C` — `#253746`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7546 C"))]
    Pms7546C,
    /// `Pantone 7546 U` — `#67707A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7546 U"))]
    Pms7546U,
    /// `Pantone 7547 C` — `#131E29`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7547 C"))]
    Pms7547C,
    /// `Pantone 7547 U` — `#51545D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7547 U"))]
    Pms7547U,
    /// `Pantone 7548 C` — `#FFC600`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7548 C"))]
    Pms7548C,
    /// `Pantone 7548 U` — `#FFC915`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7548 U"))]
    Pms7548U,
    /// `Pantone 7549 C` — `#FFB500`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7549 C"))]
    Pms7549C,
    /// `Pantone 7549 U` — `#FFB838`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7549 U"))]
    Pms7549U,
    /// `Pantone 7550 C` — `#D19000`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7550 C"))]
    Pms7550C,
    /// `Pantone 7550 U` — `#CE973D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7550 U"))]
    Pms7550U,
    /// `Pantone 7551 C` — `#B47E00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7551 C"))]
    Pms7551C,
    /// `Pantone 7551 U` — `#AE8643`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7551 U"))]
    Pms7551U,
    /// `Pantone 7552 C` — `#73531D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7552 C"))]
    Pms7552C,
    /// `Pantone 7552 U` — `#826F4B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7552 U"))]
    Pms7552U,
    /// `Pantone 7553 C` — `#5A4522`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7553 C"))]
    Pms7553C,
    /// `Pantone 7553 U` — `#70654B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7553 U"))]
    Pms7553U,
    /// `Pantone 7554 C` — `#4B3D2A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7554 C"))]
    Pms7554C,
    /// `Pantone 7554 U` — `#6C6558`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7554 U"))]
    Pms7554U,
    /// `Pantone 7555 C` — `#D29F13`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7555 C"))]
    Pms7555C,
    /// `Pantone 7555 U` — `#D7A95A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7555 U"))]
    Pms7555U,
    /// `Pantone 7556 C` — `#B78B20`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7556 C"))]
    Pms7556C,
    /// `Pantone 7556 U` — `#BC995D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7556 U"))]
    Pms7556U,
    /// `Pantone 7557 C` — `#9F7D23`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7557 C"))]
    Pms7557C,
    /// `Pantone 7557 U` — `#A58D5F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7557 U"))]
    Pms7557U,
    /// `Pantone 7558 C` — `#967126`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7558 C"))]
    Pms7558C,
    /// `Pantone 7558 U` — `#9B835E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7558 U"))]
    Pms7558U,
    /// `Pantone 7559 C` — `#8F6A2A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7559 C"))]
    Pms7559C,
    /// `Pantone 7559 U` — `#977F5E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7559 U"))]
    Pms7559U,
    /// `Pantone 7560 C` — `#7D622E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7560 C"))]
    Pms7560C,
    /// `Pantone 7560 U` — `#8E7E62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7560 U"))]
    Pms7560U,
    /// `Pantone 7561 C` — `#6C5D34`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7561 C"))]
    Pms7561C,
    /// `Pantone 7561 U` — `#807964`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7561 U"))]
    Pms7561U,
    /// `Pantone 7562 C` — `#BD9B60`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7562 C"))]
    Pms7562C,
    /// `Pantone 7562 U` — `#BDA078`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7562 U"))]
    Pms7562U,
    /// `Pantone 7563 C` — `#D69A2D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7563 C"))]
    Pms7563C,
    /// `Pantone 7563 U` — `#DAA563`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7563 U"))]
    Pms7563U,
    /// `Pantone 7564 C` — `#DB8A06`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7564 C"))]
    Pms7564C,
    /// `Pantone 7564 U` — `#DE9858`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7564 U"))]
    Pms7564U,
    /// `Pantone 7565 C` — `#CD7925`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7565 C"))]
    Pms7565C,
    /// `Pantone 7565 U` — `#C98659`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7565 U"))]
    Pms7565U,
    /// `Pantone 7566 C` — `#AD6433`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7566 C"))]
    Pms7566C,
    /// `Pantone 7566 U` — `#AE7B5E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7566 U"))]
    Pms7566U,
    /// `Pantone 7567 C` — `#89532F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7567 C"))]
    Pms7567C,
    /// `Pantone 7567 U` — `#90705C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7567 U"))]
    Pms7567U,
    /// `Pantone 7568 C` — `#775135`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7568 C"))]
    Pms7568C,
    /// `Pantone 7568 U` — `#826D5E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7568 U"))]
    Pms7568U,
    /// `Pantone 7569 C` — `#D78825`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7569 C"))]
    Pms7569C,
    /// `Pantone 7569 U` — `#D28E56`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7569 U"))]
    Pms7569U,
    /// `Pantone 7570 C` — `#D3832B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7570 C"))]
    Pms7570C,
    /// `Pantone 7570 U` — `#D18D5B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7570 U"))]
    Pms7570U,
    /// `Pantone 7571 C` — `#C67D30`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7571 C"))]
    Pms7571C,
    /// `Pantone 7571 U` — `#C3895D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7571 U"))]
    Pms7571U,
    /// `Pantone 7572 C` — `#B67233`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7572 C"))]
    Pms7572C,
    /// `Pantone 7572 U` — `#B17F5B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7572 U"))]
    Pms7572U,
    /// `Pantone 7573 C` — `#A7662B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7573 C"))]
    Pms7573C,
    /// `Pantone 7573 U` — `#A87B59`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7573 U"))]
    Pms7573U,
    /// `Pantone 7574 C` — `#9E6A38`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7574 C"))]
    Pms7574C,
    /// `Pantone 7574 U` — `#99775B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7574 U"))]
    Pms7574U,
    /// `Pantone 7575 C` — `#835D32`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7575 C"))]
    Pms7575C,
    /// `Pantone 7575 U` — `#86715A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7575 U"))]
    Pms7575U,
    /// `Pantone 7576 C` — `#DB864E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7576 C"))]
    Pms7576C,
    /// `Pantone 7576 U` — `#DA8E6B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7576 U"))]
    Pms7576U,
    /// `Pantone 7577 C` — `#E07E3C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7577 C"))]
    Pms7577C,
    /// `Pantone 7577 U` — `#E18B63`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7577 U"))]
    Pms7577U,
    /// `Pantone 7578 C` — `#DC6B2F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7578 C"))]
    Pms7578C,
    /// `Pantone 7578 U` — `#DB7D59`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7578 U"))]
    Pms7578U,
    /// `Pantone 7579 C` — `#DC582A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7579 C"))]
    Pms7579C,
    /// `Pantone 7579 U` — `#DF7357`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7579 U"))]
    Pms7579U,
    /// `Pantone 7580 C` — `#C05131`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7580 C"))]
    Pms7580C,
    /// `Pantone 7580 U` — `#C06D59`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7580 U"))]
    Pms7580U,
    /// `Pantone 7581 C` — `#864A33`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7581 C"))]
    Pms7581C,
    /// `Pantone 7581 U` — `#916B5D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7581 U"))]
    Pms7581U,
    /// `Pantone 7582 C` — `#674736`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7582 C"))]
    Pms7582C,
    /// `Pantone 7582 U` — `#7A685E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7582 U"))]
    Pms7582U,
    /// `Pantone 7583 C` — `#C4622D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7583 C"))]
    Pms7583C,
    /// `Pantone 7583 U` — `#CA7D5F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7583 U"))]
    Pms7583U,
    /// `Pantone 7584 C` — `#BA5826`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7584 C"))]
    Pms7584C,
    /// `Pantone 7584 U` — `#C07759`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7584 U"))]
    Pms7584U,
    /// `Pantone 7585 C` — `#AF5C37`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7585 C"))]
    Pms7585C,
    /// `Pantone 7585 U` — `#B67965`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7585 U"))]
    Pms7585U,
    /// `Pantone 7586 C` — `#9E5330`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7586 C"))]
    Pms7586C,
    /// `Pantone 7586 U` — `#A2715D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7586 U"))]
    Pms7586U,
    /// `Pantone 7587 C` — `#924C2E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7587 C"))]
    Pms7587C,
    /// `Pantone 7587 U` — `#996E5C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7587 U"))]
    Pms7587U,
    /// `Pantone 7588 C` — `#7B4D35`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7588 C"))]
    Pms7588C,
    /// `Pantone 7588 U` — `#896E61`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7588 U"))]
    Pms7588U,
    /// `Pantone 7589 C` — `#5C4738`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7589 C"))]
    Pms7589C,
    /// `Pantone 7589 U` — `#756A63`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7589 U"))]
    Pms7589U,
    /// `Pantone 7590 C` — `#D4B59E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7590 C"))]
    Pms7590C,
    /// `Pantone 7590 U` — `#DBBCA9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7590 U"))]
    Pms7590U,
    /// `Pantone 7591 C` — `#C07D59`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7591 C"))]
    Pms7591C,
    /// `Pantone 7591 U` — `#C68D78`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7591 U"))]
    Pms7591U,
    /// `Pantone 7592 C` — `#B15533`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7592 C"))]
    Pms7592C,
    /// `Pantone 7592 U` — `#B47260`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7592 U"))]
    Pms7592U,
    /// `Pantone 7593 C` — `#9D432C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7593 C"))]
    Pms7593C,
    /// `Pantone 7593 U` — `#A26859`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7593 U"))]
    Pms7593U,
    /// `Pantone 7594 C` — `#7C3A2D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7594 C"))]
    Pms7594C,
    /// `Pantone 7594 U` — `#8B645A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7594 U"))]
    Pms7594U,
    /// `Pantone 7595 C` — `#6B3D2E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7595 C"))]
    Pms7595C,
    /// `Pantone 7595 U` — `#80665C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7595 U"))]
    Pms7595U,
    /// `Pantone 7596 C` — `#5C3D31`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7596 C"))]
    Pms7596C,
    /// `Pantone 7596 U` — `#776760`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7596 U"))]
    Pms7596U,
    /// `Pantone 7597 C` — `#D14124`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7597 C"))]
    Pms7597C,
    /// `Pantone 7597 U` — `#D66756`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7597 U"))]
    Pms7597U,
    /// `Pantone 7598 C` — `#BD472A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7598 C"))]
    Pms7598C,
    /// `Pantone 7598 U` — `#C36D5C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7598 U"))]
    Pms7598U,
    /// `Pantone 7599 C` — `#B33D26`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7599 C"))]
    Pms7599C,
    /// `Pantone 7599 U` — `#BD695A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7599 U"))]
    Pms7599U,
    /// `Pantone 7600 C` — `#8D3F2B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7600 C"))]
    Pms7600C,
    /// `Pantone 7600 U` — `#9B6A5E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7600 U"))]
    Pms7600U,
    /// `Pantone 7601 C` — `#83412C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7601 C"))]
    Pms7601C,
    /// `Pantone 7601 U` — `#92695C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7601 U"))]
    Pms7601U,
    /// `Pantone 7602 C` — `#7B4931`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7602 C"))]
    Pms7602C,
    /// `Pantone 7602 U` — `#8D6E61`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7602 U"))]
    Pms7602U,
    /// `Pantone 7603 C` — `#674230`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7603 C"))]
    Pms7603C,
    /// `Pantone 7603 U` — `#7C675D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7603 U"))]
    Pms7603U,
    /// `Pantone 7604 C` — `#E4D5D3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7604 C"))]
    Pms7604C,
    /// `Pantone 7604 U` — `#EAD8D6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7604 U"))]
    Pms7604U,
    /// `Pantone 7605 C` — `#E1BBB4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7605 C"))]
    Pms7605C,
    /// `Pantone 7605 U` — `#EAC4BE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7605 U"))]
    Pms7605U,
    /// `Pantone 7606 C` — `#D6938A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7606 C"))]
    Pms7606C,
    /// `Pantone 7606 U` — `#DEA09A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7606 U"))]
    Pms7606U,
    /// `Pantone 7607 C` — `#C26E60`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7607 C"))]
    Pms7607C,
    /// `Pantone 7607 U` — `#CB867F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7607 U"))]
    Pms7607U,
    /// `Pantone 7608 C` — `#A4493D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7608 C"))]
    Pms7608C,
    /// `Pantone 7608 U` — `#AB6E67`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7608 U"))]
    Pms7608U,
    /// `Pantone 7609 C` — `#823B34`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7609 C"))]
    Pms7609C,
    /// `Pantone 7609 U` — `#926660`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7609 U"))]
    Pms7609U,
    /// `Pantone 7610 C` — `#683431`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7610 C"))]
    Pms7610C,
    /// `Pantone 7610 U` — `#80615E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7610 U"))]
    Pms7610U,
    /// `Pantone 7611 C` — `#DDBCB0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7611 C"))]
    Pms7611C,
    /// `Pantone 7611 U` — `#E7C8BE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7611 U"))]
    Pms7611U,
    /// `Pantone 7612 C` — `#CA9A8E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7612 C"))]
    Pms7612C,
    /// `Pantone 7612 U` — `#D4AAA0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7612 U"))]
    Pms7612U,
    /// `Pantone 7613 C` — `#BC8A7E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7613 C"))]
    Pms7613C,
    /// `Pantone 7613 U` — `#C89F97`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7613 U"))]
    Pms7613U,
    /// `Pantone 7614 C` — `#A37F74`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7614 C"))]
    Pms7614C,
    /// `Pantone 7614 U` — `#AE928C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7614 U"))]
    Pms7614U,
    /// `Pantone 7615 C` — `#866761`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7615 C"))]
    Pms7615C,
    /// `Pantone 7615 U` — `#978480`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7615 U"))]
    Pms7615U,
    /// `Pantone 7616 C` — `#6B4C4C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7616 C"))]
    Pms7616C,
    /// `Pantone 7616 U` — `#867474`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7616 U"))]
    Pms7616U,
    /// `Pantone 7617 C` — `#583D3E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7617 C"))]
    Pms7617C,
    /// `Pantone 7617 U` — `#756667`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7617 U"))]
    Pms7617U,
    /// `Pantone 7618 C` — `#C66E4E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7618 C"))]
    Pms7618C,
    /// `Pantone 7618 U` — `#D08A77`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7618 U"))]
    Pms7618U,
    /// `Pantone 7619 C` — `#C04C36`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7619 C"))]
    Pms7619C,
    /// `Pantone 7619 U` — `#C87164`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7619 U"))]
    Pms7619U,
    /// `Pantone 7620 C` — `#B7312C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7620 C"))]
    Pms7620C,
    /// `Pantone 7620 U` — `#C0615B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7620 U"))]
    Pms7620U,
    /// `Pantone 7621 C` — `#AB2328`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7621 C"))]
    Pms7621C,
    /// `Pantone 7621 U` — `#BA5D5A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7621 U"))]
    Pms7621U,
    /// `Pantone 7622 C` — `#93272C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7622 C"))]
    Pms7622C,
    /// `Pantone 7622 U` — `#A55D5C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7622 U"))]
    Pms7622U,
    /// `Pantone 7623 C` — `#8A2A2B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7623 C"))]
    Pms7623C,
    /// `Pantone 7623 U` — `#9B5C57`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7623 U"))]
    Pms7623U,
    /// `Pantone 7624 C` — `#802F2D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7624 C"))]
    Pms7624C,
    /// `Pantone 7624 U` — `#8D5A54`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7624 U"))]
    Pms7624U,
    /// `Pantone 7625 C` — `#E1523D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7625 C"))]
    Pms7625C,
    /// `Pantone 7625 U` — `#EA776C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7625 U"))]
    Pms7625U,
    /// `Pantone 7626 C` — `#C63527`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7626 C"))]
    Pms7626C,
    /// `Pantone 7626 U` — `#D2625F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7626 U"))]
    Pms7626U,
    /// `Pantone 7627 C` — `#A72B2A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7627 C"))]
    Pms7627C,
    /// `Pantone 7627 U` — `#AD5857`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7627 U"))]
    Pms7627U,
    /// `Pantone 7628 C` — `#9E2A2B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7628 C"))]
    Pms7628C,
    /// `Pantone 7628 U` — `#A85959`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7628 U"))]
    Pms7628U,
    /// `Pantone 7629 C` — `#6D3332`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7629 C"))]
    Pms7629C,
    /// `Pantone 7629 U` — `#805B5A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7629 U"))]
    Pms7629U,
    /// `Pantone 7630 C` — `#633231`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7630 C"))]
    Pms7630C,
    /// `Pantone 7630 U` — `#765957`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7630 U"))]
    Pms7630U,
    /// `Pantone 7631 C` — `#572D2D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7631 C"))]
    Pms7631C,
    /// `Pantone 7631 U` — `#6E5555`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7631 U"))]
    Pms7631U,
    /// `Pantone 7632 C` — `#D6C9CA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7632 C"))]
    Pms7632C,
    /// `Pantone 7632 U` — `#DDCFD0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7632 U"))]
    Pms7632U,
    /// `Pantone 7633 C` — `#C4A4A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7633 C"))]
    Pms7633C,
    /// `Pantone 7633 U` — `#C9AAAE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7633 U"))]
    Pms7633U,
    /// `Pantone 7634 C` — `#C16784`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7634 C"))]
    Pms7634C,
    /// `Pantone 7634 U` — `#C77D92`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7634 U"))]
    Pms7634U,
    /// `Pantone 7635 C` — `#C63663`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7635 C"))]
    Pms7635C,
    /// `Pantone 7635 U` — `#CD627D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7635 U"))]
    Pms7635U,
    /// `Pantone 7636 C` — `#BC204B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7636 C"))]
    Pms7636C,
    /// `Pantone 7636 U` — `#C3566D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7636 U"))]
    Pms7636U,
    /// `Pantone 7637 C` — `#912F46`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7637 C"))]
    Pms7637C,
    /// `Pantone 7637 U` — `#955864`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7637 U"))]
    Pms7637U,
    /// `Pantone 7638 C` — `#7E2D40`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7638 C"))]
    Pms7638C,
    /// `Pantone 7638 U` — `#855660`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7638 U"))]
    Pms7638U,
    /// `Pantone 7639 C` — `#936D73`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7639 C"))]
    Pms7639C,
    /// `Pantone 7639 U` — `#A0858A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7639 U"))]
    Pms7639U,
    /// `Pantone 7640 C` — `#934054`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7640 C"))]
    Pms7640C,
    /// `Pantone 7640 U` — `#9B6673`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7640 U"))]
    Pms7640U,
    /// `Pantone 7641 C` — `#8E2C48`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7641 C"))]
    Pms7641C,
    /// `Pantone 7641 U` — `#9A5D6D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7641 U"))]
    Pms7641U,
    /// `Pantone 7642 C` — `#732E4A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7642 C"))]
    Pms7642C,
    /// `Pantone 7642 U` — `#7E5867`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7642 U"))]
    Pms7642U,
    /// `Pantone 7643 C` — `#672E45`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7643 C"))]
    Pms7643C,
    /// `Pantone 7643 U` — `#7A5A67`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7643 U"))]
    Pms7643U,
    /// `Pantone 7644 C` — `#582D40`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7644 C"))]
    Pms7644C,
    /// `Pantone 7644 U` — `#715862`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7644 U"))]
    Pms7644U,
    /// `Pantone 7645 C` — `#502B3A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7645 C"))]
    Pms7645C,
    /// `Pantone 7645 U` — `#6C5760`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7645 U"))]
    Pms7645U,
    /// `Pantone 7646 C` — `#A56E87`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7646 C"))]
    Pms7646C,
    /// `Pantone 7646 U` — `#B08799`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7646 U"))]
    Pms7646U,
    /// `Pantone 7647 C` — `#A83D72`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7647 C"))]
    Pms7647C,
    /// `Pantone 7647 U` — `#B16889`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7647 U"))]
    Pms7647U,
    /// `Pantone 7648 C` — `#991E66`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7648 C"))]
    Pms7648C,
    /// `Pantone 7648 U` — `#9E537B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7648 U"))]
    Pms7648U,
    /// `Pantone 7649 C` — `#8A1B61`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7649 C"))]
    Pms7649C,
    /// `Pantone 7649 U` — `#935379`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7649 U"))]
    Pms7649U,
    /// `Pantone 7650 C` — `#722257`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7650 C"))]
    Pms7650C,
    /// `Pantone 7650 U` — `#835573`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7650 U"))]
    Pms7650U,
    /// `Pantone 7651 C` — `#6A2A5B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7651 C"))]
    Pms7651C,
    /// `Pantone 7651 U` — `#7F5C78`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7651 U"))]
    Pms7651U,
    /// `Pantone 7652 C` — `#5E2751`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7652 C"))]
    Pms7652C,
    /// `Pantone 7652 U` — `#73556C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7652 U"))]
    Pms7652U,
    /// `Pantone 7653 C` — `#948794`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7653 C"))]
    Pms7653C,
    /// `Pantone 7653 U` — `#9E95A0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7653 U"))]
    Pms7653U,
    /// `Pantone 7654 C` — `#A2789C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7654 C"))]
    Pms7654C,
    /// `Pantone 7654 U` — `#AA8AA5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7654 U"))]
    Pms7654U,
    /// `Pantone 7655 C` — `#A15A95`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7655 C"))]
    Pms7655C,
    /// `Pantone 7655 U` — `#A978A0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7655 U"))]
    Pms7655U,
    /// `Pantone 7656 C` — `#8E3A80`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7656 C"))]
    Pms7656C,
    /// `Pantone 7656 U` — `#96648D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7656 U"))]
    Pms7656U,
    /// `Pantone 7657 C` — `#6E2B62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7657 C"))]
    Pms7657C,
    /// `Pantone 7657 U` — `#7E5978`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7657 U"))]
    Pms7657U,
    /// `Pantone 7658 C` — `#6A3460`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7658 C"))]
    Pms7658C,
    /// `Pantone 7658 U` — `#7C5E77`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7658 U"))]
    Pms7658U,
    /// `Pantone 7659 C` — `#5D3754`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7659 C"))]
    Pms7659C,
    /// `Pantone 7659 U` — `#6E5A6B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7659 U"))]
    Pms7659U,
    /// `Pantone 7660 C` — `#9991A4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7660 C"))]
    Pms7660C,
    /// `Pantone 7660 U` — `#9C97A8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7660 U"))]
    Pms7660U,
    /// `Pantone 7661 C` — `#8D6E97`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7661 C"))]
    Pms7661C,
    /// `Pantone 7661 U` — `#947F9D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7661 U"))]
    Pms7661U,
    /// `Pantone 7662 C` — `#7A4183`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7662 C"))]
    Pms7662C,
    /// `Pantone 7662 U` — `#84668C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7662 U"))]
    Pms7662U,
    /// `Pantone 7663 C` — `#6B3077`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7663 C"))]
    Pms7663C,
    /// `Pantone 7663 U` — `#7A5C83`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7663 U"))]
    Pms7663U,
    /// `Pantone 7664 C` — `#653279`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7664 C"))]
    Pms7664C,
    /// `Pantone 7664 U` — `#765D84`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7664 U"))]
    Pms7664U,
    /// `Pantone 7665 C` — `#5E366E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7665 C"))]
    Pms7665C,
    /// `Pantone 7665 U` — `#725E7C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7665 U"))]
    Pms7665U,
    /// `Pantone 7666 C` — `#5C4E63`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7666 C"))]
    Pms7666C,
    /// `Pantone 7666 U` — `#706977`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7666 U"))]
    Pms7666U,
    /// `Pantone 7667 C` — `#6E7CA0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7667 C"))]
    Pms7667C,
    /// `Pantone 7667 U` — `#828CA8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7667 U"))]
    Pms7667U,
    /// `Pantone 7668 C` — `#686E9F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7668 C"))]
    Pms7668C,
    /// `Pantone 7668 U` — `#7B81A4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7668 U"))]
    Pms7668U,
    /// `Pantone 7669 C` — `#615E9B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7669 C"))]
    Pms7669C,
    /// `Pantone 7669 U` — `#7878A2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7669 U"))]
    Pms7669U,
    /// `Pantone 7670 C` — `#565294`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7670 C"))]
    Pms7670C,
    /// `Pantone 7670 U` — `#706E9A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7670 U"))]
    Pms7670U,
    /// `Pantone 7671 C` — `#514689`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7671 C"))]
    Pms7671C,
    /// `Pantone 7671 U` — `#6E6993`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7671 U"))]
    Pms7671U,
    /// `Pantone 7672 C` — `#4C4184`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7672 C"))]
    Pms7672C,
    /// `Pantone 7672 U` — `#6B668E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7672 U"))]
    Pms7672U,
    /// `Pantone 7673 C` — `#535486`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7673 C"))]
    Pms7673C,
    /// `Pantone 7673 U` — `#6C6D8D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7673 U"))]
    Pms7673U,
    /// `Pantone 7674 C` — `#878CB4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7674 C"))]
    Pms7674C,
    /// `Pantone 7674 U` — `#9296B8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7674 U"))]
    Pms7674U,
    /// `Pantone 7675 C` — `#7C7FAB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7675 C"))]
    Pms7675C,
    /// `Pantone 7675 U` — `#888CAF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7675 U"))]
    Pms7675U,
    /// `Pantone 7676 C` — `#7566A0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7676 C"))]
    Pms7676C,
    /// `Pantone 7676 U` — `#827AA3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7676 U"))]
    Pms7676U,
    /// `Pantone 7677 C` — `#6F5091`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7677 C"))]
    Pms7677C,
    /// `Pantone 7677 U` — `#7C6D96`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7677 U"))]
    Pms7677U,
    /// `Pantone 7678 C` — `#68478D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7678 C"))]
    Pms7678C,
    /// `Pantone 7678 U` — `#766691`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7678 U"))]
    Pms7678U,
    /// `Pantone 7679 C` — `#563D82`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7679 C"))]
    Pms7679C,
    /// `Pantone 7679 U` — `#6E628B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7679 U"))]
    Pms7679U,
    /// `Pantone 7680 C` — `#523178`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7680 C"))]
    Pms7680C,
    /// `Pantone 7680 U` — `#695981`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7680 U"))]
    Pms7680U,
    /// `Pantone 7681 C` — `#94A9CB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7681 C"))]
    Pms7681C,
    /// `Pantone 7681 U` — `#9DB0D0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7681 U"))]
    Pms7681U,
    /// `Pantone 7682 C` — `#6787B7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7682 C"))]
    Pms7682C,
    /// `Pantone 7682 U` — `#8097BF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7682 U"))]
    Pms7682U,
    /// `Pantone 7683 C` — `#426DA9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7683 C"))]
    Pms7683C,
    /// `Pantone 7683 U` — `#6982B0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7683 U"))]
    Pms7683U,
    /// `Pantone 7684 C` — `#385E9D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7684 C"))]
    Pms7684C,
    /// `Pantone 7684 U` — `#6276A3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7684 U"))]
    Pms7684U,
    /// `Pantone 7685 C` — `#2C5697`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7685 C"))]
    Pms7685C,
    /// `Pantone 7685 U` — `#5E719D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7685 U"))]
    Pms7685U,
    /// `Pantone 7686 C` — `#1D4F91`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7686 C"))]
    Pms7686C,
    /// `Pantone 7686 U` — `#556995`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7686 U"))]
    Pms7686U,
    /// `Pantone 7687 C` — `#1D428A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7687 C"))]
    Pms7687C,
    /// `Pantone 7687 U` — `#525F8D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7687 U"))]
    Pms7687U,
    /// `Pantone 7688 C` — `#4698CB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7688 C"))]
    Pms7688C,
    /// `Pantone 7688 U` — `#5E9ACA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7688 U"))]
    Pms7688U,
    /// `Pantone 7689 C` — `#298FC2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7689 C"))]
    Pms7689C,
    /// `Pantone 7689 U` — `#5693C1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7689 U"))]
    Pms7689U,
    /// `Pantone 7690 C` — `#0076A8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7690 C"))]
    Pms7690C,
    /// `Pantone 7690 U` — `#4C7FA7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7690 U"))]
    Pms7690U,
    /// `Pantone 7691 C` — `#006298`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7691 C"))]
    Pms7691C,
    /// `Pantone 7691 U` — `#467198`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7691 U"))]
    Pms7691U,
    /// `Pantone 7692 C` — `#005587`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7692 C"))]
    Pms7692C,
    /// `Pantone 7692 U` — `#4C6B8B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7692 U"))]
    Pms7692U,
    /// `Pantone 7693 C` — `#004976`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7693 C"))]
    Pms7693C,
    /// `Pantone 7693 U` — `#4D6480`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7693 U"))]
    Pms7693U,
    /// `Pantone 7694 C` — `#01426A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7694 C"))]
    Pms7694C,
    /// `Pantone 7694 U` — `#4D6179`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7694 U"))]
    Pms7694U,
    /// `Pantone 7695 C` — `#7BA7BC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7695 C"))]
    Pms7695C,
    /// `Pantone 7695 U` — `#82A8BD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7695 U"))]
    Pms7695U,
    /// `Pantone 7696 C` — `#6399AE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7696 C"))]
    Pms7696C,
    /// `Pantone 7696 U` — `#719BAF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7696 U"))]
    Pms7696U,
    /// `Pantone 7697 C` — `#4E87A0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7697 C"))]
    Pms7697C,
    /// `Pantone 7697 U` — `#698FA4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7697 U"))]
    Pms7697U,
    /// `Pantone 7698 C` — `#41748D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7698 C"))]
    Pms7698C,
    /// `Pantone 7698 U` — `#637F92`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7698 U"))]
    Pms7698U,
    /// `Pantone 7699 C` — `#34657F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7699 C"))]
    Pms7699C,
    /// `Pantone 7699 U` — `#5E7688`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7699 U"))]
    Pms7699U,
    /// `Pantone 7700 C` — `#165C7D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7700 C"))]
    Pms7700C,
    /// `Pantone 7700 U` — `#557288`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7700 U"))]
    Pms7700U,
    /// `Pantone 7701 C` — `#005776`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7701 C"))]
    Pms7701C,
    /// `Pantone 7701 U` — `#496B80`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7701 U"))]
    Pms7701U,
    /// `Pantone 7702 C` — `#48A9C5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7702 C"))]
    Pms7702C,
    /// `Pantone 7702 U` — `#5DABC6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7702 U"))]
    Pms7702U,
    /// `Pantone 7703 C` — `#009CBD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7703 C"))]
    Pms7703C,
    /// `Pantone 7703 U` — `#46A0BF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7703 U"))]
    Pms7703U,
    /// `Pantone 7704 C` — `#0085AD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7704 C"))]
    Pms7704C,
    /// `Pantone 7704 U` — `#348BAC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7704 U"))]
    Pms7704U,
    /// `Pantone 7705 C` — `#007096`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7705 C"))]
    Pms7705C,
    /// `Pantone 7705 U` — `#3C7D9A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7705 U"))]
    Pms7705U,
    /// `Pantone 7706 C` — `#006A8E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7706 C"))]
    Pms7706C,
    /// `Pantone 7706 U` — `#437A94`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7706 U"))]
    Pms7706U,
    /// `Pantone 7707 C` — `#00617F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7707 C"))]
    Pms7707C,
    /// `Pantone 7707 U` — `#477388`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7707 U"))]
    Pms7707U,
    /// `Pantone 7708 C` — `#005670`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7708 C"))]
    Pms7708C,
    /// `Pantone 7708 U` — `#456B7D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7708 U"))]
    Pms7708U,
    /// `Pantone 7709 C` — `#63B1BC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7709 C"))]
    Pms7709C,
    /// `Pantone 7709 U` — `#6EB2BD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7709 U"))]
    Pms7709U,
    /// `Pantone 7710 C` — `#00A7B5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7710 C"))]
    Pms7710C,
    /// `Pantone 7710 U` — `#40AAB8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7710 U"))]
    Pms7710U,
    /// `Pantone 7711 C` — `#0097A9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7711 C"))]
    Pms7711C,
    /// `Pantone 7711 U` — `#179DAB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7711 U"))]
    Pms7711U,
    /// `Pantone 7712 C` — `#00859B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7712 C"))]
    Pms7712C,
    /// `Pantone 7712 U` — `#228B9D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7712 U"))]
    Pms7712U,
    /// `Pantone 7713 C` — `#007D8A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7713 C"))]
    Pms7713C,
    /// `Pantone 7713 U` — `#2D8692`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7713 U"))]
    Pms7713U,
    /// `Pantone 7714 C` — `#007680`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7714 C"))]
    Pms7714C,
    /// `Pantone 7714 U` — `#358189`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7714 U"))]
    Pms7714U,
    /// `Pantone 7715 C` — `#006269`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7715 C"))]
    Pms7715C,
    /// `Pantone 7715 U` — `#417178`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7715 U"))]
    Pms7715U,
    /// `Pantone 7716 C` — `#00968F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7716 C"))]
    Pms7716C,
    /// `Pantone 7716 U` — `#499996`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7716 U"))]
    Pms7716U,
    /// `Pantone 7717 C` — `#00857D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7717 C"))]
    Pms7717C,
    /// `Pantone 7717 U` — `#3E8E8B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7717 U"))]
    Pms7717U,
    /// `Pantone 7718 C` — `#007672`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7718 C"))]
    Pms7718C,
    /// `Pantone 7718 U` — `#358282`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7718 U"))]
    Pms7718U,
    /// `Pantone 7719 C` — `#006D68`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7719 C"))]
    Pms7719C,
    /// `Pantone 7719 U` — `#417B7B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7719 U"))]
    Pms7719U,
    /// `Pantone 7720 C` — `#00635B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7720 C"))]
    Pms7720C,
    /// `Pantone 7720 U` — `#477675`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7720 U"))]
    Pms7720U,
    /// `Pantone 7721 C` — `#005E5D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7721 C"))]
    Pms7721C,
    /// `Pantone 7721 U` — `#497375`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7721 U"))]
    Pms7721U,
    /// `Pantone 7722 C` — `#005151`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7722 C"))]
    Pms7722C,
    /// `Pantone 7722 U` — `#4A6B6E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7722 U"))]
    Pms7722U,
    /// `Pantone 7723 C` — `#50A684`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7723 C"))]
    Pms7723C,
    /// `Pantone 7723 U` — `#6BAC92`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7723 U"))]
    Pms7723U,
    /// `Pantone 7724 C` — `#00966C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7724 C"))]
    Pms7724C,
    /// `Pantone 7724 U` — `#4E9B80`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7724 U"))]
    Pms7724U,
    /// `Pantone 7725 C` — `#008755`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7725 C"))]
    Pms7725C,
    /// `Pantone 7725 U` — `#379272`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7725 U"))]
    Pms7725U,
    /// `Pantone 7726 C` — `#007B4B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7726 C"))]
    Pms7726C,
    /// `Pantone 7726 U` — `#348569`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7726 U"))]
    Pms7726U,
    /// `Pantone 7727 C` — `#006F44`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7727 C"))]
    Pms7727C,
    /// `Pantone 7727 U` — `#3E7C65`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7727 U"))]
    Pms7727U,
    /// `Pantone 7728 C` — `#006845`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7728 C"))]
    Pms7728C,
    /// `Pantone 7728 U` — `#457865`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7728 U"))]
    Pms7728U,
    /// `Pantone 7729 C` — `#005844`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7729 C"))]
    Pms7729C,
    /// `Pantone 7729 U` — `#486F67`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7729 U"))]
    Pms7729U,
    /// `Pantone 7730 C` — `#4B9560`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7730 C"))]
    Pms7730C,
    /// `Pantone 7730 U` — `#689C7A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7730 U"))]
    Pms7730U,
    /// `Pantone 7731 C` — `#228848`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7731 C"))]
    Pms7731C,
    /// `Pantone 7731 U` — `#58926B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7731 U"))]
    Pms7731U,
    /// `Pantone 7732 C` — `#007A3E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7732 C"))]
    Pms7732C,
    /// `Pantone 7732 U` — `#498564`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7732 U"))]
    Pms7732U,
    /// `Pantone 7733 C` — `#007041`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7733 C"))]
    Pms7733C,
    /// `Pantone 7733 U` — `#4B7C63`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7733 U"))]
    Pms7733U,
    /// `Pantone 7734 C` — `#286140`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7734 C"))]
    Pms7734C,
    /// `Pantone 7734 U` — `#597765`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7734 U"))]
    Pms7734U,
    /// `Pantone 7735 C` — `#36573B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7735 C"))]
    Pms7735C,
    /// `Pantone 7735 U` — `#5F7163`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7735 U"))]
    Pms7735U,
    /// `Pantone 7736 C` — `#395542`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7736 C"))]
    Pms7736C,
    /// `Pantone 7736 U` — `#5C6B62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7736 U"))]
    Pms7736U,
    /// `Pantone 7737 C` — `#6BA539`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7737 C"))]
    Pms7737C,
    /// `Pantone 7737 U` — `#7BAC66`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7737 U"))]
    Pms7737U,
    /// `Pantone 7738 C` — `#48A23F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7738 C"))]
    Pms7738C,
    /// `Pantone 7738 U` — `#65A969`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7738 U"))]
    Pms7738U,
    /// `Pantone 7739 C` — `#319B42`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7739 C"))]
    Pms7739C,
    /// `Pantone 7739 U` — `#5CA369`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7739 U"))]
    Pms7739U,
    /// `Pantone 7740 C` — `#3A913F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7740 C"))]
    Pms7740C,
    /// `Pantone 7740 U` — `#5E9865`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7740 U"))]
    Pms7740U,
    /// `Pantone 7741 C` — `#44883E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7741 C"))]
    Pms7741C,
    /// `Pantone 7741 U` — `#659267`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7741 U"))]
    Pms7741U,
    /// `Pantone 7742 C` — `#4A773C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7742 C"))]
    Pms7742C,
    /// `Pantone 7742 U` — `#688564`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7742 U"))]
    Pms7742U,
    /// `Pantone 7743 C` — `#44693D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7743 C"))]
    Pms7743C,
    /// `Pantone 7743 U` — `#5E745D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7743 U"))]
    Pms7743U,
    /// `Pantone 7744 C` — `#BABC16`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7744 C"))]
    Pms7744C,
    /// `Pantone 7744 U` — `#B9BE56`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7744 U"))]
    Pms7744U,
    /// `Pantone 7745 C` — `#ABAD23`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7745 C"))]
    Pms7745C,
    /// `Pantone 7745 U` — `#AFB35D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7745 U"))]
    Pms7745U,
    /// `Pantone 7746 C` — `#999B30`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7746 C"))]
    Pms7746C,
    /// `Pantone 7746 U` — `#9B9F60`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7746 U"))]
    Pms7746U,
    /// `Pantone 7747 C` — `#888D30`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7747 C"))]
    Pms7747C,
    /// `Pantone 7747 U` — `#909661`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7747 U"))]
    Pms7747U,
    /// `Pantone 7748 C` — `#7C8034`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7748 C"))]
    Pms7748C,
    /// `Pantone 7748 U` — `#878C62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7748 U"))]
    Pms7748U,
    /// `Pantone 7749 C` — `#727337`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7749 C"))]
    Pms7749C,
    /// `Pantone 7749 U` — `#808363`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7749 U"))]
    Pms7749U,
    /// `Pantone 7750 C` — `#656635`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7750 C"))]
    Pms7750C,
    /// `Pantone 7750 U` — `#777A61`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7750 U"))]
    Pms7750U,
    /// `Pantone 7751 C` — `#CAB64B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7751 C"))]
    Pms7751C,
    /// `Pantone 7751 U` — `#D1BD71`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7751 U"))]
    Pms7751U,
    /// `Pantone 7752 C` — `#CFB023`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7752 C"))]
    Pms7752C,
    /// `Pantone 7752 U` — `#D5B961`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7752 U"))]
    Pms7752U,
    /// `Pantone 7753 C` — `#C1A01E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7753 C"))]
    Pms7753C,
    /// `Pantone 7753 U` — `#C6AA5D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7753 U"))]
    Pms7753U,
    /// `Pantone 7754 C` — `#A08629`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7754 C"))]
    Pms7754C,
    /// `Pantone 7754 U` — `#A4915E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7754 U"))]
    Pms7754U,
    /// `Pantone 7755 C` — `#897630`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7755 C"))]
    Pms7755C,
    /// `Pantone 7755 U` — `#90855F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7755 U"))]
    Pms7755U,
    /// `Pantone 7756 C` — `#736635`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7756 C"))]
    Pms7756C,
    /// `Pantone 7756 U` — `#837D62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7756 U"))]
    Pms7756U,
    /// `Pantone 7757 C` — `#675E33`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7757 C"))]
    Pms7757C,
    /// `Pantone 7757 U` — `#7A755F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7757 U"))]
    Pms7757U,
    /// `Pantone 7758 C` — `#D4C304`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7758 C"))]
    Pms7758C,
    /// `Pantone 7758 U` — `#DAC958`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7758 U"))]
    Pms7758U,
    /// `Pantone 7759 C` — `#C4B200`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7759 C"))]
    Pms7759C,
    /// `Pantone 7759 U` — `#C8B852`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7759 U"))]
    Pms7759U,
    /// `Pantone 7760 C` — `#91852C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7760 C"))]
    Pms7760C,
    /// `Pantone 7760 U` — `#968F5E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7760 U"))]
    Pms7760U,
    /// `Pantone 7761 C` — `#747136`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7761 C"))]
    Pms7761C,
    /// `Pantone 7761 U` — `#828162`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7761 U"))]
    Pms7761U,
    /// `Pantone 7762 C` — `#5D6439`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7762 C"))]
    Pms7762C,
    /// `Pantone 7762 U` — `#747A63`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7762 U"))]
    Pms7762U,
    /// `Pantone 7763 C` — `#585C3B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7763 C"))]
    Pms7763C,
    /// `Pantone 7763 U` — `#717463`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7763 U"))]
    Pms7763U,
    /// `Pantone 7764 C` — `#535435`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7764 C"))]
    Pms7764C,
    /// `Pantone 7764 U` — `#6C6E60`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7764 U"))]
    Pms7764U,
    /// `Pantone 7765 C` — `#BBB323`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7765 C"))]
    Pms7765C,
    /// `Pantone 7765 U` — `#C0BA5E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7765 U"))]
    Pms7765U,
    /// `Pantone 7766 C` — `#B4A91F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7766 C"))]
    Pms7766C,
    /// `Pantone 7766 U` — `#BBB35F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7766 U"))]
    Pms7766U,
    /// `Pantone 7767 C` — `#AA9D2E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7767 C"))]
    Pms7767C,
    /// `Pantone 7767 U` — `#B1A763`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7767 U"))]
    Pms7767U,
    /// `Pantone 7768 C` — `#8F7E35`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7768 C"))]
    Pms7768C,
    /// `Pantone 7768 U` — `#958B62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7768 U"))]
    Pms7768U,
    /// `Pantone 7769 C` — `#716135`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7769 C"))]
    Pms7769C,
    /// `Pantone 7769 U` — `#81775F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7769 U"))]
    Pms7769U,
    /// `Pantone 7770 C` — `#635939`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7770 C"))]
    Pms7770C,
    /// `Pantone 7770 U` — `#777161`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7770 U"))]
    Pms7770U,
    /// `Pantone 7771 C` — `#4E4934`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7771 C"))]
    Pms7771C,
    /// `Pantone 7771 U` — `#67655A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 7771 U"))]
    Pms7771U,
    /// `Pantone 801 C` — `#009ACE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 801 C"))]
    Pms801C,
    /// `Pantone 801 U` — `#009CCD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 801 U"))]
    Pms801U,
    /// `Pantone 802 C` — `#44D62C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 802 C"))]
    Pms802C,
    /// `Pantone 802 U` — `#3BD23D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 802 U"))]
    Pms802U,
    /// `Pantone 803 C` — `#FFE900`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 803 C"))]
    Pms803C,
    /// `Pantone 803 U` — `#FFE916`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 803 U"))]
    Pms803U,
    /// `Pantone 804 C` — `#FFAA4D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 804 C"))]
    Pms804C,
    /// `Pantone 804 U` — `#FFAA52`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 804 U"))]
    Pms804U,
    /// `Pantone 805 C` — `#FF7276`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 805 C"))]
    Pms805C,
    /// `Pantone 805 U` — `#FF7477`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 805 U"))]
    Pms805U,
    /// `Pantone 806 C` — `#FF3EB5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 806 C"))]
    Pms806C,
    /// `Pantone 806 U` — `#FF48B0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 806 U"))]
    Pms806U,
    /// `Pantone 807 C` — `#EA27C2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 807 C"))]
    Pms807C,
    /// `Pantone 807 U` — `#E838BF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 807 U"))]
    Pms807U,
    /// `Pantone 871 C` — `#84754E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 871 C"))]
    Pms871C,
    /// `Pantone 871 U` — `#AD986E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 871 U"))]
    Pms871U,
    /// `Pantone 872 C` — `#85714D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 872 C"))]
    Pms872C,
    /// `Pantone 872 U` — `#AC936E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 872 U"))]
    Pms872U,
    /// `Pantone 873 C` — `#866D4B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 873 C"))]
    Pms873C,
    /// `Pantone 873 U` — `#AE906F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 873 U"))]
    Pms873U,
    /// `Pantone 874 C` — `#8B6F4E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 874 C"))]
    Pms874C,
    /// `Pantone 874 U` — `#AE8F6F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 874 U"))]
    Pms874U,
    /// `Pantone 875 C` — `#87674F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 875 C"))]
    Pms875C,
    /// `Pantone 875 U` — `#B18B74`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 875 U"))]
    Pms875U,
    /// `Pantone 876 C` — `#8B634B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 876 C"))]
    Pms876C,
    /// `Pantone 876 U` — `#B08770`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 876 U"))]
    Pms876U,
    /// `Pantone 877 C` — `#8A8D8F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 877 C"))]
    Pms877C,
    /// `Pantone 877 U` — `#B4B7B9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 877 U"))]
    Pms877U,
    /// `Pantone Black 0961 C` — `#9E978E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Black 0961 C"))]
    Black0961C,
    /// `Pantone Black 0961 U` — `#9D9994`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Black 0961 U"))]
    Black0961U,
    /// `Pantone Black 2 C` — `#332F21`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Black 2 C"))]
    Black2C,
    /// `Pantone Black 2 U` — `#646052`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Black 2 U"))]
    Black2U,
    /// `Pantone Black 3 C` — `#212721`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Black 3 C"))]
    Black3C,
    /// `Pantone Black 3 U` — `#595D59`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Black 3 U"))]
    Black3U,
    /// `Pantone Black 4 C` — `#31261D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Black 4 C"))]
    Black4C,
    /// `Pantone Black 4 U` — `#696056`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Black 4 U"))]
    Black4U,
    /// `Pantone Black 5 C` — `#3E2B2E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Black 5 C"))]
    Black5C,
    /// `Pantone Black 5 U` — `#685A5C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Black 5 U"))]
    Black5U,
    /// `Pantone Black 6 C` — `#101820`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Black 6 C"))]
    Black6C,
    /// `Pantone Black 6 U` — `#4C4E56`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Black 6 U"))]
    Black6U,
    /// `Pantone Black 7 C` — `#3D3935`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Black 7 C"))]
    Black7C,
    /// `Pantone Black 7 U` — `#6C6864`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Black 7 U"))]
    Black7U,
    /// `Pantone Black C` — `#2D2926`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Black C"))]
    BlackC,
    /// `Pantone Black U` — `#615D59`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Black U"))]
    BlackU,
    /// `Pantone Blue 072 C` — `#10069F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Blue 072 C"))]
    Blue072C,
    /// `Pantone Blue 072 U` — `#3F43AD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Blue 072 U"))]
    Blue072U,
    /// `Pantone Blue 0821 C` — `#74D1EA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Blue 0821 C"))]
    Blue0821C,
    /// `Pantone Blue 0821 U` — `#6CD1EF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Blue 0821 U"))]
    Blue0821U,
    /// `Pantone Bright Green C` — `#00B08B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Bright Green C"))]
    BrightGreenC,
    /// `Pantone Bright Orange C` — `#FF5E00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Bright Orange C"))]
    BrightOrangeC,
    /// `Pantone Bright Red C` — `#F93822`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Bright Red C"))]
    BrightRedC,
    /// `Pantone Bright Red U` — `#FE5442`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Bright Red U"))]
    BrightRedU,
    /// `Pantone Cool Gray 1 C` — `#D9D9D6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 1 C"))]
    CoolGray1C,
    /// `Pantone Cool Gray 1 U` — `#DAD9D6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 1 U"))]
    CoolGray1U,
    /// `Pantone Cool Gray 10 C` — `#63666A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 10 C"))]
    CoolGray10C,
    /// `Pantone Cool Gray 10 U` — `#818387`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 10 U"))]
    CoolGray10U,
    /// `Pantone Cool Gray 11 C` — `#53565A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 11 C"))]
    CoolGray11C,
    /// `Pantone Cool Gray 11 U` — `#7A7D81`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 11 U"))]
    CoolGray11U,
    /// `Pantone Cool Gray 2 C` — `#D0D0CE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 2 C"))]
    CoolGray2C,
    /// `Pantone Cool Gray 2 U` — `#CACAC8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 2 U"))]
    CoolGray2U,
    /// `Pantone Cool Gray 3 C` — `#C8C9C7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 3 C"))]
    CoolGray3C,
    /// `Pantone Cool Gray 3 U` — `#C5C5C5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 3 U"))]
    CoolGray3U,
    /// `Pantone Cool Gray 4 C` — `#BBBCBC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 4 C"))]
    CoolGray4C,
    /// `Pantone Cool Gray 4 U` — `#B7B8B9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 4 U"))]
    CoolGray4U,
    /// `Pantone Cool Gray 5 C` — `#B1B3B3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 5 C"))]
    CoolGray5C,
    /// `Pantone Cool Gray 5 U` — `#ADAEB0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 5 U"))]
    CoolGray5U,
    /// `Pantone Cool Gray 6 C` — `#A7A8AA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 6 C"))]
    CoolGray6C,
    /// `Pantone Cool Gray 6 U` — `#A4A6A8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 6 U"))]
    CoolGray6U,
    /// `Pantone Cool Gray 7 C` — `#97999B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 7 C"))]
    CoolGray7C,
    /// `Pantone Cool Gray 7 U` — `#9B9EA0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 7 U"))]
    CoolGray7U,
    /// `Pantone Cool Gray 8 C` — `#888B8D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 8 C"))]
    CoolGray8C,
    /// `Pantone Cool Gray 8 U` — `#939598`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 8 U"))]
    CoolGray8U,
    /// `Pantone Cool Gray 9 C` — `#75787B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 9 C"))]
    CoolGray9C,
    /// `Pantone Cool Gray 9 U` — `#898B8E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Cool Gray 9 U"))]
    CoolGray9U,
    /// `Pantone Dark Blue C` — `#00239C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Dark Blue C"))]
    DarkBlueC,
    /// `Pantone Dark Blue U` — `#444795`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Dark Blue U"))]
    DarkBlueU,
    /// `Pantone Green 0921 C` — `#9DE7D7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Green 0921 C"))]
    Green0921C,
    /// `Pantone Green 0921 U` — `#78E6D0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Green 0921 U"))]
    Green0921U,
    /// `Pantone Green C` — `#00AB84`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Green C"))]
    GreenC,
    /// `Pantone Green U` — `#00AC8C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Green U"))]
    GreenU,
    /// `Pantone Magenta 0521 C` — `#F1B2DC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Magenta 0521 C"))]
    Magenta0521C,
    /// `Pantone Magenta 0521 U` — `#F8AADD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Magenta 0521 U"))]
    Magenta0521U,
    /// `Pantone Medium Blue C` — `#0084CA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Medium Blue C"))]
    MediumBlueC,
    /// `Pantone Medium Purple C` — `#4E008E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Medium Purple C"))]
    MediumPurpleC,
    /// `Pantone Medium Purple U` — `#65428A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Medium Purple U"))]
    MediumPurpleU,
    /// `Pantone Medium Yellow C` — `#FFD900`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Medium Yellow C"))]
    MediumYellowC,
    /// `Pantone Neutral Black C` — `#222223`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Neutral Black C"))]
    NeutralBlackC,
    /// `Pantone Orange 021 C` — `#FE5000`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Orange 021 C"))]
    Orange021C,
    /// `Pantone Orange 021 U` — `#FF6C2F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Orange 021 U"))]
    Orange021U,
    /// `Pantone Pink C` — `#D62598`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Pink C"))]
    PinkC,
    /// `Pantone Pink U` — `#D1428D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Pink U"))]
    PinkU,
    /// `Pantone Process Blue C` — `#0085CA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Process Blue C"))]
    ProcessBlueC,
    /// `Pantone Process Blue U` — `#0083C3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Process Blue U"))]
    ProcessBlueU,
    /// `Pantone Purple C` — `#BB29BB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Purple C"))]
    PurpleC,
    /// `Pantone Purple U` — `#BF53B6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Purple U"))]
    PurpleU,
    /// `Pantone Red 032 C` — `#EF3340`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Red 032 C"))]
    Red032C,
    /// `Pantone Red 032 U` — `#F65058`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Red 032 U"))]
    Red032U,
    /// `Pantone Red 0331 C` — `#FCAEBB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Red 0331 C"))]
    Red0331C,
    /// `Pantone Red 0331 U` — `#FFB1BE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Red 0331 U"))]
    Red0331U,
    /// `Pantone Reflex Blue C` — `#001489`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Reflex Blue C"))]
    ReflexBlueC,
    /// `Pantone Reflex Blue U` — `#39499C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Reflex Blue U"))]
    ReflexBlueU,
    /// `Pantone Rhodamine Red C` — `#E10098`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Rhodamine Red C"))]
    RhodamineRedC,
    /// `Pantone Rhodamine Red U` — `#E44C9A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Rhodamine Red U"))]
    RhodamineRedU,
    /// `Pantone Rubine Red C` — `#CE0058`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Rubine Red C"))]
    RubineRedC,
    /// `Pantone Rubine Red U` — `#DB487E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Rubine Red U"))]
    RubineRedU,
    /// `Pantone Strong Red C` — `#CE0056`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Strong Red C"))]
    StrongRedC,
    /// `Pantone Violet 0631 C` — `#BF9BDE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Violet 0631 C"))]
    Violet0631C,
    /// `Pantone Violet 0631 U` — `#BA93DF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Violet 0631 U"))]
    Violet0631U,
    /// `Pantone Violet C` — `#440099`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Violet C"))]
    VioletC,
    /// `Pantone Violet U` — `#7758B3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Violet U"))]
    VioletU,
    /// `Pantone Warm Gray 1 C` — `#D7D2CB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 1 C"))]
    WarmGray1C,
    /// `Pantone Warm Gray 1 U` — `#DBD5CD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 1 U"))]
    WarmGray1U,
    /// `Pantone Warm Gray 10 C` — `#796E65`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 10 C"))]
    WarmGray10C,
    /// `Pantone Warm Gray 10 U` — `#827C78`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 10 U"))]
    WarmGray10U,
    /// `Pantone Warm Gray 11 C` — `#6E6259`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 11 C"))]
    WarmGray11C,
    /// `Pantone Warm Gray 11 U` — `#7D7773`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 11 U"))]
    WarmGray11U,
    /// `Pantone Warm Gray 2 C` — `#CBC4BC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 2 C"))]
    WarmGray2C,
    /// `Pantone Warm Gray 2 U` — `#CFC7C0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 2 U"))]
    WarmGray2U,
    /// `Pantone Warm Gray 3 C` — `#BFB8AF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 3 C"))]
    WarmGray3C,
    /// `Pantone Warm Gray 3 U` — `#BEB6AF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 3 U"))]
    WarmGray3U,
    /// `Pantone Warm Gray 4 C` — `#B6ADA5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 4 C"))]
    WarmGray4C,
    /// `Pantone Warm Gray 4 U` — `#B4ACA6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 4 U"))]
    WarmGray4U,
    /// `Pantone Warm Gray 5 C` — `#ACA39A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 5 C"))]
    WarmGray5C,
    /// `Pantone Warm Gray 5 U` — `#A9A19C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 5 U"))]
    WarmGray5U,
    /// `Pantone Warm Gray 6 C` — `#A59C94`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 6 C"))]
    WarmGray6C,
    /// `Pantone Warm Gray 6 U` — `#A29B96`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 6 U"))]
    WarmGray6U,
    /// `Pantone Warm Gray 7 C` — `#968C83`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 7 C"))]
    WarmGray7C,
    /// `Pantone Warm Gray 7 U` — `#968F8B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 7 U"))]
    WarmGray7U,
    /// `Pantone Warm Gray 8 C` — `#8C8279`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 8 C"))]
    WarmGray8C,
    /// `Pantone Warm Gray 8 U` — `#8F8884`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 8 U"))]
    WarmGray8U,
    /// `Pantone Warm Gray 9 C` — `#83786F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 9 C"))]
    WarmGray9C,
    /// `Pantone Warm Gray 9 U` — `#867F7C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Gray 9 U"))]
    WarmGray9U,
    /// `Pantone Warm Red C` — `#F9423A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Red C"))]
    WarmRedC,
    /// `Pantone Warm Red U` — `#FF665E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Warm Red U"))]
    WarmRedU,
    /// `Pantone White C` — `#FFFFFF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone White C"))]
    WhiteC,
    /// `Pantone Yellow 012 C` — `#FFD700`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Yellow 012 C"))]
    Yellow012C,
    /// `Pantone Yellow 012 U` — `#FFE000`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Yellow 012 U"))]
    Yellow012U,
    /// `Pantone Yellow 0131 C` — `#F2F0A1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Yellow 0131 C"))]
    Yellow0131C,
    /// `Pantone Yellow 0131 U` — `#FBF59B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Yellow 0131 U"))]
    Yellow0131U,
    /// `Pantone Yellow C` — `#FEDD00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Yellow C"))]
    YellowC,
    /// `Pantone Yellow U` — `#FFE800`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone Yellow U"))]
    YellowU,
}

impl PantoneNamedColor {
    /// All Pantone colors, sorted by canonical name.
    pub const ALL: &'static [Self] = &[
        Self::Pms100C,
        Self::Pms100U,
        Self::Pms101C,
        Self::Pms101U,
        Self::Pms102C,
        Self::Pms102U,
        Self::Pms103C,
        Self::Pms103U,
        Self::Pms104C,
        Self::Pms104U,
        Self::Pms105C,
        Self::Pms105U,
        Self::Pms106C,
        Self::Pms106U,
        Self::Pms107C,
        Self::Pms107U,
        Self::Pms108C,
        Self::Pms108U,
        Self::Pms109C,
        Self::Pms109U,
        Self::Pms110C,
        Self::Pms110U,
        Self::Pms111C,
        Self::Pms111U,
        Self::Pms112C,
        Self::Pms112U,
        Self::Pms113C,
        Self::Pms113U,
        Self::Pms114C,
        Self::Pms114U,
        Self::Pms115C,
        Self::Pms115U,
        Self::Pms116C,
        Self::Pms116U,
        Self::Pms117C,
        Self::Pms117U,
        Self::Pms118C,
        Self::Pms118U,
        Self::Pms119C,
        Self::Pms119U,
        Self::Pms120C,
        Self::Pms120U,
        Self::Pms1205C,
        Self::Pms1205U,
        Self::Pms121C,
        Self::Pms121U,
        Self::Pms1215C,
        Self::Pms1215U,
        Self::Pms122C,
        Self::Pms122U,
        Self::Pms1225C,
        Self::Pms1225U,
        Self::Pms123C,
        Self::Pms123U,
        Self::Pms1235C,
        Self::Pms1235U,
        Self::Pms124C,
        Self::Pms124U,
        Self::Pms1245C,
        Self::Pms1245U,
        Self::Pms125C,
        Self::Pms125U,
        Self::Pms1255C,
        Self::Pms1255U,
        Self::Pms126C,
        Self::Pms126U,
        Self::Pms1265C,
        Self::Pms1265U,
        Self::Pms127C,
        Self::Pms127U,
        Self::Pms128C,
        Self::Pms128U,
        Self::Pms129C,
        Self::Pms129U,
        Self::Pms130C,
        Self::Pms130U,
        Self::Pms131C,
        Self::Pms131U,
        Self::Pms132C,
        Self::Pms132U,
        Self::Pms133C,
        Self::Pms133U,
        Self::Pms134C,
        Self::Pms134U,
        Self::Pms1345C,
        Self::Pms1345U,
        Self::Pms135C,
        Self::Pms135U,
        Self::Pms1355C,
        Self::Pms1355U,
        Self::Pms136C,
        Self::Pms136U,
        Self::Pms1365C,
        Self::Pms1365U,
        Self::Pms137C,
        Self::Pms137U,
        Self::Pms1375C,
        Self::Pms1375U,
        Self::Pms138C,
        Self::Pms138U,
        Self::Pms1385C,
        Self::Pms1385U,
        Self::Pms139C,
        Self::Pms139U,
        Self::Pms1395C,
        Self::Pms1395U,
        Self::Pms140C,
        Self::Pms140U,
        Self::Pms1405C,
        Self::Pms1405U,
        Self::Pms141C,
        Self::Pms141U,
        Self::Pms142C,
        Self::Pms142U,
        Self::Pms143C,
        Self::Pms143U,
        Self::Pms144C,
        Self::Pms144U,
        Self::Pms145C,
        Self::Pms145U,
        Self::Pms146C,
        Self::Pms146U,
        Self::Pms147C,
        Self::Pms147U,
        Self::Pms148C,
        Self::Pms148U,
        Self::Pms1485C,
        Self::Pms1485U,
        Self::Pms149C,
        Self::Pms149U,
        Self::Pms1495C,
        Self::Pms1495U,
        Self::Pms150C,
        Self::Pms150U,
        Self::Pms1505C,
        Self::Pms1505U,
        Self::Pms151C,
        Self::Pms151U,
        Self::Pms152C,
        Self::Pms152U,
        Self::Pms1525C,
        Self::Pms1525U,
        Self::Pms153C,
        Self::Pms153U,
        Self::Pms1535C,
        Self::Pms1535U,
        Self::Pms154C,
        Self::Pms154U,
        Self::Pms1545C,
        Self::Pms1545U,
        Self::Pms155C,
        Self::Pms155U,
        Self::Pms1555C,
        Self::Pms1555U,
        Self::Pms156C,
        Self::Pms156U,
        Self::Pms1565C,
        Self::Pms1565U,
        Self::Pms157C,
        Self::Pms157U,
        Self::Pms1575C,
        Self::Pms1575U,
        Self::Pms158C,
        Self::Pms158U,
        Self::Pms1585C,
        Self::Pms1585U,
        Self::Pms159C,
        Self::Pms159U,
        Self::Pms1595C,
        Self::Pms1595U,
        Self::Pms160C,
        Self::Pms160U,
        Self::Pms1605C,
        Self::Pms1605U,
        Self::Pms161C,
        Self::Pms161U,
        Self::Pms1615C,
        Self::Pms1615U,
        Self::Pms162C,
        Self::Pms162U,
        Self::Pms1625C,
        Self::Pms1625U,
        Self::Pms163C,
        Self::Pms163U,
        Self::Pms1635C,
        Self::Pms1635U,
        Self::Pms164C,
        Self::Pms164U,
        Self::Pms1645C,
        Self::Pms1645U,
        Self::Pms165C,
        Self::Pms165U,
        Self::Pms1655C,
        Self::Pms1655U,
        Self::Pms166C,
        Self::Pms166U,
        Self::Pms1665C,
        Self::Pms1665U,
        Self::Pms167C,
        Self::Pms167U,
        Self::Pms1675C,
        Self::Pms1675U,
        Self::Pms168C,
        Self::Pms168U,
        Self::Pms1685C,
        Self::Pms1685U,
        Self::Pms169C,
        Self::Pms169U,
        Self::Pms170C,
        Self::Pms170U,
        Self::Pms171C,
        Self::Pms171U,
        Self::Pms172C,
        Self::Pms172U,
        Self::Pms173C,
        Self::Pms173U,
        Self::Pms174C,
        Self::Pms174U,
        Self::Pms175C,
        Self::Pms175U,
        Self::Pms176C,
        Self::Pms176U,
        Self::Pms1765C,
        Self::Pms1765U,
        Self::Pms1767C,
        Self::Pms1767U,
        Self::Pms177C,
        Self::Pms177U,
        Self::Pms1775C,
        Self::Pms1775U,
        Self::Pms1777C,
        Self::Pms1777U,
        Self::Pms178C,
        Self::Pms178U,
        Self::Pms1785C,
        Self::Pms1785U,
        Self::Pms1787C,
        Self::Pms1787U,
        Self::Pms1788C,
        Self::Pms1788U,
        Self::Pms179C,
        Self::Pms179U,
        Self::Pms1795C,
        Self::Pms1795U,
        Self::Pms1797C,
        Self::Pms1797U,
        Self::Pms180C,
        Self::Pms180U,
        Self::Pms1805C,
        Self::Pms1805U,
        Self::Pms1807C,
        Self::Pms1807U,
        Self::Pms181C,
        Self::Pms181U,
        Self::Pms1815C,
        Self::Pms1815U,
        Self::Pms1817C,
        Self::Pms1817U,
        Self::Pms182C,
        Self::Pms182U,
        Self::Pms183C,
        Self::Pms183U,
        Self::Pms184C,
        Self::Pms184U,
        Self::Pms185C,
        Self::Pms185U,
        Self::Pms186C,
        Self::Pms186U,
        Self::Pms187C,
        Self::Pms187U,
        Self::Pms188C,
        Self::Pms188U,
        Self::Pms189C,
        Self::Pms189U,
        Self::Pms1895C,
        Self::Pms1895U,
        Self::Pms190C,
        Self::Pms190U,
        Self::Pms1905C,
        Self::Pms1905U,
        Self::Pms191C,
        Self::Pms191U,
        Self::Pms1915C,
        Self::Pms1915U,
        Self::Pms192C,
        Self::Pms192U,
        Self::Pms1925C,
        Self::Pms1925U,
        Self::Pms193C,
        Self::Pms193U,
        Self::Pms1935C,
        Self::Pms1935U,
        Self::Pms194C,
        Self::Pms194U,
        Self::Pms1945C,
        Self::Pms1945U,
        Self::Pms195C,
        Self::Pms195U,
        Self::Pms1955C,
        Self::Pms1955U,
        Self::Pms196C,
        Self::Pms196U,
        Self::Pms197C,
        Self::Pms197U,
        Self::Pms198C,
        Self::Pms198U,
        Self::Pms199C,
        Self::Pms199U,
        Self::Pms200C,
        Self::Pms200U,
        Self::Pms201C,
        Self::Pms201U,
        Self::Pms202C,
        Self::Pms202U,
        Self::Pms203C,
        Self::Pms203U,
        Self::Pms204C,
        Self::Pms204U,
        Self::Pms205C,
        Self::Pms205U,
        Self::Pms206C,
        Self::Pms206U,
        Self::Pms207C,
        Self::Pms207U,
        Self::Pms208C,
        Self::Pms208U,
        Self::Pms209C,
        Self::Pms209U,
        Self::Pms210C,
        Self::Pms210U,
        Self::Pms211C,
        Self::Pms211U,
        Self::Pms212C,
        Self::Pms212U,
        Self::Pms213C,
        Self::Pms213U,
        Self::Pms214C,
        Self::Pms214U,
        Self::Pms215C,
        Self::Pms215U,
        Self::Pms216C,
        Self::Pms216U,
        Self::Pms217C,
        Self::Pms217U,
        Self::Pms218C,
        Self::Pms218U,
        Self::Pms219C,
        Self::Pms219U,
        Self::Pms220C,
        Self::Pms220U,
        Self::Pms221C,
        Self::Pms221U,
        Self::Pms222C,
        Self::Pms222U,
        Self::Pms223C,
        Self::Pms223U,
        Self::Pms224C,
        Self::Pms224U,
        Self::Pms225C,
        Self::Pms225U,
        Self::Pms226C,
        Self::Pms226U,
        Self::Pms227C,
        Self::Pms227U,
        Self::Pms228C,
        Self::Pms228U,
        Self::Pms229C,
        Self::Pms229U,
        Self::Pms230C,
        Self::Pms230U,
        Self::Pms231C,
        Self::Pms231U,
        Self::Pms232C,
        Self::Pms232U,
        Self::Pms233C,
        Self::Pms233U,
        Self::Pms234C,
        Self::Pms234U,
        Self::Pms235C,
        Self::Pms235U,
        Self::Pms236C,
        Self::Pms236U,
        Self::Pms2365C,
        Self::Pms2365U,
        Self::Pms237C,
        Self::Pms237U,
        Self::Pms2375C,
        Self::Pms2375U,
        Self::Pms238C,
        Self::Pms238U,
        Self::Pms2385C,
        Self::Pms2385U,
        Self::Pms239C,
        Self::Pms239U,
        Self::Pms2395C,
        Self::Pms2395U,
        Self::Pms240C,
        Self::Pms240U,
        Self::Pms2405C,
        Self::Pms2405U,
        Self::Pms241C,
        Self::Pms241U,
        Self::Pms2415C,
        Self::Pms2415U,
        Self::Pms242C,
        Self::Pms242U,
        Self::Pms2425C,
        Self::Pms2425U,
        Self::Pms243C,
        Self::Pms243U,
        Self::Pms244C,
        Self::Pms244U,
        Self::Pms245C,
        Self::Pms245U,
        Self::Pms246C,
        Self::Pms246U,
        Self::Pms247C,
        Self::Pms247U,
        Self::Pms248C,
        Self::Pms248U,
        Self::Pms249C,
        Self::Pms249U,
        Self::Pms250C,
        Self::Pms250U,
        Self::Pms251C,
        Self::Pms251U,
        Self::Pms252C,
        Self::Pms252U,
        Self::Pms253C,
        Self::Pms253U,
        Self::Pms254C,
        Self::Pms254U,
        Self::Pms255C,
        Self::Pms255U,
        Self::Pms256C,
        Self::Pms256U,
        Self::Pms2562C,
        Self::Pms2562U,
        Self::Pms2563C,
        Self::Pms2563U,
        Self::Pms2567C,
        Self::Pms2567U,
        Self::Pms257C,
        Self::Pms257U,
        Self::Pms2572C,
        Self::Pms2572U,
        Self::Pms2573C,
        Self::Pms2573U,
        Self::Pms2577C,
        Self::Pms2577U,
        Self::Pms258C,
        Self::Pms258U,
        Self::Pms2582C,
        Self::Pms2582U,
        Self::Pms2583C,
        Self::Pms2583U,
        Self::Pms2587C,
        Self::Pms2587U,
        Self::Pms259C,
        Self::Pms259U,
        Self::Pms2592C,
        Self::Pms2592U,
        Self::Pms2593C,
        Self::Pms2593U,
        Self::Pms2597C,
        Self::Pms2597U,
        Self::Pms260C,
        Self::Pms260U,
        Self::Pms2602C,
        Self::Pms2602U,
        Self::Pms2603C,
        Self::Pms2603U,
        Self::Pms2607C,
        Self::Pms2607U,
        Self::Pms261C,
        Self::Pms261U,
        Self::Pms2612C,
        Self::Pms2612U,
        Self::Pms2613C,
        Self::Pms2613U,
        Self::Pms2617C,
        Self::Pms2617U,
        Self::Pms262C,
        Self::Pms262U,
        Self::Pms2622C,
        Self::Pms2622U,
        Self::Pms2623C,
        Self::Pms2623U,
        Self::Pms2627C,
        Self::Pms2627U,
        Self::Pms263C,
        Self::Pms263U,
        Self::Pms2635C,
        Self::Pms2635U,
        Self::Pms264C,
        Self::Pms264U,
        Self::Pms2645C,
        Self::Pms2645U,
        Self::Pms265C,
        Self::Pms265U,
        Self::Pms2655C,
        Self::Pms2655U,
        Self::Pms266C,
        Self::Pms266U,
        Self::Pms2665C,
        Self::Pms2665U,
        Self::Pms267C,
        Self::Pms267U,
        Self::Pms268C,
        Self::Pms268U,
        Self::Pms2685C,
        Self::Pms2685U,
        Self::Pms269C,
        Self::Pms269U,
        Self::Pms2695C,
        Self::Pms2695U,
        Self::Pms270C,
        Self::Pms270U,
        Self::Pms2705C,
        Self::Pms2705U,
        Self::Pms2706C,
        Self::Pms2706U,
        Self::Pms2707C,
        Self::Pms2707U,
        Self::Pms2708C,
        Self::Pms2708U,
        Self::Pms271C,
        Self::Pms271U,
        Self::Pms2715C,
        Self::Pms2715U,
        Self::Pms2716C,
        Self::Pms2716U,
        Self::Pms2717C,
        Self::Pms2717U,
        Self::Pms2718C,
        Self::Pms2718U,
        Self::Pms272C,
        Self::Pms272U,
        Self::Pms2725C,
        Self::Pms2725U,
        Self::Pms2726C,
        Self::Pms2726U,
        Self::Pms2727C,
        Self::Pms2727U,
        Self::Pms2728C,
        Self::Pms2728U,
        Self::Pms273C,
        Self::Pms273U,
        Self::Pms2735C,
        Self::Pms2735U,
        Self::Pms2736C,
        Self::Pms2736U,
        Self::Pms2738C,
        Self::Pms2738U,
        Self::Pms274C,
        Self::Pms274U,
        Self::Pms2745C,
        Self::Pms2745U,
        Self::Pms2746C,
        Self::Pms2746U,
        Self::Pms2747C,
        Self::Pms2747U,
        Self::Pms2748C,
        Self::Pms2748U,
        Self::Pms275C,
        Self::Pms275U,
        Self::Pms2755C,
        Self::Pms2755U,
        Self::Pms2756C,
        Self::Pms2756U,
        Self::Pms2757C,
        Self::Pms2757U,
        Self::Pms2758C,
        Self::Pms2758U,
        Self::Pms276C,
        Self::Pms276U,
        Self::Pms2765C,
        Self::Pms2765U,
        Self::Pms2766C,
        Self::Pms2766U,
        Self::Pms2767C,
        Self::Pms2767U,
        Self::Pms2768C,
        Self::Pms2768U,
        Self::Pms277C,
        Self::Pms277U,
        Self::Pms278C,
        Self::Pms278U,
        Self::Pms279C,
        Self::Pms279U,
        Self::Pms280C,
        Self::Pms280U,
        Self::Pms281C,
        Self::Pms281U,
        Self::Pms282C,
        Self::Pms282U,
        Self::Pms283C,
        Self::Pms283U,
        Self::Pms284C,
        Self::Pms284U,
        Self::Pms285C,
        Self::Pms285U,
        Self::Pms286C,
        Self::Pms286U,
        Self::Pms287C,
        Self::Pms287U,
        Self::Pms288C,
        Self::Pms288U,
        Self::Pms289C,
        Self::Pms289U,
        Self::Pms290C,
        Self::Pms290U,
        Self::Pms2905C,
        Self::Pms2905U,
        Self::Pms291C,
        Self::Pms291U,
        Self::Pms2915C,
        Self::Pms2915U,
        Self::Pms292C,
        Self::Pms292U,
        Self::Pms2925C,
        Self::Pms2925U,
        Self::Pms293C,
        Self::Pms293U,
        Self::Pms2935C,
        Self::Pms2935U,
        Self::Pms294C,
        Self::Pms294U,
        Self::Pms2945C,
        Self::Pms2945U,
        Self::Pms295C,
        Self::Pms295U,
        Self::Pms2955C,
        Self::Pms2955U,
        Self::Pms296C,
        Self::Pms296U,
        Self::Pms2965C,
        Self::Pms2965U,
        Self::Pms297C,
        Self::Pms297U,
        Self::Pms2975C,
        Self::Pms2975U,
        Self::Pms298C,
        Self::Pms298U,
        Self::Pms2985C,
        Self::Pms2985U,
        Self::Pms299C,
        Self::Pms299U,
        Self::Pms2995C,
        Self::Pms2995U,
        Self::Pms300C,
        Self::Pms300U,
        Self::Pms3005C,
        Self::Pms3005U,
        Self::Pms301C,
        Self::Pms301U,
        Self::Pms3015C,
        Self::Pms3015U,
        Self::Pms302C,
        Self::Pms302U,
        Self::Pms3025C,
        Self::Pms3025U,
        Self::Pms303C,
        Self::Pms303U,
        Self::Pms3035C,
        Self::Pms3035U,
        Self::Pms304C,
        Self::Pms304U,
        Self::Pms305C,
        Self::Pms305U,
        Self::Pms306C,
        Self::Pms306U,
        Self::Pms307C,
        Self::Pms307U,
        Self::Pms308C,
        Self::Pms308U,
        Self::Pms309C,
        Self::Pms309U,
        Self::Pms310C,
        Self::Pms310U,
        Self::Pms3105C,
        Self::Pms3105U,
        Self::Pms311C,
        Self::Pms311U,
        Self::Pms3115C,
        Self::Pms3115U,
        Self::Pms312C,
        Self::Pms312U,
        Self::Pms3125C,
        Self::Pms3125U,
        Self::Pms313C,
        Self::Pms313U,
        Self::Pms3135C,
        Self::Pms3135U,
        Self::Pms314C,
        Self::Pms314U,
        Self::Pms3145C,
        Self::Pms3145U,
        Self::Pms315C,
        Self::Pms315U,
        Self::Pms3155C,
        Self::Pms3155U,
        Self::Pms316C,
        Self::Pms316U,
        Self::Pms3165C,
        Self::Pms3165U,
        Self::Pms317C,
        Self::Pms317U,
        Self::Pms318C,
        Self::Pms318U,
        Self::Pms319C,
        Self::Pms319U,
        Self::Pms320C,
        Self::Pms320U,
        Self::Pms321C,
        Self::Pms321U,
        Self::Pms322C,
        Self::Pms322U,
        Self::Pms323C,
        Self::Pms323U,
        Self::Pms324C,
        Self::Pms324U,
        Self::Pms3242C,
        Self::Pms3242U,
        Self::Pms3245C,
        Self::Pms3245U,
        Self::Pms3248C,
        Self::Pms3248U,
        Self::Pms325C,
        Self::Pms325U,
        Self::Pms3252C,
        Self::Pms3252U,
        Self::Pms3255C,
        Self::Pms3255U,
        Self::Pms3258C,
        Self::Pms3258U,
        Self::Pms326C,
        Self::Pms326U,
        Self::Pms3262C,
        Self::Pms3262U,
        Self::Pms3265C,
        Self::Pms3265U,
        Self::Pms3268C,
        Self::Pms3268U,
        Self::Pms327C,
        Self::Pms327U,
        Self::Pms3272C,
        Self::Pms3272U,
        Self::Pms3275C,
        Self::Pms3275U,
        Self::Pms3278C,
        Self::Pms3278U,
        Self::Pms328C,
        Self::Pms328U,
        Self::Pms3282C,
        Self::Pms3282U,
        Self::Pms3285C,
        Self::Pms3285U,
        Self::Pms3288C,
        Self::Pms3288U,
        Self::Pms329C,
        Self::Pms329U,
        Self::Pms3292C,
        Self::Pms3292U,
        Self::Pms3295C,
        Self::Pms3295U,
        Self::Pms3298C,
        Self::Pms3298U,
        Self::Pms330C,
        Self::Pms330U,
        Self::Pms3302C,
        Self::Pms3302U,
        Self::Pms3305C,
        Self::Pms3305U,
        Self::Pms3308C,
        Self::Pms3308U,
        Self::Pms331C,
        Self::Pms331U,
        Self::Pms332C,
        Self::Pms332U,
        Self::Pms333C,
        Self::Pms333U,
        Self::Pms334C,
        Self::Pms334U,
        Self::Pms335C,
        Self::Pms335U,
        Self::Pms336C,
        Self::Pms336U,
        Self::Pms337C,
        Self::Pms337U,
        Self::Pms3375C,
        Self::Pms3375U,
        Self::Pms338C,
        Self::Pms338U,
        Self::Pms3385C,
        Self::Pms3385U,
        Self::Pms339C,
        Self::Pms339U,
        Self::Pms3395C,
        Self::Pms3395U,
        Self::Pms340C,
        Self::Pms340U,
        Self::Pms3405C,
        Self::Pms3405U,
        Self::Pms341C,
        Self::Pms341U,
        Self::Pms3415C,
        Self::Pms3415U,
        Self::Pms342C,
        Self::Pms342U,
        Self::Pms3425C,
        Self::Pms3425U,
        Self::Pms343C,
        Self::Pms343U,
        Self::Pms3435C,
        Self::Pms3435U,
        Self::Pms344C,
        Self::Pms344U,
        Self::Pms345C,
        Self::Pms345U,
        Self::Pms346C,
        Self::Pms346U,
        Self::Pms347C,
        Self::Pms347U,
        Self::Pms348C,
        Self::Pms348U,
        Self::Pms349C,
        Self::Pms349U,
        Self::Pms350C,
        Self::Pms350U,
        Self::Pms351C,
        Self::Pms351U,
        Self::Pms352C,
        Self::Pms352U,
        Self::Pms353C,
        Self::Pms353U,
        Self::Pms354C,
        Self::Pms354U,
        Self::Pms355C,
        Self::Pms355U,
        Self::Pms356C,
        Self::Pms356U,
        Self::Pms357C,
        Self::Pms357U,
        Self::Pms358C,
        Self::Pms358U,
        Self::Pms359C,
        Self::Pms359U,
        Self::Pms360C,
        Self::Pms360U,
        Self::Pms361C,
        Self::Pms361U,
        Self::Pms362C,
        Self::Pms362U,
        Self::Pms363C,
        Self::Pms363U,
        Self::Pms364C,
        Self::Pms364U,
        Self::Pms365C,
        Self::Pms365U,
        Self::Pms366C,
        Self::Pms366U,
        Self::Pms367C,
        Self::Pms367U,
        Self::Pms368C,
        Self::Pms368U,
        Self::Pms369C,
        Self::Pms369U,
        Self::Pms370C,
        Self::Pms370U,
        Self::Pms371C,
        Self::Pms371U,
        Self::Pms372C,
        Self::Pms372U,
        Self::Pms373C,
        Self::Pms373U,
        Self::Pms374C,
        Self::Pms374U,
        Self::Pms375C,
        Self::Pms375U,
        Self::Pms376C,
        Self::Pms376U,
        Self::Pms377C,
        Self::Pms377U,
        Self::Pms378C,
        Self::Pms378U,
        Self::Pms379C,
        Self::Pms379U,
        Self::Pms380C,
        Self::Pms380U,
        Self::Pms381C,
        Self::Pms381U,
        Self::Pms382C,
        Self::Pms382U,
        Self::Pms383C,
        Self::Pms383U,
        Self::Pms384C,
        Self::Pms384U,
        Self::Pms385C,
        Self::Pms385U,
        Self::Pms386C,
        Self::Pms386U,
        Self::Pms387C,
        Self::Pms387U,
        Self::Pms388C,
        Self::Pms388U,
        Self::Pms389C,
        Self::Pms389U,
        Self::Pms390C,
        Self::Pms390U,
        Self::Pms391C,
        Self::Pms391U,
        Self::Pms392C,
        Self::Pms392U,
        Self::Pms393C,
        Self::Pms393U,
        Self::Pms3935C,
        Self::Pms3935U,
        Self::Pms394C,
        Self::Pms394U,
        Self::Pms3945C,
        Self::Pms3945U,
        Self::Pms395C,
        Self::Pms395U,
        Self::Pms3955C,
        Self::Pms3955U,
        Self::Pms396C,
        Self::Pms396U,
        Self::Pms3965C,
        Self::Pms3965U,
        Self::Pms397C,
        Self::Pms397U,
        Self::Pms3975C,
        Self::Pms3975U,
        Self::Pms398C,
        Self::Pms398U,
        Self::Pms3985C,
        Self::Pms3985U,
        Self::Pms399C,
        Self::Pms399U,
        Self::Pms3995C,
        Self::Pms3995U,
        Self::Pms400C,
        Self::Pms400U,
        Self::Pms401C,
        Self::Pms401U,
        Self::Pms402C,
        Self::Pms402U,
        Self::Pms403C,
        Self::Pms403U,
        Self::Pms404C,
        Self::Pms404U,
        Self::Pms405C,
        Self::Pms405U,
        Self::Pms406C,
        Self::Pms406U,
        Self::Pms407C,
        Self::Pms407U,
        Self::Pms408C,
        Self::Pms408U,
        Self::Pms409C,
        Self::Pms409U,
        Self::Pms410C,
        Self::Pms410U,
        Self::Pms411C,
        Self::Pms411U,
        Self::Pms412C,
        Self::Pms412U,
        Self::Pms413C,
        Self::Pms413U,
        Self::Pms414C,
        Self::Pms414U,
        Self::Pms415C,
        Self::Pms415U,
        Self::Pms416C,
        Self::Pms416U,
        Self::Pms417C,
        Self::Pms417U,
        Self::Pms418C,
        Self::Pms418U,
        Self::Pms419C,
        Self::Pms419U,
        Self::Pms420C,
        Self::Pms420U,
        Self::Pms421C,
        Self::Pms421U,
        Self::Pms422C,
        Self::Pms422U,
        Self::Pms423C,
        Self::Pms423U,
        Self::Pms424C,
        Self::Pms424U,
        Self::Pms425C,
        Self::Pms425U,
        Self::Pms426C,
        Self::Pms426U,
        Self::Pms427C,
        Self::Pms427U,
        Self::Pms428C,
        Self::Pms428U,
        Self::Pms429C,
        Self::Pms429U,
        Self::Pms430C,
        Self::Pms430U,
        Self::Pms431C,
        Self::Pms431U,
        Self::Pms432C,
        Self::Pms432U,
        Self::Pms433C,
        Self::Pms433U,
        Self::Pms434C,
        Self::Pms434U,
        Self::Pms435C,
        Self::Pms435U,
        Self::Pms436C,
        Self::Pms436U,
        Self::Pms437C,
        Self::Pms437U,
        Self::Pms438C,
        Self::Pms438U,
        Self::Pms439C,
        Self::Pms439U,
        Self::Pms440C,
        Self::Pms440U,
        Self::Pms441C,
        Self::Pms441U,
        Self::Pms442C,
        Self::Pms442U,
        Self::Pms443C,
        Self::Pms443U,
        Self::Pms444C,
        Self::Pms444U,
        Self::Pms445C,
        Self::Pms445U,
        Self::Pms446C,
        Self::Pms446U,
        Self::Pms447C,
        Self::Pms447U,
        Self::Pms448C,
        Self::Pms448U,
        Self::Pms4485C,
        Self::Pms4485U,
        Self::Pms449C,
        Self::Pms449U,
        Self::Pms4495C,
        Self::Pms4495U,
        Self::Pms450C,
        Self::Pms450U,
        Self::Pms4505C,
        Self::Pms4505U,
        Self::Pms451C,
        Self::Pms451U,
        Self::Pms4515C,
        Self::Pms4515U,
        Self::Pms452C,
        Self::Pms452U,
        Self::Pms4525C,
        Self::Pms4525U,
        Self::Pms453C,
        Self::Pms453U,
        Self::Pms4535C,
        Self::Pms4535U,
        Self::Pms454C,
        Self::Pms454U,
        Self::Pms4545C,
        Self::Pms4545U,
        Self::Pms455C,
        Self::Pms455U,
        Self::Pms456C,
        Self::Pms456U,
        Self::Pms457C,
        Self::Pms457U,
        Self::Pms458C,
        Self::Pms458U,
        Self::Pms459C,
        Self::Pms459U,
        Self::Pms460C,
        Self::Pms460U,
        Self::Pms461C,
        Self::Pms461U,
        Self::Pms462C,
        Self::Pms462U,
        Self::Pms4625C,
        Self::Pms4625U,
        Self::Pms463C,
        Self::Pms463U,
        Self::Pms4635C,
        Self::Pms4635U,
        Self::Pms464C,
        Self::Pms464U,
        Self::Pms4645C,
        Self::Pms4645U,
        Self::Pms465C,
        Self::Pms465U,
        Self::Pms4655C,
        Self::Pms4655U,
        Self::Pms466C,
        Self::Pms466U,
        Self::Pms4665C,
        Self::Pms4665U,
        Self::Pms467C,
        Self::Pms467U,
        Self::Pms4675C,
        Self::Pms4675U,
        Self::Pms468C,
        Self::Pms468U,
        Self::Pms4685C,
        Self::Pms4685U,
        Self::Pms469C,
        Self::Pms469U,
        Self::Pms4695C,
        Self::Pms4695U,
        Self::Pms470C,
        Self::Pms470U,
        Self::Pms4705C,
        Self::Pms4705U,
        Self::Pms471C,
        Self::Pms471U,
        Self::Pms4715C,
        Self::Pms4715U,
        Self::Pms472C,
        Self::Pms472U,
        Self::Pms4725C,
        Self::Pms4725U,
        Self::Pms473C,
        Self::Pms473U,
        Self::Pms4735C,
        Self::Pms4735U,
        Self::Pms474C,
        Self::Pms474U,
        Self::Pms4745C,
        Self::Pms4745U,
        Self::Pms475C,
        Self::Pms475U,
        Self::Pms4755C,
        Self::Pms4755U,
        Self::Pms476C,
        Self::Pms476U,
        Self::Pms477C,
        Self::Pms477U,
        Self::Pms478C,
        Self::Pms478U,
        Self::Pms479C,
        Self::Pms479U,
        Self::Pms480C,
        Self::Pms480U,
        Self::Pms481C,
        Self::Pms481U,
        Self::Pms482C,
        Self::Pms482U,
        Self::Pms483C,
        Self::Pms483U,
        Self::Pms484C,
        Self::Pms484U,
        Self::Pms485C,
        Self::Pms485U,
        Self::Pms486C,
        Self::Pms486U,
        Self::Pms487C,
        Self::Pms487U,
        Self::Pms488C,
        Self::Pms488U,
        Self::Pms489C,
        Self::Pms489U,
        Self::Pms490C,
        Self::Pms490U,
        Self::Pms491C,
        Self::Pms491U,
        Self::Pms492C,
        Self::Pms492U,
        Self::Pms493C,
        Self::Pms493U,
        Self::Pms494C,
        Self::Pms494U,
        Self::Pms495C,
        Self::Pms495U,
        Self::Pms496C,
        Self::Pms496U,
        Self::Pms497C,
        Self::Pms497U,
        Self::Pms4975C,
        Self::Pms4975U,
        Self::Pms498C,
        Self::Pms498U,
        Self::Pms4985C,
        Self::Pms4985U,
        Self::Pms499C,
        Self::Pms499U,
        Self::Pms4995C,
        Self::Pms4995U,
        Self::Pms500C,
        Self::Pms500U,
        Self::Pms5005C,
        Self::Pms5005U,
        Self::Pms501C,
        Self::Pms501U,
        Self::Pms5015C,
        Self::Pms5015U,
        Self::Pms502C,
        Self::Pms502U,
        Self::Pms5025C,
        Self::Pms5025U,
        Self::Pms503C,
        Self::Pms503U,
        Self::Pms5035C,
        Self::Pms5035U,
        Self::Pms504C,
        Self::Pms504U,
        Self::Pms505C,
        Self::Pms505U,
        Self::Pms506C,
        Self::Pms506U,
        Self::Pms507C,
        Self::Pms507U,
        Self::Pms508C,
        Self::Pms508U,
        Self::Pms509C,
        Self::Pms509U,
        Self::Pms510C,
        Self::Pms510U,
        Self::Pms511C,
        Self::Pms511U,
        Self::Pms5115C,
        Self::Pms5115U,
        Self::Pms512C,
        Self::Pms512U,
        Self::Pms5125C,
        Self::Pms5125U,
        Self::Pms513C,
        Self::Pms513U,
        Self::Pms5135C,
        Self::Pms5135U,
        Self::Pms514C,
        Self::Pms514U,
        Self::Pms5145C,
        Self::Pms5145U,
        Self::Pms515C,
        Self::Pms515U,
        Self::Pms5155C,
        Self::Pms5155U,
        Self::Pms516C,
        Self::Pms516U,
        Self::Pms5165C,
        Self::Pms5165U,
        Self::Pms517C,
        Self::Pms517U,
        Self::Pms5175C,
        Self::Pms5175U,
        Self::Pms518C,
        Self::Pms518U,
        Self::Pms5185C,
        Self::Pms5185U,
        Self::Pms519C,
        Self::Pms519U,
        Self::Pms5195C,
        Self::Pms5195U,
        Self::Pms520C,
        Self::Pms520U,
        Self::Pms5205C,
        Self::Pms5205U,
        Self::Pms521C,
        Self::Pms521U,
        Self::Pms5215C,
        Self::Pms5215U,
        Self::Pms522C,
        Self::Pms522U,
        Self::Pms5225C,
        Self::Pms5225U,
        Self::Pms523C,
        Self::Pms523U,
        Self::Pms5235C,
        Self::Pms5235U,
        Self::Pms524C,
        Self::Pms524U,
        Self::Pms5245C,
        Self::Pms5245U,
        Self::Pms525C,
        Self::Pms525U,
        Self::Pms5255C,
        Self::Pms5255U,
        Self::Pms526C,
        Self::Pms526U,
        Self::Pms5265C,
        Self::Pms5265U,
        Self::Pms527C,
        Self::Pms527U,
        Self::Pms5275C,
        Self::Pms5275U,
        Self::Pms528C,
        Self::Pms528U,
        Self::Pms5285C,
        Self::Pms5285U,
        Self::Pms529C,
        Self::Pms529U,
        Self::Pms5295C,
        Self::Pms5295U,
        Self::Pms530C,
        Self::Pms530U,
        Self::Pms5305C,
        Self::Pms5305U,
        Self::Pms531C,
        Self::Pms531U,
        Self::Pms5315C,
        Self::Pms5315U,
        Self::Pms532C,
        Self::Pms532U,
        Self::Pms533C,
        Self::Pms533U,
        Self::Pms534C,
        Self::Pms534U,
        Self::Pms535C,
        Self::Pms535U,
        Self::Pms536C,
        Self::Pms536U,
        Self::Pms537C,
        Self::Pms537U,
        Self::Pms538C,
        Self::Pms538U,
        Self::Pms539C,
        Self::Pms539U,
        Self::Pms5395C,
        Self::Pms5395U,
        Self::Pms540C,
        Self::Pms540U,
        Self::Pms5405C,
        Self::Pms5405U,
        Self::Pms541C,
        Self::Pms541U,
        Self::Pms5415C,
        Self::Pms5415U,
        Self::Pms542C,
        Self::Pms542U,
        Self::Pms5425C,
        Self::Pms5425U,
        Self::Pms543C,
        Self::Pms543U,
        Self::Pms5435C,
        Self::Pms5435U,
        Self::Pms544C,
        Self::Pms544U,
        Self::Pms5445C,
        Self::Pms5445U,
        Self::Pms545C,
        Self::Pms545U,
        Self::Pms5455C,
        Self::Pms5455U,
        Self::Pms546C,
        Self::Pms546U,
        Self::Pms5463C,
        Self::Pms5463U,
        Self::Pms5467C,
        Self::Pms5467U,
        Self::Pms547C,
        Self::Pms547U,
        Self::Pms5473C,
        Self::Pms5473U,
        Self::Pms5477C,
        Self::Pms5477U,
        Self::Pms548C,
        Self::Pms548U,
        Self::Pms5483C,
        Self::Pms5483U,
        Self::Pms5487C,
        Self::Pms5487U,
        Self::Pms549C,
        Self::Pms549U,
        Self::Pms5493C,
        Self::Pms5493U,
        Self::Pms5497C,
        Self::Pms5497U,
        Self::Pms550C,
        Self::Pms550U,
        Self::Pms5503C,
        Self::Pms5503U,
        Self::Pms5507C,
        Self::Pms5507U,
        Self::Pms551C,
        Self::Pms551U,
        Self::Pms5513C,
        Self::Pms5513U,
        Self::Pms5517C,
        Self::Pms5517U,
        Self::Pms552C,
        Self::Pms552U,
        Self::Pms5523C,
        Self::Pms5523U,
        Self::Pms5527C,
        Self::Pms5527U,
        Self::Pms553C,
        Self::Pms553U,
        Self::Pms5535C,
        Self::Pms5535U,
        Self::Pms554C,
        Self::Pms554U,
        Self::Pms5545C,
        Self::Pms5545U,
        Self::Pms555C,
        Self::Pms555U,
        Self::Pms5555C,
        Self::Pms5555U,
        Self::Pms556C,
        Self::Pms556U,
        Self::Pms5565C,
        Self::Pms5565U,
        Self::Pms557C,
        Self::Pms557U,
        Self::Pms5575C,
        Self::Pms5575U,
        Self::Pms558C,
        Self::Pms558U,
        Self::Pms5585C,
        Self::Pms5585U,
        Self::Pms559C,
        Self::Pms559U,
        Self::Pms5595C,
        Self::Pms5595U,
        Self::Pms560C,
        Self::Pms560U,
        Self::Pms5605C,
        Self::Pms5605U,
        Self::Pms561C,
        Self::Pms561U,
        Self::Pms5615C,
        Self::Pms5615U,
        Self::Pms562C,
        Self::Pms562U,
        Self::Pms5625C,
        Self::Pms5625U,
        Self::Pms563C,
        Self::Pms563U,
        Self::Pms5635C,
        Self::Pms5635U,
        Self::Pms564C,
        Self::Pms564U,
        Self::Pms5645C,
        Self::Pms5645U,
        Self::Pms565C,
        Self::Pms565U,
        Self::Pms5655C,
        Self::Pms5655U,
        Self::Pms566C,
        Self::Pms566U,
        Self::Pms5665C,
        Self::Pms5665U,
        Self::Pms567C,
        Self::Pms567U,
        Self::Pms568C,
        Self::Pms568U,
        Self::Pms569C,
        Self::Pms569U,
        Self::Pms570C,
        Self::Pms570U,
        Self::Pms571C,
        Self::Pms571U,
        Self::Pms572C,
        Self::Pms572U,
        Self::Pms573C,
        Self::Pms573U,
        Self::Pms574C,
        Self::Pms574U,
        Self::Pms5743C,
        Self::Pms5743U,
        Self::Pms5747C,
        Self::Pms5747U,
        Self::Pms575C,
        Self::Pms575U,
        Self::Pms5753C,
        Self::Pms5753U,
        Self::Pms5757C,
        Self::Pms5757U,
        Self::Pms576C,
        Self::Pms576U,
        Self::Pms5763C,
        Self::Pms5763U,
        Self::Pms5767C,
        Self::Pms5767U,
        Self::Pms577C,
        Self::Pms577U,
        Self::Pms5773C,
        Self::Pms5773U,
        Self::Pms5777C,
        Self::Pms5777U,
        Self::Pms578C,
        Self::Pms578U,
        Self::Pms5783C,
        Self::Pms5783U,
        Self::Pms5787C,
        Self::Pms5787U,
        Self::Pms579C,
        Self::Pms579U,
        Self::Pms5793C,
        Self::Pms5793U,
        Self::Pms5797C,
        Self::Pms5797U,
        Self::Pms580C,
        Self::Pms580U,
        Self::Pms5803C,
        Self::Pms5803U,
        Self::Pms5807C,
        Self::Pms5807U,
        Self::Pms581C,
        Self::Pms581U,
        Self::Pms5815C,
        Self::Pms5815U,
        Self::Pms582C,
        Self::Pms582U,
        Self::Pms5825C,
        Self::Pms5825U,
        Self::Pms583C,
        Self::Pms583U,
        Self::Pms5835C,
        Self::Pms5835U,
        Self::Pms584C,
        Self::Pms584U,
        Self::Pms5845C,
        Self::Pms5845U,
        Self::Pms585C,
        Self::Pms585U,
        Self::Pms5855C,
        Self::Pms5855U,
        Self::Pms586C,
        Self::Pms586U,
        Self::Pms5865C,
        Self::Pms5865U,
        Self::Pms587C,
        Self::Pms587U,
        Self::Pms5875C,
        Self::Pms5875U,
        Self::Pms600C,
        Self::Pms600U,
        Self::Pms601C,
        Self::Pms601U,
        Self::Pms602C,
        Self::Pms602U,
        Self::Pms603C,
        Self::Pms603U,
        Self::Pms604C,
        Self::Pms604U,
        Self::Pms605C,
        Self::Pms605U,
        Self::Pms606C,
        Self::Pms606U,
        Self::Pms607C,
        Self::Pms607U,
        Self::Pms608C,
        Self::Pms608U,
        Self::Pms609C,
        Self::Pms609U,
        Self::Pms610C,
        Self::Pms610U,
        Self::Pms611C,
        Self::Pms611U,
        Self::Pms612C,
        Self::Pms612U,
        Self::Pms613C,
        Self::Pms613U,
        Self::Pms614C,
        Self::Pms614U,
        Self::Pms615C,
        Self::Pms615U,
        Self::Pms616C,
        Self::Pms616U,
        Self::Pms617C,
        Self::Pms617U,
        Self::Pms618C,
        Self::Pms618U,
        Self::Pms619C,
        Self::Pms619U,
        Self::Pms620C,
        Self::Pms620U,
        Self::Pms621C,
        Self::Pms621U,
        Self::Pms622C,
        Self::Pms622U,
        Self::Pms623C,
        Self::Pms623U,
        Self::Pms624C,
        Self::Pms624U,
        Self::Pms625C,
        Self::Pms625U,
        Self::Pms626C,
        Self::Pms626U,
        Self::Pms627C,
        Self::Pms627U,
        Self::Pms628C,
        Self::Pms628U,
        Self::Pms629C,
        Self::Pms629U,
        Self::Pms630C,
        Self::Pms630U,
        Self::Pms631C,
        Self::Pms631U,
        Self::Pms632C,
        Self::Pms632U,
        Self::Pms633C,
        Self::Pms633U,
        Self::Pms634C,
        Self::Pms634U,
        Self::Pms635C,
        Self::Pms635U,
        Self::Pms636C,
        Self::Pms636U,
        Self::Pms637C,
        Self::Pms637U,
        Self::Pms638C,
        Self::Pms638U,
        Self::Pms639C,
        Self::Pms639U,
        Self::Pms640C,
        Self::Pms640U,
        Self::Pms641C,
        Self::Pms641U,
        Self::Pms642C,
        Self::Pms642U,
        Self::Pms643C,
        Self::Pms643U,
        Self::Pms644C,
        Self::Pms644U,
        Self::Pms645C,
        Self::Pms645U,
        Self::Pms646C,
        Self::Pms646U,
        Self::Pms647C,
        Self::Pms647U,
        Self::Pms648C,
        Self::Pms648U,
        Self::Pms649C,
        Self::Pms649U,
        Self::Pms650C,
        Self::Pms650U,
        Self::Pms651C,
        Self::Pms651U,
        Self::Pms652C,
        Self::Pms652U,
        Self::Pms653C,
        Self::Pms653U,
        Self::Pms654C,
        Self::Pms654U,
        Self::Pms655C,
        Self::Pms655U,
        Self::Pms656C,
        Self::Pms656U,
        Self::Pms657C,
        Self::Pms657U,
        Self::Pms658C,
        Self::Pms658U,
        Self::Pms659C,
        Self::Pms659U,
        Self::Pms660C,
        Self::Pms660U,
        Self::Pms661C,
        Self::Pms661U,
        Self::Pms662C,
        Self::Pms662U,
        Self::Pms663C,
        Self::Pms663U,
        Self::Pms664C,
        Self::Pms664U,
        Self::Pms665C,
        Self::Pms665U,
        Self::Pms666C,
        Self::Pms666U,
        Self::Pms667C,
        Self::Pms667U,
        Self::Pms668C,
        Self::Pms668U,
        Self::Pms669C,
        Self::Pms669U,
        Self::Pms670C,
        Self::Pms670U,
        Self::Pms671C,
        Self::Pms671U,
        Self::Pms672C,
        Self::Pms672U,
        Self::Pms673C,
        Self::Pms673U,
        Self::Pms674C,
        Self::Pms674U,
        Self::Pms675C,
        Self::Pms675U,
        Self::Pms676C,
        Self::Pms676U,
        Self::Pms677C,
        Self::Pms677U,
        Self::Pms678C,
        Self::Pms678U,
        Self::Pms679C,
        Self::Pms679U,
        Self::Pms680C,
        Self::Pms680U,
        Self::Pms681C,
        Self::Pms681U,
        Self::Pms682C,
        Self::Pms682U,
        Self::Pms683C,
        Self::Pms683U,
        Self::Pms684C,
        Self::Pms684U,
        Self::Pms685C,
        Self::Pms685U,
        Self::Pms686C,
        Self::Pms686U,
        Self::Pms687C,
        Self::Pms687U,
        Self::Pms688C,
        Self::Pms688U,
        Self::Pms689C,
        Self::Pms689U,
        Self::Pms690C,
        Self::Pms690U,
        Self::Pms691C,
        Self::Pms691U,
        Self::Pms692C,
        Self::Pms692U,
        Self::Pms693C,
        Self::Pms693U,
        Self::Pms694C,
        Self::Pms694U,
        Self::Pms695C,
        Self::Pms695U,
        Self::Pms696C,
        Self::Pms696U,
        Self::Pms697C,
        Self::Pms697U,
        Self::Pms698C,
        Self::Pms698U,
        Self::Pms699C,
        Self::Pms699U,
        Self::Pms700C,
        Self::Pms700U,
        Self::Pms701C,
        Self::Pms701U,
        Self::Pms702C,
        Self::Pms702U,
        Self::Pms703C,
        Self::Pms703U,
        Self::Pms704C,
        Self::Pms704U,
        Self::Pms705C,
        Self::Pms705U,
        Self::Pms706C,
        Self::Pms706U,
        Self::Pms707C,
        Self::Pms707U,
        Self::Pms708C,
        Self::Pms708U,
        Self::Pms709C,
        Self::Pms709U,
        Self::Pms710C,
        Self::Pms710U,
        Self::Pms711C,
        Self::Pms711U,
        Self::Pms712C,
        Self::Pms712U,
        Self::Pms713C,
        Self::Pms713U,
        Self::Pms714C,
        Self::Pms714U,
        Self::Pms715C,
        Self::Pms715U,
        Self::Pms716C,
        Self::Pms716U,
        Self::Pms717C,
        Self::Pms717U,
        Self::Pms718C,
        Self::Pms718U,
        Self::Pms719C,
        Self::Pms719U,
        Self::Pms720C,
        Self::Pms720U,
        Self::Pms721C,
        Self::Pms721U,
        Self::Pms722C,
        Self::Pms722U,
        Self::Pms723C,
        Self::Pms723U,
        Self::Pms724C,
        Self::Pms724U,
        Self::Pms725C,
        Self::Pms725U,
        Self::Pms726C,
        Self::Pms726U,
        Self::Pms727C,
        Self::Pms727U,
        Self::Pms728C,
        Self::Pms728U,
        Self::Pms729C,
        Self::Pms729U,
        Self::Pms730C,
        Self::Pms730U,
        Self::Pms731C,
        Self::Pms731U,
        Self::Pms732C,
        Self::Pms732U,
        Self::Pms7401C,
        Self::Pms7401U,
        Self::Pms7402C,
        Self::Pms7402U,
        Self::Pms7403C,
        Self::Pms7403U,
        Self::Pms7404C,
        Self::Pms7404U,
        Self::Pms7405C,
        Self::Pms7405U,
        Self::Pms7406C,
        Self::Pms7406U,
        Self::Pms7407C,
        Self::Pms7407U,
        Self::Pms7408C,
        Self::Pms7408U,
        Self::Pms7409C,
        Self::Pms7409U,
        Self::Pms7410C,
        Self::Pms7410U,
        Self::Pms7411C,
        Self::Pms7411U,
        Self::Pms7412C,
        Self::Pms7412U,
        Self::Pms7413C,
        Self::Pms7413U,
        Self::Pms7414C,
        Self::Pms7414U,
        Self::Pms7415C,
        Self::Pms7415U,
        Self::Pms7416C,
        Self::Pms7416U,
        Self::Pms7417C,
        Self::Pms7417U,
        Self::Pms7418C,
        Self::Pms7418U,
        Self::Pms7419C,
        Self::Pms7419U,
        Self::Pms7420C,
        Self::Pms7420U,
        Self::Pms7421C,
        Self::Pms7421U,
        Self::Pms7422C,
        Self::Pms7422U,
        Self::Pms7423C,
        Self::Pms7423U,
        Self::Pms7424C,
        Self::Pms7424U,
        Self::Pms7425C,
        Self::Pms7425U,
        Self::Pms7426C,
        Self::Pms7426U,
        Self::Pms7427C,
        Self::Pms7427U,
        Self::Pms7428C,
        Self::Pms7428U,
        Self::Pms7429C,
        Self::Pms7429U,
        Self::Pms7430C,
        Self::Pms7430U,
        Self::Pms7431C,
        Self::Pms7431U,
        Self::Pms7432C,
        Self::Pms7432U,
        Self::Pms7433C,
        Self::Pms7433U,
        Self::Pms7434C,
        Self::Pms7434U,
        Self::Pms7435C,
        Self::Pms7435U,
        Self::Pms7436C,
        Self::Pms7436U,
        Self::Pms7437C,
        Self::Pms7437U,
        Self::Pms7438C,
        Self::Pms7438U,
        Self::Pms7439C,
        Self::Pms7439U,
        Self::Pms7440C,
        Self::Pms7440U,
        Self::Pms7441C,
        Self::Pms7441U,
        Self::Pms7442C,
        Self::Pms7442U,
        Self::Pms7443C,
        Self::Pms7443U,
        Self::Pms7444C,
        Self::Pms7444U,
        Self::Pms7445C,
        Self::Pms7445U,
        Self::Pms7446C,
        Self::Pms7446U,
        Self::Pms7447C,
        Self::Pms7447U,
        Self::Pms7448C,
        Self::Pms7448U,
        Self::Pms7449C,
        Self::Pms7449U,
        Self::Pms7450C,
        Self::Pms7450U,
        Self::Pms7451C,
        Self::Pms7451U,
        Self::Pms7452C,
        Self::Pms7452U,
        Self::Pms7453C,
        Self::Pms7453U,
        Self::Pms7454C,
        Self::Pms7454U,
        Self::Pms7455C,
        Self::Pms7455U,
        Self::Pms7456C,
        Self::Pms7456U,
        Self::Pms7457C,
        Self::Pms7457U,
        Self::Pms7458C,
        Self::Pms7458U,
        Self::Pms7459C,
        Self::Pms7459U,
        Self::Pms7460C,
        Self::Pms7460U,
        Self::Pms7461C,
        Self::Pms7461U,
        Self::Pms7462C,
        Self::Pms7462U,
        Self::Pms7463C,
        Self::Pms7463U,
        Self::Pms7464C,
        Self::Pms7464U,
        Self::Pms7465C,
        Self::Pms7465U,
        Self::Pms7466C,
        Self::Pms7466U,
        Self::Pms7467C,
        Self::Pms7467U,
        Self::Pms7468C,
        Self::Pms7468U,
        Self::Pms7469C,
        Self::Pms7469U,
        Self::Pms7470C,
        Self::Pms7470U,
        Self::Pms7471C,
        Self::Pms7471U,
        Self::Pms7472C,
        Self::Pms7472U,
        Self::Pms7473C,
        Self::Pms7473U,
        Self::Pms7474C,
        Self::Pms7474U,
        Self::Pms7475C,
        Self::Pms7475U,
        Self::Pms7476C,
        Self::Pms7476U,
        Self::Pms7477C,
        Self::Pms7477U,
        Self::Pms7478C,
        Self::Pms7478U,
        Self::Pms7479C,
        Self::Pms7479U,
        Self::Pms7480C,
        Self::Pms7480U,
        Self::Pms7481C,
        Self::Pms7481U,
        Self::Pms7482C,
        Self::Pms7482U,
        Self::Pms7483C,
        Self::Pms7483U,
        Self::Pms7484C,
        Self::Pms7484U,
        Self::Pms7485C,
        Self::Pms7485U,
        Self::Pms7486C,
        Self::Pms7486U,
        Self::Pms7487C,
        Self::Pms7487U,
        Self::Pms7488C,
        Self::Pms7488U,
        Self::Pms7489C,
        Self::Pms7489U,
        Self::Pms7490C,
        Self::Pms7490U,
        Self::Pms7491C,
        Self::Pms7491U,
        Self::Pms7492C,
        Self::Pms7492U,
        Self::Pms7493C,
        Self::Pms7493U,
        Self::Pms7494C,
        Self::Pms7494U,
        Self::Pms7495C,
        Self::Pms7495U,
        Self::Pms7496C,
        Self::Pms7496U,
        Self::Pms7497C,
        Self::Pms7497U,
        Self::Pms7498C,
        Self::Pms7498U,
        Self::Pms7499C,
        Self::Pms7499U,
        Self::Pms7500C,
        Self::Pms7500U,
        Self::Pms7501C,
        Self::Pms7501U,
        Self::Pms7502C,
        Self::Pms7502U,
        Self::Pms7503C,
        Self::Pms7503U,
        Self::Pms7504C,
        Self::Pms7504U,
        Self::Pms7505C,
        Self::Pms7505U,
        Self::Pms7506C,
        Self::Pms7506U,
        Self::Pms7507C,
        Self::Pms7507U,
        Self::Pms7508C,
        Self::Pms7508U,
        Self::Pms7509C,
        Self::Pms7509U,
        Self::Pms7510C,
        Self::Pms7510U,
        Self::Pms7511C,
        Self::Pms7511U,
        Self::Pms7512C,
        Self::Pms7512U,
        Self::Pms7513C,
        Self::Pms7513U,
        Self::Pms7514C,
        Self::Pms7514U,
        Self::Pms7515C,
        Self::Pms7515U,
        Self::Pms7516C,
        Self::Pms7516U,
        Self::Pms7517C,
        Self::Pms7517U,
        Self::Pms7518C,
        Self::Pms7518U,
        Self::Pms7519C,
        Self::Pms7519U,
        Self::Pms7520C,
        Self::Pms7520U,
        Self::Pms7521C,
        Self::Pms7521U,
        Self::Pms7522C,
        Self::Pms7522U,
        Self::Pms7523C,
        Self::Pms7523U,
        Self::Pms7524C,
        Self::Pms7524U,
        Self::Pms7525C,
        Self::Pms7525U,
        Self::Pms7526C,
        Self::Pms7526U,
        Self::Pms7527C,
        Self::Pms7527U,
        Self::Pms7528C,
        Self::Pms7528U,
        Self::Pms7529C,
        Self::Pms7529U,
        Self::Pms7530C,
        Self::Pms7530U,
        Self::Pms7531C,
        Self::Pms7531U,
        Self::Pms7532C,
        Self::Pms7532U,
        Self::Pms7533C,
        Self::Pms7533U,
        Self::Pms7534C,
        Self::Pms7534U,
        Self::Pms7535C,
        Self::Pms7535U,
        Self::Pms7536C,
        Self::Pms7536U,
        Self::Pms7537C,
        Self::Pms7537U,
        Self::Pms7538C,
        Self::Pms7538U,
        Self::Pms7539C,
        Self::Pms7539U,
        Self::Pms7540C,
        Self::Pms7540U,
        Self::Pms7541C,
        Self::Pms7541U,
        Self::Pms7542C,
        Self::Pms7542U,
        Self::Pms7543C,
        Self::Pms7543U,
        Self::Pms7544C,
        Self::Pms7544U,
        Self::Pms7545C,
        Self::Pms7545U,
        Self::Pms7546C,
        Self::Pms7546U,
        Self::Pms7547C,
        Self::Pms7547U,
        Self::Pms7548C,
        Self::Pms7548U,
        Self::Pms7549C,
        Self::Pms7549U,
        Self::Pms7550C,
        Self::Pms7550U,
        Self::Pms7551C,
        Self::Pms7551U,
        Self::Pms7552C,
        Self::Pms7552U,
        Self::Pms7553C,
        Self::Pms7553U,
        Self::Pms7554C,
        Self::Pms7554U,
        Self::Pms7555C,
        Self::Pms7555U,
        Self::Pms7556C,
        Self::Pms7556U,
        Self::Pms7557C,
        Self::Pms7557U,
        Self::Pms7558C,
        Self::Pms7558U,
        Self::Pms7559C,
        Self::Pms7559U,
        Self::Pms7560C,
        Self::Pms7560U,
        Self::Pms7561C,
        Self::Pms7561U,
        Self::Pms7562C,
        Self::Pms7562U,
        Self::Pms7563C,
        Self::Pms7563U,
        Self::Pms7564C,
        Self::Pms7564U,
        Self::Pms7565C,
        Self::Pms7565U,
        Self::Pms7566C,
        Self::Pms7566U,
        Self::Pms7567C,
        Self::Pms7567U,
        Self::Pms7568C,
        Self::Pms7568U,
        Self::Pms7569C,
        Self::Pms7569U,
        Self::Pms7570C,
        Self::Pms7570U,
        Self::Pms7571C,
        Self::Pms7571U,
        Self::Pms7572C,
        Self::Pms7572U,
        Self::Pms7573C,
        Self::Pms7573U,
        Self::Pms7574C,
        Self::Pms7574U,
        Self::Pms7575C,
        Self::Pms7575U,
        Self::Pms7576C,
        Self::Pms7576U,
        Self::Pms7577C,
        Self::Pms7577U,
        Self::Pms7578C,
        Self::Pms7578U,
        Self::Pms7579C,
        Self::Pms7579U,
        Self::Pms7580C,
        Self::Pms7580U,
        Self::Pms7581C,
        Self::Pms7581U,
        Self::Pms7582C,
        Self::Pms7582U,
        Self::Pms7583C,
        Self::Pms7583U,
        Self::Pms7584C,
        Self::Pms7584U,
        Self::Pms7585C,
        Self::Pms7585U,
        Self::Pms7586C,
        Self::Pms7586U,
        Self::Pms7587C,
        Self::Pms7587U,
        Self::Pms7588C,
        Self::Pms7588U,
        Self::Pms7589C,
        Self::Pms7589U,
        Self::Pms7590C,
        Self::Pms7590U,
        Self::Pms7591C,
        Self::Pms7591U,
        Self::Pms7592C,
        Self::Pms7592U,
        Self::Pms7593C,
        Self::Pms7593U,
        Self::Pms7594C,
        Self::Pms7594U,
        Self::Pms7595C,
        Self::Pms7595U,
        Self::Pms7596C,
        Self::Pms7596U,
        Self::Pms7597C,
        Self::Pms7597U,
        Self::Pms7598C,
        Self::Pms7598U,
        Self::Pms7599C,
        Self::Pms7599U,
        Self::Pms7600C,
        Self::Pms7600U,
        Self::Pms7601C,
        Self::Pms7601U,
        Self::Pms7602C,
        Self::Pms7602U,
        Self::Pms7603C,
        Self::Pms7603U,
        Self::Pms7604C,
        Self::Pms7604U,
        Self::Pms7605C,
        Self::Pms7605U,
        Self::Pms7606C,
        Self::Pms7606U,
        Self::Pms7607C,
        Self::Pms7607U,
        Self::Pms7608C,
        Self::Pms7608U,
        Self::Pms7609C,
        Self::Pms7609U,
        Self::Pms7610C,
        Self::Pms7610U,
        Self::Pms7611C,
        Self::Pms7611U,
        Self::Pms7612C,
        Self::Pms7612U,
        Self::Pms7613C,
        Self::Pms7613U,
        Self::Pms7614C,
        Self::Pms7614U,
        Self::Pms7615C,
        Self::Pms7615U,
        Self::Pms7616C,
        Self::Pms7616U,
        Self::Pms7617C,
        Self::Pms7617U,
        Self::Pms7618C,
        Self::Pms7618U,
        Self::Pms7619C,
        Self::Pms7619U,
        Self::Pms7620C,
        Self::Pms7620U,
        Self::Pms7621C,
        Self::Pms7621U,
        Self::Pms7622C,
        Self::Pms7622U,
        Self::Pms7623C,
        Self::Pms7623U,
        Self::Pms7624C,
        Self::Pms7624U,
        Self::Pms7625C,
        Self::Pms7625U,
        Self::Pms7626C,
        Self::Pms7626U,
        Self::Pms7627C,
        Self::Pms7627U,
        Self::Pms7628C,
        Self::Pms7628U,
        Self::Pms7629C,
        Self::Pms7629U,
        Self::Pms7630C,
        Self::Pms7630U,
        Self::Pms7631C,
        Self::Pms7631U,
        Self::Pms7632C,
        Self::Pms7632U,
        Self::Pms7633C,
        Self::Pms7633U,
        Self::Pms7634C,
        Self::Pms7634U,
        Self::Pms7635C,
        Self::Pms7635U,
        Self::Pms7636C,
        Self::Pms7636U,
        Self::Pms7637C,
        Self::Pms7637U,
        Self::Pms7638C,
        Self::Pms7638U,
        Self::Pms7639C,
        Self::Pms7639U,
        Self::Pms7640C,
        Self::Pms7640U,
        Self::Pms7641C,
        Self::Pms7641U,
        Self::Pms7642C,
        Self::Pms7642U,
        Self::Pms7643C,
        Self::Pms7643U,
        Self::Pms7644C,
        Self::Pms7644U,
        Self::Pms7645C,
        Self::Pms7645U,
        Self::Pms7646C,
        Self::Pms7646U,
        Self::Pms7647C,
        Self::Pms7647U,
        Self::Pms7648C,
        Self::Pms7648U,
        Self::Pms7649C,
        Self::Pms7649U,
        Self::Pms7650C,
        Self::Pms7650U,
        Self::Pms7651C,
        Self::Pms7651U,
        Self::Pms7652C,
        Self::Pms7652U,
        Self::Pms7653C,
        Self::Pms7653U,
        Self::Pms7654C,
        Self::Pms7654U,
        Self::Pms7655C,
        Self::Pms7655U,
        Self::Pms7656C,
        Self::Pms7656U,
        Self::Pms7657C,
        Self::Pms7657U,
        Self::Pms7658C,
        Self::Pms7658U,
        Self::Pms7659C,
        Self::Pms7659U,
        Self::Pms7660C,
        Self::Pms7660U,
        Self::Pms7661C,
        Self::Pms7661U,
        Self::Pms7662C,
        Self::Pms7662U,
        Self::Pms7663C,
        Self::Pms7663U,
        Self::Pms7664C,
        Self::Pms7664U,
        Self::Pms7665C,
        Self::Pms7665U,
        Self::Pms7666C,
        Self::Pms7666U,
        Self::Pms7667C,
        Self::Pms7667U,
        Self::Pms7668C,
        Self::Pms7668U,
        Self::Pms7669C,
        Self::Pms7669U,
        Self::Pms7670C,
        Self::Pms7670U,
        Self::Pms7671C,
        Self::Pms7671U,
        Self::Pms7672C,
        Self::Pms7672U,
        Self::Pms7673C,
        Self::Pms7673U,
        Self::Pms7674C,
        Self::Pms7674U,
        Self::Pms7675C,
        Self::Pms7675U,
        Self::Pms7676C,
        Self::Pms7676U,
        Self::Pms7677C,
        Self::Pms7677U,
        Self::Pms7678C,
        Self::Pms7678U,
        Self::Pms7679C,
        Self::Pms7679U,
        Self::Pms7680C,
        Self::Pms7680U,
        Self::Pms7681C,
        Self::Pms7681U,
        Self::Pms7682C,
        Self::Pms7682U,
        Self::Pms7683C,
        Self::Pms7683U,
        Self::Pms7684C,
        Self::Pms7684U,
        Self::Pms7685C,
        Self::Pms7685U,
        Self::Pms7686C,
        Self::Pms7686U,
        Self::Pms7687C,
        Self::Pms7687U,
        Self::Pms7688C,
        Self::Pms7688U,
        Self::Pms7689C,
        Self::Pms7689U,
        Self::Pms7690C,
        Self::Pms7690U,
        Self::Pms7691C,
        Self::Pms7691U,
        Self::Pms7692C,
        Self::Pms7692U,
        Self::Pms7693C,
        Self::Pms7693U,
        Self::Pms7694C,
        Self::Pms7694U,
        Self::Pms7695C,
        Self::Pms7695U,
        Self::Pms7696C,
        Self::Pms7696U,
        Self::Pms7697C,
        Self::Pms7697U,
        Self::Pms7698C,
        Self::Pms7698U,
        Self::Pms7699C,
        Self::Pms7699U,
        Self::Pms7700C,
        Self::Pms7700U,
        Self::Pms7701C,
        Self::Pms7701U,
        Self::Pms7702C,
        Self::Pms7702U,
        Self::Pms7703C,
        Self::Pms7703U,
        Self::Pms7704C,
        Self::Pms7704U,
        Self::Pms7705C,
        Self::Pms7705U,
        Self::Pms7706C,
        Self::Pms7706U,
        Self::Pms7707C,
        Self::Pms7707U,
        Self::Pms7708C,
        Self::Pms7708U,
        Self::Pms7709C,
        Self::Pms7709U,
        Self::Pms7710C,
        Self::Pms7710U,
        Self::Pms7711C,
        Self::Pms7711U,
        Self::Pms7712C,
        Self::Pms7712U,
        Self::Pms7713C,
        Self::Pms7713U,
        Self::Pms7714C,
        Self::Pms7714U,
        Self::Pms7715C,
        Self::Pms7715U,
        Self::Pms7716C,
        Self::Pms7716U,
        Self::Pms7717C,
        Self::Pms7717U,
        Self::Pms7718C,
        Self::Pms7718U,
        Self::Pms7719C,
        Self::Pms7719U,
        Self::Pms7720C,
        Self::Pms7720U,
        Self::Pms7721C,
        Self::Pms7721U,
        Self::Pms7722C,
        Self::Pms7722U,
        Self::Pms7723C,
        Self::Pms7723U,
        Self::Pms7724C,
        Self::Pms7724U,
        Self::Pms7725C,
        Self::Pms7725U,
        Self::Pms7726C,
        Self::Pms7726U,
        Self::Pms7727C,
        Self::Pms7727U,
        Self::Pms7728C,
        Self::Pms7728U,
        Self::Pms7729C,
        Self::Pms7729U,
        Self::Pms7730C,
        Self::Pms7730U,
        Self::Pms7731C,
        Self::Pms7731U,
        Self::Pms7732C,
        Self::Pms7732U,
        Self::Pms7733C,
        Self::Pms7733U,
        Self::Pms7734C,
        Self::Pms7734U,
        Self::Pms7735C,
        Self::Pms7735U,
        Self::Pms7736C,
        Self::Pms7736U,
        Self::Pms7737C,
        Self::Pms7737U,
        Self::Pms7738C,
        Self::Pms7738U,
        Self::Pms7739C,
        Self::Pms7739U,
        Self::Pms7740C,
        Self::Pms7740U,
        Self::Pms7741C,
        Self::Pms7741U,
        Self::Pms7742C,
        Self::Pms7742U,
        Self::Pms7743C,
        Self::Pms7743U,
        Self::Pms7744C,
        Self::Pms7744U,
        Self::Pms7745C,
        Self::Pms7745U,
        Self::Pms7746C,
        Self::Pms7746U,
        Self::Pms7747C,
        Self::Pms7747U,
        Self::Pms7748C,
        Self::Pms7748U,
        Self::Pms7749C,
        Self::Pms7749U,
        Self::Pms7750C,
        Self::Pms7750U,
        Self::Pms7751C,
        Self::Pms7751U,
        Self::Pms7752C,
        Self::Pms7752U,
        Self::Pms7753C,
        Self::Pms7753U,
        Self::Pms7754C,
        Self::Pms7754U,
        Self::Pms7755C,
        Self::Pms7755U,
        Self::Pms7756C,
        Self::Pms7756U,
        Self::Pms7757C,
        Self::Pms7757U,
        Self::Pms7758C,
        Self::Pms7758U,
        Self::Pms7759C,
        Self::Pms7759U,
        Self::Pms7760C,
        Self::Pms7760U,
        Self::Pms7761C,
        Self::Pms7761U,
        Self::Pms7762C,
        Self::Pms7762U,
        Self::Pms7763C,
        Self::Pms7763U,
        Self::Pms7764C,
        Self::Pms7764U,
        Self::Pms7765C,
        Self::Pms7765U,
        Self::Pms7766C,
        Self::Pms7766U,
        Self::Pms7767C,
        Self::Pms7767U,
        Self::Pms7768C,
        Self::Pms7768U,
        Self::Pms7769C,
        Self::Pms7769U,
        Self::Pms7770C,
        Self::Pms7770U,
        Self::Pms7771C,
        Self::Pms7771U,
        Self::Pms801C,
        Self::Pms801U,
        Self::Pms802C,
        Self::Pms802U,
        Self::Pms803C,
        Self::Pms803U,
        Self::Pms804C,
        Self::Pms804U,
        Self::Pms805C,
        Self::Pms805U,
        Self::Pms806C,
        Self::Pms806U,
        Self::Pms807C,
        Self::Pms807U,
        Self::Pms871C,
        Self::Pms871U,
        Self::Pms872C,
        Self::Pms872U,
        Self::Pms873C,
        Self::Pms873U,
        Self::Pms874C,
        Self::Pms874U,
        Self::Pms875C,
        Self::Pms875U,
        Self::Pms876C,
        Self::Pms876U,
        Self::Pms877C,
        Self::Pms877U,
        Self::Black0961C,
        Self::Black0961U,
        Self::Black2C,
        Self::Black2U,
        Self::Black3C,
        Self::Black3U,
        Self::Black4C,
        Self::Black4U,
        Self::Black5C,
        Self::Black5U,
        Self::Black6C,
        Self::Black6U,
        Self::Black7C,
        Self::Black7U,
        Self::BlackC,
        Self::BlackU,
        Self::Blue072C,
        Self::Blue072U,
        Self::Blue0821C,
        Self::Blue0821U,
        Self::BrightGreenC,
        Self::BrightOrangeC,
        Self::BrightRedC,
        Self::BrightRedU,
        Self::CoolGray1C,
        Self::CoolGray1U,
        Self::CoolGray10C,
        Self::CoolGray10U,
        Self::CoolGray11C,
        Self::CoolGray11U,
        Self::CoolGray2C,
        Self::CoolGray2U,
        Self::CoolGray3C,
        Self::CoolGray3U,
        Self::CoolGray4C,
        Self::CoolGray4U,
        Self::CoolGray5C,
        Self::CoolGray5U,
        Self::CoolGray6C,
        Self::CoolGray6U,
        Self::CoolGray7C,
        Self::CoolGray7U,
        Self::CoolGray8C,
        Self::CoolGray8U,
        Self::CoolGray9C,
        Self::CoolGray9U,
        Self::DarkBlueC,
        Self::DarkBlueU,
        Self::Green0921C,
        Self::Green0921U,
        Self::GreenC,
        Self::GreenU,
        Self::Magenta0521C,
        Self::Magenta0521U,
        Self::MediumBlueC,
        Self::MediumPurpleC,
        Self::MediumPurpleU,
        Self::MediumYellowC,
        Self::NeutralBlackC,
        Self::Orange021C,
        Self::Orange021U,
        Self::PinkC,
        Self::PinkU,
        Self::ProcessBlueC,
        Self::ProcessBlueU,
        Self::PurpleC,
        Self::PurpleU,
        Self::Red032C,
        Self::Red032U,
        Self::Red0331C,
        Self::Red0331U,
        Self::ReflexBlueC,
        Self::ReflexBlueU,
        Self::RhodamineRedC,
        Self::RhodamineRedU,
        Self::RubineRedC,
        Self::RubineRedU,
        Self::StrongRedC,
        Self::Violet0631C,
        Self::Violet0631U,
        Self::VioletC,
        Self::VioletU,
        Self::WarmGray1C,
        Self::WarmGray1U,
        Self::WarmGray10C,
        Self::WarmGray10U,
        Self::WarmGray11C,
        Self::WarmGray11U,
        Self::WarmGray2C,
        Self::WarmGray2U,
        Self::WarmGray3C,
        Self::WarmGray3U,
        Self::WarmGray4C,
        Self::WarmGray4U,
        Self::WarmGray5C,
        Self::WarmGray5U,
        Self::WarmGray6C,
        Self::WarmGray6U,
        Self::WarmGray7C,
        Self::WarmGray7U,
        Self::WarmGray8C,
        Self::WarmGray8U,
        Self::WarmGray9C,
        Self::WarmGray9U,
        Self::WarmRedC,
        Self::WarmRedU,
        Self::WhiteC,
        Self::Yellow012C,
        Self::Yellow012U,
        Self::Yellow0131C,
        Self::Yellow0131U,
        Self::YellowC,
        Self::YellowU,
    ];

    /// The fan-book variant encoded in the name's suffix letter.
    pub fn suffix(&self) -> PantoneSuffix {
        match self.as_str().bytes().last() {
            Some(b'U') => PantoneSuffix::Uncoated,
            Some(b'M') => PantoneSuffix::Matte,
            _ => PantoneSuffix::Coated,
        }
    }

    /// The canonical `Pantone <name> <suffix>` string.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Pms100C => "Pantone 100 C",
            Self::Pms100U => "Pantone 100 U",
            Self::Pms101C => "Pantone 101 C",
            Self::Pms101U => "Pantone 101 U",
            Self::Pms102C => "Pantone 102 C",
            Self::Pms102U => "Pantone 102 U",
            Self::Pms103C => "Pantone 103 C",
            Self::Pms103U => "Pantone 103 U",
            Self::Pms104C => "Pantone 104 C",
            Self::Pms104U => "Pantone 104 U",
            Self::Pms105C => "Pantone 105 C",
            Self::Pms105U => "Pantone 105 U",
            Self::Pms106C => "Pantone 106 C",
            Self::Pms106U => "Pantone 106 U",
            Self::Pms107C => "Pantone 107 C",
            Self::Pms107U => "Pantone 107 U",
            Self::Pms108C => "Pantone 108 C",
            Self::Pms108U => "Pantone 108 U",
            Self::Pms109C => "Pantone 109 C",
            Self::Pms109U => "Pantone 109 U",
            Self::Pms110C => "Pantone 110 C",
            Self::Pms110U => "Pantone 110 U",
            Self::Pms111C => "Pantone 111 C",
            Self::Pms111U => "Pantone 111 U",
            Self::Pms112C => "Pantone 112 C",
            Self::Pms112U => "Pantone 112 U",
            Self::Pms113C => "Pantone 113 C",
            Self::Pms113U => "Pantone 113 U",
            Self::Pms114C => "Pantone 114 C",
            Self::Pms114U => "Pantone 114 U",
            Self::Pms115C => "Pantone 115 C",
            Self::Pms115U => "Pantone 115 U",
            Self::Pms116C => "Pantone 116 C",
            Self::Pms116U => "Pantone 116 U",
            Self::Pms117C => "Pantone 117 C",
            Self::Pms117U => "Pantone 117 U",
            Self::Pms118C => "Pantone 118 C",
            Self::Pms118U => "Pantone 118 U",
            Self::Pms119C => "Pantone 119 C",
            Self::Pms119U => "Pantone 119 U",
            Self::Pms120C => "Pantone 120 C",
            Self::Pms120U => "Pantone 120 U",
            Self::Pms1205C => "Pantone 1205 C",
            Self::Pms1205U => "Pantone 1205 U",
            Self::Pms121C => "Pantone 121 C",
            Self::Pms121U => "Pantone 121 U",
            Self::Pms1215C => "Pantone 1215 C",
            Self::Pms1215U => "Pantone 1215 U",
            Self::Pms122C => "Pantone 122 C",
            Self::Pms122U => "Pantone 122 U",
            Self::Pms1225C => "Pantone 1225 C",
            Self::Pms1225U => "Pantone 1225 U",
            Self::Pms123C => "Pantone 123 C",
            Self::Pms123U => "Pantone 123 U",
            Self::Pms1235C => "Pantone 1235 C",
            Self::Pms1235U => "Pantone 1235 U",
            Self::Pms124C => "Pantone 124 C",
            Self::Pms124U => "Pantone 124 U",
            Self::Pms1245C => "Pantone 1245 C",
            Self::Pms1245U => "Pantone 1245 U",
            Self::Pms125C => "Pantone 125 C",
            Self::Pms125U => "Pantone 125 U",
            Self::Pms1255C => "Pantone 1255 C",
            Self::Pms1255U => "Pantone 1255 U",
            Self::Pms126C => "Pantone 126 C",
            Self::Pms126U => "Pantone 126 U",
            Self::Pms1265C => "Pantone 1265 C",
            Self::Pms1265U => "Pantone 1265 U",
            Self::Pms127C => "Pantone 127 C",
            Self::Pms127U => "Pantone 127 U",
            Self::Pms128C => "Pantone 128 C",
            Self::Pms128U => "Pantone 128 U",
            Self::Pms129C => "Pantone 129 C",
            Self::Pms129U => "Pantone 129 U",
            Self::Pms130C => "Pantone 130 C",
            Self::Pms130U => "Pantone 130 U",
            Self::Pms131C => "Pantone 131 C",
            Self::Pms131U => "Pantone 131 U",
            Self::Pms132C => "Pantone 132 C",
            Self::Pms132U => "Pantone 132 U",
            Self::Pms133C => "Pantone 133 C",
            Self::Pms133U => "Pantone 133 U",
            Self::Pms134C => "Pantone 134 C",
            Self::Pms134U => "Pantone 134 U",
            Self::Pms1345C => "Pantone 1345 C",
            Self::Pms1345U => "Pantone 1345 U",
            Self::Pms135C => "Pantone 135 C",
            Self::Pms135U => "Pantone 135 U",
            Self::Pms1355C => "Pantone 1355 C",
            Self::Pms1355U => "Pantone 1355 U",
            Self::Pms136C => "Pantone 136 C",
            Self::Pms136U => "Pantone 136 U",
            Self::Pms1365C => "Pantone 1365 C",
            Self::Pms1365U => "Pantone 1365 U",
            Self::Pms137C => "Pantone 137 C",
            Self::Pms137U => "Pantone 137 U",
            Self::Pms1375C => "Pantone 1375 C",
            Self::Pms1375U => "Pantone 1375 U",
            Self::Pms138C => "Pantone 138 C",
            Self::Pms138U => "Pantone 138 U",
            Self::Pms1385C => "Pantone 1385 C",
            Self::Pms1385U => "Pantone 1385 U",
            Self::Pms139C => "Pantone 139 C",
            Self::Pms139U => "Pantone 139 U",
            Self::Pms1395C => "Pantone 1395 C",
            Self::Pms1395U => "Pantone 1395 U",
            Self::Pms140C => "Pantone 140 C",
            Self::Pms140U => "Pantone 140 U",
            Self::Pms1405C => "Pantone 1405 C",
            Self::Pms1405U => "Pantone 1405 U",
            Self::Pms141C => "Pantone 141 C",
            Self::Pms141U => "Pantone 141 U",
            Self::Pms142C => "Pantone 142 C",
            Self::Pms142U => "Pantone 142 U",
            Self::Pms143C => "Pantone 143 C",
            Self::Pms143U => "Pantone 143 U",
            Self::Pms144C => "Pantone 144 C",
            Self::Pms144U => "Pantone 144 U",
            Self::Pms145C => "Pantone 145 C",
            Self::Pms145U => "Pantone 145 U",
            Self::Pms146C => "Pantone 146 C",
            Self::Pms146U => "Pantone 146 U",
            Self::Pms147C => "Pantone 147 C",
            Self::Pms147U => "Pantone 147 U",
            Self::Pms148C => "Pantone 148 C",
            Self::Pms148U => "Pantone 148 U",
            Self::Pms1485C => "Pantone 1485 C",
            Self::Pms1485U => "Pantone 1485 U",
            Self::Pms149C => "Pantone 149 C",
            Self::Pms149U => "Pantone 149 U",
            Self::Pms1495C => "Pantone 1495 C",
            Self::Pms1495U => "Pantone 1495 U",
            Self::Pms150C => "Pantone 150 C",
            Self::Pms150U => "Pantone 150 U",
            Self::Pms1505C => "Pantone 1505 C",
            Self::Pms1505U => "Pantone 1505 U",
            Self::Pms151C => "Pantone 151 C",
            Self::Pms151U => "Pantone 151 U",
            Self::Pms152C => "Pantone 152 C",
            Self::Pms152U => "Pantone 152 U",
            Self::Pms1525C => "Pantone 1525 C",
            Self::Pms1525U => "Pantone 1525 U",
            Self::Pms153C => "Pantone 153 C",
            Self::Pms153U => "Pantone 153 U",
            Self::Pms1535C => "Pantone 1535 C",
            Self::Pms1535U => "Pantone 1535 U",
            Self::Pms154C => "Pantone 154 C",
            Self::Pms154U => "Pantone 154 U",
            Self::Pms1545C => "Pantone 1545 C",
            Self::Pms1545U => "Pantone 1545 U",
            Self::Pms155C => "Pantone 155 C",
            Self::Pms155U => "Pantone 155 U",
            Self::Pms1555C => "Pantone 1555 C",
            Self::Pms1555U => "Pantone 1555 U",
            Self::Pms156C => "Pantone 156 C",
            Self::Pms156U => "Pantone 156 U",
            Self::Pms1565C => "Pantone 1565 C",
            Self::Pms1565U => "Pantone 1565 U",
            Self::Pms157C => "Pantone 157 C",
            Self::Pms157U => "Pantone 157 U",
            Self::Pms1575C => "Pantone 1575 C",
            Self::Pms1575U => "Pantone 1575 U",
            Self::Pms158C => "Pantone 158 C",
            Self::Pms158U => "Pantone 158 U",
            Self::Pms1585C => "Pantone 1585 C",
            Self::Pms1585U => "Pantone 1585 U",
            Self::Pms159C => "Pantone 159 C",
            Self::Pms159U => "Pantone 159 U",
            Self::Pms1595C => "Pantone 1595 C",
            Self::Pms1595U => "Pantone 1595 U",
            Self::Pms160C => "Pantone 160 C",
            Self::Pms160U => "Pantone 160 U",
            Self::Pms1605C => "Pantone 1605 C",
            Self::Pms1605U => "Pantone 1605 U",
            Self::Pms161C => "Pantone 161 C",
            Self::Pms161U => "Pantone 161 U",
            Self::Pms1615C => "Pantone 1615 C",
            Self::Pms1615U => "Pantone 1615 U",
            Self::Pms162C => "Pantone 162 C",
            Self::Pms162U => "Pantone 162 U",
            Self::Pms1625C => "Pantone 1625 C",
            Self::Pms1625U => "Pantone 1625 U",
            Self::Pms163C => "Pantone 163 C",
            Self::Pms163U => "Pantone 163 U",
            Self::Pms1635C => "Pantone 1635 C",
            Self::Pms1635U => "Pantone 1635 U",
            Self::Pms164C => "Pantone 164 C",
            Self::Pms164U => "Pantone 164 U",
            Self::Pms1645C => "Pantone 1645 C",
            Self::Pms1645U => "Pantone 1645 U",
            Self::Pms165C => "Pantone 165 C",
            Self::Pms165U => "Pantone 165 U",
            Self::Pms1655C => "Pantone 1655 C",
            Self::Pms1655U => "Pantone 1655 U",
            Self::Pms166C => "Pantone 166 C",
            Self::Pms166U => "Pantone 166 U",
            Self::Pms1665C => "Pantone 1665 C",
            Self::Pms1665U => "Pantone 1665 U",
            Self::Pms167C => "Pantone 167 C",
            Self::Pms167U => "Pantone 167 U",
            Self::Pms1675C => "Pantone 1675 C",
            Self::Pms1675U => "Pantone 1675 U",
            Self::Pms168C => "Pantone 168 C",
            Self::Pms168U => "Pantone 168 U",
            Self::Pms1685C => "Pantone 1685 C",
            Self::Pms1685U => "Pantone 1685 U",
            Self::Pms169C => "Pantone 169 C",
            Self::Pms169U => "Pantone 169 U",
            Self::Pms170C => "Pantone 170 C",
            Self::Pms170U => "Pantone 170 U",
            Self::Pms171C => "Pantone 171 C",
            Self::Pms171U => "Pantone 171 U",
            Self::Pms172C => "Pantone 172 C",
            Self::Pms172U => "Pantone 172 U",
            Self::Pms173C => "Pantone 173 C",
            Self::Pms173U => "Pantone 173 U",
            Self::Pms174C => "Pantone 174 C",
            Self::Pms174U => "Pantone 174 U",
            Self::Pms175C => "Pantone 175 C",
            Self::Pms175U => "Pantone 175 U",
            Self::Pms176C => "Pantone 176 C",
            Self::Pms176U => "Pantone 176 U",
            Self::Pms1765C => "Pantone 1765 C",
            Self::Pms1765U => "Pantone 1765 U",
            Self::Pms1767C => "Pantone 1767 C",
            Self::Pms1767U => "Pantone 1767 U",
            Self::Pms177C => "Pantone 177 C",
            Self::Pms177U => "Pantone 177 U",
            Self::Pms1775C => "Pantone 1775 C",
            Self::Pms1775U => "Pantone 1775 U",
            Self::Pms1777C => "Pantone 1777 C",
            Self::Pms1777U => "Pantone 1777 U",
            Self::Pms178C => "Pantone 178 C",
            Self::Pms178U => "Pantone 178 U",
            Self::Pms1785C => "Pantone 1785 C",
            Self::Pms1785U => "Pantone 1785 U",
            Self::Pms1787C => "Pantone 1787 C",
            Self::Pms1787U => "Pantone 1787 U",
            Self::Pms1788C => "Pantone 1788 C",
            Self::Pms1788U => "Pantone 1788 U",
            Self::Pms179C => "Pantone 179 C",
            Self::Pms179U => "Pantone 179 U",
            Self::Pms1795C => "Pantone 1795 C",
            Self::Pms1795U => "Pantone 1795 U",
            Self::Pms1797C => "Pantone 1797 C",
            Self::Pms1797U => "Pantone 1797 U",
            Self::Pms180C => "Pantone 180 C",
            Self::Pms180U => "Pantone 180 U",
            Self::Pms1805C => "Pantone 1805 C",
            Self::Pms1805U => "Pantone 1805 U",
            Self::Pms1807C => "Pantone 1807 C",
            Self::Pms1807U => "Pantone 1807 U",
            Self::Pms181C => "Pantone 181 C",
            Self::Pms181U => "Pantone 181 U",
            Self::Pms1815C => "Pantone 1815 C",
            Self::Pms1815U => "Pantone 1815 U",
            Self::Pms1817C => "Pantone 1817 C",
            Self::Pms1817U => "Pantone 1817 U",
            Self::Pms182C => "Pantone 182 C",
            Self::Pms182U => "Pantone 182 U",
            Self::Pms183C => "Pantone 183 C",
            Self::Pms183U => "Pantone 183 U",
            Self::Pms184C => "Pantone 184 C",
            Self::Pms184U => "Pantone 184 U",
            Self::Pms185C => "Pantone 185 C",
            Self::Pms185U => "Pantone 185 U",
            Self::Pms186C => "Pantone 186 C",
            Self::Pms186U => "Pantone 186 U",
            Self::Pms187C => "Pantone 187 C",
            Self::Pms187U => "Pantone 187 U",
            Self::Pms188C => "Pantone 188 C",
            Self::Pms188U => "Pantone 188 U",
            Self::Pms189C => "Pantone 189 C",
            Self::Pms189U => "Pantone 189 U",
            Self::Pms1895C => "Pantone 1895 C",
            Self::Pms1895U => "Pantone 1895 U",
            Self::Pms190C => "Pantone 190 C",
            Self::Pms190U => "Pantone 190 U",
            Self::Pms1905C => "Pantone 1905 C",
            Self::Pms1905U => "Pantone 1905 U",
            Self::Pms191C => "Pantone 191 C",
            Self::Pms191U => "Pantone 191 U",
            Self::Pms1915C => "Pantone 1915 C",
            Self::Pms1915U => "Pantone 1915 U",
            Self::Pms192C => "Pantone 192 C",
            Self::Pms192U => "Pantone 192 U",
            Self::Pms1925C => "Pantone 1925 C",
            Self::Pms1925U => "Pantone 1925 U",
            Self::Pms193C => "Pantone 193 C",
            Self::Pms193U => "Pantone 193 U",
            Self::Pms1935C => "Pantone 1935 C",
            Self::Pms1935U => "Pantone 1935 U",
            Self::Pms194C => "Pantone 194 C",
            Self::Pms194U => "Pantone 194 U",
            Self::Pms1945C => "Pantone 1945 C",
            Self::Pms1945U => "Pantone 1945 U",
            Self::Pms195C => "Pantone 195 C",
            Self::Pms195U => "Pantone 195 U",
            Self::Pms1955C => "Pantone 1955 C",
            Self::Pms1955U => "Pantone 1955 U",
            Self::Pms196C => "Pantone 196 C",
            Self::Pms196U => "Pantone 196 U",
            Self::Pms197C => "Pantone 197 C",
            Self::Pms197U => "Pantone 197 U",
            Self::Pms198C => "Pantone 198 C",
            Self::Pms198U => "Pantone 198 U",
            Self::Pms199C => "Pantone 199 C",
            Self::Pms199U => "Pantone 199 U",
            Self::Pms200C => "Pantone 200 C",
            Self::Pms200U => "Pantone 200 U",
            Self::Pms201C => "Pantone 201 C",
            Self::Pms201U => "Pantone 201 U",
            Self::Pms202C => "Pantone 202 C",
            Self::Pms202U => "Pantone 202 U",
            Self::Pms203C => "Pantone 203 C",
            Self::Pms203U => "Pantone 203 U",
            Self::Pms204C => "Pantone 204 C",
            Self::Pms204U => "Pantone 204 U",
            Self::Pms205C => "Pantone 205 C",
            Self::Pms205U => "Pantone 205 U",
            Self::Pms206C => "Pantone 206 C",
            Self::Pms206U => "Pantone 206 U",
            Self::Pms207C => "Pantone 207 C",
            Self::Pms207U => "Pantone 207 U",
            Self::Pms208C => "Pantone 208 C",
            Self::Pms208U => "Pantone 208 U",
            Self::Pms209C => "Pantone 209 C",
            Self::Pms209U => "Pantone 209 U",
            Self::Pms210C => "Pantone 210 C",
            Self::Pms210U => "Pantone 210 U",
            Self::Pms211C => "Pantone 211 C",
            Self::Pms211U => "Pantone 211 U",
            Self::Pms212C => "Pantone 212 C",
            Self::Pms212U => "Pantone 212 U",
            Self::Pms213C => "Pantone 213 C",
            Self::Pms213U => "Pantone 213 U",
            Self::Pms214C => "Pantone 214 C",
            Self::Pms214U => "Pantone 214 U",
            Self::Pms215C => "Pantone 215 C",
            Self::Pms215U => "Pantone 215 U",
            Self::Pms216C => "Pantone 216 C",
            Self::Pms216U => "Pantone 216 U",
            Self::Pms217C => "Pantone 217 C",
            Self::Pms217U => "Pantone 217 U",
            Self::Pms218C => "Pantone 218 C",
            Self::Pms218U => "Pantone 218 U",
            Self::Pms219C => "Pantone 219 C",
            Self::Pms219U => "Pantone 219 U",
            Self::Pms220C => "Pantone 220 C",
            Self::Pms220U => "Pantone 220 U",
            Self::Pms221C => "Pantone 221 C",
            Self::Pms221U => "Pantone 221 U",
            Self::Pms222C => "Pantone 222 C",
            Self::Pms222U => "Pantone 222 U",
            Self::Pms223C => "Pantone 223 C",
            Self::Pms223U => "Pantone 223 U",
            Self::Pms224C => "Pantone 224 C",
            Self::Pms224U => "Pantone 224 U",
            Self::Pms225C => "Pantone 225 C",
            Self::Pms225U => "Pantone 225 U",
            Self::Pms226C => "Pantone 226 C",
            Self::Pms226U => "Pantone 226 U",
            Self::Pms227C => "Pantone 227 C",
            Self::Pms227U => "Pantone 227 U",
            Self::Pms228C => "Pantone 228 C",
            Self::Pms228U => "Pantone 228 U",
            Self::Pms229C => "Pantone 229 C",
            Self::Pms229U => "Pantone 229 U",
            Self::Pms230C => "Pantone 230 C",
            Self::Pms230U => "Pantone 230 U",
            Self::Pms231C => "Pantone 231 C",
            Self::Pms231U => "Pantone 231 U",
            Self::Pms232C => "Pantone 232 C",
            Self::Pms232U => "Pantone 232 U",
            Self::Pms233C => "Pantone 233 C",
            Self::Pms233U => "Pantone 233 U",
            Self::Pms234C => "Pantone 234 C",
            Self::Pms234U => "Pantone 234 U",
            Self::Pms235C => "Pantone 235 C",
            Self::Pms235U => "Pantone 235 U",
            Self::Pms236C => "Pantone 236 C",
            Self::Pms236U => "Pantone 236 U",
            Self::Pms2365C => "Pantone 2365 C",
            Self::Pms2365U => "Pantone 2365 U",
            Self::Pms237C => "Pantone 237 C",
            Self::Pms237U => "Pantone 237 U",
            Self::Pms2375C => "Pantone 2375 C",
            Self::Pms2375U => "Pantone 2375 U",
            Self::Pms238C => "Pantone 238 C",
            Self::Pms238U => "Pantone 238 U",
            Self::Pms2385C => "Pantone 2385 C",
            Self::Pms2385U => "Pantone 2385 U",
            Self::Pms239C => "Pantone 239 C",
            Self::Pms239U => "Pantone 239 U",
            Self::Pms2395C => "Pantone 2395 C",
            Self::Pms2395U => "Pantone 2395 U",
            Self::Pms240C => "Pantone 240 C",
            Self::Pms240U => "Pantone 240 U",
            Self::Pms2405C => "Pantone 2405 C",
            Self::Pms2405U => "Pantone 2405 U",
            Self::Pms241C => "Pantone 241 C",
            Self::Pms241U => "Pantone 241 U",
            Self::Pms2415C => "Pantone 2415 C",
            Self::Pms2415U => "Pantone 2415 U",
            Self::Pms242C => "Pantone 242 C",
            Self::Pms242U => "Pantone 242 U",
            Self::Pms2425C => "Pantone 2425 C",
            Self::Pms2425U => "Pantone 2425 U",
            Self::Pms243C => "Pantone 243 C",
            Self::Pms243U => "Pantone 243 U",
            Self::Pms244C => "Pantone 244 C",
            Self::Pms244U => "Pantone 244 U",
            Self::Pms245C => "Pantone 245 C",
            Self::Pms245U => "Pantone 245 U",
            Self::Pms246C => "Pantone 246 C",
            Self::Pms246U => "Pantone 246 U",
            Self::Pms247C => "Pantone 247 C",
            Self::Pms247U => "Pantone 247 U",
            Self::Pms248C => "Pantone 248 C",
            Self::Pms248U => "Pantone 248 U",
            Self::Pms249C => "Pantone 249 C",
            Self::Pms249U => "Pantone 249 U",
            Self::Pms250C => "Pantone 250 C",
            Self::Pms250U => "Pantone 250 U",
            Self::Pms251C => "Pantone 251 C",
            Self::Pms251U => "Pantone 251 U",
            Self::Pms252C => "Pantone 252 C",
            Self::Pms252U => "Pantone 252 U",
            Self::Pms253C => "Pantone 253 C",
            Self::Pms253U => "Pantone 253 U",
            Self::Pms254C => "Pantone 254 C",
            Self::Pms254U => "Pantone 254 U",
            Self::Pms255C => "Pantone 255 C",
            Self::Pms255U => "Pantone 255 U",
            Self::Pms256C => "Pantone 256 C",
            Self::Pms256U => "Pantone 256 U",
            Self::Pms2562C => "Pantone 2562 C",
            Self::Pms2562U => "Pantone 2562 U",
            Self::Pms2563C => "Pantone 2563 C",
            Self::Pms2563U => "Pantone 2563 U",
            Self::Pms2567C => "Pantone 2567 C",
            Self::Pms2567U => "Pantone 2567 U",
            Self::Pms257C => "Pantone 257 C",
            Self::Pms257U => "Pantone 257 U",
            Self::Pms2572C => "Pantone 2572 C",
            Self::Pms2572U => "Pantone 2572 U",
            Self::Pms2573C => "Pantone 2573 C",
            Self::Pms2573U => "Pantone 2573 U",
            Self::Pms2577C => "Pantone 2577 C",
            Self::Pms2577U => "Pantone 2577 U",
            Self::Pms258C => "Pantone 258 C",
            Self::Pms258U => "Pantone 258 U",
            Self::Pms2582C => "Pantone 2582 C",
            Self::Pms2582U => "Pantone 2582 U",
            Self::Pms2583C => "Pantone 2583 C",
            Self::Pms2583U => "Pantone 2583 U",
            Self::Pms2587C => "Pantone 2587 C",
            Self::Pms2587U => "Pantone 2587 U",
            Self::Pms259C => "Pantone 259 C",
            Self::Pms259U => "Pantone 259 U",
            Self::Pms2592C => "Pantone 2592 C",
            Self::Pms2592U => "Pantone 2592 U",
            Self::Pms2593C => "Pantone 2593 C",
            Self::Pms2593U => "Pantone 2593 U",
            Self::Pms2597C => "Pantone 2597 C",
            Self::Pms2597U => "Pantone 2597 U",
            Self::Pms260C => "Pantone 260 C",
            Self::Pms260U => "Pantone 260 U",
            Self::Pms2602C => "Pantone 2602 C",
            Self::Pms2602U => "Pantone 2602 U",
            Self::Pms2603C => "Pantone 2603 C",
            Self::Pms2603U => "Pantone 2603 U",
            Self::Pms2607C => "Pantone 2607 C",
            Self::Pms2607U => "Pantone 2607 U",
            Self::Pms261C => "Pantone 261 C",
            Self::Pms261U => "Pantone 261 U",
            Self::Pms2612C => "Pantone 2612 C",
            Self::Pms2612U => "Pantone 2612 U",
            Self::Pms2613C => "Pantone 2613 C",
            Self::Pms2613U => "Pantone 2613 U",
            Self::Pms2617C => "Pantone 2617 C",
            Self::Pms2617U => "Pantone 2617 U",
            Self::Pms262C => "Pantone 262 C",
            Self::Pms262U => "Pantone 262 U",
            Self::Pms2622C => "Pantone 2622 C",
            Self::Pms2622U => "Pantone 2622 U",
            Self::Pms2623C => "Pantone 2623 C",
            Self::Pms2623U => "Pantone 2623 U",
            Self::Pms2627C => "Pantone 2627 C",
            Self::Pms2627U => "Pantone 2627 U",
            Self::Pms263C => "Pantone 263 C",
            Self::Pms263U => "Pantone 263 U",
            Self::Pms2635C => "Pantone 2635 C",
            Self::Pms2635U => "Pantone 2635 U",
            Self::Pms264C => "Pantone 264 C",
            Self::Pms264U => "Pantone 264 U",
            Self::Pms2645C => "Pantone 2645 C",
            Self::Pms2645U => "Pantone 2645 U",
            Self::Pms265C => "Pantone 265 C",
            Self::Pms265U => "Pantone 265 U",
            Self::Pms2655C => "Pantone 2655 C",
            Self::Pms2655U => "Pantone 2655 U",
            Self::Pms266C => "Pantone 266 C",
            Self::Pms266U => "Pantone 266 U",
            Self::Pms2665C => "Pantone 2665 C",
            Self::Pms2665U => "Pantone 2665 U",
            Self::Pms267C => "Pantone 267 C",
            Self::Pms267U => "Pantone 267 U",
            Self::Pms268C => "Pantone 268 C",
            Self::Pms268U => "Pantone 268 U",
            Self::Pms2685C => "Pantone 2685 C",
            Self::Pms2685U => "Pantone 2685 U",
            Self::Pms269C => "Pantone 269 C",
            Self::Pms269U => "Pantone 269 U",
            Self::Pms2695C => "Pantone 2695 C",
            Self::Pms2695U => "Pantone 2695 U",
            Self::Pms270C => "Pantone 270 C",
            Self::Pms270U => "Pantone 270 U",
            Self::Pms2705C => "Pantone 2705 C",
            Self::Pms2705U => "Pantone 2705 U",
            Self::Pms2706C => "Pantone 2706 C",
            Self::Pms2706U => "Pantone 2706 U",
            Self::Pms2707C => "Pantone 2707 C",
            Self::Pms2707U => "Pantone 2707 U",
            Self::Pms2708C => "Pantone 2708 C",
            Self::Pms2708U => "Pantone 2708 U",
            Self::Pms271C => "Pantone 271 C",
            Self::Pms271U => "Pantone 271 U",
            Self::Pms2715C => "Pantone 2715 C",
            Self::Pms2715U => "Pantone 2715 U",
            Self::Pms2716C => "Pantone 2716 C",
            Self::Pms2716U => "Pantone 2716 U",
            Self::Pms2717C => "Pantone 2717 C",
            Self::Pms2717U => "Pantone 2717 U",
            Self::Pms2718C => "Pantone 2718 C",
            Self::Pms2718U => "Pantone 2718 U",
            Self::Pms272C => "Pantone 272 C",
            Self::Pms272U => "Pantone 272 U",
            Self::Pms2725C => "Pantone 2725 C",
            Self::Pms2725U => "Pantone 2725 U",
            Self::Pms2726C => "Pantone 2726 C",
            Self::Pms2726U => "Pantone 2726 U",
            Self::Pms2727C => "Pantone 2727 C",
            Self::Pms2727U => "Pantone 2727 U",
            Self::Pms2728C => "Pantone 2728 C",
            Self::Pms2728U => "Pantone 2728 U",
            Self::Pms273C => "Pantone 273 C",
            Self::Pms273U => "Pantone 273 U",
            Self::Pms2735C => "Pantone 2735 C",
            Self::Pms2735U => "Pantone 2735 U",
            Self::Pms2736C => "Pantone 2736 C",
            Self::Pms2736U => "Pantone 2736 U",
            Self::Pms2738C => "Pantone 2738 C",
            Self::Pms2738U => "Pantone 2738 U",
            Self::Pms274C => "Pantone 274 C",
            Self::Pms274U => "Pantone 274 U",
            Self::Pms2745C => "Pantone 2745 C",
            Self::Pms2745U => "Pantone 2745 U",
            Self::Pms2746C => "Pantone 2746 C",
            Self::Pms2746U => "Pantone 2746 U",
            Self::Pms2747C => "Pantone 2747 C",
            Self::Pms2747U => "Pantone 2747 U",
            Self::Pms2748C => "Pantone 2748 C",
            Self::Pms2748U => "Pantone 2748 U",
            Self::Pms275C => "Pantone 275 C",
            Self::Pms275U => "Pantone 275 U",
            Self::Pms2755C => "Pantone 2755 C",
            Self::Pms2755U => "Pantone 2755 U",
            Self::Pms2756C => "Pantone 2756 C",
            Self::Pms2756U => "Pantone 2756 U",
            Self::Pms2757C => "Pantone 2757 C",
            Self::Pms2757U => "Pantone 2757 U",
            Self::Pms2758C => "Pantone 2758 C",
            Self::Pms2758U => "Pantone 2758 U",
            Self::Pms276C => "Pantone 276 C",
            Self::Pms276U => "Pantone 276 U",
            Self::Pms2765C => "Pantone 2765 C",
            Self::Pms2765U => "Pantone 2765 U",
            Self::Pms2766C => "Pantone 2766 C",
            Self::Pms2766U => "Pantone 2766 U",
            Self::Pms2767C => "Pantone 2767 C",
            Self::Pms2767U => "Pantone 2767 U",
            Self::Pms2768C => "Pantone 2768 C",
            Self::Pms2768U => "Pantone 2768 U",
            Self::Pms277C => "Pantone 277 C",
            Self::Pms277U => "Pantone 277 U",
            Self::Pms278C => "Pantone 278 C",
            Self::Pms278U => "Pantone 278 U",
            Self::Pms279C => "Pantone 279 C",
            Self::Pms279U => "Pantone 279 U",
            Self::Pms280C => "Pantone 280 C",
            Self::Pms280U => "Pantone 280 U",
            Self::Pms281C => "Pantone 281 C",
            Self::Pms281U => "Pantone 281 U",
            Self::Pms282C => "Pantone 282 C",
            Self::Pms282U => "Pantone 282 U",
            Self::Pms283C => "Pantone 283 C",
            Self::Pms283U => "Pantone 283 U",
            Self::Pms284C => "Pantone 284 C",
            Self::Pms284U => "Pantone 284 U",
            Self::Pms285C => "Pantone 285 C",
            Self::Pms285U => "Pantone 285 U",
            Self::Pms286C => "Pantone 286 C",
            Self::Pms286U => "Pantone 286 U",
            Self::Pms287C => "Pantone 287 C",
            Self::Pms287U => "Pantone 287 U",
            Self::Pms288C => "Pantone 288 C",
            Self::Pms288U => "Pantone 288 U",
            Self::Pms289C => "Pantone 289 C",
            Self::Pms289U => "Pantone 289 U",
            Self::Pms290C => "Pantone 290 C",
            Self::Pms290U => "Pantone 290 U",
            Self::Pms2905C => "Pantone 2905 C",
            Self::Pms2905U => "Pantone 2905 U",
            Self::Pms291C => "Pantone 291 C",
            Self::Pms291U => "Pantone 291 U",
            Self::Pms2915C => "Pantone 2915 C",
            Self::Pms2915U => "Pantone 2915 U",
            Self::Pms292C => "Pantone 292 C",
            Self::Pms292U => "Pantone 292 U",
            Self::Pms2925C => "Pantone 2925 C",
            Self::Pms2925U => "Pantone 2925 U",
            Self::Pms293C => "Pantone 293 C",
            Self::Pms293U => "Pantone 293 U",
            Self::Pms2935C => "Pantone 2935 C",
            Self::Pms2935U => "Pantone 2935 U",
            Self::Pms294C => "Pantone 294 C",
            Self::Pms294U => "Pantone 294 U",
            Self::Pms2945C => "Pantone 2945 C",
            Self::Pms2945U => "Pantone 2945 U",
            Self::Pms295C => "Pantone 295 C",
            Self::Pms295U => "Pantone 295 U",
            Self::Pms2955C => "Pantone 2955 C",
            Self::Pms2955U => "Pantone 2955 U",
            Self::Pms296C => "Pantone 296 C",
            Self::Pms296U => "Pantone 296 U",
            Self::Pms2965C => "Pantone 2965 C",
            Self::Pms2965U => "Pantone 2965 U",
            Self::Pms297C => "Pantone 297 C",
            Self::Pms297U => "Pantone 297 U",
            Self::Pms2975C => "Pantone 2975 C",
            Self::Pms2975U => "Pantone 2975 U",
            Self::Pms298C => "Pantone 298 C",
            Self::Pms298U => "Pantone 298 U",
            Self::Pms2985C => "Pantone 2985 C",
            Self::Pms2985U => "Pantone 2985 U",
            Self::Pms299C => "Pantone 299 C",
            Self::Pms299U => "Pantone 299 U",
            Self::Pms2995C => "Pantone 2995 C",
            Self::Pms2995U => "Pantone 2995 U",
            Self::Pms300C => "Pantone 300 C",
            Self::Pms300U => "Pantone 300 U",
            Self::Pms3005C => "Pantone 3005 C",
            Self::Pms3005U => "Pantone 3005 U",
            Self::Pms301C => "Pantone 301 C",
            Self::Pms301U => "Pantone 301 U",
            Self::Pms3015C => "Pantone 3015 C",
            Self::Pms3015U => "Pantone 3015 U",
            Self::Pms302C => "Pantone 302 C",
            Self::Pms302U => "Pantone 302 U",
            Self::Pms3025C => "Pantone 3025 C",
            Self::Pms3025U => "Pantone 3025 U",
            Self::Pms303C => "Pantone 303 C",
            Self::Pms303U => "Pantone 303 U",
            Self::Pms3035C => "Pantone 3035 C",
            Self::Pms3035U => "Pantone 3035 U",
            Self::Pms304C => "Pantone 304 C",
            Self::Pms304U => "Pantone 304 U",
            Self::Pms305C => "Pantone 305 C",
            Self::Pms305U => "Pantone 305 U",
            Self::Pms306C => "Pantone 306 C",
            Self::Pms306U => "Pantone 306 U",
            Self::Pms307C => "Pantone 307 C",
            Self::Pms307U => "Pantone 307 U",
            Self::Pms308C => "Pantone 308 C",
            Self::Pms308U => "Pantone 308 U",
            Self::Pms309C => "Pantone 309 C",
            Self::Pms309U => "Pantone 309 U",
            Self::Pms310C => "Pantone 310 C",
            Self::Pms310U => "Pantone 310 U",
            Self::Pms3105C => "Pantone 3105 C",
            Self::Pms3105U => "Pantone 3105 U",
            Self::Pms311C => "Pantone 311 C",
            Self::Pms311U => "Pantone 311 U",
            Self::Pms3115C => "Pantone 3115 C",
            Self::Pms3115U => "Pantone 3115 U",
            Self::Pms312C => "Pantone 312 C",
            Self::Pms312U => "Pantone 312 U",
            Self::Pms3125C => "Pantone 3125 C",
            Self::Pms3125U => "Pantone 3125 U",
            Self::Pms313C => "Pantone 313 C",
            Self::Pms313U => "Pantone 313 U",
            Self::Pms3135C => "Pantone 3135 C",
            Self::Pms3135U => "Pantone 3135 U",
            Self::Pms314C => "Pantone 314 C",
            Self::Pms314U => "Pantone 314 U",
            Self::Pms3145C => "Pantone 3145 C",
            Self::Pms3145U => "Pantone 3145 U",
            Self::Pms315C => "Pantone 315 C",
            Self::Pms315U => "Pantone 315 U",
            Self::Pms3155C => "Pantone 3155 C",
            Self::Pms3155U => "Pantone 3155 U",
            Self::Pms316C => "Pantone 316 C",
            Self::Pms316U => "Pantone 316 U",
            Self::Pms3165C => "Pantone 3165 C",
            Self::Pms3165U => "Pantone 3165 U",
            Self::Pms317C => "Pantone 317 C",
            Self::Pms317U => "Pantone 317 U",
            Self::Pms318C => "Pantone 318 C",
            Self::Pms318U => "Pantone 318 U",
            Self::Pms319C => "Pantone 319 C",
            Self::Pms319U => "Pantone 319 U",
            Self::Pms320C => "Pantone 320 C",
            Self::Pms320U => "Pantone 320 U",
            Self::Pms321C => "Pantone 321 C",
            Self::Pms321U => "Pantone 321 U",
            Self::Pms322C => "Pantone 322 C",
            Self::Pms322U => "Pantone 322 U",
            Self::Pms323C => "Pantone 323 C",
            Self::Pms323U => "Pantone 323 U",
            Self::Pms324C => "Pantone 324 C",
            Self::Pms324U => "Pantone 324 U",
            Self::Pms3242C => "Pantone 3242 C",
            Self::Pms3242U => "Pantone 3242 U",
            Self::Pms3245C => "Pantone 3245 C",
            Self::Pms3245U => "Pantone 3245 U",
            Self::Pms3248C => "Pantone 3248 C",
            Self::Pms3248U => "Pantone 3248 U",
            Self::Pms325C => "Pantone 325 C",
            Self::Pms325U => "Pantone 325 U",
            Self::Pms3252C => "Pantone 3252 C",
            Self::Pms3252U => "Pantone 3252 U",
            Self::Pms3255C => "Pantone 3255 C",
            Self::Pms3255U => "Pantone 3255 U",
            Self::Pms3258C => "Pantone 3258 C",
            Self::Pms3258U => "Pantone 3258 U",
            Self::Pms326C => "Pantone 326 C",
            Self::Pms326U => "Pantone 326 U",
            Self::Pms3262C => "Pantone 3262 C",
            Self::Pms3262U => "Pantone 3262 U",
            Self::Pms3265C => "Pantone 3265 C",
            Self::Pms3265U => "Pantone 3265 U",
            Self::Pms3268C => "Pantone 3268 C",
            Self::Pms3268U => "Pantone 3268 U",
            Self::Pms327C => "Pantone 327 C",
            Self::Pms327U => "Pantone 327 U",
            Self::Pms3272C => "Pantone 3272 C",
            Self::Pms3272U => "Pantone 3272 U",
            Self::Pms3275C => "Pantone 3275 C",
            Self::Pms3275U => "Pantone 3275 U",
            Self::Pms3278C => "Pantone 3278 C",
            Self::Pms3278U => "Pantone 3278 U",
            Self::Pms328C => "Pantone 328 C",
            Self::Pms328U => "Pantone 328 U",
            Self::Pms3282C => "Pantone 3282 C",
            Self::Pms3282U => "Pantone 3282 U",
            Self::Pms3285C => "Pantone 3285 C",
            Self::Pms3285U => "Pantone 3285 U",
            Self::Pms3288C => "Pantone 3288 C",
            Self::Pms3288U => "Pantone 3288 U",
            Self::Pms329C => "Pantone 329 C",
            Self::Pms329U => "Pantone 329 U",
            Self::Pms3292C => "Pantone 3292 C",
            Self::Pms3292U => "Pantone 3292 U",
            Self::Pms3295C => "Pantone 3295 C",
            Self::Pms3295U => "Pantone 3295 U",
            Self::Pms3298C => "Pantone 3298 C",
            Self::Pms3298U => "Pantone 3298 U",
            Self::Pms330C => "Pantone 330 C",
            Self::Pms330U => "Pantone 330 U",
            Self::Pms3302C => "Pantone 3302 C",
            Self::Pms3302U => "Pantone 3302 U",
            Self::Pms3305C => "Pantone 3305 C",
            Self::Pms3305U => "Pantone 3305 U",
            Self::Pms3308C => "Pantone 3308 C",
            Self::Pms3308U => "Pantone 3308 U",
            Self::Pms331C => "Pantone 331 C",
            Self::Pms331U => "Pantone 331 U",
            Self::Pms332C => "Pantone 332 C",
            Self::Pms332U => "Pantone 332 U",
            Self::Pms333C => "Pantone 333 C",
            Self::Pms333U => "Pantone 333 U",
            Self::Pms334C => "Pantone 334 C",
            Self::Pms334U => "Pantone 334 U",
            Self::Pms335C => "Pantone 335 C",
            Self::Pms335U => "Pantone 335 U",
            Self::Pms336C => "Pantone 336 C",
            Self::Pms336U => "Pantone 336 U",
            Self::Pms337C => "Pantone 337 C",
            Self::Pms337U => "Pantone 337 U",
            Self::Pms3375C => "Pantone 3375 C",
            Self::Pms3375U => "Pantone 3375 U",
            Self::Pms338C => "Pantone 338 C",
            Self::Pms338U => "Pantone 338 U",
            Self::Pms3385C => "Pantone 3385 C",
            Self::Pms3385U => "Pantone 3385 U",
            Self::Pms339C => "Pantone 339 C",
            Self::Pms339U => "Pantone 339 U",
            Self::Pms3395C => "Pantone 3395 C",
            Self::Pms3395U => "Pantone 3395 U",
            Self::Pms340C => "Pantone 340 C",
            Self::Pms340U => "Pantone 340 U",
            Self::Pms3405C => "Pantone 3405 C",
            Self::Pms3405U => "Pantone 3405 U",
            Self::Pms341C => "Pantone 341 C",
            Self::Pms341U => "Pantone 341 U",
            Self::Pms3415C => "Pantone 3415 C",
            Self::Pms3415U => "Pantone 3415 U",
            Self::Pms342C => "Pantone 342 C",
            Self::Pms342U => "Pantone 342 U",
            Self::Pms3425C => "Pantone 3425 C",
            Self::Pms3425U => "Pantone 3425 U",
            Self::Pms343C => "Pantone 343 C",
            Self::Pms343U => "Pantone 343 U",
            Self::Pms3435C => "Pantone 3435 C",
            Self::Pms3435U => "Pantone 3435 U",
            Self::Pms344C => "Pantone 344 C",
            Self::Pms344U => "Pantone 344 U",
            Self::Pms345C => "Pantone 345 C",
            Self::Pms345U => "Pantone 345 U",
            Self::Pms346C => "Pantone 346 C",
            Self::Pms346U => "Pantone 346 U",
            Self::Pms347C => "Pantone 347 C",
            Self::Pms347U => "Pantone 347 U",
            Self::Pms348C => "Pantone 348 C",
            Self::Pms348U => "Pantone 348 U",
            Self::Pms349C => "Pantone 349 C",
            Self::Pms349U => "Pantone 349 U",
            Self::Pms350C => "Pantone 350 C",
            Self::Pms350U => "Pantone 350 U",
            Self::Pms351C => "Pantone 351 C",
            Self::Pms351U => "Pantone 351 U",
            Self::Pms352C => "Pantone 352 C",
            Self::Pms352U => "Pantone 352 U",
            Self::Pms353C => "Pantone 353 C",
            Self::Pms353U => "Pantone 353 U",
            Self::Pms354C => "Pantone 354 C",
            Self::Pms354U => "Pantone 354 U",
            Self::Pms355C => "Pantone 355 C",
            Self::Pms355U => "Pantone 355 U",
            Self::Pms356C => "Pantone 356 C",
            Self::Pms356U => "Pantone 356 U",
            Self::Pms357C => "Pantone 357 C",
            Self::Pms357U => "Pantone 357 U",
            Self::Pms358C => "Pantone 358 C",
            Self::Pms358U => "Pantone 358 U",
            Self::Pms359C => "Pantone 359 C",
            Self::Pms359U => "Pantone 359 U",
            Self::Pms360C => "Pantone 360 C",
            Self::Pms360U => "Pantone 360 U",
            Self::Pms361C => "Pantone 361 C",
            Self::Pms361U => "Pantone 361 U",
            Self::Pms362C => "Pantone 362 C",
            Self::Pms362U => "Pantone 362 U",
            Self::Pms363C => "Pantone 363 C",
            Self::Pms363U => "Pantone 363 U",
            Self::Pms364C => "Pantone 364 C",
            Self::Pms364U => "Pantone 364 U",
            Self::Pms365C => "Pantone 365 C",
            Self::Pms365U => "Pantone 365 U",
            Self::Pms366C => "Pantone 366 C",
            Self::Pms366U => "Pantone 366 U",
            Self::Pms367C => "Pantone 367 C",
            Self::Pms367U => "Pantone 367 U",
            Self::Pms368C => "Pantone 368 C",
            Self::Pms368U => "Pantone 368 U",
            Self::Pms369C => "Pantone 369 C",
            Self::Pms369U => "Pantone 369 U",
            Self::Pms370C => "Pantone 370 C",
            Self::Pms370U => "Pantone 370 U",
            Self::Pms371C => "Pantone 371 C",
            Self::Pms371U => "Pantone 371 U",
            Self::Pms372C => "Pantone 372 C",
            Self::Pms372U => "Pantone 372 U",
            Self::Pms373C => "Pantone 373 C",
            Self::Pms373U => "Pantone 373 U",
            Self::Pms374C => "Pantone 374 C",
            Self::Pms374U => "Pantone 374 U",
            Self::Pms375C => "Pantone 375 C",
            Self::Pms375U => "Pantone 375 U",
            Self::Pms376C => "Pantone 376 C",
            Self::Pms376U => "Pantone 376 U",
            Self::Pms377C => "Pantone 377 C",
            Self::Pms377U => "Pantone 377 U",
            Self::Pms378C => "Pantone 378 C",
            Self::Pms378U => "Pantone 378 U",
            Self::Pms379C => "Pantone 379 C",
            Self::Pms379U => "Pantone 379 U",
            Self::Pms380C => "Pantone 380 C",
            Self::Pms380U => "Pantone 380 U",
            Self::Pms381C => "Pantone 381 C",
            Self::Pms381U => "Pantone 381 U",
            Self::Pms382C => "Pantone 382 C",
            Self::Pms382U => "Pantone 382 U",
            Self::Pms383C => "Pantone 383 C",
            Self::Pms383U => "Pantone 383 U",
            Self::Pms384C => "Pantone 384 C",
            Self::Pms384U => "Pantone 384 U",
            Self::Pms385C => "Pantone 385 C",
            Self::Pms385U => "Pantone 385 U",
            Self::Pms386C => "Pantone 386 C",
            Self::Pms386U => "Pantone 386 U",
            Self::Pms387C => "Pantone 387 C",
            Self::Pms387U => "Pantone 387 U",
            Self::Pms388C => "Pantone 388 C",
            Self::Pms388U => "Pantone 388 U",
            Self::Pms389C => "Pantone 389 C",
            Self::Pms389U => "Pantone 389 U",
            Self::Pms390C => "Pantone 390 C",
            Self::Pms390U => "Pantone 390 U",
            Self::Pms391C => "Pantone 391 C",
            Self::Pms391U => "Pantone 391 U",
            Self::Pms392C => "Pantone 392 C",
            Self::Pms392U => "Pantone 392 U",
            Self::Pms393C => "Pantone 393 C",
            Self::Pms393U => "Pantone 393 U",
            Self::Pms3935C => "Pantone 3935 C",
            Self::Pms3935U => "Pantone 3935 U",
            Self::Pms394C => "Pantone 394 C",
            Self::Pms394U => "Pantone 394 U",
            Self::Pms3945C => "Pantone 3945 C",
            Self::Pms3945U => "Pantone 3945 U",
            Self::Pms395C => "Pantone 395 C",
            Self::Pms395U => "Pantone 395 U",
            Self::Pms3955C => "Pantone 3955 C",
            Self::Pms3955U => "Pantone 3955 U",
            Self::Pms396C => "Pantone 396 C",
            Self::Pms396U => "Pantone 396 U",
            Self::Pms3965C => "Pantone 3965 C",
            Self::Pms3965U => "Pantone 3965 U",
            Self::Pms397C => "Pantone 397 C",
            Self::Pms397U => "Pantone 397 U",
            Self::Pms3975C => "Pantone 3975 C",
            Self::Pms3975U => "Pantone 3975 U",
            Self::Pms398C => "Pantone 398 C",
            Self::Pms398U => "Pantone 398 U",
            Self::Pms3985C => "Pantone 3985 C",
            Self::Pms3985U => "Pantone 3985 U",
            Self::Pms399C => "Pantone 399 C",
            Self::Pms399U => "Pantone 399 U",
            Self::Pms3995C => "Pantone 3995 C",
            Self::Pms3995U => "Pantone 3995 U",
            Self::Pms400C => "Pantone 400 C",
            Self::Pms400U => "Pantone 400 U",
            Self::Pms401C => "Pantone 401 C",
            Self::Pms401U => "Pantone 401 U",
            Self::Pms402C => "Pantone 402 C",
            Self::Pms402U => "Pantone 402 U",
            Self::Pms403C => "Pantone 403 C",
            Self::Pms403U => "Pantone 403 U",
            Self::Pms404C => "Pantone 404 C",
            Self::Pms404U => "Pantone 404 U",
            Self::Pms405C => "Pantone 405 C",
            Self::Pms405U => "Pantone 405 U",
            Self::Pms406C => "Pantone 406 C",
            Self::Pms406U => "Pantone 406 U",
            Self::Pms407C => "Pantone 407 C",
            Self::Pms407U => "Pantone 407 U",
            Self::Pms408C => "Pantone 408 C",
            Self::Pms408U => "Pantone 408 U",
            Self::Pms409C => "Pantone 409 C",
            Self::Pms409U => "Pantone 409 U",
            Self::Pms410C => "Pantone 410 C",
            Self::Pms410U => "Pantone 410 U",
            Self::Pms411C => "Pantone 411 C",
            Self::Pms411U => "Pantone 411 U",
            Self::Pms412C => "Pantone 412 C",
            Self::Pms412U => "Pantone 412 U",
            Self::Pms413C => "Pantone 413 C",
            Self::Pms413U => "Pantone 413 U",
            Self::Pms414C => "Pantone 414 C",
            Self::Pms414U => "Pantone 414 U",
            Self::Pms415C => "Pantone 415 C",
            Self::Pms415U => "Pantone 415 U",
            Self::Pms416C => "Pantone 416 C",
            Self::Pms416U => "Pantone 416 U",
            Self::Pms417C => "Pantone 417 C",
            Self::Pms417U => "Pantone 417 U",
            Self::Pms418C => "Pantone 418 C",
            Self::Pms418U => "Pantone 418 U",
            Self::Pms419C => "Pantone 419 C",
            Self::Pms419U => "Pantone 419 U",
            Self::Pms420C => "Pantone 420 C",
            Self::Pms420U => "Pantone 420 U",
            Self::Pms421C => "Pantone 421 C",
            Self::Pms421U => "Pantone 421 U",
            Self::Pms422C => "Pantone 422 C",
            Self::Pms422U => "Pantone 422 U",
            Self::Pms423C => "Pantone 423 C",
            Self::Pms423U => "Pantone 423 U",
            Self::Pms424C => "Pantone 424 C",
            Self::Pms424U => "Pantone 424 U",
            Self::Pms425C => "Pantone 425 C",
            Self::Pms425U => "Pantone 425 U",
            Self::Pms426C => "Pantone 426 C",
            Self::Pms426U => "Pantone 426 U",
            Self::Pms427C => "Pantone 427 C",
            Self::Pms427U => "Pantone 427 U",
            Self::Pms428C => "Pantone 428 C",
            Self::Pms428U => "Pantone 428 U",
            Self::Pms429C => "Pantone 429 C",
            Self::Pms429U => "Pantone 429 U",
            Self::Pms430C => "Pantone 430 C",
            Self::Pms430U => "Pantone 430 U",
            Self::Pms431C => "Pantone 431 C",
            Self::Pms431U => "Pantone 431 U",
            Self::Pms432C => "Pantone 432 C",
            Self::Pms432U => "Pantone 432 U",
            Self::Pms433C => "Pantone 433 C",
            Self::Pms433U => "Pantone 433 U",
            Self::Pms434C => "Pantone 434 C",
            Self::Pms434U => "Pantone 434 U",
            Self::Pms435C => "Pantone 435 C",
            Self::Pms435U => "Pantone 435 U",
            Self::Pms436C => "Pantone 436 C",
            Self::Pms436U => "Pantone 436 U",
            Self::Pms437C => "Pantone 437 C",
            Self::Pms437U => "Pantone 437 U",
            Self::Pms438C => "Pantone 438 C",
            Self::Pms438U => "Pantone 438 U",
            Self::Pms439C => "Pantone 439 C",
            Self::Pms439U => "Pantone 439 U",
            Self::Pms440C => "Pantone 440 C",
            Self::Pms440U => "Pantone 440 U",
            Self::Pms441C => "Pantone 441 C",
            Self::Pms441U => "Pantone 441 U",
            Self::Pms442C => "Pantone 442 C",
            Self::Pms442U => "Pantone 442 U",
            Self::Pms443C => "Pantone 443 C",
            Self::Pms443U => "Pantone 443 U",
            Self::Pms444C => "Pantone 444 C",
            Self::Pms444U => "Pantone 444 U",
            Self::Pms445C => "Pantone 445 C",
            Self::Pms445U => "Pantone 445 U",
            Self::Pms446C => "Pantone 446 C",
            Self::Pms446U => "Pantone 446 U",
            Self::Pms447C => "Pantone 447 C",
            Self::Pms447U => "Pantone 447 U",
            Self::Pms448C => "Pantone 448 C",
            Self::Pms448U => "Pantone 448 U",
            Self::Pms4485C => "Pantone 4485 C",
            Self::Pms4485U => "Pantone 4485 U",
            Self::Pms449C => "Pantone 449 C",
            Self::Pms449U => "Pantone 449 U",
            Self::Pms4495C => "Pantone 4495 C",
            Self::Pms4495U => "Pantone 4495 U",
            Self::Pms450C => "Pantone 450 C",
            Self::Pms450U => "Pantone 450 U",
            Self::Pms4505C => "Pantone 4505 C",
            Self::Pms4505U => "Pantone 4505 U",
            Self::Pms451C => "Pantone 451 C",
            Self::Pms451U => "Pantone 451 U",
            Self::Pms4515C => "Pantone 4515 C",
            Self::Pms4515U => "Pantone 4515 U",
            Self::Pms452C => "Pantone 452 C",
            Self::Pms452U => "Pantone 452 U",
            Self::Pms4525C => "Pantone 4525 C",
            Self::Pms4525U => "Pantone 4525 U",
            Self::Pms453C => "Pantone 453 C",
            Self::Pms453U => "Pantone 453 U",
            Self::Pms4535C => "Pantone 4535 C",
            Self::Pms4535U => "Pantone 4535 U",
            Self::Pms454C => "Pantone 454 C",
            Self::Pms454U => "Pantone 454 U",
            Self::Pms4545C => "Pantone 4545 C",
            Self::Pms4545U => "Pantone 4545 U",
            Self::Pms455C => "Pantone 455 C",
            Self::Pms455U => "Pantone 455 U",
            Self::Pms456C => "Pantone 456 C",
            Self::Pms456U => "Pantone 456 U",
            Self::Pms457C => "Pantone 457 C",
            Self::Pms457U => "Pantone 457 U",
            Self::Pms458C => "Pantone 458 C",
            Self::Pms458U => "Pantone 458 U",
            Self::Pms459C => "Pantone 459 C",
            Self::Pms459U => "Pantone 459 U",
            Self::Pms460C => "Pantone 460 C",
            Self::Pms460U => "Pantone 460 U",
            Self::Pms461C => "Pantone 461 C",
            Self::Pms461U => "Pantone 461 U",
            Self::Pms462C => "Pantone 462 C",
            Self::Pms462U => "Pantone 462 U",
            Self::Pms4625C => "Pantone 4625 C",
            Self::Pms4625U => "Pantone 4625 U",
            Self::Pms463C => "Pantone 463 C",
            Self::Pms463U => "Pantone 463 U",
            Self::Pms4635C => "Pantone 4635 C",
            Self::Pms4635U => "Pantone 4635 U",
            Self::Pms464C => "Pantone 464 C",
            Self::Pms464U => "Pantone 464 U",
            Self::Pms4645C => "Pantone 4645 C",
            Self::Pms4645U => "Pantone 4645 U",
            Self::Pms465C => "Pantone 465 C",
            Self::Pms465U => "Pantone 465 U",
            Self::Pms4655C => "Pantone 4655 C",
            Self::Pms4655U => "Pantone 4655 U",
            Self::Pms466C => "Pantone 466 C",
            Self::Pms466U => "Pantone 466 U",
            Self::Pms4665C => "Pantone 4665 C",
            Self::Pms4665U => "Pantone 4665 U",
            Self::Pms467C => "Pantone 467 C",
            Self::Pms467U => "Pantone 467 U",
            Self::Pms4675C => "Pantone 4675 C",
            Self::Pms4675U => "Pantone 4675 U",
            Self::Pms468C => "Pantone 468 C",
            Self::Pms468U => "Pantone 468 U",
            Self::Pms4685C => "Pantone 4685 C",
            Self::Pms4685U => "Pantone 4685 U",
            Self::Pms469C => "Pantone 469 C",
            Self::Pms469U => "Pantone 469 U",
            Self::Pms4695C => "Pantone 4695 C",
            Self::Pms4695U => "Pantone 4695 U",
            Self::Pms470C => "Pantone 470 C",
            Self::Pms470U => "Pantone 470 U",
            Self::Pms4705C => "Pantone 4705 C",
            Self::Pms4705U => "Pantone 4705 U",
            Self::Pms471C => "Pantone 471 C",
            Self::Pms471U => "Pantone 471 U",
            Self::Pms4715C => "Pantone 4715 C",
            Self::Pms4715U => "Pantone 4715 U",
            Self::Pms472C => "Pantone 472 C",
            Self::Pms472U => "Pantone 472 U",
            Self::Pms4725C => "Pantone 4725 C",
            Self::Pms4725U => "Pantone 4725 U",
            Self::Pms473C => "Pantone 473 C",
            Self::Pms473U => "Pantone 473 U",
            Self::Pms4735C => "Pantone 4735 C",
            Self::Pms4735U => "Pantone 4735 U",
            Self::Pms474C => "Pantone 474 C",
            Self::Pms474U => "Pantone 474 U",
            Self::Pms4745C => "Pantone 4745 C",
            Self::Pms4745U => "Pantone 4745 U",
            Self::Pms475C => "Pantone 475 C",
            Self::Pms475U => "Pantone 475 U",
            Self::Pms4755C => "Pantone 4755 C",
            Self::Pms4755U => "Pantone 4755 U",
            Self::Pms476C => "Pantone 476 C",
            Self::Pms476U => "Pantone 476 U",
            Self::Pms477C => "Pantone 477 C",
            Self::Pms477U => "Pantone 477 U",
            Self::Pms478C => "Pantone 478 C",
            Self::Pms478U => "Pantone 478 U",
            Self::Pms479C => "Pantone 479 C",
            Self::Pms479U => "Pantone 479 U",
            Self::Pms480C => "Pantone 480 C",
            Self::Pms480U => "Pantone 480 U",
            Self::Pms481C => "Pantone 481 C",
            Self::Pms481U => "Pantone 481 U",
            Self::Pms482C => "Pantone 482 C",
            Self::Pms482U => "Pantone 482 U",
            Self::Pms483C => "Pantone 483 C",
            Self::Pms483U => "Pantone 483 U",
            Self::Pms484C => "Pantone 484 C",
            Self::Pms484U => "Pantone 484 U",
            Self::Pms485C => "Pantone 485 C",
            Self::Pms485U => "Pantone 485 U",
            Self::Pms486C => "Pantone 486 C",
            Self::Pms486U => "Pantone 486 U",
            Self::Pms487C => "Pantone 487 C",
            Self::Pms487U => "Pantone 487 U",
            Self::Pms488C => "Pantone 488 C",
            Self::Pms488U => "Pantone 488 U",
            Self::Pms489C => "Pantone 489 C",
            Self::Pms489U => "Pantone 489 U",
            Self::Pms490C => "Pantone 490 C",
            Self::Pms490U => "Pantone 490 U",
            Self::Pms491C => "Pantone 491 C",
            Self::Pms491U => "Pantone 491 U",
            Self::Pms492C => "Pantone 492 C",
            Self::Pms492U => "Pantone 492 U",
            Self::Pms493C => "Pantone 493 C",
            Self::Pms493U => "Pantone 493 U",
            Self::Pms494C => "Pantone 494 C",
            Self::Pms494U => "Pantone 494 U",
            Self::Pms495C => "Pantone 495 C",
            Self::Pms495U => "Pantone 495 U",
            Self::Pms496C => "Pantone 496 C",
            Self::Pms496U => "Pantone 496 U",
            Self::Pms497C => "Pantone 497 C",
            Self::Pms497U => "Pantone 497 U",
            Self::Pms4975C => "Pantone 4975 C",
            Self::Pms4975U => "Pantone 4975 U",
            Self::Pms498C => "Pantone 498 C",
            Self::Pms498U => "Pantone 498 U",
            Self::Pms4985C => "Pantone 4985 C",
            Self::Pms4985U => "Pantone 4985 U",
            Self::Pms499C => "Pantone 499 C",
            Self::Pms499U => "Pantone 499 U",
            Self::Pms4995C => "Pantone 4995 C",
            Self::Pms4995U => "Pantone 4995 U",
            Self::Pms500C => "Pantone 500 C",
            Self::Pms500U => "Pantone 500 U",
            Self::Pms5005C => "Pantone 5005 C",
            Self::Pms5005U => "Pantone 5005 U",
            Self::Pms501C => "Pantone 501 C",
            Self::Pms501U => "Pantone 501 U",
            Self::Pms5015C => "Pantone 5015 C",
            Self::Pms5015U => "Pantone 5015 U",
            Self::Pms502C => "Pantone 502 C",
            Self::Pms502U => "Pantone 502 U",
            Self::Pms5025C => "Pantone 5025 C",
            Self::Pms5025U => "Pantone 5025 U",
            Self::Pms503C => "Pantone 503 C",
            Self::Pms503U => "Pantone 503 U",
            Self::Pms5035C => "Pantone 5035 C",
            Self::Pms5035U => "Pantone 5035 U",
            Self::Pms504C => "Pantone 504 C",
            Self::Pms504U => "Pantone 504 U",
            Self::Pms505C => "Pantone 505 C",
            Self::Pms505U => "Pantone 505 U",
            Self::Pms506C => "Pantone 506 C",
            Self::Pms506U => "Pantone 506 U",
            Self::Pms507C => "Pantone 507 C",
            Self::Pms507U => "Pantone 507 U",
            Self::Pms508C => "Pantone 508 C",
            Self::Pms508U => "Pantone 508 U",
            Self::Pms509C => "Pantone 509 C",
            Self::Pms509U => "Pantone 509 U",
            Self::Pms510C => "Pantone 510 C",
            Self::Pms510U => "Pantone 510 U",
            Self::Pms511C => "Pantone 511 C",
            Self::Pms511U => "Pantone 511 U",
            Self::Pms5115C => "Pantone 5115 C",
            Self::Pms5115U => "Pantone 5115 U",
            Self::Pms512C => "Pantone 512 C",
            Self::Pms512U => "Pantone 512 U",
            Self::Pms5125C => "Pantone 5125 C",
            Self::Pms5125U => "Pantone 5125 U",
            Self::Pms513C => "Pantone 513 C",
            Self::Pms513U => "Pantone 513 U",
            Self::Pms5135C => "Pantone 5135 C",
            Self::Pms5135U => "Pantone 5135 U",
            Self::Pms514C => "Pantone 514 C",
            Self::Pms514U => "Pantone 514 U",
            Self::Pms5145C => "Pantone 5145 C",
            Self::Pms5145U => "Pantone 5145 U",
            Self::Pms515C => "Pantone 515 C",
            Self::Pms515U => "Pantone 515 U",
            Self::Pms5155C => "Pantone 5155 C",
            Self::Pms5155U => "Pantone 5155 U",
            Self::Pms516C => "Pantone 516 C",
            Self::Pms516U => "Pantone 516 U",
            Self::Pms5165C => "Pantone 5165 C",
            Self::Pms5165U => "Pantone 5165 U",
            Self::Pms517C => "Pantone 517 C",
            Self::Pms517U => "Pantone 517 U",
            Self::Pms5175C => "Pantone 5175 C",
            Self::Pms5175U => "Pantone 5175 U",
            Self::Pms518C => "Pantone 518 C",
            Self::Pms518U => "Pantone 518 U",
            Self::Pms5185C => "Pantone 5185 C",
            Self::Pms5185U => "Pantone 5185 U",
            Self::Pms519C => "Pantone 519 C",
            Self::Pms519U => "Pantone 519 U",
            Self::Pms5195C => "Pantone 5195 C",
            Self::Pms5195U => "Pantone 5195 U",
            Self::Pms520C => "Pantone 520 C",
            Self::Pms520U => "Pantone 520 U",
            Self::Pms5205C => "Pantone 5205 C",
            Self::Pms5205U => "Pantone 5205 U",
            Self::Pms521C => "Pantone 521 C",
            Self::Pms521U => "Pantone 521 U",
            Self::Pms5215C => "Pantone 5215 C",
            Self::Pms5215U => "Pantone 5215 U",
            Self::Pms522C => "Pantone 522 C",
            Self::Pms522U => "Pantone 522 U",
            Self::Pms5225C => "Pantone 5225 C",
            Self::Pms5225U => "Pantone 5225 U",
            Self::Pms523C => "Pantone 523 C",
            Self::Pms523U => "Pantone 523 U",
            Self::Pms5235C => "Pantone 5235 C",
            Self::Pms5235U => "Pantone 5235 U",
            Self::Pms524C => "Pantone 524 C",
            Self::Pms524U => "Pantone 524 U",
            Self::Pms5245C => "Pantone 5245 C",
            Self::Pms5245U => "Pantone 5245 U",
            Self::Pms525C => "Pantone 525 C",
            Self::Pms525U => "Pantone 525 U",
            Self::Pms5255C => "Pantone 5255 C",
            Self::Pms5255U => "Pantone 5255 U",
            Self::Pms526C => "Pantone 526 C",
            Self::Pms526U => "Pantone 526 U",
            Self::Pms5265C => "Pantone 5265 C",
            Self::Pms5265U => "Pantone 5265 U",
            Self::Pms527C => "Pantone 527 C",
            Self::Pms527U => "Pantone 527 U",
            Self::Pms5275C => "Pantone 5275 C",
            Self::Pms5275U => "Pantone 5275 U",
            Self::Pms528C => "Pantone 528 C",
            Self::Pms528U => "Pantone 528 U",
            Self::Pms5285C => "Pantone 5285 C",
            Self::Pms5285U => "Pantone 5285 U",
            Self::Pms529C => "Pantone 529 C",
            Self::Pms529U => "Pantone 529 U",
            Self::Pms5295C => "Pantone 5295 C",
            Self::Pms5295U => "Pantone 5295 U",
            Self::Pms530C => "Pantone 530 C",
            Self::Pms530U => "Pantone 530 U",
            Self::Pms5305C => "Pantone 5305 C",
            Self::Pms5305U => "Pantone 5305 U",
            Self::Pms531C => "Pantone 531 C",
            Self::Pms531U => "Pantone 531 U",
            Self::Pms5315C => "Pantone 5315 C",
            Self::Pms5315U => "Pantone 5315 U",
            Self::Pms532C => "Pantone 532 C",
            Self::Pms532U => "Pantone 532 U",
            Self::Pms533C => "Pantone 533 C",
            Self::Pms533U => "Pantone 533 U",
            Self::Pms534C => "Pantone 534 C",
            Self::Pms534U => "Pantone 534 U",
            Self::Pms535C => "Pantone 535 C",
            Self::Pms535U => "Pantone 535 U",
            Self::Pms536C => "Pantone 536 C",
            Self::Pms536U => "Pantone 536 U",
            Self::Pms537C => "Pantone 537 C",
            Self::Pms537U => "Pantone 537 U",
            Self::Pms538C => "Pantone 538 C",
            Self::Pms538U => "Pantone 538 U",
            Self::Pms539C => "Pantone 539 C",
            Self::Pms539U => "Pantone 539 U",
            Self::Pms5395C => "Pantone 5395 C",
            Self::Pms5395U => "Pantone 5395 U",
            Self::Pms540C => "Pantone 540 C",
            Self::Pms540U => "Pantone 540 U",
            Self::Pms5405C => "Pantone 5405 C",
            Self::Pms5405U => "Pantone 5405 U",
            Self::Pms541C => "Pantone 541 C",
            Self::Pms541U => "Pantone 541 U",
            Self::Pms5415C => "Pantone 5415 C",
            Self::Pms5415U => "Pantone 5415 U",
            Self::Pms542C => "Pantone 542 C",
            Self::Pms542U => "Pantone 542 U",
            Self::Pms5425C => "Pantone 5425 C",
            Self::Pms5425U => "Pantone 5425 U",
            Self::Pms543C => "Pantone 543 C",
            Self::Pms543U => "Pantone 543 U",
            Self::Pms5435C => "Pantone 5435 C",
            Self::Pms5435U => "Pantone 5435 U",
            Self::Pms544C => "Pantone 544 C",
            Self::Pms544U => "Pantone 544 U",
            Self::Pms5445C => "Pantone 5445 C",
            Self::Pms5445U => "Pantone 5445 U",
            Self::Pms545C => "Pantone 545 C",
            Self::Pms545U => "Pantone 545 U",
            Self::Pms5455C => "Pantone 5455 C",
            Self::Pms5455U => "Pantone 5455 U",
            Self::Pms546C => "Pantone 546 C",
            Self::Pms546U => "Pantone 546 U",
            Self::Pms5463C => "Pantone 5463 C",
            Self::Pms5463U => "Pantone 5463 U",
            Self::Pms5467C => "Pantone 5467 C",
            Self::Pms5467U => "Pantone 5467 U",
            Self::Pms547C => "Pantone 547 C",
            Self::Pms547U => "Pantone 547 U",
            Self::Pms5473C => "Pantone 5473 C",
            Self::Pms5473U => "Pantone 5473 U",
            Self::Pms5477C => "Pantone 5477 C",
            Self::Pms5477U => "Pantone 5477 U",
            Self::Pms548C => "Pantone 548 C",
            Self::Pms548U => "Pantone 548 U",
            Self::Pms5483C => "Pantone 5483 C",
            Self::Pms5483U => "Pantone 5483 U",
            Self::Pms5487C => "Pantone 5487 C",
            Self::Pms5487U => "Pantone 5487 U",
            Self::Pms549C => "Pantone 549 C",
            Self::Pms549U => "Pantone 549 U",
            Self::Pms5493C => "Pantone 5493 C",
            Self::Pms5493U => "Pantone 5493 U",
            Self::Pms5497C => "Pantone 5497 C",
            Self::Pms5497U => "Pantone 5497 U",
            Self::Pms550C => "Pantone 550 C",
            Self::Pms550U => "Pantone 550 U",
            Self::Pms5503C => "Pantone 5503 C",
            Self::Pms5503U => "Pantone 5503 U",
            Self::Pms5507C => "Pantone 5507 C",
            Self::Pms5507U => "Pantone 5507 U",
            Self::Pms551C => "Pantone 551 C",
            Self::Pms551U => "Pantone 551 U",
            Self::Pms5513C => "Pantone 5513 C",
            Self::Pms5513U => "Pantone 5513 U",
            Self::Pms5517C => "Pantone 5517 C",
            Self::Pms5517U => "Pantone 5517 U",
            Self::Pms552C => "Pantone 552 C",
            Self::Pms552U => "Pantone 552 U",
            Self::Pms5523C => "Pantone 5523 C",
            Self::Pms5523U => "Pantone 5523 U",
            Self::Pms5527C => "Pantone 5527 C",
            Self::Pms5527U => "Pantone 5527 U",
            Self::Pms553C => "Pantone 553 C",
            Self::Pms553U => "Pantone 553 U",
            Self::Pms5535C => "Pantone 5535 C",
            Self::Pms5535U => "Pantone 5535 U",
            Self::Pms554C => "Pantone 554 C",
            Self::Pms554U => "Pantone 554 U",
            Self::Pms5545C => "Pantone 5545 C",
            Self::Pms5545U => "Pantone 5545 U",
            Self::Pms555C => "Pantone 555 C",
            Self::Pms555U => "Pantone 555 U",
            Self::Pms5555C => "Pantone 5555 C",
            Self::Pms5555U => "Pantone 5555 U",
            Self::Pms556C => "Pantone 556 C",
            Self::Pms556U => "Pantone 556 U",
            Self::Pms5565C => "Pantone 5565 C",
            Self::Pms5565U => "Pantone 5565 U",
            Self::Pms557C => "Pantone 557 C",
            Self::Pms557U => "Pantone 557 U",
            Self::Pms5575C => "Pantone 5575 C",
            Self::Pms5575U => "Pantone 5575 U",
            Self::Pms558C => "Pantone 558 C",
            Self::Pms558U => "Pantone 558 U",
            Self::Pms5585C => "Pantone 5585 C",
            Self::Pms5585U => "Pantone 5585 U",
            Self::Pms559C => "Pantone 559 C",
            Self::Pms559U => "Pantone 559 U",
            Self::Pms5595C => "Pantone 5595 C",
            Self::Pms5595U => "Pantone 5595 U",
            Self::Pms560C => "Pantone 560 C",
            Self::Pms560U => "Pantone 560 U",
            Self::Pms5605C => "Pantone 5605 C",
            Self::Pms5605U => "Pantone 5605 U",
            Self::Pms561C => "Pantone 561 C",
            Self::Pms561U => "Pantone 561 U",
            Self::Pms5615C => "Pantone 5615 C",
            Self::Pms5615U => "Pantone 5615 U",
            Self::Pms562C => "Pantone 562 C",
            Self::Pms562U => "Pantone 562 U",
            Self::Pms5625C => "Pantone 5625 C",
            Self::Pms5625U => "Pantone 5625 U",
            Self::Pms563C => "Pantone 563 C",
            Self::Pms563U => "Pantone 563 U",
            Self::Pms5635C => "Pantone 5635 C",
            Self::Pms5635U => "Pantone 5635 U",
            Self::Pms564C => "Pantone 564 C",
            Self::Pms564U => "Pantone 564 U",
            Self::Pms5645C => "Pantone 5645 C",
            Self::Pms5645U => "Pantone 5645 U",
            Self::Pms565C => "Pantone 565 C",
            Self::Pms565U => "Pantone 565 U",
            Self::Pms5655C => "Pantone 5655 C",
            Self::Pms5655U => "Pantone 5655 U",
            Self::Pms566C => "Pantone 566 C",
            Self::Pms566U => "Pantone 566 U",
            Self::Pms5665C => "Pantone 5665 C",
            Self::Pms5665U => "Pantone 5665 U",
            Self::Pms567C => "Pantone 567 C",
            Self::Pms567U => "Pantone 567 U",
            Self::Pms568C => "Pantone 568 C",
            Self::Pms568U => "Pantone 568 U",
            Self::Pms569C => "Pantone 569 C",
            Self::Pms569U => "Pantone 569 U",
            Self::Pms570C => "Pantone 570 C",
            Self::Pms570U => "Pantone 570 U",
            Self::Pms571C => "Pantone 571 C",
            Self::Pms571U => "Pantone 571 U",
            Self::Pms572C => "Pantone 572 C",
            Self::Pms572U => "Pantone 572 U",
            Self::Pms573C => "Pantone 573 C",
            Self::Pms573U => "Pantone 573 U",
            Self::Pms574C => "Pantone 574 C",
            Self::Pms574U => "Pantone 574 U",
            Self::Pms5743C => "Pantone 5743 C",
            Self::Pms5743U => "Pantone 5743 U",
            Self::Pms5747C => "Pantone 5747 C",
            Self::Pms5747U => "Pantone 5747 U",
            Self::Pms575C => "Pantone 575 C",
            Self::Pms575U => "Pantone 575 U",
            Self::Pms5753C => "Pantone 5753 C",
            Self::Pms5753U => "Pantone 5753 U",
            Self::Pms5757C => "Pantone 5757 C",
            Self::Pms5757U => "Pantone 5757 U",
            Self::Pms576C => "Pantone 576 C",
            Self::Pms576U => "Pantone 576 U",
            Self::Pms5763C => "Pantone 5763 C",
            Self::Pms5763U => "Pantone 5763 U",
            Self::Pms5767C => "Pantone 5767 C",
            Self::Pms5767U => "Pantone 5767 U",
            Self::Pms577C => "Pantone 577 C",
            Self::Pms577U => "Pantone 577 U",
            Self::Pms5773C => "Pantone 5773 C",
            Self::Pms5773U => "Pantone 5773 U",
            Self::Pms5777C => "Pantone 5777 C",
            Self::Pms5777U => "Pantone 5777 U",
            Self::Pms578C => "Pantone 578 C",
            Self::Pms578U => "Pantone 578 U",
            Self::Pms5783C => "Pantone 5783 C",
            Self::Pms5783U => "Pantone 5783 U",
            Self::Pms5787C => "Pantone 5787 C",
            Self::Pms5787U => "Pantone 5787 U",
            Self::Pms579C => "Pantone 579 C",
            Self::Pms579U => "Pantone 579 U",
            Self::Pms5793C => "Pantone 5793 C",
            Self::Pms5793U => "Pantone 5793 U",
            Self::Pms5797C => "Pantone 5797 C",
            Self::Pms5797U => "Pantone 5797 U",
            Self::Pms580C => "Pantone 580 C",
            Self::Pms580U => "Pantone 580 U",
            Self::Pms5803C => "Pantone 5803 C",
            Self::Pms5803U => "Pantone 5803 U",
            Self::Pms5807C => "Pantone 5807 C",
            Self::Pms5807U => "Pantone 5807 U",
            Self::Pms581C => "Pantone 581 C",
            Self::Pms581U => "Pantone 581 U",
            Self::Pms5815C => "Pantone 5815 C",
            Self::Pms5815U => "Pantone 5815 U",
            Self::Pms582C => "Pantone 582 C",
            Self::Pms582U => "Pantone 582 U",
            Self::Pms5825C => "Pantone 5825 C",
            Self::Pms5825U => "Pantone 5825 U",
            Self::Pms583C => "Pantone 583 C",
            Self::Pms583U => "Pantone 583 U",
            Self::Pms5835C => "Pantone 5835 C",
            Self::Pms5835U => "Pantone 5835 U",
            Self::Pms584C => "Pantone 584 C",
            Self::Pms584U => "Pantone 584 U",
            Self::Pms5845C => "Pantone 5845 C",
            Self::Pms5845U => "Pantone 5845 U",
            Self::Pms585C => "Pantone 585 C",
            Self::Pms585U => "Pantone 585 U",
            Self::Pms5855C => "Pantone 5855 C",
            Self::Pms5855U => "Pantone 5855 U",
            Self::Pms586C => "Pantone 586 C",
            Self::Pms586U => "Pantone 586 U",
            Self::Pms5865C => "Pantone 5865 C",
            Self::Pms5865U => "Pantone 5865 U",
            Self::Pms587C => "Pantone 587 C",
            Self::Pms587U => "Pantone 587 U",
            Self::Pms5875C => "Pantone 5875 C",
            Self::Pms5875U => "Pantone 5875 U",
            Self::Pms600C => "Pantone 600 C",
            Self::Pms600U => "Pantone 600 U",
            Self::Pms601C => "Pantone 601 C",
            Self::Pms601U => "Pantone 601 U",
            Self::Pms602C => "Pantone 602 C",
            Self::Pms602U => "Pantone 602 U",
            Self::Pms603C => "Pantone 603 C",
            Self::Pms603U => "Pantone 603 U",
            Self::Pms604C => "Pantone 604 C",
            Self::Pms604U => "Pantone 604 U",
            Self::Pms605C => "Pantone 605 C",
            Self::Pms605U => "Pantone 605 U",
            Self::Pms606C => "Pantone 606 C",
            Self::Pms606U => "Pantone 606 U",
            Self::Pms607C => "Pantone 607 C",
            Self::Pms607U => "Pantone 607 U",
            Self::Pms608C => "Pantone 608 C",
            Self::Pms608U => "Pantone 608 U",
            Self::Pms609C => "Pantone 609 C",
            Self::Pms609U => "Pantone 609 U",
            Self::Pms610C => "Pantone 610 C",
            Self::Pms610U => "Pantone 610 U",
            Self::Pms611C => "Pantone 611 C",
            Self::Pms611U => "Pantone 611 U",
            Self::Pms612C => "Pantone 612 C",
            Self::Pms612U => "Pantone 612 U",
            Self::Pms613C => "Pantone 613 C",
            Self::Pms613U => "Pantone 613 U",
            Self::Pms614C => "Pantone 614 C",
            Self::Pms614U => "Pantone 614 U",
            Self::Pms615C => "Pantone 615 C",
            Self::Pms615U => "Pantone 615 U",
            Self::Pms616C => "Pantone 616 C",
            Self::Pms616U => "Pantone 616 U",
            Self::Pms617C => "Pantone 617 C",
            Self::Pms617U => "Pantone 617 U",
            Self::Pms618C => "Pantone 618 C",
            Self::Pms618U => "Pantone 618 U",
            Self::Pms619C => "Pantone 619 C",
            Self::Pms619U => "Pantone 619 U",
            Self::Pms620C => "Pantone 620 C",
            Self::Pms620U => "Pantone 620 U",
            Self::Pms621C => "Pantone 621 C",
            Self::Pms621U => "Pantone 621 U",
            Self::Pms622C => "Pantone 622 C",
            Self::Pms622U => "Pantone 622 U",
            Self::Pms623C => "Pantone 623 C",
            Self::Pms623U => "Pantone 623 U",
            Self::Pms624C => "Pantone 624 C",
            Self::Pms624U => "Pantone 624 U",
            Self::Pms625C => "Pantone 625 C",
            Self::Pms625U => "Pantone 625 U",
            Self::Pms626C => "Pantone 626 C",
            Self::Pms626U => "Pantone 626 U",
            Self::Pms627C => "Pantone 627 C",
            Self::Pms627U => "Pantone 627 U",
            Self::Pms628C => "Pantone 628 C",
            Self::Pms628U => "Pantone 628 U",
            Self::Pms629C => "Pantone 629 C",
            Self::Pms629U => "Pantone 629 U",
            Self::Pms630C => "Pantone 630 C",
            Self::Pms630U => "Pantone 630 U",
            Self::Pms631C => "Pantone 631 C",
            Self::Pms631U => "Pantone 631 U",
            Self::Pms632C => "Pantone 632 C",
            Self::Pms632U => "Pantone 632 U",
            Self::Pms633C => "Pantone 633 C",
            Self::Pms633U => "Pantone 633 U",
            Self::Pms634C => "Pantone 634 C",
            Self::Pms634U => "Pantone 634 U",
            Self::Pms635C => "Pantone 635 C",
            Self::Pms635U => "Pantone 635 U",
            Self::Pms636C => "Pantone 636 C",
            Self::Pms636U => "Pantone 636 U",
            Self::Pms637C => "Pantone 637 C",
            Self::Pms637U => "Pantone 637 U",
            Self::Pms638C => "Pantone 638 C",
            Self::Pms638U => "Pantone 638 U",
            Self::Pms639C => "Pantone 639 C",
            Self::Pms639U => "Pantone 639 U",
            Self::Pms640C => "Pantone 640 C",
            Self::Pms640U => "Pantone 640 U",
            Self::Pms641C => "Pantone 641 C",
            Self::Pms641U => "Pantone 641 U",
            Self::Pms642C => "Pantone 642 C",
            Self::Pms642U => "Pantone 642 U",
            Self::Pms643C => "Pantone 643 C",
            Self::Pms643U => "Pantone 643 U",
            Self::Pms644C => "Pantone 644 C",
            Self::Pms644U => "Pantone 644 U",
            Self::Pms645C => "Pantone 645 C",
            Self::Pms645U => "Pantone 645 U",
            Self::Pms646C => "Pantone 646 C",
            Self::Pms646U => "Pantone 646 U",
            Self::Pms647C => "Pantone 647 C",
            Self::Pms647U => "Pantone 647 U",
            Self::Pms648C => "Pantone 648 C",
            Self::Pms648U => "Pantone 648 U",
            Self::Pms649C => "Pantone 649 C",
            Self::Pms649U => "Pantone 649 U",
            Self::Pms650C => "Pantone 650 C",
            Self::Pms650U => "Pantone 650 U",
            Self::Pms651C => "Pantone 651 C",
            Self::Pms651U => "Pantone 651 U",
            Self::Pms652C => "Pantone 652 C",
            Self::Pms652U => "Pantone 652 U",
            Self::Pms653C => "Pantone 653 C",
            Self::Pms653U => "Pantone 653 U",
            Self::Pms654C => "Pantone 654 C",
            Self::Pms654U => "Pantone 654 U",
            Self::Pms655C => "Pantone 655 C",
            Self::Pms655U => "Pantone 655 U",
            Self::Pms656C => "Pantone 656 C",
            Self::Pms656U => "Pantone 656 U",
            Self::Pms657C => "Pantone 657 C",
            Self::Pms657U => "Pantone 657 U",
            Self::Pms658C => "Pantone 658 C",
            Self::Pms658U => "Pantone 658 U",
            Self::Pms659C => "Pantone 659 C",
            Self::Pms659U => "Pantone 659 U",
            Self::Pms660C => "Pantone 660 C",
            Self::Pms660U => "Pantone 660 U",
            Self::Pms661C => "Pantone 661 C",
            Self::Pms661U => "Pantone 661 U",
            Self::Pms662C => "Pantone 662 C",
            Self::Pms662U => "Pantone 662 U",
            Self::Pms663C => "Pantone 663 C",
            Self::Pms663U => "Pantone 663 U",
            Self::Pms664C => "Pantone 664 C",
            Self::Pms664U => "Pantone 664 U",
            Self::Pms665C => "Pantone 665 C",
            Self::Pms665U => "Pantone 665 U",
            Self::Pms666C => "Pantone 666 C",
            Self::Pms666U => "Pantone 666 U",
            Self::Pms667C => "Pantone 667 C",
            Self::Pms667U => "Pantone 667 U",
            Self::Pms668C => "Pantone 668 C",
            Self::Pms668U => "Pantone 668 U",
            Self::Pms669C => "Pantone 669 C",
            Self::Pms669U => "Pantone 669 U",
            Self::Pms670C => "Pantone 670 C",
            Self::Pms670U => "Pantone 670 U",
            Self::Pms671C => "Pantone 671 C",
            Self::Pms671U => "Pantone 671 U",
            Self::Pms672C => "Pantone 672 C",
            Self::Pms672U => "Pantone 672 U",
            Self::Pms673C => "Pantone 673 C",
            Self::Pms673U => "Pantone 673 U",
            Self::Pms674C => "Pantone 674 C",
            Self::Pms674U => "Pantone 674 U",
            Self::Pms675C => "Pantone 675 C",
            Self::Pms675U => "Pantone 675 U",
            Self::Pms676C => "Pantone 676 C",
            Self::Pms676U => "Pantone 676 U",
            Self::Pms677C => "Pantone 677 C",
            Self::Pms677U => "Pantone 677 U",
            Self::Pms678C => "Pantone 678 C",
            Self::Pms678U => "Pantone 678 U",
            Self::Pms679C => "Pantone 679 C",
            Self::Pms679U => "Pantone 679 U",
            Self::Pms680C => "Pantone 680 C",
            Self::Pms680U => "Pantone 680 U",
            Self::Pms681C => "Pantone 681 C",
            Self::Pms681U => "Pantone 681 U",
            Self::Pms682C => "Pantone 682 C",
            Self::Pms682U => "Pantone 682 U",
            Self::Pms683C => "Pantone 683 C",
            Self::Pms683U => "Pantone 683 U",
            Self::Pms684C => "Pantone 684 C",
            Self::Pms684U => "Pantone 684 U",
            Self::Pms685C => "Pantone 685 C",
            Self::Pms685U => "Pantone 685 U",
            Self::Pms686C => "Pantone 686 C",
            Self::Pms686U => "Pantone 686 U",
            Self::Pms687C => "Pantone 687 C",
            Self::Pms687U => "Pantone 687 U",
            Self::Pms688C => "Pantone 688 C",
            Self::Pms688U => "Pantone 688 U",
            Self::Pms689C => "Pantone 689 C",
            Self::Pms689U => "Pantone 689 U",
            Self::Pms690C => "Pantone 690 C",
            Self::Pms690U => "Pantone 690 U",
            Self::Pms691C => "Pantone 691 C",
            Self::Pms691U => "Pantone 691 U",
            Self::Pms692C => "Pantone 692 C",
            Self::Pms692U => "Pantone 692 U",
            Self::Pms693C => "Pantone 693 C",
            Self::Pms693U => "Pantone 693 U",
            Self::Pms694C => "Pantone 694 C",
            Self::Pms694U => "Pantone 694 U",
            Self::Pms695C => "Pantone 695 C",
            Self::Pms695U => "Pantone 695 U",
            Self::Pms696C => "Pantone 696 C",
            Self::Pms696U => "Pantone 696 U",
            Self::Pms697C => "Pantone 697 C",
            Self::Pms697U => "Pantone 697 U",
            Self::Pms698C => "Pantone 698 C",
            Self::Pms698U => "Pantone 698 U",
            Self::Pms699C => "Pantone 699 C",
            Self::Pms699U => "Pantone 699 U",
            Self::Pms700C => "Pantone 700 C",
            Self::Pms700U => "Pantone 700 U",
            Self::Pms701C => "Pantone 701 C",
            Self::Pms701U => "Pantone 701 U",
            Self::Pms702C => "Pantone 702 C",
            Self::Pms702U => "Pantone 702 U",
            Self::Pms703C => "Pantone 703 C",
            Self::Pms703U => "Pantone 703 U",
            Self::Pms704C => "Pantone 704 C",
            Self::Pms704U => "Pantone 704 U",
            Self::Pms705C => "Pantone 705 C",
            Self::Pms705U => "Pantone 705 U",
            Self::Pms706C => "Pantone 706 C",
            Self::Pms706U => "Pantone 706 U",
            Self::Pms707C => "Pantone 707 C",
            Self::Pms707U => "Pantone 707 U",
            Self::Pms708C => "Pantone 708 C",
            Self::Pms708U => "Pantone 708 U",
            Self::Pms709C => "Pantone 709 C",
            Self::Pms709U => "Pantone 709 U",
            Self::Pms710C => "Pantone 710 C",
            Self::Pms710U => "Pantone 710 U",
            Self::Pms711C => "Pantone 711 C",
            Self::Pms711U => "Pantone 711 U",
            Self::Pms712C => "Pantone 712 C",
            Self::Pms712U => "Pantone 712 U",
            Self::Pms713C => "Pantone 713 C",
            Self::Pms713U => "Pantone 713 U",
            Self::Pms714C => "Pantone 714 C",
            Self::Pms714U => "Pantone 714 U",
            Self::Pms715C => "Pantone 715 C",
            Self::Pms715U => "Pantone 715 U",
            Self::Pms716C => "Pantone 716 C",
            Self::Pms716U => "Pantone 716 U",
            Self::Pms717C => "Pantone 717 C",
            Self::Pms717U => "Pantone 717 U",
            Self::Pms718C => "Pantone 718 C",
            Self::Pms718U => "Pantone 718 U",
            Self::Pms719C => "Pantone 719 C",
            Self::Pms719U => "Pantone 719 U",
            Self::Pms720C => "Pantone 720 C",
            Self::Pms720U => "Pantone 720 U",
            Self::Pms721C => "Pantone 721 C",
            Self::Pms721U => "Pantone 721 U",
            Self::Pms722C => "Pantone 722 C",
            Self::Pms722U => "Pantone 722 U",
            Self::Pms723C => "Pantone 723 C",
            Self::Pms723U => "Pantone 723 U",
            Self::Pms724C => "Pantone 724 C",
            Self::Pms724U => "Pantone 724 U",
            Self::Pms725C => "Pantone 725 C",
            Self::Pms725U => "Pantone 725 U",
            Self::Pms726C => "Pantone 726 C",
            Self::Pms726U => "Pantone 726 U",
            Self::Pms727C => "Pantone 727 C",
            Self::Pms727U => "Pantone 727 U",
            Self::Pms728C => "Pantone 728 C",
            Self::Pms728U => "Pantone 728 U",
            Self::Pms729C => "Pantone 729 C",
            Self::Pms729U => "Pantone 729 U",
            Self::Pms730C => "Pantone 730 C",
            Self::Pms730U => "Pantone 730 U",
            Self::Pms731C => "Pantone 731 C",
            Self::Pms731U => "Pantone 731 U",
            Self::Pms732C => "Pantone 732 C",
            Self::Pms732U => "Pantone 732 U",
            Self::Pms7401C => "Pantone 7401 C",
            Self::Pms7401U => "Pantone 7401 U",
            Self::Pms7402C => "Pantone 7402 C",
            Self::Pms7402U => "Pantone 7402 U",
            Self::Pms7403C => "Pantone 7403 C",
            Self::Pms7403U => "Pantone 7403 U",
            Self::Pms7404C => "Pantone 7404 C",
            Self::Pms7404U => "Pantone 7404 U",
            Self::Pms7405C => "Pantone 7405 C",
            Self::Pms7405U => "Pantone 7405 U",
            Self::Pms7406C => "Pantone 7406 C",
            Self::Pms7406U => "Pantone 7406 U",
            Self::Pms7407C => "Pantone 7407 C",
            Self::Pms7407U => "Pantone 7407 U",
            Self::Pms7408C => "Pantone 7408 C",
            Self::Pms7408U => "Pantone 7408 U",
            Self::Pms7409C => "Pantone 7409 C",
            Self::Pms7409U => "Pantone 7409 U",
            Self::Pms7410C => "Pantone 7410 C",
            Self::Pms7410U => "Pantone 7410 U",
            Self::Pms7411C => "Pantone 7411 C",
            Self::Pms7411U => "Pantone 7411 U",
            Self::Pms7412C => "Pantone 7412 C",
            Self::Pms7412U => "Pantone 7412 U",
            Self::Pms7413C => "Pantone 7413 C",
            Self::Pms7413U => "Pantone 7413 U",
            Self::Pms7414C => "Pantone 7414 C",
            Self::Pms7414U => "Pantone 7414 U",
            Self::Pms7415C => "Pantone 7415 C",
            Self::Pms7415U => "Pantone 7415 U",
            Self::Pms7416C => "Pantone 7416 C",
            Self::Pms7416U => "Pantone 7416 U",
            Self::Pms7417C => "Pantone 7417 C",
            Self::Pms7417U => "Pantone 7417 U",
            Self::Pms7418C => "Pantone 7418 C",
            Self::Pms7418U => "Pantone 7418 U",
            Self::Pms7419C => "Pantone 7419 C",
            Self::Pms7419U => "Pantone 7419 U",
            Self::Pms7420C => "Pantone 7420 C",
            Self::Pms7420U => "Pantone 7420 U",
            Self::Pms7421C => "Pantone 7421 C",
            Self::Pms7421U => "Pantone 7421 U",
            Self::Pms7422C => "Pantone 7422 C",
            Self::Pms7422U => "Pantone 7422 U",
            Self::Pms7423C => "Pantone 7423 C",
            Self::Pms7423U => "Pantone 7423 U",
            Self::Pms7424C => "Pantone 7424 C",
            Self::Pms7424U => "Pantone 7424 U",
            Self::Pms7425C => "Pantone 7425 C",
            Self::Pms7425U => "Pantone 7425 U",
            Self::Pms7426C => "Pantone 7426 C",
            Self::Pms7426U => "Pantone 7426 U",
            Self::Pms7427C => "Pantone 7427 C",
            Self::Pms7427U => "Pantone 7427 U",
            Self::Pms7428C => "Pantone 7428 C",
            Self::Pms7428U => "Pantone 7428 U",
            Self::Pms7429C => "Pantone 7429 C",
            Self::Pms7429U => "Pantone 7429 U",
            Self::Pms7430C => "Pantone 7430 C",
            Self::Pms7430U => "Pantone 7430 U",
            Self::Pms7431C => "Pantone 7431 C",
            Self::Pms7431U => "Pantone 7431 U",
            Self::Pms7432C => "Pantone 7432 C",
            Self::Pms7432U => "Pantone 7432 U",
            Self::Pms7433C => "Pantone 7433 C",
            Self::Pms7433U => "Pantone 7433 U",
            Self::Pms7434C => "Pantone 7434 C",
            Self::Pms7434U => "Pantone 7434 U",
            Self::Pms7435C => "Pantone 7435 C",
            Self::Pms7435U => "Pantone 7435 U",
            Self::Pms7436C => "Pantone 7436 C",
            Self::Pms7436U => "Pantone 7436 U",
            Self::Pms7437C => "Pantone 7437 C",
            Self::Pms7437U => "Pantone 7437 U",
            Self::Pms7438C => "Pantone 7438 C",
            Self::Pms7438U => "Pantone 7438 U",
            Self::Pms7439C => "Pantone 7439 C",
            Self::Pms7439U => "Pantone 7439 U",
            Self::Pms7440C => "Pantone 7440 C",
            Self::Pms7440U => "Pantone 7440 U",
            Self::Pms7441C => "Pantone 7441 C",
            Self::Pms7441U => "Pantone 7441 U",
            Self::Pms7442C => "Pantone 7442 C",
            Self::Pms7442U => "Pantone 7442 U",
            Self::Pms7443C => "Pantone 7443 C",
            Self::Pms7443U => "Pantone 7443 U",
            Self::Pms7444C => "Pantone 7444 C",
            Self::Pms7444U => "Pantone 7444 U",
            Self::Pms7445C => "Pantone 7445 C",
            Self::Pms7445U => "Pantone 7445 U",
            Self::Pms7446C => "Pantone 7446 C",
            Self::Pms7446U => "Pantone 7446 U",
            Self::Pms7447C => "Pantone 7447 C",
            Self::Pms7447U => "Pantone 7447 U",
            Self::Pms7448C => "Pantone 7448 C",
            Self::Pms7448U => "Pantone 7448 U",
            Self::Pms7449C => "Pantone 7449 C",
            Self::Pms7449U => "Pantone 7449 U",
            Self::Pms7450C => "Pantone 7450 C",
            Self::Pms7450U => "Pantone 7450 U",
            Self::Pms7451C => "Pantone 7451 C",
            Self::Pms7451U => "Pantone 7451 U",
            Self::Pms7452C => "Pantone 7452 C",
            Self::Pms7452U => "Pantone 7452 U",
            Self::Pms7453C => "Pantone 7453 C",
            Self::Pms7453U => "Pantone 7453 U",
            Self::Pms7454C => "Pantone 7454 C",
            Self::Pms7454U => "Pantone 7454 U",
            Self::Pms7455C => "Pantone 7455 C",
            Self::Pms7455U => "Pantone 7455 U",
            Self::Pms7456C => "Pantone 7456 C",
            Self::Pms7456U => "Pantone 7456 U",
            Self::Pms7457C => "Pantone 7457 C",
            Self::Pms7457U => "Pantone 7457 U",
            Self::Pms7458C => "Pantone 7458 C",
            Self::Pms7458U => "Pantone 7458 U",
            Self::Pms7459C => "Pantone 7459 C",
            Self::Pms7459U => "Pantone 7459 U",
            Self::Pms7460C => "Pantone 7460 C",
            Self::Pms7460U => "Pantone 7460 U",
            Self::Pms7461C => "Pantone 7461 C",
            Self::Pms7461U => "Pantone 7461 U",
            Self::Pms7462C => "Pantone 7462 C",
            Self::Pms7462U => "Pantone 7462 U",
            Self::Pms7463C => "Pantone 7463 C",
            Self::Pms7463U => "Pantone 7463 U",
            Self::Pms7464C => "Pantone 7464 C",
            Self::Pms7464U => "Pantone 7464 U",
            Self::Pms7465C => "Pantone 7465 C",
            Self::Pms7465U => "Pantone 7465 U",
            Self::Pms7466C => "Pantone 7466 C",
            Self::Pms7466U => "Pantone 7466 U",
            Self::Pms7467C => "Pantone 7467 C",
            Self::Pms7467U => "Pantone 7467 U",
            Self::Pms7468C => "Pantone 7468 C",
            Self::Pms7468U => "Pantone 7468 U",
            Self::Pms7469C => "Pantone 7469 C",
            Self::Pms7469U => "Pantone 7469 U",
            Self::Pms7470C => "Pantone 7470 C",
            Self::Pms7470U => "Pantone 7470 U",
            Self::Pms7471C => "Pantone 7471 C",
            Self::Pms7471U => "Pantone 7471 U",
            Self::Pms7472C => "Pantone 7472 C",
            Self::Pms7472U => "Pantone 7472 U",
            Self::Pms7473C => "Pantone 7473 C",
            Self::Pms7473U => "Pantone 7473 U",
            Self::Pms7474C => "Pantone 7474 C",
            Self::Pms7474U => "Pantone 7474 U",
            Self::Pms7475C => "Pantone 7475 C",
            Self::Pms7475U => "Pantone 7475 U",
            Self::Pms7476C => "Pantone 7476 C",
            Self::Pms7476U => "Pantone 7476 U",
            Self::Pms7477C => "Pantone 7477 C",
            Self::Pms7477U => "Pantone 7477 U",
            Self::Pms7478C => "Pantone 7478 C",
            Self::Pms7478U => "Pantone 7478 U",
            Self::Pms7479C => "Pantone 7479 C",
            Self::Pms7479U => "Pantone 7479 U",
            Self::Pms7480C => "Pantone 7480 C",
            Self::Pms7480U => "Pantone 7480 U",
            Self::Pms7481C => "Pantone 7481 C",
            Self::Pms7481U => "Pantone 7481 U",
            Self::Pms7482C => "Pantone 7482 C",
            Self::Pms7482U => "Pantone 7482 U",
            Self::Pms7483C => "Pantone 7483 C",
            Self::Pms7483U => "Pantone 7483 U",
            Self::Pms7484C => "Pantone 7484 C",
            Self::Pms7484U => "Pantone 7484 U",
            Self::Pms7485C => "Pantone 7485 C",
            Self::Pms7485U => "Pantone 7485 U",
            Self::Pms7486C => "Pantone 7486 C",
            Self::Pms7486U => "Pantone 7486 U",
            Self::Pms7487C => "Pantone 7487 C",
            Self::Pms7487U => "Pantone 7487 U",
            Self::Pms7488C => "Pantone 7488 C",
            Self::Pms7488U => "Pantone 7488 U",
            Self::Pms7489C => "Pantone 7489 C",
            Self::Pms7489U => "Pantone 7489 U",
            Self::Pms7490C => "Pantone 7490 C",
            Self::Pms7490U => "Pantone 7490 U",
            Self::Pms7491C => "Pantone 7491 C",
            Self::Pms7491U => "Pantone 7491 U",
            Self::Pms7492C => "Pantone 7492 C",
            Self::Pms7492U => "Pantone 7492 U",
            Self::Pms7493C => "Pantone 7493 C",
            Self::Pms7493U => "Pantone 7493 U",
            Self::Pms7494C => "Pantone 7494 C",
            Self::Pms7494U => "Pantone 7494 U",
            Self::Pms7495C => "Pantone 7495 C",
            Self::Pms7495U => "Pantone 7495 U",
            Self::Pms7496C => "Pantone 7496 C",
            Self::Pms7496U => "Pantone 7496 U",
            Self::Pms7497C => "Pantone 7497 C",
            Self::Pms7497U => "Pantone 7497 U",
            Self::Pms7498C => "Pantone 7498 C",
            Self::Pms7498U => "Pantone 7498 U",
            Self::Pms7499C => "Pantone 7499 C",
            Self::Pms7499U => "Pantone 7499 U",
            Self::Pms7500C => "Pantone 7500 C",
            Self::Pms7500U => "Pantone 7500 U",
            Self::Pms7501C => "Pantone 7501 C",
            Self::Pms7501U => "Pantone 7501 U",
            Self::Pms7502C => "Pantone 7502 C",
            Self::Pms7502U => "Pantone 7502 U",
            Self::Pms7503C => "Pantone 7503 C",
            Self::Pms7503U => "Pantone 7503 U",
            Self::Pms7504C => "Pantone 7504 C",
            Self::Pms7504U => "Pantone 7504 U",
            Self::Pms7505C => "Pantone 7505 C",
            Self::Pms7505U => "Pantone 7505 U",
            Self::Pms7506C => "Pantone 7506 C",
            Self::Pms7506U => "Pantone 7506 U",
            Self::Pms7507C => "Pantone 7507 C",
            Self::Pms7507U => "Pantone 7507 U",
            Self::Pms7508C => "Pantone 7508 C",
            Self::Pms7508U => "Pantone 7508 U",
            Self::Pms7509C => "Pantone 7509 C",
            Self::Pms7509U => "Pantone 7509 U",
            Self::Pms7510C => "Pantone 7510 C",
            Self::Pms7510U => "Pantone 7510 U",
            Self::Pms7511C => "Pantone 7511 C",
            Self::Pms7511U => "Pantone 7511 U",
            Self::Pms7512C => "Pantone 7512 C",
            Self::Pms7512U => "Pantone 7512 U",
            Self::Pms7513C => "Pantone 7513 C",
            Self::Pms7513U => "Pantone 7513 U",
            Self::Pms7514C => "Pantone 7514 C",
            Self::Pms7514U => "Pantone 7514 U",
            Self::Pms7515C => "Pantone 7515 C",
            Self::Pms7515U => "Pantone 7515 U",
            Self::Pms7516C => "Pantone 7516 C",
            Self::Pms7516U => "Pantone 7516 U",
            Self::Pms7517C => "Pantone 7517 C",
            Self::Pms7517U => "Pantone 7517 U",
            Self::Pms7518C => "Pantone 7518 C",
            Self::Pms7518U => "Pantone 7518 U",
            Self::Pms7519C => "Pantone 7519 C",
            Self::Pms7519U => "Pantone 7519 U",
            Self::Pms7520C => "Pantone 7520 C",
            Self::Pms7520U => "Pantone 7520 U",
            Self::Pms7521C => "Pantone 7521 C",
            Self::Pms7521U => "Pantone 7521 U",
            Self::Pms7522C => "Pantone 7522 C",
            Self::Pms7522U => "Pantone 7522 U",
            Self::Pms7523C => "Pantone 7523 C",
            Self::Pms7523U => "Pantone 7523 U",
            Self::Pms7524C => "Pantone 7524 C",
            Self::Pms7524U => "Pantone 7524 U",
            Self::Pms7525C => "Pantone 7525 C",
            Self::Pms7525U => "Pantone 7525 U",
            Self::Pms7526C => "Pantone 7526 C",
            Self::Pms7526U => "Pantone 7526 U",
            Self::Pms7527C => "Pantone 7527 C",
            Self::Pms7527U => "Pantone 7527 U",
            Self::Pms7528C => "Pantone 7528 C",
            Self::Pms7528U => "Pantone 7528 U",
            Self::Pms7529C => "Pantone 7529 C",
            Self::Pms7529U => "Pantone 7529 U",
            Self::Pms7530C => "Pantone 7530 C",
            Self::Pms7530U => "Pantone 7530 U",
            Self::Pms7531C => "Pantone 7531 C",
            Self::Pms7531U => "Pantone 7531 U",
            Self::Pms7532C => "Pantone 7532 C",
            Self::Pms7532U => "Pantone 7532 U",
            Self::Pms7533C => "Pantone 7533 C",
            Self::Pms7533U => "Pantone 7533 U",
            Self::Pms7534C => "Pantone 7534 C",
            Self::Pms7534U => "Pantone 7534 U",
            Self::Pms7535C => "Pantone 7535 C",
            Self::Pms7535U => "Pantone 7535 U",
            Self::Pms7536C => "Pantone 7536 C",
            Self::Pms7536U => "Pantone 7536 U",
            Self::Pms7537C => "Pantone 7537 C",
            Self::Pms7537U => "Pantone 7537 U",
            Self::Pms7538C => "Pantone 7538 C",
            Self::Pms7538U => "Pantone 7538 U",
            Self::Pms7539C => "Pantone 7539 C",
            Self::Pms7539U => "Pantone 7539 U",
            Self::Pms7540C => "Pantone 7540 C",
            Self::Pms7540U => "Pantone 7540 U",
            Self::Pms7541C => "Pantone 7541 C",
            Self::Pms7541U => "Pantone 7541 U",
            Self::Pms7542C => "Pantone 7542 C",
            Self::Pms7542U => "Pantone 7542 U",
            Self::Pms7543C => "Pantone 7543 C",
            Self::Pms7543U => "Pantone 7543 U",
            Self::Pms7544C => "Pantone 7544 C",
            Self::Pms7544U => "Pantone 7544 U",
            Self::Pms7545C => "Pantone 7545 C",
            Self::Pms7545U => "Pantone 7545 U",
            Self::Pms7546C => "Pantone 7546 C",
            Self::Pms7546U => "Pantone 7546 U",
            Self::Pms7547C => "Pantone 7547 C",
            Self::Pms7547U => "Pantone 7547 U",
            Self::Pms7548C => "Pantone 7548 C",
            Self::Pms7548U => "Pantone 7548 U",
            Self::Pms7549C => "Pantone 7549 C",
            Self::Pms7549U => "Pantone 7549 U",
            Self::Pms7550C => "Pantone 7550 C",
            Self::Pms7550U => "Pantone 7550 U",
            Self::Pms7551C => "Pantone 7551 C",
            Self::Pms7551U => "Pantone 7551 U",
            Self::Pms7552C => "Pantone 7552 C",
            Self::Pms7552U => "Pantone 7552 U",
            Self::Pms7553C => "Pantone 7553 C",
            Self::Pms7553U => "Pantone 7553 U",
            Self::Pms7554C => "Pantone 7554 C",
            Self::Pms7554U => "Pantone 7554 U",
            Self::Pms7555C => "Pantone 7555 C",
            Self::Pms7555U => "Pantone 7555 U",
            Self::Pms7556C => "Pantone 7556 C",
            Self::Pms7556U => "Pantone 7556 U",
            Self::Pms7557C => "Pantone 7557 C",
            Self::Pms7557U => "Pantone 7557 U",
            Self::Pms7558C => "Pantone 7558 C",
            Self::Pms7558U => "Pantone 7558 U",
            Self::Pms7559C => "Pantone 7559 C",
            Self::Pms7559U => "Pantone 7559 U",
            Self::Pms7560C => "Pantone 7560 C",
            Self::Pms7560U => "Pantone 7560 U",
            Self::Pms7561C => "Pantone 7561 C",
            Self::Pms7561U => "Pantone 7561 U",
            Self::Pms7562C => "Pantone 7562 C",
            Self::Pms7562U => "Pantone 7562 U",
            Self::Pms7563C => "Pantone 7563 C",
            Self::Pms7563U => "Pantone 7563 U",
            Self::Pms7564C => "Pantone 7564 C",
            Self::Pms7564U => "Pantone 7564 U",
            Self::Pms7565C => "Pantone 7565 C",
            Self::Pms7565U => "Pantone 7565 U",
            Self::Pms7566C => "Pantone 7566 C",
            Self::Pms7566U => "Pantone 7566 U",
            Self::Pms7567C => "Pantone 7567 C",
            Self::Pms7567U => "Pantone 7567 U",
            Self::Pms7568C => "Pantone 7568 C",
            Self::Pms7568U => "Pantone 7568 U",
            Self::Pms7569C => "Pantone 7569 C",
            Self::Pms7569U => "Pantone 7569 U",
            Self::Pms7570C => "Pantone 7570 C",
            Self::Pms7570U => "Pantone 7570 U",
            Self::Pms7571C => "Pantone 7571 C",
            Self::Pms7571U => "Pantone 7571 U",
            Self::Pms7572C => "Pantone 7572 C",
            Self::Pms7572U => "Pantone 7572 U",
            Self::Pms7573C => "Pantone 7573 C",
            Self::Pms7573U => "Pantone 7573 U",
            Self::Pms7574C => "Pantone 7574 C",
            Self::Pms7574U => "Pantone 7574 U",
            Self::Pms7575C => "Pantone 7575 C",
            Self::Pms7575U => "Pantone 7575 U",
            Self::Pms7576C => "Pantone 7576 C",
            Self::Pms7576U => "Pantone 7576 U",
            Self::Pms7577C => "Pantone 7577 C",
            Self::Pms7577U => "Pantone 7577 U",
            Self::Pms7578C => "Pantone 7578 C",
            Self::Pms7578U => "Pantone 7578 U",
            Self::Pms7579C => "Pantone 7579 C",
            Self::Pms7579U => "Pantone 7579 U",
            Self::Pms7580C => "Pantone 7580 C",
            Self::Pms7580U => "Pantone 7580 U",
            Self::Pms7581C => "Pantone 7581 C",
            Self::Pms7581U => "Pantone 7581 U",
            Self::Pms7582C => "Pantone 7582 C",
            Self::Pms7582U => "Pantone 7582 U",
            Self::Pms7583C => "Pantone 7583 C",
            Self::Pms7583U => "Pantone 7583 U",
            Self::Pms7584C => "Pantone 7584 C",
            Self::Pms7584U => "Pantone 7584 U",
            Self::Pms7585C => "Pantone 7585 C",
            Self::Pms7585U => "Pantone 7585 U",
            Self::Pms7586C => "Pantone 7586 C",
            Self::Pms7586U => "Pantone 7586 U",
            Self::Pms7587C => "Pantone 7587 C",
            Self::Pms7587U => "Pantone 7587 U",
            Self::Pms7588C => "Pantone 7588 C",
            Self::Pms7588U => "Pantone 7588 U",
            Self::Pms7589C => "Pantone 7589 C",
            Self::Pms7589U => "Pantone 7589 U",
            Self::Pms7590C => "Pantone 7590 C",
            Self::Pms7590U => "Pantone 7590 U",
            Self::Pms7591C => "Pantone 7591 C",
            Self::Pms7591U => "Pantone 7591 U",
            Self::Pms7592C => "Pantone 7592 C",
            Self::Pms7592U => "Pantone 7592 U",
            Self::Pms7593C => "Pantone 7593 C",
            Self::Pms7593U => "Pantone 7593 U",
            Self::Pms7594C => "Pantone 7594 C",
            Self::Pms7594U => "Pantone 7594 U",
            Self::Pms7595C => "Pantone 7595 C",
            Self::Pms7595U => "Pantone 7595 U",
            Self::Pms7596C => "Pantone 7596 C",
            Self::Pms7596U => "Pantone 7596 U",
            Self::Pms7597C => "Pantone 7597 C",
            Self::Pms7597U => "Pantone 7597 U",
            Self::Pms7598C => "Pantone 7598 C",
            Self::Pms7598U => "Pantone 7598 U",
            Self::Pms7599C => "Pantone 7599 C",
            Self::Pms7599U => "Pantone 7599 U",
            Self::Pms7600C => "Pantone 7600 C",
            Self::Pms7600U => "Pantone 7600 U",
            Self::Pms7601C => "Pantone 7601 C",
            Self::Pms7601U => "Pantone 7601 U",
            Self::Pms7602C => "Pantone 7602 C",
            Self::Pms7602U => "Pantone 7602 U",
            Self::Pms7603C => "Pantone 7603 C",
            Self::Pms7603U => "Pantone 7603 U",
            Self::Pms7604C => "Pantone 7604 C",
            Self::Pms7604U => "Pantone 7604 U",
            Self::Pms7605C => "Pantone 7605 C",
            Self::Pms7605U => "Pantone 7605 U",
            Self::Pms7606C => "Pantone 7606 C",
            Self::Pms7606U => "Pantone 7606 U",
            Self::Pms7607C => "Pantone 7607 C",
            Self::Pms7607U => "Pantone 7607 U",
            Self::Pms7608C => "Pantone 7608 C",
            Self::Pms7608U => "Pantone 7608 U",
            Self::Pms7609C => "Pantone 7609 C",
            Self::Pms7609U => "Pantone 7609 U",
            Self::Pms7610C => "Pantone 7610 C",
            Self::Pms7610U => "Pantone 7610 U",
            Self::Pms7611C => "Pantone 7611 C",
            Self::Pms7611U => "Pantone 7611 U",
            Self::Pms7612C => "Pantone 7612 C",
            Self::Pms7612U => "Pantone 7612 U",
            Self::Pms7613C => "Pantone 7613 C",
            Self::Pms7613U => "Pantone 7613 U",
            Self::Pms7614C => "Pantone 7614 C",
            Self::Pms7614U => "Pantone 7614 U",
            Self::Pms7615C => "Pantone 7615 C",
            Self::Pms7615U => "Pantone 7615 U",
            Self::Pms7616C => "Pantone 7616 C",
            Self::Pms7616U => "Pantone 7616 U",
            Self::Pms7617C => "Pantone 7617 C",
            Self::Pms7617U => "Pantone 7617 U",
            Self::Pms7618C => "Pantone 7618 C",
            Self::Pms7618U => "Pantone 7618 U",
            Self::Pms7619C => "Pantone 7619 C",
            Self::Pms7619U => "Pantone 7619 U",
            Self::Pms7620C => "Pantone 7620 C",
            Self::Pms7620U => "Pantone 7620 U",
            Self::Pms7621C => "Pantone 7621 C",
            Self::Pms7621U => "Pantone 7621 U",
            Self::Pms7622C => "Pantone 7622 C",
            Self::Pms7622U => "Pantone 7622 U",
            Self::Pms7623C => "Pantone 7623 C",
            Self::Pms7623U => "Pantone 7623 U",
            Self::Pms7624C => "Pantone 7624 C",
            Self::Pms7624U => "Pantone 7624 U",
            Self::Pms7625C => "Pantone 7625 C",
            Self::Pms7625U => "Pantone 7625 U",
            Self::Pms7626C => "Pantone 7626 C",
            Self::Pms7626U => "Pantone 7626 U",
            Self::Pms7627C => "Pantone 7627 C",
            Self::Pms7627U => "Pantone 7627 U",
            Self::Pms7628C => "Pantone 7628 C",
            Self::Pms7628U => "Pantone 7628 U",
            Self::Pms7629C => "Pantone 7629 C",
            Self::Pms7629U => "Pantone 7629 U",
            Self::Pms7630C => "Pantone 7630 C",
            Self::Pms7630U => "Pantone 7630 U",
            Self::Pms7631C => "Pantone 7631 C",
            Self::Pms7631U => "Pantone 7631 U",
            Self::Pms7632C => "Pantone 7632 C",
            Self::Pms7632U => "Pantone 7632 U",
            Self::Pms7633C => "Pantone 7633 C",
            Self::Pms7633U => "Pantone 7633 U",
            Self::Pms7634C => "Pantone 7634 C",
            Self::Pms7634U => "Pantone 7634 U",
            Self::Pms7635C => "Pantone 7635 C",
            Self::Pms7635U => "Pantone 7635 U",
            Self::Pms7636C => "Pantone 7636 C",
            Self::Pms7636U => "Pantone 7636 U",
            Self::Pms7637C => "Pantone 7637 C",
            Self::Pms7637U => "Pantone 7637 U",
            Self::Pms7638C => "Pantone 7638 C",
            Self::Pms7638U => "Pantone 7638 U",
            Self::Pms7639C => "Pantone 7639 C",
            Self::Pms7639U => "Pantone 7639 U",
            Self::Pms7640C => "Pantone 7640 C",
            Self::Pms7640U => "Pantone 7640 U",
            Self::Pms7641C => "Pantone 7641 C",
            Self::Pms7641U => "Pantone 7641 U",
            Self::Pms7642C => "Pantone 7642 C",
            Self::Pms7642U => "Pantone 7642 U",
            Self::Pms7643C => "Pantone 7643 C",
            Self::Pms7643U => "Pantone 7643 U",
            Self::Pms7644C => "Pantone 7644 C",
            Self::Pms7644U => "Pantone 7644 U",
            Self::Pms7645C => "Pantone 7645 C",
            Self::Pms7645U => "Pantone 7645 U",
            Self::Pms7646C => "Pantone 7646 C",
            Self::Pms7646U => "Pantone 7646 U",
            Self::Pms7647C => "Pantone 7647 C",
            Self::Pms7647U => "Pantone 7647 U",
            Self::Pms7648C => "Pantone 7648 C",
            Self::Pms7648U => "Pantone 7648 U",
            Self::Pms7649C => "Pantone 7649 C",
            Self::Pms7649U => "Pantone 7649 U",
            Self::Pms7650C => "Pantone 7650 C",
            Self::Pms7650U => "Pantone 7650 U",
            Self::Pms7651C => "Pantone 7651 C",
            Self::Pms7651U => "Pantone 7651 U",
            Self::Pms7652C => "Pantone 7652 C",
            Self::Pms7652U => "Pantone 7652 U",
            Self::Pms7653C => "Pantone 7653 C",
            Self::Pms7653U => "Pantone 7653 U",
            Self::Pms7654C => "Pantone 7654 C",
            Self::Pms7654U => "Pantone 7654 U",
            Self::Pms7655C => "Pantone 7655 C",
            Self::Pms7655U => "Pantone 7655 U",
            Self::Pms7656C => "Pantone 7656 C",
            Self::Pms7656U => "Pantone 7656 U",
            Self::Pms7657C => "Pantone 7657 C",
            Self::Pms7657U => "Pantone 7657 U",
            Self::Pms7658C => "Pantone 7658 C",
            Self::Pms7658U => "Pantone 7658 U",
            Self::Pms7659C => "Pantone 7659 C",
            Self::Pms7659U => "Pantone 7659 U",
            Self::Pms7660C => "Pantone 7660 C",
            Self::Pms7660U => "Pantone 7660 U",
            Self::Pms7661C => "Pantone 7661 C",
            Self::Pms7661U => "Pantone 7661 U",
            Self::Pms7662C => "Pantone 7662 C",
            Self::Pms7662U => "Pantone 7662 U",
            Self::Pms7663C => "Pantone 7663 C",
            Self::Pms7663U => "Pantone 7663 U",
            Self::Pms7664C => "Pantone 7664 C",
            Self::Pms7664U => "Pantone 7664 U",
            Self::Pms7665C => "Pantone 7665 C",
            Self::Pms7665U => "Pantone 7665 U",
            Self::Pms7666C => "Pantone 7666 C",
            Self::Pms7666U => "Pantone 7666 U",
            Self::Pms7667C => "Pantone 7667 C",
            Self::Pms7667U => "Pantone 7667 U",
            Self::Pms7668C => "Pantone 7668 C",
            Self::Pms7668U => "Pantone 7668 U",
            Self::Pms7669C => "Pantone 7669 C",
            Self::Pms7669U => "Pantone 7669 U",
            Self::Pms7670C => "Pantone 7670 C",
            Self::Pms7670U => "Pantone 7670 U",
            Self::Pms7671C => "Pantone 7671 C",
            Self::Pms7671U => "Pantone 7671 U",
            Self::Pms7672C => "Pantone 7672 C",
            Self::Pms7672U => "Pantone 7672 U",
            Self::Pms7673C => "Pantone 7673 C",
            Self::Pms7673U => "Pantone 7673 U",
            Self::Pms7674C => "Pantone 7674 C",
            Self::Pms7674U => "Pantone 7674 U",
            Self::Pms7675C => "Pantone 7675 C",
            Self::Pms7675U => "Pantone 7675 U",
            Self::Pms7676C => "Pantone 7676 C",
            Self::Pms7676U => "Pantone 7676 U",
            Self::Pms7677C => "Pantone 7677 C",
            Self::Pms7677U => "Pantone 7677 U",
            Self::Pms7678C => "Pantone 7678 C",
            Self::Pms7678U => "Pantone 7678 U",
            Self::Pms7679C => "Pantone 7679 C",
            Self::Pms7679U => "Pantone 7679 U",
            Self::Pms7680C => "Pantone 7680 C",
            Self::Pms7680U => "Pantone 7680 U",
            Self::Pms7681C => "Pantone 7681 C",
            Self::Pms7681U => "Pantone 7681 U",
            Self::Pms7682C => "Pantone 7682 C",
            Self::Pms7682U => "Pantone 7682 U",
            Self::Pms7683C => "Pantone 7683 C",
            Self::Pms7683U => "Pantone 7683 U",
            Self::Pms7684C => "Pantone 7684 C",
            Self::Pms7684U => "Pantone 7684 U",
            Self::Pms7685C => "Pantone 7685 C",
            Self::Pms7685U => "Pantone 7685 U",
            Self::Pms7686C => "Pantone 7686 C",
            Self::Pms7686U => "Pantone 7686 U",
            Self::Pms7687C => "Pantone 7687 C",
            Self::Pms7687U => "Pantone 7687 U",
            Self::Pms7688C => "Pantone 7688 C",
            Self::Pms7688U => "Pantone 7688 U",
            Self::Pms7689C => "Pantone 7689 C",
            Self::Pms7689U => "Pantone 7689 U",
            Self::Pms7690C => "Pantone 7690 C",
            Self::Pms7690U => "Pantone 7690 U",
            Self::Pms7691C => "Pantone 7691 C",
            Self::Pms7691U => "Pantone 7691 U",
            Self::Pms7692C => "Pantone 7692 C",
            Self::Pms7692U => "Pantone 7692 U",
            Self::Pms7693C => "Pantone 7693 C",
            Self::Pms7693U => "Pantone 7693 U",
            Self::Pms7694C => "Pantone 7694 C",
            Self::Pms7694U => "Pantone 7694 U",
            Self::Pms7695C => "Pantone 7695 C",
            Self::Pms7695U => "Pantone 7695 U",
            Self::Pms7696C => "Pantone 7696 C",
            Self::Pms7696U => "Pantone 7696 U",
            Self::Pms7697C => "Pantone 7697 C",
            Self::Pms7697U => "Pantone 7697 U",
            Self::Pms7698C => "Pantone 7698 C",
            Self::Pms7698U => "Pantone 7698 U",
            Self::Pms7699C => "Pantone 7699 C",
            Self::Pms7699U => "Pantone 7699 U",
            Self::Pms7700C => "Pantone 7700 C",
            Self::Pms7700U => "Pantone 7700 U",
            Self::Pms7701C => "Pantone 7701 C",
            Self::Pms7701U => "Pantone 7701 U",
            Self::Pms7702C => "Pantone 7702 C",
            Self::Pms7702U => "Pantone 7702 U",
            Self::Pms7703C => "Pantone 7703 C",
            Self::Pms7703U => "Pantone 7703 U",
            Self::Pms7704C => "Pantone 7704 C",
            Self::Pms7704U => "Pantone 7704 U",
            Self::Pms7705C => "Pantone 7705 C",
            Self::Pms7705U => "Pantone 7705 U",
            Self::Pms7706C => "Pantone 7706 C",
            Self::Pms7706U => "Pantone 7706 U",
            Self::Pms7707C => "Pantone 7707 C",
            Self::Pms7707U => "Pantone 7707 U",
            Self::Pms7708C => "Pantone 7708 C",
            Self::Pms7708U => "Pantone 7708 U",
            Self::Pms7709C => "Pantone 7709 C",
            Self::Pms7709U => "Pantone 7709 U",
            Self::Pms7710C => "Pantone 7710 C",
            Self::Pms7710U => "Pantone 7710 U",
            Self::Pms7711C => "Pantone 7711 C",
            Self::Pms7711U => "Pantone 7711 U",
            Self::Pms7712C => "Pantone 7712 C",
            Self::Pms7712U => "Pantone 7712 U",
            Self::Pms7713C => "Pantone 7713 C",
            Self::Pms7713U => "Pantone 7713 U",
            Self::Pms7714C => "Pantone 7714 C",
            Self::Pms7714U => "Pantone 7714 U",
            Self::Pms7715C => "Pantone 7715 C",
            Self::Pms7715U => "Pantone 7715 U",
            Self::Pms7716C => "Pantone 7716 C",
            Self::Pms7716U => "Pantone 7716 U",
            Self::Pms7717C => "Pantone 7717 C",
            Self::Pms7717U => "Pantone 7717 U",
            Self::Pms7718C => "Pantone 7718 C",
            Self::Pms7718U => "Pantone 7718 U",
            Self::Pms7719C => "Pantone 7719 C",
            Self::Pms7719U => "Pantone 7719 U",
            Self::Pms7720C => "Pantone 7720 C",
            Self::Pms7720U => "Pantone 7720 U",
            Self::Pms7721C => "Pantone 7721 C",
            Self::Pms7721U => "Pantone 7721 U",
            Self::Pms7722C => "Pantone 7722 C",
            Self::Pms7722U => "Pantone 7722 U",
            Self::Pms7723C => "Pantone 7723 C",
            Self::Pms7723U => "Pantone 7723 U",
            Self::Pms7724C => "Pantone 7724 C",
            Self::Pms7724U => "Pantone 7724 U",
            Self::Pms7725C => "Pantone 7725 C",
            Self::Pms7725U => "Pantone 7725 U",
            Self::Pms7726C => "Pantone 7726 C",
            Self::Pms7726U => "Pantone 7726 U",
            Self::Pms7727C => "Pantone 7727 C",
            Self::Pms7727U => "Pantone 7727 U",
            Self::Pms7728C => "Pantone 7728 C",
            Self::Pms7728U => "Pantone 7728 U",
            Self::Pms7729C => "Pantone 7729 C",
            Self::Pms7729U => "Pantone 7729 U",
            Self::Pms7730C => "Pantone 7730 C",
            Self::Pms7730U => "Pantone 7730 U",
            Self::Pms7731C => "Pantone 7731 C",
            Self::Pms7731U => "Pantone 7731 U",
            Self::Pms7732C => "Pantone 7732 C",
            Self::Pms7732U => "Pantone 7732 U",
            Self::Pms7733C => "Pantone 7733 C",
            Self::Pms7733U => "Pantone 7733 U",
            Self::Pms7734C => "Pantone 7734 C",
            Self::Pms7734U => "Pantone 7734 U",
            Self::Pms7735C => "Pantone 7735 C",
            Self::Pms7735U => "Pantone 7735 U",
            Self::Pms7736C => "Pantone 7736 C",
            Self::Pms7736U => "Pantone 7736 U",
            Self::Pms7737C => "Pantone 7737 C",
            Self::Pms7737U => "Pantone 7737 U",
            Self::Pms7738C => "Pantone 7738 C",
            Self::Pms7738U => "Pantone 7738 U",
            Self::Pms7739C => "Pantone 7739 C",
            Self::Pms7739U => "Pantone 7739 U",
            Self::Pms7740C => "Pantone 7740 C",
            Self::Pms7740U => "Pantone 7740 U",
            Self::Pms7741C => "Pantone 7741 C",
            Self::Pms7741U => "Pantone 7741 U",
            Self::Pms7742C => "Pantone 7742 C",
            Self::Pms7742U => "Pantone 7742 U",
            Self::Pms7743C => "Pantone 7743 C",
            Self::Pms7743U => "Pantone 7743 U",
            Self::Pms7744C => "Pantone 7744 C",
            Self::Pms7744U => "Pantone 7744 U",
            Self::Pms7745C => "Pantone 7745 C",
            Self::Pms7745U => "Pantone 7745 U",
            Self::Pms7746C => "Pantone 7746 C",
            Self::Pms7746U => "Pantone 7746 U",
            Self::Pms7747C => "Pantone 7747 C",
            Self::Pms7747U => "Pantone 7747 U",
            Self::Pms7748C => "Pantone 7748 C",
            Self::Pms7748U => "Pantone 7748 U",
            Self::Pms7749C => "Pantone 7749 C",
            Self::Pms7749U => "Pantone 7749 U",
            Self::Pms7750C => "Pantone 7750 C",
            Self::Pms7750U => "Pantone 7750 U",
            Self::Pms7751C => "Pantone 7751 C",
            Self::Pms7751U => "Pantone 7751 U",
            Self::Pms7752C => "Pantone 7752 C",
            Self::Pms7752U => "Pantone 7752 U",
            Self::Pms7753C => "Pantone 7753 C",
            Self::Pms7753U => "Pantone 7753 U",
            Self::Pms7754C => "Pantone 7754 C",
            Self::Pms7754U => "Pantone 7754 U",
            Self::Pms7755C => "Pantone 7755 C",
            Self::Pms7755U => "Pantone 7755 U",
            Self::Pms7756C => "Pantone 7756 C",
            Self::Pms7756U => "Pantone 7756 U",
            Self::Pms7757C => "Pantone 7757 C",
            Self::Pms7757U => "Pantone 7757 U",
            Self::Pms7758C => "Pantone 7758 C",
            Self::Pms7758U => "Pantone 7758 U",
            Self::Pms7759C => "Pantone 7759 C",
            Self::Pms7759U => "Pantone 7759 U",
            Self::Pms7760C => "Pantone 7760 C",
            Self::Pms7760U => "Pantone 7760 U",
            Self::Pms7761C => "Pantone 7761 C",
            Self::Pms7761U => "Pantone 7761 U",
            Self::Pms7762C => "Pantone 7762 C",
            Self::Pms7762U => "Pantone 7762 U",
            Self::Pms7763C => "Pantone 7763 C",
            Self::Pms7763U => "Pantone 7763 U",
            Self::Pms7764C => "Pantone 7764 C",
            Self::Pms7764U => "Pantone 7764 U",
            Self::Pms7765C => "Pantone 7765 C",
            Self::Pms7765U => "Pantone 7765 U",
            Self::Pms7766C => "Pantone 7766 C",
            Self::Pms7766U => "Pantone 7766 U",
            Self::Pms7767C => "Pantone 7767 C",
            Self::Pms7767U => "Pantone 7767 U",
            Self::Pms7768C => "Pantone 7768 C",
            Self::Pms7768U => "Pantone 7768 U",
            Self::Pms7769C => "Pantone 7769 C",
            Self::Pms7769U => "Pantone 7769 U",
            Self::Pms7770C => "Pantone 7770 C",
            Self::Pms7770U => "Pantone 7770 U",
            Self::Pms7771C => "Pantone 7771 C",
            Self::Pms7771U => "Pantone 7771 U",
            Self::Pms801C => "Pantone 801 C",
            Self::Pms801U => "Pantone 801 U",
            Self::Pms802C => "Pantone 802 C",
            Self::Pms802U => "Pantone 802 U",
            Self::Pms803C => "Pantone 803 C",
            Self::Pms803U => "Pantone 803 U",
            Self::Pms804C => "Pantone 804 C",
            Self::Pms804U => "Pantone 804 U",
            Self::Pms805C => "Pantone 805 C",
            Self::Pms805U => "Pantone 805 U",
            Self::Pms806C => "Pantone 806 C",
            Self::Pms806U => "Pantone 806 U",
            Self::Pms807C => "Pantone 807 C",
            Self::Pms807U => "Pantone 807 U",
            Self::Pms871C => "Pantone 871 C",
            Self::Pms871U => "Pantone 871 U",
            Self::Pms872C => "Pantone 872 C",
            Self::Pms872U => "Pantone 872 U",
            Self::Pms873C => "Pantone 873 C",
            Self::Pms873U => "Pantone 873 U",
            Self::Pms874C => "Pantone 874 C",
            Self::Pms874U => "Pantone 874 U",
            Self::Pms875C => "Pantone 875 C",
            Self::Pms875U => "Pantone 875 U",
            Self::Pms876C => "Pantone 876 C",
            Self::Pms876U => "Pantone 876 U",
            Self::Pms877C => "Pantone 877 C",
            Self::Pms877U => "Pantone 877 U",
            Self::Black0961C => "Pantone Black 0961 C",
            Self::Black0961U => "Pantone Black 0961 U",
            Self::Black2C => "Pantone Black 2 C",
            Self::Black2U => "Pantone Black 2 U",
            Self::Black3C => "Pantone Black 3 C",
            Self::Black3U => "Pantone Black 3 U",
            Self::Black4C => "Pantone Black 4 C",
            Self::Black4U => "Pantone Black 4 U",
            Self::Black5C => "Pantone Black 5 C",
            Self::Black5U => "Pantone Black 5 U",
            Self::Black6C => "Pantone Black 6 C",
            Self::Black6U => "Pantone Black 6 U",
            Self::Black7C => "Pantone Black 7 C",
            Self::Black7U => "Pantone Black 7 U",
            Self::BlackC => "Pantone Black C",
            Self::BlackU => "Pantone Black U",
            Self::Blue072C => "Pantone Blue 072 C",
            Self::Blue072U => "Pantone Blue 072 U",
            Self::Blue0821C => "Pantone Blue 0821 C",
            Self::Blue0821U => "Pantone Blue 0821 U",
            Self::BrightGreenC => "Pantone Bright Green C",
            Self::BrightOrangeC => "Pantone Bright Orange C",
            Self::BrightRedC => "Pantone Bright Red C",
            Self::BrightRedU => "Pantone Bright Red U",
            Self::CoolGray1C => "Pantone Cool Gray 1 C",
            Self::CoolGray1U => "Pantone Cool Gray 1 U",
            Self::CoolGray10C => "Pantone Cool Gray 10 C",
            Self::CoolGray10U => "Pantone Cool Gray 10 U",
            Self::CoolGray11C => "Pantone Cool Gray 11 C",
            Self::CoolGray11U => "Pantone Cool Gray 11 U",
            Self::CoolGray2C => "Pantone Cool Gray 2 C",
            Self::CoolGray2U => "Pantone Cool Gray 2 U",
            Self::CoolGray3C => "Pantone Cool Gray 3 C",
            Self::CoolGray3U => "Pantone Cool Gray 3 U",
            Self::CoolGray4C => "Pantone Cool Gray 4 C",
            Self::CoolGray4U => "Pantone Cool Gray 4 U",
            Self::CoolGray5C => "Pantone Cool Gray 5 C",
            Self::CoolGray5U => "Pantone Cool Gray 5 U",
            Self::CoolGray6C => "Pantone Cool Gray 6 C",
            Self::CoolGray6U => "Pantone Cool Gray 6 U",
            Self::CoolGray7C => "Pantone Cool Gray 7 C",
            Self::CoolGray7U => "Pantone Cool Gray 7 U",
            Self::CoolGray8C => "Pantone Cool Gray 8 C",
            Self::CoolGray8U => "Pantone Cool Gray 8 U",
            Self::CoolGray9C => "Pantone Cool Gray 9 C",
            Self::CoolGray9U => "Pantone Cool Gray 9 U",
            Self::DarkBlueC => "Pantone Dark Blue C",
            Self::DarkBlueU => "Pantone Dark Blue U",
            Self::Green0921C => "Pantone Green 0921 C",
            Self::Green0921U => "Pantone Green 0921 U",
            Self::GreenC => "Pantone Green C",
            Self::GreenU => "Pantone Green U",
            Self::Magenta0521C => "Pantone Magenta 0521 C",
            Self::Magenta0521U => "Pantone Magenta 0521 U",
            Self::MediumBlueC => "Pantone Medium Blue C",
            Self::MediumPurpleC => "Pantone Medium Purple C",
            Self::MediumPurpleU => "Pantone Medium Purple U",
            Self::MediumYellowC => "Pantone Medium Yellow C",
            Self::NeutralBlackC => "Pantone Neutral Black C",
            Self::Orange021C => "Pantone Orange 021 C",
            Self::Orange021U => "Pantone Orange 021 U",
            Self::PinkC => "Pantone Pink C",
            Self::PinkU => "Pantone Pink U",
            Self::ProcessBlueC => "Pantone Process Blue C",
            Self::ProcessBlueU => "Pantone Process Blue U",
            Self::PurpleC => "Pantone Purple C",
            Self::PurpleU => "Pantone Purple U",
            Self::Red032C => "Pantone Red 032 C",
            Self::Red032U => "Pantone Red 032 U",
            Self::Red0331C => "Pantone Red 0331 C",
            Self::Red0331U => "Pantone Red 0331 U",
            Self::ReflexBlueC => "Pantone Reflex Blue C",
            Self::ReflexBlueU => "Pantone Reflex Blue U",
            Self::RhodamineRedC => "Pantone Rhodamine Red C",
            Self::RhodamineRedU => "Pantone Rhodamine Red U",
            Self::RubineRedC => "Pantone Rubine Red C",
            Self::RubineRedU => "Pantone Rubine Red U",
            Self::StrongRedC => "Pantone Strong Red C",
            Self::Violet0631C => "Pantone Violet 0631 C",
            Self::Violet0631U => "Pantone Violet 0631 U",
            Self::VioletC => "Pantone Violet C",
            Self::VioletU => "Pantone Violet U",
            Self::WarmGray1C => "Pantone Warm Gray 1 C",
            Self::WarmGray1U => "Pantone Warm Gray 1 U",
            Self::WarmGray10C => "Pantone Warm Gray 10 C",
            Self::WarmGray10U => "Pantone Warm Gray 10 U",
            Self::WarmGray11C => "Pantone Warm Gray 11 C",
            Self::WarmGray11U => "Pantone Warm Gray 11 U",
            Self::WarmGray2C => "Pantone Warm Gray 2 C",
            Self::WarmGray2U => "Pantone Warm Gray 2 U",
            Self::WarmGray3C => "Pantone Warm Gray 3 C",
            Self::WarmGray3U => "Pantone Warm Gray 3 U",
            Self::WarmGray4C => "Pantone Warm Gray 4 C",
            Self::WarmGray4U => "Pantone Warm Gray 4 U",
            Self::WarmGray5C => "Pantone Warm Gray 5 C",
            Self::WarmGray5U => "Pantone Warm Gray 5 U",
            Self::WarmGray6C => "Pantone Warm Gray 6 C",
            Self::WarmGray6U => "Pantone Warm Gray 6 U",
            Self::WarmGray7C => "Pantone Warm Gray 7 C",
            Self::WarmGray7U => "Pantone Warm Gray 7 U",
            Self::WarmGray8C => "Pantone Warm Gray 8 C",
            Self::WarmGray8U => "Pantone Warm Gray 8 U",
            Self::WarmGray9C => "Pantone Warm Gray 9 C",
            Self::WarmGray9U => "Pantone Warm Gray 9 U",
            Self::WarmRedC => "Pantone Warm Red C",
            Self::WarmRedU => "Pantone Warm Red U",
            Self::WhiteC => "Pantone White C",
            Self::Yellow012C => "Pantone Yellow 012 C",
            Self::Yellow012U => "Pantone Yellow 012 U",
            Self::Yellow0131C => "Pantone Yellow 0131 C",
            Self::Yellow0131U => "Pantone Yellow 0131 U",
            Self::YellowC => "Pantone Yellow C",
            Self::YellowU => "Pantone Yellow U",
        }
    }

    /// The color as opaque byte components.
    pub const fn to_color(&self) -> Color<u8> {
        Color::from_rgb_u32(self.packed_rgb())
    }

    /// The nearest Pantone color to `color` in the sRGB space -
    /// squared Euclidean distance over the RGB channels, alpha
    /// ignored.
    pub fn nearest(color: &Color<u8>) -> Self {
        // `ALL` is a non-empty const table, so the minimum always exists.
        Self::ALL
            .iter()
            .min_by_key(|candidate| color.squared_rgb_distance(&candidate.to_color()))
            .copied()
            .unwrap_or(Self::Pms100C)
    }

    /// The published CMYK build of the color, if the reference data
    /// covers it.
    ///
    /// The values are community reference values for the coated
    /// guide - metallic and newer colors have no published build and
    /// return [`None`], as do all `U` variants (uncoated CMYK builds
    /// differ and are not covered by the dataset). For a naive
    /// conversion of any color see `Cmyk::from(Color<u8>)`.
    pub const fn to_cmyk(&self) -> Option<Cmyk> {
        match self {
            Self::Pms100C => Some(Cmyk::new(0, 0, 51, 0)),
            Self::Pms101C => Some(Cmyk::new(0, 0, 79, 0)),
            Self::Pms102C => Some(Cmyk::new(0, 1, 95, 0)),
            Self::Pms103C => Some(Cmyk::new(0, 3, 100, 18)),
            Self::Pms104C => Some(Cmyk::new(0, 3, 100, 30)),
            Self::Pms105C => Some(Cmyk::new(0, 3, 100, 50)),
            Self::Pms106C => Some(Cmyk::new(0, 2, 69, 0)),
            Self::Pms107C => Some(Cmyk::new(0, 4, 79, 0)),
            Self::Pms108C => Some(Cmyk::new(0, 6, 95, 0)),
            Self::Pms109C => Some(Cmyk::new(0, 10, 100, 0)),
            Self::Pms110C => Some(Cmyk::new(0, 12, 100, 7)),
            Self::Pms111C => Some(Cmyk::new(0, 11, 100, 27)),
            Self::Pms112C => Some(Cmyk::new(0, 10, 100, 38)),
            Self::Pms113C => Some(Cmyk::new(0, 7, 66, 0)),
            Self::Pms114C => Some(Cmyk::new(0, 8, 73, 0)),
            Self::Pms115C => Some(Cmyk::new(0, 9, 80, 0)),
            Self::Pms116C => Some(Cmyk::new(0, 16, 100, 0)),
            Self::Pms117C => Some(Cmyk::new(0, 18, 100, 15)),
            Self::Pms118C => Some(Cmyk::new(0, 18, 100, 27)),
            Self::Pms119C => Some(Cmyk::new(0, 12, 100, 49)),
            Self::Pms120C => Some(Cmyk::new(0, 9, 58, 0)),
            Self::Pms1205C => Some(Cmyk::new(0, 5, 31, 0)),
            Self::Pms121C => Some(Cmyk::new(0, 11, 69, 0)),
            Self::Pms1215C => Some(Cmyk::new(0, 9, 45, 0)),
            Self::Pms122C => Some(Cmyk::new(0, 17, 80, 0)),
            Self::Pms1225C => Some(Cmyk::new(0, 17, 62, 0)),
            Self::Pms123C => Some(Cmyk::new(0, 24, 94, 0)),
            Self::Pms1235C => Some(Cmyk::new(0, 29, 91, 0)),
            Self::Pms124C => Some(Cmyk::new(0, 28, 100, 6)),
            Self::Pms1245C => Some(Cmyk::new(0, 28, 100, 18)),
            Self::Pms125C => Some(Cmyk::new(0, 26, 100, 26)),
            Self::Pms1255C => Some(Cmyk::new(0, 27, 100, 34)),
            Self::Pms126C => Some(Cmyk::new(0, 25, 100, 37)),
            Self::Pms1265C => Some(Cmyk::new(0, 27, 100, 51)),
            Self::Pms127C => Some(Cmyk::new(0, 7, 50, 0)),
            Self::Pms128C => Some(Cmyk::new(0, 11, 65, 0)),
            Self::Pms129C => Some(Cmyk::new(0, 16, 77, 0)),
            Self::Pms130C => Some(Cmyk::new(0, 30, 100, 0)),
            Self::Pms131C => Some(Cmyk::new(0, 32, 100, 9)),
            Self::Pms132C => Some(Cmyk::new(0, 28, 100, 30)),
            Self::Pms133C => Some(Cmyk::new(0, 20, 100, 56)),
            Self::Pms134C => Some(Cmyk::new(0, 11, 45, 0)),
            Self::Pms1345C => Some(Cmyk::new(0, 14, 47, 0)),
            Self::Pms135C => Some(Cmyk::new(0, 19, 60, 0)),
            Self::Pms1355C => Some(Cmyk::new(0, 20, 56, 0)),
            Self::Pms136C => Some(Cmyk::new(0, 27, 76, 0)),
            Self::Pms1365C => Some(Cmyk::new(0, 29, 72, 0)),
            Self::Pms137C => Some(Cmyk::new(0, 35, 90, 0)),
            Self::Pms1375C => Some(Cmyk::new(0, 40, 90, 0)),
            Self::Pms138C => Some(Cmyk::new(0, 42, 100, 1)),
            Self::Pms1385C => Some(Cmyk::new(0, 44, 100, 7)),
            Self::Pms139C => Some(Cmyk::new(0, 37, 100, 23)),
            Self::Pms1395C => Some(Cmyk::new(0, 41, 100, 37)),
            Self::Pms140C => Some(Cmyk::new(0, 27, 100, 54)),
            Self::Pms1405C => Some(Cmyk::new(0, 36, 100, 63)),
            Self::Pms141C => Some(Cmyk::new(0, 19, 51, 0)),
            Self::Pms142C => Some(Cmyk::new(0, 28, 76, 0)),
            Self::Pms143C => Some(Cmyk::new(0, 35, 85, 0)),
            Self::Pms144C => Some(Cmyk::new(0, 48, 100, 0)),
            Self::Pms145C => Some(Cmyk::new(0, 47, 100, 8)),
            Self::Pms146C => Some(Cmyk::new(0, 43, 100, 33)),
            Self::Pms147C => Some(Cmyk::new(0, 28, 100, 56)),
            Self::Pms148C => Some(Cmyk::new(0, 16, 37, 0)),
            Self::Pms1485C => Some(Cmyk::new(0, 27, 54, 0)),
            Self::Pms149C => Some(Cmyk::new(0, 23, 47, 0)),
            Self::Pms1495C => Some(Cmyk::new(0, 33, 67, 0)),
            Self::Pms150C => Some(Cmyk::new(0, 35, 70, 0)),
            Self::Pms1505C => Some(Cmyk::new(0, 42, 77, 0)),
            Self::Pms151C => Some(Cmyk::new(0, 48, 95, 0)),
            Self::Pms152C => Some(Cmyk::new(0, 51, 100, 1)),
            Self::Pms1525C => Some(Cmyk::new(0, 58, 100, 10)),
            Self::Pms153C => Some(Cmyk::new(0, 46, 100, 18)),
            Self::Pms1535C => Some(Cmyk::new(0, 53, 100, 38)),
            Self::Pms154C => Some(Cmyk::new(0, 46, 100, 34)),
            Self::Pms1545C => Some(Cmyk::new(0, 53, 100, 72)),
            Self::Pms155C => Some(Cmyk::new(0, 12, 28, 0)),
            Self::Pms1555C => Some(Cmyk::new(0, 22, 34, 0)),
            Self::Pms156C => Some(Cmyk::new(0, 22, 42, 0)),
            Self::Pms1565C => Some(Cmyk::new(0, 34, 49, 0)),
            Self::Pms157C => Some(Cmyk::new(0, 43, 70, 0)),
            Self::Pms1575C => Some(Cmyk::new(0, 45, 72, 0)),
            Self::Pms158C => Some(Cmyk::new(0, 61, 97, 0)),
            Self::Pms1585C => Some(Cmyk::new(0, 56, 90, 0)),
            Self::Pms159C => Some(Cmyk::new(0, 66, 100, 7)),
            Self::Pms1595C => Some(Cmyk::new(0, 59, 100, 5)),
            Self::Pms160C => Some(Cmyk::new(0, 62, 100, 32)),
            Self::Pms1605C => Some(Cmyk::new(0, 56, 100, 30)),
            Self::Pms161C => Some(Cmyk::new(0, 52, 100, 64)),
            Self::Pms1615C => Some(Cmyk::new(0, 56, 100, 43)),
            Self::Pms162C => Some(Cmyk::new(0, 15, 22, 0)),
            Self::Pms1625C => Some(Cmyk::new(0, 31, 37, 0)),
            Self::Pms163C => Some(Cmyk::new(0, 31, 44, 0)),
            Self::Pms1635C => Some(Cmyk::new(0, 39, 48, 0)),
            Self::Pms164C => Some(Cmyk::new(0, 46, 73, 0)),
            Self::Pms1645C => Some(Cmyk::new(0, 49, 66, 0)),
            Self::Pms165C => Some(Cmyk::new(0, 59, 96, 0)),
            Self::Pms1655C => Some(Cmyk::new(0, 63, 91, 0)),
            Self::Pms166C => Some(Cmyk::new(0, 64, 100, 0)),
            Self::Pms1665C => Some(Cmyk::new(0, 68, 100, 0)),
            Self::Pms167C => Some(Cmyk::new(0, 60, 100, 17)),
            Self::Pms1675C => Some(Cmyk::new(0, 67, 100, 28)),
            Self::Pms168C => Some(Cmyk::new(0, 57, 100, 59)),
            Self::Pms1685C => Some(Cmyk::new(0, 68, 100, 44)),
            Self::Pms169C => Some(Cmyk::new(0, 20, 20, 0)),
            Self::Pms170C => Some(Cmyk::new(0, 40, 44, 0)),
            Self::Pms171C => Some(Cmyk::new(0, 53, 68, 0)),
            Self::Pms172C => Some(Cmyk::new(0, 66, 88, 0)),
            Self::Pms173C => Some(Cmyk::new(0, 69, 100, 4)),
            Self::Pms174C => Some(Cmyk::new(0, 70, 100, 36)),
            Self::Pms175C => Some(Cmyk::new(0, 65, 100, 60)),
            Self::Pms176C => Some(Cmyk::new(0, 25, 18, 0)),
            Self::Pms1765C => Some(Cmyk::new(0, 38, 21, 0)),
            Self::Pms1767C => Some(Cmyk::new(0, 27, 12, 0)),
            Self::Pms177C => Some(Cmyk::new(0, 45, 40, 0)),
            Self::Pms1775C => Some(Cmyk::new(0, 47, 29, 0)),
            Self::Pms1777C => Some(Cmyk::new(0, 58, 36, 0)),
            Self::Pms178C => Some(Cmyk::new(0, 59, 56, 0)),
            Self::Pms1785C => Some(Cmyk::new(0, 67, 50, 0)),
            Self::Pms1787C => Some(Cmyk::new(0, 76, 60, 0)),
            Self::Pms1788C => Some(Cmyk::new(0, 84, 88, 0)),
            Self::Pms179C => Some(Cmyk::new(0, 79, 100, 0)),
            Self::Pms1795C => Some(Cmyk::new(0, 94, 100, 0)),
            Self::Pms1797C => Some(Cmyk::new(0, 100, 99, 4)),
            Self::Pms180C => Some(Cmyk::new(0, 79, 100, 11)),
            Self::Pms1805C => Some(Cmyk::new(0, 91, 100, 23)),
            Self::Pms1807C => Some(Cmyk::new(0, 100, 96, 28)),
            Self::Pms181C => Some(Cmyk::new(0, 74, 100, 47)),
            Self::Pms1815C => Some(Cmyk::new(0, 90, 100, 51)),
            Self::Pms1817C => Some(Cmyk::new(0, 90, 100, 66)),
            Self::Pms182C => Some(Cmyk::new(0, 26, 10, 0)),
            Self::Pms183C => Some(Cmyk::new(0, 46, 21, 0)),
            Self::Pms184C => Some(Cmyk::new(0, 68, 41, 0)),
            Self::Pms185C => Some(Cmyk::new(0, 91, 76, 0)),
            Self::Pms186C => Some(Cmyk::new(0, 100, 81, 4)),
            Self::Pms187C => Some(Cmyk::new(0, 100, 79, 20)),
            Self::Pms188C => Some(Cmyk::new(0, 97, 100, 50)),
            Self::Pms189C => Some(Cmyk::new(0, 37, 10, 0)),
            Self::Pms1895C => Some(Cmyk::new(0, 28, 7, 0)),
            Self::Pms190C => Some(Cmyk::new(0, 55, 22, 0)),
            Self::Pms1905C => Some(Cmyk::new(0, 41, 9, 0)),
            Self::Pms191C => Some(Cmyk::new(0, 76, 38, 0)),
            Self::Pms1915C => Some(Cmyk::new(0, 71, 20, 0)),
            Self::Pms192C => Some(Cmyk::new(0, 100, 68, 0)),
            Self::Pms1925C => Some(Cmyk::new(0, 100, 55, 0)),
            Self::Pms193C => Some(Cmyk::new(0, 100, 66, 13)),
            Self::Pms1935C => Some(Cmyk::new(0, 100, 57, 5)),
            Self::Pms194C => Some(Cmyk::new(0, 100, 64, 33)),
            Self::Pms1945C => Some(Cmyk::new(0, 100, 56, 19)),
            Self::Pms195C => Some(Cmyk::new(0, 100, 60, 55)),
            Self::Pms1955C => Some(Cmyk::new(0, 100, 60, 37)),
            Self::Pms196C => Some(Cmyk::new(0, 25, 4, 0)),
            Self::Pms197C => Some(Cmyk::new(0, 45, 10, 0)),
            Self::Pms198C => Some(Cmyk::new(0, 78, 33, 0)),
            Self::Pms199C => Some(Cmyk::new(0, 100, 62, 0)),
            Self::Pms200C => Some(Cmyk::new(0, 100, 63, 12)),
            Self::Pms201C => Some(Cmyk::new(0, 100, 63, 29)),
            Self::Pms202C => Some(Cmyk::new(0, 100, 61, 43)),
            Self::Pms203C => Some(Cmyk::new(0, 34, 3, 0)),
            Self::Pms204C => Some(Cmyk::new(0, 58, 3, 0)),
            Self::Pms205C => Some(Cmyk::new(0, 84, 9, 0)),
            Self::Pms206C => Some(Cmyk::new(0, 100, 38, 3)),
            Self::Pms207C => Some(Cmyk::new(0, 100, 43, 19)),
            Self::Pms208C => Some(Cmyk::new(0, 100, 36, 37)),
            Self::Pms209C => Some(Cmyk::new(0, 100, 34, 53)),
            Self::Pms210C => Some(Cmyk::new(0, 39, 6, 0)),
            Self::Pms211C => Some(Cmyk::new(0, 55, 8, 0)),
            Self::Pms212C => Some(Cmyk::new(0, 72, 11, 0)),
            Self::Pms213C => Some(Cmyk::new(0, 95, 27, 0)),
            Self::Pms214C => Some(Cmyk::new(0, 100, 34, 8)),
            Self::Pms215C => Some(Cmyk::new(0, 100, 35, 27)),
            Self::Pms216C => Some(Cmyk::new(0, 95, 40, 49)),
            Self::Pms217C => Some(Cmyk::new(0, 28, 0, 0)),
            Self::Pms218C => Some(Cmyk::new(2, 61, 0, 0)),
            Self::Pms219C => Some(Cmyk::new(1, 88, 0, 0)),
            Self::Pms220C => Some(Cmyk::new(0, 100, 13, 17)),
            Self::Pms221C => Some(Cmyk::new(0, 100, 15, 30)),
            Self::Pms222C => Some(Cmyk::new(0, 100, 10, 59)),
            Self::Pms223C => Some(Cmyk::new(0, 46, 0, 0)),
            Self::Pms224C => Some(Cmyk::new(1, 63, 0, 0)),
            Self::Pms225C => Some(Cmyk::new(1, 83, 0, 0)),
            Self::Pms226C => Some(Cmyk::new(0, 100, 0, 0)),
            Self::Pms227C => Some(Cmyk::new(0, 100, 7, 19)),
            Self::Pms228C => Some(Cmyk::new(0, 100, 4, 41)),
            Self::Pms229C => Some(Cmyk::new(0, 100, 15, 60)),
            Self::Pms230C => Some(Cmyk::new(0, 34, 0, 0)),
            Self::Pms231C => Some(Cmyk::new(1, 52, 0, 0)),
            Self::Pms232C => Some(Cmyk::new(3, 67, 0, 0)),
            Self::Pms233C => Some(Cmyk::new(11, 100, 0, 0)),
            Self::Pms234C => Some(Cmyk::new(6, 100, 0, 26)),
            Self::Pms235C => Some(Cmyk::new(5, 100, 0, 40)),
            Self::Pms236C => Some(Cmyk::new(1, 30, 0, 0)),
            Self::Pms2365C => Some(Cmyk::new(2, 27, 0, 0)),
            Self::Pms237C => Some(Cmyk::new(3, 49, 0, 0)),
            Self::Pms2375C => Some(Cmyk::new(10, 57, 0, 0)),
            Self::Pms238C => Some(Cmyk::new(6, 63, 0, 0)),
            Self::Pms2385C => Some(Cmyk::new(19, 79, 0, 0)),
            Self::Pms239C => Some(Cmyk::new(11, 79, 0, 0)),
            Self::Pms2395C => Some(Cmyk::new(27, 95, 0, 0)),
            Self::Pms240C => Some(Cmyk::new(18, 94, 0, 0)),
            Self::Pms2405C => Some(Cmyk::new(34, 100, 0, 0)),
            Self::Pms241C => Some(Cmyk::new(27, 100, 0, 2)),
            Self::Pms2415C => Some(Cmyk::new(33, 100, 0, 8)),
            Self::Pms242C => Some(Cmyk::new(10, 100, 0, 49)),
            Self::Pms2425C => Some(Cmyk::new(37, 100, 0, 26)),
            Self::Pms243C => Some(Cmyk::new(5, 29, 0, 0)),
            Self::Pms244C => Some(Cmyk::new(9, 38, 0, 0)),
            Self::Pms245C => Some(Cmyk::new(14, 53, 0, 0)),
            Self::Pms246C => Some(Cmyk::new(29, 90, 0, 0)),
            Self::Pms247C => Some(Cmyk::new(36, 100, 0, 0)),
            Self::Pms248C => Some(Cmyk::new(40, 100, 0, 2)),
            Self::Pms249C => Some(Cmyk::new(40, 100, 0, 28)),
            Self::Pms250C => Some(Cmyk::new(5, 18, 0, 0)),
            Self::Pms251C => Some(Cmyk::new(13, 39, 0, 0)),
            Self::Pms252C => Some(Cmyk::new(24, 56, 0, 0)),
            Self::Pms253C => Some(Cmyk::new(43, 95, 0, 0)),
            Self::Pms254C => Some(Cmyk::new(50, 100, 0, 0)),
            Self::Pms255C => Some(Cmyk::new(51, 100, 0, 25)),
            Self::Pms256C => Some(Cmyk::new(7, 20, 0, 0)),
            Self::Pms2562C => Some(Cmyk::new(19, 35, 0, 0)),
            Self::Pms2563C => Some(Cmyk::new(22, 33, 0, 0)),
            Self::Pms2567C => Some(Cmyk::new(29, 36, 0, 0)),
            Self::Pms257C => Some(Cmyk::new(14, 34, 0, 0)),
            Self::Pms2572C => Some(Cmyk::new(30, 47, 0, 0)),
            Self::Pms2573C => Some(Cmyk::new(30, 43, 0, 0)),
            Self::Pms2577C => Some(Cmyk::new(40, 45, 0, 0)),
            Self::Pms258C => Some(Cmyk::new(43, 76, 0, 0)),
            Self::Pms2582C => Some(Cmyk::new(46, 72, 0, 0)),
            Self::Pms2583C => Some(Cmyk::new(46, 63, 0, 0)),
            Self::Pms2587C => Some(Cmyk::new(59, 66, 0, 0)),
            Self::Pms259C => Some(Cmyk::new(56, 100, 0, 15)),
            Self::Pms2592C => Some(Cmyk::new(60, 90, 0, 0)),
            Self::Pms2593C => Some(Cmyk::new(61, 89, 0, 0)),
            Self::Pms2597C => Some(Cmyk::new(85, 100, 0, 0)),
            Self::Pms260C => Some(Cmyk::new(52, 100, 0, 26)),
            Self::Pms2602C => Some(Cmyk::new(63, 100, 0, 3)),
            Self::Pms2603C => Some(Cmyk::new(69, 100, 0, 2)),
            Self::Pms2607C => Some(Cmyk::new(81, 100, 0, 7)),
            Self::Pms261C => Some(Cmyk::new(48, 100, 0, 40)),
            Self::Pms2612C => Some(Cmyk::new(64, 100, 0, 14)),
            Self::Pms2613C => Some(Cmyk::new(63, 100, 0, 15)),
            Self::Pms2617C => Some(Cmyk::new(79, 100, 0, 15)),
            Self::Pms262C => Some(Cmyk::new(45, 100, 0, 55)),
            Self::Pms2622C => Some(Cmyk::new(58, 100, 0, 44)),
            Self::Pms2623C => Some(Cmyk::new(59, 100, 0, 32)),
            Self::Pms2627C => Some(Cmyk::new(77, 100, 0, 31)),
            Self::Pms263C => Some(Cmyk::new(10, 14, 0, 0)),
            Self::Pms2635C => Some(Cmyk::new(28, 27, 0, 0)),
            Self::Pms264C => Some(Cmyk::new(26, 28, 0, 0)),
            Self::Pms2645C => Some(Cmyk::new(40, 36, 0, 0)),
            Self::Pms265C => Some(Cmyk::new(54, 56, 0, 0)),
            Self::Pms2655C => Some(Cmyk::new(54, 49, 0, 0)),
            Self::Pms266C => Some(Cmyk::new(79, 90, 0, 0)),
            Self::Pms2665C => Some(Cmyk::new(62, 60, 0, 0)),
            Self::Pms267C => Some(Cmyk::new(89, 100, 0, 0)),
            Self::Pms268C => Some(Cmyk::new(82, 100, 0, 12)),
            Self::Pms2685C => Some(Cmyk::new(96, 100, 0, 10)),
            Self::Pms269C => Some(Cmyk::new(78, 100, 0, 33)),
            Self::Pms2695C => Some(Cmyk::new(91, 100, 0, 49)),
            Self::Pms270C => Some(Cmyk::new(31, 27, 0, 0)),
            Self::Pms2705C => Some(Cmyk::new(40, 30, 0, 0)),
            Self::Pms2706C => Some(Cmyk::new(19, 9, 0, 0)),
            Self::Pms2707C => Some(Cmyk::new(17, 6, 0, 0)),
            Self::Pms2708C => Some(Cmyk::new(26, 10, 0, 0)),
            Self::Pms271C => Some(Cmyk::new(43, 37, 0, 0)),
            Self::Pms2715C => Some(Cmyk::new(57, 45, 0, 0)),
            Self::Pms2716C => Some(Cmyk::new(45, 29, 0, 0)),
            Self::Pms2717C => Some(Cmyk::new(29, 12, 0, 0)),
            Self::Pms2718C => Some(Cmyk::new(67, 41, 0, 0)),
            Self::Pms272C => Some(Cmyk::new(58, 48, 0, 0)),
            Self::Pms2725C => Some(Cmyk::new(77, 68, 0, 0)),
            Self::Pms2726C => Some(Cmyk::new(79, 66, 0, 0)),
            Self::Pms2727C => Some(Cmyk::new(71, 42, 0, 0)),
            Self::Pms2728C => Some(Cmyk::new(96, 69, 0, 0)),
            Self::Pms273C => Some(Cmyk::new(100, 96, 0, 8)),
            Self::Pms2735C => Some(Cmyk::new(100, 95, 0, 0)),
            Self::Pms2736C => Some(Cmyk::new(100, 91, 0, 0)),
            Self::Pms2738C => Some(Cmyk::new(100, 87, 0, 2)),
            Self::Pms274C => Some(Cmyk::new(100, 100, 0, 28)),
            Self::Pms2745C => Some(Cmyk::new(100, 95, 0, 15)),
            Self::Pms2746C => Some(Cmyk::new(100, 92, 0, 10)),
            Self::Pms2747C => Some(Cmyk::new(100, 86, 0, 15)),
            Self::Pms2748C => Some(Cmyk::new(100, 88, 0, 15)),
            Self::Pms275C => Some(Cmyk::new(98, 100, 0, 43)),
            Self::Pms2755C => Some(Cmyk::new(100, 97, 0, 30)),
            Self::Pms2756C => Some(Cmyk::new(100, 94, 0, 29)),
            Self::Pms2757C => Some(Cmyk::new(100, 82, 0, 30)),
            Self::Pms2758C => Some(Cmyk::new(100, 80, 0, 26)),
            Self::Pms276C => Some(Cmyk::new(98, 100, 0, 55)),
            Self::Pms2765C => Some(Cmyk::new(100, 97, 0, 45)),
            Self::Pms2766C => Some(Cmyk::new(100, 94, 0, 47)),
            Self::Pms2767C => Some(Cmyk::new(100, 78, 0, 54)),
            Self::Pms2768C => Some(Cmyk::new(100, 78, 0, 44)),
            Self::Pms277C => Some(Cmyk::new(27, 7, 0, 0)),
            Self::Pms278C => Some(Cmyk::new(39, 14, 0, 0)),
            Self::Pms279C => Some(Cmyk::new(68, 34, 0, 0)),
            Self::Pms280C => Some(Cmyk::new(100, 72, 0, 18)),
            Self::Pms281C => Some(Cmyk::new(100, 72, 0, 32)),
            Self::Pms282C => Some(Cmyk::new(100, 68, 0, 54)),
            Self::Pms283C => Some(Cmyk::new(35, 9, 0, 0)),
            Self::Pms284C => Some(Cmyk::new(55, 19, 0, 0)),
            Self::Pms285C => Some(Cmyk::new(89, 43, 0, 0)),
            Self::Pms286C => Some(Cmyk::new(100, 66, 0, 2)),
            Self::Pms287C => Some(Cmyk::new(100, 68, 0, 12)),
            Self::Pms288C => Some(Cmyk::new(100, 67, 0, 23)),
            Self::Pms289C => Some(Cmyk::new(100, 66, 0, 58)),
            Self::Pms290C => Some(Cmyk::new(25, 2, 0, 0)),
            Self::Pms2905C => Some(Cmyk::new(41, 2, 0, 0)),
            Self::Pms291C => Some(Cmyk::new(33, 3, 0, 0)),
            Self::Pms2915C => Some(Cmyk::new(59, 7, 0, 0)),
            Self::Pms292C => Some(Cmyk::new(49, 11, 0, 0)),
            Self::Pms2925C => Some(Cmyk::new(85, 24, 0, 0)),
            Self::Pms293C => Some(Cmyk::new(100, 57, 0, 2)),
            Self::Pms2935C => Some(Cmyk::new(100, 46, 0, 0)),
            Self::Pms294C => Some(Cmyk::new(100, 58, 0, 21)),
            Self::Pms2945C => Some(Cmyk::new(100, 45, 0, 14)),
            Self::Pms295C => Some(Cmyk::new(100, 57, 0, 37)),
            Self::Pms2955C => Some(Cmyk::new(100, 45, 0, 37)),
            Self::Pms296C => Some(Cmyk::new(100, 46, 0, 70)),
            Self::Pms2965C => Some(Cmyk::new(100, 38, 0, 64)),
            Self::Pms297C => Some(Cmyk::new(49, 1, 0, 0)),
            Self::Pms2975C => Some(Cmyk::new(30, 0, 5, 0)),
            Self::Pms298C => Some(Cmyk::new(69, 7, 0, 0)),
            Self::Pms2985C => Some(Cmyk::new(59, 0, 6, 0)),
            Self::Pms299C => Some(Cmyk::new(85, 19, 0, 0)),
            Self::Pms2995C => Some(Cmyk::new(90, 11, 0, 0)),
            Self::Pms300C => Some(Cmyk::new(100, 44, 0, 0)),
            Self::Pms3005C => Some(Cmyk::new(100, 36, 0, 2)),
            Self::Pms301C => Some(Cmyk::new(100, 45, 0, 18)),
            Self::Pms3015C => Some(Cmyk::new(100, 33, 0, 20)),
            Self::Pms302C => Some(Cmyk::new(100, 25, 0, 50)),
            Self::Pms3025C => Some(Cmyk::new(100, 17, 0, 51)),
            Self::Pms303C => Some(Cmyk::new(100, 11, 0, 74)),
            Self::Pms3035C => Some(Cmyk::new(100, 0, 5, 72)),
            Self::Pms304C => Some(Cmyk::new(30, 0, 8, 0)),
            Self::Pms305C => Some(Cmyk::new(51, 0, 9, 0)),
            Self::Pms306C => Some(Cmyk::new(75, 0, 7, 0)),
            Self::Pms307C => Some(Cmyk::new(100, 16, 0, 27)),
            Self::Pms308C => Some(Cmyk::new(100, 5, 0, 47)),
            Self::Pms309C => Some(Cmyk::new(100, 0, 9, 72)),
            Self::Pms310C => Some(Cmyk::new(43, 0, 10, 0)),
            Self::Pms3105C => Some(Cmyk::new(43, 0, 12, 0)),
            Self::Pms311C => Some(Cmyk::new(63, 0, 12, 0)),
            Self::Pms3115C => Some(Cmyk::new(63, 0, 18, 0)),
            Self::Pms312C => Some(Cmyk::new(96, 0, 11, 0)),
            Self::Pms3125C => Some(Cmyk::new(83, 0, 21, 0)),
            Self::Pms313C => Some(Cmyk::new(100, 0, 8, 13)),
            Self::Pms3135C => Some(Cmyk::new(100, 0, 16, 9)),
            Self::Pms314C => Some(Cmyk::new(100, 0, 9, 30)),
            Self::Pms3145C => Some(Cmyk::new(100, 0, 19, 23)),
            Self::Pms315C => Some(Cmyk::new(100, 0, 12, 43)),
            Self::Pms3155C => Some(Cmyk::new(100, 0, 24, 38)),
            Self::Pms316C => Some(Cmyk::new(100, 0, 27, 68)),
            Self::Pms3165C => Some(Cmyk::new(100, 0, 28, 65)),
            Self::Pms317C => Some(Cmyk::new(18, 0, 8, 0)),
            Self::Pms318C => Some(Cmyk::new(38, 0, 15, 0)),
            Self::Pms319C => Some(Cmyk::new(52, 0, 19, 0)),
            Self::Pms320C => Some(Cmyk::new(100, 0, 31, 7)),
            Self::Pms321C => Some(Cmyk::new(100, 0, 31, 23)),
            Self::Pms322C => Some(Cmyk::new(100, 0, 33, 35)),
            Self::Pms323C => Some(Cmyk::new(100, 0, 38, 47)),
            Self::Pms324C => Some(Cmyk::new(28, 0, 12, 0)),
            Self::Pms3242C => Some(Cmyk::new(37, 0, 18, 0)),
            Self::Pms3245C => Some(Cmyk::new(34, 0, 19, 0)),
            Self::Pms3248C => Some(Cmyk::new(43, 0, 24, 0)),
            Self::Pms325C => Some(Cmyk::new(56, 0, 26, 0)),
            Self::Pms3252C => Some(Cmyk::new(47, 0, 24, 0)),
            Self::Pms3255C => Some(Cmyk::new(49, 0, 28, 0)),
            Self::Pms3258C => Some(Cmyk::new(59, 0, 33, 0)),
            Self::Pms326C => Some(Cmyk::new(87, 0, 38, 0)),
            Self::Pms3262C => Some(Cmyk::new(71, 0, 33, 0)),
            Self::Pms3265C => Some(Cmyk::new(69, 0, 37, 0)),
            Self::Pms3268C => Some(Cmyk::new(90, 0, 49, 0)),
            Self::Pms327C => Some(Cmyk::new(100, 0, 44, 17)),
            Self::Pms3272C => Some(Cmyk::new(100, 0, 44, 0)),
            Self::Pms3275C => Some(Cmyk::new(95, 0, 47, 0)),
            Self::Pms3278C => Some(Cmyk::new(100, 0, 55, 5)),
            Self::Pms328C => Some(Cmyk::new(100, 0, 45, 32)),
            Self::Pms3282C => Some(Cmyk::new(100, 0, 46, 15)),
            Self::Pms3285C => Some(Cmyk::new(100, 0, 50, 7)),
            Self::Pms3288C => Some(Cmyk::new(100, 0, 54, 20)),
            Self::Pms329C => Some(Cmyk::new(100, 0, 46, 46)),
            Self::Pms3292C => Some(Cmyk::new(100, 0, 49, 46)),
            Self::Pms3295C => Some(Cmyk::new(100, 0, 53, 21)),
            Self::Pms3298C => Some(Cmyk::new(100, 0, 57, 42)),
            Self::Pms330C => Some(Cmyk::new(100, 0, 48, 60)),
            Self::Pms3302C => Some(Cmyk::new(100, 0, 54, 69)),
            Self::Pms3305C => Some(Cmyk::new(100, 0, 61, 61)),
            Self::Pms3308C => Some(Cmyk::new(100, 0, 60, 72)),
            Self::Pms331C => Some(Cmyk::new(24, 0, 16, 0)),
            Self::Pms332C => Some(Cmyk::new(30, 0, 20, 0)),
            Self::Pms333C => Some(Cmyk::new(43, 0, 27, 0)),
            Self::Pms334C => Some(Cmyk::new(100, 0, 60, 3)),
            Self::Pms335C => Some(Cmyk::new(100, 0, 65, 30)),
            Self::Pms336C => Some(Cmyk::new(100, 0, 67, 47)),
            Self::Pms337C => Some(Cmyk::new(31, 0, 20, 0)),
            Self::Pms3375C => Some(Cmyk::new(35, 0, 25, 0)),
            Self::Pms338C => Some(Cmyk::new(47, 0, 32, 0)),
            Self::Pms3385C => Some(Cmyk::new(45, 0, 33, 0)),
            Self::Pms339C => Some(Cmyk::new(84, 0, 56, 0)),
            Self::Pms3395C => Some(Cmyk::new(61, 0, 45, 0)),
            Self::Pms340C => Some(Cmyk::new(100, 0, 66, 9)),
            Self::Pms3405C => Some(Cmyk::new(85, 0, 65, 0)),
            Self::Pms341C => Some(Cmyk::new(100, 0, 67, 29)),
            Self::Pms3415C => Some(Cmyk::new(100, 0, 77, 22)),
            Self::Pms342C => Some(Cmyk::new(100, 0, 71, 43)),
            Self::Pms3425C => Some(Cmyk::new(100, 0, 78, 42)),
            Self::Pms343C => Some(Cmyk::new(98, 0, 72, 61)),
            Self::Pms3435C => Some(Cmyk::new(100, 0, 81, 66)),
            Self::Pms344C => Some(Cmyk::new(27, 0, 23, 0)),
            Self::Pms345C => Some(Cmyk::new(38, 0, 32, 0)),
            Self::Pms346C => Some(Cmyk::new(55, 0, 47, 0)),
            Self::Pms347C => Some(Cmyk::new(100, 0, 86, 3)),
            Self::Pms348C => Some(Cmyk::new(100, 0, 85, 24)),
            Self::Pms349C => Some(Cmyk::new(100, 0, 91, 42)),
            Self::Pms350C => Some(Cmyk::new(79, 0, 100, 75)),
            Self::Pms351C => Some(Cmyk::new(17, 0, 16, 0)),
            Self::Pms352C => Some(Cmyk::new(27, 0, 25, 0)),
            Self::Pms353C => Some(Cmyk::new(38, 0, 36, 0)),
            Self::Pms354C => Some(Cmyk::new(80, 0, 90, 0)),
            Self::Pms355C => Some(Cmyk::new(94, 0, 100, 0)),
            Self::Pms356C => Some(Cmyk::new(95, 0, 100, 27)),
            Self::Pms357C => Some(Cmyk::new(80, 0, 100, 56)),
            Self::Pms358C => Some(Cmyk::new(27, 0, 38, 0)),
            Self::Pms359C => Some(Cmyk::new(36, 0, 49, 0)),
            Self::Pms360C => Some(Cmyk::new(58, 0, 80, 0)),
            Self::Pms361C => Some(Cmyk::new(69, 0, 100, 0)),
            Self::Pms362C => Some(Cmyk::new(70, 0, 100, 9)),
            Self::Pms363C => Some(Cmyk::new(68, 0, 100, 24)),
            Self::Pms364C => Some(Cmyk::new(65, 0, 100, 42)),
            Self::Pms365C => Some(Cmyk::new(12, 0, 29, 0)),
            Self::Pms366C => Some(Cmyk::new(20, 0, 44, 0)),
            Self::Pms367C => Some(Cmyk::new(32, 0, 59, 0)),
            Self::Pms368C => Some(Cmyk::new(57, 0, 100, 0)),
            Self::Pms369C => Some(Cmyk::new(59, 0, 100, 7)),
            Self::Pms370C => Some(Cmyk::new(56, 0, 100, 27)),
            Self::Pms371C => Some(Cmyk::new(43, 0, 100, 56)),
            Self::Pms372C => Some(Cmyk::new(10, 0, 33, 0)),
            Self::Pms373C => Some(Cmyk::new(16, 0, 46, 0)),
            Self::Pms374C => Some(Cmyk::new(24, 0, 57, 0)),
            Self::Pms375C => Some(Cmyk::new(41, 0, 78, 0)),
            Self::Pms376C => Some(Cmyk::new(52, 0, 100, 5)),
            Self::Pms377C => Some(Cmyk::new(45, 0, 100, 24)),
            Self::Pms378C => Some(Cmyk::new(34, 0, 100, 60)),
            Self::Pms379C => Some(Cmyk::new(9, 0, 58, 0)),
            Self::Pms380C => Some(Cmyk::new(13, 0, 72, 0)),
            Self::Pms381C => Some(Cmyk::new(20, 0, 91, 0)),
            Self::Pms382C => Some(Cmyk::new(29, 0, 100, 0)),
            Self::Pms383C => Some(Cmyk::new(20, 0, 100, 19)),
            Self::Pms384C => Some(Cmyk::new(18, 0, 100, 31)),
            Self::Pms385C => Some(Cmyk::new(3, 0, 100, 58)),
            Self::Pms386C => Some(Cmyk::new(6, 0, 56, 0)),
            Self::Pms387C => Some(Cmyk::new(10, 0, 74, 0)),
            Self::Pms388C => Some(Cmyk::new(14, 0, 79, 0)),
            Self::Pms389C => Some(Cmyk::new(20, 0, 85, 0)),
            Self::Pms390C => Some(Cmyk::new(22, 0, 100, 8)),
            Self::Pms391C => Some(Cmyk::new(13, 0, 100, 33)),
            Self::Pms392C => Some(Cmyk::new(7, 0, 100, 49)),
            Self::Pms393C => Some(Cmyk::new(3, 0, 55, 0)),
            Self::Pms3935C => Some(Cmyk::new(1, 0, 68, 0)),
            Self::Pms394C => Some(Cmyk::new(6, 0, 76, 0)),
            Self::Pms3945C => Some(Cmyk::new(3, 0, 85, 0)),
            Self::Pms395C => Some(Cmyk::new(8, 0, 85, 0)),
            Self::Pms3955C => Some(Cmyk::new(6, 0, 100, 0)),
            Self::Pms396C => Some(Cmyk::new(11, 0, 94, 0)),
            Self::Pms3965C => Some(Cmyk::new(8, 0, 100, 0)),
            Self::Pms397C => Some(Cmyk::new(10, 0, 100, 11)),
            Self::Pms3975C => Some(Cmyk::new(0, 0, 100, 29)),
            Self::Pms398C => Some(Cmyk::new(7, 0, 100, 28)),
            Self::Pms3985C => Some(Cmyk::new(0, 3, 100, 41)),
            Self::Pms399C => Some(Cmyk::new(0, 0, 100, 43)),
            Self::Pms3995C => Some(Cmyk::new(0, 3, 100, 64)),
            Self::Pms400C => Some(Cmyk::new(0, 3, 6, 16)),
            Self::Pms401C => Some(Cmyk::new(0, 5, 11, 23)),
            Self::Pms402C => Some(Cmyk::new(0, 6, 14, 31)),
            Self::Pms403C => Some(Cmyk::new(0, 7, 17, 43)),
            Self::Pms404C => Some(Cmyk::new(0, 8, 22, 56)),
            Self::Pms405C => Some(Cmyk::new(0, 10, 33, 72)),
            Self::Pms406C => Some(Cmyk::new(0, 5, 6, 16)),
            Self::Pms407C => Some(Cmyk::new(0, 8, 9, 26)),
            Self::Pms408C => Some(Cmyk::new(0, 10, 11, 34)),
            Self::Pms409C => Some(Cmyk::new(0, 13, 15, 45)),
            Self::Pms410C => Some(Cmyk::new(0, 18, 21, 56)),
            Self::Pms411C => Some(Cmyk::new(0, 27, 36, 72)),
            Self::Pms412C => Some(Cmyk::new(0, 30, 66, 98)),
            Self::Pms413C => Some(Cmyk::new(0, 0, 9, 20)),
            Self::Pms414C => Some(Cmyk::new(0, 0, 10, 30)),
            Self::Pms415C => Some(Cmyk::new(0, 0, 12, 41)),
            Self::Pms416C => Some(Cmyk::new(0, 0, 16, 50)),
            Self::Pms417C => Some(Cmyk::new(1, 0, 25, 65)),
            Self::Pms418C => Some(Cmyk::new(3, 0, 31, 75)),
            Self::Pms419C => Some(Cmyk::new(29, 0, 36, 100)),
            Self::Pms420C => Some(Cmyk::new(0, 0, 0, 15)),
            Self::Pms421C => Some(Cmyk::new(0, 0, 0, 26)),
            Self::Pms422C => Some(Cmyk::new(0, 0, 0, 33)),
            Self::Pms423C => Some(Cmyk::new(0, 0, 0, 44)),
            Self::Pms424C => Some(Cmyk::new(0, 0, 0, 61)),
            Self::Pms425C => Some(Cmyk::new(0, 0, 0, 77)),
            Self::Pms426C => Some(Cmyk::new(0, 0, 0, 100)),
            Self::Pms427C => Some(Cmyk::new(0, 0, 0, 11)),
            Self::Pms428C => Some(Cmyk::new(2, 0, 0, 18)),
            Self::Pms429C => Some(Cmyk::new(3, 0, 0, 32)),
            Self::Pms430C => Some(Cmyk::new(5, 0, 0, 45)),
            Self::Pms431C => Some(Cmyk::new(11, 1, 0, 64)),
            Self::Pms432C => Some(Cmyk::new(23, 2, 0, 77)),
            Self::Pms433C => Some(Cmyk::new(33, 3, 0, 95)),
            Self::Pms434C => Some(Cmyk::new(7, 10, 9, 0)),
            Self::Pms435C => Some(Cmyk::new(13, 16, 14, 0)),
            Self::Pms436C => Some(Cmyk::new(24, 25, 25, 0)),
            Self::Pms437C => Some(Cmyk::new(46, 45, 46, 0)),
            Self::Pms438C => Some(Cmyk::new(75, 70, 100, 10)),
            Self::Pms439C => Some(Cmyk::new(80, 75, 100, 20)),
            Self::Pms440C => Some(Cmyk::new(82, 78, 100, 30)),
            Self::Pms441C => Some(Cmyk::new(6, 0, 7, 9)),
            Self::Pms442C => Some(Cmyk::new(8, 0, 9, 19)),
            Self::Pms443C => Some(Cmyk::new(12, 0, 12, 30)),
            Self::Pms444C => Some(Cmyk::new(15, 0, 15, 42)),
            Self::Pms445C => Some(Cmyk::new(20, 0, 20, 65)),
            Self::Pms446C => Some(Cmyk::new(21, 0, 23, 75)),
            Self::Pms447C => Some(Cmyk::new(16, 0, 31, 82)),
            Self::Pms448C => Some(Cmyk::new(75, 65, 100, 30)),
            Self::Pms4485C => Some(Cmyk::new(0, 26, 100, 69)),
            Self::Pms449C => Some(Cmyk::new(70, 60, 100, 20)),
            Self::Pms4495C => Some(Cmyk::new(0, 20, 95, 46)),
            Self::Pms450C => Some(Cmyk::new(65, 55, 100, 10)),
            Self::Pms4505C => Some(Cmyk::new(0, 15, 78, 36)),
            Self::Pms451C => Some(Cmyk::new(33, 29, 55, 0)),
            Self::Pms4515C => Some(Cmyk::new(0, 9, 50, 24)),
            Self::Pms452C => Some(Cmyk::new(24, 19, 39, 0)),
            Self::Pms4525C => Some(Cmyk::new(0, 7, 39, 17)),
            Self::Pms453C => Some(Cmyk::new(14, 11, 25, 0)),
            Self::Pms4535C => Some(Cmyk::new(0, 4, 30, 11)),
            Self::Pms454C => Some(Cmyk::new(9, 7, 16, 0)),
            Self::Pms4545C => Some(Cmyk::new(0, 3, 19, 6)),
            Self::Pms455C => Some(Cmyk::new(0, 17, 100, 65)),
            Self::Pms456C => Some(Cmyk::new(0, 15, 100, 43)),
            Self::Pms457C => Some(Cmyk::new(0, 15, 100, 28)),
            Self::Pms458C => Some(Cmyk::new(10, 10, 73, 0)),
            Self::Pms459C => Some(Cmyk::new(6, 7, 55, 0)),
            Self::Pms460C => Some(Cmyk::new(4, 5, 44, 0)),
            Self::Pms461C => Some(Cmyk::new(3, 3, 35, 0)),
            Self::Pms462C => Some(Cmyk::new(50, 70, 100, 50)),
            Self::Pms4625C => Some(Cmyk::new(0, 60, 100, 79)),
            Self::Pms463C => Some(Cmyk::new(30, 60, 100, 40)),
            Self::Pms4635C => Some(Cmyk::new(0, 48, 96, 44)),
            Self::Pms464C => Some(Cmyk::new(10, 50, 100, 35)),
            Self::Pms4645C => Some(Cmyk::new(0, 37, 68, 28)),
            Self::Pms465C => Some(Cmyk::new(20, 32, 58, 0)),
            Self::Pms4655C => Some(Cmyk::new(0, 26, 45, 18)),
            Self::Pms466C => Some(Cmyk::new(12, 22, 43, 0)),
            Self::Pms4665C => Some(Cmyk::new(0, 18, 32, 10)),
            Self::Pms467C => Some(Cmyk::new(9, 15, 34, 0)),
            Self::Pms4675C => Some(Cmyk::new(0, 11, 21, 6)),
            Self::Pms468C => Some(Cmyk::new(6, 9, 23, 0)),
            Self::Pms4685C => Some(Cmyk::new(0, 7, 14, 4)),
            Self::Pms469C => Some(Cmyk::new(0, 65, 100, 71)),
            Self::Pms4695C => Some(Cmyk::new(0, 81, 100, 74)),
            Self::Pms470C => Some(Cmyk::new(0, 58, 100, 33)),
            Self::Pms4705C => Some(Cmyk::new(0, 62, 71, 49)),
            Self::Pms471C => Some(Cmyk::new(0, 59, 100, 18)),
            Self::Pms4715C => Some(Cmyk::new(0, 42, 45, 34)),
            Self::Pms472C => Some(Cmyk::new(0, 34, 52, 0)),
            Self::Pms4725C => Some(Cmyk::new(0, 32, 35, 25)),
            Self::Pms473C => Some(Cmyk::new(0, 23, 36, 0)),
            Self::Pms4735C => Some(Cmyk::new(0, 22, 23, 15)),
            Self::Pms474C => Some(Cmyk::new(0, 15, 26, 0)),
            Self::Pms4745C => Some(Cmyk::new(0, 17, 18, 10)),
            Self::Pms475C => Some(Cmyk::new(0, 11, 20, 0)),
            Self::Pms4755C => Some(Cmyk::new(0, 10, 12, 6)),
            Self::Pms476C => Some(Cmyk::new(60, 80, 100, 45)),
            Self::Pms477C => Some(Cmyk::new(50, 85, 100, 38)),
            Self::Pms478C => Some(Cmyk::new(40, 90, 100, 30)),
            Self::Pms479C => Some(Cmyk::new(31, 48, 55, 0)),
            Self::Pms480C => Some(Cmyk::new(15, 29, 31, 0)),
            Self::Pms481C => Some(Cmyk::new(9, 19, 21, 0)),
            Self::Pms482C => Some(Cmyk::new(5, 12, 13, 0)),
            Self::Pms483C => Some(Cmyk::new(0, 93, 100, 60)),
            Self::Pms484C => Some(Cmyk::new(0, 95, 100, 29)),
            Self::Pms485C => Some(Cmyk::new(0, 95, 100, 0)),
            Self::Pms486C => Some(Cmyk::new(0, 47, 41, 0)),
            Self::Pms487C => Some(Cmyk::new(0, 35, 28, 0)),
            Self::Pms488C => Some(Cmyk::new(0, 26, 19, 0)),
            Self::Pms489C => Some(Cmyk::new(0, 15, 11, 0)),
            Self::Pms490C => Some(Cmyk::new(0, 74, 100, 72)),
            Self::Pms491C => Some(Cmyk::new(0, 79, 100, 52)),
            Self::Pms492C => Some(Cmyk::new(0, 77, 100, 42)),
            Self::Pms493C => Some(Cmyk::new(0, 46, 23, 5)),
            Self::Pms494C => Some(Cmyk::new(0, 33, 13, 0)),
            Self::Pms495C => Some(Cmyk::new(0, 24, 11, 0)),
            Self::Pms496C => Some(Cmyk::new(0, 16, 9, 0)),
            Self::Pms497C => Some(Cmyk::new(0, 70, 100, 78)),
            Self::Pms4975C => Some(Cmyk::new(0, 73, 100, 80)),
            Self::Pms498C => Some(Cmyk::new(0, 64, 100, 60)),
            Self::Pms4985C => Some(Cmyk::new(0, 62, 51, 48)),
            Self::Pms499C => Some(Cmyk::new(0, 63, 100, 48)),
            Self::Pms4995C => Some(Cmyk::new(0, 48, 38, 34)),
            Self::Pms500C => Some(Cmyk::new(0, 38, 21, 11)),
            Self::Pms5005C => Some(Cmyk::new(0, 38, 27, 23)),
            Self::Pms501C => Some(Cmyk::new(0, 27, 13, 3)),
            Self::Pms5015C => Some(Cmyk::new(0, 25, 15, 11)),
            Self::Pms502C => Some(Cmyk::new(0, 18, 10, 1)),
            Self::Pms5025C => Some(Cmyk::new(0, 18, 12, 7)),
            Self::Pms503C => Some(Cmyk::new(0, 11, 8, 0)),
            Self::Pms5035C => Some(Cmyk::new(0, 10, 9, 3)),
            Self::Pms504C => Some(Cmyk::new(70, 100, 100, 35)),
            Self::Pms505C => Some(Cmyk::new(50, 100, 100, 25)),
            Self::Pms506C => Some(Cmyk::new(45, 100, 100, 15)),
            Self::Pms507C => Some(Cmyk::new(13, 49, 23, 0)),
            Self::Pms508C => Some(Cmyk::new(5, 36, 11, 0)),
            Self::Pms509C => Some(Cmyk::new(0, 24, 7, 0)),
            Self::Pms510C => Some(Cmyk::new(0, 17, 6, 0)),
            Self::Pms511C => Some(Cmyk::new(70, 100, 50, 20)),
            Self::Pms5115C => Some(Cmyk::new(80, 100, 70, 15)),
            Self::Pms512C => Some(Cmyk::new(60, 100, 15, 5)),
            Self::Pms5125C => Some(Cmyk::new(65, 86, 49, 0)),
            Self::Pms513C => Some(Cmyk::new(46, 88, 0, 0)),
            Self::Pms5135C => Some(Cmyk::new(49, 66, 28, 0)),
            Self::Pms514C => Some(Cmyk::new(15, 50, 0, 0)),
            Self::Pms5145C => Some(Cmyk::new(30, 45, 12, 0)),
            Self::Pms515C => Some(Cmyk::new(7, 38, 0, 0)),
            Self::Pms5155C => Some(Cmyk::new(17, 30, 8, 0)),
            Self::Pms516C => Some(Cmyk::new(3, 27, 0, 0)),
            Self::Pms5165C => Some(Cmyk::new(9, 19, 5, 0)),
            Self::Pms517C => Some(Cmyk::new(0, 18, 0, 0)),
            Self::Pms5175C => Some(Cmyk::new(5, 12, 3, 0)),
            Self::Pms518C => Some(Cmyk::new(80, 100, 60, 10)),
            Self::Pms5185C => Some(Cmyk::new(88, 100, 85, 10)),
            Self::Pms519C => Some(Cmyk::new(72, 100, 35, 5)),
            Self::Pms5195C => Some(Cmyk::new(75, 95, 68, 5)),
            Self::Pms520C => Some(Cmyk::new(64, 100, 12, 0)),
            Self::Pms5205C => Some(Cmyk::new(52, 63, 48, 0)),
            Self::Pms521C => Some(Cmyk::new(27, 47, 0, 0)),
            Self::Pms5215C => Some(Cmyk::new(29, 38, 24, 0)),
            Self::Pms522C => Some(Cmyk::new(17, 37, 0, 0)),
            Self::Pms5225C => Some(Cmyk::new(17, 27, 15, 0)),
            Self::Pms523C => Some(Cmyk::new(10, 26, 0, 0)),
            Self::Pms5235C => Some(Cmyk::new(10, 17, 10, 0)),
            Self::Pms524C => Some(Cmyk::new(6, 15, 0, 0)),
            Self::Pms5245C => Some(Cmyk::new(6, 10, 7, 0)),
            Self::Pms525C => Some(Cmyk::new(84, 100, 45, 5)),
            Self::Pms5255C => Some(Cmyk::new(83, 85, 0, 70)),
            Self::Pms526C => Some(Cmyk::new(77, 100, 7, 0)),
            Self::Pms5265C => Some(Cmyk::new(77, 73, 0, 46)),
            Self::Pms527C => Some(Cmyk::new(73, 100, 0, 0)),
            Self::Pms5275C => Some(Cmyk::new(67, 57, 0, 34)),
            Self::Pms528C => Some(Cmyk::new(41, 55, 0, 0)),
            Self::Pms5285C => Some(Cmyk::new(42, 35, 0, 23)),
            Self::Pms529C => Some(Cmyk::new(26, 40, 0, 0)),
            Self::Pms5295C => Some(Cmyk::new(27, 22, 0, 10)),
            Self::Pms530C => Some(Cmyk::new(18, 31, 0, 0)),
            Self::Pms5305C => Some(Cmyk::new(16, 13, 0, 7)),
            Self::Pms531C => Some(Cmyk::new(10, 20, 0, 0)),
            Self::Pms5315C => Some(Cmyk::new(6, 6, 0, 5)),
            Self::Pms532C => Some(Cmyk::new(100, 88, 70, 18)),
            Self::Pms533C => Some(Cmyk::new(100, 85, 46, 11)),
            Self::Pms534C => Some(Cmyk::new(100, 82, 30, 5)),
            Self::Pms535C => Some(Cmyk::new(42, 30, 7, 0)),
            Self::Pms536C => Some(Cmyk::new(31, 22, 5, 0)),
            Self::Pms537C => Some(Cmyk::new(22, 14, 3, 0)),
            Self::Pms538C => Some(Cmyk::new(13, 9, 2, 0)),
            Self::Pms539C => Some(Cmyk::new(100, 49, 0, 70)),
            Self::Pms5395C => Some(Cmyk::new(100, 44, 0, 76)),
            Self::Pms540C => Some(Cmyk::new(100, 55, 0, 55)),
            Self::Pms5405C => Some(Cmyk::new(58, 17, 0, 46)),
            Self::Pms541C => Some(Cmyk::new(100, 57, 0, 38)),
            Self::Pms5415C => Some(Cmyk::new(40, 8, 0, 41)),
            Self::Pms542C => Some(Cmyk::new(62, 22, 0, 3)),
            Self::Pms5425C => Some(Cmyk::new(30, 4, 0, 32)),
            Self::Pms543C => Some(Cmyk::new(41, 11, 0, 0)),
            Self::Pms5435C => Some(Cmyk::new(13, 3, 0, 17)),
            Self::Pms544C => Some(Cmyk::new(30, 6, 0, 0)),
            Self::Pms5445C => Some(Cmyk::new(8, 1, 0, 13)),
            Self::Pms545C => Some(Cmyk::new(22, 3, 0, 0)),
            Self::Pms5455C => Some(Cmyk::new(6, 0, 0, 9)),
            Self::Pms546C => Some(Cmyk::new(95, 9, 0, 83)),
            Self::Pms5463C => Some(Cmyk::new(100, 0, 18, 83)),
            Self::Pms5467C => Some(Cmyk::new(100, 0, 30, 95)),
            Self::Pms547C => Some(Cmyk::new(100, 19, 0, 75)),
            Self::Pms5473C => Some(Cmyk::new(82, 0, 28, 52)),
            Self::Pms5477C => Some(Cmyk::new(56, 0, 27, 74)),
            Self::Pms548C => Some(Cmyk::new(100, 24, 0, 64)),
            Self::Pms5483C => Some(Cmyk::new(62, 0, 21, 31)),
            Self::Pms5487C => Some(Cmyk::new(36, 0, 17, 56)),
            Self::Pms549C => Some(Cmyk::new(54, 7, 0, 27)),
            Self::Pms5493C => Some(Cmyk::new(43, 0, 14, 21)),
            Self::Pms5497C => Some(Cmyk::new(18, 0, 9, 38)),
            Self::Pms550C => Some(Cmyk::new(39, 4, 0, 21)),
            Self::Pms5503C => Some(Cmyk::new(29, 0, 10, 14)),
            Self::Pms5507C => Some(Cmyk::new(11, 0, 7, 29)),
            Self::Pms551C => Some(Cmyk::new(27, 3, 0, 13)),
            Self::Pms5513C => Some(Cmyk::new(18, 0, 7, 5)),
            Self::Pms5517C => Some(Cmyk::new(9, 0, 6, 18)),
            Self::Pms552C => Some(Cmyk::new(15, 0, 0, 9)),
            Self::Pms5523C => Some(Cmyk::new(11, 0, 5, 3)),
            Self::Pms5527C => Some(Cmyk::new(6, 0, 4, 11)),
            Self::Pms553C => Some(Cmyk::new(60, 0, 51, 80)),
            Self::Pms5535C => Some(Cmyk::new(66, 0, 57, 82)),
            Self::Pms554C => Some(Cmyk::new(80, 0, 60, 67)),
            Self::Pms5545C => Some(Cmyk::new(59, 0, 50, 52)),
            Self::Pms555C => Some(Cmyk::new(79, 0, 56, 56)),
            Self::Pms5555C => Some(Cmyk::new(43, 0, 34, 38)),
            Self::Pms556C => Some(Cmyk::new(45, 0, 31, 27)),
            Self::Pms5565C => Some(Cmyk::new(30, 0, 24, 26)),
            Self::Pms557C => Some(Cmyk::new(30, 0, 20, 15)),
            Self::Pms5575C => Some(Cmyk::new(20, 0, 16, 17)),
            Self::Pms558C => Some(Cmyk::new(20, 0, 13, 9)),
            Self::Pms5585C => Some(Cmyk::new(12, 0, 11, 10)),
            Self::Pms559C => Some(Cmyk::new(14, 0, 10, 6)),
            Self::Pms5595C => Some(Cmyk::new(7, 0, 8, 7)),
            Self::Pms560C => Some(Cmyk::new(80, 0, 63, 75)),
            Self::Pms5605C => Some(Cmyk::new(65, 0, 56, 94)),
            Self::Pms561C => Some(Cmyk::new(85, 0, 54, 52)),
            Self::Pms5615C => Some(Cmyk::new(44, 0, 47, 68)),
            Self::Pms562C => Some(Cmyk::new(85, 0, 50, 31)),
            Self::Pms5625C => Some(Cmyk::new(28, 0, 30, 51)),
            Self::Pms563C => Some(Cmyk::new(52, 0, 32, 1)),
            Self::Pms5635C => Some(Cmyk::new(16, 0, 18, 36)),
            Self::Pms564C => Some(Cmyk::new(37, 0, 20, 0)),
            Self::Pms5645C => Some(Cmyk::new(9, 0, 13, 25)),
            Self::Pms565C => Some(Cmyk::new(23, 0, 13, 0)),
            Self::Pms5655C => Some(Cmyk::new(6, 0, 9, 18)),
            Self::Pms566C => Some(Cmyk::new(14, 0, 9, 0)),
            Self::Pms5665C => Some(Cmyk::new(5, 0, 7, 10)),
            Self::Pms567C => Some(Cmyk::new(82, 0, 64, 70)),
            Self::Pms568C => Some(Cmyk::new(88, 0, 57, 36)),
            Self::Pms569C => Some(Cmyk::new(98, 0, 57, 17)),
            Self::Pms570C => Some(Cmyk::new(48, 0, 29, 0)),
            Self::Pms571C => Some(Cmyk::new(32, 0, 19, 0)),
            Self::Pms572C => Some(Cmyk::new(23, 0, 14, 0)),
            Self::Pms573C => Some(Cmyk::new(14, 0, 9, 0)),
            Self::Pms574C => Some(Cmyk::new(34, 0, 81, 71)),
            Self::Pms5743C => Some(Cmyk::new(33, 0, 85, 82)),
            Self::Pms5747C => Some(Cmyk::new(32, 0, 100, 79)),
            Self::Pms575C => Some(Cmyk::new(48, 0, 100, 53)),
            Self::Pms5753C => Some(Cmyk::new(25, 0, 81, 67)),
            Self::Pms5757C => Some(Cmyk::new(27, 0, 95, 55)),
            Self::Pms576C => Some(Cmyk::new(49, 0, 100, 39)),
            Self::Pms5763C => Some(Cmyk::new(16, 0, 74, 57)),
            Self::Pms5767C => Some(Cmyk::new(15, 0, 68, 39)),
            Self::Pms577C => Some(Cmyk::new(24, 0, 46, 10)),
            Self::Pms5773C => Some(Cmyk::new(9, 0, 43, 38)),
            Self::Pms5777C => Some(Cmyk::new(10, 0, 49, 28)),
            Self::Pms578C => Some(Cmyk::new(20, 0, 40, 6)),
            Self::Pms5783C => Some(Cmyk::new(6, 0, 28, 27)),
            Self::Pms5787C => Some(Cmyk::new(7, 0, 31, 13)),
            Self::Pms579C => Some(Cmyk::new(17, 0, 34, 3)),
            Self::Pms5793C => Some(Cmyk::new(4, 0, 21, 18)),
            Self::Pms5797C => Some(Cmyk::new(5, 0, 24, 9)),
            Self::Pms580C => Some(Cmyk::new(12, 0, 26, 2)),
            Self::Pms5803C => Some(Cmyk::new(2, 0, 12, 11)),
            Self::Pms5807C => Some(Cmyk::new(3, 0, 14, 6)),
            Self::Pms581C => Some(Cmyk::new(2, 0, 100, 72)),
            Self::Pms5815C => Some(Cmyk::new(0, 0, 91, 79)),
            Self::Pms582C => Some(Cmyk::new(13, 0, 100, 46)),
            Self::Pms5825C => Some(Cmyk::new(0, 2, 87, 59)),
            Self::Pms583C => Some(Cmyk::new(23, 0, 100, 17)),
            Self::Pms5835C => Some(Cmyk::new(0, 2, 67, 40)),
            Self::Pms584C => Some(Cmyk::new(12, 0, 79, 6)),
            Self::Pms5845C => Some(Cmyk::new(0, 1, 47, 30)),
            Self::Pms585C => Some(Cmyk::new(11, 0, 66, 2)),
            Self::Pms5855C => Some(Cmyk::new(0, 0, 31, 18)),
            Self::Pms586C => Some(Cmyk::new(9, 0, 53, 0)),
            Self::Pms5865C => Some(Cmyk::new(0, 0, 25, 13)),
            Self::Pms587C => Some(Cmyk::new(5, 0, 40, 0)),
            Self::Pms5875C => Some(Cmyk::new(0, 0, 18, 8)),
            Self::Pms600C => Some(Cmyk::new(0, 0, 29, 0)),
            Self::Pms601C => Some(Cmyk::new(0, 0, 40, 0)),
            Self::Pms602C => Some(Cmyk::new(0, 0, 51, 0)),
            Self::Pms603C => Some(Cmyk::new(0, 0, 69, 1)),
            Self::Pms604C => Some(Cmyk::new(0, 0, 88, 3)),
            Self::Pms605C => Some(Cmyk::new(0, 2, 100, 7)),
            Self::Pms606C => Some(Cmyk::new(0, 4, 100, 12)),
            Self::Pms607C => Some(Cmyk::new(0, 0, 18, 1)),
            Self::Pms608C => Some(Cmyk::new(0, 0, 32, 2)),
            Self::Pms609C => Some(Cmyk::new(0, 0, 46, 4)),
            Self::Pms610C => Some(Cmyk::new(0, 0, 58, 6)),
            Self::Pms611C => Some(Cmyk::new(0, 1, 92, 11)),
            Self::Pms612C => Some(Cmyk::new(0, 2, 100, 20)),
            Self::Pms613C => Some(Cmyk::new(0, 4, 100, 30)),
            Self::Pms614C => Some(Cmyk::new(0, 0, 20, 4)),
            Self::Pms615C => Some(Cmyk::new(0, 1, 27, 6)),
            Self::Pms616C => Some(Cmyk::new(0, 2, 35, 9)),
            Self::Pms617C => Some(Cmyk::new(0, 2, 48, 17)),
            Self::Pms618C => Some(Cmyk::new(0, 3, 87, 30)),
            Self::Pms619C => Some(Cmyk::new(0, 4, 100, 43)),
            Self::Pms620C => Some(Cmyk::new(0, 5, 100, 53)),
            Self::Pms621C => Some(Cmyk::new(13, 0, 10, 2)),
            Self::Pms622C => Some(Cmyk::new(24, 0, 19, 4)),
            Self::Pms623C => Some(Cmyk::new(32, 0, 24, 10)),
            Self::Pms624C => Some(Cmyk::new(44, 0, 35, 20)),
            Self::Pms625C => Some(Cmyk::new(56, 0, 44, 33)),
            Self::Pms626C => Some(Cmyk::new(76, 0, 64, 63)),
            Self::Pms627C => Some(Cmyk::new(90, 0, 76, 84)),
            Self::Pms628C => Some(Cmyk::new(19, 0, 6, 0)),
            Self::Pms629C => Some(Cmyk::new(34, 0, 9, 0)),
            Self::Pms630C => Some(Cmyk::new(47, 0, 11, 0)),
            Self::Pms631C => Some(Cmyk::new(67, 0, 12, 2)),
            Self::Pms632C => Some(Cmyk::new(92, 0, 15, 5)),
            Self::Pms633C => Some(Cmyk::new(100, 0, 10, 25)),
            Self::Pms634C => Some(Cmyk::new(100, 0, 9, 40)),
            Self::Pms635C => Some(Cmyk::new(32, 0, 8, 0)),
            Self::Pms636C => Some(Cmyk::new(45, 0, 9, 0)),
            Self::Pms637C => Some(Cmyk::new(55, 0, 9, 0)),
            Self::Pms638C => Some(Cmyk::new(83, 0, 10, 0)),
            Self::Pms639C => Some(Cmyk::new(100, 0, 5, 5)),
            Self::Pms640C => Some(Cmyk::new(100, 0, 0, 22)),
            Self::Pms641C => Some(Cmyk::new(100, 4, 0, 30)),
            Self::Pms642C => Some(Cmyk::new(16, 4, 0, 2)),
            Self::Pms643C => Some(Cmyk::new(25, 7, 0, 4)),
            Self::Pms644C => Some(Cmyk::new(42, 15, 0, 6)),
            Self::Pms645C => Some(Cmyk::new(57, 28, 0, 9)),
            Self::Pms646C => Some(Cmyk::new(69, 34, 0, 11)),
            Self::Pms647C => Some(Cmyk::new(100, 56, 0, 23)),
            Self::Pms648C => Some(Cmyk::new(100, 62, 0, 54)),
            Self::Pms649C => Some(Cmyk::new(14, 6, 0, 1)),
            Self::Pms650C => Some(Cmyk::new(24, 9, 0, 2)),
            Self::Pms651C => Some(Cmyk::new(38, 18, 0, 6)),
            Self::Pms652C => Some(Cmyk::new(60, 34, 0, 10)),
            Self::Pms653C => Some(Cmyk::new(100, 62, 0, 20)),
            Self::Pms654C => Some(Cmyk::new(100, 64, 0, 38)),
            Self::Pms655C => Some(Cmyk::new(100, 65, 0, 52)),
            Self::Pms656C => Some(Cmyk::new(14, 4, 0, 0)),
            Self::Pms657C => Some(Cmyk::new(24, 8, 0, 0)),
            Self::Pms658C => Some(Cmyk::new(41, 20, 0, 0)),
            Self::Pms659C => Some(Cmyk::new(64, 35, 0, 0)),
            Self::Pms660C => Some(Cmyk::new(90, 57, 0, 0)),
            Self::Pms661C => Some(Cmyk::new(100, 69, 0, 9)),
            Self::Pms662C => Some(Cmyk::new(100, 72, 0, 18)),
            Self::Pms663C => Some(Cmyk::new(7, 6, 0, 0)),
            Self::Pms664C => Some(Cmyk::new(11, 9, 0, 0)),
            Self::Pms665C => Some(Cmyk::new(20, 17, 0, 2)),
            Self::Pms666C => Some(Cmyk::new(31, 30, 0, 7)),
            Self::Pms667C => Some(Cmyk::new(52, 49, 0, 14)),
            Self::Pms668C => Some(Cmyk::new(65, 64, 0, 30)),
            Self::Pms669C => Some(Cmyk::new(76, 78, 0, 47)),
            Self::Pms670C => Some(Cmyk::new(0, 13, 0, 0)),
            Self::Pms671C => Some(Cmyk::new(1, 20, 0, 0)),
            Self::Pms672C => Some(Cmyk::new(3, 34, 0, 0)),
            Self::Pms673C => Some(Cmyk::new(6, 49, 0, 0)),
            Self::Pms674C => Some(Cmyk::new(9, 67, 0, 0)),
            Self::Pms675C => Some(Cmyk::new(17, 100, 0, 3)),
            Self::Pms676C => Some(Cmyk::new(6, 100, 0, 22)),
            Self::Pms677C => Some(Cmyk::new(2, 13, 0, 0)),
            Self::Pms678C => Some(Cmyk::new(3, 21, 0, 0)),
            Self::Pms679C => Some(Cmyk::new(5, 27, 0, 0)),
            Self::Pms680C => Some(Cmyk::new(10, 43, 0, 2)),
            Self::Pms681C => Some(Cmyk::new(21, 61, 0, 4)),
            Self::Pms682C => Some(Cmyk::new(25, 79, 0, 12)),
            Self::Pms683C => Some(Cmyk::new(11, 100, 0, 43)),
            Self::Pms684C => Some(Cmyk::new(0, 17, 0, 2)),
            Self::Pms685C => Some(Cmyk::new(0, 25, 0, 3)),
            Self::Pms686C => Some(Cmyk::new(0, 30, 0, 5)),
            Self::Pms687C => Some(Cmyk::new(2, 44, 0, 12)),
            Self::Pms688C => Some(Cmyk::new(5, 57, 0, 19)),
            Self::Pms689C => Some(Cmyk::new(7, 77, 0, 34)),
            Self::Pms690C => Some(Cmyk::new(0, 97, 0, 59)),
            Self::Pms691C => Some(Cmyk::new(0, 15, 8, 1)),
            Self::Pms692C => Some(Cmyk::new(0, 23, 10, 2)),
            Self::Pms693C => Some(Cmyk::new(0, 30, 12, 6)),
            Self::Pms694C => Some(Cmyk::new(0, 43, 19, 10)),
            Self::Pms695C => Some(Cmyk::new(0, 58, 28, 25)),
            Self::Pms696C => Some(Cmyk::new(0, 84, 54, 40)),
            Self::Pms697C => Some(Cmyk::new(0, 93, 70, 51)),
            Self::Pms698C => Some(Cmyk::new(0, 16, 8, 0)),
            Self::Pms699C => Some(Cmyk::new(0, 24, 10, 0)),
            Self::Pms700C => Some(Cmyk::new(0, 36, 14, 0)),
            Self::Pms701C => Some(Cmyk::new(0, 53, 20, 0)),
            Self::Pms702C => Some(Cmyk::new(0, 69, 34, 5)),
            Self::Pms703C => Some(Cmyk::new(0, 83, 54, 16)),
            Self::Pms704C => Some(Cmyk::new(0, 100, 80, 27)),
            Self::Pms705C => Some(Cmyk::new(0, 9, 5, 0)),
            Self::Pms706C => Some(Cmyk::new(0, 17, 10, 0)),
            Self::Pms707C => Some(Cmyk::new(0, 30, 14, 0)),
            Self::Pms708C => Some(Cmyk::new(0, 46, 22, 0)),
            Self::Pms709C => Some(Cmyk::new(0, 66, 38, 0)),
            Self::Pms710C => Some(Cmyk::new(0, 79, 58, 0)),
            Self::Pms711C => Some(Cmyk::new(0, 100, 80, 2)),
            Self::Pms712C => Some(Cmyk::new(0, 14, 31, 0)),
            Self::Pms713C => Some(Cmyk::new(0, 19, 41, 0)),
            Self::Pms714C => Some(Cmyk::new(0, 28, 54, 0)),
            Self::Pms715C => Some(Cmyk::new(0, 37, 71, 0)),
            Self::Pms716C => Some(Cmyk::new(0, 47, 91, 0)),
            Self::Pms717C => Some(Cmyk::new(0, 55, 100, 2)),
            Self::Pms718C => Some(Cmyk::new(0, 58, 100, 8)),
            Self::Pms719C => Some(Cmyk::new(0, 10, 25, 0)),
            Self::Pms720C => Some(Cmyk::new(0, 15, 34, 1)),
            Self::Pms721C => Some(Cmyk::new(0, 24, 52, 3)),
            Self::Pms722C => Some(Cmyk::new(0, 36, 76, 9)),
            Self::Pms723C => Some(Cmyk::new(0, 45, 95, 17)),
            Self::Pms724C => Some(Cmyk::new(0, 53, 100, 37)),
            Self::Pms725C => Some(Cmyk::new(0, 55, 100, 49)),
            Self::Pms726C => Some(Cmyk::new(0, 8, 23, 2)),
            Self::Pms727C => Some(Cmyk::new(0, 15, 34, 5)),
            Self::Pms728C => Some(Cmyk::new(0, 21, 48, 10)),
            Self::Pms729C => Some(Cmyk::new(0, 31, 61, 18)),
            Self::Pms730C => Some(Cmyk::new(0, 39, 76, 29)),
            Self::Pms731C => Some(Cmyk::new(0, 53, 100, 54)),
            Self::Pms732C => Some(Cmyk::new(0, 56, 100, 64)),
            Self::Pms7401C => Some(Cmyk::new(0, 4, 18, 0)),
            Self::Pms7402C => Some(Cmyk::new(0, 6, 30, 0)),
            Self::Pms7403C => Some(Cmyk::new(0, 10, 50, 0)),
            Self::Pms7404C => Some(Cmyk::new(0, 9, 80, 0)),
            Self::Pms7405C => Some(Cmyk::new(0, 10, 100, 0)),
            Self::Pms7406C => Some(Cmyk::new(0, 18, 100, 0)),
            Self::Pms7407C => Some(Cmyk::new(0, 22, 85, 11)),
            Self::Pms7408C => Some(Cmyk::new(0, 25, 95, 0)),
            Self::Pms7409C => Some(Cmyk::new(0, 30, 95, 0)),
            Self::Pms7410C => Some(Cmyk::new(0, 30, 55, 0)),
            Self::Pms7411C => Some(Cmyk::new(0, 35, 69, 0)),
            Self::Pms7412C => Some(Cmyk::new(0, 42, 100, 7)),
            Self::Pms7413C => Some(Cmyk::new(0, 53, 100, 4)),
            Self::Pms7414C => Some(Cmyk::new(0, 46, 100, 11)),
            Self::Pms7415C => Some(Cmyk::new(0, 18, 15, 0)),
            Self::Pms7416C => Some(Cmyk::new(0, 60, 60, 0)),
            Self::Pms7417C => Some(Cmyk::new(0, 75, 75, 0)),
            Self::Pms7418C => Some(Cmyk::new(0, 70, 60, 5)),
            Self::Pms7419C => Some(Cmyk::new(0, 60, 45, 18)),
            Self::Pms7420C => Some(Cmyk::new(0, 80, 42, 20)),
            Self::Pms7421C => Some(Cmyk::new(0, 100, 30, 61)),
            Self::Pms7422C => Some(Cmyk::new(0, 9, 5, 0)),
            Self::Pms7423C => Some(Cmyk::new(0, 55, 22, 0)),
            Self::Pms7424C => Some(Cmyk::new(0, 75, 30, 0)),
            Self::Pms7425C => Some(Cmyk::new(0, 90, 30, 7)),
            Self::Pms7426C => Some(Cmyk::new(0, 100, 45, 18)),
            Self::Pms7427C => Some(Cmyk::new(0, 100, 65, 28)),
            Self::Pms7428C => Some(Cmyk::new(0, 80, 45, 55)),
            Self::Pms7429C => Some(Cmyk::new(0, 18, 3, 0)),
            Self::Pms7430C => Some(Cmyk::new(2, 31, 0, 0)),
            Self::Pms7431C => Some(Cmyk::new(0, 38, 2, 5)),
            Self::Pms7432C => Some(Cmyk::new(0, 55, 3, 10)),
            Self::Pms7433C => Some(Cmyk::new(0, 75, 15, 15)),
            Self::Pms7434C => Some(Cmyk::new(0, 80, 15, 20)),
            Self::Pms7435C => Some(Cmyk::new(0, 100, 10, 35)),
            Self::Pms7436C => Some(Cmyk::new(3, 8, 0, 0)),
            Self::Pms7437C => Some(Cmyk::new(6, 20, 0, 0)),
            Self::Pms7438C => Some(Cmyk::new(15, 35, 0, 0)),
            Self::Pms7439C => Some(Cmyk::new(20, 35, 0, 0)),
            Self::Pms7440C => Some(Cmyk::new(30, 40, 0, 0)),
            Self::Pms7441C => Some(Cmyk::new(36, 50, 0, 0)),
            Self::Pms7442C => Some(Cmyk::new(50, 70, 0, 0)),
            Self::Pms7443C => Some(Cmyk::new(6, 5, 0, 0)),
            Self::Pms7444C => Some(Cmyk::new(20, 17, 0, 0)),
            Self::Pms7445C => Some(Cmyk::new(30, 20, 0, 3)),
            Self::Pms7446C => Some(Cmyk::new(43, 38, 0, 0)),
            Self::Pms7447C => Some(Cmyk::new(60, 58, 0, 19)),
            Self::Pms7448C => Some(Cmyk::new(32, 42, 0, 55)),
            Self::Pms7449C => Some(Cmyk::new(72, 100, 77, 46)),
            Self::Pms7450C => Some(Cmyk::new(20, 10, 0, 0)),
            Self::Pms7451C => Some(Cmyk::new(40, 21, 0, 0)),
            Self::Pms7452C => Some(Cmyk::new(50, 32, 0, 0)),
            Self::Pms7453C => Some(Cmyk::new(50, 26, 0, 0)),
            Self::Pms7454C => Some(Cmyk::new(50, 24, 0, 10)),
            Self::Pms7455C => Some(Cmyk::new(80, 53, 0, 0)),
            Self::Pms7456C => Some(Cmyk::new(55, 35, 0, 7)),
            Self::Pms7457C => Some(Cmyk::new(12, 0, 2, 0)),
            Self::Pms7458C => Some(Cmyk::new(40, 0, 5, 6)),
            Self::Pms7459C => Some(Cmyk::new(57, 0, 6, 13)),
            Self::Pms7460C => Some(Cmyk::new(100, 0, 0, 5)),
            Self::Pms7461C => Some(Cmyk::new(78, 28, 0, 0)),
            Self::Pms7462C => Some(Cmyk::new(100, 50, 0, 10)),
            Self::Pms7463C => Some(Cmyk::new(100, 43, 0, 65)),
            Self::Pms7464C => Some(Cmyk::new(25, 0, 10, 0)),
            Self::Pms7465C => Some(Cmyk::new(50, 0, 25, 0)),
            Self::Pms7466C => Some(Cmyk::new(70, 0, 23, 0)),
            Self::Pms7467C => Some(Cmyk::new(95, 0, 25, 0)),
            Self::Pms7468C => Some(Cmyk::new(100, 10, 0, 28)),
            Self::Pms7469C => Some(Cmyk::new(100, 20, 0, 40)),
            Self::Pms7470C => Some(Cmyk::new(80, 15, 0, 45)),
            Self::Pms7471C => Some(Cmyk::new(28, 0, 14, 0)),
            Self::Pms7472C => Some(Cmyk::new(52, 0, 25, 0)),
            Self::Pms7473C => Some(Cmyk::new(70, 0, 38, 8)),
            Self::Pms7474C => Some(Cmyk::new(90, 0, 28, 22)),
            Self::Pms7475C => Some(Cmyk::new(50, 0, 25, 30)),
            Self::Pms7476C => Some(Cmyk::new(100, 0, 43, 60)),
            Self::Pms7477C => Some(Cmyk::new(80, 0, 10, 68)),
            Self::Pms7478C => Some(Cmyk::new(18, 0, 14, 0)),
            Self::Pms7479C => Some(Cmyk::new(55, 0, 50, 0)),
            Self::Pms7480C => Some(Cmyk::new(60, 0, 50, 0)),
            Self::Pms7481C => Some(Cmyk::new(60, 0, 55, 0)),
            Self::Pms7482C => Some(Cmyk::new(80, 0, 75, 0)),
            Self::Pms7483C => Some(Cmyk::new(85, 0, 100, 55)),
            Self::Pms7484C => Some(Cmyk::new(100, 0, 85, 50)),
            Self::Pms7485C => Some(Cmyk::new(6, 0, 10, 0)),
            Self::Pms7486C => Some(Cmyk::new(20, 0, 30, 0)),
            Self::Pms7487C => Some(Cmyk::new(30, 0, 45, 0)),
            Self::Pms7488C => Some(Cmyk::new(43, 0, 60, 0)),
            Self::Pms7489C => Some(Cmyk::new(60, 0, 80, 7)),
            Self::Pms7490C => Some(Cmyk::new(45, 0, 80, 35)),
            Self::Pms7491C => Some(Cmyk::new(32, 0, 100, 40)),
            Self::Pms7492C => Some(Cmyk::new(12, 0, 50, 7)),
            Self::Pms7493C => Some(Cmyk::new(14, 0, 36, 10)),
            Self::Pms7494C => Some(Cmyk::new(25, 0, 40, 15)),
            Self::Pms7495C => Some(Cmyk::new(25, 0, 80, 30)),
            Self::Pms7496C => Some(Cmyk::new(40, 0, 100, 38)),
            Self::Pms7497C => Some(Cmyk::new(40, 30, 70, 25)),
            Self::Pms7498C => Some(Cmyk::new(25, 0, 100, 80)),
            Self::Pms7499C => Some(Cmyk::new(0, 2, 15, 0)),
            Self::Pms7500C => Some(Cmyk::new(0, 2, 15, 3)),
            Self::Pms7501C => Some(Cmyk::new(0, 4, 20, 6)),
            Self::Pms7502C => Some(Cmyk::new(0, 8, 35, 10)),
            Self::Pms7503C => Some(Cmyk::new(0, 12, 35, 25)),
            Self::Pms7504C => Some(Cmyk::new(0, 25, 45, 40)),
            Self::Pms7505C => Some(Cmyk::new(0, 30, 70, 55)),
            Self::Pms7506C => Some(Cmyk::new(0, 5, 15, 0)),
            Self::Pms7507C => Some(Cmyk::new(0, 10, 30, 0)),
            Self::Pms7508C => Some(Cmyk::new(0, 15, 40, 4)),
            Self::Pms7509C => Some(Cmyk::new(0, 20, 50, 5)),
            Self::Pms7510C => Some(Cmyk::new(0, 30, 72, 11)),
            Self::Pms7511C => Some(Cmyk::new(0, 45, 100, 25)),
            Self::Pms7512C => Some(Cmyk::new(0, 46, 100, 33)),
            Self::Pms7513C => Some(Cmyk::new(0, 18, 28, 3)),
            Self::Pms7514C => Some(Cmyk::new(0, 24, 38, 5)),
            Self::Pms7515C => Some(Cmyk::new(0, 35, 50, 12)),
            Self::Pms7516C => Some(Cmyk::new(0, 52, 100, 35)),
            Self::Pms7517C => Some(Cmyk::new(0, 60, 100, 44)),
            Self::Pms7518C => Some(Cmyk::new(0, 40, 55, 60)),
            Self::Pms7519C => Some(Cmyk::new(50, 60, 100, 48)),
            Self::Pms7520C => Some(Cmyk::new(0, 16, 19, 0)),
            Self::Pms7521C => Some(Cmyk::new(0, 25, 20, 10)),
            Self::Pms7522C => Some(Cmyk::new(0, 40, 30, 16)),
            Self::Pms7523C => Some(Cmyk::new(0, 40, 35, 20)),
            Self::Pms7524C => Some(Cmyk::new(0, 55, 60, 27)),
            Self::Pms7525C => Some(Cmyk::new(0, 45, 50, 30)),
            Self::Pms7526C => Some(Cmyk::new(0, 65, 100, 35)),
            Self::Pms7527C => Some(Cmyk::new(0, 2, 6, 7)),
            Self::Pms7528C => Some(Cmyk::new(0, 3, 10, 10)),
            Self::Pms7529C => Some(Cmyk::new(0, 4, 12, 17)),
            Self::Pms7530C => Some(Cmyk::new(0, 8, 21, 32)),
            Self::Pms7531C => Some(Cmyk::new(0, 10, 27, 50)),
            Self::Pms7532C => Some(Cmyk::new(0, 17, 50, 65)),
            Self::Pms7533C => Some(Cmyk::new(0, 22, 85, 85)),
            Self::Pms7534C => Some(Cmyk::new(0, 2, 8, 10)),
            Self::Pms7535C => Some(Cmyk::new(0, 3, 15, 20)),
            Self::Pms7536C => Some(Cmyk::new(0, 4, 22, 32)),
            Self::Pms7537C => Some(Cmyk::new(3, 0, 10, 20)),
            Self::Pms7538C => Some(Cmyk::new(9, 0, 13, 30)),
            Self::Pms7539C => Some(Cmyk::new(2, 0, 9, 36)),
            Self::Pms7540C => Some(Cmyk::new(0, 0, 0, 72)),
            Self::Pms7541C => Some(Cmyk::new(2, 0, 0, 5)),
            Self::Pms7542C => Some(Cmyk::new(10, 0, 3, 16)),
            Self::Pms7543C => Some(Cmyk::new(7, 0, 0, 30)),
            Self::Pms7544C => Some(Cmyk::new(10, 1, 0, 40)),
            Self::Pms7545C => Some(Cmyk::new(23, 2, 0, 63)),
            Self::Pms7546C => Some(Cmyk::new(33, 4, 0, 72)),
            Self::Pms7547C => Some(Cmyk::new(35, 4, 0, 94)),
            Self::Pms801C => Some(Cmyk::new(91, 0, 11, 0)),
            Self::Pms802C => Some(Cmyk::new(58, 0, 98, 0)),
            Self::Pms803C => Some(Cmyk::new(1, 9, 88, 0)),
            Self::Pms804C => Some(Cmyk::new(0, 65, 92, 0)),
            Self::Pms805C => Some(Cmyk::new(0, 87, 78, 0)),
            Self::Pms806C => Some(Cmyk::new(2, 94, 41, 0)),
            Self::Pms807C => Some(Cmyk::new(10, 96, 8, 0)),
            Self::Black2C => Some(Cmyk::new(0, 0, 30, 100)),
            Self::Black3C => Some(Cmyk::new(69, 0, 43, 100)),
            Self::Black4C => Some(Cmyk::new(0, 27, 91, 100)),
            Self::Black5C => Some(Cmyk::new(0, 38, 0, 100)),
            Self::Black6C => Some(Cmyk::new(100, 38, 0, 100)),
            Self::Black7C => Some(Cmyk::new(0, 0, 11, 87)),
            Self::BlackC => Some(Cmyk::new(0, 13, 49, 98)),
            Self::Blue072C => Some(Cmyk::new(100, 88, 0, 5)),
            Self::CoolGray1C => Some(Cmyk::new(0, 0, 0, 6)),
            Self::CoolGray10C => Some(Cmyk::new(0, 0, 0, 72)),
            Self::CoolGray11C => Some(Cmyk::new(0, 0, 0, 79)),
            Self::CoolGray2C => Some(Cmyk::new(0, 0, 0, 11)),
            Self::CoolGray3C => Some(Cmyk::new(0, 0, 0, 18)),
            Self::CoolGray4C => Some(Cmyk::new(0, 0, 0, 27)),
            Self::CoolGray5C => Some(Cmyk::new(0, 0, 0, 34)),
            Self::CoolGray6C => Some(Cmyk::new(0, 0, 0, 38)),
            Self::CoolGray7C => Some(Cmyk::new(0, 0, 0, 47)),
            Self::CoolGray8C => Some(Cmyk::new(0, 0, 0, 56)),
            Self::CoolGray9C => Some(Cmyk::new(0, 0, 0, 65)),
            Self::GreenC => Some(Cmyk::new(100, 0, 59, 0)),
            Self::Orange021C => Some(Cmyk::new(0, 53, 100, 0)),
            Self::ProcessBlueC => Some(Cmyk::new(100, 10, 0, 10)),
            Self::PurpleC => Some(Cmyk::new(38, 88, 0, 0)),
            Self::Red032C => Some(Cmyk::new(0, 90, 86, 0)),
            Self::ReflexBlueC => Some(Cmyk::new(100, 73, 0, 2)),
            Self::RhodamineRedC => Some(Cmyk::new(3, 89, 0, 0)),
            Self::RubineRedC => Some(Cmyk::new(0, 100, 15, 4)),
            Self::VioletC => Some(Cmyk::new(98, 100, 0, 0)),
            Self::WarmGray1C => Some(Cmyk::new(0, 2, 3, 6)),
            Self::WarmGray10C => Some(Cmyk::new(0, 14, 28, 55)),
            Self::WarmGray11C => Some(Cmyk::new(0, 17, 34, 62)),
            Self::WarmGray2C => Some(Cmyk::new(0, 2, 5, 9)),
            Self::WarmGray3C => Some(Cmyk::new(0, 4, 8, 17)),
            Self::WarmGray4C => Some(Cmyk::new(0, 4, 9, 24)),
            Self::WarmGray5C => Some(Cmyk::new(0, 5, 10, 29)),
            Self::WarmGray6C => Some(Cmyk::new(0, 6, 12, 31)),
            Self::WarmGray7C => Some(Cmyk::new(0, 8, 14, 38)),
            Self::WarmGray8C => Some(Cmyk::new(0, 9, 16, 43)),
            Self::WarmGray9C => Some(Cmyk::new(0, 11, 20, 47)),
            Self::WarmRedC => Some(Cmyk::new(0, 75, 90, 0)),
            Self::YellowC => Some(Cmyk::new(0, 1, 100, 0)),
            _ => None,
        }
    }

    /// The packed `0xRRGGBB` value of the color.
    const fn packed_rgb(&self) -> u32 {
        match self {
            Self::Pms100C => 0xF6EB61,
            Self::Pms100U => 0xFEF380,
            Self::Pms101C => 0xF7EA48,
            Self::Pms101U => 0xFFF166,
            Self::Pms102C => 0xFCE300,
            Self::Pms102U => 0xFFEC2D,
            Self::Pms103C => 0xC5A900,
            Self::Pms103U => 0xB49F29,
            Self::Pms104C => 0xAF9800,
            Self::Pms104U => 0x9B8D39,
            Self::Pms105C => 0x897A27,
            Self::Pms105U => 0x7D7547,
            Self::Pms106C => 0xF9E547,
            Self::Pms106U => 0xFFEB68,
            Self::Pms107C => 0xFBE122,
            Self::Pms107U => 0xFFE552,
            Self::Pms108C => 0xFEDB00,
            Self::Pms108U => 0xFFDD35,
            Self::Pms109C => 0xFFD100,
            Self::Pms109U => 0xFFC700,
            Self::Pms110C => 0xDAAA00,
            Self::Pms110U => 0xCC9F26,
            Self::Pms111C => 0xAA8A00,
            Self::Pms111U => 0x9C873B,
            Self::Pms112C => 0x9C8412,
            Self::Pms112U => 0x8A7A3E,
            Self::Pms113C => 0xFAE053,
            Self::Pms113U => 0xFFE15E,
            Self::Pms114C => 0xFBDD40,
            Self::Pms114U => 0xFFD94A,
            Self::Pms115C => 0xFDDA24,
            Self::Pms115U => 0xFFCE2E,
            Self::Pms116C => 0xFFCD00,
            Self::Pms116U => 0xFFB511,
            Self::Pms117C => 0xC99700,
            Self::Pms117U => 0xB78C30,
            Self::Pms118C => 0xAC8400,
            Self::Pms118U => 0x9E8139,
            Self::Pms119C => 0x897322,
            Self::Pms119U => 0x847748,
            Self::Pms120C => 0xFBDB65,
            Self::Pms120U => 0xFFD96A,
            Self::Pms1205C => 0xF8E08E,
            Self::Pms1205U => 0xFFE495,
            Self::Pms121C => 0xFDD757,
            Self::Pms121U => 0xFFCC52,
            Self::Pms1215C => 0xFBD872,
            Self::Pms1215U => 0xFFDA7E,
            Self::Pms122C => 0xFED141,
            Self::Pms122U => 0xFFC03F,
            Self::Pms1225C => 0xFFC845,
            Self::Pms1225U => 0xFFBB4F,
            Self::Pms123C => 0xFFC72C,
            Self::Pms123U => 0xFFAC2A,
            Self::Pms1235C => 0xFFB81C,
            Self::Pms1235U => 0xFFAE3B,
            Self::Pms124C => 0xEAAA00,
            Self::Pms124U => 0xD79133,
            Self::Pms1245C => 0xC69214,
            Self::Pms1245U => 0xBB8B41,
            Self::Pms125C => 0xB58500,
            Self::Pms125U => 0xA6813D,
            Self::Pms1255C => 0xAD841F,
            Self::Pms1255U => 0xA08246,
            Self::Pms126C => 0x9A7611,
            Self::Pms126U => 0x8E7742,
            Self::Pms1265C => 0x886B25,
            Self::Pms1265U => 0x887549,
            Self::Pms127C => 0xF3DD6D,
            Self::Pms127U => 0xFCE385,
            Self::Pms128C => 0xF3D54E,
            Self::Pms128U => 0xFCCF61,
            Self::Pms129C => 0xF3D03E,
            Self::Pms129U => 0xFAAF3F,
            Self::Pms130C => 0xF2A900,
            Self::Pms130U => 0xF79B2E,
            Self::Pms131C => 0xCC8A00,
            Self::Pms131U => 0xBA8032,
            Self::Pms132C => 0xA07400,
            Self::Pms132U => 0x977A3C,
            Self::Pms133C => 0x6C571B,
            Self::Pms133U => 0x7C7047,
            Self::Pms134C => 0xFDD26E,
            Self::Pms134U => 0xFFD378,
            Self::Pms1345C => 0xFDD086,
            Self::Pms1345U => 0xFFD591,
            Self::Pms135C => 0xFFC658,
            Self::Pms135U => 0xFFC267,
            Self::Pms1355C => 0xFFC56E,
            Self::Pms1355U => 0xFFC87E,
            Self::Pms136C => 0xFFBF3F,
            Self::Pms136U => 0xFFAD4A,
            Self::Pms1365C => 0xFFB549,
            Self::Pms1365U => 0xFFAD5B,
            Self::Pms137C => 0xFFA300,
            Self::Pms137U => 0xFF9F37,
            Self::Pms1375C => 0xFF9E1B,
            Self::Pms1375U => 0xFF9940,
            Self::Pms138C => 0xDE7C00,
            Self::Pms138U => 0xD37C32,
            Self::Pms1385C => 0xD57800,
            Self::Pms1385U => 0xCD7E3E,
            Self::Pms139C => 0xAF6D04,
            Self::Pms139U => 0x9D6F3C,
            Self::Pms1395C => 0x996017,
            Self::Pms1395U => 0x946E43,
            Self::Pms140C => 0x74531C,
            Self::Pms140U => 0x7F6A45,
            Self::Pms1405C => 0x6E4C1E,
            Self::Pms1405U => 0x796648,
            Self::Pms141C => 0xF2C75C,
            Self::Pms141U => 0xF9C16E,
            Self::Pms142C => 0xF1BE48,
            Self::Pms142U => 0xF8B05D,
            Self::Pms143C => 0xF1B434,
            Self::Pms143U => 0xF6A04D,
            Self::Pms144C => 0xED8B00,
            Self::Pms144U => 0xF38B3C,
            Self::Pms145C => 0xCF7F00,
            Self::Pms145U => 0xC97F3A,
            Self::Pms146C => 0xA76D11,
            Self::Pms146U => 0x9D7441,
            Self::Pms147C => 0x715C2A,
            Self::Pms147U => 0x76694A,
            Self::Pms148C => 0xFECB8B,
            Self::Pms148U => 0xFFC688,
            Self::Pms1485C => 0xFFAE62,
            Self::Pms1485U => 0xFFA563,
            Self::Pms149C => 0xFFC27B,
            Self::Pms149U => 0xFFB575,
            Self::Pms1495C => 0xFF8F1C,
            Self::Pms1495U => 0xFF9351,
            Self::Pms150C => 0xFFB25B,
            Self::Pms150U => 0xFF9955,
            Self::Pms1505C => 0xFF6900,
            Self::Pms1505U => 0xFF8038,
            Self::Pms151C => 0xFF8200,
            Self::Pms151U => 0xFF883E,
            Self::Pms152C => 0xE57200,
            Self::Pms152U => 0xE07A3F,
            Self::Pms1525C => 0xB94700,
            Self::Pms1525U => 0xBD6439,
            Self::Pms153C => 0xBE6A14,
            Self::Pms153U => 0xAC6F42,
            Self::Pms1535C => 0x94450B,
            Self::Pms1535U => 0x976241,
            Self::Pms154C => 0x9B5A1A,
            Self::Pms154U => 0x926844,
            Self::Pms1545C => 0x653819,
            Self::Pms1545U => 0x7A5F48,
            Self::Pms155C => 0xEFD19F,
            Self::Pms155U => 0xF7C995,
            Self::Pms1555C => 0xFFB990,
            Self::Pms1555U => 0xFFC29E,
            Self::Pms156C => 0xEFBE7D,
            Self::Pms156U => 0xF7B984,
            Self::Pms1565C => 0xFFA06A,
            Self::Pms1565U => 0xFFAA81,
            Self::Pms157C => 0xECA154,
            Self::Pms157U => 0xF39662,
            Self::Pms1575C => 0xFF7F32,
            Self::Pms1575U => 0xFF9665,
            Self::Pms158C => 0xE87722,
            Self::Pms158U => 0xEE7F4B,
            Self::Pms1585C => 0xFF6A13,
            Self::Pms1585U => 0xFF8450,
            Self::Pms159C => 0xCB6015,
            Self::Pms159U => 0xC97146,
            Self::Pms1595C => 0xD86018,
            Self::Pms1595U => 0xD37046,
            Self::Pms160C => 0xA1561C,
            Self::Pms160U => 0x9D6B47,
            Self::Pms1605C => 0xA65523,
            Self::Pms1605U => 0xA76C4C,
            Self::Pms161C => 0x603D20,
            Self::Pms161U => 0x786249,
            Self::Pms1615C => 0x8B4720,
            Self::Pms1615U => 0x93654A,
            Self::Pms162C => 0xFFBE9F,
            Self::Pms162U => 0xFFBB9B,
            Self::Pms1625C => 0xFFA38B,
            Self::Pms1625U => 0xFFA891,
            Self::Pms163C => 0xFF9D6E,
            Self::Pms163U => 0xFF9C76,
            Self::Pms1635C => 0xFF8D6D,
            Self::Pms1635U => 0xFF967E,
            Self::Pms164C => 0xFF7F41,
            Self::Pms164U => 0xFF865F,
            Self::Pms1645C => 0xFF6A39,
            Self::Pms1645U => 0xFF7F61,
            Self::Pms165C => 0xFF671F,
            Self::Pms165U => 0xFF7548,
            Self::Pms1655C => 0xFC4C02,
            Self::Pms1655U => 0xFF6F4C,
            Self::Pms166C => 0xE35205,
            Self::Pms166U => 0xE16740,
            Self::Pms1665C => 0xDC4405,
            Self::Pms1665U => 0xE56849,
            Self::Pms167C => 0xBE531C,
            Self::Pms167U => 0xB46547,
            Self::Pms1675C => 0xA9431E,
            Self::Pms1675U => 0xAB614A,
            Self::Pms168C => 0x73381D,
            Self::Pms168U => 0x8A614B,
            Self::Pms1685C => 0x833921,
            Self::Pms1685U => 0x95614D,
            Self::Pms169C => 0xFFB3AB,
            Self::Pms169U => 0xFFB7AE,
            Self::Pms170C => 0xFF8674,
            Self::Pms170U => 0xFF8D7E,
            Self::Pms171C => 0xFF5C39,
            Self::Pms171U => 0xFF7865,
            Self::Pms172C => 0xFA4616,
            Self::Pms172U => 0xFF674D,
            Self::Pms173C => 0xCF4520,
            Self::Pms173U => 0xCE614A,
            Self::Pms174C => 0x963821,
            Self::Pms174U => 0x9C5B4B,
            Self::Pms175C => 0x6B3529,
            Self::Pms175U => 0x7D594D,
            Self::Pms176C => 0xFFB1BB,
            Self::Pms176U => 0xFFB1BA,
            Self::Pms1765C => 0xFFA3B5,
            Self::Pms1765U => 0xFFA7B6,
            Self::Pms1767C => 0xFCAFC0,
            Self::Pms1767U => 0xFFBAC9,
            Self::Pms177C => 0xFF808B,
            Self::Pms177U => 0xFF8C91,
            Self::Pms1775C => 0xFF8DA1,
            Self::Pms1775U => 0xFF8B9A,
            Self::Pms1777C => 0xFB637E,
            Self::Pms1777U => 0xFF8091,
            Self::Pms178C => 0xFF585D,
            Self::Pms178U => 0xFF7677,
            Self::Pms1785C => 0xF8485E,
            Self::Pms1785U => 0xFE6672,
            Self::Pms1787C => 0xF4364C,
            Self::Pms1787U => 0xFC6371,
            Self::Pms1788C => 0xEE2737,
            Self::Pms1788U => 0xF95D62,
            Self::Pms179C => 0xE03C31,
            Self::Pms179U => 0xE15F55,
            Self::Pms1795C => 0xD22630,
            Self::Pms1795U => 0xDA5959,
            Self::Pms1797C => 0xCB333B,
            Self::Pms1797U => 0xCB4F53,
            Self::Pms180C => 0xBE3A34,
            Self::Pms180U => 0xBF6059,
            Self::Pms1805C => 0xAF272F,
            Self::Pms1805U => 0xB45858,
            Self::Pms1807C => 0xA4343A,
            Self::Pms1807U => 0xA75154,
            Self::Pms181C => 0x81312F,
            Self::Pms181U => 0x8B5952,
            Self::Pms1815C => 0x7C2529,
            Self::Pms1815U => 0x8F5652,
            Self::Pms1817C => 0x643335,
            Self::Pms1817U => 0x795453,
            Self::Pms182C => 0xFABBCB,
            Self::Pms182U => 0xFFB8CA,
            Self::Pms183C => 0xFC9BB3,
            Self::Pms183U => 0xFF8DA3,
            Self::Pms184C => 0xF65275,
            Self::Pms184U => 0xFC6E83,
            Self::Pms185C => 0xE4002B,
            Self::Pms185U => 0xF15060,
            Self::Pms186C => 0xC8102E,
            Self::Pms186U => 0xD2515E,
            Self::Pms187C => 0xA6192E,
            Self::Pms187U => 0xAE535C,
            Self::Pms188C => 0x76232F,
            Self::Pms188U => 0x895559,
            Self::Pms189C => 0xF8A3BC,
            Self::Pms189U => 0xFFABC2,
            Self::Pms1895C => 0xF5B6CD,
            Self::Pms1895U => 0xFEB3CD,
            Self::Pms190C => 0xF67599,
            Self::Pms190U => 0xFE87A3,
            Self::Pms1905C => 0xF59BBB,
            Self::Pms1905U => 0xFD9CBA,
            Self::Pms191C => 0xEF426F,
            Self::Pms191U => 0xF87089,
            Self::Pms1915C => 0xEF4A81,
            Self::Pms1915U => 0xF66F92,
            Self::Pms192C => 0xE40046,
            Self::Pms192U => 0xEE536B,
            Self::Pms1925C => 0xE0004D,
            Self::Pms1925U => 0xE9506E,
            Self::Pms193C => 0xBF0D3E,
            Self::Pms193U => 0xC55265,
            Self::Pms1935C => 0xC5003E,
            Self::Pms1935U => 0xC94C63,
            Self::Pms194C => 0x9B2743,
            Self::Pms194U => 0xA35965,
            Self::Pms1945C => 0xA6093D,
            Self::Pms1945U => 0xB05468,
            Self::Pms195C => 0x782F40,
            Self::Pms195U => 0x885A61,
            Self::Pms1955C => 0x8A1538,
            Self::Pms1955U => 0x975361,
            Self::Pms196C => 0xECC7CD,
            Self::Pms196U => 0xF4C3CB,
            Self::Pms197C => 0xE89CAE,
            Self::Pms197U => 0xF2A1B2,
            Self::Pms198C => 0xDF4661,
            Self::Pms198U => 0xE76B7C,
            Self::Pms199C => 0xD50032,
            Self::Pms199U => 0xDD5061,
            Self::Pms200C => 0xBA0C2F,
            Self::Pms200U => 0xBD4F5C,
            Self::Pms201C => 0x9D2235,
            Self::Pms201U => 0xA15760,
            Self::Pms202C => 0x862633,
            Self::Pms202U => 0x90585E,
            Self::Pms203C => 0xECB3CB,
            Self::Pms203U => 0xF5B1CC,
            Self::Pms204C => 0xE782A9,
            Self::Pms204U => 0xEE82A4,
            Self::Pms205C => 0xE0457B,
            Self::Pms205U => 0xE56284,
            Self::Pms206C => 0xCE0037,
            Self::Pms206U => 0xD84465,
            Self::Pms207C => 0xA50034,
            Self::Pms207U => 0xAF4C64,
            Self::Pms208C => 0x861F41,
            Self::Pms208U => 0x905567,
            Self::Pms209C => 0x6F263D,
            Self::Pms209U => 0x825864,
            Self::Pms210C => 0xF99FC9,
            Self::Pms210U => 0xFFA2CB,
            Self::Pms211C => 0xF57EB6,
            Self::Pms211U => 0xFC85B6,
            Self::Pms212C => 0xF04E98,
            Self::Pms212U => 0xF56D9E,
            Self::Pms213C => 0xE31C79,
            Self::Pms213U => 0xE95485,
            Self::Pms214C => 0xCE0F69,
            Self::Pms214U => 0xD1517A,
            Self::Pms215C => 0xAC145A,
            Self::Pms215U => 0xAE5171,
            Self::Pms216C => 0x7D2248,
            Self::Pms216U => 0x8A5566,
            Self::Pms217C => 0xEABEDB,
            Self::Pms217U => 0xF4C1E1,
            Self::Pms218C => 0xE56DB1,
            Self::Pms218U => 0xEF84B9,
            Self::Pms219C => 0xDA1884,
            Self::Pms219U => 0xE45C96,
            Self::Pms220C => 0xA50050,
            Self::Pms220U => 0xB04C75,
            Self::Pms221C => 0x910048,
            Self::Pms221U => 0x9E4C6E,
            Self::Pms222C => 0x6C1D45,
            Self::Pms222U => 0x82566B,
            Self::Pms223C => 0xEF95CF,
            Self::Pms223U => 0xF9A4D8,
            Self::Pms224C => 0xEB6FBD,
            Self::Pms224U => 0xF587C7,
            Self::Pms225C => 0xDF1995,
            Self::Pms225U => 0xEA60A7,
            Self::Pms226C => 0xD0006F,
            Self::Pms226U => 0xDC4588,
            Self::Pms227C => 0xAA0061,
            Self::Pms227U => 0xAF4879,
            Self::Pms228C => 0x890C58,
            Self::Pms228U => 0x8D4D6D,
            Self::Pms229C => 0x672146,
            Self::Pms229U => 0x7A5466,
            Self::Pms230C => 0xF4A6D7,
            Self::Pms230U => 0xFEB2E0,
            Self::Pms231C => 0xF277C6,
            Self::Pms231U => 0xF984CA,
            Self::Pms232C => 0xE93CAC,
            Self::Pms232U => 0xF16AB7,
            Self::Pms233C => 0xC6007E,
            Self::Pms233U => 0xC84687,
            Self::Pms234C => 0xA20067,
            Self::Pms234U => 0xAA4C7D,
            Self::Pms235C => 0x840B55,
            Self::Pms235U => 0x914E72,
            Self::Pms236C => 0xF1A7DC,
            Self::Pms236U => 0xF8A6DE,
            Self::Pms2365C => 0xEFBAE1,
            Self::Pms2365U => 0xF8BFE7,
            Self::Pms237C => 0xEC86D0,
            Self::Pms237U => 0xF28DD2,
            Self::Pms2375C => 0xE277CD,
            Self::Pms2375U => 0xEA88D2,
            Self::Pms238C => 0xE45DBF,
            Self::Pms238U => 0xE971C0,
            Self::Pms2385C => 0xD539B5,
            Self::Pms2385U => 0xE06FC2,
            Self::Pms239C => 0xDB3EB1,
            Self::Pms239U => 0xDB55AA,
            Self::Pms2395C => 0xC800A1,
            Self::Pms2395U => 0xD557AF,
            Self::Pms240C => 0xC5299B,
            Self::Pms240U => 0xC15598,
            Self::Pms2405C => 0xB0008E,
            Self::Pms2405U => 0xBB569D,
            Self::Pms241C => 0xAF1685,
            Self::Pms241U => 0xAF548B,
            Self::Pms2415C => 0x9E007E,
            Self::Pms2415U => 0xAB5491,
            Self::Pms242C => 0x80225F,
            Self::Pms242U => 0x8D5977,
            Self::Pms2425C => 0x830065,
            Self::Pms2425U => 0x975381,
            Self::Pms243C => 0xEAB8E4,
            Self::Pms243U => 0xF1B2E6,
            Self::Pms244C => 0xE59BDC,
            Self::Pms244U => 0xEB9ADE,
            Self::Pms245C => 0xDD7FD3,
            Self::Pms245U => 0xE07FD0,
            Self::Pms246C => 0xC724B1,
            Self::Pms246U => 0xCE57B3,
            Self::Pms247C => 0xBB16A3,
            Self::Pms247U => 0xBD51A2,
            Self::Pms248C => 0xA51890,
            Self::Pms248U => 0xA14F8C,
            Self::Pms249C => 0x80276C,
            Self::Pms249U => 0x885779,
            Self::Pms250C => 0xE7BAE4,
            Self::Pms250U => 0xEEBAE9,
            Self::Pms251C => 0xDD9CDF,
            Self::Pms251U => 0xE096DF,
            Self::Pms252C => 0xC964CF,
            Self::Pms252U => 0xCE70CC,
            Self::Pms253C => 0xAD1AAC,
            Self::Pms253U => 0xB456AE,
            Self::Pms254C => 0x981D97,
            Self::Pms254U => 0x9E549A,
            Self::Pms255C => 0x72246C,
            Self::Pms255U => 0x83577D,
            Self::Pms256C => 0xD6BFDD,
            Self::Pms256U => 0xD4B4DA,
            Self::Pms2562C => 0xD7A9E3,
            Self::Pms2562U => 0xDDAEE9,
            Self::Pms2563C => 0xCBA3D8,
            Self::Pms2563U => 0xCDA2D9,
            Self::Pms2567C => 0xC1A0DA,
            Self::Pms2567U => 0xC09FDB,
            Self::Pms257C => 0xC6A1CF,
            Self::Pms257U => 0xC29BCB,
            Self::Pms2572C => 0xC98BDB,
            Self::Pms2572U => 0xCD90DE,
            Self::Pms2573C => 0xB884CB,
            Self::Pms2573U => 0xB989CB,
            Self::Pms2577C => 0xA77BCA,
            Self::Pms2577U => 0xA47EC6,
            Self::Pms258C => 0x8C4799,
            Self::Pms258U => 0xA276AE,
            Self::Pms2582C => 0xAC4FC6,
            Self::Pms2582U => 0xBB76CF,
            Self::Pms2583C => 0xA05EB5,
            Self::Pms2583U => 0xA372B5,
            Self::Pms2587C => 0x8246AF,
            Self::Pms2587U => 0x936DB7,
            Self::Pms259C => 0x6D2077,
            Self::Pms259U => 0x94679C,
            Self::Pms2592C => 0x9B26B6,
            Self::Pms2592U => 0xAA60BF,
            Self::Pms2593C => 0x84329B,
            Self::Pms2593U => 0x8E5DA0,
            Self::Pms2597C => 0x5C068C,
            Self::Pms2597U => 0x825AA4,
            Self::Pms260C => 0x642667,
            Self::Pms260U => 0x8A658D,
            Self::Pms2602C => 0x87189D,
            Self::Pms2602U => 0x9757A6,
            Self::Pms2603C => 0x702082,
            Self::Pms2603U => 0x845991,
            Self::Pms2607C => 0x500778,
            Self::Pms2607U => 0x7A5896,
            Self::Pms261C => 0x5D285F,
            Self::Pms261U => 0x7F6282,
            Self::Pms2612C => 0x772583,
            Self::Pms2612U => 0x86578D,
            Self::Pms2613C => 0x671E75,
            Self::Pms2613U => 0x7D5787,
            Self::Pms2617C => 0x470A68,
            Self::Pms2617U => 0x735489,
            Self::Pms262C => 0x51284F,
            Self::Pms262U => 0x755E74,
            Self::Pms2622C => 0x653165,
            Self::Pms2622U => 0x7B5B7A,
            Self::Pms2623C => 0x5F2167,
            Self::Pms2623U => 0x75557A,
            Self::Pms2627C => 0x3C1053,
            Self::Pms2627U => 0x6E527C,
            Self::Pms263C => 0xD7C6E6,
            Self::Pms263U => 0xDCC6EA,
            Self::Pms2635C => 0xC5B4E3,
            Self::Pms2635U => 0xC9B7E8,
            Self::Pms264C => 0xC1A7E2,
            Self::Pms264U => 0xBFA1E3,
            Self::Pms2645C => 0xAD96DC,
            Self::Pms2645U => 0xB9A3E3,
            Self::Pms265C => 0x9063CD,
            Self::Pms265U => 0x9D7AD2,
            Self::Pms2655C => 0x9678D3,
            Self::Pms2655U => 0xA288D9,
            Self::Pms266C => 0x753BBD,
            Self::Pms266U => 0x8A65C4,
            Self::Pms2665C => 0x7D55C7,
            Self::Pms2665U => 0x8C6FCA,
            Self::Pms267C => 0x5F259F,
            Self::Pms267U => 0x7F5DAE,
            Self::Pms268C => 0x582C83,
            Self::Pms268U => 0x735990,
            Self::Pms2685C => 0x330072,
            Self::Pms2685U => 0x765BA7,
            Self::Pms269C => 0x512D6D,
            Self::Pms269U => 0x6E587F,
            Self::Pms2695C => 0x2E1A47,
            Self::Pms2695U => 0x6C5D80,
            Self::Pms270C => 0xB4B5DF,
            Self::Pms270U => 0xADAEDE,
            Self::Pms2705C => 0xA7A4E0,
            Self::Pms2705U => 0xBAB6E9,
            Self::Pms2706C => 0xCBD3EB,
            Self::Pms2706U => 0xCCD4F1,
            Self::Pms2707C => 0xC3D7EE,
            Self::Pms2707U => 0xC7DBF4,
            Self::Pms2708C => 0xB8CCEA,
            Self::Pms2708U => 0xB2C9EF,
            Self::Pms271C => 0x9595D2,
            Self::Pms271U => 0x9A9BD4,
            Self::Pms2715C => 0x8B84D7,
            Self::Pms2715U => 0x9F98DF,
            Self::Pms2716C => 0x9FAEE5,
            Self::Pms2716U => 0xA2AFE9,
            Self::Pms2717C => 0xA7C6ED,
            Self::Pms2717U => 0xA9C8F4,
            Self::Pms2718C => 0x5C88DA,
            Self::Pms2718U => 0x6A87D8,
            Self::Pms272C => 0x7474C1,
            Self::Pms272U => 0x8586C6,
            Self::Pms2725C => 0x685BC7,
            Self::Pms2725U => 0x8980D4,
            Self::Pms2726C => 0x485CC7,
            Self::Pms2726U => 0x717ACF,
            Self::Pms2727C => 0x307FE2,
            Self::Pms2727U => 0x6490E8,
            Self::Pms2728C => 0x0047BB,
            Self::Pms2728U => 0x5268C2,
            Self::Pms273C => 0x24135F,
            Self::Pms273U => 0x645D9B,
            Self::Pms2735C => 0x2E008B,
            Self::Pms2735U => 0x6857B2,
            Self::Pms2736C => 0x1E22AA,
            Self::Pms2736U => 0x5E60BA,
            Self::Pms2738C => 0x06038D,
            Self::Pms2738U => 0x474DA4,
            Self::Pms274C => 0x211551,
            Self::Pms274U => 0x5F578B,
            Self::Pms2745C => 0x280071,
            Self::Pms2745U => 0x61559A,
            Self::Pms2746C => 0x171C8F,
            Self::Pms2746U => 0x595CA5,
            Self::Pms2747C => 0x001A72,
            Self::Pms2747U => 0x424F90,
            Self::Pms2748C => 0x001871,
            Self::Pms2748U => 0x484F8C,
            Self::Pms275C => 0x201747,
            Self::Pms275U => 0x5D5683,
            Self::Pms2755C => 0x250E62,
            Self::Pms2755U => 0x5D538B,
            Self::Pms2756C => 0x151F6D,
            Self::Pms2756U => 0x585C8C,
            Self::Pms2757C => 0x001E60,
            Self::Pms2757U => 0x454F7E,
            Self::Pms2758C => 0x001E62,
            Self::Pms2758U => 0x484D7A,
            Self::Pms276C => 0x221C35,
            Self::Pms276U => 0x5C556F,
            Self::Pms2765C => 0x201547,
            Self::Pms2765U => 0x574F77,
            Self::Pms2766C => 0x141B4D,
            Self::Pms2766U => 0x575A7C,
            Self::Pms2767C => 0x13294B,
            Self::Pms2767U => 0x4A4C64,
            Self::Pms2768C => 0x071D49,
            Self::Pms2768U => 0x4A4E6D,
            Self::Pms277C => 0xABCAE9,
            Self::Pms277U => 0xADCFF1,
            Self::Pms278C => 0x8BB8E8,
            Self::Pms278U => 0x8DBAEE,
            Self::Pms279C => 0x418FDE,
            Self::Pms279U => 0x5B8EDB,
            Self::Pms280C => 0x012169,
            Self::Pms280U => 0x3C4981,
            Self::Pms281C => 0x00205B,
            Self::Pms281U => 0x3F4A75,
            Self::Pms282C => 0x041E42,
            Self::Pms282U => 0x434967,
            Self::Pms283C => 0x92C1E9,
            Self::Pms283U => 0x9BC7EE,
            Self::Pms284C => 0x6CACE4,
            Self::Pms284U => 0x7AB1E8,
            Self::Pms285C => 0x0072CE,
            Self::Pms285U => 0x4982CF,
            Self::Pms286C => 0x0033A0,
            Self::Pms286U => 0x3255A4,
            Self::Pms287C => 0x003087,
            Self::Pms287U => 0x3C5896,
            Self::Pms288C => 0x002D72,
            Self::Pms288U => 0x3D5588,
            Self::Pms289C => 0x0C2340,
            Self::Pms289U => 0x434F69,
            Self::Pms290C => 0xB9D9EB,
            Self::Pms290U => 0xA7D4EE,
            Self::Pms2905C => 0x8DC8E8,
            Self::Pms2905U => 0x8AC9ED,
            Self::Pms291C => 0x9BCBEB,
            Self::Pms291U => 0x7FBEEB,
            Self::Pms2915C => 0x62B5E5,
            Self::Pms2915U => 0x64B3E8,
            Self::Pms292C => 0x69B3E7,
            Self::Pms292U => 0x62A8E5,
            Self::Pms2925C => 0x009CDE,
            Self::Pms2925U => 0x4097DB,
            Self::Pms293C => 0x003DA5,
            Self::Pms293U => 0x235BA8,
            Self::Pms2935C => 0x0057B8,
            Self::Pms2935U => 0x1B5FAA,
            Self::Pms294C => 0x002F6C,
            Self::Pms294U => 0x385988,
            Self::Pms2945C => 0x004C97,
            Self::Pms2945U => 0x29588C,
            Self::Pms295C => 0x002855,
            Self::Pms295U => 0x375172,
            Self::Pms2955C => 0x003865,
            Self::Pms2955U => 0x395775,
            Self::Pms296C => 0x041C2C,
            Self::Pms296U => 0x435060,
            Self::Pms2965C => 0x00263E,
            Self::Pms2965U => 0x3D5265,
            Self::Pms297C => 0x71C5E8,
            Self::Pms297U => 0x7ECCEE,
            Self::Pms2975C => 0x99D6EA,
            Self::Pms2975U => 0x8BD5EE,
            Self::Pms298C => 0x41B6E6,
            Self::Pms298U => 0x4CB4E7,
            Self::Pms2985C => 0x5BC2E7,
            Self::Pms2985U => 0x48B8E7,
            Self::Pms299C => 0x00A3E0,
            Self::Pms299U => 0x1295D8,
            Self::Pms2995C => 0x00A9E0,
            Self::Pms2995U => 0x0D9DDB,
            Self::Pms300C => 0x005EB8,
            Self::Pms300U => 0x006EB6,
            Self::Pms3005C => 0x0077C8,
            Self::Pms3005U => 0x0078BF,
            Self::Pms301C => 0x004B87,
            Self::Pms301U => 0x28628E,
            Self::Pms3015C => 0x00629B,
            Self::Pms3015U => 0x216A95,
            Self::Pms302C => 0x003B5C,
            Self::Pms302U => 0x375E77,
            Self::Pms3025C => 0x004F71,
            Self::Pms3025U => 0x37617A,
            Self::Pms303C => 0x002A3A,
            Self::Pms303U => 0x3F5666,
            Self::Pms3035C => 0x003E51,
            Self::Pms3035U => 0x405866,
            Self::Pms304C => 0x9ADBE8,
            Self::Pms304U => 0x8EDDED,
            Self::Pms305C => 0x59CBE8,
            Self::Pms305U => 0x53CAEC,
            Self::Pms306C => 0x00B5E2,
            Self::Pms306U => 0x00B4E4,
            Self::Pms307C => 0x006BA6,
            Self::Pms307U => 0x0074A2,
            Self::Pms308C => 0x00587C,
            Self::Pms308U => 0x266782,
            Self::Pms309C => 0x003B49,
            Self::Pms309U => 0x3F5864,
            Self::Pms310C => 0x6AD1E3,
            Self::Pms310U => 0x5BD0E6,
            Self::Pms3105C => 0x68D2DF,
            Self::Pms3105U => 0x4CCEDE,
            Self::Pms311C => 0x05C3DE,
            Self::Pms311U => 0x01BEDB,
            Self::Pms3115C => 0x00C1D5,
            Self::Pms3115U => 0x00BFD3,
            Self::Pms312C => 0x00A9CE,
            Self::Pms312U => 0x00AFD0,
            Self::Pms3125C => 0x00AEC7,
            Self::Pms3125U => 0x00ABC2,
            Self::Pms313C => 0x0092BC,
            Self::Pms313U => 0x008DB1,
            Self::Pms3135C => 0x008EAA,
            Self::Pms3135U => 0x008EA6,
            Self::Pms314C => 0x007FA3,
            Self::Pms314U => 0x007D99,
            Self::Pms3145C => 0x00778B,
            Self::Pms3145U => 0x007788,
            Self::Pms315C => 0x00677F,
            Self::Pms315U => 0x206779,
            Self::Pms3155C => 0x006272,
            Self::Pms3155U => 0x1B6875,
            Self::Pms316C => 0x004851,
            Self::Pms316U => 0x3E5A62,
            Self::Pms3165C => 0x004F59,
            Self::Pms3165U => 0x2F5C66,
            Self::Pms317C => 0xB1E4E3,
            Self::Pms317U => 0xAEE9E8,
            Self::Pms318C => 0x88DBDF,
            Self::Pms318U => 0x6BD9DE,
            Self::Pms319C => 0x2DCCD3,
            Self::Pms319U => 0x25C9D0,
            Self::Pms320C => 0x009CA6,
            Self::Pms320U => 0x009DA5,
            Self::Pms321C => 0x008C95,
            Self::Pms321U => 0x00838A,
            Self::Pms322C => 0x007377,
            Self::Pms322U => 0x167378,
            Self::Pms323C => 0x005F61,
            Self::Pms323U => 0x2F6165,
            Self::Pms324C => 0x9CDBD9,
            Self::Pms324U => 0x82D8D5,
            Self::Pms3242C => 0x71DBD4,
            Self::Pms3242U => 0x5DDBD3,
            Self::Pms3245C => 0x7CE0D3,
            Self::Pms3245U => 0x4EDCCA,
            Self::Pms3248C => 0x6DCDB8,
            Self::Pms3248U => 0x6ACEB9,
            Self::Pms325C => 0x64CCC9,
            Self::Pms325U => 0x40C1BB,
            Self::Pms3252C => 0x2AD2C9,
            Self::Pms3252U => 0x00CDC2,
            Self::Pms3255C => 0x2CD5C4,
            Self::Pms3255U => 0x00CFBB,
            Self::Pms3258C => 0x49C5B1,
            Self::Pms3258U => 0x48C1AC,
            Self::Pms326C => 0x00B2A9,
            Self::Pms326U => 0x00AAA1,
            Self::Pms3262C => 0x00BFB3,
            Self::Pms3262U => 0x00BAAE,
            Self::Pms3265C => 0x00C7B1,
            Self::Pms3265U => 0x00C0AA,
            Self::Pms3268C => 0x00AB8E,
            Self::Pms3268U => 0x28B29A,
            Self::Pms327C => 0x008675,
            Self::Pms327U => 0x008F85,
            Self::Pms3272C => 0x00A499,
            Self::Pms3272U => 0x00A49A,
            Self::Pms3275C => 0x00B398,
            Self::Pms3275U => 0x00AA93,
            Self::Pms3278C => 0x009B77,
            Self::Pms3278U => 0x009C82,
            Self::Pms328C => 0x007367,
            Self::Pms328U => 0x137D79,
            Self::Pms3282C => 0x008578,
            Self::Pms3282U => 0x008780,
            Self::Pms3285C => 0x009681,
            Self::Pms3285U => 0x009081,
            Self::Pms3288C => 0x008264,
            Self::Pms3288U => 0x208776,
            Self::Pms329C => 0x00685E,
            Self::Pms329U => 0x2E7572,
            Self::Pms3292C => 0x00594F,
            Self::Pms3292U => 0x346E6A,
            Self::Pms3295C => 0x007864,
            Self::Pms3295U => 0x237E74,
            Self::Pms3298C => 0x006A52,
            Self::Pms3298U => 0x387469,
            Self::Pms330C => 0x00534C,
            Self::Pms330U => 0x446463,
            Self::Pms3302C => 0x004C45,
            Self::Pms3302U => 0x41615F,
            Self::Pms3305C => 0x004E42,
            Self::Pms3305U => 0x466964,
            Self::Pms3308C => 0x034638,
            Self::Pms3308U => 0x48615B,
            Self::Pms331C => 0xA7E6D7,
            Self::Pms331U => 0x93E8D3,
            Self::Pms332C => 0x8CE2D0,
            Self::Pms332U => 0x70E2CB,
            Self::Pms333C => 0x3CDBC0,
            Self::Pms333U => 0x00D2B3,
            Self::Pms334C => 0x009775,
            Self::Pms334U => 0x00957E,
            Self::Pms335C => 0x007B5F,
            Self::Pms335U => 0x247E6F,
            Self::Pms336C => 0x00664F,
            Self::Pms336U => 0x3A7267,
            Self::Pms337C => 0x8FD6BD,
            Self::Pms337U => 0x82D5BA,
            Self::Pms3375C => 0x7AE1BF,
            Self::Pms3375U => 0x5FE0B7,
            Self::Pms338C => 0x6ECEB2,
            Self::Pms338U => 0x4CC1A1,
            Self::Pms3385C => 0x47D7AC,
            Self::Pms3385U => 0x3AD4A7,
            Self::Pms339C => 0x00B388,
            Self::Pms339U => 0x22AE8A,
            Self::Pms3395C => 0x00C389,
            Self::Pms3395U => 0x00C795,
            Self::Pms340C => 0x00965E,
            Self::Pms340U => 0x009E77,
            Self::Pms3405C => 0x00AF66,
            Self::Pms3405U => 0x00B27A,
            Self::Pms341C => 0x007A53,
            Self::Pms341U => 0x347963,
            Self::Pms3415C => 0x007749,
            Self::Pms3415U => 0x308167,
            Self::Pms342C => 0x006747,
            Self::Pms342U => 0x407060,
            Self::Pms3425C => 0x006341,
            Self::Pms3425U => 0x417764,
            Self::Pms343C => 0x115740,
            Self::Pms343U => 0x48655B,
            Self::Pms3435C => 0x154734,
            Self::Pms3435U => 0x4B675B,
            Self::Pms344C => 0xA0DAB3,
            Self::Pms344U => 0x8ED8AC,
            Self::Pms345C => 0x91D6AC,
            Self::Pms345U => 0x6ECD9C,
            Self::Pms346C => 0x71CC98,
            Self::Pms346U => 0x4EBD88,
            Self::Pms347C => 0x009A44,
            Self::Pms347U => 0x169B62,
            Self::Pms348C => 0x00843D,
            Self::Pms348U => 0x34855B,
            Self::Pms349C => 0x046A38,
            Self::Pms349U => 0x477258,
            Self::Pms350C => 0x2C5234,
            Self::Pms350U => 0x546758,
            Self::Pms351C => 0xA2E4B8,
            Self::Pms351U => 0x93E6B4,
            Self::Pms352C => 0x8FE2B0,
            Self::Pms352U => 0x70E0A3,
            Self::Pms353C => 0x80E0A7,
            Self::Pms353U => 0x50D691,
            Self::Pms354C => 0x00B140,
            Self::Pms354U => 0x00A95C,
            Self::Pms355C => 0x009639,
            Self::Pms355U => 0x19975D,
            Self::Pms356C => 0x007A33,
            Self::Pms356U => 0x397E58,
            Self::Pms357C => 0x215732,
            Self::Pms357U => 0x516E5A,
            Self::Pms358C => 0xADDC91,
            Self::Pms358U => 0x97D88A,
            Self::Pms359C => 0xA1D884,
            Self::Pms359U => 0x7DCC77,
            Self::Pms360C => 0x6CC24A,
            Self::Pms360U => 0x5DB860,
            Self::Pms361C => 0x43B02A,
            Self::Pms361U => 0x4DAA50,
            Self::Pms362C => 0x509E2F,
            Self::Pms362U => 0x56944F,
            Self::Pms363C => 0x4C8C2B,
            Self::Pms363U => 0x57854E,
            Self::Pms364C => 0x4A7729,
            Self::Pms364U => 0x5B794E,
            Self::Pms365C => 0xC2E189,
            Self::Pms365U => 0xC2E692,
            Self::Pms366C => 0xB7DD79,
            Self::Pms366U => 0xA8DC7D,
            Self::Pms367C => 0xA4D65E,
            Self::Pms367U => 0x8FD16A,
            Self::Pms368C => 0x78BE20,
            Self::Pms368U => 0x67B346,
            Self::Pms369C => 0x64A70B,
            Self::Pms369U => 0x62A044,
            Self::Pms370C => 0x658D1B,
            Self::Pms370U => 0x698A48,
            Self::Pms371C => 0x546223,
            Self::Pms371U => 0x68724D,
            Self::Pms372C => 0xD4EB8E,
            Self::Pms372U => 0xD1F08F,
            Self::Pms373C => 0xCDEA80,
            Self::Pms373U => 0xB9E972,
            Self::Pms374C => 0xC5E86C,
            Self::Pms374U => 0xA6E35F,
            Self::Pms375C => 0x97D700,
            Self::Pms375U => 0x73C92D,
            Self::Pms376C => 0x84BD00,
            Self::Pms376U => 0x70AB37,
            Self::Pms377C => 0x7A9A01,
            Self::Pms377U => 0x728F41,
            Self::Pms378C => 0x59621D,
            Self::Pms378U => 0x747B4D,
            Self::Pms379C => 0xE2E868,
            Self::Pms379U => 0xE7EE7B,
            Self::Pms380C => 0xDBE442,
            Self::Pms380U => 0xD4E458,
            Self::Pms381C => 0xCEDC00,
            Self::Pms381U => 0xBAD739,
            Self::Pms382C => 0xC4D600,
            Self::Pms382U => 0x99C221,
            Self::Pms383C => 0xA8AD00,
            Self::Pms383U => 0x8E9A36,
            Self::Pms384C => 0x949300,
            Self::Pms384U => 0x868B3C,
            Self::Pms385C => 0x787121,
            Self::Pms385U => 0x7E7B4B,
            Self::Pms386C => 0xE9EC6B,
            Self::Pms386U => 0xEFF178,
            Self::Pms387C => 0xE3E935,
            Self::Pms387U => 0xE3ED55,
            Self::Pms388C => 0xE0E721,
            Self::Pms388U => 0xD5E739,
            Self::Pms389C => 0xD0DF00,
            Self::Pms389U => 0xBDDC04,
            Self::Pms390C => 0xB5BD00,
            Self::Pms390U => 0x97A926,
            Self::Pms391C => 0x9A9500,
            Self::Pms391U => 0x888A3B,
            Self::Pms392C => 0x827A04,
            Self::Pms392U => 0x7E7D41,
            Self::Pms393C => 0xF0EC74,
            Self::Pms393U => 0xF8F283,
            Self::Pms3935C => 0xF3EA5D,
            Self::Pms3935U => 0xFCF389,
            Self::Pms394C => 0xEDE939,
            Self::Pms394U => 0xF2EE56,
            Self::Pms3945C => 0xF3E500,
            Self::Pms3945U => 0xFCF05F,
            Self::Pms395C => 0xECE81A,
            Self::Pms395U => 0xEDEC3F,
            Self::Pms3955C => 0xEFDF00,
            Self::Pms3955U => 0xFAEB37,
            Self::Pms396C => 0xE1E000,
            Self::Pms396U => 0xDBE200,
            Self::Pms3965C => 0xEEDC00,
            Self::Pms3965U => 0xF8E815,
            Self::Pms397C => 0xBFB800,
            Self::Pms397U => 0xA8AA31,
            Self::Pms3975C => 0xBBA600,
            Self::Pms3975U => 0xB6A72D,
            Self::Pms398C => 0xADA400,
            Self::Pms398U => 0x959539,
            Self::Pms3985C => 0x9A8700,
            Self::Pms3985U => 0x9C903F,
            Self::Pms399C => 0xA09200,
            Self::Pms399U => 0x8A8537,
            Self::Pms3995C => 0x685C20,
            Self::Pms3995U => 0x837B4A,
            Self::Pms400C => 0xC4BFB6,
            Self::Pms400U => 0xC7C1BA,
            Self::Pms401C => 0xAFA9A0,
            Self::Pms401U => 0xB4AEA7,
            Self::Pms402C => 0x9D968D,
            Self::Pms402U => 0xA39E98,
            Self::Pms403C => 0x8C857B,
            Self::Pms403U => 0x928D88,
            Self::Pms404C => 0x776E64,
            Self::Pms404U => 0x8B8681,
            Self::Pms405C => 0x696158,
            Self::Pms405U => 0x77726D,
            Self::Pms406C => 0xC4BCB7,
            Self::Pms406U => 0xC7BEBA,
            Self::Pms407C => 0xB2A8A2,
            Self::Pms407U => 0xABA2A0,
            Self::Pms408C => 0x978C87,
            Self::Pms408U => 0x9D9594,
            Self::Pms409C => 0x857874,
            Self::Pms409U => 0x948B8A,
            Self::Pms410C => 0x746661,
            Self::Pms410U => 0x8A8280,
            Self::Pms411C => 0x5E514D,
            Self::Pms411U => 0x7A726F,
            Self::Pms412C => 0x382F2D,
            Self::Pms412U => 0x645C59,
            Self::Pms413C => 0xBABBB1,
            Self::Pms413U => 0xBCBDB4,
            Self::Pms414C => 0xA8A99E,
            Self::Pms414U => 0xABACA5,
            Self::Pms415C => 0x919388,
            Self::Pms415U => 0x9D9F99,
            Self::Pms416C => 0x7E7F74,
            Self::Pms416U => 0x91928C,
            Self::Pms417C => 0x65665C,
            Self::Pms417U => 0x82847F,
            Self::Pms418C => 0x51534A,
            Self::Pms418U => 0x7A7B75,
            Self::Pms419C => 0x212322,
            Self::Pms419U => 0x595955,
            Self::Pms420C => 0xC7C9C7,
            Self::Pms420U => 0xBDBEBD,
            Self::Pms421C => 0xB2B4B2,
            Self::Pms421U => 0xB3B5B5,
            Self::Pms422C => 0x9EA2A2,
            Self::Pms422U => 0x9EA1A2,
            Self::Pms423C => 0x898D8D,
            Self::Pms423U => 0x909496,
            Self::Pms424C => 0x707372,
            Self::Pms424U => 0x88898A,
            Self::Pms425C => 0x54585A,
            Self::Pms425U => 0x7C7E7F,
            Self::Pms426C => 0x25282A,
            Self::Pms426U => 0x686869,
            Self::Pms427C => 0xD0D3D4,
            Self::Pms427U => 0xC2C7CA,
            Self::Pms428C => 0xC1C6C8,
            Self::Pms428U => 0xACB3B8,
            Self::Pms429C => 0xA2AAAD,
            Self::Pms429U => 0x91999F,
            Self::Pms430C => 0x7C878E,
            Self::Pms430U => 0x808991,
            Self::Pms431C => 0x5B6770,
            Self::Pms431U => 0x737B82,
            Self::Pms432C => 0x333F48,
            Self::Pms432U => 0x676C73,
            Self::Pms433C => 0x1D252D,
            Self::Pms433U => 0x5B5D62,
            Self::Pms434C => 0xD0C4C5,
            Self::Pms434U => 0xD6CACB,
            Self::Pms435C => 0xC1B2B6,
            Self::Pms435U => 0xB7A9AD,
            Self::Pms436C => 0xAB989D,
            Self::Pms436U => 0xA99B9F,
            Self::Pms437C => 0x7B6469,
            Self::Pms437U => 0x8D7F83,
            Self::Pms438C => 0x584446,
            Self::Pms438U => 0x817275,
            Self::Pms439C => 0x453536,
            Self::Pms439U => 0x736869,
            Self::Pms440C => 0x382E2C,
            Self::Pms440U => 0x69615F,
            Self::Pms441C => 0xBEC6C4,
            Self::Pms441U => 0xB9C2C1,
            Self::Pms442C => 0xA2ACAB,
            Self::Pms442U => 0xACB6B6,
            Self::Pms443C => 0x919D9D,
            Self::Pms443U => 0x8F9A9B,
            Self::Pms444C => 0x717C7D,
            Self::Pms444U => 0x7E8689,
            Self::Pms445C => 0x505759,
            Self::Pms445U => 0x6B7173,
            Self::Pms446C => 0x3F4444,
            Self::Pms446U => 0x636968,
            Self::Pms447C => 0x373A36,
            Self::Pms447U => 0x5F625F,
            Self::Pms448C => 0x4A412A,
            Self::Pms448U => 0x66614E,
            Self::Pms4485C => 0x614F25,
            Self::Pms4485U => 0x7B6F4B,
            Self::Pms449C => 0x524727,
            Self::Pms449U => 0x716B54,
            Self::Pms4495C => 0x8C7732,
            Self::Pms4495U => 0x90845F,
            Self::Pms450C => 0x594A25,
            Self::Pms450U => 0x7E785A,
            Self::Pms4505C => 0x998542,
            Self::Pms4505U => 0xA39771,
            Self::Pms451C => 0x9B945F,
            Self::Pms451U => 0x929172,
            Self::Pms4515C => 0xB3A369,
            Self::Pms4515U => 0xB1A57E,
            Self::Pms452C => 0xB0AA7E,
            Self::Pms452U => 0xA8A687,
            Self::Pms4525C => 0xC5B783,
            Self::Pms4525U => 0xC1B58C,
            Self::Pms453C => 0xBFBB98,
            Self::Pms453U => 0xBCB999,
            Self::Pms4535C => 0xCFC493,
            Self::Pms4535U => 0xCABE96,
            Self::Pms454C => 0xCAC7A7,
            Self::Pms454U => 0xCAC8AC,
            Self::Pms4545C => 0xD5CB9F,
            Self::Pms4545U => 0xD6CCA6,
            Self::Pms455C => 0x695B24,
            Self::Pms455U => 0x776F48,
            Self::Pms456C => 0xA28E2A,
            Self::Pms456U => 0x908145,
            Self::Pms457C => 0xB89D18,
            Self::Pms457U => 0xAD9844,
            Self::Pms458C => 0xD9C756,
            Self::Pms458U => 0xCEB964,
            Self::Pms459C => 0xDECD63,
            Self::Pms459U => 0xDDCB77,
            Self::Pms460C => 0xE4D77E,
            Self::Pms460U => 0xE8D98B,
            Self::Pms461C => 0xE9DF97,
            Self::Pms461U => 0xEEE29F,
            Self::Pms462C => 0x5C462B,
            Self::Pms462U => 0x70624D,
            Self::Pms4625C => 0x4F2C1D,
            Self::Pms4625U => 0x715845,
            Self::Pms463C => 0x744F28,
            Self::Pms463U => 0x856F51,
            Self::Pms4635C => 0x946037,
            Self::Pms4635U => 0x92715E,
            Self::Pms464C => 0x8B5B29,
            Self::Pms464U => 0x9B7C56,
            Self::Pms4645C => 0xAD7C59,
            Self::Pms4645U => 0xA07D6B,
            Self::Pms465C => 0xB9975B,
            Self::Pms465U => 0xB0946F,
            Self::Pms4655C => 0xBF9474,
            Self::Pms4655U => 0xB28E7B,
            Self::Pms466C => 0xC6AA76,
            Self::Pms466U => 0xC1A67F,
            Self::Pms4665C => 0xCDA788,
            Self::Pms4665U => 0xCAA58E,
            Self::Pms467C => 0xD3BC8D,
            Self::Pms467U => 0xD2BA92,
            Self::Pms4675C => 0xDCBFA6,
            Self::Pms4675U => 0xDAB9A1,
            Self::Pms468C => 0xDDCBA4,
            Self::Pms468U => 0xE0CEAA,
            Self::Pms4685C => 0xE0C6AD,
            Self::Pms4685U => 0xE6CAB4,
            Self::Pms469C => 0x693F23,
            Self::Pms469U => 0x7B604B,
            Self::Pms4695C => 0x5B3427,
            Self::Pms4695U => 0x775C50,
            Self::Pms470C => 0xA45A2A,
            Self::Pms470U => 0xA37052,
            Self::Pms4705C => 0x7C4D3A,
            Self::Pms4705U => 0x8B6F66,
            Self::Pms471C => 0xB86125,
            Self::Pms471U => 0xBE7B54,
            Self::Pms4715C => 0x956C58,
            Self::Pms4715U => 0x997D73,
            Self::Pms472C => 0xE59E6D,
            Self::Pms472U => 0xDB9271,
            Self::Pms4725C => 0xAE8A79,
            Self::Pms4725U => 0xAE9188,
            Self::Pms473C => 0xF0BF9B,
            Self::Pms473U => 0xEEAB88,
            Self::Pms4735C => 0xC0A392,
            Self::Pms4735U => 0xBDA195,
            Self::Pms474C => 0xF1C6A7,
            Self::Pms474U => 0xF6BD9D,
            Self::Pms4745C => 0xCDB5A7,
            Self::Pms4745U => 0xCEB4A9,
            Self::Pms475C => 0xF3CFB3,
            Self::Pms475U => 0xFACCAD,
            Self::Pms4755C => 0xD7C4B7,
            Self::Pms4755U => 0xDCC6BB,
            Self::Pms476C => 0x4E3629,
            Self::Pms476U => 0x6E5E52,
            Self::Pms477C => 0x623B2A,
            Self::Pms477U => 0x7D6556,
            Self::Pms478C => 0x703F2A,
            Self::Pms478U => 0x8E6E5C,
            Self::Pms479C => 0xAA8066,
            Self::Pms479U => 0xAC8C7E,
            Self::Pms480C => 0xC6A992,
            Self::Pms480U => 0xC3A492,
            Self::Pms481C => 0xD3BBA8,
            Self::Pms481U => 0xD3B7A7,
            Self::Pms482C => 0xDBC8B6,
            Self::Pms482U => 0xDFC8B7,
            Self::Pms483C => 0x653024,
            Self::Pms483U => 0x7D5B4F,
            Self::Pms484C => 0x9A3324,
            Self::Pms484U => 0xA05B4E,
            Self::Pms485C => 0xDA291C,
            Self::Pms485U => 0xE45D50,
            Self::Pms486C => 0xE8927C,
            Self::Pms486U => 0xF08D80,
            Self::Pms487C => 0xEAA794,
            Self::Pms487U => 0xF3A293,
            Self::Pms488C => 0xECBAA8,
            Self::Pms488U => 0xF4B9A9,
            Self::Pms489C => 0xECC3B2,
            Self::Pms489U => 0xF5C7B8,
            Self::Pms490C => 0x5D2A2C,
            Self::Pms490U => 0x775554,
            Self::Pms491C => 0x7F3035,
            Self::Pms491U => 0x8E595A,
            Self::Pms492C => 0x8F3237,
            Self::Pms492U => 0x9D5D60,
            Self::Pms493C => 0xDC8699,
            Self::Pms493U => 0xC77C89,
            Self::Pms494C => 0xE9A2B2,
            Self::Pms494U => 0xE59AA8,
            Self::Pms495C => 0xF1BDC8,
            Self::Pms495U => 0xF1AFBC,
            Self::Pms496C => 0xF2C6CF,
            Self::Pms496U => 0xF9C2CC,
            Self::Pms497C => 0x512F2E,
            Self::Pms497U => 0x715956,
            Self::Pms4975C => 0x3F2021,
            Self::Pms4975U => 0x745654,
            Self::Pms498C => 0x6A3735,
            Self::Pms498U => 0x805D5B,
            Self::Pms4985C => 0x874B52,
            Self::Pms4985U => 0x8F6A6F,
            Self::Pms499C => 0x7A3E3A,
            Self::Pms499U => 0x8D615F,
            Self::Pms4995C => 0x9C6169,
            Self::Pms4995U => 0x9F787F,
            Self::Pms500C => 0xC6858F,
            Self::Pms500U => 0xBD898F,
            Self::Pms5005C => 0xB07C83,
            Self::Pms5005U => 0xB28A90,
            Self::Pms501C => 0xDAA5AD,
            Self::Pms501U => 0xD8A5AC,
            Self::Pms5015C => 0xCCA1A6,
            Self::Pms5015U => 0xCAA1A6,
            Self::Pms502C => 0xE5BAC1,
            Self::Pms502U => 0xE6B7BD,
            Self::Pms5025C => 0xDBB7BB,
            Self::Pms5025U => 0xDCB6BA,
            Self::Pms503C => 0xE9C4C7,
            Self::Pms503U => 0xF2CDCF,
            Self::Pms5035C => 0xDFC2C3,
            Self::Pms5035U => 0xE6C6C7,
            Self::Pms504C => 0x572932,
            Self::Pms504U => 0x7F5D62,
            Self::Pms505C => 0x6F2C3F,
            Self::Pms505U => 0x8A5C66,
            Self::Pms506C => 0x84344E,
            Self::Pms506U => 0x9A6372,
            Self::Pms507C => 0xD592AA,
            Self::Pms507U => 0xCF8FA4,
            Self::Pms508C => 0xE4A9BB,
            Self::Pms508U => 0xDF9CAE,
            Self::Pms509C => 0xE8B3C3,
            Self::Pms509U => 0xEAAEBE,
            Self::Pms510C => 0xEBBECB,
            Self::Pms510U => 0xF2BFCB,
            Self::Pms511C => 0x612C51,
            Self::Pms511U => 0x745568,
            Self::Pms5115C => 0x512A44,
            Self::Pms5115U => 0x6F5564,
            Self::Pms512C => 0x833177,
            Self::Pms512U => 0x8A587F,
            Self::Pms5125C => 0x693C5E,
            Self::Pms5125U => 0x7D6378,
            Self::Pms513C => 0x93328E,
            Self::Pms513U => 0x9E5F98,
            Self::Pms5135C => 0x7E5475,
            Self::Pms5135U => 0x8C7287,
            Self::Pms514C => 0xD48BC8,
            Self::Pms514U => 0xCE89C3,
            Self::Pms5145C => 0x9B7793,
            Self::Pms5145U => 0xA4899F,
            Self::Pms515C => 0xE2ACD7,
            Self::Pms515U => 0xE2A5D4,
            Self::Pms5155C => 0xBFA5B8,
            Self::Pms5155U => 0xB99EB2,
            Self::Pms516C => 0xE6BEDD,
            Self::Pms516U => 0xE8B6DC,
            Self::Pms5165C => 0xD3C0CD,
            Self::Pms5165U => 0xCCB4C4,
            Self::Pms517C => 0xEBC6DF,
            Self::Pms517U => 0xF2C8E3,
            Self::Pms5175C => 0xD8C8D1,
            Self::Pms5175U => 0xDAC5D0,
            Self::Pms518C => 0x4B3048,
            Self::Pms518U => 0x6C5969,
            Self::Pms5185C => 0x4A3041,
            Self::Pms5185U => 0x756069,
            Self::Pms519C => 0x59315F,
            Self::Pms519U => 0x775D7A,
            Self::Pms5195C => 0x66435A,
            Self::Pms5195U => 0x816C78,
            Self::Pms520C => 0x642F6C,
            Self::Pms520U => 0x815F87,
            Self::Pms5205C => 0x86647A,
            Self::Pms5205U => 0x907B8A,
            Self::Pms521C => 0xA57FB2,
            Self::Pms521U => 0xA98BB5,
            Self::Pms5215C => 0xAF95A6,
            Self::Pms5215U => 0xA28C9C,
            Self::Pms522C => 0xBA9CC5,
            Self::Pms522U => 0xBC9FC6,
            Self::Pms5225C => 0xC6B0BC,
            Self::Pms5225U => 0xC2ACB9,
            Self::Pms523C => 0xC9B1D0,
            Self::Pms523U => 0xD0B7D5,
            Self::Pms5235C => 0xD0BEC7,
            Self::Pms5235U => 0xD6C3CC,
            Self::Pms524C => 0xD5C2D8,
            Self::Pms524U => 0xDAC5DC,
            Self::Pms5245C => 0xDBCDD3,
            Self::Pms5245U => 0xE0CFD6,
            Self::Pms525C => 0x572C5F,
            Self::Pms525U => 0x735775,
            Self::Pms5255C => 0x1E1A34,
            Self::Pms5255U => 0x5D566C,
            Self::Pms526C => 0x702F8A,
            Self::Pms526U => 0x805791,
            Self::Pms5265C => 0x403A60,
            Self::Pms5265U => 0x6D6B82,
            Self::Pms527C => 0x8031A7,
            Self::Pms527U => 0x9361B0,
            Self::Pms5275C => 0x595478,
            Self::Pms5275U => 0x7B7992,
            Self::Pms528C => 0xB580D1,
            Self::Pms528U => 0xB889D2,
            Self::Pms5285C => 0x8D89A5,
            Self::Pms5285U => 0x9694AC,
            Self::Pms529C => 0xCAA2DD,
            Self::Pms529U => 0xCEA5E1,
            Self::Pms5295C => 0xB3B0C4,
            Self::Pms5295U => 0xB1AFC3,
            Self::Pms530C => 0xD7B9E4,
            Self::Pms530U => 0xDBB9E8,
            Self::Pms5305C => 0xC6C4D2,
            Self::Pms5305U => 0xC5C2D2,
            Self::Pms531C => 0xDFC8E7,
            Self::Pms531U => 0xE4C7EB,
            Self::Pms5315C => 0xD8D7DF,
            Self::Pms5315U => 0xDBD8E1,
            Self::Pms532C => 0x1C1F2A,
            Self::Pms532U => 0x5B5C64,
            Self::Pms533C => 0x1F2A44,
            Self::Pms533U => 0x5E6578,
            Self::Pms534C => 0x1B365D,
            Self::Pms534U => 0x5B6983,
            Self::Pms535C => 0x8E9FBC,
            Self::Pms535U => 0x8C9DB8,
            Self::Pms536C => 0xA2B2C8,
            Self::Pms536U => 0xA4B5CB,
            Self::Pms537C => 0xBBC7D6,
            Self::Pms537U => 0xBDCADA,
            Self::Pms538C => 0xC5CFDA,
            Self::Pms538U => 0xCDD7E1,
            Self::Pms539C => 0x00263A,
            Self::Pms539U => 0x465062,
            Self::Pms5395C => 0x081F2C,
            Self::Pms5395U => 0x464C59,
            Self::Pms540C => 0x003057,
            Self::Pms540U => 0x3F526F,
            Self::Pms5405C => 0x4F758B,
            Self::Pms5405U => 0x5B6F80,
            Self::Pms541C => 0x003C71,
            Self::Pms541U => 0x3A557C,
            Self::Pms5415C => 0x5B7F95,
            Self::Pms5415U => 0x677F91,
            Self::Pms542C => 0x7BAFD4,
            Self::Pms542U => 0x699CC6,
            Self::Pms5425C => 0x7A99AC,
            Self::Pms5425U => 0x7993A5,
            Self::Pms543C => 0xA4C8E1,
            Self::Pms543U => 0x79ABD2,
            Self::Pms5435C => 0xA6BBC8,
            Self::Pms5435U => 0x9DB4C4,
            Self::Pms544C => 0xBDD6E6,
            Self::Pms544U => 0x97C1DF,
            Self::Pms5445C => 0xB7C9D3,
            Self::Pms5445U => 0xB4C7D3,
            Self::Pms545C => 0xC6DAE7,
            Self::Pms545U => 0xAECFE6,
            Self::Pms5455C => 0xBFCED6,
            Self::Pms5455U => 0xC7D7E0,
            Self::Pms546C => 0x072B31,
            Self::Pms546U => 0x465058,
            Self::Pms5463C => 0x07272D,
            Self::Pms5463U => 0x3E4E56,
            Self::Pms5467C => 0x18332F,
            Self::Pms5467U => 0x545F5F,
            Self::Pms547C => 0x00313C,
            Self::Pms547U => 0x415866,
            Self::Pms5473C => 0x115E67,
            Self::Pms5473U => 0x516D75,
            Self::Pms5477C => 0x3E5D58,
            Self::Pms5477U => 0x5F7070,
            Self::Pms548C => 0x003D4C,
            Self::Pms548U => 0x3E5E6F,
            Self::Pms5483C => 0x4F868E,
            Self::Pms5483U => 0x5F8289,
            Self::Pms5487C => 0x5D7975,
            Self::Pms5487U => 0x697C7D,
            Self::Pms549C => 0x6BA4B8,
            Self::Pms549U => 0x6899AE,
            Self::Pms5493C => 0x7FA9AE,
            Self::Pms5493U => 0x769CA3,
            Self::Pms5497C => 0x829995,
            Self::Pms5497U => 0x819595,
            Self::Pms550C => 0x8DB9CA,
            Self::Pms550U => 0x7AAABE,
            Self::Pms5503C => 0x94B7BB,
            Self::Pms5503U => 0x94B7BC,
            Self::Pms5507C => 0x9DB0AC,
            Self::Pms5507U => 0x96A9A7,
            Self::Pms551C => 0xA3C7D2,
            Self::Pms551U => 0x96C0CE,
            Self::Pms5513C => 0xABC7CA,
            Self::Pms5513U => 0xACCACD,
            Self::Pms5517C => 0xB1C0BC,
            Self::Pms5517U => 0xA9BAB7,
            Self::Pms552C => 0xB9D3DC,
            Self::Pms552U => 0xB0D0DC,
            Self::Pms5523C => 0xB6CFD0,
            Self::Pms5523U => 0xBCD5D6,
            Self::Pms5527C => 0xBCC9C5,
            Self::Pms5527U => 0xB8C7C4,
            Self::Pms553C => 0x284734,
            Self::Pms553U => 0x5A685D,
            Self::Pms5535C => 0x183028,
            Self::Pms5535U => 0x485956,
            Self::Pms554C => 0x205C40,
            Self::Pms554U => 0x577565,
            Self::Pms5545C => 0x43695B,
            Self::Pms5545U => 0x60766F,
            Self::Pms555C => 0x28724F,
            Self::Pms555U => 0x567E6A,
            Self::Pms5555C => 0x5C7F71,
            Self::Pms5555U => 0x768E87,
            Self::Pms556C => 0x6FA287,
            Self::Pms556U => 0x719E8B,
            Self::Pms5565C => 0x7F9C90,
            Self::Pms5565U => 0x8AA29A,
            Self::Pms557C => 0x85B09A,
            Self::Pms557U => 0x88B29F,
            Self::Pms5575C => 0x92ACA0,
            Self::Pms5575U => 0x9DB4AB,
            Self::Pms558C => 0x9ABEAA,
            Self::Pms558U => 0x9AC0AE,
            Self::Pms5585C => 0xA7BDB1,
            Self::Pms5585U => 0xA2B9AF,
            Self::Pms559C => 0xADCAB8,
            Self::Pms559U => 0xB5D1C0,
            Self::Pms5595C => 0xBFCEC2,
            Self::Pms5595U => 0xC1D2C7,
            Self::Pms560C => 0x1D3C34,
            Self::Pms560U => 0x4B5E5B,
            Self::Pms5605C => 0x22372B,
            Self::Pms5605U => 0x5E695E,
            Self::Pms561C => 0x00594C,
            Self::Pms561U => 0x466D69,
            Self::Pms5615C => 0x5E7461,
            Self::Pms5615U => 0x6A7970,
            Self::Pms562C => 0x006F62,
            Self::Pms562U => 0x43807A,
            Self::Pms5625C => 0x708573,
            Self::Pms5625U => 0x798981,
            Self::Pms563C => 0x6BBBAE,
            Self::Pms563U => 0x76B9AF,
            Self::Pms5635C => 0x94A596,
            Self::Pms5635U => 0x8D9D94,
            Self::Pms564C => 0x86C8BC,
            Self::Pms564U => 0x88C6BC,
            Self::Pms5645C => 0xA3B2A4,
            Self::Pms5645U => 0x9FAFA5,
            Self::Pms565C => 0xA1D6CA,
            Self::Pms565U => 0x91D1C5,
            Self::Pms5655C => 0xB0BDB0,
            Self::Pms5655U => 0xB3C0B6,
            Self::Pms566C => 0xB9DCD2,
            Self::Pms566U => 0xB4E1D6,
            Self::Pms5665C => 0xBAC5B9,
            Self::Pms5665U => 0xBCC8BD,
            Self::Pms567C => 0x173F35,
            Self::Pms567U => 0x4A635D,
            Self::Pms568C => 0x006C5B,
            Self::Pms568U => 0x458075,
            Self::Pms569C => 0x00816D,
            Self::Pms569U => 0x2C8F82,
            Self::Pms570C => 0x6BCABA,
            Self::Pms570U => 0x62C2B1,
            Self::Pms571C => 0x98DBCE,
            Self::Pms571U => 0x85D8C8,
            Self::Pms572C => 0xA5DFD3,
            Self::Pms572U => 0x95DECF,
            Self::Pms573C => 0xB5E3D8,
            Self::Pms573U => 0xBBE9DE,
            Self::Pms574C => 0x4E5B31,
            Self::Pms574U => 0x6A7355,
            Self::Pms5743C => 0x3E4827,
            Self::Pms5743U => 0x636A51,
            Self::Pms5747C => 0x3D441E,
            Self::Pms5747U => 0x70744E,
            Self::Pms575C => 0x67823A,
            Self::Pms575U => 0x6F8456,
            Self::Pms5753C => 0x5E6738,
            Self::Pms5753U => 0x747C61,
            Self::Pms5757C => 0x6D712E,
            Self::Pms5757U => 0x81855C,
            Self::Pms576C => 0x789D4A,
            Self::Pms576U => 0x7A9B62,
            Self::Pms5763C => 0x737B4C,
            Self::Pms5763U => 0x7E876D,
            Self::Pms5767C => 0x8A8D4A,
            Self::Pms5767U => 0x94986C,
            Self::Pms577C => 0xA9C47F,
            Self::Pms577U => 0x93B479,
            Self::Pms5773C => 0x899064,
            Self::Pms5773U => 0x9AA184,
            Self::Pms5777C => 0xA2A569,
            Self::Pms5777U => 0xA6A97B,
            Self::Pms578C => 0xB7CE95,
            Self::Pms578U => 0xA7C58E,
            Self::Pms5783C => 0xA3AA83,
            Self::Pms5783U => 0xB5BB9F,
            Self::Pms5787C => 0xBABD8B,
            Self::Pms5787U => 0xBABE8F,
            Self::Pms579C => 0xBCD19B,
            Self::Pms579U => 0xBAD29E,
            Self::Pms5793C => 0xB3B995,
            Self::Pms5793U => 0xC5CAAF,
            Self::Pms5797C => 0xC6C89B,
            Self::Pms5797U => 0xC7CA9C,
            Self::Pms580C => 0xC4D6A4,
            Self::Pms580U => 0xC9DCAC,
            Self::Pms5803C => 0xC3C6A8,
            Self::Pms5803U => 0xCFD3B8,
            Self::Pms5807C => 0xD0D1AB,
            Self::Pms5807U => 0xD1D3A9,
            Self::Pms581C => 0x625D20,
            Self::Pms581U => 0x78754A,
            Self::Pms5815C => 0x555025,
            Self::Pms5815U => 0x736F4C,
            Self::Pms582C => 0x8E8C13,
            Self::Pms582U => 0x919145,
            Self::Pms5825C => 0x89813D,
            Self::Pms5825U => 0x8B865D,
            Self::Pms583C => 0xB7BF10,
            Self::Pms583U => 0xA6B340,
            Self::Pms5835C => 0xA09958,
            Self::Pms5835U => 0x938F66,
            Self::Pms584C => 0xD2D755,
            Self::Pms584U => 0xCED665,
            Self::Pms5845C => 0xAFA96E,
            Self::Pms5845U => 0xA7A375,
            Self::Pms585C => 0xDBDE70,
            Self::Pms585U => 0xE3E685,
            Self::Pms5855C => 0xC0BB87,
            Self::Pms5855U => 0xBDB889,
            Self::Pms586C => 0xE0E27C,
            Self::Pms586U => 0xE7E988,
            Self::Pms5865C => 0xCBC793,
            Self::Pms5865U => 0xCCC895,
            Self::Pms587C => 0xE3E48D,
            Self::Pms587U => 0xEAEC97,
            Self::Pms5875C => 0xD2CE9E,
            Self::Pms5875U => 0xD7D3A1,
            Self::Pms600C => 0xF1EB9C,
            Self::Pms600U => 0xF9F29D,
            Self::Pms601C => 0xF0E991,
            Self::Pms601U => 0xF7EF8F,
            Self::Pms602C => 0xF0E87B,
            Self::Pms602U => 0xF7EE83,
            Self::Pms603C => 0xEDE04B,
            Self::Pms603U => 0xF3E564,
            Self::Pms604C => 0xEADA24,
            Self::Pms604U => 0xEDDC4D,
            Self::Pms605C => 0xE1CD00,
            Self::Pms605U => 0xE0CC39,
            Self::Pms606C => 0xCFB500,
            Self::Pms606U => 0xCEB829,
            Self::Pms607C => 0xEBE49A,
            Self::Pms607U => 0xF3EAA1,
            Self::Pms608C => 0xE9E186,
            Self::Pms608U => 0xF0E691,
            Self::Pms609C => 0xE6DE77,
            Self::Pms609U => 0xEBE081,
            Self::Pms610C => 0xE1D555,
            Self::Pms610U => 0xE0D268,
            Self::Pms611C => 0xD7C826,
            Self::Pms611U => 0xC8B94E,
            Self::Pms612C => 0xC4B000,
            Self::Pms612U => 0xB7A73F,
            Self::Pms613C => 0xB39B00,
            Self::Pms613U => 0xA29438,
            Self::Pms614C => 0xDCD59A,
            Self::Pms614U => 0xE1D9A1,
            Self::Pms615C => 0xD6CF8D,
            Self::Pms615U => 0xD8D096,
            Self::Pms616C => 0xD0C883,
            Self::Pms616U => 0xCDC588,
            Self::Pms617C => 0xC0B561,
            Self::Pms617U => 0xBBB275,
            Self::Pms618C => 0xAC9F3C,
            Self::Pms618U => 0xA69D62,
            Self::Pms619C => 0x9F912A,
            Self::Pms619U => 0x968D54,
            Self::Pms620C => 0x8A7B19,
            Self::Pms620U => 0x877F49,
            Self::Pms621C => 0xD1E0D7,
            Self::Pms621U => 0xCADFD4,
            Self::Pms622C => 0xB7CDC2,
            Self::Pms622U => 0xB3CBC1,
            Self::Pms623C => 0x9AB9AD,
            Self::Pms623U => 0x96B5AB,
            Self::Pms624C => 0x789F90,
            Self::Pms624U => 0x82A198,
            Self::Pms625C => 0x507F70,
            Self::Pms625U => 0x6C8A82,
            Self::Pms626C => 0x285C4D,
            Self::Pms626U => 0x59756F,
            Self::Pms627C => 0x13322B,
            Self::Pms627U => 0x455D58,
            Self::Pms628C => 0xB8DDE1,
            Self::Pms628U => 0xB4E0E6,
            Self::Pms629C => 0x9BD3DD,
            Self::Pms629U => 0x8DD0DE,
            Self::Pms630C => 0x77C5D5,
            Self::Pms630U => 0x74C4D7,
            Self::Pms631C => 0x3EB1C8,
            Self::Pms631U => 0x56B2CB,
            Self::Pms632C => 0x0093B2,
            Self::Pms632U => 0x3998B5,
            Self::Pms633C => 0x007396,
            Self::Pms633U => 0x27829E,
            Self::Pms634C => 0x005F83,
            Self::Pms634U => 0x226E8A,
            Self::Pms635C => 0xA4DBE8,
            Self::Pms635U => 0xACE3EF,
            Self::Pms636C => 0x8BD3E6,
            Self::Pms636U => 0x8AD8ED,
            Self::Pms637C => 0x4EC3E0,
            Self::Pms637U => 0x5EC8E5,
            Self::Pms638C => 0x00AFD7,
            Self::Pms638U => 0x32B8DF,
            Self::Pms639C => 0x0095C8,
            Self::Pms639U => 0x009ECC,
            Self::Pms640C => 0x0082BA,
            Self::Pms640U => 0x008ABA,
            Self::Pms641C => 0x0067A0,
            Self::Pms641U => 0x0078A7,
            Self::Pms642C => 0xD1DDE6,
            Self::Pms642U => 0xD3E0EA,
            Self::Pms643C => 0xC6D6E3,
            Self::Pms643U => 0xBFD2E4,
            Self::Pms644C => 0x9BB8D3,
            Self::Pms644U => 0x9DB9D5,
            Self::Pms645C => 0x7DA1C4,
            Self::Pms645U => 0x84A3C4,
            Self::Pms646C => 0x5E8AB4,
            Self::Pms646U => 0x6E8DB0,
            Self::Pms647C => 0x236192,
            Self::Pms647U => 0x5F7A9D,
            Self::Pms648C => 0x002E5D,
            Self::Pms648U => 0x4E6282,
            Self::Pms649C => 0xDBE2E9,
            Self::Pms649U => 0xE1E7EE,
            Self::Pms650C => 0xCED9E5,
            Self::Pms650U => 0xD7E0EB,
            Self::Pms651C => 0xA7BCD6,
            Self::Pms651U => 0xAFC3DC,
            Self::Pms652C => 0x7D9BC1,
            Self::Pms652U => 0x89A2C4,
            Self::Pms653C => 0x326295,
            Self::Pms653U => 0x6780A4,
            Self::Pms654C => 0x003A70,
            Self::Pms654U => 0x55688B,
            Self::Pms655C => 0x002554,
            Self::Pms655U => 0x4E5D7F,
            Self::Pms656C => 0xDDE5ED,
            Self::Pms656U => 0xE2EAF2,
            Self::Pms657C => 0xC8D8EB,
            Self::Pms657U => 0xD6E2F2,
            Self::Pms658C => 0xB1C9E8,
            Self::Pms658U => 0xB8CFEE,
            Self::Pms659C => 0x7BA4DB,
            Self::Pms659U => 0x96B6E5,
            Self::Pms660C => 0x407EC9,
            Self::Pms660U => 0x658BCC,
            Self::Pms661C => 0x003594,
            Self::Pms661U => 0x4967AA,
            Self::Pms662C => 0x001A70,
            Self::Pms662U => 0x425390,
            Self::Pms663C => 0xE5E1E6,
            Self::Pms663U => 0xE2D9E3,
            Self::Pms664C => 0xE0DBE3,
            Self::Pms664U => 0xDACFDD,
            Self::Pms665C => 0xC6BCD0,
            Self::Pms665U => 0xC5B8CE,
            Self::Pms666C => 0xA192B2,
            Self::Pms666U => 0xAEA1BC,
            Self::Pms667C => 0x7C6992,
            Self::Pms667U => 0x9083A1,
            Self::Pms668C => 0x614B79,
            Self::Pms668U => 0x817391,
            Self::Pms669C => 0x3F2A56,
            Self::Pms669U => 0x71637F,
            Self::Pms670C => 0xEAD3E2,
            Self::Pms670U => 0xF3D6E5,
            Self::Pms671C => 0xE6BCD8,
            Self::Pms671U => 0xEFC3DD,
            Self::Pms672C => 0xDFA0C9,
            Self::Pms672U => 0xE8A7CD,
            Self::Pms673C => 0xD986BA,
            Self::Pms673U => 0xDF8CBA,
            Self::Pms674C => 0xC6579A,
            Self::Pms674U => 0xCD72A1,
            Self::Pms675C => 0xAE2573,
            Self::Pms675U => 0xBD618E,
            Self::Pms676C => 0x960051,
            Self::Pms676U => 0xA85079,
            Self::Pms677C => 0xE5CEDB,
            Self::Pms677U => 0xEDD2E0,
            Self::Pms678C => 0xE3C8D8,
            Self::Pms678U => 0xE5C0D4,
            Self::Pms679C => 0xDEBED2,
            Self::Pms679U => 0xDAACC6,
            Self::Pms680C => 0xC996B6,
            Self::Pms680U => 0xC08BA9,
            Self::Pms681C => 0xB06C96,
            Self::Pms681U => 0xA5708D,
            Self::Pms682C => 0x994878,
            Self::Pms682U => 0x905D78,
            Self::Pms683C => 0x7C2855,
            Self::Pms683U => 0x805268,
            Self::Pms684C => 0xE4C6D4,
            Self::Pms684U => 0xECCBD9,
            Self::Pms685C => 0xDCB6C9,
            Self::Pms685U => 0xE4BDD0,
            Self::Pms686C => 0xD0A1BA,
            Self::Pms686U => 0xD5A6BE,
            Self::Pms687C => 0xBE84A3,
            Self::Pms687U => 0xBD8CA6,
            Self::Pms688C => 0xA76389,
            Self::Pms688U => 0xA2738C,
            Self::Pms689C => 0x893B67,
            Self::Pms689U => 0x8E6178,
            Self::Pms690C => 0x612141,
            Self::Pms690U => 0x7C5060,
            Self::Pms691C => 0xE9CDD0,
            Self::Pms691U => 0xF1D3D5,
            Self::Pms692C => 0xE4BEC3,
            Self::Pms692U => 0xE9C0C5,
            Self::Pms693C => 0xD7A3AB,
            Self::Pms693U => 0xDAA5AD,
            Self::Pms694C => 0xC48490,
            Self::Pms694U => 0xC38D96,
            Self::Pms695C => 0xB46B7A,
            Self::Pms695U => 0xAC7680,
            Self::Pms696C => 0x984856,
            Self::Pms696U => 0x98656C,
            Self::Pms697C => 0x893C47,
            Self::Pms697U => 0x87565B,
            Self::Pms698C => 0xF2D4D7,
            Self::Pms698U => 0xFAD7DA,
            Self::Pms699C => 0xF4C3CC,
            Self::Pms699U => 0xFCC2CC,
            Self::Pms700C => 0xF2ACB9,
            Self::Pms700U => 0xF8AAB6,
            Self::Pms701C => 0xE68699,
            Self::Pms701U => 0xE98F9D,
            Self::Pms702C => 0xD25B73,
            Self::Pms702U => 0xD87987,
            Self::Pms703C => 0xB83A4B,
            Self::Pms703U => 0xB45A65,
            Self::Pms704C => 0x9E2A2F,
            Self::Pms704U => 0xA24F56,
            Self::Pms705C => 0xF5DADF,
            Self::Pms705U => 0xFFDDE2,
            Self::Pms706C => 0xF7CED7,
            Self::Pms706U => 0xFFCAD4,
            Self::Pms707C => 0xF9B5C4,
            Self::Pms707U => 0xFFB0BF,
            Self::Pms708C => 0xF890A5,
            Self::Pms708U => 0xFE99A8,
            Self::Pms709C => 0xEF6079,
            Self::Pms709U => 0xF47C8A,
            Self::Pms710C => 0xE03E52,
            Self::Pms710U => 0xE4636F,
            Self::Pms711C => 0xCB2C30,
            Self::Pms711U => 0xCE5055,
            Self::Pms712C => 0xFCC89B,
            Self::Pms712U => 0xFFCA9D,
            Self::Pms713C => 0xFDBE87,
            Self::Pms713U => 0xFFC190,
            Self::Pms714C => 0xFDAA63,
            Self::Pms714U => 0xFFB07B,
            Self::Pms715C => 0xF68D2E,
            Self::Pms715U => 0xF99963,
            Self::Pms716C => 0xEA7600,
            Self::Pms716U => 0xEA8651,
            Self::Pms717C => 0xD45D00,
            Self::Pms717U => 0xD77540,
            Self::Pms718C => 0xBE4D00,
            Self::Pms718U => 0xC76C3D,
            Self::Pms719C => 0xEDC8A3,
            Self::Pms719U => 0xF2C6A2,
            Self::Pms720C => 0xE7B78A,
            Self::Pms720U => 0xECBA93,
            Self::Pms721C => 0xDDA46F,
            Self::Pms721U => 0xDFA880,
            Self::Pms722C => 0xC88242,
            Self::Pms722U => 0xCA916C,
            Self::Pms723C => 0xB36924,
            Self::Pms723U => 0xB9825F,
            Self::Pms724C => 0x934D11,
            Self::Pms724U => 0xA57350,
            Self::Pms725C => 0x7D3F16,
            Self::Pms725U => 0x946747,
            Self::Pms726C => 0xE0C09F,
            Self::Pms726U => 0xE8C6A9,
            Self::Pms727C => 0xD9B48F,
            Self::Pms727U => 0xDCB596,
            Self::Pms728C => 0xCDA077,
            Self::Pms728U => 0xCAA182,
            Self::Pms729C => 0xB58150,
            Self::Pms729U => 0xBB9275,
            Self::Pms730C => 0x9E652E,
            Self::Pms730U => 0xA57E62,
            Self::Pms731C => 0x774212,
            Self::Pms731U => 0x957156,
            Self::Pms732C => 0x623412,
            Self::Pms732U => 0x88674D,
            Self::Pms7401C => 0xF5E1A4,
            Self::Pms7401U => 0xFEE5A5,
            Self::Pms7402C => 0xECD898,
            Self::Pms7402U => 0xF3DA9A,
            Self::Pms7403C => 0xEED484,
            Self::Pms7403U => 0xF4CF81,
            Self::Pms7404C => 0xF4DA40,
            Self::Pms7404U => 0xFDCF41,
            Self::Pms7405C => 0xF2CD00,
            Self::Pms7405U => 0xE6B10E,
            Self::Pms7406C => 0xF1C400,
            Self::Pms7406U => 0xF1B828,
            Self::Pms7407C => 0xCBA052,
            Self::Pms7407U => 0xCCA36E,
            Self::Pms7408C => 0xF6BE00,
            Self::Pms7408U => 0xEE9D26,
            Self::Pms7409C => 0xF0B323,
            Self::Pms7409U => 0xF7AD50,
            Self::Pms7410C => 0xFEAD77,
            Self::Pms7410U => 0xFFA06E,
            Self::Pms7411C => 0xE6A65D,
            Self::Pms7411U => 0xE9A16F,
            Self::Pms7412C => 0xD38235,
            Self::Pms7412U => 0xD08C60,
            Self::Pms7413C => 0xDC8633,
            Self::Pms7413U => 0xDE8958,
            Self::Pms7414C => 0xC16C18,
            Self::Pms7414U => 0xC07F53,
            Self::Pms7415C => 0xE6BAA8,
            Self::Pms7415U => 0xEBB5A5,
            Self::Pms7416C => 0xE56A54,
            Self::Pms7416U => 0xEA746A,
            Self::Pms7417C => 0xE04E39,
            Self::Pms7417U => 0xE76B61,
            Self::Pms7418C => 0xCD545B,
            Self::Pms7418U => 0xBF6366,
            Self::Pms7419C => 0xB04A5A,
            Self::Pms7419U => 0xA6626B,
            Self::Pms7420C => 0x9B2242,
            Self::Pms7420U => 0xA45768,
            Self::Pms7421C => 0x651D32,
            Self::Pms7421U => 0x7D4E5B,
            Self::Pms7422C => 0xF4CDD4,
            Self::Pms7422U => 0xFDD4DA,
            Self::Pms7423C => 0xE06287,
            Self::Pms7423U => 0xE2758F,
            Self::Pms7424C => 0xE24585,
            Self::Pms7424U => 0xD9577D,
            Self::Pms7425C => 0xB52555,
            Self::Pms7425U => 0xBB5A73,
            Self::Pms7426C => 0xA4123F,
            Self::Pms7426U => 0xB1556A,
            Self::Pms7427C => 0x971B2F,
            Self::Pms7427U => 0xA5525C,
            Self::Pms7428C => 0x6A2C3E,
            Self::Pms7428U => 0x805C67,
            Self::Pms7429C => 0xE2BCCB,
            Self::Pms7429U => 0xEECCD9,
            Self::Pms7430C => 0xDCA9BF,
            Self::Pms7430U => 0xE6B5C9,
            Self::Pms7431C => 0xC9809E,
            Self::Pms7431U => 0xDCA2B9,
            Self::Pms7432C => 0xB55C80,
            Self::Pms7432U => 0xC5819A,
            Self::Pms7433C => 0xA73A64,
            Self::Pms7433U => 0xB96F89,
            Self::Pms7434C => 0x9B3259,
            Self::Pms7434U => 0xA8677D,
            Self::Pms7435C => 0x872651,
            Self::Pms7435U => 0x8C566B,
            Self::Pms7436C => 0xEEDAEA,
            Self::Pms7436U => 0xF3D5ED,
            Self::Pms7437C => 0xCCAED0,
            Self::Pms7437U => 0xD4B6D6,
            Self::Pms7438C => 0xD59ED7,
            Self::Pms7438U => 0xD79BD7,
            Self::Pms7439C => 0xB288B9,
            Self::Pms7439U => 0xC09CC6,
            Self::Pms7440C => 0xA277A6,
            Self::Pms7440U => 0xB796BA,
            Self::Pms7441C => 0x9F5CC0,
            Self::Pms7441U => 0xAE7ECA,
            Self::Pms7442C => 0x963CBD,
            Self::Pms7442U => 0x9E62BE,
            Self::Pms7443C => 0xDDDAE8,
            Self::Pms7443U => 0xDDD8EB,
            Self::Pms7444C => 0xB6B8DC,
            Self::Pms7444U => 0xBCBDE2,
            Self::Pms7445C => 0xA7A2C3,
            Self::Pms7445U => 0xACA8C7,
            Self::Pms7446C => 0x8986CA,
            Self::Pms7446U => 0x9493CF,
            Self::Pms7447C => 0x5D4777,
            Self::Pms7447U => 0x7F7392,
            Self::Pms7448C => 0x4B384C,
            Self::Pms7448U => 0x726874,
            Self::Pms7449C => 0x41273B,
            Self::Pms7449U => 0x63515C,
            Self::Pms7450C => 0xBDC5DB,
            Self::Pms7450U => 0xC3CBE0,
            Self::Pms7451C => 0x89ABE3,
            Self::Pms7451U => 0x94B3EA,
            Self::Pms7452C => 0x8094DD,
            Self::Pms7452U => 0x91A0E4,
            Self::Pms7453C => 0x7BA6DE,
            Self::Pms7453U => 0x88ADE3,
            Self::Pms7454C => 0x5F8FB4,
            Self::Pms7454U => 0x7599BB,
            Self::Pms7455C => 0x3A5DAE,
            Self::Pms7455U => 0x6B7DB9,
            Self::Pms7456C => 0x606EB2,
            Self::Pms7456U => 0x8089C0,
            Self::Pms7457C => 0xBBDDE6,
            Self::Pms7457U => 0xBFE3ED,
            Self::Pms7458C => 0x71B2C9,
            Self::Pms7458U => 0x67A5BF,
            Self::Pms7459C => 0x4298B5,
            Self::Pms7459U => 0x518EAA,
            Self::Pms7460C => 0x0086BF,
            Self::Pms7460U => 0x008EC0,
            Self::Pms7461C => 0x007DBA,
            Self::Pms7461U => 0x4492C6,
            Self::Pms7462C => 0x00558C,
            Self::Pms7462U => 0x547AA3,
            Self::Pms7463C => 0x002B49,
            Self::Pms7463U => 0x51657D,
            Self::Pms7464C => 0xA0D1CA,
            Self::Pms7464U => 0xAADBD7,
            Self::Pms7465C => 0x40C1AC,
            Self::Pms7465U => 0x55C7B5,
            Self::Pms7466C => 0x00B0B9,
            Self::Pms7466U => 0x00B4BC,
            Self::Pms7467C => 0x00A3AD,
            Self::Pms7467U => 0x00A8B0,
            Self::Pms7468C => 0x007398,
            Self::Pms7468U => 0x3F7793,
            Self::Pms7469C => 0x005F86,
            Self::Pms7469U => 0x3C6E89,
            Self::Pms7470C => 0x005A70,
            Self::Pms7470U => 0x437083,
            Self::Pms7471C => 0x7EDDD3,
            Self::Pms7471U => 0x7FE2D8,
            Self::Pms7472C => 0x5CB8B2,
            Self::Pms7472U => 0x6BBDB9,
            Self::Pms7473C => 0x279989,
            Self::Pms7473U => 0x5EA59A,
            Self::Pms7474C => 0x007681,
            Self::Pms7474U => 0x458A94,
            Self::Pms7475C => 0x487A7B,
            Self::Pms7475U => 0x6B8B8D,
            Self::Pms7476C => 0x0D5257,
            Self::Pms7476U => 0x5A777B,
            Self::Pms7477C => 0x244C5A,
            Self::Pms7477U => 0x5D7079,
            Self::Pms7478C => 0x9BE3BF,
            Self::Pms7478U => 0xA0EAC5,
            Self::Pms7479C => 0x26D07C,
            Self::Pms7479U => 0x29CB7E,
            Self::Pms7480C => 0x00BF6F,
            Self::Pms7480U => 0x00C281,
            Self::Pms7481C => 0x00B74F,
            Self::Pms7481U => 0x00BA6E,
            Self::Pms7482C => 0x009F4D,
            Self::Pms7482U => 0x27A770,
            Self::Pms7483C => 0x275D38,
            Self::Pms7483U => 0x587762,
            Self::Pms7484C => 0x00573F,
            Self::Pms7484U => 0x456E60,
            Self::Pms7485C => 0xD0DEBB,
            Self::Pms7485U => 0xD5E4C0,
            Self::Pms7486C => 0xBCE194,
            Self::Pms7486U => 0xB7E394,
            Self::Pms7487C => 0x8EDD65,
            Self::Pms7487U => 0x8BE076,
            Self::Pms7488C => 0x78D64B,
            Self::Pms7488U => 0x63D05A,
            Self::Pms7489C => 0x74AA50,
            Self::Pms7489U => 0x7AAB6D,
            Self::Pms7490C => 0x719949,
            Self::Pms7490U => 0x7B9E6B,
            Self::Pms7491C => 0x79863C,
            Self::Pms7491U => 0x818C61,
            Self::Pms7492C => 0xBFCC80,
            Self::Pms7492U => 0xBCCA87,
            Self::Pms7493C => 0xBBC592,
            Self::Pms7493U => 0xB7C194,
            Self::Pms7494C => 0x9CAF88,
            Self::Pms7494U => 0x9EB091,
            Self::Pms7495C => 0x8F993E,
            Self::Pms7495U => 0x838E59,
            Self::Pms7496C => 0x76881D,
            Self::Pms7496U => 0x758448,
            Self::Pms7497C => 0x7A7256,
            Self::Pms7497U => 0x8C8778,
            Self::Pms7498C => 0x5B6236,
            Self::Pms7498U => 0x777D64,
            Self::Pms7499C => 0xF1E6B2,
            Self::Pms7499U => 0xFAEDBC,
            Self::Pms7500C => 0xDFD1A7,
            Self::Pms7500U => 0xE6D5AB,
            Self::Pms7501C => 0xD9C89E,
            Self::Pms7501U => 0xDECBA5,
            Self::Pms7502C => 0xCEB888,
            Self::Pms7502U => 0xC7AF87,
            Self::Pms7503C => 0xA89968,
            Self::Pms7503U => 0x9D9272,
            Self::Pms7504C => 0x94795D,
            Self::Pms7504U => 0x948272,
            Self::Pms7505C => 0x816040,
            Self::Pms7505U => 0x887566,
            Self::Pms7506C => 0xEFDBB2,
            Self::Pms7506U => 0xF9E1B7,
            Self::Pms7507C => 0xFCD299,
            Self::Pms7507U => 0xFFDDAA,
            Self::Pms7508C => 0xE1B87F,
            Self::Pms7508U => 0xE0B384,
            Self::Pms7509C => 0xD6A461,
            Self::Pms7509U => 0xD2A273,
            Self::Pms7510C => 0xC6893F,
            Self::Pms7510U => 0xBE8B60,
            Self::Pms7511C => 0xB77729,
            Self::Pms7511U => 0xA77750,
            Self::Pms7512C => 0xA6631B,
            Self::Pms7512U => 0x966B47,
            Self::Pms7513C => 0xE1B7A7,
            Self::Pms7513U => 0xE7BBB4,
            Self::Pms7514C => 0xD5A286,
            Self::Pms7514U => 0xDAAB9C,
            Self::Pms7515C => 0xC58B68,
            Self::Pms7515U => 0xC2907F,
            Self::Pms7516C => 0x99552B,
            Self::Pms7516U => 0xA07462,
            Self::Pms7517C => 0x85431E,
            Self::Pms7517U => 0x936957,
            Self::Pms7518C => 0x6D4F47,
            Self::Pms7518U => 0x817172,
            Self::Pms7519C => 0x5E4B3C,
            Self::Pms7519U => 0x756B66,
            Self::Pms7520C => 0xEABEB0,
            Self::Pms7520U => 0xF3C4BF,
            Self::Pms7521C => 0xC09C83,
            Self::Pms7521U => 0xC4A195,
            Self::Pms7522C => 0xB46A55,
            Self::Pms7522U => 0xAF7871,
            Self::Pms7523C => 0xAB5C57,
            Self::Pms7523U => 0xA97576,
            Self::Pms7524C => 0xA45248,
            Self::Pms7524U => 0xA36D6A,
            Self::Pms7525C => 0x9A6A4F,
            Self::Pms7525U => 0x997970,
            Self::Pms7526C => 0x8A391B,
            Self::Pms7526U => 0x925F52,
            Self::Pms7527C => 0xD6D2C4,
            Self::Pms7527U => 0xDCD7D2,
            Self::Pms7528C => 0xC5B9AC,
            Self::Pms7528U => 0xD2C7C4,
            Self::Pms7529C => 0xB7A99A,
            Self::Pms7529U => 0xBBAFA9,
            Self::Pms7530C => 0xA39382,
            Self::Pms7530U => 0xA99C95,
            Self::Pms7531C => 0x7A6855,
            Self::Pms7531U => 0x8B7F79,
            Self::Pms7532C => 0x63513D,
            Self::Pms7532U => 0x786E66,
            Self::Pms7533C => 0x473729,
            Self::Pms7533U => 0x6E645C,
            Self::Pms7534C => 0xD1CCBD,
            Self::Pms7534U => 0xD0CBC3,
            Self::Pms7535C => 0xB7B09C,
            Self::Pms7535U => 0xC0BAB0,
            Self::Pms7536C => 0xA69F88,
            Self::Pms7536U => 0xAFA89D,
            Self::Pms7537C => 0xA7ACA2,
            Self::Pms7537U => 0xB4B8B6,
            Self::Pms7538C => 0x949A90,
            Self::Pms7538U => 0xA5AAA8,
            Self::Pms7539C => 0x8E9089,
            Self::Pms7539U => 0x9C9EA0,
            Self::Pms7540C => 0x4B4F54,
            Self::Pms7540U => 0x70747C,
            Self::Pms7541C => 0xD9E1E2,
            Self::Pms7541U => 0xDDE4E6,
            Self::Pms7542C => 0xA4BCC2,
            Self::Pms7542U => 0xB0C5CC,
            Self::Pms7543C => 0x98A4AE,
            Self::Pms7543U => 0xAEB8C0,
            Self::Pms7544C => 0x768692,
            Self::Pms7544U => 0x9AA5AF,
            Self::Pms7545C => 0x425563,
            Self::Pms7545U => 0x78838E,
            Self::Pms7546C => 0x253746,
            Self::Pms7546U => 0x67707A,
            Self::Pms7547C => 0x131E29,
            Self::Pms7547U => 0x51545D,
            Self::Pms7548C => 0xFFC600,
            Self::Pms7548U => 0xFFC915,
            Self::Pms7549C => 0xFFB500,
            Self::Pms7549U => 0xFFB838,
            Self::Pms7550C => 0xD19000,
            Self::Pms7550U => 0xCE973D,
            Self::Pms7551C => 0xB47E00,
            Self::Pms7551U => 0xAE8643,
            Self::Pms7552C => 0x73531D,
            Self::Pms7552U => 0x826F4B,
            Self::Pms7553C => 0x5A4522,
            Self::Pms7553U => 0x70654B,
            Self::Pms7554C => 0x4B3D2A,
            Self::Pms7554U => 0x6C6558,
            Self::Pms7555C => 0xD29F13,
            Self::Pms7555U => 0xD7A95A,
            Self::Pms7556C => 0xB78B20,
            Self::Pms7556U => 0xBC995D,
            Self::Pms7557C => 0x9F7D23,
            Self::Pms7557U => 0xA58D5F,
            Self::Pms7558C => 0x967126,
            Self::Pms7558U => 0x9B835E,
            Self::Pms7559C => 0x8F6A2A,
            Self::Pms7559U => 0x977F5E,
            Self::Pms7560C => 0x7D622E,
            Self::Pms7560U => 0x8E7E62,
            Self::Pms7561C => 0x6C5D34,
            Self::Pms7561U => 0x807964,
            Self::Pms7562C => 0xBD9B60,
            Self::Pms7562U => 0xBDA078,
            Self::Pms7563C => 0xD69A2D,
            Self::Pms7563U => 0xDAA563,
            Self::Pms7564C => 0xDB8A06,
            Self::Pms7564U => 0xDE9858,
            Self::Pms7565C => 0xCD7925,
            Self::Pms7565U => 0xC98659,
            Self::Pms7566C => 0xAD6433,
            Self::Pms7566U => 0xAE7B5E,
            Self::Pms7567C => 0x89532F,
            Self::Pms7567U => 0x90705C,
            Self::Pms7568C => 0x775135,
            Self::Pms7568U => 0x826D5E,
            Self::Pms7569C => 0xD78825,
            Self::Pms7569U => 0xD28E56,
            Self::Pms7570C => 0xD3832B,
            Self::Pms7570U => 0xD18D5B,
            Self::Pms7571C => 0xC67D30,
            Self::Pms7571U => 0xC3895D,
            Self::Pms7572C => 0xB67233,
            Self::Pms7572U => 0xB17F5B,
            Self::Pms7573C => 0xA7662B,
            Self::Pms7573U => 0xA87B59,
            Self::Pms7574C => 0x9E6A38,
            Self::Pms7574U => 0x99775B,
            Self::Pms7575C => 0x835D32,
            Self::Pms7575U => 0x86715A,
            Self::Pms7576C => 0xDB864E,
            Self::Pms7576U => 0xDA8E6B,
            Self::Pms7577C => 0xE07E3C,
            Self::Pms7577U => 0xE18B63,
            Self::Pms7578C => 0xDC6B2F,
            Self::Pms7578U => 0xDB7D59,
            Self::Pms7579C => 0xDC582A,
            Self::Pms7579U => 0xDF7357,
            Self::Pms7580C => 0xC05131,
            Self::Pms7580U => 0xC06D59,
            Self::Pms7581C => 0x864A33,
            Self::Pms7581U => 0x916B5D,
            Self::Pms7582C => 0x674736,
            Self::Pms7582U => 0x7A685E,
            Self::Pms7583C => 0xC4622D,
            Self::Pms7583U => 0xCA7D5F,
            Self::Pms7584C => 0xBA5826,
            Self::Pms7584U => 0xC07759,
            Self::Pms7585C => 0xAF5C37,
            Self::Pms7585U => 0xB67965,
            Self::Pms7586C => 0x9E5330,
            Self::Pms7586U => 0xA2715D,
            Self::Pms7587C => 0x924C2E,
            Self::Pms7587U => 0x996E5C,
            Self::Pms7588C => 0x7B4D35,
            Self::Pms7588U => 0x896E61,
            Self::Pms7589C => 0x5C4738,
            Self::Pms7589U => 0x756A63,
            Self::Pms7590C => 0xD4B59E,
            Self::Pms7590U => 0xDBBCA9,
            Self::Pms7591C => 0xC07D59,
            Self::Pms7591U => 0xC68D78,
            Self::Pms7592C => 0xB15533,
            Self::Pms7592U => 0xB47260,
            Self::Pms7593C => 0x9D432C,
            Self::Pms7593U => 0xA26859,
            Self::Pms7594C => 0x7C3A2D,
            Self::Pms7594U => 0x8B645A,
            Self::Pms7595C => 0x6B3D2E,
            Self::Pms7595U => 0x80665C,
            Self::Pms7596C => 0x5C3D31,
            Self::Pms7596U => 0x776760,
            Self::Pms7597C => 0xD14124,
            Self::Pms7597U => 0xD66756,
            Self::Pms7598C => 0xBD472A,
            Self::Pms7598U => 0xC36D5C,
            Self::Pms7599C => 0xB33D26,
            Self::Pms7599U => 0xBD695A,
            Self::Pms7600C => 0x8D3F2B,
            Self::Pms7600U => 0x9B6A5E,
            Self::Pms7601C => 0x83412C,
            Self::Pms7601U => 0x92695C,
            Self::Pms7602C => 0x7B4931,
            Self::Pms7602U => 0x8D6E61,
            Self::Pms7603C => 0x674230,
            Self::Pms7603U => 0x7C675D,
            Self::Pms7604C => 0xE4D5D3,
            Self::Pms7604U => 0xEAD8D6,
            Self::Pms7605C => 0xE1BBB4,
            Self::Pms7605U => 0xEAC4BE,
            Self::Pms7606C => 0xD6938A,
            Self::Pms7606U => 0xDEA09A,
            Self::Pms7607C => 0xC26E60,
            Self::Pms7607U => 0xCB867F,
            Self::Pms7608C => 0xA4493D,
            Self::Pms7608U => 0xAB6E67,
            Self::Pms7609C => 0x823B34,
            Self::Pms7609U => 0x926660,
            Self::Pms7610C => 0x683431,
            Self::Pms7610U => 0x80615E,
            Self::Pms7611C => 0xDDBCB0,
            Self::Pms7611U => 0xE7C8BE,
            Self::Pms7612C => 0xCA9A8E,
            Self::Pms7612U => 0xD4AAA0,
            Self::Pms7613C => 0xBC8A7E,
            Self::Pms7613U => 0xC89F97,
            Self::Pms7614C => 0xA37F74,
            Self::Pms7614U => 0xAE928C,
            Self::Pms7615C => 0x866761,
            Self::Pms7615U => 0x978480,
            Self::Pms7616C => 0x6B4C4C,
            Self::Pms7616U => 0x867474,
            Self::Pms7617C => 0x583D3E,
            Self::Pms7617U => 0x756667,
            Self::Pms7618C => 0xC66E4E,
            Self::Pms7618U => 0xD08A77,
            Self::Pms7619C => 0xC04C36,
            Self::Pms7619U => 0xC87164,
            Self::Pms7620C => 0xB7312C,
            Self::Pms7620U => 0xC0615B,
            Self::Pms7621C => 0xAB2328,
            Self::Pms7621U => 0xBA5D5A,
            Self::Pms7622C => 0x93272C,
            Self::Pms7622U => 0xA55D5C,
            Self::Pms7623C => 0x8A2A2B,
            Self::Pms7623U => 0x9B5C57,
            Self::Pms7624C => 0x802F2D,
            Self::Pms7624U => 0x8D5A54,
            Self::Pms7625C => 0xE1523D,
            Self::Pms7625U => 0xEA776C,
            Self::Pms7626C => 0xC63527,
            Self::Pms7626U => 0xD2625F,
            Self::Pms7627C => 0xA72B2A,
            Self::Pms7627U => 0xAD5857,
            Self::Pms7628C => 0x9E2A2B,
            Self::Pms7628U => 0xA85959,
            Self::Pms7629C => 0x6D3332,
            Self::Pms7629U => 0x805B5A,
            Self::Pms7630C => 0x633231,
            Self::Pms7630U => 0x765957,
            Self::Pms7631C => 0x572D2D,
            Self::Pms7631U => 0x6E5555,
            Self::Pms7632C => 0xD6C9CA,
            Self::Pms7632U => 0xDDCFD0,
            Self::Pms7633C => 0xC4A4A7,
            Self::Pms7633U => 0xC9AAAE,
            Self::Pms7634C => 0xC16784,
            Self::Pms7634U => 0xC77D92,
            Self::Pms7635C => 0xC63663,
            Self::Pms7635U => 0xCD627D,
            Self::Pms7636C => 0xBC204B,
            Self::Pms7636U => 0xC3566D,
            Self::Pms7637C => 0x912F46,
            Self::Pms7637U => 0x955864,
            Self::Pms7638C => 0x7E2D40,
            Self::Pms7638U => 0x855660,
            Self::Pms7639C => 0x936D73,
            Self::Pms7639U => 0xA0858A,
            Self::Pms7640C => 0x934054,
            Self::Pms7640U => 0x9B6673,
            Self::Pms7641C => 0x8E2C48,
            Self::Pms7641U => 0x9A5D6D,
            Self::Pms7642C => 0x732E4A,
            Self::Pms7642U => 0x7E5867,
            Self::Pms7643C => 0x672E45,
            Self::Pms7643U => 0x7A5A67,
            Self::Pms7644C => 0x582D40,
            Self::Pms7644U => 0x715862,
            Self::Pms7645C => 0x502B3A,
            Self::Pms7645U => 0x6C5760,
            Self::Pms7646C => 0xA56E87,
            Self::Pms7646U => 0xB08799,
            Self::Pms7647C => 0xA83D72,
            Self::Pms7647U => 0xB16889,
            Self::Pms7648C => 0x991E66,
            Self::Pms7648U => 0x9E537B,
            Self::Pms7649C => 0x8A1B61,
            Self::Pms7649U => 0x935379,
            Self::Pms7650C => 0x722257,
            Self::Pms7650U => 0x835573,
            Self::Pms7651C => 0x6A2A5B,
            Self::Pms7651U => 0x7F5C78,
            Self::Pms7652C => 0x5E2751,
            Self::Pms7652U => 0x73556C,
            Self::Pms7653C => 0x948794,
            Self::Pms7653U => 0x9E95A0,
            Self::Pms7654C => 0xA2789C,
            Self::Pms7654U => 0xAA8AA5,
            Self::Pms7655C => 0xA15A95,
            Self::Pms7655U => 0xA978A0,
            Self::Pms7656C => 0x8E3A80,
            Self::Pms7656U => 0x96648D,
            Self::Pms7657C => 0x6E2B62,
            Self::Pms7657U => 0x7E5978,
            Self::Pms7658C => 0x6A3460,
            Self::Pms7658U => 0x7C5E77,
            Self::Pms7659C => 0x5D3754,
            Self::Pms7659U => 0x6E5A6B,
            Self::Pms7660C => 0x9991A4,
            Self::Pms7660U => 0x9C97A8,
            Self::Pms7661C => 0x8D6E97,
            Self::Pms7661U => 0x947F9D,
            Self::Pms7662C => 0x7A4183,
            Self::Pms7662U => 0x84668C,
            Self::Pms7663C => 0x6B3077,
            Self::Pms7663U => 0x7A5C83,
            Self::Pms7664C => 0x653279,
            Self::Pms7664U => 0x765D84,
            Self::Pms7665C => 0x5E366E,
            Self::Pms7665U => 0x725E7C,
            Self::Pms7666C => 0x5C4E63,
            Self::Pms7666U => 0x706977,
            Self::Pms7667C => 0x6E7CA0,
            Self::Pms7667U => 0x828CA8,
            Self::Pms7668C => 0x686E9F,
            Self::Pms7668U => 0x7B81A4,
            Self::Pms7669C => 0x615E9B,
            Self::Pms7669U => 0x7878A2,
            Self::Pms7670C => 0x565294,
            Self::Pms7670U => 0x706E9A,
            Self::Pms7671C => 0x514689,
            Self::Pms7671U => 0x6E6993,
            Self::Pms7672C => 0x4C4184,
            Self::Pms7672U => 0x6B668E,
            Self::Pms7673C => 0x535486,
            Self::Pms7673U => 0x6C6D8D,
            Self::Pms7674C => 0x878CB4,
            Self::Pms7674U => 0x9296B8,
            Self::Pms7675C => 0x7C7FAB,
            Self::Pms7675U => 0x888CAF,
            Self::Pms7676C => 0x7566A0,
            Self::Pms7676U => 0x827AA3,
            Self::Pms7677C => 0x6F5091,
            Self::Pms7677U => 0x7C6D96,
            Self::Pms7678C => 0x68478D,
            Self::Pms7678U => 0x766691,
            Self::Pms7679C => 0x563D82,
            Self::Pms7679U => 0x6E628B,
            Self::Pms7680C => 0x523178,
            Self::Pms7680U => 0x695981,
            Self::Pms7681C => 0x94A9CB,
            Self::Pms7681U => 0x9DB0D0,
            Self::Pms7682C => 0x6787B7,
            Self::Pms7682U => 0x8097BF,
            Self::Pms7683C => 0x426DA9,
            Self::Pms7683U => 0x6982B0,
            Self::Pms7684C => 0x385E9D,
            Self::Pms7684U => 0x6276A3,
            Self::Pms7685C => 0x2C5697,
            Self::Pms7685U => 0x5E719D,
            Self::Pms7686C => 0x1D4F91,
            Self::Pms7686U => 0x556995,
            Self::Pms7687C => 0x1D428A,
            Self::Pms7687U => 0x525F8D,
            Self::Pms7688C => 0x4698CB,
            Self::Pms7688U => 0x5E9ACA,
            Self::Pms7689C => 0x298FC2,
            Self::Pms7689U => 0x5693C1,
            Self::Pms7690C => 0x0076A8,
            Self::Pms7690U => 0x4C7FA7,
            Self::Pms7691C => 0x006298,
            Self::Pms7691U => 0x467198,
            Self::Pms7692C => 0x005587,
            Self::Pms7692U => 0x4C6B8B,
            Self::Pms7693C => 0x004976,
            Self::Pms7693U => 0x4D6480,
            Self::Pms7694C => 0x01426A,
            Self::Pms7694U => 0x4D6179,
            Self::Pms7695C => 0x7BA7BC,
            Self::Pms7695U => 0x82A8BD,
            Self::Pms7696C => 0x6399AE,
            Self::Pms7696U => 0x719BAF,
            Self::Pms7697C => 0x4E87A0,
            Self::Pms7697U => 0x698FA4,
            Self::Pms7698C => 0x41748D,
            Self::Pms7698U => 0x637F92,
            Self::Pms7699C => 0x34657F,
            Self::Pms7699U => 0x5E7688,
            Self::Pms7700C => 0x165C7D,
            Self::Pms7700U => 0x557288,
            Self::Pms7701C => 0x005776,
            Self::Pms7701U => 0x496B80,
            Self::Pms7702C => 0x48A9C5,
            Self::Pms7702U => 0x5DABC6,
            Self::Pms7703C => 0x009CBD,
            Self::Pms7703U => 0x46A0BF,
            Self::Pms7704C => 0x0085AD,
            Self::Pms7704U => 0x348BAC,
            Self::Pms7705C => 0x007096,
            Self::Pms7705U => 0x3C7D9A,
            Self::Pms7706C => 0x006A8E,
            Self::Pms7706U => 0x437A94,
            Self::Pms7707C => 0x00617F,
            Self::Pms7707U => 0x477388,
            Self::Pms7708C => 0x005670,
            Self::Pms7708U => 0x456B7D,
            Self::Pms7709C => 0x63B1BC,
            Self::Pms7709U => 0x6EB2BD,
            Self::Pms7710C => 0x00A7B5,
            Self::Pms7710U => 0x40AAB8,
            Self::Pms7711C => 0x0097A9,
            Self::Pms7711U => 0x179DAB,
            Self::Pms7712C => 0x00859B,
            Self::Pms7712U => 0x228B9D,
            Self::Pms7713C => 0x007D8A,
            Self::Pms7713U => 0x2D8692,
            Self::Pms7714C => 0x007680,
            Self::Pms7714U => 0x358189,
            Self::Pms7715C => 0x006269,
            Self::Pms7715U => 0x417178,
            Self::Pms7716C => 0x00968F,
            Self::Pms7716U => 0x499996,
            Self::Pms7717C => 0x00857D,
            Self::Pms7717U => 0x3E8E8B,
            Self::Pms7718C => 0x007672,
            Self::Pms7718U => 0x358282,
            Self::Pms7719C => 0x006D68,
            Self::Pms7719U => 0x417B7B,
            Self::Pms7720C => 0x00635B,
            Self::Pms7720U => 0x477675,
            Self::Pms7721C => 0x005E5D,
            Self::Pms7721U => 0x497375,
            Self::Pms7722C => 0x005151,
            Self::Pms7722U => 0x4A6B6E,
            Self::Pms7723C => 0x50A684,
            Self::Pms7723U => 0x6BAC92,
            Self::Pms7724C => 0x00966C,
            Self::Pms7724U => 0x4E9B80,
            Self::Pms7725C => 0x008755,
            Self::Pms7725U => 0x379272,
            Self::Pms7726C => 0x007B4B,
            Self::Pms7726U => 0x348569,
            Self::Pms7727C => 0x006F44,
            Self::Pms7727U => 0x3E7C65,
            Self::Pms7728C => 0x006845,
            Self::Pms7728U => 0x457865,
            Self::Pms7729C => 0x005844,
            Self::Pms7729U => 0x486F67,
            Self::Pms7730C => 0x4B9560,
            Self::Pms7730U => 0x689C7A,
            Self::Pms7731C => 0x228848,
            Self::Pms7731U => 0x58926B,
            Self::Pms7732C => 0x007A3E,
            Self::Pms7732U => 0x498564,
            Self::Pms7733C => 0x007041,
            Self::Pms7733U => 0x4B7C63,
            Self::Pms7734C => 0x286140,
            Self::Pms7734U => 0x597765,
            Self::Pms7735C => 0x36573B,
            Self::Pms7735U => 0x5F7163,
            Self::Pms7736C => 0x395542,
            Self::Pms7736U => 0x5C6B62,
            Self::Pms7737C => 0x6BA539,
            Self::Pms7737U => 0x7BAC66,
            Self::Pms7738C => 0x48A23F,
            Self::Pms7738U => 0x65A969,
            Self::Pms7739C => 0x319B42,
            Self::Pms7739U => 0x5CA369,
            Self::Pms7740C => 0x3A913F,
            Self::Pms7740U => 0x5E9865,
            Self::Pms7741C => 0x44883E,
            Self::Pms7741U => 0x659267,
            Self::Pms7742C => 0x4A773C,
            Self::Pms7742U => 0x688564,
            Self::Pms7743C => 0x44693D,
            Self::Pms7743U => 0x5E745D,
            Self::Pms7744C => 0xBABC16,
            Self::Pms7744U => 0xB9BE56,
            Self::Pms7745C => 0xABAD23,
            Self::Pms7745U => 0xAFB35D,
            Self::Pms7746C => 0x999B30,
            Self::Pms7746U => 0x9B9F60,
            Self::Pms7747C => 0x888D30,
            Self::Pms7747U => 0x909661,
            Self::Pms7748C => 0x7C8034,
            Self::Pms7748U => 0x878C62,
            Self::Pms7749C => 0x727337,
            Self::Pms7749U => 0x808363,
            Self::Pms7750C => 0x656635,
            Self::Pms7750U => 0x777A61,
            Self::Pms7751C => 0xCAB64B,
            Self::Pms7751U => 0xD1BD71,
            Self::Pms7752C => 0xCFB023,
            Self::Pms7752U => 0xD5B961,
            Self::Pms7753C => 0xC1A01E,
            Self::Pms7753U => 0xC6AA5D,
            Self::Pms7754C => 0xA08629,
            Self::Pms7754U => 0xA4915E,
            Self::Pms7755C => 0x897630,
            Self::Pms7755U => 0x90855F,
            Self::Pms7756C => 0x736635,
            Self::Pms7756U => 0x837D62,
            Self::Pms7757C => 0x675E33,
            Self::Pms7757U => 0x7A755F,
            Self::Pms7758C => 0xD4C304,
            Self::Pms7758U => 0xDAC958,
            Self::Pms7759C => 0xC4B200,
            Self::Pms7759U => 0xC8B852,
            Self::Pms7760C => 0x91852C,
            Self::Pms7760U => 0x968F5E,
            Self::Pms7761C => 0x747136,
            Self::Pms7761U => 0x828162,
            Self::Pms7762C => 0x5D6439,
            Self::Pms7762U => 0x747A63,
            Self::Pms7763C => 0x585C3B,
            Self::Pms7763U => 0x717463,
            Self::Pms7764C => 0x535435,
            Self::Pms7764U => 0x6C6E60,
            Self::Pms7765C => 0xBBB323,
            Self::Pms7765U => 0xC0BA5E,
            Self::Pms7766C => 0xB4A91F,
            Self::Pms7766U => 0xBBB35F,
            Self::Pms7767C => 0xAA9D2E,
            Self::Pms7767U => 0xB1A763,
            Self::Pms7768C => 0x8F7E35,
            Self::Pms7768U => 0x958B62,
            Self::Pms7769C => 0x716135,
            Self::Pms7769U => 0x81775F,
            Self::Pms7770C => 0x635939,
            Self::Pms7770U => 0x777161,
            Self::Pms7771C => 0x4E4934,
            Self::Pms7771U => 0x67655A,
            Self::Pms801C => 0x009ACE,
            Self::Pms801U => 0x009CCD,
            Self::Pms802C => 0x44D62C,
            Self::Pms802U => 0x3BD23D,
            Self::Pms803C => 0xFFE900,
            Self::Pms803U => 0xFFE916,
            Self::Pms804C => 0xFFAA4D,
            Self::Pms804U => 0xFFAA52,
            Self::Pms805C => 0xFF7276,
            Self::Pms805U => 0xFF7477,
            Self::Pms806C => 0xFF3EB5,
            Self::Pms806U => 0xFF48B0,
            Self::Pms807C => 0xEA27C2,
            Self::Pms807U => 0xE838BF,
            Self::Pms871C => 0x84754E,
            Self::Pms871U => 0xAD986E,
            Self::Pms872C => 0x85714D,
            Self::Pms872U => 0xAC936E,
            Self::Pms873C => 0x866D4B,
            Self::Pms873U => 0xAE906F,
            Self::Pms874C => 0x8B6F4E,
            Self::Pms874U => 0xAE8F6F,
            Self::Pms875C => 0x87674F,
            Self::Pms875U => 0xB18B74,
            Self::Pms876C => 0x8B634B,
            Self::Pms876U => 0xB08770,
            Self::Pms877C => 0x8A8D8F,
            Self::Pms877U => 0xB4B7B9,
            Self::Black0961C => 0x9E978E,
            Self::Black0961U => 0x9D9994,
            Self::Black2C => 0x332F21,
            Self::Black2U => 0x646052,
            Self::Black3C => 0x212721,
            Self::Black3U => 0x595D59,
            Self::Black4C => 0x31261D,
            Self::Black4U => 0x696056,
            Self::Black5C => 0x3E2B2E,
            Self::Black5U => 0x685A5C,
            Self::Black6C => 0x101820,
            Self::Black6U => 0x4C4E56,
            Self::Black7C => 0x3D3935,
            Self::Black7U => 0x6C6864,
            Self::BlackC => 0x2D2926,
            Self::BlackU => 0x615D59,
            Self::Blue072C => 0x10069F,
            Self::Blue072U => 0x3F43AD,
            Self::Blue0821C => 0x74D1EA,
            Self::Blue0821U => 0x6CD1EF,
            Self::BrightGreenC => 0x00B08B,
            Self::BrightOrangeC => 0xFF5E00,
            Self::BrightRedC => 0xF93822,
            Self::BrightRedU => 0xFE5442,
            Self::CoolGray1C => 0xD9D9D6,
            Self::CoolGray1U => 0xDAD9D6,
            Self::CoolGray10C => 0x63666A,
            Self::CoolGray10U => 0x818387,
            Self::CoolGray11C => 0x53565A,
            Self::CoolGray11U => 0x7A7D81,
            Self::CoolGray2C => 0xD0D0CE,
            Self::CoolGray2U => 0xCACAC8,
            Self::CoolGray3C => 0xC8C9C7,
            Self::CoolGray3U => 0xC5C5C5,
            Self::CoolGray4C => 0xBBBCBC,
            Self::CoolGray4U => 0xB7B8B9,
            Self::CoolGray5C => 0xB1B3B3,
            Self::CoolGray5U => 0xADAEB0,
            Self::CoolGray6C => 0xA7A8AA,
            Self::CoolGray6U => 0xA4A6A8,
            Self::CoolGray7C => 0x97999B,
            Self::CoolGray7U => 0x9B9EA0,
            Self::CoolGray8C => 0x888B8D,
            Self::CoolGray8U => 0x939598,
            Self::CoolGray9C => 0x75787B,
            Self::CoolGray9U => 0x898B8E,
            Self::DarkBlueC => 0x00239C,
            Self::DarkBlueU => 0x444795,
            Self::Green0921C => 0x9DE7D7,
            Self::Green0921U => 0x78E6D0,
            Self::GreenC => 0x00AB84,
            Self::GreenU => 0x00AC8C,
            Self::Magenta0521C => 0xF1B2DC,
            Self::Magenta0521U => 0xF8AADD,
            Self::MediumBlueC => 0x0084CA,
            Self::MediumPurpleC => 0x4E008E,
            Self::MediumPurpleU => 0x65428A,
            Self::MediumYellowC => 0xFFD900,
            Self::NeutralBlackC => 0x222223,
            Self::Orange021C => 0xFE5000,
            Self::Orange021U => 0xFF6C2F,
            Self::PinkC => 0xD62598,
            Self::PinkU => 0xD1428D,
            Self::ProcessBlueC => 0x0085CA,
            Self::ProcessBlueU => 0x0083C3,
            Self::PurpleC => 0xBB29BB,
            Self::PurpleU => 0xBF53B6,
            Self::Red032C => 0xEF3340,
            Self::Red032U => 0xF65058,
            Self::Red0331C => 0xFCAEBB,
            Self::Red0331U => 0xFFB1BE,
            Self::ReflexBlueC => 0x001489,
            Self::ReflexBlueU => 0x39499C,
            Self::RhodamineRedC => 0xE10098,
            Self::RhodamineRedU => 0xE44C9A,
            Self::RubineRedC => 0xCE0058,
            Self::RubineRedU => 0xDB487E,
            Self::StrongRedC => 0xCE0056,
            Self::Violet0631C => 0xBF9BDE,
            Self::Violet0631U => 0xBA93DF,
            Self::VioletC => 0x440099,
            Self::VioletU => 0x7758B3,
            Self::WarmGray1C => 0xD7D2CB,
            Self::WarmGray1U => 0xDBD5CD,
            Self::WarmGray10C => 0x796E65,
            Self::WarmGray10U => 0x827C78,
            Self::WarmGray11C => 0x6E6259,
            Self::WarmGray11U => 0x7D7773,
            Self::WarmGray2C => 0xCBC4BC,
            Self::WarmGray2U => 0xCFC7C0,
            Self::WarmGray3C => 0xBFB8AF,
            Self::WarmGray3U => 0xBEB6AF,
            Self::WarmGray4C => 0xB6ADA5,
            Self::WarmGray4U => 0xB4ACA6,
            Self::WarmGray5C => 0xACA39A,
            Self::WarmGray5U => 0xA9A19C,
            Self::WarmGray6C => 0xA59C94,
            Self::WarmGray6U => 0xA29B96,
            Self::WarmGray7C => 0x968C83,
            Self::WarmGray7U => 0x968F8B,
            Self::WarmGray8C => 0x8C8279,
            Self::WarmGray8U => 0x8F8884,
            Self::WarmGray9C => 0x83786F,
            Self::WarmGray9U => 0x867F7C,
            Self::WarmRedC => 0xF9423A,
            Self::WarmRedU => 0xFF665E,
            Self::WhiteC => 0xFFFFFF,
            Self::Yellow012C => 0xFFD700,
            Self::Yellow012U => 0xFFE000,
            Self::Yellow0131C => 0xF2F0A1,
            Self::Yellow0131U => 0xFBF59B,
            Self::YellowC => 0xFEDD00,
            Self::YellowU => 0xFFE800,
        }
    }
}

impl FromStr for PantoneNamedColor {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .iter()
            .find(|color| s.eq_ignore_ascii_case(color.as_str()))
            .copied()
            .ok_or_else(|| ParseColorError::UnknownPantoneColor { input: s.into() })
    }
}

impl Display for PantoneNamedColor {
    /// Formats the canonical `Pantone <name> <suffix>` name.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<PantoneNamedColor> for Color<u8> {
    fn from(color: PantoneNamedColor) -> Self {
        color.to_color()
    }
}

impl From<PantoneNamedColor> for Color<f64> {
    fn from(color: PantoneNamedColor) -> Self {
        color.to_color().into()
    }
}

impl From<PantoneNamedColor> for Color<f32> {
    fn from(color: PantoneNamedColor) -> Self {
        color.to_color().into()
    }
}

impl From<PantoneNamedColor> for HexColor {
    fn from(color: PantoneNamedColor) -> Self {
        Self::from(color.to_color())
    }
}

/// The wire format is the canonical `Pantone <name> <suffix>` string.
impl Serialize for PantoneNamedColor {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// Deserialization accepts the name case-insensitively.
impl<'de> Deserialize<'de> for PantoneNamedColor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "zbus")]
impl zvariant::Type for PantoneNamedColor {
    const SIGNATURE: &'static zvariant::Signature = <str as zvariant::Type>::SIGNATURE;
}

#[cfg(feature = "async-graphql")]
async_graphql::scalar!(PantoneNamedColor);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_covers_the_2677_colors() {
        assert_eq!(PantoneNamedColor::ALL.len(), 2677);
    }

    #[test]
    fn all_is_sorted_by_name() {
        let names: Vec<&str> = PantoneNamedColor::ALL.iter().map(PantoneNamedColor::as_str).collect();
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(names, sorted);
    }

    #[test]
    fn from_str_is_case_insensitive() {
        assert_eq!("Pantone 485 C".parse::<PantoneNamedColor>().unwrap(), PantoneNamedColor::Pms485C);
        assert_eq!("pantone 485 c".parse::<PantoneNamedColor>().unwrap(), PantoneNamedColor::Pms485C);
        assert_eq!("PANTONE REFLEX BLUE U".parse::<PantoneNamedColor>().unwrap(), PantoneNamedColor::ReflexBlueU);
    }

    #[test]
    fn display_and_as_str_produce_the_canonical_name() {
        assert_eq!(PantoneNamedColor::Pms485C.to_string(), "Pantone 485 C");
        assert_eq!(PantoneNamedColor::Pms485U.as_str(), "Pantone 485 U");
        assert_eq!(PantoneNamedColor::ReflexBlueU.as_str(), "Pantone Reflex Blue U");
    }

    #[test]
    fn display_from_str_round_trip_covers_all_colors() {
        for color in PantoneNamedColor::ALL {
            assert_eq!(color.to_string().parse::<PantoneNamedColor>().unwrap(), *color);
        }
    }

    #[test]
    fn to_color_maps_names_to_srgb() {
        assert_eq!(PantoneNamedColor::Pms485C.to_color(), Color::<u8>::new(0xDA, 0x29, 0x1C));
        assert_eq!(PantoneNamedColor::Pms485U.to_color(), Color::<u8>::new(0xE4, 0x5D, 0x50));
        assert_eq!(PantoneNamedColor::YellowC.to_color(), Color::<u8>::new(0xFE, 0xDD, 0x00));
    }

    #[test]
    fn suffix_resolves_the_name_ending() {
        assert_eq!(PantoneNamedColor::Pms485C.suffix(), PantoneSuffix::Coated);
        assert_eq!(PantoneNamedColor::Pms485U.suffix(), PantoneSuffix::Uncoated);
        assert_eq!(PantoneNamedColor::ALL.iter().filter(|color| color.suffix() == PantoneSuffix::Uncoated).count(), 1335);
    }

    #[test]
    fn nearest_finds_the_closest_named_color() {
        assert_eq!(PantoneNamedColor::nearest(&Color::<u8>::new(0xDA, 0x29, 0x1C)), PantoneNamedColor::Pms485C);
        assert_eq!(PantoneNamedColor::nearest(&Color::<u8>::new(0xFF, 0xFF, 0xFF)), PantoneNamedColor::WhiteC);
    }

    #[test]
    fn to_cmyk_exposes_the_published_build() {
        assert_eq!(PantoneNamedColor::Pms485C.to_cmyk(), Some(Cmyk::new(0, 95, 100, 0)));
        assert_eq!(PantoneNamedColor::Red032C.to_cmyk(), Some(Cmyk::new(0, 90, 86, 0)));
        // Uncoated variants and colors without a published build return None.
        assert_eq!(PantoneNamedColor::Pms485U.to_cmyk(), None);
    }

    #[test]
    fn unknown_names_are_rejected() {
        assert!("Pantone 99999 C".parse::<PantoneNamedColor>().is_err());
        assert_eq!("red".parse::<PantoneNamedColor>().unwrap_err(), ParseColorError::UnknownPantoneColor { input: "red".into() });
    }

    #[test]
    fn serde_round_trip() {
        let json = serde_json::to_string(&PantoneNamedColor::Pms485C).unwrap();
        assert_eq!(json, "\"Pantone 485 C\"");
        assert_eq!(serde_json::from_str::<PantoneNamedColor>("\"pantone 485 u\"").unwrap(), PantoneNamedColor::Pms485U);
    }

    #[cfg(feature = "zbus")]
    #[test]
    fn zvariant_round_trip() {
        assert_eq!(<PantoneNamedColor as zvariant::Type>::SIGNATURE.to_string(), "s");
        let context = zvariant::serialized::Context::new_dbus(zvariant::LE, 0);
        let bytes = zvariant::to_bytes(context, &PantoneNamedColor::Pms485C).unwrap();
        let decoded: PantoneNamedColor = bytes.deserialize().unwrap().0;
        assert_eq!(decoded, PantoneNamedColor::Pms485C);
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn json_schema_lists_the_canonical_names() {
        let schema = serde_json::to_value(schemars::schema_for!(PantoneNamedColor)).unwrap();
        let schema = schema.to_string();
        assert!(schema.contains("\"Pantone 485 C\""));
        assert!(schema.contains("\"Pantone 485 U\""));
    }

    #[cfg(feature = "async-graphql")]
    #[test]
    fn graphql_scalar_round_trip() {
        use async_graphql::ScalarType;
        let parsed = PantoneNamedColor::parse(PantoneNamedColor::Pms485C.to_value()).unwrap();
        assert_eq!(parsed, PantoneNamedColor::Pms485C);
    }
}
