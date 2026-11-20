use crate::Color;
use crate::HexColor;
use crate::PantoneFhiSuffix;
use crate::ParseColorError;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

/// A Pantone Fashion, Home + Interiors (FHI) color.
///
/// The enum covers the 2310 colors of the Pantone FHI system - each
/// color is identified by a `NN-NNNN` code and a name
/// (`Pantone 19-4057 TCX True Blue`). In contrast to
/// [`crate::PantoneNamedColor`] (the print-mixing PMS vocabulary),
/// FHI colors describe dyed or coated material swatches and share
/// their names across the substrate suffixes - see
/// [`PantoneFhiSuffix`]. The canonical string uses `TCX`; `TPX` is
/// discontinued and `TPG` is the current paper guide.
///
/// [`FromStr`] accepts the canonical `Pantone <code> TCX <name>`
/// form, a bare `<code>` or `<code> <suffix>` (`19-4057`,
/// `19-4057 TPX`) and the bare color name (`True Blue`), all
/// case-insensitively. [`Display`] and the wire format use the
/// canonical form.
///
/// The stored sRGB values are community approximations of the
/// physical swatches as published in the
/// [pantone-colors dataset](https://github.com/geoffreycrofte/pantone-colors);
/// FHI colors are defined for textiles and interiors, so the
/// values are indicative rather than authoritative. PANTONE is a
/// trademark of Pantone LLC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum PantoneFhiNamedColor {
    /// `Pantone 11-0103 TCX Egret` — `#F3ECE0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0103 TCX Egret"))]
    Egret,
    /// `Pantone 11-0104 TCX Vanilla Ice` — `#F0EADA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0104 TCX Vanilla Ice"))]
    VanillaIce,
    /// `Pantone 11-0105 TCX Antique White` — `#EDE3D2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0105 TCX Antique White"))]
    AntiqueWhite,
    /// `Pantone 11-0106 TCX Sweet Cream` — `#F0EAD6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0106 TCX Sweet Cream"))]
    SweetCream,
    /// `Pantone 11-0107 TCX Papyrus` — `#F5EDD6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0107 TCX Papyrus"))]
    Papyrus,
    /// `Pantone 11-0110 TCX Buttercream` — `#EFE0CD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0110 TCX Buttercream"))]
    Buttercream,
    /// `Pantone 11-0205 TCX Glass Green` — `#ECEAD0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0205 TCX Glass Green"))]
    GlassGreen,
    /// `Pantone 11-0304 TCX Water Lily` — `#DDE3D5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0304 TCX Water Lily"))]
    WaterLily,
    /// `Pantone 11-0410 TCX Sylvan Green` — `#E7EACB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0410 TCX Sylvan Green"))]
    SylvanGreen,
    /// `Pantone 11-0507 TCX Winter White` — `#F5ECD2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0507 TCX Winter White"))]
    WinterWhite,
    /// `Pantone 11-0510 TCX Afterglow` — `#F3E6C9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0510 TCX Afterglow"))]
    Afterglow,
    /// `Pantone 11-0515 TCX Lemon Icing` — `#F6EBC8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0515 TCX Lemon Icing"))]
    LemonIcing,
    /// `Pantone 11-0601 TCX Bright White` — `#F4F5F0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0601 TCX Bright White"))]
    BrightWhite,
    /// `Pantone 11-0602 TCX Snow White` — `#F2F0EB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0602 TCX Snow White"))]
    SnowWhite,
    /// `Pantone 11-0603 TCX Pastel Parchment` — `#E5D9D3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0603 TCX Pastel Parchment"))]
    PastelParchment,
    /// `Pantone 11-0604 TCX Gardenia` — `#F1E8DF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0604 TCX Gardenia"))]
    Gardenia,
    /// `Pantone 11-0605 TCX Jet Stream` — `#EDE6DE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0605 TCX Jet Stream"))]
    JetStream,
    /// `Pantone 11-0606 TCX Pristine` — `#F2E8DA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0606 TCX Pristine"))]
    Pristine,
    /// `Pantone 11-0607 TCX Sugar Swizzle` — `#F3EEE7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0607 TCX Sugar Swizzle"))]
    SugarSwizzle,
    /// `Pantone 11-0608 TCX Coconut Milk` — `#F0EDE5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0608 TCX Coconut Milk"))]
    CoconutMilk,
    /// `Pantone 11-0609 TCX Ethereal Green` — `#F1ECCA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0609 TCX Ethereal Green"))]
    EtherealGreen,
    /// `Pantone 11-0615 TCX Pear Sorbet` — `#F3EAC3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0615 TCX Pear Sorbet"))]
    PearSorbet,
    /// `Pantone 11-0616 TCX Pastel Yellow` — `#F2E6B1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0616 TCX Pastel Yellow"))]
    PastelYellow,
    /// `Pantone 11-0617 TCX Transparent Yellow` — `#F4ECC2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0617 TCX Transparent Yellow"))]
    TransparentYellow,
    /// `Pantone 11-0618 TCX Wax Yellow` — `#EDE9AD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0618 TCX Wax Yellow"))]
    WaxYellow,
    /// `Pantone 11-0619 TCX Flan` — `#F6E3B4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0619 TCX Flan"))]
    Flan,
    /// `Pantone 11-0620 TCX Elfin Yellow` — `#EEEA97`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0620 TCX Elfin Yellow"))]
    ElfinYellow,
    /// `Pantone 11-0622 TCX Yellow Iris` — `#EEE78E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0622 TCX Yellow Iris"))]
    YellowIris,
    /// `Pantone 11-0623 TCX Yellow Pear` — `#ECE99B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0623 TCX Yellow Pear"))]
    YellowPear,
    /// `Pantone 11-0701 TCX Whisper White` — `#EDE6DB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0701 TCX Whisper White"))]
    WhisperWhite,
    /// `Pantone 11-0710 TCX Tender Yellow` — `#EDEDB7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0710 TCX Tender Yellow"))]
    TenderYellow,
    /// `Pantone 11-0809 TCX Ecru` — `#F3DFCA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0809 TCX Ecru"))]
    Ecru,
    /// `Pantone 11-0907 TCX Pearled Ivory` — `#F0DFCC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-0907 TCX Pearled Ivory"))]
    PearledIvory,
    /// `Pantone 11-1001 TCX White Alyssum` — `#EFEBE7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-1001 TCX White Alyssum"))]
    WhiteAlyssum,
    /// `Pantone 11-1005 TCX Bridal Blush` — `#EEE2DD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-1005 TCX Bridal Blush"))]
    BridalBlush,
    /// `Pantone 11-1302 TCX Sea Salt` — `#F1E6DE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-1302 TCX Sea Salt"))]
    SeaSalt,
    /// `Pantone 11-1305 TCX Angel Wing` — `#F3DFD7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-1305 TCX Angel Wing"))]
    AngelWing,
    /// `Pantone 11-1306 TCX Cream Pink` — `#F6E4D9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-1306 TCX Cream Pink"))]
    CreamPink,
    /// `Pantone 11-1404 TCX Powder Puff` — `#F3E0D6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-1404 TCX Powder Puff"))]
    PowderPuff,
    /// `Pantone 11-1408 TCX Rosewater` — `#F6DBD8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-1408 TCX Rosewater"))]
    Rosewater,
    /// `Pantone 11-2309 TCX Petal Pink` — `#F2E2E0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-2309 TCX Petal Pink"))]
    PetalPink,
    /// `Pantone 11-2409 TCX Delicacy` — `#F5E3E2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-2409 TCX Delicacy"))]
    Delicacy,
    /// `Pantone 11-2511 TCX Shrinking Violet` — `#F4E1E6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-2511 TCX Shrinking Violet"))]
    ShrinkingViolet,
    /// `Pantone 11-4001 TCX Brilliant White` — `#EDF1FE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-4001 TCX Brilliant White"))]
    BrilliantWhite,
    /// `Pantone 11-4201 TCX Cloud Dancer` — `#F0EEE9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-4201 TCX Cloud Dancer"))]
    CloudDancer,
    /// `Pantone 11-4202 TCX Star White` — `#EFEFE8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-4202 TCX Star White"))]
    StarWhite,
    /// `Pantone 11-4300 TCX Marshmallow` — `#F0EEE4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-4300 TCX Marshmallow"))]
    Marshmallow,
    /// `Pantone 11-4301 TCX Lily White` — `#E2E2DA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-4301 TCX Lily White"))]
    LilyWhite,
    /// `Pantone 11-4302 TCX Cannoli Cream` — `#F0EFE2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-4302 TCX Cannoli Cream"))]
    CannoliCream,
    /// `Pantone 11-4303 TCX Mystic Blue` — `#E1E3DE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-4303 TCX Mystic Blue"))]
    MysticBlue,
    /// `Pantone 11-4601 TCX Bit Of Blue` — `#E2EAEB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-4601 TCX Bit Of Blue"))]
    BitOfBlue,
    /// `Pantone 11-4604 TCX Billowing Sail` — `#D8E7E7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-4604 TCX Billowing Sail"))]
    BillowingSail,
    /// `Pantone 11-4800 TCX Blanc De Blanc` — `#E7E9E7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-4800 TCX Blanc De Blanc"))]
    BlancDeBlanc,
    /// `Pantone 11-4801 TCX Tofu` — `#E8E3D9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-4801 TCX Tofu"))]
    Tofu,
    /// `Pantone 11-4802 TCX Summer Shower` — `#E5EBE3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-4802 TCX Summer Shower"))]
    SummerShower,
    /// `Pantone 11-4803 TCX Ice` — `#E0E4D9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-4803 TCX Ice"))]
    Ice,
    /// `Pantone 11-4804 TCX Lightest Sky` — `#E4EADF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-4804 TCX Lightest Sky"))]
    LightestSky,
    /// `Pantone 11-4805 TCX Hint Of Mint` — `#D8E8E6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 11-4805 TCX Hint Of Mint"))]
    HintOfMint,
    /// `Pantone 12-0000 TCX White Swan` — `#E4D7C5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0000 TCX White Swan"))]
    WhiteSwan,
    /// `Pantone 12-0104 TCX White Asparagus` — `#E1DBC8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0104 TCX White Asparagus"))]
    WhiteAsparagus,
    /// `Pantone 12-0105 TCX Bone White` — `#D7D0C0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0105 TCX Bone White"))]
    BoneWhite,
    /// `Pantone 12-0106 TCX Meadow Mist` — `#D3DEC4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0106 TCX Meadow Mist"))]
    MeadowMist,
    /// `Pantone 12-0108 TCX Canary Green` — `#D6DEC9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0108 TCX Canary Green"))]
    CanaryGreen,
    /// `Pantone 12-0109 TCX Ambrosia` — `#D2E7CA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0109 TCX Ambrosia"))]
    Ambrosia,
    /// `Pantone 12-0110 TCX Pistachio Shell` — `#D7CFBB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0110 TCX Pistachio Shell"))]
    PistachioShell,
    /// `Pantone 12-0225 TCX Patina Green` — `#B9EAB3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0225 TCX Patina Green"))]
    PatinaGreen,
    /// `Pantone 12-0304 TCX Whitecap Gray` — `#E0D5C6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0304 TCX Whitecap Gray"))]
    WhitecapGray,
    /// `Pantone 12-0311 TCX Asparagus Green` — `#D2CDB4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0311 TCX Asparagus Green"))]
    AsparagusGreen,
    /// `Pantone 12-0312 TCX Lime Cream` — `#D7E8BC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0312 TCX Lime Cream"))]
    LimeCream,
    /// `Pantone 12-0313 TCX Seafoam Green` — `#CBD5B1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0313 TCX Seafoam Green"))]
    SeafoamGreen,
    /// `Pantone 12-0315 TCX White Jade` — `#D4DBB2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0315 TCX White Jade"))]
    WhiteJade,
    /// `Pantone 12-0317 TCX Gleam` — `#BFD1AD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0317 TCX Gleam"))]
    Gleam,
    /// `Pantone 12-0322 TCX Butterfly` — `#CADEA5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0322 TCX Butterfly"))]
    Butterfly,
    /// `Pantone 12-0404 TCX Light Gray` — `#DAD8C9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0404 TCX Light Gray"))]
    LightGray,
    /// `Pantone 12-0418 TCX Hay` — `#D3CCA3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0418 TCX Hay"))]
    Hay,
    /// `Pantone 12-0426 TCX Mellow Green` — `#D5D593`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0426 TCX Mellow Green"))]
    MellowGreen,
    /// `Pantone 12-0435 TCX Daiquiri Green` — `#C9D77E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0435 TCX Daiquiri Green"))]
    DaiquiriGreen,
    /// `Pantone 12-0520 TCX Pale Lime Yellow` — `#DFE69F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0520 TCX Pale Lime Yellow"))]
    PaleLimeYellow,
    /// `Pantone 12-0521 TCX Young Wheat` — `#E1E3A9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0521 TCX Young Wheat"))]
    YoungWheat,
    /// `Pantone 12-0524 TCX Citron` — `#DFDE9B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0524 TCX Citron"))]
    Citron,
    /// `Pantone 12-0525 TCX Luminary Green` — `#E3EAA5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0525 TCX Luminary Green"))]
    LuminaryGreen,
    /// `Pantone 12-0530 TCX Charlock` — `#E5E790`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0530 TCX Charlock"))]
    Charlock,
    /// `Pantone 12-0601 TCX Eggnog` — `#ECE1D3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0601 TCX Eggnog"))]
    Eggnog,
    /// `Pantone 12-0605 TCX Angora` — `#DFD1BB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0605 TCX Angora"))]
    Angora,
    /// `Pantone 12-0607 TCX Green Essence` — `#E9EAC8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0607 TCX Green Essence"))]
    GreenEssence,
    /// `Pantone 12-0619 TCX Dusty Yellow` — `#D4CC9A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0619 TCX Dusty Yellow"))]
    DustyYellow,
    /// `Pantone 12-0626 TCX Lemon Grass` — `#DCD494`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0626 TCX Lemon Grass"))]
    LemonGrass,
    /// `Pantone 12-0633 TCX Canary Yellow` — `#DFD87E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0633 TCX Canary Yellow"))]
    CanaryYellow,
    /// `Pantone 12-0642 TCX Aurora` — `#EDDD59`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0642 TCX Aurora"))]
    Aurora,
    /// `Pantone 12-0643 TCX Blazing Yellow` — `#FEE715`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0643 TCX Blazing Yellow"))]
    BlazingYellow,
    /// `Pantone 12-0646 TCX Celandine` — `#EBDF67`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0646 TCX Celandine"))]
    Celandine,
    /// `Pantone 12-0703 TCX Seedpearl` — `#E6DAC4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0703 TCX Seedpearl"))]
    Seedpearl,
    /// `Pantone 12-0704 TCX White Smoke` — `#EDDCC9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0704 TCX White Smoke"))]
    WhiteSmoke,
    /// `Pantone 12-0709 TCX Macadamia` — `#E4CFB6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0709 TCX Macadamia"))]
    Macadamia,
    /// `Pantone 12-0710 TCX Navajo` — `#EFDCC3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0710 TCX Navajo"))]
    Navajo,
    /// `Pantone 12-0711 TCX Lemon Meringue` — `#F6E199`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0711 TCX Lemon Meringue"))]
    LemonMeringue,
    /// `Pantone 12-0712 TCX Vanilla` — `#F4E1C1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0712 TCX Vanilla"))]
    Vanilla,
    /// `Pantone 12-0713 TCX Almond Oil` — `#F4EFC1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0713 TCX Almond Oil"))]
    AlmondOil,
    /// `Pantone 12-0714 TCX Cornhusk` — `#F2D6AE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0714 TCX Cornhusk"))]
    Cornhusk,
    /// `Pantone 12-0715 TCX Double Cream` — `#F3E0AC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0715 TCX Double Cream"))]
    DoubleCream,
    /// `Pantone 12-0717 TCX Anise Flower` — `#F4E3B5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0717 TCX Anise Flower"))]
    AniseFlower,
    /// `Pantone 12-0718 TCX Pineapple Slice` — `#E7D391`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0718 TCX Pineapple Slice"))]
    PineappleSlice,
    /// `Pantone 12-0720 TCX Mellow Yellow` — `#F0DD9D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0720 TCX Mellow Yellow"))]
    MellowYellow,
    /// `Pantone 12-0721 TCX Lemonade` — `#F0E79D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0721 TCX Lemonade"))]
    Lemonade,
    /// `Pantone 12-0722 TCX French Vanilla` — `#EFE1A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0722 TCX French Vanilla"))]
    FrenchVanilla,
    /// `Pantone 12-0727 TCX Sunshine` — `#FADE85`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0727 TCX Sunshine"))]
    Sunshine,
    /// `Pantone 12-0729 TCX Sundress` — `#EBCF89`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0729 TCX Sundress"))]
    Sundress,
    /// `Pantone 12-0736 TCX Lemon Drop` — `#FDD878`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0736 TCX Lemon Drop"))]
    LemonDrop,
    /// `Pantone 12-0737 TCX Goldfinch` — `#F8DC6C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0737 TCX Goldfinch"))]
    Goldfinch,
    /// `Pantone 12-0738 TCX Yellow Cream` — `#EFDC75`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0738 TCX Yellow Cream"))]
    YellowCream,
    /// `Pantone 12-0740 TCX Limelight` — `#F0E87D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0740 TCX Limelight"))]
    Limelight,
    /// `Pantone 12-0741 TCX Sunny Lime` — `#DFEF87`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0741 TCX Sunny Lime"))]
    SunnyLime,
    /// `Pantone 12-0742 TCX Lemon Verbena` — `#F3E779`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0742 TCX Lemon Verbena"))]
    LemonVerbena,
    /// `Pantone 12-0752 TCX Buttercup` — `#FAE03C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0752 TCX Buttercup"))]
    Buttercup,
    /// `Pantone 12-0758 TCX Yarrow` — `#FACE6D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0758 TCX Yarrow"))]
    Yarrow,
    /// `Pantone 12-0804 TCX Cloud Cream` — `#E6DDC5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0804 TCX Cloud Cream"))]
    CloudCream,
    /// `Pantone 12-0806 TCX Rutabaga` — `#ECDDBE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0806 TCX Rutabaga"))]
    Rutabaga,
    /// `Pantone 12-0807 TCX Sun Kiss` — `#EBD1BB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0807 TCX Sun Kiss"))]
    SunKiss,
    /// `Pantone 12-0811 TCX Dawn` — `#EBD2B7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0811 TCX Dawn"))]
    Dawn,
    /// `Pantone 12-0812 TCX Alabaster Gleam` — `#F0DEBD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0812 TCX Alabaster Gleam"))]
    AlabasterGleam,
    /// `Pantone 12-0813 TCX Autumn Blonde` — `#EED0AE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0813 TCX Autumn Blonde"))]
    AutumnBlonde,
    /// `Pantone 12-0815 TCX Vanilla Custard` — `#F3E0BE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0815 TCX Vanilla Custard"))]
    VanillaCustard,
    /// `Pantone 12-0817 TCX Apricot Gelato` — `#F5D7AF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0817 TCX Apricot Gelato"))]
    ApricotGelato,
    /// `Pantone 12-0822 TCX Golden Fleece` — `#F2D1A0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0822 TCX Golden Fleece"))]
    GoldenFleece,
    /// `Pantone 12-0824 TCX Pale Banana` — `#FAE199`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0824 TCX Pale Banana"))]
    PaleBanana,
    /// `Pantone 12-0825 TCX Popcorn` — `#F8DE8D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0825 TCX Popcorn"))]
    Popcorn,
    /// `Pantone 12-0826 TCX Golden Haze` — `#FBD897`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0826 TCX Golden Haze"))]
    GoldenHaze,
    /// `Pantone 12-0910 TCX Lambs Wool` — `#E5D0B1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0910 TCX Lambs Wool"))]
    LambsWool,
    /// `Pantone 12-0911 TCX Nude` — `#F2D3BC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0911 TCX Nude"))]
    Nude,
    /// `Pantone 12-0912 TCX Tender Peach` — `#F8D5B8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0912 TCX Tender Peach"))]
    TenderPeach,
    /// `Pantone 12-0913 TCX Alesan` — `#F1CEB3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0913 TCX Alesan"))]
    Alesan,
    /// `Pantone 12-0915 TCX Pale Peach` — `#FED1BD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0915 TCX Pale Peach"))]
    PalePeach,
    /// `Pantone 12-0917 TCX Bleached Apricot` — `#FCCAAC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0917 TCX Bleached Apricot"))]
    BleachedApricot,
    /// `Pantone 12-0921 TCX Golden Straw` — `#E6BD8F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-0921 TCX Golden Straw"))]
    GoldenStraw,
    /// `Pantone 12-1005 TCX Novelle Peach` — `#E7CFBD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1005 TCX Novelle Peach"))]
    NovellePeach,
    /// `Pantone 12-1006 TCX Mother Of Pearl` — `#E9D4C3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1006 TCX Mother Of Pearl"))]
    MotherOfPearl,
    /// `Pantone 12-1007 TCX Pastel Rose Tan` — `#E9D1BF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1007 TCX Pastel Rose Tan"))]
    PastelRoseTan,
    /// `Pantone 12-1008 TCX Linen` — `#EDD2C0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1008 TCX Linen"))]
    Linen,
    /// `Pantone 12-1009 TCX Vanilla Cream` — `#F4D8C6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1009 TCX Vanilla Cream"))]
    VanillaCream,
    /// `Pantone 12-1010 TCX Scallop Shell` — `#FBD8C9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1010 TCX Scallop Shell"))]
    ScallopShell,
    /// `Pantone 12-1011 TCX Peach Puree` — `#EFCFBA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1011 TCX Peach Puree"))]
    PeachPuree,
    /// `Pantone 12-1106 TCX Sheer Pink` — `#F6E5DB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1106 TCX Sheer Pink"))]
    SheerPink,
    /// `Pantone 12-1107 TCX Peach Dust` — `#F0D8CC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1107 TCX Peach Dust"))]
    PeachDust,
    /// `Pantone 12-1108 TCX Dew` — `#EEDED1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1108 TCX Dew"))]
    Dew,
    /// `Pantone 12-1110 TCX Creme De Peche` — `#F5D6C6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1110 TCX Creme De Peche"))]
    CremeDePeche,
    /// `Pantone 12-1206 TCX Silver Peony` — `#E7CFC7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1206 TCX Silver Peony"))]
    SilverPeony,
    /// `Pantone 12-1207 TCX Pearl Blush` — `#F4CEC5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1207 TCX Pearl Blush"))]
    PearlBlush,
    /// `Pantone 12-1209 TCX Soft Pink` — `#F2D8CD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1209 TCX Soft Pink"))]
    SoftPink,
    /// `Pantone 12-1212 TCX Veiled Rose` — `#F8CDC9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1212 TCX Veiled Rose"))]
    VeiledRose,
    /// `Pantone 12-1304 TCX Pearl` — `#F9DBD8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1304 TCX Pearl"))]
    Pearl,
    /// `Pantone 12-1305 TCX Heavenly Pink` — `#F4DEDE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1305 TCX Heavenly Pink"))]
    HeavenlyPink,
    /// `Pantone 12-1310 TCX Blushing Bride` — `#FBD3D9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1310 TCX Blushing Bride"))]
    BlushingBride,
    /// `Pantone 12-1403 TCX Tapioca` — `#DCCDBC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1403 TCX Tapioca"))]
    Tapioca,
    /// `Pantone 12-1404 TCX Pink Tint` — `#DBCBBD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1404 TCX Pink Tint"))]
    PinkTint,
    /// `Pantone 12-1605 TCX Crystal Pink` — `#EDD0CE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1605 TCX Crystal Pink"))]
    CrystalPink,
    /// `Pantone 12-1706 TCX Pink Dogwood` — `#F7D1D1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1706 TCX Pink Dogwood"))]
    PinkDogwood,
    /// `Pantone 12-1708 TCX Crystal Rose` — `#FDC3C6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1708 TCX Crystal Rose"))]
    CrystalRose,
    /// `Pantone 12-1813 TCX Marys Rose` — `#F7D1D4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-1813 TCX Marys Rose"))]
    MarysRose,
    /// `Pantone 12-2102 TCX Mauve Morn` — `#ECD6D6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-2102 TCX Mauve Morn"))]
    MauveMorn,
    /// `Pantone 12-2103 TCX Almost Mauve` — `#E7DCD9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-2103 TCX Almost Mauve"))]
    AlmostMauve,
    /// `Pantone 12-2901 TCX Morganite` — `#DFCDC6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-2901 TCX Morganite"))]
    Morganite,
    /// `Pantone 12-2902 TCX Mauve Chalk` — `#E5D0CF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-2902 TCX Mauve Chalk"))]
    MauveChalk,
    /// `Pantone 12-2903 TCX Light Lilac` — `#DEC6D3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-2903 TCX Light Lilac"))]
    LightLilac,
    /// `Pantone 12-2904 TCX Primrose Pink` — `#EED4D9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-2904 TCX Primrose Pink"))]
    PrimrosePink,
    /// `Pantone 12-2905 TCX Cradle Pink` — `#EDD0DD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-2905 TCX Cradle Pink"))]
    CradlePink,
    /// `Pantone 12-2906 TCX Barely Pink` — `#F8D7DD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-2906 TCX Barely Pink"))]
    BarelyPink,
    /// `Pantone 12-2907 TCX Rose Water` — `#F8E0E7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-2907 TCX Rose Water"))]
    RoseWater,
    /// `Pantone 12-4301 TCX Almond Milk` — `#D6CEBE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-4301 TCX Almond Milk"))]
    AlmondMilk,
    /// `Pantone 12-4302 TCX Vaporous Gray` — `#DFDDD7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-4302 TCX Vaporous Gray"))]
    VaporousGray,
    /// `Pantone 12-4304 TCX Bluewash` — `#E2E6E0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-4304 TCX Bluewash"))]
    Bluewash,
    /// `Pantone 12-4305 TCX Spa Blue` — `#D3DEDF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-4305 TCX Spa Blue"))]
    SpaBlue,
    /// `Pantone 12-4306 TCX Barely Blue` — `#DDE0DF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-4306 TCX Barely Blue"))]
    BarelyBlue,
    /// `Pantone 12-4604 TCX Skylight` — `#C8E0E0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-4604 TCX Skylight"))]
    Skylight,
    /// `Pantone 12-4607 TCX Pastel Blue` — `#BCD3D5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-4607 TCX Pastel Blue"))]
    PastelBlue,
    /// `Pantone 12-4608 TCX Clearwater` — `#AAD5DB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-4608 TCX Clearwater"))]
    Clearwater,
    /// `Pantone 12-4609 TCX Starlight Blue` — `#B5CED4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-4609 TCX Starlight Blue"))]
    StarlightBlue,
    /// `Pantone 12-4610 TCX Whispering Blue` — `#C9DCDC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-4610 TCX Whispering Blue"))]
    WhisperingBlue,
    /// `Pantone 12-4705 TCX Blue Blush` — `#D6DBD9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-4705 TCX Blue Blush"))]
    BlueBlush,
    /// `Pantone 12-4805 TCX Wan Blue` — `#CBDCDF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-4805 TCX Wan Blue"))]
    WanBlue,
    /// `Pantone 12-4806 TCX Chalk Blue` — `#CCDAD7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-4806 TCX Chalk Blue"))]
    ChalkBlue,
    /// `Pantone 12-5201 TCX Icicle` — `#DADCD0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5201 TCX Icicle"))]
    Icicle,
    /// `Pantone 12-5202 TCX Turtledove` — `#DED7C8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5202 TCX Turtledove"))]
    Turtledove,
    /// `Pantone 12-5203 TCX Murmur` — `#D2D8D2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5203 TCX Murmur"))]
    Murmur,
    /// `Pantone 12-5204 TCX Morning Mist` — `#CFDFDB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5204 TCX Morning Mist"))]
    MorningMist,
    /// `Pantone 12-5206 TCX Blue Glass` — `#C6E3E1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5206 TCX Blue Glass"))]
    BlueGlass,
    /// `Pantone 12-5209 TCX Soothing Sea` — `#C3E9E4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5209 TCX Soothing Sea"))]
    SoothingSea,
    /// `Pantone 12-5303 TCX Sprout Green` — `#CBD7D2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5303 TCX Sprout Green"))]
    SproutGreen,
    /// `Pantone 12-5403 TCX Blue Flower` — `#D0D9D4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5403 TCX Blue Flower"))]
    BlueFlower,
    /// `Pantone 12-5404 TCX Whisper Green` — `#E0E6D7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5404 TCX Whisper Green"))]
    WhisperGreen,
    /// `Pantone 12-5406 TCX Opal Blue` — `#C3DDD6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5406 TCX Opal Blue"))]
    OpalBlue,
    /// `Pantone 12-5407 TCX Aqua Glass` — `#D2E8E0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5407 TCX Aqua Glass"))]
    AquaGlass,
    /// `Pantone 12-5408 TCX Moonlight Jade` — `#C7E5DF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5408 TCX Moonlight Jade"))]
    MoonlightJade,
    /// `Pantone 12-5409 TCX Fair Aqua` — `#B8E2DC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5409 TCX Fair Aqua"))]
    FairAqua,
    /// `Pantone 12-5410 TCX Bleached Aqua` — `#BCE3DF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5410 TCX Bleached Aqua"))]
    BleachedAqua,
    /// `Pantone 12-5504 TCX Clearly Aqua` — `#CEE1D4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5504 TCX Clearly Aqua"))]
    ClearlyAqua,
    /// `Pantone 12-5505 TCX Glacier` — `#C3DBD4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5505 TCX Glacier"))]
    Glacier,
    /// `Pantone 12-5506 TCX Dusty Aqua` — `#C0DCCD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5506 TCX Dusty Aqua"))]
    DustyAqua,
    /// `Pantone 12-5507 TCX Bay` — `#BAE5D6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5507 TCX Bay"))]
    Bay,
    /// `Pantone 12-5508 TCX Hushed Green` — `#D8E9E5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5508 TCX Hushed Green"))]
    HushedGreen,
    /// `Pantone 12-5603 TCX Zephyr Blue` — `#D3D9D1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5603 TCX Zephyr Blue"))]
    ZephyrBlue,
    /// `Pantone 12-5808 TCX Honeydew` — `#BAE1D3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-5808 TCX Honeydew"))]
    Honeydew,
    /// `Pantone 12-6204 TCX Silver Green` — `#D7D7C7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-6204 TCX Silver Green"))]
    SilverGreen,
    /// `Pantone 12-6205 TCX Milky Green` — `#CFDBD1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-6205 TCX Milky Green"))]
    MilkyGreen,
    /// `Pantone 12-6206 TCX Fairest Jade` — `#D8E3D7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-6206 TCX Fairest Jade"))]
    FairestJade,
    /// `Pantone 12-6207 TCX Frost` — `#DDE2D6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-6207 TCX Frost"))]
    Frost,
    /// `Pantone 12-6208 TCX Phantom Green` — `#DCE4D7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 12-6208 TCX Phantom Green"))]
    PhantomGreen,
    /// `Pantone 13-0000 TCX Moonbeam` — `#CDC6BD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0000 TCX Moonbeam"))]
    Moonbeam,
    /// `Pantone 13-0002 TCX White Sand` — `#DBD5D1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0002 TCX White Sand"))]
    WhiteSand,
    /// `Pantone 13-0107 TCX Dewkist` — `#C4D1C2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0107 TCX Dewkist"))]
    Dewkist,
    /// `Pantone 13-0111 TCX Seacrest` — `#BFD1B3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0111 TCX Seacrest"))]
    Seacrest,
    /// `Pantone 13-0116 TCX Pastel Green` — `#B4D3B2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0116 TCX Pastel Green"))]
    PastelGreen,
    /// `Pantone 13-0117 TCX Green Ash` — `#A0DAA9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0117 TCX Green Ash"))]
    GreenAsh,
    /// `Pantone 13-0210 TCX Fog Green` — `#C2CBB4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0210 TCX Fog Green"))]
    FogGreen,
    /// `Pantone 13-0212 TCX Tender Greens` — `#C5CFB6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0212 TCX Tender Greens"))]
    TenderGreens,
    /// `Pantone 13-0215 TCX Reed` — `#C3D3A8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0215 TCX Reed"))]
    Reed,
    /// `Pantone 13-0220 TCX Paradise Green` — `#B2E79F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0220 TCX Paradise Green"))]
    ParadiseGreen,
    /// `Pantone 13-0221 TCX Pistachio Green` — `#A9D39E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0221 TCX Pistachio Green"))]
    PistachioGreen,
    /// `Pantone 13-0317 TCX Lily Green` — `#C5CF98`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0317 TCX Lily Green"))]
    LilyGreen,
    /// `Pantone 13-0319 TCX Shadow Lime` — `#CFE09D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0319 TCX Shadow Lime"))]
    ShadowLime,
    /// `Pantone 13-0324 TCX Lettuce Green` — `#BED38E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0324 TCX Lettuce Green"))]
    LettuceGreen,
    /// `Pantone 13-0331 TCX Sap Green` — `#AFCB80`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0331 TCX Sap Green"))]
    SapGreen,
    /// `Pantone 13-0333 TCX Lima Bean` — `#E1D590`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0333 TCX Lima Bean"))]
    LimaBean,
    /// `Pantone 13-0401 TCX Oatmeal` — `#CBC3B4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0401 TCX Oatmeal"))]
    Oatmeal,
    /// `Pantone 13-0403 TCX Gray Morn` — `#CABEB5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0403 TCX Gray Morn"))]
    GrayMorn,
    /// `Pantone 13-0442 TCX Green Glow` — `#B0C965`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0442 TCX Green Glow"))]
    GreenGlow,
    /// `Pantone 13-0443 TCX Lime Popsicle` — `#C0DB3A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0443 TCX Lime Popsicle"))]
    LimePopsicle,
    /// `Pantone 13-0513 TCX Frozen Dew` — `#D8CFB2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0513 TCX Frozen Dew"))]
    FrozenDew,
    /// `Pantone 13-0522 TCX Pale Green` — `#CBCE91`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0522 TCX Pale Green"))]
    PaleGreen,
    /// `Pantone 13-0530 TCX Lime Sherbet` — `#CDD78A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0530 TCX Lime Sherbet"))]
    LimeSherbet,
    /// `Pantone 13-0532 TCX Celery Green` — `#C5CC7B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0532 TCX Celery Green"))]
    CeleryGreen,
    /// `Pantone 13-0535 TCX Sharp Green` — `#C6EC7A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0535 TCX Sharp Green"))]
    SharpGreen,
    /// `Pantone 13-0540 TCX Wild Lime` — `#C3D363`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0540 TCX Wild Lime"))]
    WildLime,
    /// `Pantone 13-0550 TCX Lime Punch` — `#C0D725`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0550 TCX Lime Punch"))]
    LimePunch,
    /// `Pantone 13-0607 TCX Fog` — `#D0C5B1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0607 TCX Fog"))]
    Fog,
    /// `Pantone 13-0608 TCX Aloe Wash` — `#D0D3B7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0608 TCX Aloe Wash"))]
    AloeWash,
    /// `Pantone 13-0611 TCX Moth` — `#D2CBAF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0611 TCX Moth"))]
    Moth,
    /// `Pantone 13-0613 TCX Chino Green` — `#D9CAA5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0613 TCX Chino Green"))]
    ChinoGreen,
    /// `Pantone 13-0614 TCX Garden Glade` — `#DCD8A8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0614 TCX Garden Glade"))]
    GardenGlade,
    /// `Pantone 13-0624 TCX Golden Mist` — `#D5CD94`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0624 TCX Golden Mist"))]
    GoldenMist,
    /// `Pantone 13-0632 TCX Endive` — `#D2CC81`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0632 TCX Endive"))]
    Endive,
    /// `Pantone 13-0633 TCX Chardonnay` — `#E7DF99`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0633 TCX Chardonnay"))]
    Chardonnay,
    /// `Pantone 13-0640 TCX Acacia` — `#DACD65`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0640 TCX Acacia"))]
    Acacia,
    /// `Pantone 13-0644 TCX Golden Kiwi` — `#F3DD3E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0644 TCX Golden Kiwi"))]
    GoldenKiwi,
    /// `Pantone 13-0645 TCX Limeade` — `#D3D95F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0645 TCX Limeade"))]
    Limeade,
    /// `Pantone 13-0646 TCX Meadowlark` — `#EAD94E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0646 TCX Meadowlark"))]
    Meadowlark,
    /// `Pantone 13-0648 TCX Green Sheen` — `#D9CE52`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0648 TCX Green Sheen"))]
    GreenSheen,
    /// `Pantone 13-0650 TCX Sulphur Spring` — `#D5D717`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0650 TCX Sulphur Spring"))]
    SulphurSpring,
    /// `Pantone 13-0651 TCX Evening Primrose` — `#CCDB1E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0651 TCX Evening Primrose"))]
    EveningPrimrose,
    /// `Pantone 13-0711 TCX Putty` — `#D4CAB0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0711 TCX Putty"))]
    Putty,
    /// `Pantone 13-0715 TCX Sea Mist` — `#D8C9A3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0715 TCX Sea Mist"))]
    SeaMist,
    /// `Pantone 13-0720 TCX Custard` — `#E5D68E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0720 TCX Custard"))]
    Custard,
    /// `Pantone 13-0725 TCX Raffia` — `#DAC483`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0725 TCX Raffia"))]
    Raffia,
    /// `Pantone 13-0739 TCX Cream Gold` — `#DEC05F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0739 TCX Cream Gold"))]
    CreamGold,
    /// `Pantone 13-0746 TCX Maize` — `#EEC843`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0746 TCX Maize"))]
    Maize,
    /// `Pantone 13-0752 TCX Lemon` — `#F3BF08`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0752 TCX Lemon"))]
    Lemon,
    /// `Pantone 13-0755 TCX Primrose Yellow` — `#F6D155`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0755 TCX Primrose Yellow"))]
    PrimroseYellow,
    /// `Pantone 13-0756 TCX Lemon Zest` — `#F9D857`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0756 TCX Lemon Zest"))]
    LemonZest,
    /// `Pantone 13-0758 TCX Dandelion` — `#FFD02E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0758 TCX Dandelion"))]
    Dandelion,
    /// `Pantone 13-0759 TCX Solar Power` — `#F4BF3A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0759 TCX Solar Power"))]
    SolarPower,
    /// `Pantone 13-0814 TCX Summer Melon` — `#EAD3AE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0814 TCX Summer Melon"))]
    SummerMelon,
    /// `Pantone 13-0815 TCX Banana Crepe` — `#E7D3AD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0815 TCX Banana Crepe"))]
    BananaCrepe,
    /// `Pantone 13-0822 TCX Sunlight` — `#EDD59E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0822 TCX Sunlight"))]
    Sunlight,
    /// `Pantone 13-0840 TCX Snapdragon` — `#FED777`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0840 TCX Snapdragon"))]
    Snapdragon,
    /// `Pantone 13-0849 TCX Habanero Gold` — `#FED450`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0849 TCX Habanero Gold"))]
    HabaneroGold,
    /// `Pantone 13-0850 TCX Aspen Gold` — `#FFD662`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0850 TCX Aspen Gold"))]
    AspenGold,
    /// `Pantone 13-0851 TCX Minion Yellow` — `#FED55D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0851 TCX Minion Yellow"))]
    MinionYellow,
    /// `Pantone 13-0858 TCX Vibrant Yellow` — `#FFDA29`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0858 TCX Vibrant Yellow"))]
    VibrantYellow,
    /// `Pantone 13-0859 TCX Lemon Chrome` — `#FFC300`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0859 TCX Lemon Chrome"))]
    LemonChrome,
    /// `Pantone 13-0905 TCX Birch` — `#DDD5C7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0905 TCX Birch"))]
    Birch,
    /// `Pantone 13-0907 TCX Sandshell` — `#D8CCBB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0907 TCX Sandshell"))]
    Sandshell,
    /// `Pantone 13-0908 TCX Parchment` — `#DFD1BE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0908 TCX Parchment"))]
    Parchment,
    /// `Pantone 13-0915 TCX Reed Yellow` — `#DCC99E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0915 TCX Reed Yellow"))]
    ReedYellow,
    /// `Pantone 13-0916 TCX Chamomile` — `#E8D0A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0916 TCX Chamomile"))]
    Chamomile,
    /// `Pantone 13-0917 TCX Italian Straw` — `#E7D1A1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0917 TCX Italian Straw"))]
    ItalianStraw,
    /// `Pantone 13-0919 TCX Soybean` — `#D2C29D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0919 TCX Soybean"))]
    Soybean,
    /// `Pantone 13-0922 TCX Straw` — `#E0C992`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0922 TCX Straw"))]
    Straw,
    /// `Pantone 13-0932 TCX Cornsilk` — `#EDC373`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0932 TCX Cornsilk"))]
    Cornsilk,
    /// `Pantone 13-0935 TCX Flax` — `#FFC87D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0935 TCX Flax"))]
    Flax,
    /// `Pantone 13-0939 TCX Golden Cream` — `#F7B768`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0939 TCX Golden Cream"))]
    GoldenCream,
    /// `Pantone 13-0940 TCX Sunset Gold` — `#F7C46C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0940 TCX Sunset Gold"))]
    SunsetGold,
    /// `Pantone 13-0941 TCX Banana Cream` — `#FFCF73`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0941 TCX Banana Cream"))]
    BananaCream,
    /// `Pantone 13-0942 TCX Amber Yellow` — `#FAB75A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0942 TCX Amber Yellow"))]
    AmberYellow,
    /// `Pantone 13-0945 TCX Pale Marigold` — `#FFC66E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0945 TCX Pale Marigold"))]
    PaleMarigold,
    /// `Pantone 13-0946 TCX Jurassic Gold` — `#E7AA56`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0946 TCX Jurassic Gold"))]
    JurassicGold,
    /// `Pantone 13-0947 TCX Banana` — `#FCB953`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-0947 TCX Banana"))]
    Banana,
    /// `Pantone 13-1006 TCX Creme Brulee` — `#DBCCB5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1006 TCX Creme Brulee"))]
    CremeBrulee,
    /// `Pantone 13-1007 TCX Oyster White` — `#D2CAAF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1007 TCX Oyster White"))]
    OysterWhite,
    /// `Pantone 13-1008 TCX Bleached Sand` — `#DACCB4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1008 TCX Bleached Sand"))]
    BleachedSand,
    /// `Pantone 13-1009 TCX Biscotti` — `#DAC7AB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1009 TCX Biscotti"))]
    Biscotti,
    /// `Pantone 13-1010 TCX Gray Sand` — `#E5CCAF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1010 TCX Gray Sand"))]
    GraySand,
    /// `Pantone 13-1011 TCX Ivory Cream` — `#DAC0A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1011 TCX Ivory Cream"))]
    IvoryCream,
    /// `Pantone 13-1012 TCX Frosted Almond` — `#D2C2AC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1012 TCX Frosted Almond"))]
    FrostedAlmond,
    /// `Pantone 13-1013 TCX Appleblossom` — `#DDBCA0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1013 TCX Appleblossom"))]
    Appleblossom,
    /// `Pantone 13-1014 TCX Mellow Buff` — `#D8B998`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1014 TCX Mellow Buff"))]
    MellowBuff,
    /// `Pantone 13-1015 TCX Honey Peach` — `#DCBD9E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1015 TCX Honey Peach"))]
    HoneyPeach,
    /// `Pantone 13-1016 TCX Wheat` — `#DEC5A5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1016 TCX Wheat"))]
    Wheat,
    /// `Pantone 13-1017 TCX Almond Cream` — `#F4C29F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1017 TCX Almond Cream"))]
    AlmondCream,
    /// `Pantone 13-1018 TCX Desert Dust` — `#E3BC8E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1018 TCX Desert Dust"))]
    DesertDust,
    /// `Pantone 13-1019 TCX Cream Blush` — `#F8C19A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1019 TCX Cream Blush"))]
    CreamBlush,
    /// `Pantone 13-1020 TCX Apricot Ice` — `#FBBE99`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1020 TCX Apricot Ice"))]
    ApricotIce,
    /// `Pantone 13-1021 TCX Prairie Sunset` — `#FFBB9E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1021 TCX Prairie Sunset"))]
    PrairieSunset,
    /// `Pantone 13-1022 TCX Caramel Cream` — `#F4BA94`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1022 TCX Caramel Cream"))]
    CaramelCream,
    /// `Pantone 13-1023 TCX Peach Fuzz` — `#FFBE98`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1023 TCX Peach Fuzz"))]
    PeachFuzz,
    /// `Pantone 13-1024 TCX Buff` — `#EBC396`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1024 TCX Buff"))]
    Buff,
    /// `Pantone 13-1025 TCX Impala` — `#F8CE97`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1025 TCX Impala"))]
    Impala,
    /// `Pantone 13-1026 TCX Creampuff` — `#FFCDA8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1026 TCX Creampuff"))]
    Creampuff,
    /// `Pantone 13-1027 TCX Apricot Cream` — `#F1BD89`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1027 TCX Apricot Cream"))]
    ApricotCream,
    /// `Pantone 13-1030 TCX Sunburst` — `#F6C289`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1030 TCX Sunburst"))]
    Sunburst,
    /// `Pantone 13-1031 TCX Apricot Sherbet` — `#FACD9E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1031 TCX Apricot Sherbet"))]
    ApricotSherbet,
    /// `Pantone 13-1105 TCX Brown Rice` — `#C7BBA4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1105 TCX Brown Rice"))]
    BrownRice,
    /// `Pantone 13-1106 TCX Sand Dollar` — `#DECDBE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1106 TCX Sand Dollar"))]
    SandDollar,
    /// `Pantone 13-1107 TCX Whisper Pink` — `#DACBBE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1107 TCX Whisper Pink"))]
    WhisperPink,
    /// `Pantone 13-1108 TCX Cream Tan` — `#E4C7B8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1108 TCX Cream Tan"))]
    CreamTan,
    /// `Pantone 13-1109 TCX Bisque` — `#EDCAB5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1109 TCX Bisque"))]
    Bisque,
    /// `Pantone 13-1114 TCX Bellini` — `#F4C9B1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1114 TCX Bellini"))]
    Bellini,
    /// `Pantone 13-1125 TCX Peach Quartz` — `#F5B895`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1125 TCX Peach Quartz"))]
    PeachQuartz,
    /// `Pantone 13-1208 TCX Peachy Keen` — `#E2BDB3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1208 TCX Peachy Keen"))]
    PeachyKeen,
    /// `Pantone 13-1308 TCX Brazilian Sand` — `#DACAB7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1308 TCX Brazilian Sand"))]
    BrazilianSand,
    /// `Pantone 13-1310 TCX English Rose` — `#F4C6C3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1310 TCX English Rose"))]
    EnglishRose,
    /// `Pantone 13-1318 TCX Tropical Peach` — `#FFC4B2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1318 TCX Tropical Peach"))]
    TropicalPeach,
    /// `Pantone 13-1404 TCX Pale Dogwood` — `#EDCDC2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1404 TCX Pale Dogwood"))]
    PaleDogwood,
    /// `Pantone 13-1405 TCX Shell` — `#E1CFC6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1405 TCX Shell"))]
    Shell,
    /// `Pantone 13-1406 TCX Cloud Pink` — `#F5D1C8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1406 TCX Cloud Pink"))]
    CloudPink,
    /// `Pantone 13-1407 TCX Creole Pink` — `#F7D5CC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1407 TCX Creole Pink"))]
    CreolePink,
    /// `Pantone 13-1408 TCX Chintz Rose` — `#EEC4BE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1408 TCX Chintz Rose"))]
    ChintzRose,
    /// `Pantone 13-1409 TCX Seashell Pink` — `#F7C8C2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1409 TCX Seashell Pink"))]
    SeashellPink,
    /// `Pantone 13-1504 TCX Peach Blush` — `#E4CCC6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1504 TCX Peach Blush"))]
    PeachBlush,
    /// `Pantone 13-1510 TCX Impatiens Pink` — `#FFC4BC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1510 TCX Impatiens Pink"))]
    ImpatiensPink,
    /// `Pantone 13-1511 TCX Pink Salt` — `#F7CDC7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1511 TCX Pink Salt"))]
    PinkSalt,
    /// `Pantone 13-1513 TCX Gossamer Pink` — `#FAC8C3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1513 TCX Gossamer Pink"))]
    GossamerPink,
    /// `Pantone 13-1520 TCX Rose Quartz` — `#F7CAC9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1520 TCX Rose Quartz"))]
    RoseQuartz,
    /// `Pantone 13-1904 TCX Chalk Pink` — `#E6C5CA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1904 TCX Chalk Pink"))]
    ChalkPink,
    /// `Pantone 13-1906 TCX Rose Shadow` — `#F9C2CD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-1906 TCX Rose Shadow"))]
    RoseShadow,
    /// `Pantone 13-2004 TCX Potpourri` — `#E7C9CA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-2004 TCX Potpourri"))]
    Potpourri,
    /// `Pantone 13-2005 TCX Strawberry Cream` — `#F4C3C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-2005 TCX Strawberry Cream"))]
    StrawberryCream,
    /// `Pantone 13-2006 TCX Almond Blossom` — `#F5BEC7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-2006 TCX Almond Blossom"))]
    AlmondBlossom,
    /// `Pantone 13-2010 TCX Orchid Pink` — `#F3BBCA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-2010 TCX Orchid Pink"))]
    OrchidPink,
    /// `Pantone 13-2802 TCX Fairy Tale` — `#F2C1D1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-2802 TCX Fairy Tale"))]
    FairyTale,
    /// `Pantone 13-2803 TCX Pale Lilac` — `#E1C6CC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-2803 TCX Pale Lilac"))]
    PaleLilac,
    /// `Pantone 13-2804 TCX Parfait Pink` — `#E9C3CF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-2804 TCX Parfait Pink"))]
    ParfaitPink,
    /// `Pantone 13-2805 TCX Pink Mist` — `#E6BCCD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-2805 TCX Pink Mist"))]
    PinkMist,
    /// `Pantone 13-2806 TCX Pink Lady` — `#EFC1D6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-2806 TCX Pink Lady"))]
    PinkLady,
    /// `Pantone 13-2807 TCX Ballerina` — `#F2CFDC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-2807 TCX Ballerina"))]
    Ballerina,
    /// `Pantone 13-2808 TCX Ballet Slipper` — `#EBCED5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-2808 TCX Ballet Slipper"))]
    BalletSlipper,
    /// `Pantone 13-3207 TCX Cherry Blossom` — `#F7CEE0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-3207 TCX Cherry Blossom"))]
    CherryBlossom,
    /// `Pantone 13-3405 TCX Lilac Snow` — `#E0C7D7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-3405 TCX Lilac Snow"))]
    LilacSnow,
    /// `Pantone 13-3406 TCX Orchid Ice` — `#E0D0DB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-3406 TCX Orchid Ice"))]
    OrchidIce,
    /// `Pantone 13-3801 TCX Crystal Gray` — `#D7CBC4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-3801 TCX Crystal Gray"))]
    CrystalGray,
    /// `Pantone 13-3802 TCX Orchid Tint` — `#DBD2DB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-3802 TCX Orchid Tint"))]
    OrchidTint,
    /// `Pantone 13-3803 TCX Lilac Ash` — `#D7CDCD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-3803 TCX Lilac Ash"))]
    LilacAsh,
    /// `Pantone 13-3804 TCX Gray Lilac` — `#D4CACD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-3804 TCX Gray Lilac"))]
    GrayLilac,
    /// `Pantone 13-3805 TCX Orchid Hush` — `#CEC3D2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-3805 TCX Orchid Hush"))]
    OrchidHush,
    /// `Pantone 13-3820 TCX Lavender Fog` — `#D2C4D6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-3820 TCX Lavender Fog"))]
    LavenderFog,
    /// `Pantone 13-3920 TCX Halogen Blue` — `#BDC6DC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-3920 TCX Halogen Blue"))]
    HalogenBlue,
    /// `Pantone 13-4103 TCX Illusion Blue` — `#C9D3DC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4103 TCX Illusion Blue"))]
    IllusionBlue,
    /// `Pantone 13-4104 TCX Antarctica` — `#C6C5C6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4104 TCX Antarctica"))]
    Antarctica,
    /// `Pantone 13-4105 TCX Lilac Hint` — `#D0D0DA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4105 TCX Lilac Hint"))]
    LilacHint,
    /// `Pantone 13-4108 TCX Nimbus Cloud` — `#D5D5D8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4108 TCX Nimbus Cloud"))]
    NimbusCloud,
    /// `Pantone 13-4110 TCX Arctic Ice` — `#BFC7D6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4110 TCX Arctic Ice"))]
    ArcticIce,
    /// `Pantone 13-4111 TCX Plein Air` — `#BFCAD6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4111 TCX Plein Air"))]
    PleinAir,
    /// `Pantone 13-4200 TCX Omphalodes` — `#B5CEDF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4200 TCX Omphalodes"))]
    Omphalodes,
    /// `Pantone 13-4201 TCX Oyster Mushroom` — `#C3C6C8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4201 TCX Oyster Mushroom"))]
    OysterMushroom,
    /// `Pantone 13-4303 TCX Dawn Blue` — `#CACCCB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4303 TCX Dawn Blue"))]
    DawnBlue,
    /// `Pantone 13-4304 TCX Ballad Blue` — `#C0CEDA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4304 TCX Ballad Blue"))]
    BalladBlue,
    /// `Pantone 13-4305 TCX Foggy Dew` — `#D1D5D0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4305 TCX Foggy Dew"))]
    FoggyDew,
    /// `Pantone 13-4308 TCX Baby Blue` — `#B5C7D3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4308 TCX Baby Blue"))]
    BabyBlue,
    /// `Pantone 13-4403 TCX Silver Birch` — `#D2CFC4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4403 TCX Silver Birch"))]
    SilverBirch,
    /// `Pantone 13-4404 TCX Ice Flow` — `#C6D2D2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4404 TCX Ice Flow"))]
    IceFlow,
    /// `Pantone 13-4405 TCX Misty Blue` — `#BFCDCC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4405 TCX Misty Blue"))]
    MistyBlue,
    /// `Pantone 13-4409 TCX Blue Glow` — `#B2D4DD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4409 TCX Blue Glow"))]
    BlueGlow,
    /// `Pantone 13-4411 TCX Crystal Blue` — `#A1C8DB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4411 TCX Crystal Blue"))]
    CrystalBlue,
    /// `Pantone 13-4720 TCX Tanager Turquoise` — `#91DCE8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4720 TCX Tanager Turquoise"))]
    TanagerTurquoise,
    /// `Pantone 13-4804 TCX Pale Blue` — `#C4D6D3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4804 TCX Pale Blue"))]
    PaleBlue,
    /// `Pantone 13-4809 TCX Plume` — `#A5CFD5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4809 TCX Plume"))]
    Plume,
    /// `Pantone 13-4810 TCX Limpet Shell` — `#98DDDE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4810 TCX Limpet Shell"))]
    LimpetShell,
    /// `Pantone 13-4909 TCX Blue Light` — `#ACDFDD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4909 TCX Blue Light"))]
    BlueLight,
    /// `Pantone 13-4910 TCX Blue Tint` — `#9FD9D7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-4910 TCX Blue Tint"))]
    BlueTint,
    /// `Pantone 13-5304 TCX Rainy Day` — `#CFC8BD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-5304 TCX Rainy Day"))]
    RainyDay,
    /// `Pantone 13-5305 TCX Pale Aqua` — `#C1CCC2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-5305 TCX Pale Aqua"))]
    PaleAqua,
    /// `Pantone 13-5306 TCX Icy Morn` — `#B0D3D1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-5306 TCX Icy Morn"))]
    IcyMorn,
    /// `Pantone 13-5309 TCX Pastel Turquoise` — `#99C5C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-5309 TCX Pastel Turquoise"))]
    PastelTurquoise,
    /// `Pantone 13-5313 TCX Aruba Blue` — `#81D7D3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-5313 TCX Aruba Blue"))]
    ArubaBlue,
    /// `Pantone 13-5409 TCX Yucca` — `#A1D7C9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-5409 TCX Yucca"))]
    Yucca,
    /// `Pantone 13-5410 TCX Iced Aqua` — `#ABD3DB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-5410 TCX Iced Aqua"))]
    IcedAqua,
    /// `Pantone 13-5412 TCX Beach Glass` — `#96DFCE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-5412 TCX Beach Glass"))]
    BeachGlass,
    /// `Pantone 13-5414 TCX Ice Green` — `#87D8C3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-5414 TCX Ice Green"))]
    IceGreen,
    /// `Pantone 13-5714 TCX Cabbage` — `#87D7BE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-5714 TCX Cabbage"))]
    Cabbage,
    /// `Pantone 13-5907 TCX Gossamer Green` — `#B2CFBE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-5907 TCX Gossamer Green"))]
    GossamerGreen,
    /// `Pantone 13-5911 TCX Birds Egg Green` — `#AACCB9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-5911 TCX Birds Egg Green"))]
    BirdsEggGreen,
    /// `Pantone 13-6006 TCX Almost Aqua` — `#CAD3C1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-6006 TCX Almost Aqua"))]
    AlmostAqua,
    /// `Pantone 13-6007 TCX Spray` — `#BED3BB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-6007 TCX Spray"))]
    Spray,
    /// `Pantone 13-6008 TCX Misty Jade` — `#BCD9C8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-6008 TCX Misty Jade"))]
    MistyJade,
    /// `Pantone 13-6009 TCX Brook Green` — `#AFDDCC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-6009 TCX Brook Green"))]
    BrookGreen,
    /// `Pantone 13-6105 TCX Celadon Tint` — `#CBCEBE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-6105 TCX Celadon Tint"))]
    CeladonTint,
    /// `Pantone 13-6106 TCX Green Tint` — `#C5CCC0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-6106 TCX Green Tint"))]
    GreenTint,
    /// `Pantone 13-6107 TCX Green Lily` — `#C1CEC1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-6107 TCX Green Lily"))]
    GreenLily,
    /// `Pantone 13-6108 TCX Celadon` — `#B8CCBA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-6108 TCX Celadon"))]
    Celadon,
    /// `Pantone 13-6110 TCX Mist Green` — `#AACEBC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-6110 TCX Mist Green"))]
    MistGreen,
    /// `Pantone 13-6208 TCX Bok Choy` — `#BCCAB3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 13-6208 TCX Bok Choy"))]
    BokChoy,
    /// `Pantone 14-0000 TCX Silver Gray` — `#C1B7B0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0000 TCX Silver Gray"))]
    SilverGray,
    /// `Pantone 14-0002 TCX Pumice Stone` — `#CAC2B9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0002 TCX Pumice Stone"))]
    PumiceStone,
    /// `Pantone 14-0105 TCX Overcast` — `#C3BDAB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0105 TCX Overcast"))]
    Overcast,
    /// `Pantone 14-0108 TCX Castle Wall` — `#C8C1AB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0108 TCX Castle Wall"))]
    CastleWall,
    /// `Pantone 14-0114 TCX Celadon Green` — `#B5C1A5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0114 TCX Celadon Green"))]
    CeladonGreen,
    /// `Pantone 14-0115 TCX Foam Green` — `#B4C79C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0115 TCX Foam Green"))]
    FoamGreen,
    /// `Pantone 14-0116 TCX Margarita` — `#B5C38E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0116 TCX Margarita"))]
    Margarita,
    /// `Pantone 14-0121 TCX Nile Green` — `#A7C796`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0121 TCX Nile Green"))]
    NileGreen,
    /// `Pantone 14-0123 TCX Arcadian Green` — `#A3C893`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0123 TCX Arcadian Green"))]
    ArcadianGreen,
    /// `Pantone 14-0127 TCX Greengage` — `#8BC28C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0127 TCX Greengage"))]
    Greengage,
    /// `Pantone 14-0156 TCX Summer Green` — `#7ED37F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0156 TCX Summer Green"))]
    SummerGreen,
    /// `Pantone 14-0210 TCX Tidal Foam` — `#BFB9A3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0210 TCX Tidal Foam"))]
    TidalFoam,
    /// `Pantone 14-0216 TCX Lint` — `#B6BA99`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0216 TCX Lint"))]
    Lint,
    /// `Pantone 14-0217 TCX Seedling` — `#C0CBA1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0217 TCX Seedling"))]
    Seedling,
    /// `Pantone 14-0223 TCX Nile` — `#B4BB85`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0223 TCX Nile"))]
    Nile,
    /// `Pantone 14-0226 TCX Opaline Green` — `#A3C57D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0226 TCX Opaline Green"))]
    OpalineGreen,
    /// `Pantone 14-0232 TCX Jade Lime` — `#A1CA7B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0232 TCX Jade Lime"))]
    JadeLime,
    /// `Pantone 14-0244 TCX Bright Lime Green` — `#97BC62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0244 TCX Bright Lime Green"))]
    BrightLimeGreen,
    /// `Pantone 14-0340 TCX Acid Lime` — `#BADF30`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0340 TCX Acid Lime"))]
    AcidLime,
    /// `Pantone 14-0418 TCX Bog` — `#BAB696`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0418 TCX Bog"))]
    Bog,
    /// `Pantone 14-0425 TCX Beechnut` — `#C2C18D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0425 TCX Beechnut"))]
    Beechnut,
    /// `Pantone 14-0434 TCX Green Banana` — `#BABC72`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0434 TCX Green Banana"))]
    GreenBanana,
    /// `Pantone 14-0445 TCX Bright Chartreuse` — `#B5BF50`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0445 TCX Bright Chartreuse"))]
    BrightChartreuse,
    /// `Pantone 14-0446 TCX Tender Shoots` — `#B5CC39`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0446 TCX Tender Shoots"))]
    TenderShoots,
    /// `Pantone 14-0452 TCX Lime Green` — `#9FC131`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0452 TCX Lime Green"))]
    LimeGreen,
    /// `Pantone 14-0615 TCX Green Haze` — `#CAC4A4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0615 TCX Green Haze"))]
    GreenHaze,
    /// `Pantone 14-0626 TCX Dried Moss` — `#CCB97E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0626 TCX Dried Moss"))]
    DriedMoss,
    /// `Pantone 14-0627 TCX Shadow Green` — `#CFC486`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0627 TCX Shadow Green"))]
    ShadowGreen,
    /// `Pantone 14-0636 TCX Muted Lime` — `#D1C87C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0636 TCX Muted Lime"))]
    MutedLime,
    /// `Pantone 14-0647 TCX Celery` — `#CEC153`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0647 TCX Celery"))]
    Celery,
    /// `Pantone 14-0708 TCX Cement` — `#C4B6A6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0708 TCX Cement"))]
    Cement,
    /// `Pantone 14-0721 TCX Hemp` — `#C0AD7C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0721 TCX Hemp"))]
    Hemp,
    /// `Pantone 14-0740 TCX Bamboo` — `#D2B04C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0740 TCX Bamboo"))]
    Bamboo,
    /// `Pantone 14-0754 TCX Super Lemon` — `#E4BF45`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0754 TCX Super Lemon"))]
    SuperLemon,
    /// `Pantone 14-0755 TCX Sulphur` — `#DDB614`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0755 TCX Sulphur"))]
    Sulphur,
    /// `Pantone 14-0756 TCX Empire Yellow` — `#F7D000`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0756 TCX Empire Yellow"))]
    EmpireYellow,
    /// `Pantone 14-0760 TCX Cyber Yellow` — `#FFD400`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0760 TCX Cyber Yellow"))]
    CyberYellow,
    /// `Pantone 14-0826 TCX Pampas` — `#CFBB7B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0826 TCX Pampas"))]
    Pampas,
    /// `Pantone 14-0827 TCX Dusky Citron` — `#E3CC81`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0827 TCX Dusky Citron"))]
    DuskyCitron,
    /// `Pantone 14-0837 TCX Misted Yellow` — `#DAB965`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0837 TCX Misted Yellow"))]
    MistedYellow,
    /// `Pantone 14-0846 TCX Yolk Yellow` — `#E2B051`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0846 TCX Yolk Yellow"))]
    YolkYellow,
    /// `Pantone 14-0847 TCX Buff Yellow` — `#F1BF70`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0847 TCX Buff Yellow"))]
    BuffYellow,
    /// `Pantone 14-0848 TCX Mimosa` — `#F0C05A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0848 TCX Mimosa"))]
    Mimosa,
    /// `Pantone 14-0850 TCX Daffodil` — `#FDC04E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0850 TCX Daffodil"))]
    Daffodil,
    /// `Pantone 14-0851 TCX Samoan Sun` — `#FBC85F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0851 TCX Samoan Sun"))]
    SamoanSun,
    /// `Pantone 14-0852 TCX Freesia` — `#F3C12C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0852 TCX Freesia"))]
    Freesia,
    /// `Pantone 14-0925 TCX Parsnip` — `#D6C69A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0925 TCX Parsnip"))]
    Parsnip,
    /// `Pantone 14-0935 TCX Jojoba` — `#DABE81`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0935 TCX Jojoba"))]
    Jojoba,
    /// `Pantone 14-0936 TCX Sahara Sun` — `#DFC08A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0936 TCX Sahara Sun"))]
    SaharaSun,
    /// `Pantone 14-0941 TCX Beeswax` — `#EBA851`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0941 TCX Beeswax"))]
    Beeswax,
    /// `Pantone 14-0951 TCX Golden Rod` — `#E2A829`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0951 TCX Golden Rod"))]
    GoldenRod,
    /// `Pantone 14-0952 TCX Spicy Mustard` — `#D8AE47`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0952 TCX Spicy Mustard"))]
    SpicyMustard,
    /// `Pantone 14-0955 TCX Citrus` — `#F9AC2F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0955 TCX Citrus"))]
    Citrus,
    /// `Pantone 14-0957 TCX Spectra Yellow` — `#F7B718`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-0957 TCX Spectra Yellow"))]
    SpectraYellow,
    /// `Pantone 14-1012 TCX Gilded Beige` — `#B39F8D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1012 TCX Gilded Beige"))]
    GildedBeige,
    /// `Pantone 14-1014 TCX Gravel` — `#CBBFA2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1014 TCX Gravel"))]
    Gravel,
    /// `Pantone 14-1025 TCX Cocoon` — `#C9B27C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1025 TCX Cocoon"))]
    Cocoon,
    /// `Pantone 14-1031 TCX Rattan` — `#D1B272`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1031 TCX Rattan"))]
    Rattan,
    /// `Pantone 14-1036 TCX Ochre` — `#D6AF66`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1036 TCX Ochre"))]
    Ochre,
    /// `Pantone 14-1038 TCX New Wheat` — `#D7B57F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1038 TCX New Wheat"))]
    NewWheat,
    /// `Pantone 14-1041 TCX Golden Apricot` — `#DDA758`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1041 TCX Golden Apricot"))]
    GoldenApricot,
    /// `Pantone 14-1045 TCX Amber` — `#EFAD55`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1045 TCX Amber"))]
    Amber,
    /// `Pantone 14-1050 TCX Marigold` — `#FADC53`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1050 TCX Marigold"))]
    Marigold,
    /// `Pantone 14-1051 TCX Warm Apricot` — `#FFB865`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1051 TCX Warm Apricot"))]
    WarmApricot,
    /// `Pantone 14-1052 TCX Kumquat` — `#FBAA4C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1052 TCX Kumquat"))]
    Kumquat,
    /// `Pantone 14-1064 TCX Saffron` — `#FFA500`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1064 TCX Saffron"))]
    Saffron,
    /// `Pantone 14-1106 TCX Peyote` — `#C5BBAE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1106 TCX Peyote"))]
    Peyote,
    /// `Pantone 14-1107 TCX Oyster Gray` — `#CBC1AE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1107 TCX Oyster Gray"))]
    OysterGray,
    /// `Pantone 14-1108 TCX Wood Ash` — `#D7CAB0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1108 TCX Wood Ash"))]
    WoodAsh,
    /// `Pantone 14-1110 TCX Boulder` — `#D1BE9B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1110 TCX Boulder"))]
    Boulder,
    /// `Pantone 14-1112 TCX Pebble` — `#CAB698`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1112 TCX Pebble"))]
    Pebble,
    /// `Pantone 14-1113 TCX Marzipan` — `#D8C09D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1113 TCX Marzipan"))]
    Marzipan,
    /// `Pantone 14-1116 TCX Almond Buff` — `#CCB390`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1116 TCX Almond Buff"))]
    AlmondBuff,
    /// `Pantone 14-1118 TCX Beige` — `#D5BA98`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1118 TCX Beige"))]
    Beige,
    /// `Pantone 14-1119 TCX Winter Wheat` — `#DFC09F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1119 TCX Winter Wheat"))]
    WinterWheat,
    /// `Pantone 14-1120 TCX Apricot Illusion` — `#E2C4A6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1120 TCX Apricot Illusion"))]
    ApricotIllusion,
    /// `Pantone 14-1122 TCX Sheepskin` — `#DAB58F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1122 TCX Sheepskin"))]
    Sheepskin,
    /// `Pantone 14-1127 TCX Desert Mist` — `#E0B589`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1127 TCX Desert Mist"))]
    DesertMist,
    /// `Pantone 14-1128 TCX Buff Orange` — `#FFBB7C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1128 TCX Buff Orange"))]
    BuffOrange,
    /// `Pantone 14-1133 TCX Apricot Nectar` — `#ECAA79`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1133 TCX Apricot Nectar"))]
    ApricotNectar,
    /// `Pantone 14-1135 TCX Salmon Buff` — `#FEAA7B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1135 TCX Salmon Buff"))]
    SalmonBuff,
    /// `Pantone 14-1139 TCX Pumpkin` — `#F5A26F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1139 TCX Pumpkin"))]
    Pumpkin,
    /// `Pantone 14-1159 TCX Zinnia` — `#FFA010`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1159 TCX Zinnia"))]
    Zinnia,
    /// `Pantone 14-1208 TCX Irish Cream` — `#C0AC92`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1208 TCX Irish Cream"))]
    IrishCream,
    /// `Pantone 14-1209 TCX Smoke Gray` — `#CEBAA8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1209 TCX Smoke Gray"))]
    SmokeGray,
    /// `Pantone 14-1210 TCX Shifting Sand` — `#D8C0AD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1210 TCX Shifting Sand"))]
    ShiftingSand,
    /// `Pantone 14-1212 TCX Frappe` — `#D1B7A0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1212 TCX Frappe"))]
    Frappe,
    /// `Pantone 14-1213 TCX Toasted Almond` — `#D2B49C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1213 TCX Toasted Almond"))]
    ToastedAlmond,
    /// `Pantone 14-1217 TCX Amberlight` — `#E2BEA2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1217 TCX Amberlight"))]
    Amberlight,
    /// `Pantone 14-1219 TCX Peach Parfait` — `#F8BFA8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1219 TCX Peach Parfait"))]
    PeachParfait,
    /// `Pantone 14-1220 TCX Peach Nougat` — `#E6AF91`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1220 TCX Peach Nougat"))]
    PeachNougat,
    /// `Pantone 14-1224 TCX Coral Sands` — `#EDAA86`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1224 TCX Coral Sands"))]
    CoralSands,
    /// `Pantone 14-1225 TCX Beach Sand` — `#FBB995`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1225 TCX Beach Sand"))]
    BeachSand,
    /// `Pantone 14-1227 TCX Peach` — `#F2A987`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1227 TCX Peach"))]
    Peach,
    /// `Pantone 14-1228 TCX Peach Nectar` — `#FFB59B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1228 TCX Peach Nectar"))]
    PeachNectar,
    /// `Pantone 14-1230 TCX Apricot Wash` — `#FBAC82`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1230 TCX Apricot Wash"))]
    ApricotWash,
    /// `Pantone 14-1231 TCX Peach Cobbler` — `#FFB181`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1231 TCX Peach Cobbler"))]
    PeachCobbler,
    /// `Pantone 14-1241 TCX Orange Chiffon` — `#F9AA7D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1241 TCX Orange Chiffon"))]
    OrangeChiffon,
    /// `Pantone 14-1305 TCX Mushroom` — `#BDACA3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1305 TCX Mushroom"))]
    Mushroom,
    /// `Pantone 14-1307 TCX Rose Dust` — `#CDB2A5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1307 TCX Rose Dust"))]
    RoseDust,
    /// `Pantone 14-1309 TCX Peach Whip` — `#DBBEB7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1309 TCX Peach Whip"))]
    PeachWhip,
    /// `Pantone 14-1310 TCX Cameo Rose` — `#D7B8AB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1310 TCX Cameo Rose"))]
    CameoRose,
    /// `Pantone 14-1311 TCX Evening Sand` — `#DDB6AB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1311 TCX Evening Sand"))]
    EveningSand,
    /// `Pantone 14-1312 TCX Pale Blush` — `#E4BFB3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1312 TCX Pale Blush"))]
    PaleBlush,
    /// `Pantone 14-1313 TCX Rose Cloud` — `#DBB0A2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1313 TCX Rose Cloud"))]
    RoseCloud,
    /// `Pantone 14-1314 TCX Spanish Villa` — `#DFBAA9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1314 TCX Spanish Villa"))]
    SpanishVilla,
    /// `Pantone 14-1315 TCX Hazelnut` — `#CFB095`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1315 TCX Hazelnut"))]
    Hazelnut,
    /// `Pantone 14-1316 TCX Dusty Pink` — `#DEAA9B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1316 TCX Dusty Pink"))]
    DustyPink,
    /// `Pantone 14-1318 TCX Coral Pink` — `#E8A798`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1318 TCX Coral Pink"))]
    CoralPink,
    /// `Pantone 14-1323 TCX Salmon` — `#FAAA94`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1323 TCX Salmon"))]
    Salmon,
    /// `Pantone 14-1324 TCX Peach Bud` — `#FDB2AB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1324 TCX Peach Bud"))]
    PeachBud,
    /// `Pantone 14-1418 TCX Peach Melba` — `#FBBDAF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1418 TCX Peach Melba"))]
    PeachMelba,
    /// `Pantone 14-1419 TCX Peach Pearl` — `#FFB2A5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1419 TCX Peach Pearl"))]
    PeachPearl,
    /// `Pantone 14-1420 TCX Apricot Blush` — `#FEAEA5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1420 TCX Apricot Blush"))]
    ApricotBlush,
    /// `Pantone 14-1506 TCX Rose Smoke` — `#D3B4AD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1506 TCX Rose Smoke"))]
    RoseSmoke,
    /// `Pantone 14-1508 TCX Silver Pink` — `#DCB1AF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1508 TCX Silver Pink"))]
    SilverPink,
    /// `Pantone 14-1511 TCX Powder Pink` — `#ECB2B3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1511 TCX Powder Pink"))]
    PowderPink,
    /// `Pantone 14-1513 TCX Blossom` — `#F2B2AE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1513 TCX Blossom"))]
    Blossom,
    /// `Pantone 14-1521 TCX Peaches N Cream` — `#F4A6A3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1521 TCX Peaches N Cream"))]
    PeachesNCream,
    /// `Pantone 14-1714 TCX Quartz Pink` — `#EFA6AA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1714 TCX Quartz Pink"))]
    QuartzPink,
    /// `Pantone 14-1803 TCX Sepia Rose` — `#D4BAB6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1803 TCX Sepia Rose"))]
    SepiaRose,
    /// `Pantone 14-1905 TCX Lotus` — `#E2C1C0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1905 TCX Lotus"))]
    Lotus,
    /// `Pantone 14-1907 TCX Peachskin` — `#DFB8B6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1907 TCX Peachskin"))]
    Peachskin,
    /// `Pantone 14-1909 TCX Coral Blush` — `#E6B2B8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1909 TCX Coral Blush"))]
    CoralBlush,
    /// `Pantone 14-1911 TCX Candy Pink` — `#F5B0BD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-1911 TCX Candy Pink"))]
    CandyPink,
    /// `Pantone 14-2305 TCX Pink Nectar` — `#D8AAB7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-2305 TCX Pink Nectar"))]
    PinkNectar,
    /// `Pantone 14-2307 TCX Cameo Pink` — `#DBA9B8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-2307 TCX Cameo Pink"))]
    CameoPink,
    /// `Pantone 14-2311 TCX Prism Pink` — `#F0A1BF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-2311 TCX Prism Pink"))]
    PrismPink,
    /// `Pantone 14-2710 TCX Lilac Sachet` — `#E9ADCA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-2710 TCX Lilac Sachet"))]
    LilacSachet,
    /// `Pantone 14-2808 TCX Sweet Lilac` — `#E8B5CE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-2808 TCX Sweet Lilac"))]
    SweetLilac,
    /// `Pantone 14-3204 TCX Fragrant Lilac` — `#CEADBE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-3204 TCX Fragrant Lilac"))]
    FragrantLilac,
    /// `Pantone 14-3206 TCX Winsome Orchid` — `#D4B9CB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-3206 TCX Winsome Orchid"))]
    WinsomeOrchid,
    /// `Pantone 14-3207 TCX Pink Lavender` — `#D9AFCA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-3207 TCX Pink Lavender"))]
    PinkLavender,
    /// `Pantone 14-3209 TCX Pastel Lavender` — `#D8A1C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-3209 TCX Pastel Lavender"))]
    PastelLavender,
    /// `Pantone 14-3612 TCX Orchid Bloom` — `#C5AECF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-3612 TCX Orchid Bloom"))]
    OrchidBloom,
    /// `Pantone 14-3710 TCX Orchid Petal` — `#BFB4CB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-3710 TCX Orchid Petal"))]
    OrchidPetal,
    /// `Pantone 14-3803 TCX Hushed Violet` — `#D1C0BF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-3803 TCX Hushed Violet"))]
    HushedViolet,
    /// `Pantone 14-3805 TCX Iris` — `#BAAFBC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-3805 TCX Iris"))]
    Iris,
    /// `Pantone 14-3812 TCX Pastel Lilac` — `#BDB0D0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-3812 TCX Pastel Lilac"))]
    PastelLilac,
    /// `Pantone 14-3903 TCX Lilac Marble` — `#C3BABF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-3903 TCX Lilac Marble"))]
    LilacMarble,
    /// `Pantone 14-3904 TCX Evening Haze` — `#BDB8C7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-3904 TCX Evening Haze"))]
    EveningHaze,
    /// `Pantone 14-3905 TCX Lavender Blue` — `#C5C0D0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-3905 TCX Lavender Blue"))]
    LavenderBlue,
    /// `Pantone 14-3906 TCX Raindrops` — `#B1AAB3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-3906 TCX Raindrops"))]
    Raindrops,
    /// `Pantone 14-3907 TCX Thistle` — `#B9B3C5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-3907 TCX Thistle"))]
    Thistle,
    /// `Pantone 14-3911 TCX Purple Heather` — `#BAB8D3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-3911 TCX Purple Heather"))]
    PurpleHeather,
    /// `Pantone 14-3912 TCX Zen Blue` — `#9FA9BE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-3912 TCX Zen Blue"))]
    ZenBlue,
    /// `Pantone 14-3949 TCX Xenon Blue` — `#B7C0D7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-3949 TCX Xenon Blue"))]
    XenonBlue,
    /// `Pantone 14-4002 TCX Wind Chime` — `#CAC5C2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4002 TCX Wind Chime"))]
    WindChime,
    /// `Pantone 14-4102 TCX Glacier Gray` — `#C5C6C7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4102 TCX Glacier Gray"))]
    GlacierGray,
    /// `Pantone 14-4103 TCX Gray Violet` — `#BBBCBC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4103 TCX Gray Violet"))]
    GrayViolet,
    /// `Pantone 14-4105 TCX Micro Chip` — `#BABCC0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4105 TCX Micro Chip"))]
    MicroChip,
    /// `Pantone 14-4106 TCX Gray Dawn` — `#BBC1CC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4106 TCX Gray Dawn"))]
    GrayDawn,
    /// `Pantone 14-4107 TCX Quiet Gray` — `#B9BABD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4107 TCX Quiet Gray"))]
    QuietGray,
    /// `Pantone 14-4110 TCX Heather` — `#B7C0D6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4110 TCX Heather"))]
    Heather,
    /// `Pantone 14-4112 TCX Skyway` — `#ADBED3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4112 TCX Skyway"))]
    Skyway,
    /// `Pantone 14-4115 TCX Cashmere Blue` — `#A5B8D0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4115 TCX Cashmere Blue"))]
    CashmereBlue,
    /// `Pantone 14-4121 TCX Blue Bell` — `#93B4D7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4121 TCX Blue Bell"))]
    BlueBell,
    /// `Pantone 14-4122 TCX Airy Blue` — `#92B6D5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4122 TCX Airy Blue"))]
    AiryBlue,
    /// `Pantone 14-4201 TCX Lunar Rock` — `#C5C5C5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4201 TCX Lunar Rock"))]
    LunarRock,
    /// `Pantone 14-4202 TCX Harbor Mist` — `#AFB1B4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4202 TCX Harbor Mist"))]
    HarborMist,
    /// `Pantone 14-4203 TCX Vapor Blue` — `#BEBDBD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4203 TCX Vapor Blue"))]
    VaporBlue,
    /// `Pantone 14-4206 TCX Pearl Blue` — `#B0B7BE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4206 TCX Pearl Blue"))]
    PearlBlue,
    /// `Pantone 14-4210 TCX Celestial Blue` — `#A3B4C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4210 TCX Celestial Blue"))]
    CelestialBlue,
    /// `Pantone 14-4214 TCX Powder Blue` — `#96B3D2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4214 TCX Powder Blue"))]
    PowderBlue,
    /// `Pantone 14-4306 TCX Cloud Blue` — `#A2B6B9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4306 TCX Cloud Blue"))]
    CloudBlue,
    /// `Pantone 14-4307 TCX Winter Sky` — `#A9C0CB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4307 TCX Winter Sky"))]
    WinterSky,
    /// `Pantone 14-4310 TCX Blue Topaz` — `#78BDD4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4310 TCX Blue Topaz"))]
    BlueTopaz,
    /// `Pantone 14-4311 TCX Corydalis Blue` — `#A9CADA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4311 TCX Corydalis Blue"))]
    CorydalisBlue,
    /// `Pantone 14-4313 TCX Aquamarine` — `#9DC3D4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4313 TCX Aquamarine"))]
    Aquamarine,
    /// `Pantone 14-4315 TCX Sea Angel` — `#98BFCA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4315 TCX Sea Angel"))]
    SeaAngel,
    /// `Pantone 14-4317 TCX Cool Blue` — `#A5C5D9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4317 TCX Cool Blue"))]
    CoolBlue,
    /// `Pantone 14-4318 TCX Sky Blue` — `#8ABAD3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4318 TCX Sky Blue"))]
    SkyBlue,
    /// `Pantone 14-4320 TCX Baltic Sea` — `#79B5DB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4320 TCX Baltic Sea"))]
    BalticSea,
    /// `Pantone 14-4500 TCX Moonstruck` — `#C2BEB6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4500 TCX Moonstruck"))]
    Moonstruck,
    /// `Pantone 14-4501 TCX Silver Lining` — `#BDB6AB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4501 TCX Silver Lining"))]
    SilverLining,
    /// `Pantone 14-4502 TCX Mercury` — `#BAC2BA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4502 TCX Mercury"))]
    Mercury,
    /// `Pantone 14-4503 TCX Metal` — `#BABFBC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4503 TCX Metal"))]
    Metal,
    /// `Pantone 14-4504 TCX Sky Gray` — `#BCC8C6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4504 TCX Sky Gray"))]
    SkyGray,
    /// `Pantone 14-4505 TCX Smoke` — `#BFC8C3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4505 TCX Smoke"))]
    Smoke,
    /// `Pantone 14-4506 TCX Ether` — `#9EB6B8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4506 TCX Ether"))]
    Ether,
    /// `Pantone 14-4508 TCX Stratosphere` — `#9EC1CC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4508 TCX Stratosphere"))]
    Stratosphere,
    /// `Pantone 14-4510 TCX Aquatic` — `#99C1CC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4510 TCX Aquatic"))]
    Aquatic,
    /// `Pantone 14-4511 TCX Gulf Stream` — `#88C3D0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4511 TCX Gulf Stream"))]
    GulfStream,
    /// `Pantone 14-4512 TCX Porcelain Blue` — `#95C0CB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4512 TCX Porcelain Blue"))]
    PorcelainBlue,
    /// `Pantone 14-4516 TCX Petit Four` — `#87C2D4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4516 TCX Petit Four"))]
    PetitFour,
    /// `Pantone 14-4522 TCX Bachelor Button` — `#4ABBD5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4522 TCX Bachelor Button"))]
    BachelorButton,
    /// `Pantone 14-4615 TCX Antiqua Sand` — `#83C2CD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4615 TCX Antiqua Sand"))]
    AntiquaSand,
    /// `Pantone 14-4620 TCX Island Paradise` — `#95DEE3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4620 TCX Island Paradise"))]
    IslandParadise,
    /// `Pantone 14-4710 TCX Tibetan Stone` — `#82C2C7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4710 TCX Tibetan Stone"))]
    TibetanStone,
    /// `Pantone 14-4804 TCX Blue Fox` — `#B9BCB6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4804 TCX Blue Fox"))]
    BlueFox,
    /// `Pantone 14-4807 TCX Surf Spray` — `#B4C8C2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4807 TCX Surf Spray"))]
    SurfSpray,
    /// `Pantone 14-4809 TCX Eggshell Blue` — `#A3CCC9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4809 TCX Eggshell Blue"))]
    EggshellBlue,
    /// `Pantone 14-4810 TCX Canal Blue` — `#9CC2C5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4810 TCX Canal Blue"))]
    CanalBlue,
    /// `Pantone 14-4811 TCX Aqua Sky` — `#7BC4C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4811 TCX Aqua Sky"))]
    AquaSky,
    /// `Pantone 14-4812 TCX Aqua Splash` — `#85CED1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4812 TCX Aqua Splash"))]
    AquaSplash,
    /// `Pantone 14-4814 TCX Angel Blue` — `#83C5CD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4814 TCX Angel Blue"))]
    AngelBlue,
    /// `Pantone 14-4816 TCX Blue Radiance` — `#58C9D4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4816 TCX Blue Radiance"))]
    BlueRadiance,
    /// `Pantone 14-4908 TCX Harbor Gray` — `#A8C0BB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-4908 TCX Harbor Gray"))]
    HarborGray,
    /// `Pantone 14-5002 TCX Silver` — `#A2A2A1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-5002 TCX Silver"))]
    Silver,
    /// `Pantone 14-5413 TCX Holiday` — `#81C3B4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-5413 TCX Holiday"))]
    Holiday,
    /// `Pantone 14-5416 TCX Bermuda` — `#60C9B3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-5416 TCX Bermuda"))]
    Bermuda,
    /// `Pantone 14-5420 TCX Cockatoo` — `#58C8B6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-5420 TCX Cockatoo"))]
    Cockatoo,
    /// `Pantone 14-5706 TCX Silt Green` — `#A9BDB1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-5706 TCX Silt Green"))]
    SiltGreen,
    /// `Pantone 14-5707 TCX Aqua Foam` — `#ADC3B4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-5707 TCX Aqua Foam"))]
    AquaFoam,
    /// `Pantone 14-5711 TCX Ocean Wave` — `#8EC5B6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-5711 TCX Ocean Wave"))]
    OceanWave,
    /// `Pantone 14-5713 TCX Cascade` — `#76C1B2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-5713 TCX Cascade"))]
    Cascade,
    /// `Pantone 14-5714 TCX Beveled Glass` — `#7ACCB8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-5714 TCX Beveled Glass"))]
    BeveledGlass,
    /// `Pantone 14-5718 TCX Opal` — `#77CFB7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-5718 TCX Opal"))]
    Opal,
    /// `Pantone 14-5721 TCX Electric Green` — `#4BC3A8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-5721 TCX Electric Green"))]
    ElectricGreen,
    /// `Pantone 14-6007 TCX Sea Foam` — `#B7C2B2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-6007 TCX Sea Foam"))]
    SeaFoam,
    /// `Pantone 14-6008 TCX Subtle Green` — `#B5CBBB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-6008 TCX Subtle Green"))]
    SubtleGreen,
    /// `Pantone 14-6011 TCX Grayed Jade` — `#9BBEA9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-6011 TCX Grayed Jade"))]
    GrayedJade,
    /// `Pantone 14-6017 TCX Neptune Green` — `#7FBB9E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-6017 TCX Neptune Green"))]
    NeptuneGreen,
    /// `Pantone 14-6305 TCX Pelican` — `#C1BCAC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-6305 TCX Pelican"))]
    Pelican,
    /// `Pantone 14-6308 TCX Alfalfa` — `#B7B59F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-6308 TCX Alfalfa"))]
    Alfalfa,
    /// `Pantone 14-6312 TCX Cameo Green` — `#AAC0AD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-6312 TCX Cameo Green"))]
    CameoGreen,
    /// `Pantone 14-6316 TCX Sprucestone` — `#9FC09C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-6316 TCX Sprucestone"))]
    Sprucestone,
    /// `Pantone 14-6319 TCX Meadow` — `#8BBA94`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-6319 TCX Meadow"))]
    Meadow,
    /// `Pantone 14-6324 TCX Peapod` — `#82B185`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-6324 TCX Peapod"))]
    Peapod,
    /// `Pantone 14-6327 TCX Zephyr Green` — `#7CB083`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-6327 TCX Zephyr Green"))]
    ZephyrGreen,
    /// `Pantone 14-6329 TCX Absinthe Green` — `#76B583`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-6329 TCX Absinthe Green"))]
    AbsintheGreen,
    /// `Pantone 14-6330 TCX Spring Bud` — `#6BCD9C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-6330 TCX Spring Bud"))]
    SpringBud,
    /// `Pantone 14-6340 TCX Spring Bouquet` — `#6DCE87`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-6340 TCX Spring Bouquet"))]
    SpringBouquet,
    /// `Pantone 14-6408 TCX Abbey Stone` — `#ABA798`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 14-6408 TCX Abbey Stone"))]
    AbbeyStone,
    /// `Pantone 15-0000 TCX Dove` — `#B3ADA7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0000 TCX Dove"))]
    Dove,
    /// `Pantone 15-0146 TCX Green Flash` — `#79C753`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0146 TCX Green Flash"))]
    GreenFlash,
    /// `Pantone 15-0309 TCX Spray Green` — `#AEA692`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0309 TCX Spray Green"))]
    SprayGreen,
    /// `Pantone 15-0318 TCX Sage Green` — `#B2AC88`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0318 TCX Sage Green"))]
    SageGreen,
    /// `Pantone 15-0326 TCX Tarragon` — `#A4AE77`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0326 TCX Tarragon"))]
    Tarragon,
    /// `Pantone 15-0332 TCX Leaf Green` — `#9FAF6C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0332 TCX Leaf Green"))]
    LeafGreen,
    /// `Pantone 15-0336 TCX Herbal Garden` — `#9CAD60`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0336 TCX Herbal Garden"))]
    HerbalGarden,
    /// `Pantone 15-0341 TCX Parrot Green` — `#8DB051`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0341 TCX Parrot Green"))]
    ParrotGreen,
    /// `Pantone 15-0343 TCX Greenery` — `#88B04B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0343 TCX Greenery"))]
    Greenery,
    /// `Pantone 15-0513 TCX Eucalyptus` — `#B1A992`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0513 TCX Eucalyptus"))]
    Eucalyptus,
    /// `Pantone 15-0522 TCX Pale Olive Green` — `#B5AD88`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0522 TCX Pale Olive Green"))]
    PaleOliveGreen,
    /// `Pantone 15-0523 TCX Winter Pear` — `#B0B487`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0523 TCX Winter Pear"))]
    WinterPear,
    /// `Pantone 15-0525 TCX Weeping Willow` — `#B3B17B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0525 TCX Weeping Willow"))]
    WeepingWillow,
    /// `Pantone 15-0531 TCX Sweet Pea` — `#A3A969`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0531 TCX Sweet Pea"))]
    SweetPea,
    /// `Pantone 15-0533 TCX Linden Green` — `#C4BF71`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0533 TCX Linden Green"))]
    LindenGreen,
    /// `Pantone 15-0535 TCX Palm` — `#AFAF5E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0535 TCX Palm"))]
    Palm,
    /// `Pantone 15-0538 TCX Green Oasis` — `#B0B454`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0538 TCX Green Oasis"))]
    GreenOasis,
    /// `Pantone 15-0543 TCX Apple Green` — `#B5B644`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0543 TCX Apple Green"))]
    AppleGreen,
    /// `Pantone 15-0545 TCX Jasmine Green` — `#7EC845`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0545 TCX Jasmine Green"))]
    JasmineGreen,
    /// `Pantone 15-0548 TCX Citronelle` — `#B8AF23`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0548 TCX Citronelle"))]
    Citronelle,
    /// `Pantone 15-0628 TCX Leek Green` — `#B7B17A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0628 TCX Leek Green"))]
    LeekGreen,
    /// `Pantone 15-0636 TCX Golden Green` — `#BDB369`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0636 TCX Golden Green"))]
    GoldenGreen,
    /// `Pantone 15-0643 TCX Cress Green` — `#BCA949`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0643 TCX Cress Green"))]
    CressGreen,
    /// `Pantone 15-0646 TCX Warm Olive` — `#C7B63C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0646 TCX Warm Olive"))]
    WarmOlive,
    /// `Pantone 15-0703 TCX Ashes Of Roses` — `#B5ACAB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0703 TCX Ashes Of Roses"))]
    AshesOfRoses,
    /// `Pantone 15-0719 TCX Silver Fern` — `#BBAA7E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0719 TCX Silver Fern"))]
    SilverFern,
    /// `Pantone 15-0730 TCX Southern Moss` — `#BCA66A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0730 TCX Southern Moss"))]
    SouthernMoss,
    /// `Pantone 15-0732 TCX Olivenite` — `#C1A65C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0732 TCX Olivenite"))]
    Olivenite,
    /// `Pantone 15-0743 TCX Oil Yellow` — `#C4A647`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0743 TCX Oil Yellow"))]
    OilYellow,
    /// `Pantone 15-0751 TCX Lemon Curry` — `#CDA323`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0751 TCX Lemon Curry"))]
    LemonCurry,
    /// `Pantone 15-0850 TCX Ceylon Yellow` — `#D4AE40`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0850 TCX Ceylon Yellow"))]
    CeylonYellow,
    /// `Pantone 15-0927 TCX Pale Gold` — `#BD9865`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0927 TCX Pale Gold"))]
    PaleGold,
    /// `Pantone 15-0942 TCX Sauterne` — `#C5A253`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0942 TCX Sauterne"))]
    Sauterne,
    /// `Pantone 15-0948 TCX Golden Spice` — `#C6973F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0948 TCX Golden Spice"))]
    GoldenSpice,
    /// `Pantone 15-0953 TCX Golden Yellow` — `#CB8E16`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0953 TCX Golden Yellow"))]
    GoldenYellow,
    /// `Pantone 15-0955 TCX Old Gold` — `#ECA825`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0955 TCX Old Gold"))]
    OldGold,
    /// `Pantone 15-0960 TCX Mango Mojito` — `#D69C2F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-0960 TCX Mango Mojito"))]
    MangoMojito,
    /// `Pantone 15-1020 TCX Ginger Root` — `#BFA58A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1020 TCX Ginger Root"))]
    GingerRoot,
    /// `Pantone 15-1040 TCX Iced Coffee` — `#B18F6A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1040 TCX Iced Coffee"))]
    IcedCoffee,
    /// `Pantone 15-1045 TCX Autumn Blaze` — `#D9922E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1045 TCX Autumn Blaze"))]
    AutumnBlaze,
    /// `Pantone 15-1046 TCX Mineral Yellow` — `#D39C43`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1046 TCX Mineral Yellow"))]
    MineralYellow,
    /// `Pantone 15-1049 TCX Artisans Gold` — `#F2AB46`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1049 TCX Artisans Gold"))]
    ArtisansGold,
    /// `Pantone 15-1050 TCX Golden Glow` — `#D99938`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1050 TCX Golden Glow"))]
    GoldenGlow,
    /// `Pantone 15-1051 TCX Golden Orange` — `#D7942D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1051 TCX Golden Orange"))]
    GoldenOrange,
    /// `Pantone 15-1054 TCX Cadmium Yellow` — `#EE9626`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1054 TCX Cadmium Yellow"))]
    CadmiumYellow,
    /// `Pantone 15-1058 TCX Radiant Yellow` — `#FC9E21`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1058 TCX Radiant Yellow"))]
    RadiantYellow,
    /// `Pantone 15-1062 TCX Gold Fusion` — `#FFB000`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1062 TCX Gold Fusion"))]
    GoldFusion,
    /// `Pantone 15-1114 TCX Travertine` — `#AE997D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1114 TCX Travertine"))]
    Travertine,
    /// `Pantone 15-1116 TCX Safari` — `#BAAA91`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1116 TCX Safari"))]
    Safari,
    /// `Pantone 15-1119 TCX Taos Taupe` — `#BFA77F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1119 TCX Taos Taupe"))]
    TaosTaupe,
    /// `Pantone 15-1125 TCX Porcini` — `#CCA580`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1125 TCX Porcini"))]
    Porcini,
    /// `Pantone 15-1132 TCX Fall Leaf` — `#C9A86A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1132 TCX Fall Leaf"))]
    FallLeaf,
    /// `Pantone 15-1142 TCX Honey Gold` — `#D1A054`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1142 TCX Honey Gold"))]
    HoneyGold,
    /// `Pantone 15-1145 TCX Chamois` — `#F7B26A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1145 TCX Chamois"))]
    Chamois,
    /// `Pantone 15-1147 TCX Butterscotch` — `#E19640`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1147 TCX Butterscotch"))]
    Butterscotch,
    /// `Pantone 15-1150 TCX Dark Cheddar` — `#E08119`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1150 TCX Dark Cheddar"))]
    DarkCheddar,
    /// `Pantone 15-1151 TCX Iceland Poppy` — `#F4963A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1151 TCX Iceland Poppy"))]
    IcelandPoppy,
    /// `Pantone 15-1153 TCX Apricot` — `#F19035`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1153 TCX Apricot"))]
    Apricot,
    /// `Pantone 15-1157 TCX Flame Orange` — `#FB8B23`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1157 TCX Flame Orange"))]
    FlameOrange,
    /// `Pantone 15-1160 TCX Blazing Orange` — `#FFA64F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1160 TCX Blazing Orange"))]
    BlazingOrange,
    /// `Pantone 15-1164 TCX Bright Marigold` — `#FF8D00`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1164 TCX Bright Marigold"))]
    BrightMarigold,
    /// `Pantone 15-1213 TCX Candied Ginger` — `#BFA387`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1213 TCX Candied Ginger"))]
    CandiedGinger,
    /// `Pantone 15-1214 TCX Warm Sand` — `#C5AE91`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1214 TCX Warm Sand"))]
    WarmSand,
    /// `Pantone 15-1215 TCX Sesame` — `#BAA38B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1215 TCX Sesame"))]
    Sesame,
    /// `Pantone 15-1216 TCX Pale Khaki` — `#BFAF92`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1216 TCX Pale Khaki"))]
    PaleKhaki,
    /// `Pantone 15-1217 TCX Mojave Desert` — `#C7B595`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1217 TCX Mojave Desert"))]
    MojaveDesert,
    /// `Pantone 15-1218 TCX Semolina` — `#CEB899`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1218 TCX Semolina"))]
    Semolina,
    /// `Pantone 15-1220 TCX Latte` — `#C5A582`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1220 TCX Latte"))]
    Latte,
    /// `Pantone 15-1225 TCX Sand` — `#CCA67F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1225 TCX Sand"))]
    Sand,
    /// `Pantone 15-1231 TCX Clay` — `#D2A172`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1231 TCX Clay"))]
    Clay,
    /// `Pantone 15-1234 TCX Gold Earth` — `#DD9C6B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1234 TCX Gold Earth"))]
    GoldEarth,
    /// `Pantone 15-1237 TCX Apricot Tan` — `#DD9760`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1237 TCX Apricot Tan"))]
    ApricotTan,
    /// `Pantone 15-1239 TCX Cantaloupe` — `#FFA177`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1239 TCX Cantaloupe"))]
    Cantaloupe,
    /// `Pantone 15-1242 TCX Muskmelon` — `#EC935E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1242 TCX Muskmelon"))]
    Muskmelon,
    /// `Pantone 15-1243 TCX Papaya` — `#FEA166`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1243 TCX Papaya"))]
    Papaya,
    /// `Pantone 15-1245 TCX Mock Orange` — `#FFA368`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1245 TCX Mock Orange"))]
    MockOrange,
    /// `Pantone 15-1247 TCX Tangerine` — `#F88F58`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1247 TCX Tangerine"))]
    Tangerine,
    /// `Pantone 15-1262 TCX Carrot Curl` — `#FE8C18`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1262 TCX Carrot Curl"))]
    CarrotCurl,
    /// `Pantone 15-1263 TCX Autumn Glory` — `#FF8812`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1263 TCX Autumn Glory"))]
    AutumnGlory,
    /// `Pantone 15-1264 TCX Turmeric` — `#FE840E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1264 TCX Turmeric"))]
    Turmeric,
    /// `Pantone 15-1304 TCX Humus` — `#B7A793`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1304 TCX Humus"))]
    Humus,
    /// `Pantone 15-1305 TCX Feather Gray` — `#B8AD9E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1305 TCX Feather Gray"))]
    FeatherGray,
    /// `Pantone 15-1306 TCX Oxford Tan` — `#B8A99A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1306 TCX Oxford Tan"))]
    OxfordTan,
    /// `Pantone 15-1307 TCX White Pepper` — `#B6A893`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1307 TCX White Pepper"))]
    WhitePepper,
    /// `Pantone 15-1308 TCX Doeskin` — `#BDAB9B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1308 TCX Doeskin"))]
    Doeskin,
    /// `Pantone 15-1309 TCX Moonlight` — `#C5B1A0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1309 TCX Moonlight"))]
    Moonlight,
    /// `Pantone 15-1314 TCX Cuban Sand` — `#C1A68D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1314 TCX Cuban Sand"))]
    CubanSand,
    /// `Pantone 15-1315 TCX Rugby Tan` — `#C2A594`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1315 TCX Rugby Tan"))]
    RugbyTan,
    /// `Pantone 15-1316 TCX Maple Sugar` — `#C9A38D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1316 TCX Maple Sugar"))]
    MapleSugar,
    /// `Pantone 15-1317 TCX Sirocco` — `#C39D88`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1317 TCX Sirocco"))]
    Sirocco,
    /// `Pantone 15-1318 TCX Pink Sand` — `#DFB19B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1318 TCX Pink Sand"))]
    PinkSand,
    /// `Pantone 15-1319 TCX Almost Apricot` — `#E5B39B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1319 TCX Almost Apricot"))]
    AlmostApricot,
    /// `Pantone 15-1322 TCX Dusty Coral` — `#D29B83`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1322 TCX Dusty Coral"))]
    DustyCoral,
    /// `Pantone 15-1327 TCX Peach Bloom` — `#D99B7C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1327 TCX Peach Bloom"))]
    PeachBloom,
    /// `Pantone 15-1331 TCX Coral Reef` — `#FAA181`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1331 TCX Coral Reef"))]
    CoralReef,
    /// `Pantone 15-1333 TCX Canyon Sunset` — `#E1927A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1333 TCX Canyon Sunset"))]
    CanyonSunset,
    /// `Pantone 15-1334 TCX Shell Coral` — `#EA9575`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1334 TCX Shell Coral"))]
    ShellCoral,
    /// `Pantone 15-1335 TCX Tangelo` — `#FE7E03`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1335 TCX Tangelo"))]
    Tangelo,
    /// `Pantone 15-1340 TCX Cadmium Orange` — `#F99471`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1340 TCX Cadmium Orange"))]
    CadmiumOrange,
    /// `Pantone 15-1415 TCX Coral Cloud` — `#E2A9A1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1415 TCX Coral Cloud"))]
    CoralCloud,
    /// `Pantone 15-1423 TCX Peach Amber` — `#FB9F93`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1423 TCX Peach Amber"))]
    PeachAmber,
    /// `Pantone 15-1425 TCX Fenugreek` — `#C0916C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1425 TCX Fenugreek"))]
    Fenugreek,
    /// `Pantone 15-1429 TCX Dusted Clay` — `#CC7357`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1429 TCX Dusted Clay"))]
    DustedClay,
    /// `Pantone 15-1430 TCX Pastry Shell` — `#BD8C66`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1430 TCX Pastry Shell"))]
    PastryShell,
    /// `Pantone 15-1433 TCX Papaya Punch` — `#FCA289`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1433 TCX Papaya Punch"))]
    PapayaPunch,
    /// `Pantone 15-1435 TCX Desert Flower` — `#FF9687`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1435 TCX Desert Flower"))]
    DesertFlower,
    /// `Pantone 15-1506 TCX Etherea` — `#A5958F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1506 TCX Etherea"))]
    Etherea,
    /// `Pantone 15-1511 TCX Mahogany Rose` — `#C5A193`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1511 TCX Mahogany Rose"))]
    MahoganyRose,
    /// `Pantone 15-1512 TCX Misty Rose` — `#CAA39A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1512 TCX Misty Rose"))]
    MistyRose,
    /// `Pantone 15-1515 TCX Mellow Rose` — `#D9A6A1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1515 TCX Mellow Rose"))]
    MellowRose,
    /// `Pantone 15-1516 TCX Peach Beige` — `#D3A297`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1516 TCX Peach Beige"))]
    PeachBeige,
    /// `Pantone 15-1520 TCX Blooming Dahlia` — `#EB9687`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1520 TCX Blooming Dahlia"))]
    BloomingDahlia,
    /// `Pantone 15-1523 TCX Shrimp` — `#E29A86`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1523 TCX Shrimp"))]
    Shrimp,
    /// `Pantone 15-1530 TCX Peach Pink` — `#FA9A85`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1530 TCX Peach Pink"))]
    PeachPink,
    /// `Pantone 15-1607 TCX Pale Mauve` — `#C6A4A4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1607 TCX Pale Mauve"))]
    PaleMauve,
    /// `Pantone 15-1611 TCX Bridal Rose` — `#D69FA2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1611 TCX Bridal Rose"))]
    BridalRose,
    /// `Pantone 15-1614 TCX Blush` — `#D1969A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1614 TCX Blush"))]
    Blush,
    /// `Pantone 15-1621 TCX Candlelight Peach` — `#F8A39D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1621 TCX Candlelight Peach"))]
    CandlelightPeach,
    /// `Pantone 15-1624 TCX Conch Shell` — `#FC8F9B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1624 TCX Conch Shell"))]
    ConchShell,
    /// `Pantone 15-1626 TCX Salmon Rose` — `#FF8D94`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1626 TCX Salmon Rose"))]
    SalmonRose,
    /// `Pantone 15-1717 TCX Pink Icing` — `#EEA0A6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1717 TCX Pink Icing"))]
    PinkIcing,
    /// `Pantone 15-1816 TCX Peony` — `#ED9CA8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1816 TCX Peony"))]
    Peony,
    /// `Pantone 15-1821 TCX Flamingo Pink` — `#F7969E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1821 TCX Flamingo Pink"))]
    FlamingoPink,
    /// `Pantone 15-1905 TCX Burnished Lilac` — `#C5AEB1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1905 TCX Burnished Lilac"))]
    BurnishedLilac,
    /// `Pantone 15-1906 TCX Zephyr` — `#C89FA5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1906 TCX Zephyr"))]
    Zephyr,
    /// `Pantone 15-1912 TCX Sea Pink` — `#DE98AB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1912 TCX Sea Pink"))]
    SeaPink,
    /// `Pantone 15-1920 TCX Morning Glory` — `#EE819F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1920 TCX Morning Glory"))]
    MorningGlory,
    /// `Pantone 15-1922 TCX Geranium Pink` — `#F6909D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-1922 TCX Geranium Pink"))]
    GeraniumPink,
    /// `Pantone 15-2205 TCX Dawn Pink` — `#BFA3AF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-2205 TCX Dawn Pink"))]
    DawnPink,
    /// `Pantone 15-2210 TCX Orchid Smoke` — `#D294AA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-2210 TCX Orchid Smoke"))]
    OrchidSmoke,
    /// `Pantone 15-2214 TCX Rosebloom` — `#E290B2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-2214 TCX Rosebloom"))]
    Rosebloom,
    /// `Pantone 15-2215 TCX Begonia Pink` — `#EC9ABE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-2215 TCX Begonia Pink"))]
    BegoniaPink,
    /// `Pantone 15-2216 TCX Sachet Pink` — `#F18AAD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-2216 TCX Sachet Pink"))]
    SachetPink,
    /// `Pantone 15-2217 TCX Aurora Pink` — `#E881A6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-2217 TCX Aurora Pink"))]
    AuroraPink,
    /// `Pantone 15-2705 TCX Keepsake Lilac` — `#C0A5AE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-2705 TCX Keepsake Lilac"))]
    KeepsakeLilac,
    /// `Pantone 15-2706 TCX Violet Ice` — `#C2ACB1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-2706 TCX Violet Ice"))]
    VioletIce,
    /// `Pantone 15-2718 TCX Fuchsia Pink` — `#DF88B7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-2718 TCX Fuchsia Pink"))]
    FuchsiaPink,
    /// `Pantone 15-2913 TCX Lilac Chiffon` — `#DE9BC4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-2913 TCX Lilac Chiffon"))]
    LilacChiffon,
    /// `Pantone 15-3207 TCX Mauve Mist` — `#C49BD4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3207 TCX Mauve Mist"))]
    MauveMist,
    /// `Pantone 15-3214 TCX Orchid` — `#D198C5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3214 TCX Orchid"))]
    Orchid,
    /// `Pantone 15-3412 TCX Orchid Bouquet` — `#D1ACCE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3412 TCX Orchid Bouquet"))]
    OrchidBouquet,
    /// `Pantone 15-3507 TCX Lavender Frost` — `#BDABBE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3507 TCX Lavender Frost"))]
    LavenderFrost,
    /// `Pantone 15-3508 TCX Fair Orchid` — `#C0AAC0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3508 TCX Fair Orchid"))]
    FairOrchid,
    /// `Pantone 15-3520 TCX Crocus Petal` — `#B99BC5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3520 TCX Crocus Petal"))]
    CrocusPetal,
    /// `Pantone 15-3620 TCX Lavendula` — `#BCA4CB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3620 TCX Lavendula"))]
    Lavendula,
    /// `Pantone 15-3716 TCX Purple Rose` — `#B09FCA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3716 TCX Purple Rose"))]
    PurpleRose,
    /// `Pantone 15-3720 TCX Lilac Breeze` — `#B3A0C9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3720 TCX Lilac Breeze"))]
    LilacBreeze,
    /// `Pantone 15-3800 TCX Porpoise` — `#A7A19E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3800 TCX Porpoise"))]
    Porpoise,
    /// `Pantone 15-3802 TCX Cloud Gray` — `#B7A9AC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3802 TCX Cloud Gray"))]
    CloudGray,
    /// `Pantone 15-3807 TCX Misty Lilac` — `#BCB4C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3807 TCX Misty Lilac"))]
    MistyLilac,
    /// `Pantone 15-3817 TCX Lavender` — `#AFA4CE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3817 TCX Lavender"))]
    Lavender,
    /// `Pantone 15-3908 TCX Icelandic Blue` — `#A9ADC2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3908 TCX Icelandic Blue"))]
    IcelandicBlue,
    /// `Pantone 15-3909 TCX Cosmic Sky` — `#AAAAC4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3909 TCX Cosmic Sky"))]
    CosmicSky,
    /// `Pantone 15-3910 TCX Languid Lavender` — `#A2A1BA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3910 TCX Languid Lavender"))]
    LanguidLavender,
    /// `Pantone 15-3912 TCX Aleutian` — `#9A9EB3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3912 TCX Aleutian"))]
    Aleutian,
    /// `Pantone 15-3915 TCX Kentucky Blue` — `#A5B3CC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3915 TCX Kentucky Blue"))]
    KentuckyBlue,
    /// `Pantone 15-3919 TCX Serenity` — `#91A8D0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3919 TCX Serenity"))]
    Serenity,
    /// `Pantone 15-3920 TCX Placid Blue` — `#8CADD3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3920 TCX Placid Blue"))]
    PlacidBlue,
    /// `Pantone 15-3930 TCX Vista Blue` — `#81A0D4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3930 TCX Vista Blue"))]
    VistaBlue,
    /// `Pantone 15-3932 TCX Bel Air Blue` — `#819AC1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-3932 TCX Bel Air Blue"))]
    BelAirBlue,
    /// `Pantone 15-4003 TCX Storm Gray` — `#B5BAB6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4003 TCX Storm Gray"))]
    StormGray,
    /// `Pantone 15-4005 TCX Dream Blue` — `#A0BCD0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4005 TCX Dream Blue"))]
    DreamBlue,
    /// `Pantone 15-4008 TCX Blue Fog` — `#9BABBB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4008 TCX Blue Fog"))]
    BlueFog,
    /// `Pantone 15-4020 TCX Cerulean` — `#9BB7D4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4020 TCX Cerulean"))]
    Cerulean,
    /// `Pantone 15-4030 TCX Chambray Blue` — `#9EB4D3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4030 TCX Chambray Blue"))]
    ChambrayBlue,
    /// `Pantone 15-4101 TCX High Rise` — `#AEB2B5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4101 TCX High Rise"))]
    HighRise,
    /// `Pantone 15-4105 TCX Angel Falls` — `#A3BDD3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4105 TCX Angel Falls"))]
    AngelFalls,
    /// `Pantone 15-4225 TCX Alaskan Blue` — `#6DA9D2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4225 TCX Alaskan Blue"))]
    AlaskanBlue,
    /// `Pantone 15-4305 TCX Quarry` — `#98A0A5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4305 TCX Quarry"))]
    Quarry,
    /// `Pantone 15-4306 TCX Belgian Block` — `#A3A9A6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4306 TCX Belgian Block"))]
    BelgianBlock,
    /// `Pantone 15-4307 TCX Tradewinds` — `#7F8793`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4307 TCX Tradewinds"))]
    Tradewinds,
    /// `Pantone 15-4309 TCX Sterling Blue` — `#A2B9C2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4309 TCX Sterling Blue"))]
    SterlingBlue,
    /// `Pantone 15-4312 TCX Forget Me Not` — `#8FADBD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4312 TCX Forget Me Not"))]
    ForgetMeNot,
    /// `Pantone 15-4319 TCX Air Blue` — `#77ACC7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4319 TCX Air Blue"))]
    AirBlue,
    /// `Pantone 15-4323 TCX Ethereal Blue` — `#5CA6CE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4323 TCX Ethereal Blue"))]
    EtherealBlue,
    /// `Pantone 15-4415 TCX Milky Blue` — `#72A8BA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4415 TCX Milky Blue"))]
    MilkyBlue,
    /// `Pantone 15-4421 TCX Blue Grotto` — `#5CACCE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4421 TCX Blue Grotto"))]
    BlueGrotto,
    /// `Pantone 15-4427 TCX Norse Blue` — `#4CA5C7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4427 TCX Norse Blue"))]
    NorseBlue,
    /// `Pantone 15-4428 TCX Crystal Seas` — `#5DAFCE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4428 TCX Crystal Seas"))]
    CrystalSeas,
    /// `Pantone 15-4502 TCX Silver Cloud` — `#BEB7B0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4502 TCX Silver Cloud"))]
    SilverCloud,
    /// `Pantone 15-4503 TCX Chateau Gray` — `#BBB1A8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4503 TCX Chateau Gray"))]
    ChateauGray,
    /// `Pantone 15-4702 TCX Puritan Gray` — `#A8B0AE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4702 TCX Puritan Gray"))]
    PuritanGray,
    /// `Pantone 15-4703 TCX Mirage Gray` — `#ABAFAE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4703 TCX Mirage Gray"))]
    MirageGray,
    /// `Pantone 15-4704 TCX Pigeon` — `#A9AFAA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4704 TCX Pigeon"))]
    Pigeon,
    /// `Pantone 15-4706 TCX Gray Mist` — `#99AEAE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4706 TCX Gray Mist"))]
    GrayMist,
    /// `Pantone 15-4707 TCX Blue Haze` — `#A5BCBB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4707 TCX Blue Haze"))]
    BlueHaze,
    /// `Pantone 15-4712 TCX Marine Blue` — `#76AFB6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4712 TCX Marine Blue"))]
    MarineBlue,
    /// `Pantone 15-4714 TCX Aquarelle` — `#61AAB1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4714 TCX Aquarelle"))]
    Aquarelle,
    /// `Pantone 15-4715 TCX Aqua Sea` — `#6BAAAE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4715 TCX Aqua Sea"))]
    AquaSea,
    /// `Pantone 15-4717 TCX Aqua` — `#64A1AD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4717 TCX Aqua"))]
    Aqua,
    /// `Pantone 15-4720 TCX River Blue` — `#38AFCD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4720 TCX River Blue"))]
    RiverBlue,
    /// `Pantone 15-4722 TCX Capri` — `#44BBCA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4722 TCX Capri"))]
    Capri,
    /// `Pantone 15-4825 TCX Blue Curacao` — `#32BECC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-4825 TCX Blue Curacao"))]
    BlueCuracao,
    /// `Pantone 15-5205 TCX Aqua Gray` — `#A5B2AA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5205 TCX Aqua Gray"))]
    AquaGray,
    /// `Pantone 15-5207 TCX Aquifer` — `#89ACAC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5207 TCX Aquifer"))]
    Aquifer,
    /// `Pantone 15-5209 TCX Aqua Haze` — `#87B9BC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5209 TCX Aqua Haze"))]
    AquaHaze,
    /// `Pantone 15-5210 TCX Nile Blue` — `#76A7AB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5210 TCX Nile Blue"))]
    NileBlue,
    /// `Pantone 15-5217 TCX Blue Turquoise` — `#53B0AE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5217 TCX Blue Turquoise"))]
    BlueTurquoise,
    /// `Pantone 15-5218 TCX Pool Blue` — `#67BCB3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5218 TCX Pool Blue"))]
    PoolBlue,
    /// `Pantone 15-5416 TCX Florida Keys` — `#56BEAB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5416 TCX Florida Keys"))]
    FloridaKeys,
    /// `Pantone 15-5421 TCX Aqua Green` — `#00B89F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5421 TCX Aqua Green"))]
    AquaGreen,
    /// `Pantone 15-5425 TCX Atlantis` — `#00AF9F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5425 TCX Atlantis"))]
    Atlantis,
    /// `Pantone 15-5516 TCX Waterfall` — `#3AB0A2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5516 TCX Waterfall"))]
    Waterfall,
    /// `Pantone 15-5519 TCX Turquoise` — `#45B5AA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5519 TCX Turquoise"))]
    Turquoise,
    /// `Pantone 15-5534 TCX Bright Green` — `#009B5C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5534 TCX Bright Green"))]
    BrightGreen,
    /// `Pantone 15-5704 TCX Mineral Gray` — `#B2B6AC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5704 TCX Mineral Gray"))]
    MineralGray,
    /// `Pantone 15-5706 TCX Frosty Green` — `#A3B5A6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5706 TCX Frosty Green"))]
    FrostyGreen,
    /// `Pantone 15-5711 TCX Dusty Jade Green` — `#7BB5A3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5711 TCX Dusty Jade Green"))]
    DustyJadeGreen,
    /// `Pantone 15-5718 TCX Biscay Green` — `#55C6A9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5718 TCX Biscay Green"))]
    BiscayGreen,
    /// `Pantone 15-5728 TCX Mint Leaf` — `#00B694`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5728 TCX Mint Leaf"))]
    MintLeaf,
    /// `Pantone 15-5812 TCX Lichen` — `#9BC2B1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5812 TCX Lichen"))]
    Lichen,
    /// `Pantone 15-5819 TCX Spearmint` — `#64BFA4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-5819 TCX Spearmint"))]
    Spearmint,
    /// `Pantone 15-6114 TCX Hemlock` — `#97C1A1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6114 TCX Hemlock"))]
    Hemlock,
    /// `Pantone 15-6120 TCX Ming` — `#7CB08A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6120 TCX Ming"))]
    Ming,
    /// `Pantone 15-6123 TCX Jade Cream` — `#60B892`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6123 TCX Jade Cream"))]
    JadeCream,
    /// `Pantone 15-6304 TCX Pussywillow Gray` — `#AEACA1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6304 TCX Pussywillow Gray"))]
    PussywillowGray,
    /// `Pantone 15-6307 TCX Agate Gray` — `#B1B09F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6307 TCX Agate Gray"))]
    AgateGray,
    /// `Pantone 15-6310 TCX Swamp` — `#A8B197`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6310 TCX Swamp"))]
    Swamp,
    /// `Pantone 15-6313 TCX Laurel Green` — `#ADBBA1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6313 TCX Laurel Green"))]
    LaurelGreen,
    /// `Pantone 15-6315 TCX Smoke Green` — `#A8BBA2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6315 TCX Smoke Green"))]
    SmokeGreen,
    /// `Pantone 15-6316 TCX Fair Green` — `#92AF88`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6316 TCX Fair Green"))]
    FairGreen,
    /// `Pantone 15-6317 TCX Quiet Green` — `#9EBC97`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6317 TCX Quiet Green"))]
    QuietGreen,
    /// `Pantone 15-6322 TCX Light Grass Green` — `#7CB68E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6322 TCX Light Grass Green"))]
    LightGrassGreen,
    /// `Pantone 15-6340 TCX Irish Green` — `#45BE76`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6340 TCX Irish Green"))]
    IrishGreen,
    /// `Pantone 15-6410 TCX Moss Gray` — `#AFAB97`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6410 TCX Moss Gray"))]
    MossGray,
    /// `Pantone 15-6414 TCX Reseda` — `#A1AD92`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6414 TCX Reseda"))]
    Reseda,
    /// `Pantone 15-6423 TCX Forest Shade` — `#91AC80`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6423 TCX Forest Shade"))]
    ForestShade,
    /// `Pantone 15-6428 TCX Green Tea` — `#86A96F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6428 TCX Green Tea"))]
    GreenTea,
    /// `Pantone 15-6432 TCX Shamrock` — `#6FA26B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6432 TCX Shamrock"))]
    Shamrock,
    /// `Pantone 15-6437 TCX Grass Green` — `#7BB369`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6437 TCX Grass Green"))]
    GrassGreen,
    /// `Pantone 15-6442 TCX Bud Green` — `#79B465`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 15-6442 TCX Bud Green"))]
    BudGreen,
    /// `Pantone 16-0000 TCX Paloma` — `#9F9C99`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0000 TCX Paloma"))]
    Paloma,
    /// `Pantone 16-0110 TCX Desert Sage` — `#A7AE9E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0110 TCX Desert Sage"))]
    DesertSage,
    /// `Pantone 16-0123 TCX Tendril` — `#89A06B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0123 TCX Tendril"))]
    Tendril,
    /// `Pantone 16-0205 TCX Vintage Khaki` — `#9A9186`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0205 TCX Vintage Khaki"))]
    VintageKhaki,
    /// `Pantone 16-0207 TCX London Fog` — `#A29E92`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0207 TCX London Fog"))]
    LondonFog,
    /// `Pantone 16-0213 TCX Tea` — `#999B85`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0213 TCX Tea"))]
    Tea,
    /// `Pantone 16-0220 TCX Mistletoe` — `#8AA282`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0220 TCX Mistletoe"))]
    Mistletoe,
    /// `Pantone 16-0224 TCX Green Eyes` — `#7D956D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0224 TCX Green Eyes"))]
    GreenEyes,
    /// `Pantone 16-0228 TCX Jade Green` — `#759465`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0228 TCX Jade Green"))]
    JadeGreen,
    /// `Pantone 16-0230 TCX Macaw Green` — `#9BB53E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0230 TCX Macaw Green"))]
    MacawGreen,
    /// `Pantone 16-0233 TCX Meadow Green` — `#739957`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0233 TCX Meadow Green"))]
    MeadowGreen,
    /// `Pantone 16-0235 TCX Kiwi` — `#7AAB55`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0235 TCX Kiwi"))]
    Kiwi,
    /// `Pantone 16-0237 TCX Foliage` — `#75A14F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0237 TCX Foliage"))]
    Foliage,
    /// `Pantone 16-0421 TCX Sage` — `#91946E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0421 TCX Sage"))]
    Sage,
    /// `Pantone 16-0430 TCX Fern` — `#9AA067`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0430 TCX Fern"))]
    Fern,
    /// `Pantone 16-0435 TCX Dark Citron` — `#A0AC4F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0435 TCX Dark Citron"))]
    DarkCitron,
    /// `Pantone 16-0439 TCX Spinach Green` — `#909B4C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0439 TCX Spinach Green"))]
    SpinachGreen,
    /// `Pantone 16-0518 TCX Gray Green` — `#A49A79`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0518 TCX Gray Green"))]
    GrayGreen,
    /// `Pantone 16-0526 TCX Cedar` — `#928E64`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0526 TCX Cedar"))]
    Cedar,
    /// `Pantone 16-0532 TCX Moss` — `#A09D59`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0532 TCX Moss"))]
    Moss,
    /// `Pantone 16-0540 TCX Oasis` — `#A3A04E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0540 TCX Oasis"))]
    Oasis,
    /// `Pantone 16-0543 TCX Golden Lime` — `#9A9738`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0543 TCX Golden Lime"))]
    GoldenLime,
    /// `Pantone 16-0545 TCX Split Pea` — `#9C9A40`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0545 TCX Split Pea"))]
    SplitPea,
    /// `Pantone 16-0550 TCX Lentil Sprout` — `#ABA44D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0550 TCX Lentil Sprout"))]
    LentilSprout,
    /// `Pantone 16-0613 TCX Elm` — `#A39F86`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0613 TCX Elm"))]
    Elm,
    /// `Pantone 16-0632 TCX Willow` — `#9A8B4F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0632 TCX Willow"))]
    Willow,
    /// `Pantone 16-0639 TCX Golden Olive` — `#AF9841`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0639 TCX Golden Olive"))]
    GoldenOlive,
    /// `Pantone 16-0713 TCX Slate Green` — `#A0987C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0713 TCX Slate Green"))]
    SlateGreen,
    /// `Pantone 16-0726 TCX Khaki` — `#A39264`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0726 TCX Khaki"))]
    Khaki,
    /// `Pantone 16-0730 TCX Antique Gold` — `#B59E5F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0730 TCX Antique Gold"))]
    AntiqueGold,
    /// `Pantone 16-0737 TCX Burnished Gold` — `#AA9855`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0737 TCX Burnished Gold"))]
    BurnishedGold,
    /// `Pantone 16-0742 TCX Green Sulphur` — `#AE8E2C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0742 TCX Green Sulphur"))]
    GreenSulphur,
    /// `Pantone 16-0806 TCX Goat` — `#A89A91`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0806 TCX Goat"))]
    Goat,
    /// `Pantone 16-0836 TCX Rich Gold` — `#C8B273`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0836 TCX Rich Gold"))]
    RichGold,
    /// `Pantone 16-0840 TCX Antique Moss` — `#B9A023`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0840 TCX Antique Moss"))]
    AntiqueMoss,
    /// `Pantone 16-0847 TCX Olive Oil` — `#A98B2D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0847 TCX Olive Oil"))]
    OliveOil,
    /// `Pantone 16-0906 TCX Simply Taupe` — `#AD9F93`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0906 TCX Simply Taupe"))]
    SimplyTaupe,
    /// `Pantone 16-0920 TCX Curds And Whey` — `#BCA483`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0920 TCX Curds And Whey"))]
    CurdsAndWhey,
    /// `Pantone 16-0924 TCX Croissant` — `#C4AB86`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0924 TCX Croissant"))]
    Croissant,
    /// `Pantone 16-0928 TCX Curry` — `#BE9E6F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0928 TCX Curry"))]
    Curry,
    /// `Pantone 16-0940 TCX Taffy` — `#C39B6A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0940 TCX Taffy"))]
    Taffy,
    /// `Pantone 16-0945 TCX Tinsel` — `#C3964D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0945 TCX Tinsel"))]
    Tinsel,
    /// `Pantone 16-0946 TCX Honey` — `#BA9238`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0946 TCX Honey"))]
    Honey,
    /// `Pantone 16-0947 TCX Bright Gold` — `#CF9F52`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0947 TCX Bright Gold"))]
    BrightGold,
    /// `Pantone 16-0948 TCX Harvest Gold` — `#B68A3A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0948 TCX Harvest Gold"))]
    HarvestGold,
    /// `Pantone 16-0950 TCX Narcissus` — `#C39449`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0950 TCX Narcissus"))]
    Narcissus,
    /// `Pantone 16-0952 TCX Nugget Gold` — `#C89720`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0952 TCX Nugget Gold"))]
    NuggetGold,
    /// `Pantone 16-0953 TCX Tawny Olive` — `#C4962C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0953 TCX Tawny Olive"))]
    TawnyOlive,
    /// `Pantone 16-0954 TCX Arrowwood` — `#BC8D1F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-0954 TCX Arrowwood"))]
    Arrowwood,
    /// `Pantone 16-1010 TCX Incense` — `#AF9A7E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1010 TCX Incense"))]
    Incense,
    /// `Pantone 16-1054 TCX Sunflower` — `#D39237`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1054 TCX Sunflower"))]
    Sunflower,
    /// `Pantone 16-1103 TCX Pure Cashmere` — `#ADA396`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1103 TCX Pure Cashmere"))]
    PureCashmere,
    /// `Pantone 16-1104 TCX Crockery` — `#A49887`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1104 TCX Crockery"))]
    Crockery,
    /// `Pantone 16-1105 TCX Plaza Taupe` — `#AEA393`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1105 TCX Plaza Taupe"))]
    PlazaTaupe,
    /// `Pantone 16-1106 TCX Tuffet` — `#A59788`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1106 TCX Tuffet"))]
    Tuffet,
    /// `Pantone 16-1107 TCX Aluminum` — `#9F9586`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1107 TCX Aluminum"))]
    Aluminum,
    /// `Pantone 16-1108 TCX Twill` — `#A79B82`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1108 TCX Twill"))]
    Twill,
    /// `Pantone 16-1109 TCX Greige` — `#928475`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1109 TCX Greige"))]
    Greige,
    /// `Pantone 16-1110 TCX Olive Gray` — `#A6997A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1110 TCX Olive Gray"))]
    OliveGray,
    /// `Pantone 16-1118 TCX Sponge` — `#A49775`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1118 TCX Sponge"))]
    Sponge,
    /// `Pantone 16-1120 TCX Starfish` — `#B09A77`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1120 TCX Starfish"))]
    Starfish,
    /// `Pantone 16-1126 TCX Antelope` — `#B19664`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1126 TCX Antelope"))]
    Antelope,
    /// `Pantone 16-1133 TCX Mustard Gold` — `#B08E51`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1133 TCX Mustard Gold"))]
    MustardGold,
    /// `Pantone 16-1139 TCX Amber Gold` — `#C19552`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1139 TCX Amber Gold"))]
    AmberGold,
    /// `Pantone 16-1140 TCX Yam` — `#D0893F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1140 TCX Yam"))]
    Yam,
    /// `Pantone 16-1142 TCX Golden Nugget` — `#DB9B59`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1142 TCX Golden Nugget"))]
    GoldenNugget,
    /// `Pantone 16-1143 TCX Honey Yellow` — `#CA9456`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1143 TCX Honey Yellow"))]
    HoneyYellow,
    /// `Pantone 16-1144 TCX Oak Buff` — `#CF9C63`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1144 TCX Oak Buff"))]
    OakBuff,
    /// `Pantone 16-1148 TCX Nugget` — `#CF8848`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1148 TCX Nugget"))]
    Nugget,
    /// `Pantone 16-1149 TCX Desert Sun` — `#C87629`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1149 TCX Desert Sun"))]
    DesertSun,
    /// `Pantone 16-1150 TCX Topaz` — `#D08344`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1150 TCX Topaz"))]
    Topaz,
    /// `Pantone 16-1164 TCX Orange Pepper` — `#DF7500`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1164 TCX Orange Pepper"))]
    OrangePepper,
    /// `Pantone 16-1210 TCX Light Taupe` — `#B19D8D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1210 TCX Light Taupe"))]
    LightTaupe,
    /// `Pantone 16-1212 TCX Nomad` — `#B49F89`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1212 TCX Nomad"))]
    Nomad,
    /// `Pantone 16-1219 TCX Tuscany` — `#BE9785`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1219 TCX Tuscany"))]
    Tuscany,
    /// `Pantone 16-1220 TCX Cafe Creme` — `#C79685`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1220 TCX Cafe Creme"))]
    CafeCreme,
    /// `Pantone 16-1221 TCX Roebuck` — `#B09080`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1221 TCX Roebuck"))]
    Roebuck,
    /// `Pantone 16-1235 TCX Sandstorm` — `#BD8B69`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1235 TCX Sandstorm"))]
    Sandstorm,
    /// `Pantone 16-1253 TCX Orange Ochre` — `#DC793A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1253 TCX Orange Ochre"))]
    OrangeOchre,
    /// `Pantone 16-1255 TCX Russet Orange` — `#E47127`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1255 TCX Russet Orange"))]
    RussetOrange,
    /// `Pantone 16-1257 TCX Sun Orange` — `#F48037`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1257 TCX Sun Orange"))]
    SunOrange,
    /// `Pantone 16-1260 TCX Harvest Pumpkin` — `#D56231`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1260 TCX Harvest Pumpkin"))]
    HarvestPumpkin,
    /// `Pantone 16-1305 TCX String` — `#AA9F96`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1305 TCX String"))]
    String,
    /// `Pantone 16-1310 TCX Natural` — `#AA907D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1310 TCX Natural"))]
    Natural,
    /// `Pantone 16-1315 TCX Cornstalk` — `#A9947A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1315 TCX Cornstalk"))]
    Cornstalk,
    /// `Pantone 16-1317 TCX Brush` — `#B99984`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1317 TCX Brush"))]
    Brush,
    /// `Pantone 16-1318 TCX Warm Taupe` — `#AF9483`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1318 TCX Warm Taupe"))]
    WarmTaupe,
    /// `Pantone 16-1320 TCX Nougat` — `#B69885`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1320 TCX Nougat"))]
    Nougat,
    /// `Pantone 16-1323 TCX Macaroon` — `#B38B71`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1323 TCX Macaroon"))]
    Macaroon,
    /// `Pantone 16-1324 TCX Lark` — `#B89B72`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1324 TCX Lark"))]
    Lark,
    /// `Pantone 16-1325 TCX Copper` — `#C47E5A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1325 TCX Copper"))]
    Copper,
    /// `Pantone 16-1326 TCX Prairie Sand` — `#B59A6A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1326 TCX Prairie Sand"))]
    PrairieSand,
    /// `Pantone 16-1327 TCX Toasted Nut` — `#C08768`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1327 TCX Toasted Nut"))]
    ToastedNut,
    /// `Pantone 16-1328 TCX Sandstone` — `#C48A69`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1328 TCX Sandstone"))]
    Sandstone,
    /// `Pantone 16-1329 TCX Coral Haze` — `#E38E84`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1329 TCX Coral Haze"))]
    CoralHaze,
    /// `Pantone 16-1330 TCX Muted Clay` — `#D29380`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1330 TCX Muted Clay"))]
    MutedClay,
    /// `Pantone 16-1331 TCX Toast` — `#CA9978`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1331 TCX Toast"))]
    Toast,
    /// `Pantone 16-1332 TCX Pheasant` — `#C68463`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1332 TCX Pheasant"))]
    Pheasant,
    /// `Pantone 16-1333 TCX Doe` — `#B98E68`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1333 TCX Doe"))]
    Doe,
    /// `Pantone 16-1334 TCX Tan` — `#B69574`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1334 TCX Tan"))]
    Tan,
    /// `Pantone 16-1336 TCX Biscuit` — `#B4835B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1336 TCX Biscuit"))]
    Biscuit,
    /// `Pantone 16-1337 TCX Coral Gold` — `#D27D56`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1337 TCX Coral Gold"))]
    CoralGold,
    /// `Pantone 16-1338 TCX Copper Tan` — `#DE8E65`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1338 TCX Copper Tan"))]
    CopperTan,
    /// `Pantone 16-1340 TCX Brandied Melon` — `#CE7B5B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1340 TCX Brandied Melon"))]
    BrandiedMelon,
    /// `Pantone 16-1341 TCX Butterum` — `#C68F65`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1341 TCX Butterum"))]
    Butterum,
    /// `Pantone 16-1342 TCX Buckskin` — `#D18E54`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1342 TCX Buckskin"))]
    Buckskin,
    /// `Pantone 16-1343 TCX Autumn Sunset` — `#F38554`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1343 TCX Autumn Sunset"))]
    AutumnSunset,
    /// `Pantone 16-1344 TCX Dusty Orange` — `#E27A53`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1344 TCX Dusty Orange"))]
    DustyOrange,
    /// `Pantone 16-1345 TCX Sun Baked` — `#D27F63`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1345 TCX Sun Baked"))]
    SunBaked,
    /// `Pantone 16-1346 TCX Golden Ochre` — `#C77943`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1346 TCX Golden Ochre"))]
    GoldenOchre,
    /// `Pantone 16-1347 TCX Peach Caramel` — `#C5733D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1347 TCX Peach Caramel"))]
    PeachCaramel,
    /// `Pantone 16-1348 TCX Tomato Cream` — `#C57644`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1348 TCX Tomato Cream"))]
    TomatoCream,
    /// `Pantone 16-1349 TCX Coral Rose` — `#F3774D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1349 TCX Coral Rose"))]
    CoralRose,
    /// `Pantone 16-1350 TCX Amberglow` — `#DC793E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1350 TCX Amberglow"))]
    Amberglow,
    /// `Pantone 16-1356 TCX Persimmon Orange` — `#F47327`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1356 TCX Persimmon Orange"))]
    PersimmonOrange,
    /// `Pantone 16-1357 TCX Bird Of Paradise` — `#FF8C55`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1357 TCX Bird Of Paradise"))]
    BirdOfParadise,
    /// `Pantone 16-1358 TCX Orange Tiger` — `#F96714`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1358 TCX Orange Tiger"))]
    OrangeTiger,
    /// `Pantone 16-1359 TCX Orange Peel` — `#FA7A35`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1359 TCX Orange Peel"))]
    OrangePeel,
    /// `Pantone 16-1360 TCX Nectarine` — `#FF8656`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1360 TCX Nectarine"))]
    Nectarine,
    /// `Pantone 16-1361 TCX Carrot` — `#FD6F3B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1361 TCX Carrot"))]
    Carrot,
    /// `Pantone 16-1362 TCX Vermillion Orange` — `#F9633B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1362 TCX Vermillion Orange"))]
    VermillionOrange,
    /// `Pantone 16-1363 TCX Puffins Bill` — `#E95C20`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1363 TCX Puffins Bill"))]
    PuffinsBill,
    /// `Pantone 16-1364 TCX Vibrant Orange` — `#FF7420`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1364 TCX Vibrant Orange"))]
    VibrantOrange,
    /// `Pantone 16-1406 TCX Atmosphere` — `#A89C94`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1406 TCX Atmosphere"))]
    Atmosphere,
    /// `Pantone 16-1407 TCX Cobblestone` — `#A89A8E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1407 TCX Cobblestone"))]
    Cobblestone,
    /// `Pantone 16-1412 TCX Stucco` — `#A58D7F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1412 TCX Stucco"))]
    Stucco,
    /// `Pantone 16-1414 TCX Chanterelle` — `#A28776`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1414 TCX Chanterelle"))]
    Chanterelle,
    /// `Pantone 16-1415 TCX Almondine` — `#A78C8B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1415 TCX Almondine"))]
    Almondine,
    /// `Pantone 16-1422 TCX Cork` — `#BA8671`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1422 TCX Cork"))]
    Cork,
    /// `Pantone 16-1429 TCX Sunburn` — `#B37256`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1429 TCX Sunburn"))]
    Sunburn,
    /// `Pantone 16-1431 TCX Canyon Clay` — `#CE8477`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1431 TCX Canyon Clay"))]
    CanyonClay,
    /// `Pantone 16-1432 TCX Almond` — `#A7754D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1432 TCX Almond"))]
    Almond,
    /// `Pantone 16-1434 TCX Coral Almond` — `#E29D94`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1434 TCX Coral Almond"))]
    CoralAlmond,
    /// `Pantone 16-1435 TCX Carnelian` — `#CE785D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1435 TCX Carnelian"))]
    Carnelian,
    /// `Pantone 16-1438 TCX Meerkat` — `#A46F44`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1438 TCX Meerkat"))]
    Meerkat,
    /// `Pantone 16-1439 TCX Caramel` — `#C37C54`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1439 TCX Caramel"))]
    Caramel,
    /// `Pantone 16-1440 TCX Langoustino` — `#CA6C56`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1440 TCX Langoustino"))]
    Langoustino,
    /// `Pantone 16-1441 TCX Arabesque` — `#D16F52`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1441 TCX Arabesque"))]
    Arabesque,
    /// `Pantone 16-1442 TCX Melon` — `#FE8863`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1442 TCX Melon"))]
    Melon,
    /// `Pantone 16-1443 TCX Apricot Buff` — `#CD7E4D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1443 TCX Apricot Buff"))]
    ApricotBuff,
    /// `Pantone 16-1448 TCX Burnt Orange` — `#C86733`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1448 TCX Burnt Orange"))]
    BurntOrange,
    /// `Pantone 16-1449 TCX Gold Flame` — `#B45422`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1449 TCX Gold Flame"))]
    GoldFlame,
    /// `Pantone 16-1450 TCX Flamingo` — `#DF7253`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1450 TCX Flamingo"))]
    Flamingo,
    /// `Pantone 16-1451 TCX Nasturtium` — `#FE6347`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1451 TCX Nasturtium"))]
    Nasturtium,
    /// `Pantone 16-1452 TCX Firecracker` — `#F36944`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1452 TCX Firecracker"))]
    Firecracker,
    /// `Pantone 16-1453 TCX Exotic Orange` — `#F96531`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1453 TCX Exotic Orange"))]
    ExoticOrange,
    /// `Pantone 16-1454 TCX Jaffa Orange` — `#D86D39`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1454 TCX Jaffa Orange"))]
    JaffaOrange,
    /// `Pantone 16-1459 TCX Mandarin Orange` — `#EC6A37`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1459 TCX Mandarin Orange"))]
    MandarinOrange,
    /// `Pantone 16-1460 TCX Dragon Fire` — `#FC642D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1460 TCX Dragon Fire"))]
    DragonFire,
    /// `Pantone 16-1462 TCX Golden Poppy` — `#F56733`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1462 TCX Golden Poppy"))]
    GoldenPoppy,
    /// `Pantone 16-1506 TCX Bark` — `#A99592`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1506 TCX Bark"))]
    Bark,
    /// `Pantone 16-1508 TCX Adobe Rose` — `#BA9F99`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1508 TCX Adobe Rose"))]
    AdobeRose,
    /// `Pantone 16-1509 TCX Shadow Gray` — `#BBA5A0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1509 TCX Shadow Gray"))]
    ShadowGray,
    /// `Pantone 16-1510 TCX Fawn` — `#AE9490`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1510 TCX Fawn"))]
    Fawn,
    /// `Pantone 16-1511 TCX Rose Tan` — `#D19C97`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1511 TCX Rose Tan"))]
    RoseTan,
    /// `Pantone 16-1516 TCX Cameo Brown` — `#C08A80`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1516 TCX Cameo Brown"))]
    CameoBrown,
    /// `Pantone 16-1518 TCX Rosette` — `#CE8E8B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1518 TCX Rosette"))]
    Rosette,
    /// `Pantone 16-1520 TCX Lobster Bisque` — `#DD9289`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1520 TCX Lobster Bisque"))]
    LobsterBisque,
    /// `Pantone 16-1522 TCX Rose Dawn` — `#C2877B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1522 TCX Rose Dawn"))]
    RoseDawn,
    /// `Pantone 16-1526 TCX Terra Cotta` — `#D38377`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1526 TCX Terra Cotta"))]
    TerraCotta,
    /// `Pantone 16-1529 TCX Burnt Coral` — `#E9897E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1529 TCX Burnt Coral"))]
    BurntCoral,
    /// `Pantone 16-1532 TCX Crabapple` — `#D77E70`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1532 TCX Crabapple"))]
    Crabapple,
    /// `Pantone 16-1539 TCX Coral` — `#ED7464`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1539 TCX Coral"))]
    Coral,
    /// `Pantone 16-1541 TCX Camellia` — `#F6745F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1541 TCX Camellia"))]
    Camellia,
    /// `Pantone 16-1542 TCX Fresh Salmon` — `#FF7F6A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1542 TCX Fresh Salmon"))]
    FreshSalmon,
    /// `Pantone 16-1543 TCX Fusion Coral` — `#FF8576`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1543 TCX Fusion Coral"))]
    FusionCoral,
    /// `Pantone 16-1544 TCX Persimmon` — `#F67866`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1544 TCX Persimmon"))]
    Persimmon,
    /// `Pantone 16-1545 TCX Coral Quartz` — `#F77464`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1545 TCX Coral Quartz"))]
    CoralQuartz,
    /// `Pantone 16-1546 TCX Living Coral` — `#FF6F61`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1546 TCX Living Coral"))]
    LivingCoral,
    /// `Pantone 16-1548 TCX Peach Echo` — `#F7786B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1548 TCX Peach Echo"))]
    PeachEcho,
    /// `Pantone 16-1606 TCX Purple Dove` — `#98878C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1606 TCX Purple Dove"))]
    PurpleDove,
    /// `Pantone 16-1610 TCX Brandied Apricot` — `#CA848A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1610 TCX Brandied Apricot"))]
    BrandiedApricot,
    /// `Pantone 16-1617 TCX Mauveglow` — `#D18489`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1617 TCX Mauveglow"))]
    Mauveglow,
    /// `Pantone 16-1620 TCX Tea Rose` — `#DC7178`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1620 TCX Tea Rose"))]
    TeaRose,
    /// `Pantone 16-1624 TCX Lantana` — `#DA7E7A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1624 TCX Lantana"))]
    Lantana,
    /// `Pantone 16-1626 TCX Peach Blossom` — `#DE8286`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1626 TCX Peach Blossom"))]
    PeachBlossom,
    /// `Pantone 16-1632 TCX Shell Pink` — `#F88180`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1632 TCX Shell Pink"))]
    ShellPink,
    /// `Pantone 16-1640 TCX Sugar Coral` — `#F56C73`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1640 TCX Sugar Coral"))]
    SugarCoral,
    /// `Pantone 16-1641 TCX Georgia Peach` — `#F97272`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1641 TCX Georgia Peach"))]
    GeorgiaPeach,
    /// `Pantone 16-1703 TCX Sphinx` — `#AB9895`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1703 TCX Sphinx"))]
    Sphinx,
    /// `Pantone 16-1707 TCX Deauville Mauve` — `#AF9294`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1707 TCX Deauville Mauve"))]
    DeauvilleMauve,
    /// `Pantone 16-1708 TCX Lilas` — `#B88995`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1708 TCX Lilas"))]
    Lilas,
    /// `Pantone 16-1710 TCX Foxglove` — `#B98391`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1710 TCX Foxglove"))]
    Foxglove,
    /// `Pantone 16-1712 TCX Polignac` — `#C28799`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1712 TCX Polignac"))]
    Polignac,
    /// `Pantone 16-1715 TCX Wild Rose` — `#CE8498`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1715 TCX Wild Rose"))]
    WildRose,
    /// `Pantone 16-1720 TCX Strawberry Ice` — `#E78B90`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1720 TCX Strawberry Ice"))]
    StrawberryIce,
    /// `Pantone 16-1723 TCX Confetti` — `#E6798E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1723 TCX Confetti"))]
    Confetti,
    /// `Pantone 16-1731 TCX Strawberry Pink` — `#F57F8E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1731 TCX Strawberry Pink"))]
    StrawberryPink,
    /// `Pantone 16-1735 TCX Pink Lemonade` — `#EE6D8A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1735 TCX Pink Lemonade"))]
    PinkLemonade,
    /// `Pantone 16-1806 TCX Woodrose` — `#AE8C8E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-1806 TCX Woodrose"))]
    Woodrose,
    /// `Pantone 16-2107 TCX Orchid Haze` — `#B0879B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-2107 TCX Orchid Haze"))]
    OrchidHaze,
    /// `Pantone 16-2111 TCX Mauve Orchid` — `#B58299`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-2111 TCX Mauve Orchid"))]
    MauveOrchid,
    /// `Pantone 16-2120 TCX Wild Orchid` — `#D979A2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-2120 TCX Wild Orchid"))]
    WildOrchid,
    /// `Pantone 16-2124 TCX Pink Carnation` — `#ED7A9E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-2124 TCX Pink Carnation"))]
    PinkCarnation,
    /// `Pantone 16-2126 TCX Azalea Pink` — `#E96A97`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-2126 TCX Azalea Pink"))]
    AzaleaPink,
    /// `Pantone 16-2215 TCX Cashmere Rose` — `#CE879F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-2215 TCX Cashmere Rose"))]
    CashmereRose,
    /// `Pantone 16-2614 TCX Moonlite Mauve` — `#D28FB0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-2614 TCX Moonlite Mauve"))]
    MoonliteMauve,
    /// `Pantone 16-3110 TCX Smoky Grape` — `#B88AAC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3110 TCX Smoky Grape"))]
    SmokyGrape,
    /// `Pantone 16-3115 TCX Crocus` — `#C67FAE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3115 TCX Crocus"))]
    Crocus,
    /// `Pantone 16-3116 TCX Opera Mauve` — `#CA80B1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3116 TCX Opera Mauve"))]
    OperaMauve,
    /// `Pantone 16-3118 TCX Cyclamen` — `#D687BA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3118 TCX Cyclamen"))]
    Cyclamen,
    /// `Pantone 16-3205 TCX Mauve Shadows` — `#B598A3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3205 TCX Mauve Shadows"))]
    MauveShadows,
    /// `Pantone 16-3304 TCX Sea Fog` — `#A5929D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3304 TCX Sea Fog"))]
    SeaFog,
    /// `Pantone 16-3307 TCX Lavender Mist` — `#AE90A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3307 TCX Lavender Mist"))]
    LavenderMist,
    /// `Pantone 16-3310 TCX Lavender Herb` — `#B18EAA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3310 TCX Lavender Herb"))]
    LavenderHerb,
    /// `Pantone 16-3320 TCX Violet` — `#C17FB5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3320 TCX Violet"))]
    Violet,
    /// `Pantone 16-3416 TCX Violet Tulle` — `#C193C0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3416 TCX Violet Tulle"))]
    VioletTulle,
    /// `Pantone 16-3520 TCX African Violet` — `#B085B7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3520 TCX African Violet"))]
    AfricanViolet,
    /// `Pantone 16-3521 TCX Lupine` — `#BE9CC1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3521 TCX Lupine"))]
    Lupine,
    /// `Pantone 16-3525 TCX Regal Orchid` — `#A98BAF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3525 TCX Regal Orchid"))]
    RegalOrchid,
    /// `Pantone 16-3617 TCX Sheer Lilac` — `#B793C0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3617 TCX Sheer Lilac"))]
    SheerLilac,
    /// `Pantone 16-3720 TCX Sand Verbena` — `#9F90C1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3720 TCX Sand Verbena"))]
    SandVerbena,
    /// `Pantone 16-3800 TCX Satellite` — `#9F8D89`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3800 TCX Satellite"))]
    Satellite,
    /// `Pantone 16-3801 TCX Opal Gray` — `#A49E9E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3801 TCX Opal Gray"))]
    OpalGray,
    /// `Pantone 16-3802 TCX Ash` — `#A09998`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3802 TCX Ash"))]
    Ash,
    /// `Pantone 16-3803 TCX Gull Gray` — `#A49CA0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3803 TCX Gull Gray"))]
    GullGray,
    /// `Pantone 16-3810 TCX Wisteria` — `#A198AF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3810 TCX Wisteria"))]
    Wisteria,
    /// `Pantone 16-3812 TCX Heirloom Lilac` — `#9D96B2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3812 TCX Heirloom Lilac"))]
    HeirloomLilac,
    /// `Pantone 16-3815 TCX Viola` — `#A692BA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3815 TCX Viola"))]
    Viola,
    /// `Pantone 16-3817 TCX Rhapsody` — `#9F86AA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3817 TCX Rhapsody"))]
    Rhapsody,
    /// `Pantone 16-3823 TCX Violet Tulip` — `#9E91C3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3823 TCX Violet Tulip"))]
    VioletTulip,
    /// `Pantone 16-3850 TCX Silver Sconce` — `#A19FA5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3850 TCX Silver Sconce"))]
    SilverSconce,
    /// `Pantone 16-3905 TCX Lilac Gray` — `#9896A4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3905 TCX Lilac Gray"))]
    LilacGray,
    /// `Pantone 16-3907 TCX Dapple Gray` — `#9C9BA7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3907 TCX Dapple Gray"))]
    DappleGray,
    /// `Pantone 16-3911 TCX Lavender Aura` — `#9F99AA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3911 TCX Lavender Aura"))]
    LavenderAura,
    /// `Pantone 16-3915 TCX Alloy` — `#98979A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3915 TCX Alloy"))]
    Alloy,
    /// `Pantone 16-3916 TCX Sleet` — `#92949B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3916 TCX Sleet"))]
    Sleet,
    /// `Pantone 16-3919 TCX Eventide` — `#959EB7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3919 TCX Eventide"))]
    Eventide,
    /// `Pantone 16-3920 TCX Lavender Lustre` — `#8C9CC1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3920 TCX Lavender Lustre"))]
    LavenderLustre,
    /// `Pantone 16-3921 TCX Blue Heron` — `#96A3C7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3921 TCX Blue Heron"))]
    BlueHeron,
    /// `Pantone 16-3922 TCX Brunnera Blue` — `#9BA9CA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3922 TCX Brunnera Blue"))]
    BrunneraBlue,
    /// `Pantone 16-3925 TCX Easter Egg` — `#919BC9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3925 TCX Easter Egg"))]
    EasterEgg,
    /// `Pantone 16-3929 TCX Grapemist` — `#8398CA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3929 TCX Grapemist"))]
    Grapemist,
    /// `Pantone 16-3930 TCX Thistle Down` — `#9499BB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3930 TCX Thistle Down"))]
    ThistleDown,
    /// `Pantone 16-3931 TCX Sweet Lavender` — `#9A9BC1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-3931 TCX Sweet Lavender"))]
    SweetLavender,
    /// `Pantone 16-4010 TCX Dusty Blue` — `#8C9DAD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4010 TCX Dusty Blue"))]
    DustyBlue,
    /// `Pantone 16-4013 TCX Ashley Blue` — `#8699AB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4013 TCX Ashley Blue"))]
    AshleyBlue,
    /// `Pantone 16-4019 TCX Forever Blue` — `#899BB8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4019 TCX Forever Blue"))]
    ForeverBlue,
    /// `Pantone 16-4020 TCX Della Robbia Blue` — `#7A9DCB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4020 TCX Della Robbia Blue"))]
    DellaRobbiaBlue,
    /// `Pantone 16-4021 TCX Allure` — `#7291B4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4021 TCX Allure"))]
    Allure,
    /// `Pantone 16-4030 TCX Hydrangea` — `#849BCC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4030 TCX Hydrangea"))]
    Hydrangea,
    /// `Pantone 16-4031 TCX Cornflower Blue` — `#7391C8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4031 TCX Cornflower Blue"))]
    CornflowerBlue,
    /// `Pantone 16-4032 TCX Provence` — `#658DC6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4032 TCX Provence"))]
    Provence,
    /// `Pantone 16-4033 TCX Granada Sky` — `#5D81BB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4033 TCX Granada Sky"))]
    GranadaSky,
    /// `Pantone 16-4109 TCX Arona` — `#879BA3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4109 TCX Arona"))]
    Arona,
    /// `Pantone 16-4114 TCX Stone Blue` — `#829CA5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4114 TCX Stone Blue"))]
    StoneBlue,
    /// `Pantone 16-4120 TCX Dusk Blue` — `#7BA0C0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4120 TCX Dusk Blue"))]
    DuskBlue,
    /// `Pantone 16-4127 TCX Heritage Blue` — `#5D96BC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4127 TCX Heritage Blue"))]
    HeritageBlue,
    /// `Pantone 16-4132 TCX Little Boy Blue` — `#6EA2D5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4132 TCX Little Boy Blue"))]
    LittleBoyBlue,
    /// `Pantone 16-4134 TCX Bonnie Blue` — `#539CCC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4134 TCX Bonnie Blue"))]
    BonnieBlue,
    /// `Pantone 16-4400 TCX Mourning Dove` — `#94908B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4400 TCX Mourning Dove"))]
    MourningDove,
    /// `Pantone 16-4402 TCX Drizzle` — `#A09F9C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4402 TCX Drizzle"))]
    Drizzle,
    /// `Pantone 16-4404 TCX Abyss` — `#8F9E9D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4404 TCX Abyss"))]
    Abyss,
    /// `Pantone 16-4408 TCX Slate` — `#8C9FA1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4408 TCX Slate"))]
    Slate,
    /// `Pantone 16-4411 TCX Tourmaline` — `#86A1A9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4411 TCX Tourmaline"))]
    Tourmaline,
    /// `Pantone 16-4414 TCX Cameo Blue` — `#769DA6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4414 TCX Cameo Blue"))]
    CameoBlue,
    /// `Pantone 16-4421 TCX Blue Mist` — `#5BACC3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4421 TCX Blue Mist"))]
    BlueMist,
    /// `Pantone 16-4427 TCX Horizon Blue` — `#289DBE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4427 TCX Horizon Blue"))]
    HorizonBlue,
    /// `Pantone 16-4519 TCX Delphinium Blue` — `#6198AE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4519 TCX Delphinium Blue"))]
    DelphiniumBlue,
    /// `Pantone 16-4525 TCX Maui Blue` — `#52A2B4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4525 TCX Maui Blue"))]
    MauiBlue,
    /// `Pantone 16-4529 TCX Cyan Blue` — `#14A3C7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4529 TCX Cyan Blue"))]
    CyanBlue,
    /// `Pantone 16-4530 TCX Aquarius` — `#3CADD4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4530 TCX Aquarius"))]
    Aquarius,
    /// `Pantone 16-4535 TCX Blue Atoll` — `#00B1D2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4535 TCX Blue Atoll"))]
    BlueAtoll,
    /// `Pantone 16-4610 TCX Stillwater` — `#70A4B0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4610 TCX Stillwater"))]
    Stillwater,
    /// `Pantone 16-4612 TCX Reef Waters` — `#6F9FA9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4612 TCX Reef Waters"))]
    ReefWaters,
    /// `Pantone 16-4702 TCX Limestone` — `#989A98`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4702 TCX Limestone"))]
    Limestone,
    /// `Pantone 16-4703 TCX Ghost Gray` — `#9C9B98`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4703 TCX Ghost Gray"))]
    GhostGray,
    /// `Pantone 16-4706 TCX Silver Blue` — `#8A9A9A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4706 TCX Silver Blue"))]
    SilverBlue,
    /// `Pantone 16-4712 TCX Mineral Blue` — `#6D9192`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4712 TCX Mineral Blue"))]
    MineralBlue,
    /// `Pantone 16-4719 TCX Porcelain` — `#5D9CA4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4719 TCX Porcelain"))]
    Porcelain,
    /// `Pantone 16-4725 TCX Scuba Blue` — `#00ABC0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4725 TCX Scuba Blue"))]
    ScubaBlue,
    /// `Pantone 16-4728 TCX Peacock Blue` — `#00A0B0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4728 TCX Peacock Blue"))]
    PeacockBlue,
    /// `Pantone 16-4834 TCX Bluebird` — `#009DAE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-4834 TCX Bluebird"))]
    Bluebird,
    /// `Pantone 16-5101 TCX Wet Weather` — `#929090`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5101 TCX Wet Weather"))]
    WetWeather,
    /// `Pantone 16-5106 TCX Blue Surf` — `#90A8A4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5106 TCX Blue Surf"))]
    BlueSurf,
    /// `Pantone 16-5109 TCX Wasabi` — `#73A89E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5109 TCX Wasabi"))]
    Wasabi,
    /// `Pantone 16-5112 TCX Canton` — `#6DA29E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5112 TCX Canton"))]
    Canton,
    /// `Pantone 16-5114 TCX Dusty Turquoise` — `#649B9E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5114 TCX Dusty Turquoise"))]
    DustyTurquoise,
    /// `Pantone 16-5119 TCX Sea Blue` — `#549F98`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5119 TCX Sea Blue"))]
    SeaBlue,
    /// `Pantone 16-5121 TCX Meadowbrook` — `#60A0A3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5121 TCX Meadowbrook"))]
    Meadowbrook,
    /// `Pantone 16-5123 TCX Baltic` — `#279D9F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5123 TCX Baltic"))]
    Baltic,
    /// `Pantone 16-5127 TCX Ceramic` — `#00AAA9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5127 TCX Ceramic"))]
    Ceramic,
    /// `Pantone 16-5304 TCX Jadeite` — `#95A69F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5304 TCX Jadeite"))]
    Jadeite,
    /// `Pantone 16-5412 TCX Agate Green` — `#599F99`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5412 TCX Agate Green"))]
    AgateGreen,
    /// `Pantone 16-5418 TCX Lagoon` — `#4D9E9A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5418 TCX Lagoon"))]
    Lagoon,
    /// `Pantone 16-5421 TCX Sea Green` — `#149C88`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5421 TCX Sea Green"))]
    SeaGreen,
    /// `Pantone 16-5422 TCX Bright Aqua` — `#30A299`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5422 TCX Bright Aqua"))]
    BrightAqua,
    /// `Pantone 16-5425 TCX Pool Green` — `#00AF9D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5425 TCX Pool Green"))]
    PoolGreen,
    /// `Pantone 16-5427 TCX Billiard` — `#00AA92`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5427 TCX Billiard"))]
    Billiard,
    /// `Pantone 16-5431 TCX Peacock Green` — `#00A78B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5431 TCX Peacock Green"))]
    PeacockGreen,
    /// `Pantone 16-5515 TCX Beryl Green` — `#619187`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5515 TCX Beryl Green"))]
    BerylGreen,
    /// `Pantone 16-5533 TCX Arcadia` — `#00A28A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5533 TCX Arcadia"))]
    Arcadia,
    /// `Pantone 16-5721 TCX Marine Green` — `#40A48E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5721 TCX Marine Green"))]
    MarineGreen,
    /// `Pantone 16-5803 TCX Flint Gray` — `#A09C98`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5803 TCX Flint Gray"))]
    FlintGray,
    /// `Pantone 16-5804 TCX Slate Gray` — `#8A9691`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5804 TCX Slate Gray"))]
    SlateGray,
    /// `Pantone 16-5806 TCX Green Milieu` — `#8A9992`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5806 TCX Green Milieu"))]
    GreenMilieu,
    /// `Pantone 16-5807 TCX Lily Pad` — `#818F84`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5807 TCX Lily Pad"))]
    LilyPad,
    /// `Pantone 16-5808 TCX Iceberg Green` — `#8C9C92`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5808 TCX Iceberg Green"))]
    IcebergGreen,
    /// `Pantone 16-5810 TCX Green Bay` — `#7E9285`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5810 TCX Green Bay"))]
    GreenBay,
    /// `Pantone 16-5815 TCX Feldspar` — `#729B8B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5815 TCX Feldspar"))]
    Feldspar,
    /// `Pantone 16-5820 TCX Green Spruce` — `#589F7E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5820 TCX Green Spruce"))]
    GreenSpruce,
    /// `Pantone 16-5825 TCX Gumdrop Green` — `#2EA785`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5825 TCX Gumdrop Green"))]
    GumdropGreen,
    /// `Pantone 16-5904 TCX Wrought Iron` — `#999E98`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5904 TCX Wrought Iron"))]
    WroughtIron,
    /// `Pantone 16-5907 TCX Granite Green` — `#86A293`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5907 TCX Granite Green"))]
    GraniteGreen,
    /// `Pantone 16-5917 TCX Malachite Green` — `#709A89`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5917 TCX Malachite Green"))]
    MalachiteGreen,
    /// `Pantone 16-5919 TCX Creme De Menthe` — `#70A38D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5919 TCX Creme De Menthe"))]
    CremeDeMenthe,
    /// `Pantone 16-5924 TCX Winter Green` — `#4F9E81`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5924 TCX Winter Green"))]
    WinterGreen,
    /// `Pantone 16-5930 TCX Ming Green` — `#3AA278`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5930 TCX Ming Green"))]
    MingGreen,
    /// `Pantone 16-5932 TCX Holly Green` — `#0F9D76`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5932 TCX Holly Green"))]
    HollyGreen,
    /// `Pantone 16-5938 TCX Mint` — `#00A170`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5938 TCX Mint"))]
    Mint,
    /// `Pantone 16-5942 TCX Blarney` — `#00A776`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-5942 TCX Blarney"))]
    Blarney,
    /// `Pantone 16-6008 TCX Seagrass` — `#959889`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-6008 TCX Seagrass"))]
    Seagrass,
    /// `Pantone 16-6030 TCX Katydid` — `#66BC91`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-6030 TCX Katydid"))]
    Katydid,
    /// `Pantone 16-6116 TCX Shale Green` — `#739072`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-6116 TCX Shale Green"))]
    ShaleGreen,
    /// `Pantone 16-6127 TCX Greenbriar` — `#4B9B69`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-6127 TCX Greenbriar"))]
    Greenbriar,
    /// `Pantone 16-6138 TCX Kelly Green` — `#339C5E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-6138 TCX Kelly Green"))]
    KellyGreen,
    /// `Pantone 16-6216 TCX Basil` — `#879F84`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-6216 TCX Basil"))]
    Basil,
    /// `Pantone 16-6240 TCX Island Green` — `#2BAE66`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-6240 TCX Island Green"))]
    IslandGreen,
    /// `Pantone 16-6318 TCX Mineral Green` — `#7A9B78`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-6318 TCX Mineral Green"))]
    MineralGreen,
    /// `Pantone 16-6324 TCX Jadesheen` — `#77A276`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-6324 TCX Jadesheen"))]
    Jadesheen,
    /// `Pantone 16-6329 TCX Peppermint` — `#699E6D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-6329 TCX Peppermint"))]
    Peppermint,
    /// `Pantone 16-6339 TCX Vibrant Green` — `#55A860`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-6339 TCX Vibrant Green"))]
    VibrantGreen,
    /// `Pantone 16-6340 TCX Classic Green` — `#39A845`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-6340 TCX Classic Green"))]
    ClassicGreen,
    /// `Pantone 16-6444 TCX Poison Green` — `#4DB560`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 16-6444 TCX Poison Green"))]
    PoisonGreen,
    /// `Pantone 17-0000 TCX Frost Gray` — `#848283`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0000 TCX Frost Gray"))]
    FrostGray,
    /// `Pantone 17-0115 TCX Oil Green` — `#80856D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0115 TCX Oil Green"))]
    OilGreen,
    /// `Pantone 17-0119 TCX Turf Green` — `#6F8C69`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0119 TCX Turf Green"))]
    TurfGreen,
    /// `Pantone 17-0123 TCX Stone Green` — `#658E67`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0123 TCX Stone Green"))]
    StoneGreen,
    /// `Pantone 17-0133 TCX Fluorite Green` — `#699158`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0133 TCX Fluorite Green"))]
    FluoriteGreen,
    /// `Pantone 17-0145 TCX Online Lime` — `#44883C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0145 TCX Online Lime"))]
    OnlineLime,
    /// `Pantone 17-0205 TCX Elephant Skin` — `#8F8982`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0205 TCX Elephant Skin"))]
    ElephantSkin,
    /// `Pantone 17-0207 TCX Rock Ridge` — `#918C86`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0207 TCX Rock Ridge"))]
    RockRidge,
    /// `Pantone 17-0210 TCX Loden Frost` — `#788F74`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0210 TCX Loden Frost"))]
    LodenFrost,
    /// `Pantone 17-0215 TCX Aspen Green` — `#7E9B76`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0215 TCX Aspen Green"))]
    AspenGreen,
    /// `Pantone 17-0220 TCX Watercress` — `#748C69`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0220 TCX Watercress"))]
    Watercress,
    /// `Pantone 17-0230 TCX Forest Green` — `#6B8D53`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0230 TCX Forest Green"))]
    ForestGreen,
    /// `Pantone 17-0235 TCX Piquant Green` — `#769358`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0235 TCX Piquant Green"))]
    PiquantGreen,
    /// `Pantone 17-0324 TCX Epsom` — `#849161`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0324 TCX Epsom"))]
    Epsom,
    /// `Pantone 17-0330 TCX Turtle Green` — `#81894E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0330 TCX Turtle Green"))]
    TurtleGreen,
    /// `Pantone 17-0336 TCX Peridot` — `#819548`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0336 TCX Peridot"))]
    Peridot,
    /// `Pantone 17-0510 TCX Silver Sage` — `#938B78`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0510 TCX Silver Sage"))]
    SilverSage,
    /// `Pantone 17-0517 TCX Dusky Green` — `#746C57`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0517 TCX Dusky Green"))]
    DuskyGreen,
    /// `Pantone 17-0525 TCX Mosstone` — `#858961`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0525 TCX Mosstone"))]
    Mosstone,
    /// `Pantone 17-0530 TCX Guacamole` — `#797B3A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0530 TCX Guacamole"))]
    Guacamole,
    /// `Pantone 17-0535 TCX Green Olive` — `#8D8B55`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0535 TCX Green Olive"))]
    GreenOlive,
    /// `Pantone 17-0610 TCX Laurel Oak` — `#918C7E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0610 TCX Laurel Oak"))]
    LaurelOak,
    /// `Pantone 17-0613 TCX Vetiver` — `#807D6F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0613 TCX Vetiver"))]
    Vetiver,
    /// `Pantone 17-0618 TCX Mermaid` — `#817A65`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0618 TCX Mermaid"))]
    Mermaid,
    /// `Pantone 17-0620 TCX Aloe` — `#817A60`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0620 TCX Aloe"))]
    Aloe,
    /// `Pantone 17-0625 TCX Boa` — `#8E855F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0625 TCX Boa"))]
    Boa,
    /// `Pantone 17-0627 TCX Dried Herb` — `#847A59`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0627 TCX Dried Herb"))]
    DriedHerb,
    /// `Pantone 17-0630 TCX Tree House` — `#988C75`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0630 TCX Tree House"))]
    TreeHouse,
    /// `Pantone 17-0636 TCX Green Moss` — `#857946`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0636 TCX Green Moss"))]
    GreenMoss,
    /// `Pantone 17-0808 TCX Taupe Gray` — `#8E7C71`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0808 TCX Taupe Gray"))]
    TaupeGray,
    /// `Pantone 17-0836 TCX Ecru Olive` — `#927B3C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0836 TCX Ecru Olive"))]
    EcruOlive,
    /// `Pantone 17-0839 TCX Golden Palm` — `#AA8805`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0839 TCX Golden Palm"))]
    GoldenPalm,
    /// `Pantone 17-0840 TCX Amber Green` — `#9A803A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0840 TCX Amber Green"))]
    AmberGreen,
    /// `Pantone 17-0843 TCX Bronze Mist` — `#9C7E41`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0843 TCX Bronze Mist"))]
    BronzeMist,
    /// `Pantone 17-0909 TCX Fossil` — `#806F63`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0909 TCX Fossil"))]
    Fossil,
    /// `Pantone 17-0929 TCX Fennel Seed` — `#998456`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0929 TCX Fennel Seed"))]
    FennelSeed,
    /// `Pantone 17-0935 TCX Dull Gold` — `#8A6F48`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0935 TCX Dull Gold"))]
    DullGold,
    /// `Pantone 17-0942 TCX Medal Bronze` — `#977547`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0942 TCX Medal Bronze"))]
    MedalBronze,
    /// `Pantone 17-0949 TCX Chai Tea` — `#B1832F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-0949 TCX Chai Tea"))]
    ChaiTea,
    /// `Pantone 17-1009 TCX Dune` — `#998978`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1009 TCX Dune"))]
    Dune,
    /// `Pantone 17-1019 TCX Elmwood` — `#8C7C61`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1019 TCX Elmwood"))]
    Elmwood,
    /// `Pantone 17-1022 TCX Kelp` — `#988467`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1022 TCX Kelp"))]
    Kelp,
    /// `Pantone 17-1028 TCX Antique Bronze` — `#907954`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1028 TCX Antique Bronze"))]
    AntiqueBronze,
    /// `Pantone 17-1036 TCX Bistre` — `#98754A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1036 TCX Bistre"))]
    Bistre,
    /// `Pantone 17-1038 TCX Tigers Eye` — `#977C61`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1038 TCX Tigers Eye"))]
    TigersEye,
    /// `Pantone 17-1040 TCX Spruce Yellow` — `#BE8A4A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1040 TCX Spruce Yellow"))]
    SpruceYellow,
    /// `Pantone 17-1044 TCX Chipmunk` — `#976F4C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1044 TCX Chipmunk"))]
    Chipmunk,
    /// `Pantone 17-1045 TCX Apple Cinnamon` — `#B0885A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1045 TCX Apple Cinnamon"))]
    AppleCinnamon,
    /// `Pantone 17-1046 TCX Golden Oak` — `#BE752D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1046 TCX Golden Oak"))]
    GoldenOak,
    /// `Pantone 17-1047 TCX Honey Mustard` — `#B68F52`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1047 TCX Honey Mustard"))]
    HoneyMustard,
    /// `Pantone 17-1048 TCX Inca Gold` — `#BB7A2C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1048 TCX Inca Gold"))]
    IncaGold,
    /// `Pantone 17-1052 TCX Roasted Pecan` — `#93592B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1052 TCX Roasted Pecan"))]
    RoastedPecan,
    /// `Pantone 17-1105 TCX Roasted Cashew` — `#918579`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1105 TCX Roasted Cashew"))]
    RoastedCashew,
    /// `Pantone 17-1107 TCX Seneca Rock` — `#9A927F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1107 TCX Seneca Rock"))]
    SenecaRock,
    /// `Pantone 17-1108 TCX Winter Twig` — `#948A7A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1108 TCX Winter Twig"))]
    WinterTwig,
    /// `Pantone 17-1109 TCX Chinchilla` — `#9C8E7B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1109 TCX Chinchilla"))]
    Chinchilla,
    /// `Pantone 17-1113 TCX Coriander` — `#938772`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1113 TCX Coriander"))]
    Coriander,
    /// `Pantone 17-1115 TCX Petrified Oak` — `#8D7960`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1115 TCX Petrified Oak"))]
    PetrifiedOak,
    /// `Pantone 17-1118 TCX Lead Gray` — `#8A7963`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1118 TCX Lead Gray"))]
    LeadGray,
    /// `Pantone 17-1125 TCX Dijon` — `#97754C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1125 TCX Dijon"))]
    Dijon,
    /// `Pantone 17-1128 TCX Bone Brown` — `#9D7446`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1128 TCX Bone Brown"))]
    BoneBrown,
    /// `Pantone 17-1129 TCX Wood Thrush` — `#A47D43`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1129 TCX Wood Thrush"))]
    WoodThrush,
    /// `Pantone 17-1134 TCX Brown Sugar` — `#A17249`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1134 TCX Brown Sugar"))]
    BrownSugar,
    /// `Pantone 17-1137 TCX Cashew` — `#A47149`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1137 TCX Cashew"))]
    Cashew,
    /// `Pantone 17-1140 TCX Marmalade` — `#C16512`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1140 TCX Marmalade"))]
    Marmalade,
    /// `Pantone 17-1142 TCX Argan Oil` — `#8B593E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1142 TCX Argan Oil"))]
    ArganOil,
    /// `Pantone 17-1143 TCX Hazel` — `#AE7250`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1143 TCX Hazel"))]
    Hazel,
    /// `Pantone 17-1145 TCX Autumn Maple` — `#C46215`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1145 TCX Autumn Maple"))]
    AutumnMaple,
    /// `Pantone 17-1147 TCX Amber Brown` — `#A66646`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1147 TCX Amber Brown"))]
    AmberBrown,
    /// `Pantone 17-1210 TCX Moon Rock` — `#958B84`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1210 TCX Moon Rock"))]
    MoonRock,
    /// `Pantone 17-1212 TCX Fungi` — `#8F8177`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1212 TCX Fungi"))]
    Fungi,
    /// `Pantone 17-1223 TCX Praline` — `#AD8B75`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1223 TCX Praline"))]
    Praline,
    /// `Pantone 17-1224 TCX Camel` — `#B0846A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1224 TCX Camel"))]
    Camel,
    /// `Pantone 17-1225 TCX Tawny Birch` — `#AE856C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1225 TCX Tawny Birch"))]
    TawnyBirch,
    /// `Pantone 17-1226 TCX Tawny Brown` — `#AB856F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1226 TCX Tawny Brown"))]
    TawnyBrown,
    /// `Pantone 17-1227 TCX Cafe Au Lait` — `#AE8774`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1227 TCX Cafe Au Lait"))]
    CafeAuLait,
    /// `Pantone 17-1230 TCX Mocha Mousse` — `#A47864`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1230 TCX Mocha Mousse"))]
    MochaMousse,
    /// `Pantone 17-1310 TCX Timber Wolf` — `#8D8070`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1310 TCX Timber Wolf"))]
    TimberWolf,
    /// `Pantone 17-1311 TCX Desert Taupe` — `#8D7E71`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1311 TCX Desert Taupe"))]
    DesertTaupe,
    /// `Pantone 17-1312 TCX Silver Mink` — `#9F8D7C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1312 TCX Silver Mink"))]
    SilverMink,
    /// `Pantone 17-1314 TCX Sepia Tint` — `#897560`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1314 TCX Sepia Tint"))]
    SepiaTint,
    /// `Pantone 17-1316 TCX Portabella` — `#937B6A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1316 TCX Portabella"))]
    Portabella,
    /// `Pantone 17-1319 TCX Amphora` — `#9F8672`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1319 TCX Amphora"))]
    Amphora,
    /// `Pantone 17-1320 TCX Tannin` — `#A68A6D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1320 TCX Tannin"))]
    Tannin,
    /// `Pantone 17-1321 TCX Woodsmoke` — `#947764`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1321 TCX Woodsmoke"))]
    Woodsmoke,
    /// `Pantone 17-1322 TCX Burro` — `#947764`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1322 TCX Burro"))]
    Burro,
    /// `Pantone 17-1327 TCX Tobacco Brown` — `#9A7352`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1327 TCX Tobacco Brown"))]
    TobaccoBrown,
    /// `Pantone 17-1328 TCX Indian Tan` — `#AD8567`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1328 TCX Indian Tan"))]
    IndianTan,
    /// `Pantone 17-1330 TCX Lion` — `#A0714F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1330 TCX Lion"))]
    Lion,
    /// `Pantone 17-1336 TCX Bran` — `#A66E4A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1336 TCX Bran"))]
    Bran,
    /// `Pantone 17-1340 TCX Adobe` — `#A3623B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1340 TCX Adobe"))]
    Adobe,
    /// `Pantone 17-1341 TCX Tawny Orange` — `#D37F6F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1341 TCX Tawny Orange"))]
    TawnyOrange,
    /// `Pantone 17-1342 TCX Autumnal` — `#A15325`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1342 TCX Autumnal"))]
    Autumnal,
    /// `Pantone 17-1345 TCX Spice Route` — `#B95B3F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1345 TCX Spice Route"))]
    SpiceRoute,
    /// `Pantone 17-1347 TCX Autumn Leaf` — `#B56A4C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1347 TCX Autumn Leaf"))]
    AutumnLeaf,
    /// `Pantone 17-1349 TCX Exuberance` — `#E86800`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1349 TCX Exuberance"))]
    Exuberance,
    /// `Pantone 17-1350 TCX Orange Popsicle` — `#FF7913`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1350 TCX Orange Popsicle"))]
    OrangePopsicle,
    /// `Pantone 17-1353 TCX Apricot Orange` — `#C86B3C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1353 TCX Apricot Orange"))]
    ApricotOrange,
    /// `Pantone 17-1360 TCX Celosia Orange` — `#E8703A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1360 TCX Celosia Orange"))]
    CelosiaOrange,
    /// `Pantone 17-1361 TCX Scarlet Ibis` — `#F45520`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1361 TCX Scarlet Ibis"))]
    ScarletIbis,
    /// `Pantone 17-1410 TCX Pine Bark` — `#827064`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1410 TCX Pine Bark"))]
    PineBark,
    /// `Pantone 17-1417 TCX Beaver Fur` — `#997867`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1417 TCX Beaver Fur"))]
    BeaverFur,
    /// `Pantone 17-1418 TCX Ginger Snap` — `#977D70`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1418 TCX Ginger Snap"))]
    GingerSnap,
    /// `Pantone 17-1422 TCX Raw Umber` — `#92705F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1422 TCX Raw Umber"))]
    RawUmber,
    /// `Pantone 17-1424 TCX Brick Dust` — `#B07069`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1424 TCX Brick Dust"))]
    BrickDust,
    /// `Pantone 17-1430 TCX Pecan Brown` — `#A36E51`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1430 TCX Pecan Brown"))]
    PecanBrown,
    /// `Pantone 17-1436 TCX Raw Sienna` — `#B9714F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1436 TCX Raw Sienna"))]
    RawSienna,
    /// `Pantone 17-1444 TCX Ginger` — `#C96551`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1444 TCX Ginger"))]
    Ginger,
    /// `Pantone 17-1446 TCX Mango` — `#B75E41`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1446 TCX Mango"))]
    Mango,
    /// `Pantone 17-1449 TCX Pureed Pumpkin` — `#C34121`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1449 TCX Pureed Pumpkin"))]
    PureedPumpkin,
    /// `Pantone 17-1450 TCX Summer Fig` — `#BE4B3B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1450 TCX Summer Fig"))]
    SummerFig,
    /// `Pantone 17-1452 TCX Koi` — `#D15837`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1452 TCX Koi"))]
    Koi,
    /// `Pantone 17-1456 TCX Tigerlily` — `#E2583E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1456 TCX Tigerlily"))]
    Tigerlily,
    /// `Pantone 17-1461 TCX Orangeade` — `#E2552C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1461 TCX Orangeade"))]
    Orangeade,
    /// `Pantone 17-1462 TCX Flame` — `#F2552C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1462 TCX Flame"))]
    Flame,
    /// `Pantone 17-1463 TCX Tangerine Tango` — `#DD4124`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1463 TCX Tangerine Tango"))]
    TangerineTango,
    /// `Pantone 17-1464 TCX Red Orange` — `#F05627`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1464 TCX Red Orange"))]
    RedOrange,
    /// `Pantone 17-1500 TCX Steeple Gray` — `#827E7C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1500 TCX Steeple Gray"))]
    SteepleGray,
    /// `Pantone 17-1501 TCX Wild Dove` — `#8B8C89`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1501 TCX Wild Dove"))]
    WildDove,
    /// `Pantone 17-1502 TCX Cloudburst` — `#837F7F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1502 TCX Cloudburst"))]
    Cloudburst,
    /// `Pantone 17-1503 TCX Storm Front` — `#787376`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1503 TCX Storm Front"))]
    StormFront,
    /// `Pantone 17-1505 TCX Quail` — `#98868C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1505 TCX Quail"))]
    Quail,
    /// `Pantone 17-1506 TCX Cinder` — `#8A7E78`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1506 TCX Cinder"))]
    Cinder,
    /// `Pantone 17-1510 TCX Antler` — `#957A76`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1510 TCX Antler"))]
    Antler,
    /// `Pantone 17-1511 TCX Wistful Mauve` — `#946C74`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1511 TCX Wistful Mauve"))]
    WistfulMauve,
    /// `Pantone 17-1512 TCX Nostalgia Rose` — `#A4777E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1512 TCX Nostalgia Rose"))]
    NostalgiaRose,
    /// `Pantone 17-1514 TCX Ash Rose` — `#B5817D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1514 TCX Ash Rose"))]
    AshRose,
    /// `Pantone 17-1516 TCX Burlwood` — `#9B716B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1516 TCX Burlwood"))]
    Burlwood,
    /// `Pantone 17-1518 TCX Old Rose` — `#B47B77`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1518 TCX Old Rose"))]
    OldRose,
    /// `Pantone 17-1520 TCX Canyon Rose` — `#AF6C67`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1520 TCX Canyon Rose"))]
    CanyonRose,
    /// `Pantone 17-1522 TCX Mauvewood` — `#A75D67`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1522 TCX Mauvewood"))]
    Mauvewood,
    /// `Pantone 17-1524 TCX Desert Sand` — `#BD7B74`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1524 TCX Desert Sand"))]
    DesertSand,
    /// `Pantone 17-1525 TCX Cedar Wood` — `#A1655B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1525 TCX Cedar Wood"))]
    CedarWood,
    /// `Pantone 17-1532 TCX Aragon` — `#B06455`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1532 TCX Aragon"))]
    Aragon,
    /// `Pantone 17-1537 TCX Mineral Red` — `#B35457`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1537 TCX Mineral Red"))]
    MineralRed,
    /// `Pantone 17-1540 TCX Apricot Brandy` — `#C26A5A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1540 TCX Apricot Brandy"))]
    ApricotBrandy,
    /// `Pantone 17-1544 TCX Burnt Sienna` — `#C65D52`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1544 TCX Burnt Sienna"))]
    BurntSienna,
    /// `Pantone 17-1545 TCX Cranberry` — `#BB4A4D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1545 TCX Cranberry"))]
    Cranberry,
    /// `Pantone 17-1547 TCX Emberglow` — `#EA6759`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1547 TCX Emberglow"))]
    Emberglow,
    /// `Pantone 17-1553 TCX Paprika` — `#CE4D42`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1553 TCX Paprika"))]
    Paprika,
    /// `Pantone 17-1558 TCX Grenadine` — `#DF3F32`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1558 TCX Grenadine"))]
    Grenadine,
    /// `Pantone 17-1562 TCX Mandarin Red` — `#E74A33`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1562 TCX Mandarin Red"))]
    MandarinRed,
    /// `Pantone 17-1563 TCX Cherry Tomato` — `#EB3C27`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1563 TCX Cherry Tomato"))]
    CherryTomato,
    /// `Pantone 17-1564 TCX Fiesta` — `#DD4132`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1564 TCX Fiesta"))]
    Fiesta,
    /// `Pantone 17-1605 TCX Elderberry` — `#9D848E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1605 TCX Elderberry"))]
    Elderberry,
    /// `Pantone 17-1608 TCX Heather Rose` — `#AD6D7F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1608 TCX Heather Rose"))]
    HeatherRose,
    /// `Pantone 17-1609 TCX Mesa Rose` — `#A66E7A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1609 TCX Mesa Rose"))]
    MesaRose,
    /// `Pantone 17-1610 TCX Dusky Orchid` — `#9A7182`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1610 TCX Dusky Orchid"))]
    DuskyOrchid,
    /// `Pantone 17-1612 TCX Mellow Mauve` — `#996378`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1612 TCX Mellow Mauve"))]
    MellowMauve,
    /// `Pantone 17-1614 TCX Deco Rose` — `#985F68`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1614 TCX Deco Rose"))]
    DecoRose,
    /// `Pantone 17-1623 TCX Rose Wine` — `#A4596D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1623 TCX Rose Wine"))]
    RoseWine,
    /// `Pantone 17-1633 TCX Holly Berry` — `#B44E5D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1633 TCX Holly Berry"))]
    HollyBerry,
    /// `Pantone 17-1635 TCX Rose Of Sharon` — `#DC5B62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1635 TCX Rose Of Sharon"))]
    RoseOfSharon,
    /// `Pantone 17-1641 TCX Chrysanthemum` — `#BE454F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1641 TCX Chrysanthemum"))]
    Chrysanthemum,
    /// `Pantone 17-1643 TCX Porcelain Rose` — `#EA6B6A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1643 TCX Porcelain Rose"))]
    PorcelainRose,
    /// `Pantone 17-1644 TCX Spiced Coral` — `#D75C5D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1644 TCX Spiced Coral"))]
    SpicedCoral,
    /// `Pantone 17-1647 TCX Dubarry` — `#F25F66`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1647 TCX Dubarry"))]
    Dubarry,
    /// `Pantone 17-1654 TCX Poinsettia` — `#CB3441`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1654 TCX Poinsettia"))]
    Poinsettia,
    /// `Pantone 17-1656 TCX Hot Coral` — `#F35B53`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1656 TCX Hot Coral"))]
    HotCoral,
    /// `Pantone 17-1663 TCX Bittersweet` — `#D93744`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1663 TCX Bittersweet"))]
    Bittersweet,
    /// `Pantone 17-1664 TCX Poppy Red` — `#DC343B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1664 TCX Poppy Red"))]
    PoppyRed,
    /// `Pantone 17-1708 TCX Moonscape` — `#725F69`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1708 TCX Moonscape"))]
    Moonscape,
    /// `Pantone 17-1710 TCX Bordeaux` — `#96637B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1710 TCX Bordeaux"))]
    Bordeaux,
    /// `Pantone 17-1718 TCX Dusty Rose` — `#BA797D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1718 TCX Dusty Rose"))]
    DustyRose,
    /// `Pantone 17-1723 TCX Malaga` — `#9F5069`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1723 TCX Malaga"))]
    Malaga,
    /// `Pantone 17-1736 TCX Sunkist Coral` — `#EA6676`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1736 TCX Sunkist Coral"))]
    SunkistCoral,
    /// `Pantone 17-1740 TCX Claret Red` — `#C84C61`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1740 TCX Claret Red"))]
    ClaretRed,
    /// `Pantone 17-1744 TCX Calypso Coral` — `#EE5C6C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1744 TCX Calypso Coral"))]
    CalypsoCoral,
    /// `Pantone 17-1753 TCX Geranium` — `#DA3D58`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1753 TCX Geranium"))]
    Geranium,
    /// `Pantone 17-1755 TCX Paradise Pink` — `#E4445E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1755 TCX Paradise Pink"))]
    ParadisePink,
    /// `Pantone 17-1818 TCX Red Violet` — `#A35776`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1818 TCX Red Violet"))]
    RedViolet,
    /// `Pantone 17-1831 TCX Carmine` — `#BC4869`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1831 TCX Carmine"))]
    Carmine,
    /// `Pantone 17-1842 TCX Azalea` — `#D42E5B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1842 TCX Azalea"))]
    Azalea,
    /// `Pantone 17-1926 TCX Fruit Dove` — `#CE5B78`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1926 TCX Fruit Dove"))]
    FruitDove,
    /// `Pantone 17-1927 TCX Desert Rose` — `#CF6977`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1927 TCX Desert Rose"))]
    DesertRose,
    /// `Pantone 17-1928 TCX Bubblegum` — `#EA738D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1928 TCX Bubblegum"))]
    Bubblegum,
    /// `Pantone 17-1929 TCX Rapture Rose` — `#D16277`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1929 TCX Rapture Rose"))]
    RaptureRose,
    /// `Pantone 17-1930 TCX Camellia Rose` — `#EB6081`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1930 TCX Camellia Rose"))]
    CamelliaRose,
    /// `Pantone 17-1937 TCX Hot Pink` — `#E55982`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-1937 TCX Hot Pink"))]
    HotPink,
    /// `Pantone 17-2031 TCX Fuchsia Rose` — `#C74375`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-2031 TCX Fuchsia Rose"))]
    FuchsiaRose,
    /// `Pantone 17-2033 TCX Fandango Pink` — `#E04F80`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-2033 TCX Fandango Pink"))]
    FandangoPink,
    /// `Pantone 17-2034 TCX Pink Yarrow` — `#CE3175`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-2034 TCX Pink Yarrow"))]
    PinkYarrow,
    /// `Pantone 17-2036 TCX Magenta` — `#D23C77`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-2036 TCX Magenta"))]
    Magenta,
    /// `Pantone 17-2120 TCX Chateau Rose` — `#D2738F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-2120 TCX Chateau Rose"))]
    ChateauRose,
    /// `Pantone 17-2127 TCX Shocking Pink` — `#DE5B8C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-2127 TCX Shocking Pink"))]
    ShockingPink,
    /// `Pantone 17-2227 TCX Lilac Rose` — `#BD4275`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-2227 TCX Lilac Rose"))]
    LilacRose,
    /// `Pantone 17-2230 TCX Carmine Rose` — `#E35B8F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-2230 TCX Carmine Rose"))]
    CarmineRose,
    /// `Pantone 17-2411 TCX Toadstool` — `#988088`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-2411 TCX Toadstool"))]
    Toadstool,
    /// `Pantone 17-2520 TCX Ibis Rose` — `#CA628F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-2520 TCX Ibis Rose"))]
    IbisRose,
    /// `Pantone 17-2601 TCX Zinc` — `#92898A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-2601 TCX Zinc"))]
    Zinc,
    /// `Pantone 17-2617 TCX Dahlia Mauve` — `#A64F82`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-2617 TCX Dahlia Mauve"))]
    DahliaMauve,
    /// `Pantone 17-2624 TCX Rose Violet` — `#C0428A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-2624 TCX Rose Violet"))]
    RoseViolet,
    /// `Pantone 17-2625 TCX Super Pink` — `#CE6BA4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-2625 TCX Super Pink"))]
    SuperPink,
    /// `Pantone 17-2627 TCX Phlox Pink` — `#CE5E9A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-2627 TCX Phlox Pink"))]
    PhloxPink,
    /// `Pantone 17-3014 TCX Mulberry` — `#A76C97`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3014 TCX Mulberry"))]
    Mulberry,
    /// `Pantone 17-3020 TCX Spring Crocus` — `#BA69A1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3020 TCX Spring Crocus"))]
    SpringCrocus,
    /// `Pantone 17-3023 TCX Rosebud` — `#B65F9A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3023 TCX Rosebud"))]
    Rosebud,
    /// `Pantone 17-3240 TCX Bodacious` — `#B76BA3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3240 TCX Bodacious"))]
    Bodacious,
    /// `Pantone 17-3313 TCX Dusty Lavender` — `#A1759C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3313 TCX Dusty Lavender"))]
    DustyLavender,
    /// `Pantone 17-3323 TCX Iris Orchid` — `#A767A2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3323 TCX Iris Orchid"))]
    IrisOrchid,
    /// `Pantone 17-3410 TCX Valerian` — `#9F7A93`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3410 TCX Valerian"))]
    Valerian,
    /// `Pantone 17-3520 TCX Diffused Orchid` — `#9879A2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3520 TCX Diffused Orchid"))]
    DiffusedOrchid,
    /// `Pantone 17-3612 TCX Orchid Mist` — `#917798`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3612 TCX Orchid Mist"))]
    OrchidMist,
    /// `Pantone 17-3615 TCX Chalk Violet` — `#8F7DA5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3615 TCX Chalk Violet"))]
    ChalkViolet,
    /// `Pantone 17-3617 TCX English Lavendar` — `#9D7BB0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3617 TCX English Lavendar"))]
    EnglishLavendar,
    /// `Pantone 17-3619 TCX Hyacinth` — `#936CA7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3619 TCX Hyacinth"))]
    Hyacinth,
    /// `Pantone 17-3628 TCX Amethyst Orchid` — `#926AA6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3628 TCX Amethyst Orchid"))]
    AmethystOrchid,
    /// `Pantone 17-3640 TCX Fairy Wren` — `#9479AF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3640 TCX Fairy Wren"))]
    FairyWren,
    /// `Pantone 17-3725 TCX Bougainvillea` — `#9884B9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3725 TCX Bougainvillea"))]
    Bougainvillea,
    /// `Pantone 17-3730 TCX Paisley Purple` — `#8B79B1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3730 TCX Paisley Purple"))]
    PaisleyPurple,
    /// `Pantone 17-3735 TCX Sunlit Allium` — `#9787BB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3735 TCX Sunlit Allium"))]
    SunlitAllium,
    /// `Pantone 17-3802 TCX Gull` — `#918C8F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3802 TCX Gull"))]
    Gull,
    /// `Pantone 17-3808 TCX Nirvana` — `#A2919B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3808 TCX Nirvana"))]
    Nirvana,
    /// `Pantone 17-3810 TCX Purple Ash` — `#8F8395`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3810 TCX Purple Ash"))]
    PurpleAsh,
    /// `Pantone 17-3812 TCX Dusk` — `#897F98`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3812 TCX Dusk"))]
    Dusk,
    /// `Pantone 17-3817 TCX Daybreak` — `#8981A0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3817 TCX Daybreak"))]
    Daybreak,
    /// `Pantone 17-3826 TCX Aster Purple` — `#7D74A8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3826 TCX Aster Purple"))]
    AsterPurple,
    /// `Pantone 17-3834 TCX Dahlia Purple` — `#7E6EAC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3834 TCX Dahlia Purple"))]
    DahliaPurple,
    /// `Pantone 17-3906 TCX Minimal Gray` — `#948D99`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3906 TCX Minimal Gray"))]
    MinimalGray,
    /// `Pantone 17-3907 TCX Quicksilver` — `#7E7D88`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3907 TCX Quicksilver"))]
    Quicksilver,
    /// `Pantone 17-3910 TCX Lavender Gray` — `#9890A2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3910 TCX Lavender Gray"))]
    LavenderGray,
    /// `Pantone 17-3911 TCX Silver Filigree` — `#7F7C81`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3911 TCX Silver Filigree"))]
    SilverFiligree,
    /// `Pantone 17-3914 TCX Sharkskin` — `#838487`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3914 TCX Sharkskin"))]
    Sharkskin,
    /// `Pantone 17-3915 TCX Tempest` — `#79839B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3915 TCX Tempest"))]
    Tempest,
    /// `Pantone 17-3917 TCX Stonewash` — `#74809A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3917 TCX Stonewash"))]
    Stonewash,
    /// `Pantone 17-3918 TCX Country Blue` — `#717F9B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3918 TCX Country Blue"))]
    CountryBlue,
    /// `Pantone 17-3919 TCX Purple Impression` — `#858FB1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3919 TCX Purple Impression"))]
    PurpleImpression,
    /// `Pantone 17-3920 TCX English Manor` — `#7181A4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3920 TCX English Manor"))]
    EnglishManor,
    /// `Pantone 17-3922 TCX Blue Ice` — `#70789B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3922 TCX Blue Ice"))]
    BlueIce,
    /// `Pantone 17-3923 TCX Colony Blue` — `#65769A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3923 TCX Colony Blue"))]
    ColonyBlue,
    /// `Pantone 17-3924 TCX Lavender Violet` — `#767BA5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3924 TCX Lavender Violet"))]
    LavenderViolet,
    /// `Pantone 17-3925 TCX Persian Violet` — `#8C8EB2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3925 TCX Persian Violet"))]
    PersianViolet,
    /// `Pantone 17-3929 TCX Pale Iris` — `#8895C5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3929 TCX Pale Iris"))]
    PaleIris,
    /// `Pantone 17-3930 TCX Jacaranda` — `#848DC5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3930 TCX Jacaranda"))]
    Jacaranda,
    /// `Pantone 17-3932 TCX Deep Periwinkle` — `#7C83BC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3932 TCX Deep Periwinkle"))]
    DeepPeriwinkle,
    /// `Pantone 17-3933 TCX Silver Bullet` — `#81839A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3933 TCX Silver Bullet"))]
    SilverBullet,
    /// `Pantone 17-3934 TCX Persian Jewel` — `#6E81BE`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3934 TCX Persian Jewel"))]
    PersianJewel,
    /// `Pantone 17-3936 TCX Blue Bonnet` — `#6384B8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3936 TCX Blue Bonnet"))]
    BlueBonnet,
    /// `Pantone 17-3940 TCX Iolite` — `#707BB4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-3940 TCX Iolite"))]
    Iolite,
    /// `Pantone 17-4014 TCX Titanium` — `#807D7F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4014 TCX Titanium"))]
    Titanium,
    /// `Pantone 17-4015 TCX Infinity` — `#6E7E99`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4015 TCX Infinity"))]
    Infinity,
    /// `Pantone 17-4016 TCX Gray Flannel` — `#848182`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4016 TCX Gray Flannel"))]
    GrayFlannel,
    /// `Pantone 17-4020 TCX Blue Shadow` — `#66829A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4020 TCX Blue Shadow"))]
    BlueShadow,
    /// `Pantone 17-4021 TCX Faded Denim` — `#798EA4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4021 TCX Faded Denim"))]
    FadedDenim,
    /// `Pantone 17-4023 TCX Blue Heaven` — `#5B7E98`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4023 TCX Blue Heaven"))]
    BlueHeaven,
    /// `Pantone 17-4027 TCX Riviera` — `#5879A2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4027 TCX Riviera"))]
    Riviera,
    /// `Pantone 17-4028 TCX Riverside` — `#4C6A92`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4028 TCX Riverside"))]
    Riverside,
    /// `Pantone 17-4029 TCX Quiet Harbor` — `#5A789A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4029 TCX Quiet Harbor"))]
    QuietHarbor,
    /// `Pantone 17-4030 TCX Silver Lake Blue` — `#618BB9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4030 TCX Silver Lake Blue"))]
    SilverLakeBlue,
    /// `Pantone 17-4032 TCX Lichen Blue` — `#5D89B3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4032 TCX Lichen Blue"))]
    LichenBlue,
    /// `Pantone 17-4033 TCX Pacific Coast` — `#5480AC`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4033 TCX Pacific Coast"))]
    PacificCoast,
    /// `Pantone 17-4037 TCX Ultramarine` — `#5B7EBD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4037 TCX Ultramarine"))]
    Ultramarine,
    /// `Pantone 17-4041 TCX Marina` — `#4F84C4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4041 TCX Marina"))]
    Marina,
    /// `Pantone 17-4111 TCX Citadel` — `#748995`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4111 TCX Citadel"))]
    Citadel,
    /// `Pantone 17-4123 TCX Niagara` — `#5487A4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4123 TCX Niagara"))]
    Niagara,
    /// `Pantone 17-4131 TCX Cendre Blue` — `#3E7FA5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4131 TCX Cendre Blue"))]
    CendreBlue,
    /// `Pantone 17-4139 TCX Azure Blue` — `#4D91C6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4139 TCX Azure Blue"))]
    AzureBlue,
    /// `Pantone 17-4245 TCX Ibiza Blue` — `#007CB7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4245 TCX Ibiza Blue"))]
    IbizaBlue,
    /// `Pantone 17-4247 TCX Diva Blue` — `#007BB2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4247 TCX Diva Blue"))]
    DivaBlue,
    /// `Pantone 17-4320 TCX Adriatic Blue` — `#5C899B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4320 TCX Adriatic Blue"))]
    AdriaticBlue,
    /// `Pantone 17-4328 TCX Blue Moon` — `#3686A0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4328 TCX Blue Moon"))]
    BlueMoon,
    /// `Pantone 17-4336 TCX Blithe` — `#0084BD`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4336 TCX Blithe"))]
    Blithe,
    /// `Pantone 17-4402 TCX Neutral Gray` — `#8E918F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4402 TCX Neutral Gray"))]
    NeutralGray,
    /// `Pantone 17-4405 TCX Monument` — `#84898C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4405 TCX Monument"))]
    Monument,
    /// `Pantone 17-4408 TCX Lead` — `#7A898F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4408 TCX Lead"))]
    Lead,
    /// `Pantone 17-4412 TCX Smoke Blue` — `#6D8994`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4412 TCX Smoke Blue"))]
    SmokeBlue,
    /// `Pantone 17-4421 TCX Larkspur` — `#3C7D90`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4421 TCX Larkspur"))]
    Larkspur,
    /// `Pantone 17-4427 TCX Bluejay` — `#157EA0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4427 TCX Bluejay"))]
    Bluejay,
    /// `Pantone 17-4429 TCX Navagio Bay` — `#3183A0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4429 TCX Navagio Bay"))]
    NavagioBay,
    /// `Pantone 17-4432 TCX Vivid Blue` — `#0088B0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4432 TCX Vivid Blue"))]
    VividBlue,
    /// `Pantone 17-4433 TCX Dresden Blue` — `#0086BB`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4433 TCX Dresden Blue"))]
    DresdenBlue,
    /// `Pantone 17-4435 TCX Malibu Blue` — `#008CC1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4435 TCX Malibu Blue"))]
    MalibuBlue,
    /// `Pantone 17-4440 TCX Blue Danube` — `#0087B6`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4440 TCX Blue Danube"))]
    BlueDanube,
    /// `Pantone 17-4530 TCX Barrier Reef` — `#0084A1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4530 TCX Barrier Reef"))]
    BarrierReef,
    /// `Pantone 17-4540 TCX Hawaiian Ocean` — `#008DB9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4540 TCX Hawaiian Ocean"))]
    HawaiianOcean,
    /// `Pantone 17-4716 TCX Storm Blue` — `#47788A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4716 TCX Storm Blue"))]
    StormBlue,
    /// `Pantone 17-4724 TCX Pagoda Blue` — `#1A7F8E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4724 TCX Pagoda Blue"))]
    PagodaBlue,
    /// `Pantone 17-4728 TCX Algiers Blue` — `#00859C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4728 TCX Algiers Blue"))]
    AlgiersBlue,
    /// `Pantone 17-4730 TCX Caneel Bay` — `#00849F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4730 TCX Caneel Bay"))]
    CaneelBay,
    /// `Pantone 17-4735 TCX Capri Breeze` — `#008799`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4735 TCX Capri Breeze"))]
    CapriBreeze,
    /// `Pantone 17-4818 TCX Bristol Blue` — `#558F91`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4818 TCX Bristol Blue"))]
    BristolBlue,
    /// `Pantone 17-4911 TCX Arctic` — `#648589`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4911 TCX Arctic"))]
    Arctic,
    /// `Pantone 17-4919 TCX Teal` — `#478589`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4919 TCX Teal"))]
    Teal,
    /// `Pantone 17-4928 TCX Lake Blue` — `#008C96`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-4928 TCX Lake Blue"))]
    LakeBlue,
    /// `Pantone 17-5024 TCX Teal Blue` — `#007F7C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5024 TCX Teal Blue"))]
    TealBlue,
    /// `Pantone 17-5025 TCX Navigate` — `#008583`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5025 TCX Navigate"))]
    Navigate,
    /// `Pantone 17-5029 TCX Deep Peacock Blue` — `#008381`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5029 TCX Deep Peacock Blue"))]
    DeepPeacockBlue,
    /// `Pantone 17-5034 TCX Lapis` — `#008684`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5034 TCX Lapis"))]
    Lapis,
    /// `Pantone 17-5102 TCX Griffin` — `#8D8F8F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5102 TCX Griffin"))]
    Griffin,
    /// `Pantone 17-5107 TCX Chinois Green` — `#7C8C87`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5107 TCX Chinois Green"))]
    ChinoisGreen,
    /// `Pantone 17-5110 TCX Trellis` — `#6A8988`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5110 TCX Trellis"))]
    Trellis,
    /// `Pantone 17-5111 TCX Oil Blue` — `#658C88`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5111 TCX Oil Blue"))]
    OilBlue,
    /// `Pantone 17-5117 TCX Green Blue Slate` — `#358082`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5117 TCX Green Blue Slate"))]
    GreenBlueSlate,
    /// `Pantone 17-5122 TCX Latigo Bay` — `#379190`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5122 TCX Latigo Bay"))]
    LatigoBay,
    /// `Pantone 17-5126 TCX Viridian Green` — `#009499`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5126 TCX Viridian Green"))]
    ViridianGreen,
    /// `Pantone 17-5130 TCX Columbia` — `#009288`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5130 TCX Columbia"))]
    Columbia,
    /// `Pantone 17-5330 TCX Dynasty Green` — `#008E80`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5330 TCX Dynasty Green"))]
    DynastyGreen,
    /// `Pantone 17-5335 TCX Spectra Green` — `#009B8C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5335 TCX Spectra Green"))]
    SpectraGreen,
    /// `Pantone 17-5421 TCX Porcelain Green` — `#108780`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5421 TCX Porcelain Green"))]
    PorcelainGreen,
    /// `Pantone 17-5430 TCX Alhambra` — `#008778`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5430 TCX Alhambra"))]
    Alhambra,
    /// `Pantone 17-5513 TCX Deep Sea` — `#4F7C74`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5513 TCX Deep Sea"))]
    DeepSea,
    /// `Pantone 17-5528 TCX Greenlake` — `#007D69`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5528 TCX Greenlake"))]
    Greenlake,
    /// `Pantone 17-5633 TCX Deep Green` — `#009276`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5633 TCX Deep Green"))]
    DeepGreen,
    /// `Pantone 17-5638 TCX Vivid Green` — `#009E82`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5638 TCX Vivid Green"))]
    VividGreen,
    /// `Pantone 17-5641 TCX Emerald` — `#009473`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5641 TCX Emerald"))]
    Emerald,
    /// `Pantone 17-5722 TCX Bottle Green` — `#427D6D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5722 TCX Bottle Green"))]
    BottleGreen,
    /// `Pantone 17-5734 TCX Viridis` — `#00846B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5734 TCX Viridis"))]
    Viridis,
    /// `Pantone 17-5735 TCX Parakeet` — `#008C69`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5735 TCX Parakeet"))]
    Parakeet,
    /// `Pantone 17-5912 TCX Dark Ivy` — `#5B7763`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5912 TCX Dark Ivy"))]
    DarkIvy,
    /// `Pantone 17-5923 TCX Pine Green` — `#3A795E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5923 TCX Pine Green"))]
    PineGreen,
    /// `Pantone 17-5936 TCX Simply Green` — `#009B75`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5936 TCX Simply Green"))]
    SimplyGreen,
    /// `Pantone 17-5937 TCX Deep Mint` — `#009E6D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-5937 TCX Deep Mint"))]
    DeepMint,
    /// `Pantone 17-6009 TCX Laurel Wreath` — `#616F65`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-6009 TCX Laurel Wreath"))]
    LaurelWreath,
    /// `Pantone 17-6030 TCX Jelly Bean` — `#008658`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-6030 TCX Jelly Bean"))]
    JellyBean,
    /// `Pantone 17-6153 TCX Fern Green` — `#008C45`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-6153 TCX Fern Green"))]
    FernGreen,
    /// `Pantone 17-6206 TCX Shadow` — `#888D82`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-6206 TCX Shadow"))]
    Shadow,
    /// `Pantone 17-6212 TCX Sea Spray` — `#717E6F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-6212 TCX Sea Spray"))]
    SeaSpray,
    /// `Pantone 17-6219 TCX Deep Grass Green` — `#558367`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-6219 TCX Deep Grass Green"))]
    DeepGrassGreen,
    /// `Pantone 17-6229 TCX Medium Green` — `#3C824E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-6229 TCX Medium Green"))]
    MediumGreen,
    /// `Pantone 17-6319 TCX Kashmir` — `#6F8D6A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-6319 TCX Kashmir"))]
    Kashmir,
    /// `Pantone 17-6323 TCX Hedge Green` — `#768A75`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-6323 TCX Hedge Green"))]
    HedgeGreen,
    /// `Pantone 17-6333 TCX Mint Green` — `#487D49`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 17-6333 TCX Mint Green"))]
    MintGreen,
    /// `Pantone 18-0000 TCX Smoked Pearl` — `#656466`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0000 TCX Smoked Pearl"))]
    SmokedPearl,
    /// `Pantone 18-0107 TCX Kale` — `#5A7247`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0107 TCX Kale"))]
    Kale,
    /// `Pantone 18-0108 TCX Dill` — `#6F7755`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0108 TCX Dill"))]
    Dill,
    /// `Pantone 18-0110 TCX English Ivy` — `#61845B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0110 TCX English Ivy"))]
    EnglishIvy,
    /// `Pantone 18-0117 TCX Vineyard Green` — `#5F7355`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0117 TCX Vineyard Green"))]
    VineyardGreen,
    /// `Pantone 18-0119 TCX Willow Bough` — `#59754D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0119 TCX Willow Bough"))]
    WillowBough,
    /// `Pantone 18-0121 TCX Elm Green` — `#547053`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0121 TCX Elm Green"))]
    ElmGreen,
    /// `Pantone 18-0125 TCX Artichoke Green` — `#4B6D41`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0125 TCX Artichoke Green"))]
    ArtichokeGreen,
    /// `Pantone 18-0130 TCX Cactus` — `#53713D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0130 TCX Cactus"))]
    Cactus,
    /// `Pantone 18-0135 TCX Treetop` — `#476A30`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0135 TCX Treetop"))]
    Treetop,
    /// `Pantone 18-0201 TCX Castlerock` — `#5F5E62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0201 TCX Castlerock"))]
    Castlerock,
    /// `Pantone 18-0220 TCX Mayfly` — `#65663F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0220 TCX Mayfly"))]
    Mayfly,
    /// `Pantone 18-0228 TCX Pesto` — `#595F34`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0228 TCX Pesto"))]
    Pesto,
    /// `Pantone 18-0306 TCX Gunmetal` — `#5C5D5B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0306 TCX Gunmetal"))]
    Gunmetal,
    /// `Pantone 18-0312 TCX Deep Lichen Green` — `#6E6E5C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0312 TCX Deep Lichen Green"))]
    DeepLichenGreen,
    /// `Pantone 18-0316 TCX Olivine` — `#666B54`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0316 TCX Olivine"))]
    Olivine,
    /// `Pantone 18-0317 TCX Bronze Green` — `#525F48`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0317 TCX Bronze Green"))]
    BronzeGreen,
    /// `Pantone 18-0322 TCX Cypress` — `#545A3E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0322 TCX Cypress"))]
    Cypress,
    /// `Pantone 18-0324 TCX Calliste Green` — `#757A4E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0324 TCX Calliste Green"))]
    CallisteGreen,
    /// `Pantone 18-0328 TCX Cedar Green` — `#5E6737`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0328 TCX Cedar Green"))]
    CedarGreen,
    /// `Pantone 18-0330 TCX Twist Of Lime` — `#4E632C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0330 TCX Twist Of Lime"))]
    TwistOfLime,
    /// `Pantone 18-0332 TCX Grasshopper` — `#77824A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0332 TCX Grasshopper"))]
    Grasshopper,
    /// `Pantone 18-0403 TCX Dark Gull Gray` — `#625D5D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0403 TCX Dark Gull Gray"))]
    DarkGullGray,
    /// `Pantone 18-0420 TCX Four Leaf Clover` — `#616652`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0420 TCX Four Leaf Clover"))]
    FourLeafClover,
    /// `Pantone 18-0422 TCX Loden Green` — `#6E7153`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0422 TCX Loden Green"))]
    LodenGreen,
    /// `Pantone 18-0426 TCX Capulet Olive` — `#656344`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0426 TCX Capulet Olive"))]
    CapuletOlive,
    /// `Pantone 18-0430 TCX Avocado` — `#676232`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0430 TCX Avocado"))]
    Avocado,
    /// `Pantone 18-0435 TCX Calla Green` — `#6A6F34`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0435 TCX Calla Green"))]
    CallaGreen,
    /// `Pantone 18-0503 TCX Gargoyle` — `#686767`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0503 TCX Gargoyle"))]
    Gargoyle,
    /// `Pantone 18-0510 TCX Castor Gray` — `#646762`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0510 TCX Castor Gray"))]
    CastorGray,
    /// `Pantone 18-0513 TCX Bungee Cord` — `#696156`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0513 TCX Bungee Cord"))]
    BungeeCord,
    /// `Pantone 18-0515 TCX Dusty Olive` — `#646356`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0515 TCX Dusty Olive"))]
    DustyOlive,
    /// `Pantone 18-0521 TCX Burnt Olive` — `#646049`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0521 TCX Burnt Olive"))]
    BurntOlive,
    /// `Pantone 18-0523 TCX Winter Moss` — `#5B5A41`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0523 TCX Winter Moss"))]
    WinterMoss,
    /// `Pantone 18-0525 TCX Iguana` — `#818455`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0525 TCX Iguana"))]
    Iguana,
    /// `Pantone 18-0527 TCX Olive Branch` — `#646A45`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0527 TCX Olive Branch"))]
    OliveBranch,
    /// `Pantone 18-0538 TCX Woodbine` — `#7B7F32`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0538 TCX Woodbine"))]
    Woodbine,
    /// `Pantone 18-0601 TCX Charcoal Gray` — `#6C6868`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0601 TCX Charcoal Gray"))]
    CharcoalGray,
    /// `Pantone 18-0615 TCX Stone Gray` — `#685E4F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0615 TCX Stone Gray"))]
    StoneGray,
    /// `Pantone 18-0617 TCX Covert Green` — `#80765F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0617 TCX Covert Green"))]
    CovertGreen,
    /// `Pantone 18-0622 TCX Olive Drab` — `#756D47`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0622 TCX Olive Drab"))]
    OliveDrab,
    /// `Pantone 18-0625 TCX Martini Olive` — `#716A4D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0625 TCX Martini Olive"))]
    MartiniOlive,
    /// `Pantone 18-0627 TCX Fir Green` — `#67592A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0627 TCX Fir Green"))]
    FirGreen,
    /// `Pantone 18-0629 TCX Lizard` — `#71643E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0629 TCX Lizard"))]
    Lizard,
    /// `Pantone 18-0724 TCX Gothic Olive` — `#7C6E4F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0724 TCX Gothic Olive"))]
    GothicOlive,
    /// `Pantone 18-0820 TCX Capers` — `#695E4B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0820 TCX Capers"))]
    Capers,
    /// `Pantone 18-0825 TCX Nutria` — `#75663E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0825 TCX Nutria"))]
    Nutria,
    /// `Pantone 18-0830 TCX Butternut` — `#7A643F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0830 TCX Butternut"))]
    Butternut,
    /// `Pantone 18-0832 TCX Plantation` — `#7A6332`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0832 TCX Plantation"))]
    Plantation,
    /// `Pantone 18-0835 TCX Dried Tobacco` — `#997B38`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0835 TCX Dried Tobacco"))]
    DriedTobacco,
    /// `Pantone 18-0840 TCX Tapenade` — `#805D24`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0840 TCX Tapenade"))]
    Tapenade,
    /// `Pantone 18-0920 TCX Kangaroo` — `#725E43`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0920 TCX Kangaroo"))]
    Kangaroo,
    /// `Pantone 18-0928 TCX Sepia` — `#6B543E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0928 TCX Sepia"))]
    Sepia,
    /// `Pantone 18-0930 TCX Coffee Liqueur` — `#6A513B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0930 TCX Coffee Liqueur"))]
    CoffeeLiqueur,
    /// `Pantone 18-0933 TCX Rubber` — `#815B37`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0933 TCX Rubber"))]
    Rubber,
    /// `Pantone 18-0935 TCX Buckthorn Brown` — `#A76F1F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0935 TCX Buckthorn Brown"))]
    BuckthornBrown,
    /// `Pantone 18-0937 TCX Bronze Brown` — `#825E2F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0937 TCX Bronze Brown"))]
    BronzeBrown,
    /// `Pantone 18-0939 TCX Cumin` — `#927240`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0939 TCX Cumin"))]
    Cumin,
    /// `Pantone 18-0940 TCX Golden Brown` — `#91672F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0940 TCX Golden Brown"))]
    GoldenBrown,
    /// `Pantone 18-0950 TCX Cathay Spice` — `#99642C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-0950 TCX Cathay Spice"))]
    CathaySpice,
    /// `Pantone 18-1015 TCX Shitake` — `#736253`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1015 TCX Shitake"))]
    Shitake,
    /// `Pantone 18-1016 TCX Cub` — `#6E5C4B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1016 TCX Cub"))]
    Cub,
    /// `Pantone 18-1017 TCX Caribou` — `#816D5E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1017 TCX Caribou"))]
    Caribou,
    /// `Pantone 18-1018 TCX Otter` — `#7F674F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1018 TCX Otter"))]
    Otter,
    /// `Pantone 18-1022 TCX Ermine` — `#836B4F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1022 TCX Ermine"))]
    Ermine,
    /// `Pantone 18-1027 TCX Bison` — `#6E4F3A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1027 TCX Bison"))]
    Bison,
    /// `Pantone 18-1028 TCX Emperador` — `#684832`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1028 TCX Emperador"))]
    Emperador,
    /// `Pantone 18-1029 TCX Toasted Coconut` — `#8B6A4F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1029 TCX Toasted Coconut"))]
    ToastedCoconut,
    /// `Pantone 18-1030 TCX Thrush` — `#936B4F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1030 TCX Thrush"))]
    Thrush,
    /// `Pantone 18-1031 TCX Toffee` — `#755139`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1031 TCX Toffee"))]
    Toffee,
    /// `Pantone 18-1033 TCX Dachshund` — `#704F37`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1033 TCX Dachshund"))]
    Dachshund,
    /// `Pantone 18-1048 TCX Monks Robe` — `#704822`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1048 TCX Monks Robe"))]
    MonksRobe,
    /// `Pantone 18-1049 TCX Thai Curry` — `#AB6819`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1049 TCX Thai Curry"))]
    ThaiCurry,
    /// `Pantone 18-1050 TCX Honey Ginger` — `#A86217`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1050 TCX Honey Ginger"))]
    HoneyGinger,
    /// `Pantone 18-1108 TCX Fallen Rock` — `#807669`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1108 TCX Fallen Rock"))]
    FallenRock,
    /// `Pantone 18-1110 TCX Brindle` — `#82776B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1110 TCX Brindle"))]
    Brindle,
    /// `Pantone 18-1112 TCX Walnut` — `#776A5F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1112 TCX Walnut"))]
    Walnut,
    /// `Pantone 18-1124 TCX Partridge` — `#725440`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1124 TCX Partridge"))]
    Partridge,
    /// `Pantone 18-1130 TCX Aztec` — `#7A5747`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1130 TCX Aztec"))]
    Aztec,
    /// `Pantone 18-1137 TCX Rawhide` — `#865E49`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1137 TCX Rawhide"))]
    Rawhide,
    /// `Pantone 18-1140 TCX Mocha Bisque` — `#8C543A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1140 TCX Mocha Bisque"))]
    MochaBisque,
    /// `Pantone 18-1142 TCX Leather Brown` — `#97572B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1142 TCX Leather Brown"))]
    LeatherBrown,
    /// `Pantone 18-1148 TCX Caramel Cafe` — `#864C24`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1148 TCX Caramel Cafe"))]
    CaramelCafe,
    /// `Pantone 18-1154 TCX Glazed Ginger` — `#91552B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1154 TCX Glazed Ginger"))]
    GlazedGinger,
    /// `Pantone 18-1155 TCX Sugar Almond` — `#935529`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1155 TCX Sugar Almond"))]
    SugarAlmond,
    /// `Pantone 18-1160 TCX Sudan Brown` — `#AC6B29`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1160 TCX Sudan Brown"))]
    SudanBrown,
    /// `Pantone 18-1163 TCX Pumpkin Spice` — `#A05C17`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1163 TCX Pumpkin Spice"))]
    PumpkinSpice,
    /// `Pantone 18-1210 TCX Driftwood` — `#847A75`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1210 TCX Driftwood"))]
    Driftwood,
    /// `Pantone 18-1222 TCX Cocoa Brown` — `#6C5043`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1222 TCX Cocoa Brown"))]
    CocoaBrown,
    /// `Pantone 18-1229 TCX Carob Brown` — `#855C4C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1229 TCX Carob Brown"))]
    CarobBrown,
    /// `Pantone 18-1230 TCX Coconut Shell` — `#874E3C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1230 TCX Coconut Shell"))]
    CoconutShell,
    /// `Pantone 18-1235 TCX Russet` — `#8F5F50`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1235 TCX Russet"))]
    Russet,
    /// `Pantone 18-1238 TCX Rustic Brown` — `#855141`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1238 TCX Rustic Brown"))]
    RusticBrown,
    /// `Pantone 18-1239 TCX Sierra` — `#985C41`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1239 TCX Sierra"))]
    Sierra,
    /// `Pantone 18-1242 TCX Brown Patina` — `#834F3D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1242 TCX Brown Patina"))]
    BrownPatina,
    /// `Pantone 18-1244 TCX Ginger Bread` — `#8C4A2F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1244 TCX Ginger Bread"))]
    GingerBread,
    /// `Pantone 18-1246 TCX Umber` — `#944A1F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1246 TCX Umber"))]
    Umber,
    /// `Pantone 18-1248 TCX Rust` — `#B55A30`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1248 TCX Rust"))]
    Rust,
    /// `Pantone 18-1249 TCX Hawaiian Sunset` — `#BB5C14`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1249 TCX Hawaiian Sunset"))]
    HawaiianSunset,
    /// `Pantone 18-1250 TCX Bombay Brown` — `#9F5130`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1250 TCX Bombay Brown"))]
    BombayBrown,
    /// `Pantone 18-1304 TCX Falcon` — `#6D625B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1304 TCX Falcon"))]
    Falcon,
    /// `Pantone 18-1306 TCX Iron` — `#736460`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1306 TCX Iron"))]
    Iron,
    /// `Pantone 18-1312 TCX Deep Taupe` — `#7B6660`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1312 TCX Deep Taupe"))]
    DeepTaupe,
    /// `Pantone 18-1314 TCX Acorn` — `#7E5E52`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1314 TCX Acorn"))]
    Acorn,
    /// `Pantone 18-1320 TCX Clove` — `#876155`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1320 TCX Clove"))]
    Clove,
    /// `Pantone 18-1321 TCX Brownie` — `#8F7265`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1321 TCX Brownie"))]
    Brownie,
    /// `Pantone 18-1325 TCX Spiced Apple` — `#783937`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1325 TCX Spiced Apple"))]
    SpicedApple,
    /// `Pantone 18-1326 TCX Nutmeg` — `#7E5C54`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1326 TCX Nutmeg"))]
    Nutmeg,
    /// `Pantone 18-1336 TCX Copper Brown` — `#9A6051`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1336 TCX Copper Brown"))]
    CopperBrown,
    /// `Pantone 18-1340 TCX Potters Clay` — `#9E4624`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1340 TCX Potters Clay"))]
    PottersClay,
    /// `Pantone 18-1343 TCX Auburn` — `#A15843`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1343 TCX Auburn"))]
    Auburn,
    /// `Pantone 18-1345 TCX Cinnamon Stick` — `#9B4722`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1345 TCX Cinnamon Stick"))]
    CinnamonStick,
    /// `Pantone 18-1346 TCX Bruschetta` — `#A75949`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1346 TCX Bruschetta"))]
    Bruschetta,
    /// `Pantone 18-1350 TCX Burnt Brick` — `#A14D3A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1350 TCX Burnt Brick"))]
    BurntBrick,
    /// `Pantone 18-1354 TCX Burnt Ochre` — `#BB4F35`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1354 TCX Burnt Ochre"))]
    BurntOchre,
    /// `Pantone 18-1355 TCX Rooibos Tea` — `#A23C26`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1355 TCX Rooibos Tea"))]
    RooibosTea,
    /// `Pantone 18-1404 TCX Sparrow` — `#69595C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1404 TCX Sparrow"))]
    Sparrow,
    /// `Pantone 18-1405 TCX Flint` — `#705861`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1405 TCX Flint"))]
    Flint,
    /// `Pantone 18-1409 TCX Peppercorn` — `#6C5656`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1409 TCX Peppercorn"))]
    Peppercorn,
    /// `Pantone 18-1411 TCX Plum Wine` — `#674550`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1411 TCX Plum Wine"))]
    PlumWine,
    /// `Pantone 18-1415 TCX Marron` — `#6E4C4B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1415 TCX Marron"))]
    Marron,
    /// `Pantone 18-1418 TCX Crushed Berry` — `#804F5A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1418 TCX Crushed Berry"))]
    CrushedBerry,
    /// `Pantone 18-1420 TCX Wild Ginger` — `#7C4C53`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1420 TCX Wild Ginger"))]
    WildGinger,
    /// `Pantone 18-1421 TCX Cognac` — `#8B645A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1421 TCX Cognac"))]
    Cognac,
    /// `Pantone 18-1425 TCX Mahogany` — `#824D46`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1425 TCX Mahogany"))]
    Mahogany,
    /// `Pantone 18-1426 TCX Apple Butter` — `#844B4D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1426 TCX Apple Butter"))]
    AppleButter,
    /// `Pantone 18-1433 TCX Chutney` — `#98594B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1433 TCX Chutney"))]
    Chutney,
    /// `Pantone 18-1434 TCX Etruscan Red` — `#A2574B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1434 TCX Etruscan Red"))]
    EtruscanRed,
    /// `Pantone 18-1435 TCX Withered Rose` — `#A26666`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1435 TCX Withered Rose"))]
    WitheredRose,
    /// `Pantone 18-1436 TCX Light Mahogany` — `#AD6D68`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1436 TCX Light Mahogany"))]
    LightMahogany,
    /// `Pantone 18-1438 TCX Marsala` — `#964F4C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1438 TCX Marsala"))]
    Marsala,
    /// `Pantone 18-1440 TCX Chili Oil` — `#8E3C36`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1440 TCX Chili Oil"))]
    ChiliOil,
    /// `Pantone 18-1441 TCX Baked Clay` — `#9C5642`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1441 TCX Baked Clay"))]
    BakedClay,
    /// `Pantone 18-1442 TCX Red Ochre` — `#913832`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1442 TCX Red Ochre"))]
    RedOchre,
    /// `Pantone 18-1443 TCX Redwood` — `#A6594C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1443 TCX Redwood"))]
    Redwood,
    /// `Pantone 18-1444 TCX Tandori Spice` — `#9F4440`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1444 TCX Tandori Spice"))]
    TandoriSpice,
    /// `Pantone 18-1445 TCX Spicy Orange` — `#D73C26`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1445 TCX Spicy Orange"))]
    SpicyOrange,
    /// `Pantone 18-1447 TCX Orange Rust` — `#C25A3C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1447 TCX Orange Rust"))]
    OrangeRust,
    /// `Pantone 18-1448 TCX Chili` — `#BE5141`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1448 TCX Chili"))]
    Chili,
    /// `Pantone 18-1449 TCX Ketchup` — `#9A382D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1449 TCX Ketchup"))]
    Ketchup,
    /// `Pantone 18-1450 TCX Mecca Orange` — `#BD5745`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1450 TCX Mecca Orange"))]
    MeccaOrange,
    /// `Pantone 18-1451 TCX Autumn Glaze` — `#B3573F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1451 TCX Autumn Glaze"))]
    AutumnGlaze,
    /// `Pantone 18-1454 TCX Red Clay` — `#C2452D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1454 TCX Red Clay"))]
    RedClay,
    /// `Pantone 18-1506 TCX Plum Truffle` — `#675657`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1506 TCX Plum Truffle"))]
    PlumTruffle,
    /// `Pantone 18-1512 TCX Rose Brown` — `#80565B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1512 TCX Rose Brown"))]
    RoseBrown,
    /// `Pantone 18-1531 TCX Barn Red` — `#8F423B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1531 TCX Barn Red"))]
    BarnRed,
    /// `Pantone 18-1535 TCX Ginger Spice` — `#B65D48`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1535 TCX Ginger Spice"))]
    GingerSpice,
    /// `Pantone 18-1536 TCX Hot Sauce` — `#AB4F41`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1536 TCX Hot Sauce"))]
    HotSauce,
    /// `Pantone 18-1537 TCX Copper Coin` — `#BA6B57`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1537 TCX Copper Coin"))]
    CopperCoin,
    /// `Pantone 18-1540 TCX Cinnabar` — `#9C453B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1540 TCX Cinnabar"))]
    Cinnabar,
    /// `Pantone 18-1541 TCX Brandy Brown` — `#73362A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1541 TCX Brandy Brown"))]
    BrandyBrown,
    /// `Pantone 18-1547 TCX Bossa Nova` — `#973A36`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1547 TCX Bossa Nova"))]
    BossaNova,
    /// `Pantone 18-1549 TCX Valiant Poppy` — `#BC322C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1549 TCX Valiant Poppy"))]
    ValiantPoppy,
    /// `Pantone 18-1550 TCX Aurora Red` — `#B93A32`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1550 TCX Aurora Red"))]
    AuroraRed,
    /// `Pantone 18-1551 TCX Aura Orange` — `#B4262A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1551 TCX Aura Orange"))]
    AuraOrange,
    /// `Pantone 18-1555 TCX Molten Lava` — `#B5332E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1555 TCX Molten Lava"))]
    MoltenLava,
    /// `Pantone 18-1561 TCX Orange Com` — `#DA321C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1561 TCX Orange Com"))]
    OrangeCom,
    /// `Pantone 18-1564 TCX Poinciana` — `#CA3422`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1564 TCX Poinciana"))]
    Poinciana,
    /// `Pantone 18-1612 TCX Rose Taupe` — `#806062`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1612 TCX Rose Taupe"))]
    RoseTaupe,
    /// `Pantone 18-1613 TCX Renaissance Rose` — `#865560`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1613 TCX Renaissance Rose"))]
    RenaissanceRose,
    /// `Pantone 18-1614 TCX Nocturne` — `#7A4B56`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1614 TCX Nocturne"))]
    Nocturne,
    /// `Pantone 18-1616 TCX Roan Rouge` — `#885157`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1616 TCX Roan Rouge"))]
    RoanRouge,
    /// `Pantone 18-1619 TCX Maroon` — `#834655`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1619 TCX Maroon"))]
    Maroon,
    /// `Pantone 18-1629 TCX Faded Rose` — `#BF6464`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1629 TCX Faded Rose"))]
    FadedRose,
    /// `Pantone 18-1630 TCX Dusty Cedar` — `#AD5D5D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1630 TCX Dusty Cedar"))]
    DustyCedar,
    /// `Pantone 18-1631 TCX Earth Red` — `#95424E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1631 TCX Earth Red"))]
    EarthRed,
    /// `Pantone 18-1633 TCX Garnet Rose` — `#AC4B55`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1633 TCX Garnet Rose"))]
    GarnetRose,
    /// `Pantone 18-1634 TCX Baroque Rose` — `#B35A66`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1634 TCX Baroque Rose"))]
    BaroqueRose,
    /// `Pantone 18-1635 TCX Slate Rose` — `#B45865`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1635 TCX Slate Rose"))]
    SlateRose,
    /// `Pantone 18-1643 TCX Cardinal` — `#AD3E48`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1643 TCX Cardinal"))]
    Cardinal,
    /// `Pantone 18-1648 TCX Baked Apple` — `#B34646`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1648 TCX Baked Apple"))]
    BakedApple,
    /// `Pantone 18-1649 TCX Deep Sea Coral` — `#D9615B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1649 TCX Deep Sea Coral"))]
    DeepSeaCoral,
    /// `Pantone 18-1651 TCX Cayenne` — `#E04951`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1651 TCX Cayenne"))]
    Cayenne,
    /// `Pantone 18-1652 TCX Rococco Red` — `#BB363F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1652 TCX Rococco Red"))]
    RococcoRed,
    /// `Pantone 18-1653 TCX Toreador` — `#B61032`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1653 TCX Toreador"))]
    Toreador,
    /// `Pantone 18-1654 TCX Lychee` — `#BA0B32`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1654 TCX Lychee"))]
    Lychee,
    /// `Pantone 18-1655 TCX Mars Red` — `#BC2731`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1655 TCX Mars Red"))]
    MarsRed,
    /// `Pantone 18-1657 TCX Salsa` — `#AA182B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1657 TCX Salsa"))]
    Salsa,
    /// `Pantone 18-1658 TCX Pompeian Red` — `#A4292E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1658 TCX Pompeian Red"))]
    PompeianRed,
    /// `Pantone 18-1659 TCX Goji Berry` — `#B91228`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1659 TCX Goji Berry"))]
    GojiBerry,
    /// `Pantone 18-1660 TCX Tomato` — `#CE2939`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1660 TCX Tomato"))]
    Tomato,
    /// `Pantone 18-1661 TCX Tomato Puree` — `#C53346`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1661 TCX Tomato Puree"))]
    TomatoPuree,
    /// `Pantone 18-1662 TCX Flame Scarlet` — `#CD212A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1662 TCX Flame Scarlet"))]
    FlameScarlet,
    /// `Pantone 18-1663 TCX Chinese Red` — `#BE132D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1663 TCX Chinese Red"))]
    ChineseRed,
    /// `Pantone 18-1664 TCX Fiery Red` — `#D01C1F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1664 TCX Fiery Red"))]
    FieryRed,
    /// `Pantone 18-1703 TCX Shark` — `#6D636B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1703 TCX Shark"))]
    Shark,
    /// `Pantone 18-1705 TCX Arctic Dusk` — `#735B6A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1705 TCX Arctic Dusk"))]
    ArcticDusk,
    /// `Pantone 18-1706 TCX Black Plum` — `#6C5765`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1706 TCX Black Plum"))]
    BlackPlum,
    /// `Pantone 18-1708 TCX Ephemera` — `#6F5965`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1708 TCX Ephemera"))]
    Ephemera,
    /// `Pantone 18-1709 TCX Tulipwood` — `#805466`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1709 TCX Tulipwood"))]
    Tulipwood,
    /// `Pantone 18-1710 TCX Grape Nectar` — `#8D5C74`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1710 TCX Grape Nectar"))]
    GrapeNectar,
    /// `Pantone 18-1716 TCX Damson` — `#854C65`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1716 TCX Damson"))]
    Damson,
    /// `Pantone 18-1718 TCX Hawthorn Rose` — `#884C5E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1718 TCX Hawthorn Rose"))]
    HawthornRose,
    /// `Pantone 18-1720 TCX Violet Quartz` — `#8B4963`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1720 TCX Violet Quartz"))]
    VioletQuartz,
    /// `Pantone 18-1725 TCX Dry Rose` — `#8C4759`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1725 TCX Dry Rose"))]
    DryRose,
    /// `Pantone 18-1741 TCX Raspberry Wine` — `#B63753`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1741 TCX Raspberry Wine"))]
    RaspberryWine,
    /// `Pantone 18-1754 TCX Raspberry` — `#D32E5E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1754 TCX Raspberry"))]
    Raspberry,
    /// `Pantone 18-1755 TCX Rouge Red` — `#E24666`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1755 TCX Rouge Red"))]
    RougeRed,
    /// `Pantone 18-1756 TCX Teaberry` — `#DC3855`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1756 TCX Teaberry"))]
    Teaberry,
    /// `Pantone 18-1759 TCX Jalapeno Red` — `#B2103C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1759 TCX Jalapeno Red"))]
    JalapenoRed,
    /// `Pantone 18-1760 TCX Barberry` — `#BF1945`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1760 TCX Barberry"))]
    Barberry,
    /// `Pantone 18-1761 TCX Ski Patrol` — `#BB1237`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1761 TCX Ski Patrol"))]
    SkiPatrol,
    /// `Pantone 18-1762 TCX Hibiscus` — `#DD3848`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1762 TCX Hibiscus"))]
    Hibiscus,
    /// `Pantone 18-1763 TCX High Risk Red` — `#C71F2D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1763 TCX High Risk Red"))]
    HighRiskRed,
    /// `Pantone 18-1764 TCX Lollipop` — `#CC1C3B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1764 TCX Lollipop"))]
    Lollipop,
    /// `Pantone 18-1807 TCX Twilight Mauve` — `#8B6F70`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1807 TCX Twilight Mauve"))]
    TwilightMauve,
    /// `Pantone 18-1852 TCX Rose Red` — `#C92351`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1852 TCX Rose Red"))]
    RoseRed,
    /// `Pantone 18-1856 TCX Virtual Pink` — `#C6174E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1856 TCX Virtual Pink"))]
    VirtualPink,
    /// `Pantone 18-1945 TCX Bright Rose` — `#C51959`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1945 TCX Bright Rose"))]
    BrightRose,
    /// `Pantone 18-1950 TCX Jazzy` — `#B61C50`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1950 TCX Jazzy"))]
    Jazzy,
    /// `Pantone 18-1951 TCX Love Potion` — `#C01352`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-1951 TCX Love Potion"))]
    LovePotion,
    /// `Pantone 18-2027 TCX Beaujolais` — `#80304C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-2027 TCX Beaujolais"))]
    Beaujolais,
    /// `Pantone 18-2043 TCX Raspberry Sorbet` — `#D2386C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-2043 TCX Raspberry Sorbet"))]
    RaspberrySorbet,
    /// `Pantone 18-2045 TCX Pink Peacock` — `#C62168`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-2045 TCX Pink Peacock"))]
    PinkPeacock,
    /// `Pantone 18-2109 TCX Grape Shake` — `#886971`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-2109 TCX Grape Shake"))]
    GrapeShake,
    /// `Pantone 18-2120 TCX Honeysuckle` — `#D94F70`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-2120 TCX Honeysuckle"))]
    Honeysuckle,
    /// `Pantone 18-2133 TCX Pink Flambe` — `#D3507A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-2133 TCX Pink Flambe"))]
    PinkFlambe,
    /// `Pantone 18-2140 TCX Cabaret` — `#CB3373`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-2140 TCX Cabaret"))]
    Cabaret,
    /// `Pantone 18-2143 TCX Beetroot Purple` — `#CF2D71`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-2143 TCX Beetroot Purple"))]
    BeetrootPurple,
    /// `Pantone 18-2320 TCX Clover` — `#8A3371`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-2320 TCX Clover"))]
    Clover,
    /// `Pantone 18-2326 TCX Cactus Flower` — `#A83E6C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-2326 TCX Cactus Flower"))]
    CactusFlower,
    /// `Pantone 18-2328 TCX Fuchsia Red` — `#AB3475`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-2328 TCX Fuchsia Red"))]
    FuchsiaRed,
    /// `Pantone 18-2333 TCX Raspberry Rose` — `#CC4385`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-2333 TCX Raspberry Rose"))]
    RaspberryRose,
    /// `Pantone 18-2336 TCX Very Berry` — `#B73275`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-2336 TCX Very Berry"))]
    VeryBerry,
    /// `Pantone 18-2436 TCX Fuchsia Purple` — `#D33479`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-2436 TCX Fuchsia Purple"))]
    FuchsiaPurple,
    /// `Pantone 18-2525 TCX Magenta Haze` — `#9D446E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-2525 TCX Magenta Haze"))]
    MagentaHaze,
    /// `Pantone 18-2527 TCX Baton Rouge` — `#973C6C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-2527 TCX Baton Rouge"))]
    BatonRouge,
    /// `Pantone 18-2929 TCX Purple Wine` — `#8C3573`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-2929 TCX Purple Wine"))]
    PurpleWine,
    /// `Pantone 18-3011 TCX Argyle Purple` — `#895C79`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3011 TCX Argyle Purple"))]
    ArgylePurple,
    /// `Pantone 18-3012 TCX Purple Gumdrop` — `#7A596F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3012 TCX Purple Gumdrop"))]
    PurpleGumdrop,
    /// `Pantone 18-3013 TCX Berry Conserve` — `#765269`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3013 TCX Berry Conserve"))]
    BerryConserve,
    /// `Pantone 18-3014 TCX Grape Kiss` — `#7B4368`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3014 TCX Grape Kiss"))]
    GrapeKiss,
    /// `Pantone 18-3015 TCX Amethyst` — `#864D75`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3015 TCX Amethyst"))]
    Amethyst,
    /// `Pantone 18-3022 TCX Deep Orchid` — `#903F75`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3022 TCX Deep Orchid"))]
    DeepOrchid,
    /// `Pantone 18-3025 TCX Striking Purple` — `#944E87`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3025 TCX Striking Purple"))]
    StrikingPurple,
    /// `Pantone 18-3027 TCX Purple Orchid` — `#AD4D8C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3027 TCX Purple Orchid"))]
    PurpleOrchid,
    /// `Pantone 18-3120 TCX Willowherb` — `#8E4483`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3120 TCX Willowherb"))]
    Willowherb,
    /// `Pantone 18-3211 TCX Grapeade` — `#85677B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3211 TCX Grapeade"))]
    Grapeade,
    /// `Pantone 18-3218 TCX Concord Grape` — `#7C5379`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3218 TCX Concord Grape"))]
    ConcordGrape,
    /// `Pantone 18-3220 TCX Very Grape` — `#927288`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3220 TCX Very Grape"))]
    VeryGrape,
    /// `Pantone 18-3224 TCX Radiant Orchid` — `#AD5E99`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3224 TCX Radiant Orchid"))]
    RadiantOrchid,
    /// `Pantone 18-3230 TCX Meadow Mauve` — `#A9568C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3230 TCX Meadow Mauve"))]
    MeadowMauve,
    /// `Pantone 18-3324 TCX Dahlia` — `#843E83`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3324 TCX Dahlia"))]
    Dahlia,
    /// `Pantone 18-3331 TCX Hyacinth Violet` — `#8D4687`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3331 TCX Hyacinth Violet"))]
    HyacinthViolet,
    /// `Pantone 18-3339 TCX Vivid Viola` — `#993C7C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3339 TCX Vivid Viola"))]
    VividViola,
    /// `Pantone 18-3340 TCX Charisma` — `#632A60`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3340 TCX Charisma"))]
    Charisma,
    /// `Pantone 18-3410 TCX Vintage Violet` — `#634F62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3410 TCX Vintage Violet"))]
    VintageViolet,
    /// `Pantone 18-3415 TCX Grape Jam` — `#725671`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3415 TCX Grape Jam"))]
    GrapeJam,
    /// `Pantone 18-3418 TCX Chinese Violet` — `#835E81`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3418 TCX Chinese Violet"))]
    ChineseViolet,
    /// `Pantone 18-3513 TCX Grape Compote` — `#6B5876`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3513 TCX Grape Compote"))]
    GrapeCompote,
    /// `Pantone 18-3518 TCX Patrician Purple` — `#6C4E79`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3518 TCX Patrician Purple"))]
    PatricianPurple,
    /// `Pantone 18-3520 TCX Purple Heart` — `#745587`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3520 TCX Purple Heart"))]
    PurpleHeart,
    /// `Pantone 18-3521 TCX Plum Jam` — `#624076`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3521 TCX Plum Jam"))]
    PlumJam,
    /// `Pantone 18-3522 TCX Crushed Grape` — `#7A547F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3522 TCX Crushed Grape"))]
    CrushedGrape,
    /// `Pantone 18-3530 TCX Lavender Crystal` — `#936A98`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3530 TCX Lavender Crystal"))]
    LavenderCrystal,
    /// `Pantone 18-3531 TCX Royal Lilac` — `#774D8E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3531 TCX Royal Lilac"))]
    RoyalLilac,
    /// `Pantone 18-3533 TCX Dewberry` — `#8B5987`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3533 TCX Dewberry"))]
    Dewberry,
    /// `Pantone 18-3540 TCX Purple Sapphire` — `#6F4685`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3540 TCX Purple Sapphire"))]
    PurpleSapphire,
    /// `Pantone 18-3615 TCX Imperial Palace` — `#604E7A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3615 TCX Imperial Palace"))]
    ImperialPalace,
    /// `Pantone 18-3620 TCX Mystical` — `#5F4E72`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3620 TCX Mystical"))]
    Mystical,
    /// `Pantone 18-3628 TCX Bellflower` — `#9469A2`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3628 TCX Bellflower"))]
    Bellflower,
    /// `Pantone 18-3633 TCX Deep Lavender` — `#775496`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3633 TCX Deep Lavender"))]
    DeepLavender,
    /// `Pantone 18-3634 TCX Chive Blossom` — `#7D5D99`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3634 TCX Chive Blossom"))]
    ChiveBlossom,
    /// `Pantone 18-3635 TCX Picasso Lily` — `#634878`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3635 TCX Picasso Lily"))]
    PicassoLily,
    /// `Pantone 18-3710 TCX Gray Ridge` — `#847986`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3710 TCX Gray Ridge"))]
    GrayRidge,
    /// `Pantone 18-3712 TCX Purple Sage` — `#75697E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3712 TCX Purple Sage"))]
    PurpleSage,
    /// `Pantone 18-3714 TCX Mulled Grape` — `#675A74`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3714 TCX Mulled Grape"))]
    MulledGrape,
    /// `Pantone 18-3715 TCX Montana Grape` — `#6C5971`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3715 TCX Montana Grape"))]
    MontanaGrape,
    /// `Pantone 18-3718 TCX Purple Haze` — `#807396`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3718 TCX Purple Haze"))]
    PurpleHaze,
    /// `Pantone 18-3737 TCX Passion Flower` — `#6D5698`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3737 TCX Passion Flower"))]
    PassionFlower,
    /// `Pantone 18-3812 TCX Cadet` — `#6A6378`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3812 TCX Cadet"))]
    Cadet,
    /// `Pantone 18-3817 TCX Heron` — `#62617E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3817 TCX Heron"))]
    Heron,
    /// `Pantone 18-3820 TCX Twilight Purple` — `#66648B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3820 TCX Twilight Purple"))]
    TwilightPurple,
    /// `Pantone 18-3828 TCX Corsican Blue` — `#646093`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3828 TCX Corsican Blue"))]
    CorsicanBlue,
    /// `Pantone 18-3833 TCX Dusted Peri` — `#696BA0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3833 TCX Dusted Peri"))]
    DustedPeri,
    /// `Pantone 18-3834 TCX Veronica` — `#6D6695`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3834 TCX Veronica"))]
    Veronica,
    /// `Pantone 18-3838 TCX Ultra Violet` — `#5F4B8B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3838 TCX Ultra Violet"))]
    UltraViolet,
    /// `Pantone 18-3839 TCX Purple Corallite` — `#5A4E8F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3839 TCX Purple Corallite"))]
    PurpleCorallite,
    /// `Pantone 18-3840 TCX Purple Opulence` — `#60569A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3840 TCX Purple Opulence"))]
    PurpleOpulence,
    /// `Pantone 18-3905 TCX Excalibur` — `#676168`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3905 TCX Excalibur"))]
    Excalibur,
    /// `Pantone 18-3907 TCX Tornado` — `#5E5B60`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3907 TCX Tornado"))]
    Tornado,
    /// `Pantone 18-3908 TCX Volcanic Glass` — `#615C60`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3908 TCX Volcanic Glass"))]
    VolcanicGlass,
    /// `Pantone 18-3910 TCX Folkstone Gray` — `#626879`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3910 TCX Folkstone Gray"))]
    FolkstoneGray,
    /// `Pantone 18-3912 TCX Grisaille` — `#585E6F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3912 TCX Grisaille"))]
    Grisaille,
    /// `Pantone 18-3916 TCX Flint Stone` — `#677283`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3916 TCX Flint Stone"))]
    FlintStone,
    /// `Pantone 18-3917 TCX Gray Blue` — `#4D587A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3917 TCX Gray Blue"))]
    GrayBlue,
    /// `Pantone 18-3918 TCX China Blue` — `#546477`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3918 TCX China Blue"))]
    ChinaBlue,
    /// `Pantone 18-3920 TCX Coastal Fjord` — `#505D7E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3920 TCX Coastal Fjord"))]
    CoastalFjord,
    /// `Pantone 18-3921 TCX Bijou Blue` — `#4E5E7F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3921 TCX Bijou Blue"))]
    BijouBlue,
    /// `Pantone 18-3922 TCX Coronet Blue` — `#59728E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3922 TCX Coronet Blue"))]
    CoronetBlue,
    /// `Pantone 18-3927 TCX Velvet Morning` — `#60688D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3927 TCX Velvet Morning"))]
    VelvetMorning,
    /// `Pantone 18-3928 TCX Dutch Blue` — `#4A638D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3928 TCX Dutch Blue"))]
    DutchBlue,
    /// `Pantone 18-3929 TCX Blue Horizon` — `#4E6482`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3929 TCX Blue Horizon"))]
    BlueHorizon,
    /// `Pantone 18-3930 TCX Bleached Denim` — `#646F9B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3930 TCX Bleached Denim"))]
    BleachedDenim,
    /// `Pantone 18-3932 TCX Marlin` — `#515B87`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3932 TCX Marlin"))]
    Marlin,
    /// `Pantone 18-3933 TCX Blue Granite` — `#717388`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3933 TCX Blue Granite"))]
    BlueGranite,
    /// `Pantone 18-3935 TCX Wedgewood` — `#6479B3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3935 TCX Wedgewood"))]
    Wedgewood,
    /// `Pantone 18-3937 TCX Blue Yonder` — `#5A77A8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3937 TCX Blue Yonder"))]
    BlueYonder,
    /// `Pantone 18-3943 TCX Blue Iris` — `#5A5B9F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3943 TCX Blue Iris"))]
    BlueIris,
    /// `Pantone 18-3944 TCX Violet Storm` — `#5C619D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3944 TCX Violet Storm"))]
    VioletStorm,
    /// `Pantone 18-3945 TCX Amparo Blue` — `#4960A8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3945 TCX Amparo Blue"))]
    AmparoBlue,
    /// `Pantone 18-3946 TCX Baja Blue` — `#5F6DB0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3946 TCX Baja Blue"))]
    BajaBlue,
    /// `Pantone 18-3949 TCX Dazzling Blue` — `#3850A0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3949 TCX Dazzling Blue"))]
    DazzlingBlue,
    /// `Pantone 18-3950 TCX Iris Bloom` — `#5B609E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3950 TCX Iris Bloom"))]
    IrisBloom,
    /// `Pantone 18-3963 TCX Spectrum Blue` — `#3D3C7C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-3963 TCX Spectrum Blue"))]
    SpectrumBlue,
    /// `Pantone 18-4005 TCX Steel Gray` — `#726F70`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4005 TCX Steel Gray"))]
    SteelGray,
    /// `Pantone 18-4006 TCX Quiet Shade` — `#66676D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4006 TCX Quiet Shade"))]
    QuietShade,
    /// `Pantone 18-4011 TCX Goblin Blue` — `#5F7278`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4011 TCX Goblin Blue"))]
    GoblinBlue,
    /// `Pantone 18-4016 TCX December Sky` — `#767275`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4016 TCX December Sky"))]
    DecemberSky,
    /// `Pantone 18-4018 TCX Real Teal` — `#405D73`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4018 TCX Real Teal"))]
    RealTeal,
    /// `Pantone 18-4020 TCX Captains Blue` — `#557088`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4020 TCX Captains Blue"))]
    CaptainsBlue,
    /// `Pantone 18-4023 TCX Blue Ashes` — `#3B5F78`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4023 TCX Blue Ashes"))]
    BlueAshes,
    /// `Pantone 18-4025 TCX Copen Blue` — `#516B84`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4025 TCX Copen Blue"))]
    CopenBlue,
    /// `Pantone 18-4026 TCX Stellar` — `#46647E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4026 TCX Stellar"))]
    Stellar,
    /// `Pantone 18-4027 TCX Moonlight Blue` — `#506886`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4027 TCX Moonlight Blue"))]
    MoonlightBlue,
    /// `Pantone 18-4028 TCX Bering Sea` — `#4B5B6E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4028 TCX Bering Sea"))]
    BeringSea,
    /// `Pantone 18-4029 TCX Federal Blue` — `#43628B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4029 TCX Federal Blue"))]
    FederalBlue,
    /// `Pantone 18-4032 TCX Deep Water` — `#266691`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4032 TCX Deep Water"))]
    DeepWater,
    /// `Pantone 18-4034 TCX Vallarta Blue` — `#30658E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4034 TCX Vallarta Blue"))]
    VallartaBlue,
    /// `Pantone 18-4036 TCX Parisian Blue` — `#4F7CA4`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4036 TCX Parisian Blue"))]
    ParisianBlue,
    /// `Pantone 18-4039 TCX Regatta` — `#487AB7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4039 TCX Regatta"))]
    Regatta,
    /// `Pantone 18-4041 TCX Star Sapphire` — `#386192`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4041 TCX Star Sapphire"))]
    StarSapphire,
    /// `Pantone 18-4043 TCX Palace Blue` — `#346CB0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4043 TCX Palace Blue"))]
    PalaceBlue,
    /// `Pantone 18-4045 TCX Daphne` — `#0F5F9A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4045 TCX Daphne"))]
    Daphne,
    /// `Pantone 18-4048 TCX Nebulas Blue` — `#2D62A3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4048 TCX Nebulas Blue"))]
    NebulasBlue,
    /// `Pantone 18-4051 TCX Strong Blue` — `#1F5DA0`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4051 TCX Strong Blue"))]
    StrongBlue,
    /// `Pantone 18-4105 TCX Moon Mist` — `#80817D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4105 TCX Moon Mist"))]
    MoonMist,
    /// `Pantone 18-4140 TCX French Blue` — `#0072B5`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4140 TCX French Blue"))]
    FrenchBlue,
    /// `Pantone 18-4141 TCX Campanula` — `#3272AF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4141 TCX Campanula"))]
    Campanula,
    /// `Pantone 18-4148 TCX Victoria Blue` — `#08589D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4148 TCX Victoria Blue"))]
    VictoriaBlue,
    /// `Pantone 18-4214 TCX Stormy Weather` — `#58646D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4214 TCX Stormy Weather"))]
    StormyWeather,
    /// `Pantone 18-4215 TCX Blue Mirage` — `#5C6D7C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4215 TCX Blue Mirage"))]
    BlueMirage,
    /// `Pantone 18-4217 TCX Bluestone` — `#577284`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4217 TCX Bluestone"))]
    Bluestone,
    /// `Pantone 18-4220 TCX Provincial Blue` — `#5C798E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4220 TCX Provincial Blue"))]
    ProvincialBlue,
    /// `Pantone 18-4222 TCX Bluesteel` — `#35637C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4222 TCX Bluesteel"))]
    Bluesteel,
    /// `Pantone 18-4225 TCX Saxony Blue` — `#1F6680`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4225 TCX Saxony Blue"))]
    SaxonyBlue,
    /// `Pantone 18-4231 TCX Blue Sapphire` — `#09577B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4231 TCX Blue Sapphire"))]
    BlueSapphire,
    /// `Pantone 18-4232 TCX Faience` — `#2A6A8B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4232 TCX Faience"))]
    Faience,
    /// `Pantone 18-4244 TCX Directoire Blue` — `#0061A3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4244 TCX Directoire Blue"))]
    DirectoireBlue,
    /// `Pantone 18-4247 TCX Brilliant Blue` — `#0075B3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4247 TCX Brilliant Blue"))]
    BrilliantBlue,
    /// `Pantone 18-4250 TCX Indigo Bunting` — `#006CA9`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4250 TCX Indigo Bunting"))]
    IndigoBunting,
    /// `Pantone 18-4252 TCX Blue Aster` — `#0077B3`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4252 TCX Blue Aster"))]
    BlueAster,
    /// `Pantone 18-4320 TCX Aegean Blue` — `#4E6E81`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4320 TCX Aegean Blue"))]
    AegeanBlue,
    /// `Pantone 18-4330 TCX Swedish Blue` — `#007EB1`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4330 TCX Swedish Blue"))]
    SwedishBlue,
    /// `Pantone 18-4334 TCX Mediterranean Blue` — `#1478A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4334 TCX Mediterranean Blue"))]
    MediterraneanBlue,
    /// `Pantone 18-4417 TCX Tapestry` — `#436573`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4417 TCX Tapestry"))]
    Tapestry,
    /// `Pantone 18-4430 TCX Fjord Blue` — `#007290`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4430 TCX Fjord Blue"))]
    FjordBlue,
    /// `Pantone 18-4432 TCX Turkish Tile` — `#00698B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4432 TCX Turkish Tile"))]
    TurkishTile,
    /// `Pantone 18-4434 TCX Mykonos Blue` — `#005780`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4434 TCX Mykonos Blue"))]
    MykonosBlue,
    /// `Pantone 18-4440 TCX Cloisonne` — `#0075AF`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4440 TCX Cloisonne"))]
    Cloisonne,
    /// `Pantone 18-4510 TCX Trooper` — `#697A7E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4510 TCX Trooper"))]
    Trooper,
    /// `Pantone 18-4522 TCX Colonial Blue` — `#2D6471`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4522 TCX Colonial Blue"))]
    ColonialBlue,
    /// `Pantone 18-4525 TCX Caribbean Sea` — `#00819D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4525 TCX Caribbean Sea"))]
    CaribbeanSea,
    /// `Pantone 18-4528 TCX Mosaic Blue` — `#00758F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4528 TCX Mosaic Blue"))]
    MosaicBlue,
    /// `Pantone 18-4530 TCX Celestial` — `#006380`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4530 TCX Celestial"))]
    Celestial,
    /// `Pantone 18-4535 TCX Blue Jewel` — `#007BAA`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4535 TCX Blue Jewel"))]
    BlueJewel,
    /// `Pantone 18-4537 TCX Methyl Blue` — `#0074A8`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4537 TCX Methyl Blue"))]
    MethylBlue,
    /// `Pantone 18-4538 TCX Hawaiian Surf` — `#0078A7`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4538 TCX Hawaiian Surf"))]
    HawaiianSurf,
    /// `Pantone 18-4612 TCX North Atlantic` — `#536D70`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4612 TCX North Atlantic"))]
    NorthAtlantic,
    /// `Pantone 18-4630 TCX Tahitian Tide` — `#006B7E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4630 TCX Tahitian Tide"))]
    TahitianTide,
    /// `Pantone 18-4711 TCX Stormy Sea` — `#6E8082`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4711 TCX Stormy Sea"))]
    StormySea,
    /// `Pantone 18-4718 TCX Hydro` — `#426972`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4718 TCX Hydro"))]
    Hydro,
    /// `Pantone 18-4726 TCX Biscay Bay` — `#097988`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4726 TCX Biscay Bay"))]
    BiscayBay,
    /// `Pantone 18-4728 TCX Harbor Blue` — `#00656E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4728 TCX Harbor Blue"))]
    HarborBlue,
    /// `Pantone 18-4733 TCX Enamel Blue` — `#007A8E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4733 TCX Enamel Blue"))]
    EnamelBlue,
    /// `Pantone 18-4735 TCX Tile Blue` — `#008491`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4735 TCX Tile Blue"))]
    TileBlue,
    /// `Pantone 18-4834 TCX Deep Lake` — `#00656B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4834 TCX Deep Lake"))]
    DeepLake,
    /// `Pantone 18-4930 TCX Tropical Green` — `#008786`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4930 TCX Tropical Green"))]
    TropicalGreen,
    /// `Pantone 18-4936 TCX Fanfare` — `#006D70`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-4936 TCX Fanfare"))]
    Fanfare,
    /// `Pantone 18-5020 TCX Parasailing` — `#00736C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5020 TCX Parasailing"))]
    Parasailing,
    /// `Pantone 18-5025 TCX Quetzal Green` — `#006865`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5025 TCX Quetzal Green"))]
    QuetzalGreen,
    /// `Pantone 18-5102 TCX Brushed Nickel` — `#73706F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5102 TCX Brushed Nickel"))]
    BrushedNickel,
    /// `Pantone 18-5105 TCX Sedona Sage` — `#686D6C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5105 TCX Sedona Sage"))]
    SedonaSage,
    /// `Pantone 18-5112 TCX Sea Pine` — `#4C6969`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5112 TCX Sea Pine"))]
    SeaPine,
    /// `Pantone 18-5115 TCX North Sea` — `#316C6B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5115 TCX North Sea"))]
    NorthSea,
    /// `Pantone 18-5121 TCX Bayou` — `#20706F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5121 TCX Bayou"))]
    Bayou,
    /// `Pantone 18-5128 TCX Blue Grass` — `#007C7A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5128 TCX Blue Grass"))]
    BlueGrass,
    /// `Pantone 18-5203 TCX Pewter` — `#666564`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5203 TCX Pewter"))]
    Pewter,
    /// `Pantone 18-5204 TCX Granite Gray` — `#615E5F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5204 TCX Granite Gray"))]
    GraniteGray,
    /// `Pantone 18-5210 TCX Eiffel Tower` — `#5C5658`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5210 TCX Eiffel Tower"))]
    EiffelTower,
    /// `Pantone 18-5308 TCX Blue Spruce` — `#486B67`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5308 TCX Blue Spruce"))]
    BlueSpruce,
    /// `Pantone 18-5315 TCX Bayberry` — `#255958`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5315 TCX Bayberry"))]
    Bayberry,
    /// `Pantone 18-5322 TCX Alpine Green` — `#005F56`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5322 TCX Alpine Green"))]
    AlpineGreen,
    /// `Pantone 18-5338 TCX Ultramarine Green` — `#006B54`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5338 TCX Ultramarine Green"))]
    UltramarineGreen,
    /// `Pantone 18-5410 TCX Silver Pine` — `#4E6866`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5410 TCX Silver Pine"))]
    SilverPine,
    /// `Pantone 18-5418 TCX Antique Green` — `#29675C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5418 TCX Antique Green"))]
    AntiqueGreen,
    /// `Pantone 18-5424 TCX Cadmium Green` — `#00675B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5424 TCX Cadmium Green"))]
    CadmiumGreen,
    /// `Pantone 18-5606 TCX Balsam Green` — `#576664`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5606 TCX Balsam Green"))]
    BalsamGreen,
    /// `Pantone 18-5610 TCX Brittany Blue` — `#4C7E86`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5610 TCX Brittany Blue"))]
    BrittanyBlue,
    /// `Pantone 18-5611 TCX Dark Forest` — `#556962`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5611 TCX Dark Forest"))]
    DarkForest,
    /// `Pantone 18-5612 TCX Sagebrush Green` — `#567572`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5612 TCX Sagebrush Green"))]
    SagebrushGreen,
    /// `Pantone 18-5616 TCX Posy Green` — `#325B51`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5616 TCX Posy Green"))]
    PosyGreen,
    /// `Pantone 18-5618 TCX Deep Jungle` — `#36716F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5618 TCX Deep Jungle"))]
    DeepJungle,
    /// `Pantone 18-5619 TCX Tidepool` — `#0A6F69`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5619 TCX Tidepool"))]
    Tidepool,
    /// `Pantone 18-5620 TCX Ivy` — `#226C63`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5620 TCX Ivy"))]
    Ivy,
    /// `Pantone 18-5621 TCX Fir` — `#3A725F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5621 TCX Fir"))]
    Fir,
    /// `Pantone 18-5622 TCX Frosty Spruce` — `#578270`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5622 TCX Frosty Spruce"))]
    FrostySpruce,
    /// `Pantone 18-5624 TCX Shady Glade` — `#006E5B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5624 TCX Shady Glade"))]
    ShadyGlade,
    /// `Pantone 18-5633 TCX Bosphorus` — `#007558`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5633 TCX Bosphorus"))]
    Bosphorus,
    /// `Pantone 18-5642 TCX Golf Green` — `#008763`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5642 TCX Golf Green"))]
    GolfGreen,
    /// `Pantone 18-5718 TCX Smoke Pine` — `#3E6257`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5718 TCX Smoke Pine"))]
    SmokePine,
    /// `Pantone 18-5725 TCX Galapagos Green` — `#29685F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5725 TCX Galapagos Green"))]
    GalapagosGreen,
    /// `Pantone 18-5806 TCX Agave Green` — `#6B7169`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5806 TCX Agave Green"))]
    AgaveGreen,
    /// `Pantone 18-5841 TCX Pepper Green` — `#007D60`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5841 TCX Pepper Green"))]
    PepperGreen,
    /// `Pantone 18-5845 TCX Lush Meadow` — `#006E51`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5845 TCX Lush Meadow"))]
    LushMeadow,
    /// `Pantone 18-5913 TCX Garden Topiary` — `#3E524B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-5913 TCX Garden Topiary"))]
    GardenTopiary,
    /// `Pantone 18-6011 TCX Duck Green` — `#53665C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-6011 TCX Duck Green"))]
    DuckGreen,
    /// `Pantone 18-6018 TCX Foliage Green` — `#3E6F58`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-6018 TCX Foliage Green"))]
    FoliageGreen,
    /// `Pantone 18-6022 TCX Leprechaun` — `#378661`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-6022 TCX Leprechaun"))]
    Leprechaun,
    /// `Pantone 18-6024 TCX Amazon` — `#1F7349`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-6024 TCX Amazon"))]
    Amazon,
    /// `Pantone 18-6030 TCX Jolly Green` — `#007844`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-6030 TCX Jolly Green"))]
    JollyGreen,
    /// `Pantone 18-6114 TCX Myrtle` — `#4F6B58`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-6114 TCX Myrtle"))]
    Myrtle,
    /// `Pantone 18-6216 TCX Comfrey` — `#5B7961`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-6216 TCX Comfrey"))]
    Comfrey,
    /// `Pantone 18-6320 TCX Fairway` — `#477050`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-6320 TCX Fairway"))]
    Fairway,
    /// `Pantone 18-6330 TCX Juniper` — `#3D7245`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 18-6330 TCX Juniper"))]
    Juniper,
    /// `Pantone 19-0000 TCX Raven` — `#413E3D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0000 TCX Raven"))]
    Raven,
    /// `Pantone 19-0201 TCX Asphalt` — `#434447`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0201 TCX Asphalt"))]
    Asphalt,
    /// `Pantone 19-0203 TCX Gray Pinstripe` — `#49494D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0203 TCX Gray Pinstripe"))]
    GrayPinstripe,
    /// `Pantone 19-0230 TCX Garden Green` — `#495E35`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0230 TCX Garden Green"))]
    GardenGreen,
    /// `Pantone 19-0303 TCX Jet Black` — `#2D2C2F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0303 TCX Jet Black"))]
    JetBlack,
    /// `Pantone 19-0307 TCX Climbing Ivy` — `#444940`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0307 TCX Climbing Ivy"))]
    ClimbingIvy,
    /// `Pantone 19-0309 TCX Thyme` — `#50574C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0309 TCX Thyme"))]
    Thyme,
    /// `Pantone 19-0312 TCX Beetle` — `#55584C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0312 TCX Beetle"))]
    Beetle,
    /// `Pantone 19-0315 TCX Black Forest` — `#414F3C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0315 TCX Black Forest"))]
    BlackForest,
    /// `Pantone 19-0323 TCX Chive` — `#4A5335`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0323 TCX Chive"))]
    Chive,
    /// `Pantone 19-0403 TCX Sea Turtle` — `#5E5749`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0403 TCX Sea Turtle"))]
    SeaTurtle,
    /// `Pantone 19-0405 TCX Beluga` — `#4A4843`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0405 TCX Beluga"))]
    Beluga,
    /// `Pantone 19-0413 TCX Deep Depths` — `#46483C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0413 TCX Deep Depths"))]
    DeepDepths,
    /// `Pantone 19-0414 TCX Forest Night` — `#434237`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0414 TCX Forest Night"))]
    ForestNight,
    /// `Pantone 19-0415 TCX Duffel Bag` — `#394034`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0415 TCX Duffel Bag"))]
    DuffelBag,
    /// `Pantone 19-0417 TCX Kombu Green` — `#3A4032`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0417 TCX Kombu Green"))]
    KombuGreen,
    /// `Pantone 19-0419 TCX Rifle Green` — `#414832`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0419 TCX Rifle Green"))]
    RifleGreen,
    /// `Pantone 19-0506 TCX Black Ink` — `#44413C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0506 TCX Black Ink"))]
    BlackInk,
    /// `Pantone 19-0508 TCX Peat` — `#3B3A36`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0508 TCX Peat"))]
    Peat,
    /// `Pantone 19-0509 TCX Rosin` — `#36362D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0509 TCX Rosin"))]
    Rosin,
    /// `Pantone 19-0510 TCX Kalamata` — `#5F5B4C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0510 TCX Kalamata"))]
    Kalamata,
    /// `Pantone 19-0511 TCX Grape Leaf` — `#545144`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0511 TCX Grape Leaf"))]
    GrapeLeaf,
    /// `Pantone 19-0512 TCX Ivy Green` — `#585442`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0512 TCX Ivy Green"))]
    IvyGreen,
    /// `Pantone 19-0515 TCX Olive Night` — `#535040`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0515 TCX Olive Night"))]
    OliveNight,
    /// `Pantone 19-0516 TCX Dark Olive` — `#574D35`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0516 TCX Dark Olive"))]
    DarkOlive,
    /// `Pantone 19-0608 TCX Black Olive` — `#48413B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0608 TCX Black Olive"))]
    BlackOlive,
    /// `Pantone 19-0614 TCX Wren` — `#4A4139`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0614 TCX Wren"))]
    Wren,
    /// `Pantone 19-0617 TCX Teak` — `#655341`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0617 TCX Teak"))]
    Teak,
    /// `Pantone 19-0618 TCX Beech` — `#5B4F3B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0618 TCX Beech"))]
    Beech,
    /// `Pantone 19-0622 TCX Military Olive` — `#63563B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0622 TCX Military Olive"))]
    MilitaryOlive,
    /// `Pantone 19-0712 TCX Demitasse` — `#40342B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0712 TCX Demitasse"))]
    Demitasse,
    /// `Pantone 19-0808 TCX Morel` — `#685C53`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0808 TCX Morel"))]
    Morel,
    /// `Pantone 19-0809 TCX Chocolate Chip` — `#685A4E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0809 TCX Chocolate Chip"))]
    ChocolateChip,
    /// `Pantone 19-0810 TCX Major Brown` — `#5B5149`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0810 TCX Major Brown"))]
    MajorBrown,
    /// `Pantone 19-0812 TCX Turkish Coffee` — `#483F39`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0812 TCX Turkish Coffee"))]
    TurkishCoffee,
    /// `Pantone 19-0814 TCX Slate Black` — `#4B3D33`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0814 TCX Slate Black"))]
    SlateBlack,
    /// `Pantone 19-0815 TCX Desert Palm` — `#5A4632`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0815 TCX Desert Palm"))]
    DesertPalm,
    /// `Pantone 19-0820 TCX Canteen` — `#5E5347`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0820 TCX Canteen"))]
    Canteen,
    /// `Pantone 19-0822 TCX Tarmac` — `#5A5348`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0822 TCX Tarmac"))]
    Tarmac,
    /// `Pantone 19-0823 TCX Crocodile` — `#5D5348`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0823 TCX Crocodile"))]
    Crocodile,
    /// `Pantone 19-0840 TCX Delicioso` — `#3F352F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0840 TCX Delicioso"))]
    Delicioso,
    /// `Pantone 19-0910 TCX Mulch` — `#433937`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0910 TCX Mulch"))]
    Mulch,
    /// `Pantone 19-0912 TCX Chocolate Brown` — `#4E403B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0912 TCX Chocolate Brown"))]
    ChocolateBrown,
    /// `Pantone 19-0915 TCX Coffee Bean` — `#40312F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0915 TCX Coffee Bean"))]
    CoffeeBean,
    /// `Pantone 19-0916 TCX Rain Drum` — `#5F4C40`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-0916 TCX Rain Drum"))]
    RainDrum,
    /// `Pantone 19-1012 TCX French Roast` — `#58423F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1012 TCX French Roast"))]
    FrenchRoast,
    /// `Pantone 19-1015 TCX Bracken` — `#4F3F3B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1015 TCX Bracken"))]
    Bracken,
    /// `Pantone 19-1016 TCX Java` — `#433331`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1016 TCX Java"))]
    Java,
    /// `Pantone 19-1018 TCX Ganache` — `#34292A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1018 TCX Ganache"))]
    Ganache,
    /// `Pantone 19-1020 TCX Dark Earth` — `#5C4939`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1020 TCX Dark Earth"))]
    DarkEarth,
    /// `Pantone 19-1034 TCX Breen` — `#795D34`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1034 TCX Breen"))]
    Breen,
    /// `Pantone 19-1101 TCX After Dark` — `#3C3535`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1101 TCX After Dark"))]
    AfterDark,
    /// `Pantone 19-1102 TCX Licorice` — `#3A3536`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1102 TCX Licorice"))]
    Licorice,
    /// `Pantone 19-1103 TCX Espresso` — `#363031`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1103 TCX Espresso"))]
    Espresso,
    /// `Pantone 19-1106 TCX Mole` — `#392D2B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1106 TCX Mole"))]
    Mole,
    /// `Pantone 19-1109 TCX Chocolate Torte` — `#382E2D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1109 TCX Chocolate Torte"))]
    ChocolateTorte,
    /// `Pantone 19-1110 TCX Chocolate Plum` — `#3C2D2E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1110 TCX Chocolate Plum"))]
    ChocolatePlum,
    /// `Pantone 19-1111 TCX Black Coffee` — `#3B302F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1111 TCX Black Coffee"))]
    BlackCoffee,
    /// `Pantone 19-1116 TCX Carafe` — `#5D473A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1116 TCX Carafe"))]
    Carafe,
    /// `Pantone 19-1118 TCX Chestnut` — `#584039`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1118 TCX Chestnut"))]
    Chestnut,
    /// `Pantone 19-1121 TCX Pinecone` — `#61473B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1121 TCX Pinecone"))]
    Pinecone,
    /// `Pantone 19-1213 TCX Shopping Bag` — `#5A4743`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1213 TCX Shopping Bag"))]
    ShoppingBag,
    /// `Pantone 19-1214 TCX Chocolate Lab` — `#5C3E35`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1214 TCX Chocolate Lab"))]
    ChocolateLab,
    /// `Pantone 19-1215 TCX Shaved Chocolate` — `#543B35`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1215 TCX Shaved Chocolate"))]
    ShavedChocolate,
    /// `Pantone 19-1217 TCX Mustang` — `#684B40`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1217 TCX Mustang"))]
    Mustang,
    /// `Pantone 19-1218 TCX Potting Soil` — `#54392D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1218 TCX Potting Soil"))]
    PottingSoil,
    /// `Pantone 19-1220 TCX Cappuccino` — `#633F33`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1220 TCX Cappuccino"))]
    Cappuccino,
    /// `Pantone 19-1224 TCX Fondue Fudge` — `#5D4236`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1224 TCX Fondue Fudge"))]
    FondueFudge,
    /// `Pantone 19-1228 TCX Root Beer` — `#714A41`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1228 TCX Root Beer"))]
    RootBeer,
    /// `Pantone 19-1230 TCX Friar Brown` — `#6E493A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1230 TCX Friar Brown"))]
    FriarBrown,
    /// `Pantone 19-1233 TCX Tiramisu` — `#634235`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1233 TCX Tiramisu"))]
    Tiramisu,
    /// `Pantone 19-1234 TCX Rocky Road` — `#5A3E36`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1234 TCX Rocky Road"))]
    RockyRoad,
    /// `Pantone 19-1235 TCX Brunette` — `#664238`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1235 TCX Brunette"))]
    Brunette,
    /// `Pantone 19-1241 TCX Tortoise Shell` — `#754734`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1241 TCX Tortoise Shell"))]
    TortoiseShell,
    /// `Pantone 19-1245 TCX Arabian Spice` — `#884332`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1245 TCX Arabian Spice"))]
    ArabianSpice,
    /// `Pantone 19-1250 TCX Picante` — `#8D3F2D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1250 TCX Picante"))]
    Picante,
    /// `Pantone 19-1314 TCX Seal Brown` — `#493B39`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1314 TCX Seal Brown"))]
    SealBrown,
    /// `Pantone 19-1317 TCX Bitter Chocolate` — `#503130`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1317 TCX Bitter Chocolate"))]
    BitterChocolate,
    /// `Pantone 19-1320 TCX Sable` — `#6E403C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1320 TCX Sable"))]
    Sable,
    /// `Pantone 19-1321 TCX Rum Raisin` — `#583432`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1321 TCX Rum Raisin"))]
    RumRaisin,
    /// `Pantone 19-1322 TCX Brown Stone` — `#593C39`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1322 TCX Brown Stone"))]
    BrownStone,
    /// `Pantone 19-1325 TCX Hot Chocolate` — `#683B39`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1325 TCX Hot Chocolate"))]
    HotChocolate,
    /// `Pantone 19-1327 TCX Andorra` — `#603535`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1327 TCX Andorra"))]
    Andorra,
    /// `Pantone 19-1331 TCX Madder Brown` — `#6A3331`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1331 TCX Madder Brown"))]
    MadderBrown,
    /// `Pantone 19-1333 TCX Sequoia` — `#804839`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1333 TCX Sequoia"))]
    Sequoia,
    /// `Pantone 19-1334 TCX Henna` — `#7C423C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1334 TCX Henna"))]
    Henna,
    /// `Pantone 19-1337 TCX Fired Brick` — `#6A2E2A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1337 TCX Fired Brick"))]
    FiredBrick,
    /// `Pantone 19-1338 TCX Russet Brown` — `#743332`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1338 TCX Russet Brown"))]
    RussetBrown,
    /// `Pantone 19-1419 TCX Chicory Coffee` — `#4A342E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1419 TCX Chicory Coffee"))]
    ChicoryCoffee,
    /// `Pantone 19-1420 TCX Deep Mahogany` — `#553B39`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1420 TCX Deep Mahogany"))]
    DeepMahogany,
    /// `Pantone 19-1429 TCX Smoked Paprika` — `#6E362C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1429 TCX Smoked Paprika"))]
    SmokedPaprika,
    /// `Pantone 19-1430 TCX Mink` — `#734B42`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1430 TCX Mink"))]
    Mink,
    /// `Pantone 19-1431 TCX Fudgesickle` — `#63403A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1431 TCX Fudgesickle"))]
    Fudgesickle,
    /// `Pantone 19-1432 TCX Chocolate Fondant` — `#56352D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1432 TCX Chocolate Fondant"))]
    ChocolateFondant,
    /// `Pantone 19-1435 TCX Cherry Mahogany` — `#66352B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1435 TCX Cherry Mahogany"))]
    CherryMahogany,
    /// `Pantone 19-1436 TCX Cinnamon` — `#6B4139`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1436 TCX Cinnamon"))]
    Cinnamon,
    /// `Pantone 19-1518 TCX Puce` — `#503938`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1518 TCX Puce"))]
    Puce,
    /// `Pantone 19-1521 TCX Red Mahogany` — `#60373D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1521 TCX Red Mahogany"))]
    RedMahogany,
    /// `Pantone 19-1522 TCX Zinfandel` — `#5C2935`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1522 TCX Zinfandel"))]
    Zinfandel,
    /// `Pantone 19-1524 TCX Oxblood Red` — `#70393F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1524 TCX Oxblood Red"))]
    OxbloodRed,
    /// `Pantone 19-1525 TCX Port` — `#663336`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1525 TCX Port"))]
    Port,
    /// `Pantone 19-1526 TCX Chocolate Truffle` — `#612E35`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1526 TCX Chocolate Truffle"))]
    ChocolateTruffle,
    /// `Pantone 19-1528 TCX Windsor Wine` — `#582B36`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1528 TCX Windsor Wine"))]
    WindsorWine,
    /// `Pantone 19-1530 TCX Burnt Russet` — `#7E3940`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1530 TCX Burnt Russet"))]
    BurntRusset,
    /// `Pantone 19-1531 TCX Sun Dried Tomato` — `#752329`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1531 TCX Sun Dried Tomato"))]
    SunDriedTomato,
    /// `Pantone 19-1532 TCX Rosewood` — `#813639`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1532 TCX Rosewood"))]
    Rosewood,
    /// `Pantone 19-1533 TCX Cowhide` — `#884344`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1533 TCX Cowhide"))]
    Cowhide,
    /// `Pantone 19-1534 TCX Merlot` — `#72262C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1534 TCX Merlot"))]
    Merlot,
    /// `Pantone 19-1535 TCX Syrah` — `#6A282C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1535 TCX Syrah"))]
    Syrah,
    /// `Pantone 19-1536 TCX Red Pear` — `#7B3539`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1536 TCX Red Pear"))]
    RedPear,
    /// `Pantone 19-1540 TCX Burnt Henna` — `#7E392F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1540 TCX Burnt Henna"))]
    BurntHenna,
    /// `Pantone 19-1543 TCX Brick Red` — `#8C373E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1543 TCX Brick Red"))]
    BrickRed,
    /// `Pantone 19-1555 TCX Red Dahlia` — `#7D2027`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1555 TCX Red Dahlia"))]
    RedDahlia,
    /// `Pantone 19-1557 TCX Chili Pepper` — `#9B1B30`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1557 TCX Chili Pepper"))]
    ChiliPepper,
    /// `Pantone 19-1559 TCX Scarlet Sage` — `#9D202F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1559 TCX Scarlet Sage"))]
    ScarletSage,
    /// `Pantone 19-1606 TCX Raisin` — `#524144`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1606 TCX Raisin"))]
    Raisin,
    /// `Pantone 19-1608 TCX Prune Purple` — `#5C3A4D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1608 TCX Prune Purple"))]
    PrunePurple,
    /// `Pantone 19-1617 TCX Burgundy` — `#64313E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1617 TCX Burgundy"))]
    Burgundy,
    /// `Pantone 19-1619 TCX Fudge` — `#493338`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1619 TCX Fudge"))]
    Fudge,
    /// `Pantone 19-1620 TCX Huckleberry` — `#5B4349`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1620 TCX Huckleberry"))]
    Huckleberry,
    /// `Pantone 19-1621 TCX Catawba Grape` — `#5D3C43`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1621 TCX Catawba Grape"))]
    CatawbaGrape,
    /// `Pantone 19-1623 TCX Vineyard Wine` — `#58363D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1623 TCX Vineyard Wine"))]
    VineyardWine,
    /// `Pantone 19-1624 TCX Sassafras` — `#54353B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1624 TCX Sassafras"))]
    Sassafras,
    /// `Pantone 19-1625 TCX Decadent Chocolate` — `#513235`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1625 TCX Decadent Chocolate"))]
    DecadentChocolate,
    /// `Pantone 19-1627 TCX Port Royale` — `#502B33`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1627 TCX Port Royale"))]
    PortRoyale,
    /// `Pantone 19-1629 TCX Ruby Wine` — `#77333B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1629 TCX Ruby Wine"))]
    RubyWine,
    /// `Pantone 19-1650 TCX Biking Red` — `#77212E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1650 TCX Biking Red"))]
    BikingRed,
    /// `Pantone 19-1652 TCX Rhubarb` — `#77202F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1652 TCX Rhubarb"))]
    Rhubarb,
    /// `Pantone 19-1655 TCX Garnet` — `#953640`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1655 TCX Garnet"))]
    Garnet,
    /// `Pantone 19-1656 TCX Rio Red` — `#8A2232`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1656 TCX Rio Red"))]
    RioRed,
    /// `Pantone 19-1662 TCX Samba` — `#A2242F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1662 TCX Samba"))]
    Samba,
    /// `Pantone 19-1663 TCX Ribbon Red` — `#B92636`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1663 TCX Ribbon Red"))]
    RibbonRed,
    /// `Pantone 19-1664 TCX True Red` — `#BF1932`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1664 TCX True Red"))]
    TrueRed,
    /// `Pantone 19-1716 TCX Mauve Wine` — `#5B3644`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1716 TCX Mauve Wine"))]
    MauveWine,
    /// `Pantone 19-1718 TCX Fig` — `#532D3B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1718 TCX Fig"))]
    Fig,
    /// `Pantone 19-1724 TCX Cabernet` — `#64242E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1724 TCX Cabernet"))]
    Cabernet,
    /// `Pantone 19-1725 TCX Tawny Port` — `#5C2C35`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1725 TCX Tawny Port"))]
    TawnyPort,
    /// `Pantone 19-1726 TCX Cordovan` — `#702F3B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1726 TCX Cordovan"))]
    Cordovan,
    /// `Pantone 19-1757 TCX Barbados Cherry` — `#AA0A27`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1757 TCX Barbados Cherry"))]
    BarbadosCherry,
    /// `Pantone 19-1758 TCX Haute Red` — `#A11729`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1758 TCX Haute Red"))]
    HauteRed,
    /// `Pantone 19-1759 TCX American Beauty` — `#A73340`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1759 TCX American Beauty"))]
    AmericanBeauty,
    /// `Pantone 19-1760 TCX Scarlet` — `#BC2B3D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1760 TCX Scarlet"))]
    Scarlet,
    /// `Pantone 19-1761 TCX Tango Red` — `#AC0E2E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1761 TCX Tango Red"))]
    TangoRed,
    /// `Pantone 19-1762 TCX Crimson` — `#AE0E36`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1762 TCX Crimson"))]
    Crimson,
    /// `Pantone 19-1763 TCX Racing Red` — `#BD162C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1763 TCX Racing Red"))]
    RacingRed,
    /// `Pantone 19-1764 TCX Lipstick Red` — `#B31A38`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1764 TCX Lipstick Red"))]
    LipstickRed,
    /// `Pantone 19-1840 TCX Deep Claret` — `#973443`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1840 TCX Deep Claret"))]
    DeepClaret,
    /// `Pantone 19-1850 TCX Red Bud` — `#962D49`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1850 TCX Red Bud"))]
    RedBud,
    /// `Pantone 19-1860 TCX Persian Red` — `#A21441`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1860 TCX Persian Red"))]
    PersianRed,
    /// `Pantone 19-1862 TCX Jester Red` — `#9E1030`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1862 TCX Jester Red"))]
    JesterRed,
    /// `Pantone 19-1863 TCX Scooter` — `#941E32`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1863 TCX Scooter"))]
    Scooter,
    /// `Pantone 19-1930 TCX Pomegranate` — `#6C2831`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1930 TCX Pomegranate"))]
    Pomegranate,
    /// `Pantone 19-1934 TCX Tibetan Red` — `#782A39`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1934 TCX Tibetan Red"))]
    TibetanRed,
    /// `Pantone 19-1940 TCX Rumba Red` — `#7C2439`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1940 TCX Rumba Red"))]
    RumbaRed,
    /// `Pantone 19-1955 TCX Cerise` — `#A41247`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-1955 TCX Cerise"))]
    Cerise,
    /// `Pantone 19-2009 TCX Hortensia` — `#553B50`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2009 TCX Hortensia"))]
    Hortensia,
    /// `Pantone 19-2014 TCX Prune` — `#603749`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2014 TCX Prune"))]
    Prune,
    /// `Pantone 19-2024 TCX Rhododendron` — `#722B3F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2024 TCX Rhododendron"))]
    Rhododendron,
    /// `Pantone 19-2025 TCX Red Plum` — `#7C2946`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2025 TCX Red Plum"))]
    RedPlum,
    /// `Pantone 19-2030 TCX Beet Red` — `#7A1F3D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2030 TCX Beet Red"))]
    BeetRed,
    /// `Pantone 19-2033 TCX Anemone` — `#842C48`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2033 TCX Anemone"))]
    Anemone,
    /// `Pantone 19-2039 TCX Granita` — `#A52350`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2039 TCX Granita"))]
    Granita,
    /// `Pantone 19-2041 TCX Cherries Jubilee` — `#A22452`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2041 TCX Cherries Jubilee"))]
    CherriesJubilee,
    /// `Pantone 19-2045 TCX Vivacious` — `#A32857`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2045 TCX Vivacious"))]
    Vivacious,
    /// `Pantone 19-2047 TCX Sangria` — `#982551`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2047 TCX Sangria"))]
    Sangria,
    /// `Pantone 19-2118 TCX Winetasting` — `#492A34`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2118 TCX Winetasting"))]
    Winetasting,
    /// `Pantone 19-2311 TCX Eggplant` — `#613F4C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2311 TCX Eggplant"))]
    Eggplant,
    /// `Pantone 19-2312 TCX Crushed Violets` — `#643A4C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2312 TCX Crushed Violets"))]
    CrushedViolets,
    /// `Pantone 19-2315 TCX Grape Wine` — `#5A2F43`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2315 TCX Grape Wine"))]
    GrapeWine,
    /// `Pantone 19-2410 TCX Amaranth` — `#6F3C56`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2410 TCX Amaranth"))]
    Amaranth,
    /// `Pantone 19-2420 TCX Pickled Beet` — `#4D233D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2420 TCX Pickled Beet"))]
    PickledBeet,
    /// `Pantone 19-2428 TCX Magenta Purple` — `#6B264B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2428 TCX Magenta Purple"))]
    MagentaPurple,
    /// `Pantone 19-2429 TCX Plum Caspia` — `#61224A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2429 TCX Plum Caspia"))]
    PlumCaspia,
    /// `Pantone 19-2430 TCX Purple Potion` — `#692746`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2430 TCX Purple Potion"))]
    PurplePotion,
    /// `Pantone 19-2431 TCX Boysenberry` — `#85325C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2431 TCX Boysenberry"))]
    Boysenberry,
    /// `Pantone 19-2432 TCX Raspberry Radiance` — `#802A50`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2432 TCX Raspberry Radiance"))]
    RaspberryRadiance,
    /// `Pantone 19-2434 TCX Festival Fuchsia` — `#9E2C6A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2434 TCX Festival Fuchsia"))]
    FestivalFuchsia,
    /// `Pantone 19-2514 TCX Italian Plum` — `#533146`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2514 TCX Italian Plum"))]
    ItalianPlum,
    /// `Pantone 19-2520 TCX Potent Purple` — `#462639`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2520 TCX Potent Purple"))]
    PotentPurple,
    /// `Pantone 19-2524 TCX Dark Purple` — `#582147`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2524 TCX Dark Purple"))]
    DarkPurple,
    /// `Pantone 19-2620 TCX Winter Bloom` — `#47243B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2620 TCX Winter Bloom"))]
    WinterBloom,
    /// `Pantone 19-2630 TCX Wild Aster` — `#92316F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2630 TCX Wild Aster"))]
    WildAster,
    /// `Pantone 19-2814 TCX Wineberry` — `#5A395B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2814 TCX Wineberry"))]
    Wineberry,
    /// `Pantone 19-2816 TCX Blackberry Wine` — `#4D3246`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2816 TCX Blackberry Wine"))]
    BlackberryWine,
    /// `Pantone 19-2820 TCX Phlox` — `#692D5D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2820 TCX Phlox"))]
    Phlox,
    /// `Pantone 19-2924 TCX Hollyhock` — `#823270`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-2924 TCX Hollyhock"))]
    Hollyhock,
    /// `Pantone 19-3022 TCX Gloxinia` — `#622E5A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3022 TCX Gloxinia"))]
    Gloxinia,
    /// `Pantone 19-3138 TCX Byzantium` — `#853B7B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3138 TCX Byzantium"))]
    Byzantium,
    /// `Pantone 19-3215 TCX Indigo` — `#4C3957`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3215 TCX Indigo"))]
    Indigo,
    /// `Pantone 19-3217 TCX Shadow Purple` — `#4E334E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3217 TCX Shadow Purple"))]
    ShadowPurple,
    /// `Pantone 19-3218 TCX Plum Purple` — `#51304E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3218 TCX Plum Purple"))]
    PlumPurple,
    /// `Pantone 19-3220 TCX Plum` — `#5A315D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3220 TCX Plum"))]
    Plum,
    /// `Pantone 19-3223 TCX Purple Passion` — `#683D62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3223 TCX Purple Passion"))]
    PurplePassion,
    /// `Pantone 19-3230 TCX Grape Juice` — `#682961`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3230 TCX Grape Juice"))]
    GrapeJuice,
    /// `Pantone 19-3316 TCX Plum Perfect` — `#473442`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3316 TCX Plum Perfect"))]
    PlumPerfect,
    /// `Pantone 19-3323 TCX Deep Purple` — `#50314C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3323 TCX Deep Purple"))]
    DeepPurple,
    /// `Pantone 19-3325 TCX Wood Violet` — `#75406A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3325 TCX Wood Violet"))]
    WoodViolet,
    /// `Pantone 19-3336 TCX Sparkling Grape` — `#773376`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3336 TCX Sparkling Grape"))]
    SparklingGrape,
    /// `Pantone 19-3424 TCX Sunset Purple` — `#6F456E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3424 TCX Sunset Purple"))]
    SunsetPurple,
    /// `Pantone 19-3425 TCX Spiced Plum` — `#6D4773`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3425 TCX Spiced Plum"))]
    SpicedPlum,
    /// `Pantone 19-3438 TCX Bright Violet` — `#784384`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3438 TCX Bright Violet"))]
    BrightViolet,
    /// `Pantone 19-3514 TCX Majesty` — `#593761`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3514 TCX Majesty"))]
    Majesty,
    /// `Pantone 19-3518 TCX Grape Royale` — `#4F2D54`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3518 TCX Grape Royale"))]
    GrapeRoyale,
    /// `Pantone 19-3519 TCX Purple Pennant` — `#432C47`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3519 TCX Purple Pennant"))]
    PurplePennant,
    /// `Pantone 19-3520 TCX Blackberry Cordial` — `#3F2A47`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3520 TCX Blackberry Cordial"))]
    BlackberryCordial,
    /// `Pantone 19-3526 TCX Meadow Violet` — `#764F82`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3526 TCX Meadow Violet"))]
    MeadowViolet,
    /// `Pantone 19-3528 TCX Imperial Purple` — `#542C5D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3528 TCX Imperial Purple"))]
    ImperialPurple,
    /// `Pantone 19-3536 TCX Amaranth Purple` — `#6A397B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3536 TCX Amaranth Purple"))]
    AmaranthPurple,
    /// `Pantone 19-3540 TCX Purple Magic` — `#663271`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3540 TCX Purple Magic"))]
    PurpleMagic,
    /// `Pantone 19-3542 TCX Pansy` — `#653D7C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3542 TCX Pansy"))]
    Pansy,
    /// `Pantone 19-3617 TCX Mysterioso` — `#46394B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3617 TCX Mysterioso"))]
    Mysterioso,
    /// `Pantone 19-3619 TCX Sweet Grape` — `#4B3B4F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3619 TCX Sweet Grape"))]
    SweetGrape,
    /// `Pantone 19-3620 TCX Purple Reign` — `#56456B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3620 TCX Purple Reign"))]
    PurpleReign,
    /// `Pantone 19-3622 TCX Loganberry` — `#5A4769`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3622 TCX Loganberry"))]
    Loganberry,
    /// `Pantone 19-3628 TCX Acai` — `#46295A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3628 TCX Acai"))]
    Acai,
    /// `Pantone 19-3632 TCX Petunia` — `#4F3466`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3632 TCX Petunia"))]
    Petunia,
    /// `Pantone 19-3638 TCX Tillandsia Purple` — `#563474`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3638 TCX Tillandsia Purple"))]
    TillandsiaPurple,
    /// `Pantone 19-3640 TCX Crown Jewel` — `#482D54`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3640 TCX Crown Jewel"))]
    CrownJewel,
    /// `Pantone 19-3642 TCX Royal Purple` — `#603F83`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3642 TCX Royal Purple"))]
    RoyalPurple,
    /// `Pantone 19-3712 TCX Nightshade` — `#433748`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3712 TCX Nightshade"))]
    Nightshade,
    /// `Pantone 19-3713 TCX Deep Well` — `#2C2A33`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3713 TCX Deep Well"))]
    DeepWell,
    /// `Pantone 19-3714 TCX Navy Cosmos` — `#503B53`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3714 TCX Navy Cosmos"))]
    NavyCosmos,
    /// `Pantone 19-3716 TCX Purple Plumeria` — `#473854`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3716 TCX Purple Plumeria"))]
    PurplePlumeria,
    /// `Pantone 19-3720 TCX Gothic Grape` — `#473951`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3720 TCX Gothic Grape"))]
    GothicGrape,
    /// `Pantone 19-3722 TCX Mulberry Purple` — `#493C62`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3722 TCX Mulberry Purple"))]
    MulberryPurple,
    /// `Pantone 19-3725 TCX Purple Velvet` — `#41354D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3725 TCX Purple Velvet"))]
    PurpleVelvet,
    /// `Pantone 19-3728 TCX Grape` — `#433455`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3728 TCX Grape"))]
    Grape,
    /// `Pantone 19-3730 TCX Gentian Violet` — `#544275`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3730 TCX Gentian Violet"))]
    GentianViolet,
    /// `Pantone 19-3731 TCX Parachute Purple` — `#392852`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3731 TCX Parachute Purple"))]
    ParachutePurple,
    /// `Pantone 19-3737 TCX Heliotrope` — `#4F3872`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3737 TCX Heliotrope"))]
    Heliotrope,
    /// `Pantone 19-3748 TCX Prism Violet` — `#53357D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3748 TCX Prism Violet"))]
    PrismViolet,
    /// `Pantone 19-3750 TCX Violet Indigo` — `#3E285C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3750 TCX Violet Indigo"))]
    VioletIndigo,
    /// `Pantone 19-3803 TCX Plum Kitten` — `#625B5C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3803 TCX Plum Kitten"))]
    PlumKitten,
    /// `Pantone 19-3810 TCX Eclipse` — `#343148`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3810 TCX Eclipse"))]
    Eclipse,
    /// `Pantone 19-3815 TCX Evening Blue` — `#2A293E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3815 TCX Evening Blue"))]
    EveningBlue,
    /// `Pantone 19-3830 TCX Astral Aura` — `#363151`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3830 TCX Astral Aura"))]
    AstralAura,
    /// `Pantone 19-3831 TCX Maritime Blue` — `#27293D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3831 TCX Maritime Blue"))]
    MaritimeBlue,
    /// `Pantone 19-3832 TCX Navy Blue` — `#403F6F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3832 TCX Navy Blue"))]
    NavyBlue,
    /// `Pantone 19-3839 TCX Blue Ribbon` — `#3A395F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3839 TCX Blue Ribbon"))]
    BlueRibbon,
    /// `Pantone 19-3842 TCX Deep Wisteria` — `#443F6F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3842 TCX Deep Wisteria"))]
    DeepWisteria,
    /// `Pantone 19-3847 TCX Deep Blue` — `#44377D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3847 TCX Deep Blue"))]
    DeepBlue,
    /// `Pantone 19-3850 TCX Liberty` — `#4D448A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3850 TCX Liberty"))]
    Liberty,
    /// `Pantone 19-3864 TCX Mazarine Blue` — `#273C76`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3864 TCX Mazarine Blue"))]
    MazarineBlue,
    /// `Pantone 19-3900 TCX Pavement` — `#524D50`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3900 TCX Pavement"))]
    Pavement,
    /// `Pantone 19-3901 TCX Magnet` — `#4D4B4F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3901 TCX Magnet"))]
    Magnet,
    /// `Pantone 19-3902 TCX Obsidian` — `#3A363B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3902 TCX Obsidian"))]
    Obsidian,
    /// `Pantone 19-3903 TCX Shale` — `#4A3F41`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3903 TCX Shale"))]
    Shale,
    /// `Pantone 19-3905 TCX Rabbit` — `#5F575C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3905 TCX Rabbit"))]
    Rabbit,
    /// `Pantone 19-3906 TCX Dark Shadow` — `#4A4B4D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3906 TCX Dark Shadow"))]
    DarkShadow,
    /// `Pantone 19-3907 TCX Forged Iron` — `#48464A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3907 TCX Forged Iron"))]
    ForgedIron,
    /// `Pantone 19-3908 TCX Nine Iron` — `#46434A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3908 TCX Nine Iron"))]
    NineIron,
    /// `Pantone 19-3909 TCX Black Bean` — `#2E272A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3909 TCX Black Bean"))]
    BlackBean,
    /// `Pantone 19-3910 TCX Iron Gate` — `#4E5055`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3910 TCX Iron Gate"))]
    IronGate,
    /// `Pantone 19-3911 TCX Black Beauty` — `#26262A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3911 TCX Black Beauty"))]
    BlackBeauty,
    /// `Pantone 19-3915 TCX Graystone` — `#4D495B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3915 TCX Graystone"))]
    Graystone,
    /// `Pantone 19-3917 TCX Blackened Pearl` — `#4D4B50`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3917 TCX Blackened Pearl"))]
    BlackenedPearl,
    /// `Pantone 19-3918 TCX Periscope` — `#46444C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3918 TCX Periscope"))]
    Periscope,
    /// `Pantone 19-3919 TCX Nightshadow Blue` — `#4E5368`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3919 TCX Nightshadow Blue"))]
    NightshadowBlue,
    /// `Pantone 19-3920 TCX Peacoat` — `#2B2E43`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3920 TCX Peacoat"))]
    Peacoat,
    /// `Pantone 19-3921 TCX Black Iris` — `#2B3042`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3921 TCX Black Iris"))]
    BlackIris,
    /// `Pantone 19-3922 TCX Sky Captain` — `#262934`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3922 TCX Sky Captain"))]
    SkyCaptain,
    /// `Pantone 19-3923 TCX Navy Blazer` — `#282D3C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3923 TCX Navy Blazer"))]
    NavyBlazer,
    /// `Pantone 19-3924 TCX Night Sky` — `#2A2A35`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3924 TCX Night Sky"))]
    NightSky,
    /// `Pantone 19-3925 TCX Patriot Blue` — `#363756`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3925 TCX Patriot Blue"))]
    PatriotBlue,
    /// `Pantone 19-3926 TCX Crown Blue` — `#464B65`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3926 TCX Crown Blue"))]
    CrownBlue,
    /// `Pantone 19-3927 TCX Graphite` — `#3B3B48`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3927 TCX Graphite"))]
    Graphite,
    /// `Pantone 19-3928 TCX Blue Indigo` — `#49516D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3928 TCX Blue Indigo"))]
    BlueIndigo,
    /// `Pantone 19-3929 TCX Vintage Indigo` — `#4A556B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3929 TCX Vintage Indigo"))]
    VintageIndigo,
    /// `Pantone 19-3930 TCX Odyssey Gray` — `#434452`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3930 TCX Odyssey Gray"))]
    OdysseyGray,
    /// `Pantone 19-3933 TCX Medieval Blue` — `#29304E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3933 TCX Medieval Blue"))]
    MedievalBlue,
    /// `Pantone 19-3935 TCX Deep Cobalt` — `#404466`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3935 TCX Deep Cobalt"))]
    DeepCobalt,
    /// `Pantone 19-3936 TCX Skipper Blue` — `#484A72`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3936 TCX Skipper Blue"))]
    SkipperBlue,
    /// `Pantone 19-3938 TCX Twilight Blue` — `#313D64`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3938 TCX Twilight Blue"))]
    TwilightBlue,
    /// `Pantone 19-3939 TCX Blueprint` — `#2D3359`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3939 TCX Blueprint"))]
    Blueprint,
    /// `Pantone 19-3940 TCX Blue Depths` — `#263056`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3940 TCX Blue Depths"))]
    BlueDepths,
    /// `Pantone 19-3947 TCX Orient Blue` — `#47457A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3947 TCX Orient Blue"))]
    OrientBlue,
    /// `Pantone 19-3950 TCX Deep Ultramarine` — `#384883`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3950 TCX Deep Ultramarine"))]
    DeepUltramarine,
    /// `Pantone 19-3951 TCX Clematis Blue` — `#363B7C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3951 TCX Clematis Blue"))]
    ClematisBlue,
    /// `Pantone 19-3952 TCX Surf The Web` — `#203C7F`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3952 TCX Surf The Web"))]
    SurfTheWeb,
    /// `Pantone 19-3953 TCX Sodalite Blue` — `#253668`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3953 TCX Sodalite Blue"))]
    SodaliteBlue,
    /// `Pantone 19-3955 TCX Royal Blue` — `#3D428B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3955 TCX Royal Blue"))]
    RoyalBlue,
    /// `Pantone 19-3964 TCX Monaco Blue` — `#274374`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-3964 TCX Monaco Blue"))]
    MonacoBlue,
    /// `Pantone 19-4003 TCX Black Onyx` — `#2B272B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4003 TCX Black Onyx"))]
    BlackOnyx,
    /// `Pantone 19-4004 TCX Tap Shoe` — `#2A2B2D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4004 TCX Tap Shoe"))]
    TapShoe,
    /// `Pantone 19-4005 TCX Stretch Limo` — `#2B2C30`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4005 TCX Stretch Limo"))]
    StretchLimo,
    /// `Pantone 19-4006 TCX Caviar` — `#292A2D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4006 TCX Caviar"))]
    Caviar,
    /// `Pantone 19-4007 TCX Anthracite` — `#28282D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4007 TCX Anthracite"))]
    Anthracite,
    /// `Pantone 19-4008 TCX Meteorite` — `#2B2929`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4008 TCX Meteorite"))]
    Meteorite,
    /// `Pantone 19-4009 TCX Outer Space` — `#2F3441`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4009 TCX Outer Space"))]
    OuterSpace,
    /// `Pantone 19-4010 TCX Total Eclipse` — `#2C313D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4010 TCX Total Eclipse"))]
    TotalEclipse,
    /// `Pantone 19-4011 TCX Salute` — `#282B34`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4011 TCX Salute"))]
    Salute,
    /// `Pantone 19-4012 TCX Carbon` — `#272F38`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4012 TCX Carbon"))]
    Carbon,
    /// `Pantone 19-4013 TCX Dark Navy` — `#232F36`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4013 TCX Dark Navy"))]
    DarkNavy,
    /// `Pantone 19-4014 TCX Ombre Blue` — `#434854`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4014 TCX Ombre Blue"))]
    OmbreBlue,
    /// `Pantone 19-4015 TCX Blue Graphite` — `#323137`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4015 TCX Blue Graphite"))]
    BlueGraphite,
    /// `Pantone 19-4019 TCX India Ink` — `#3C3F4A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4019 TCX India Ink"))]
    IndiaInk,
    /// `Pantone 19-4020 TCX Dark Sapphire` — `#262B37`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4020 TCX Dark Sapphire"))]
    DarkSapphire,
    /// `Pantone 19-4021 TCX Blueberry` — `#2C333E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4021 TCX Blueberry"))]
    Blueberry,
    /// `Pantone 19-4022 TCX Parisian Night` — `#323441`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4022 TCX Parisian Night"))]
    ParisianNight,
    /// `Pantone 19-4023 TCX Blue Nights` — `#363B48`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4023 TCX Blue Nights"))]
    BlueNights,
    /// `Pantone 19-4024 TCX Dress Blues` — `#2A3244`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4024 TCX Dress Blues"))]
    DressBlues,
    /// `Pantone 19-4025 TCX Mood Indigo` — `#353A4C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4025 TCX Mood Indigo"))]
    MoodIndigo,
    /// `Pantone 19-4026 TCX Ensign Blue` — `#384C67`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4026 TCX Ensign Blue"))]
    EnsignBlue,
    /// `Pantone 19-4027 TCX Estate Blue` — `#233658`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4027 TCX Estate Blue"))]
    EstateBlue,
    /// `Pantone 19-4028 TCX Insignia Blue` — `#2F3E55`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4028 TCX Insignia Blue"))]
    InsigniaBlue,
    /// `Pantone 19-4029 TCX Navy Peony` — `#223A5E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4029 TCX Navy Peony"))]
    NavyPeony,
    /// `Pantone 19-4030 TCX True Navy` — `#3F5277`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4030 TCX True Navy"))]
    TrueNavy,
    /// `Pantone 19-4031 TCX Sargasso Sea` — `#35435A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4031 TCX Sargasso Sea"))]
    SargassoSea,
    /// `Pantone 19-4033 TCX Poseidon` — `#123955`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4033 TCX Poseidon"))]
    Poseidon,
    /// `Pantone 19-4034 TCX Sailor Blue` — `#0E3A53`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4034 TCX Sailor Blue"))]
    SailorBlue,
    /// `Pantone 19-4035 TCX Dark Blue` — `#305679`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4035 TCX Dark Blue"))]
    DarkBlue,
    /// `Pantone 19-4037 TCX Bright Cobalt` — `#385D8D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4037 TCX Bright Cobalt"))]
    BrightCobalt,
    /// `Pantone 19-4038 TCX Gibraltar Sea` — `#123850`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4038 TCX Gibraltar Sea"))]
    GibraltarSea,
    /// `Pantone 19-4039 TCX Delft` — `#3D5E8C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4039 TCX Delft"))]
    Delft,
    /// `Pantone 19-4044 TCX Limoges` — `#243F6C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4044 TCX Limoges"))]
    Limoges,
    /// `Pantone 19-4045 TCX Lapis Blue` — `#004B8D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4045 TCX Lapis Blue"))]
    LapisBlue,
    /// `Pantone 19-4048 TCX Baleine Blue` — `#155187`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4048 TCX Baleine Blue"))]
    BaleineBlue,
    /// `Pantone 19-4049 TCX Snorkel Blue` — `#034F84`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4049 TCX Snorkel Blue"))]
    SnorkelBlue,
    /// `Pantone 19-4050 TCX Nautical Blue` — `#1A5091`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4050 TCX Nautical Blue"))]
    NauticalBlue,
    /// `Pantone 19-4052 TCX Classic Blue` — `#0F4C81`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4052 TCX Classic Blue"))]
    ClassicBlue,
    /// `Pantone 19-4053 TCX Turkish Sea` — `#195190`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4053 TCX Turkish Sea"))]
    TurkishSea,
    /// `Pantone 19-4055 TCX Galaxy Blue` — `#2A4B7C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4055 TCX Galaxy Blue"))]
    GalaxyBlue,
    /// `Pantone 19-4056 TCX Olympian Blue` — `#1A4C8B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4056 TCX Olympian Blue"))]
    OlympianBlue,
    /// `Pantone 19-4057 TCX True Blue` — `#1E4477`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4057 TCX True Blue"))]
    TrueBlue,
    /// `Pantone 19-4104 TCX Ebony` — `#41424A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4104 TCX Ebony"))]
    Ebony,
    /// `Pantone 19-4110 TCX Midnight Navy` — `#34414E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4110 TCX Midnight Navy"))]
    MidnightNavy,
    /// `Pantone 19-4118 TCX Dark Denim` — `#35465E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4118 TCX Dark Denim"))]
    DarkDenim,
    /// `Pantone 19-4120 TCX Blue Opal` — `#0F3B57`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4120 TCX Blue Opal"))]
    BlueOpal,
    /// `Pantone 19-4121 TCX Blue Wing Teal` — `#2C4053`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4121 TCX Blue Wing Teal"))]
    BlueWingTeal,
    /// `Pantone 19-4122 TCX Moonlit Ocean` — `#293B4D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4122 TCX Moonlit Ocean"))]
    MoonlitOcean,
    /// `Pantone 19-4125 TCX Majolica Blue` — `#274357`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4125 TCX Majolica Blue"))]
    MajolicaBlue,
    /// `Pantone 19-4126 TCX Deep Dive` — `#29495C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4126 TCX Deep Dive"))]
    DeepDive,
    /// `Pantone 19-4127 TCX Midnight` — `#325B74`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4127 TCX Midnight"))]
    Midnight,
    /// `Pantone 19-4150 TCX Princess Blue` — `#00539C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4150 TCX Princess Blue"))]
    PrincessBlue,
    /// `Pantone 19-4151 TCX Skydiver` — `#00589B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4151 TCX Skydiver"))]
    Skydiver,
    /// `Pantone 19-4203 TCX Moonless Night` — `#2F2D30`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4203 TCX Moonless Night"))]
    MoonlessNight,
    /// `Pantone 19-4205 TCX Phantom` — `#39373B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4205 TCX Phantom"))]
    Phantom,
    /// `Pantone 19-4215 TCX Turbulence` — `#4E545B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4215 TCX Turbulence"))]
    Turbulence,
    /// `Pantone 19-4218 TCX Vulcan` — `#2D3036`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4218 TCX Vulcan"))]
    Vulcan,
    /// `Pantone 19-4220 TCX Dark Slate` — `#46515A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4220 TCX Dark Slate"))]
    DarkSlate,
    /// `Pantone 19-4227 TCX Indian Teal` — `#3C586B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4227 TCX Indian Teal"))]
    IndianTeal,
    /// `Pantone 19-4229 TCX Orion Blue` — `#3E4F5C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4229 TCX Orion Blue"))]
    OrionBlue,
    /// `Pantone 19-4234 TCX Ink Blue` — `#0B5369`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4234 TCX Ink Blue"))]
    InkBlue,
    /// `Pantone 19-4241 TCX Moroccan Blue` — `#0F4E67`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4241 TCX Moroccan Blue"))]
    MoroccanBlue,
    /// `Pantone 19-4245 TCX Imperial Blue` — `#005A92`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4245 TCX Imperial Blue"))]
    ImperialBlue,
    /// `Pantone 19-4305 TCX Pirate Black` — `#363838`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4305 TCX Pirate Black"))]
    PirateBlack,
    /// `Pantone 19-4316 TCX Stargazer` — `#39505C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4316 TCX Stargazer"))]
    Stargazer,
    /// `Pantone 19-4318 TCX Mallard Blue` — `#3A5C6E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4318 TCX Mallard Blue"))]
    MallardBlue,
    /// `Pantone 19-4324 TCX Legion Blue` — `#1F495B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4324 TCX Legion Blue"))]
    LegionBlue,
    /// `Pantone 19-4326 TCX Reflecting Pond` — `#203E4A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4326 TCX Reflecting Pond"))]
    ReflectingPond,
    /// `Pantone 19-4329 TCX Corsair` — `#18576C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4329 TCX Corsair"))]
    Corsair,
    /// `Pantone 19-4340 TCX Lyons Blue` — `#005871`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4340 TCX Lyons Blue"))]
    LyonsBlue,
    /// `Pantone 19-4342 TCX Seaport` — `#005E7D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4342 TCX Seaport"))]
    Seaport,
    /// `Pantone 19-4517 TCX Mediterranea` — `#32575D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4517 TCX Mediterranea"))]
    Mediterranea,
    /// `Pantone 19-4524 TCX Shaded Spruce` — `#00585E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4524 TCX Shaded Spruce"))]
    ShadedSpruce,
    /// `Pantone 19-4526 TCX Blue Coral` — `#1B5366`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4526 TCX Blue Coral"))]
    BlueCoral,
    /// `Pantone 19-4535 TCX Ocean Depths` — `#006175`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4535 TCX Ocean Depths"))]
    OceanDepths,
    /// `Pantone 19-4536 TCX Crystal Teal` — `#00637C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4536 TCX Crystal Teal"))]
    CrystalTeal,
    /// `Pantone 19-4540 TCX Deep Lagoon` — `#005265`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4540 TCX Deep Lagoon"))]
    DeepLagoon,
    /// `Pantone 19-4726 TCX Atlantic Deep` — `#274E55`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4726 TCX Atlantic Deep"))]
    AtlanticDeep,
    /// `Pantone 19-4818 TCX Mallard Green` — `#405E5C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4818 TCX Mallard Green"))]
    MallardGreen,
    /// `Pantone 19-4820 TCX Balsam` — `#33565E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4820 TCX Balsam"))]
    Balsam,
    /// `Pantone 19-4826 TCX Dragonfly` — `#2A5C6A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4826 TCX Dragonfly"))]
    Dragonfly,
    /// `Pantone 19-4906 TCX Green Gables` — `#324241`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4906 TCX Green Gables"))]
    GreenGables,
    /// `Pantone 19-4914 TCX Deep Teal` — `#18454B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4914 TCX Deep Teal"))]
    DeepTeal,
    /// `Pantone 19-4916 TCX Pacific` — `#1F595C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4916 TCX Pacific"))]
    Pacific,
    /// `Pantone 19-4922 TCX Teal Green` — `#006361`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-4922 TCX Teal Green"))]
    TealGreen,
    /// `Pantone 19-5004 TCX Urban Chic` — `#464E4D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5004 TCX Urban Chic"))]
    UrbanChic,
    /// `Pantone 19-5030 TCX Sea Moss` — `#254445`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5030 TCX Sea Moss"))]
    SeaMoss,
    /// `Pantone 19-5212 TCX Darkest Spruce` — `#303D3C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5212 TCX Darkest Spruce"))]
    DarkestSpruce,
    /// `Pantone 19-5217 TCX Storm` — `#035453`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5217 TCX Storm"))]
    Storm,
    /// `Pantone 19-5220 TCX Botanical Garden` — `#12403C`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5220 TCX Botanical Garden"))]
    BotanicalGarden,
    /// `Pantone 19-5226 TCX Everglade` — `#005B5D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5226 TCX Everglade"))]
    Everglade,
    /// `Pantone 19-5230 TCX Forest Biome` — `#184A45`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5230 TCX Forest Biome"))]
    ForestBiome,
    /// `Pantone 19-5232 TCX Rain Forest` — `#15463E`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5232 TCX Rain Forest"))]
    RainForest,
    /// `Pantone 19-5320 TCX Ponderosa Pine` — `#203B3D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5320 TCX Ponderosa Pine"))]
    PonderosaPine,
    /// `Pantone 19-5350 TCX Scarab` — `#23312D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5350 TCX Scarab"))]
    Scarab,
    /// `Pantone 19-5406 TCX Pine Grove` — `#213631`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5406 TCX Pine Grove"))]
    PineGrove,
    /// `Pantone 19-5408 TCX Bistro Green` — `#395551`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5408 TCX Bistro Green"))]
    BistroGreen,
    /// `Pantone 19-5411 TCX Trekking Green` — `#355048`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5411 TCX Trekking Green"))]
    TrekkingGreen,
    /// `Pantone 19-5413 TCX Jasper` — `#335959`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5413 TCX Jasper"))]
    Jasper,
    /// `Pantone 19-5414 TCX June Bug` — `#264A48`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5414 TCX June Bug"))]
    JuneBug,
    /// `Pantone 19-5420 TCX Evergreen` — `#11574A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5420 TCX Evergreen"))]
    Evergreen,
    /// `Pantone 19-5511 TCX Hunter Green` — `#335749`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5511 TCX Hunter Green"))]
    HunterGreen,
    /// `Pantone 19-5513 TCX Dark Green` — `#314F40`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5513 TCX Dark Green"))]
    DarkGreen,
    /// `Pantone 19-5621 TCX Cilantro` — `#43544B`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5621 TCX Cilantro"))]
    Cilantro,
    /// `Pantone 19-5708 TCX Jet Set` — `#262C2A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5708 TCX Jet Set"))]
    JetSet,
    /// `Pantone 19-5914 TCX Jungle Green` — `#3C4E47`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5914 TCX Jungle Green"))]
    JungleGreen,
    /// `Pantone 19-5917 TCX Sycamore` — `#35463D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5917 TCX Sycamore"))]
    Sycamore,
    /// `Pantone 19-5918 TCX Mountain View` — `#2E3D30`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5918 TCX Mountain View"))]
    MountainView,
    /// `Pantone 19-5920 TCX Pineneedle` — `#334D41`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-5920 TCX Pineneedle"))]
    Pineneedle,
    /// `Pantone 19-6026 TCX Verdant Green` — `#12674A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-6026 TCX Verdant Green"))]
    VerdantGreen,
    /// `Pantone 19-6050 TCX Eden` — `#264E36`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-6050 TCX Eden"))]
    Eden,
    /// `Pantone 19-6110 TCX Deep Forest` — `#37413A`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-6110 TCX Deep Forest"))]
    DeepForest,
    /// `Pantone 19-6311 TCX Greener Pastures` — `#37503D`.
    #[cfg_attr(feature = "schemars", schemars(rename = "Pantone 19-6311 TCX Greener Pastures"))]
    GreenerPastures,
}

impl PantoneFhiNamedColor {
    /// All FHI colors, sorted by code.
    pub const ALL: &'static [Self] = &[
        Self::Egret,
        Self::VanillaIce,
        Self::AntiqueWhite,
        Self::SweetCream,
        Self::Papyrus,
        Self::Buttercream,
        Self::GlassGreen,
        Self::WaterLily,
        Self::SylvanGreen,
        Self::WinterWhite,
        Self::Afterglow,
        Self::LemonIcing,
        Self::BrightWhite,
        Self::SnowWhite,
        Self::PastelParchment,
        Self::Gardenia,
        Self::JetStream,
        Self::Pristine,
        Self::SugarSwizzle,
        Self::CoconutMilk,
        Self::EtherealGreen,
        Self::PearSorbet,
        Self::PastelYellow,
        Self::TransparentYellow,
        Self::WaxYellow,
        Self::Flan,
        Self::ElfinYellow,
        Self::YellowIris,
        Self::YellowPear,
        Self::WhisperWhite,
        Self::TenderYellow,
        Self::Ecru,
        Self::PearledIvory,
        Self::WhiteAlyssum,
        Self::BridalBlush,
        Self::SeaSalt,
        Self::AngelWing,
        Self::CreamPink,
        Self::PowderPuff,
        Self::Rosewater,
        Self::PetalPink,
        Self::Delicacy,
        Self::ShrinkingViolet,
        Self::BrilliantWhite,
        Self::CloudDancer,
        Self::StarWhite,
        Self::Marshmallow,
        Self::LilyWhite,
        Self::CannoliCream,
        Self::MysticBlue,
        Self::BitOfBlue,
        Self::BillowingSail,
        Self::BlancDeBlanc,
        Self::Tofu,
        Self::SummerShower,
        Self::Ice,
        Self::LightestSky,
        Self::HintOfMint,
        Self::WhiteSwan,
        Self::WhiteAsparagus,
        Self::BoneWhite,
        Self::MeadowMist,
        Self::CanaryGreen,
        Self::Ambrosia,
        Self::PistachioShell,
        Self::PatinaGreen,
        Self::WhitecapGray,
        Self::AsparagusGreen,
        Self::LimeCream,
        Self::SeafoamGreen,
        Self::WhiteJade,
        Self::Gleam,
        Self::Butterfly,
        Self::LightGray,
        Self::Hay,
        Self::MellowGreen,
        Self::DaiquiriGreen,
        Self::PaleLimeYellow,
        Self::YoungWheat,
        Self::Citron,
        Self::LuminaryGreen,
        Self::Charlock,
        Self::Eggnog,
        Self::Angora,
        Self::GreenEssence,
        Self::DustyYellow,
        Self::LemonGrass,
        Self::CanaryYellow,
        Self::Aurora,
        Self::BlazingYellow,
        Self::Celandine,
        Self::Seedpearl,
        Self::WhiteSmoke,
        Self::Macadamia,
        Self::Navajo,
        Self::LemonMeringue,
        Self::Vanilla,
        Self::AlmondOil,
        Self::Cornhusk,
        Self::DoubleCream,
        Self::AniseFlower,
        Self::PineappleSlice,
        Self::MellowYellow,
        Self::Lemonade,
        Self::FrenchVanilla,
        Self::Sunshine,
        Self::Sundress,
        Self::LemonDrop,
        Self::Goldfinch,
        Self::YellowCream,
        Self::Limelight,
        Self::SunnyLime,
        Self::LemonVerbena,
        Self::Buttercup,
        Self::Yarrow,
        Self::CloudCream,
        Self::Rutabaga,
        Self::SunKiss,
        Self::Dawn,
        Self::AlabasterGleam,
        Self::AutumnBlonde,
        Self::VanillaCustard,
        Self::ApricotGelato,
        Self::GoldenFleece,
        Self::PaleBanana,
        Self::Popcorn,
        Self::GoldenHaze,
        Self::LambsWool,
        Self::Nude,
        Self::TenderPeach,
        Self::Alesan,
        Self::PalePeach,
        Self::BleachedApricot,
        Self::GoldenStraw,
        Self::NovellePeach,
        Self::MotherOfPearl,
        Self::PastelRoseTan,
        Self::Linen,
        Self::VanillaCream,
        Self::ScallopShell,
        Self::PeachPuree,
        Self::SheerPink,
        Self::PeachDust,
        Self::Dew,
        Self::CremeDePeche,
        Self::SilverPeony,
        Self::PearlBlush,
        Self::SoftPink,
        Self::VeiledRose,
        Self::Pearl,
        Self::HeavenlyPink,
        Self::BlushingBride,
        Self::Tapioca,
        Self::PinkTint,
        Self::CrystalPink,
        Self::PinkDogwood,
        Self::CrystalRose,
        Self::MarysRose,
        Self::MauveMorn,
        Self::AlmostMauve,
        Self::Morganite,
        Self::MauveChalk,
        Self::LightLilac,
        Self::PrimrosePink,
        Self::CradlePink,
        Self::BarelyPink,
        Self::RoseWater,
        Self::AlmondMilk,
        Self::VaporousGray,
        Self::Bluewash,
        Self::SpaBlue,
        Self::BarelyBlue,
        Self::Skylight,
        Self::PastelBlue,
        Self::Clearwater,
        Self::StarlightBlue,
        Self::WhisperingBlue,
        Self::BlueBlush,
        Self::WanBlue,
        Self::ChalkBlue,
        Self::Icicle,
        Self::Turtledove,
        Self::Murmur,
        Self::MorningMist,
        Self::BlueGlass,
        Self::SoothingSea,
        Self::SproutGreen,
        Self::BlueFlower,
        Self::WhisperGreen,
        Self::OpalBlue,
        Self::AquaGlass,
        Self::MoonlightJade,
        Self::FairAqua,
        Self::BleachedAqua,
        Self::ClearlyAqua,
        Self::Glacier,
        Self::DustyAqua,
        Self::Bay,
        Self::HushedGreen,
        Self::ZephyrBlue,
        Self::Honeydew,
        Self::SilverGreen,
        Self::MilkyGreen,
        Self::FairestJade,
        Self::Frost,
        Self::PhantomGreen,
        Self::Moonbeam,
        Self::WhiteSand,
        Self::Dewkist,
        Self::Seacrest,
        Self::PastelGreen,
        Self::GreenAsh,
        Self::FogGreen,
        Self::TenderGreens,
        Self::Reed,
        Self::ParadiseGreen,
        Self::PistachioGreen,
        Self::LilyGreen,
        Self::ShadowLime,
        Self::LettuceGreen,
        Self::SapGreen,
        Self::LimaBean,
        Self::Oatmeal,
        Self::GrayMorn,
        Self::GreenGlow,
        Self::LimePopsicle,
        Self::FrozenDew,
        Self::PaleGreen,
        Self::LimeSherbet,
        Self::CeleryGreen,
        Self::SharpGreen,
        Self::WildLime,
        Self::LimePunch,
        Self::Fog,
        Self::AloeWash,
        Self::Moth,
        Self::ChinoGreen,
        Self::GardenGlade,
        Self::GoldenMist,
        Self::Endive,
        Self::Chardonnay,
        Self::Acacia,
        Self::GoldenKiwi,
        Self::Limeade,
        Self::Meadowlark,
        Self::GreenSheen,
        Self::SulphurSpring,
        Self::EveningPrimrose,
        Self::Putty,
        Self::SeaMist,
        Self::Custard,
        Self::Raffia,
        Self::CreamGold,
        Self::Maize,
        Self::Lemon,
        Self::PrimroseYellow,
        Self::LemonZest,
        Self::Dandelion,
        Self::SolarPower,
        Self::SummerMelon,
        Self::BananaCrepe,
        Self::Sunlight,
        Self::Snapdragon,
        Self::HabaneroGold,
        Self::AspenGold,
        Self::MinionYellow,
        Self::VibrantYellow,
        Self::LemonChrome,
        Self::Birch,
        Self::Sandshell,
        Self::Parchment,
        Self::ReedYellow,
        Self::Chamomile,
        Self::ItalianStraw,
        Self::Soybean,
        Self::Straw,
        Self::Cornsilk,
        Self::Flax,
        Self::GoldenCream,
        Self::SunsetGold,
        Self::BananaCream,
        Self::AmberYellow,
        Self::PaleMarigold,
        Self::JurassicGold,
        Self::Banana,
        Self::CremeBrulee,
        Self::OysterWhite,
        Self::BleachedSand,
        Self::Biscotti,
        Self::GraySand,
        Self::IvoryCream,
        Self::FrostedAlmond,
        Self::Appleblossom,
        Self::MellowBuff,
        Self::HoneyPeach,
        Self::Wheat,
        Self::AlmondCream,
        Self::DesertDust,
        Self::CreamBlush,
        Self::ApricotIce,
        Self::PrairieSunset,
        Self::CaramelCream,
        Self::PeachFuzz,
        Self::Buff,
        Self::Impala,
        Self::Creampuff,
        Self::ApricotCream,
        Self::Sunburst,
        Self::ApricotSherbet,
        Self::BrownRice,
        Self::SandDollar,
        Self::WhisperPink,
        Self::CreamTan,
        Self::Bisque,
        Self::Bellini,
        Self::PeachQuartz,
        Self::PeachyKeen,
        Self::BrazilianSand,
        Self::EnglishRose,
        Self::TropicalPeach,
        Self::PaleDogwood,
        Self::Shell,
        Self::CloudPink,
        Self::CreolePink,
        Self::ChintzRose,
        Self::SeashellPink,
        Self::PeachBlush,
        Self::ImpatiensPink,
        Self::PinkSalt,
        Self::GossamerPink,
        Self::RoseQuartz,
        Self::ChalkPink,
        Self::RoseShadow,
        Self::Potpourri,
        Self::StrawberryCream,
        Self::AlmondBlossom,
        Self::OrchidPink,
        Self::FairyTale,
        Self::PaleLilac,
        Self::ParfaitPink,
        Self::PinkMist,
        Self::PinkLady,
        Self::Ballerina,
        Self::BalletSlipper,
        Self::CherryBlossom,
        Self::LilacSnow,
        Self::OrchidIce,
        Self::CrystalGray,
        Self::OrchidTint,
        Self::LilacAsh,
        Self::GrayLilac,
        Self::OrchidHush,
        Self::LavenderFog,
        Self::HalogenBlue,
        Self::IllusionBlue,
        Self::Antarctica,
        Self::LilacHint,
        Self::NimbusCloud,
        Self::ArcticIce,
        Self::PleinAir,
        Self::Omphalodes,
        Self::OysterMushroom,
        Self::DawnBlue,
        Self::BalladBlue,
        Self::FoggyDew,
        Self::BabyBlue,
        Self::SilverBirch,
        Self::IceFlow,
        Self::MistyBlue,
        Self::BlueGlow,
        Self::CrystalBlue,
        Self::TanagerTurquoise,
        Self::PaleBlue,
        Self::Plume,
        Self::LimpetShell,
        Self::BlueLight,
        Self::BlueTint,
        Self::RainyDay,
        Self::PaleAqua,
        Self::IcyMorn,
        Self::PastelTurquoise,
        Self::ArubaBlue,
        Self::Yucca,
        Self::IcedAqua,
        Self::BeachGlass,
        Self::IceGreen,
        Self::Cabbage,
        Self::GossamerGreen,
        Self::BirdsEggGreen,
        Self::AlmostAqua,
        Self::Spray,
        Self::MistyJade,
        Self::BrookGreen,
        Self::CeladonTint,
        Self::GreenTint,
        Self::GreenLily,
        Self::Celadon,
        Self::MistGreen,
        Self::BokChoy,
        Self::SilverGray,
        Self::PumiceStone,
        Self::Overcast,
        Self::CastleWall,
        Self::CeladonGreen,
        Self::FoamGreen,
        Self::Margarita,
        Self::NileGreen,
        Self::ArcadianGreen,
        Self::Greengage,
        Self::SummerGreen,
        Self::TidalFoam,
        Self::Lint,
        Self::Seedling,
        Self::Nile,
        Self::OpalineGreen,
        Self::JadeLime,
        Self::BrightLimeGreen,
        Self::AcidLime,
        Self::Bog,
        Self::Beechnut,
        Self::GreenBanana,
        Self::BrightChartreuse,
        Self::TenderShoots,
        Self::LimeGreen,
        Self::GreenHaze,
        Self::DriedMoss,
        Self::ShadowGreen,
        Self::MutedLime,
        Self::Celery,
        Self::Cement,
        Self::Hemp,
        Self::Bamboo,
        Self::SuperLemon,
        Self::Sulphur,
        Self::EmpireYellow,
        Self::CyberYellow,
        Self::Pampas,
        Self::DuskyCitron,
        Self::MistedYellow,
        Self::YolkYellow,
        Self::BuffYellow,
        Self::Mimosa,
        Self::Daffodil,
        Self::SamoanSun,
        Self::Freesia,
        Self::Parsnip,
        Self::Jojoba,
        Self::SaharaSun,
        Self::Beeswax,
        Self::GoldenRod,
        Self::SpicyMustard,
        Self::Citrus,
        Self::SpectraYellow,
        Self::GildedBeige,
        Self::Gravel,
        Self::Cocoon,
        Self::Rattan,
        Self::Ochre,
        Self::NewWheat,
        Self::GoldenApricot,
        Self::Amber,
        Self::Marigold,
        Self::WarmApricot,
        Self::Kumquat,
        Self::Saffron,
        Self::Peyote,
        Self::OysterGray,
        Self::WoodAsh,
        Self::Boulder,
        Self::Pebble,
        Self::Marzipan,
        Self::AlmondBuff,
        Self::Beige,
        Self::WinterWheat,
        Self::ApricotIllusion,
        Self::Sheepskin,
        Self::DesertMist,
        Self::BuffOrange,
        Self::ApricotNectar,
        Self::SalmonBuff,
        Self::Pumpkin,
        Self::Zinnia,
        Self::IrishCream,
        Self::SmokeGray,
        Self::ShiftingSand,
        Self::Frappe,
        Self::ToastedAlmond,
        Self::Amberlight,
        Self::PeachParfait,
        Self::PeachNougat,
        Self::CoralSands,
        Self::BeachSand,
        Self::Peach,
        Self::PeachNectar,
        Self::ApricotWash,
        Self::PeachCobbler,
        Self::OrangeChiffon,
        Self::Mushroom,
        Self::RoseDust,
        Self::PeachWhip,
        Self::CameoRose,
        Self::EveningSand,
        Self::PaleBlush,
        Self::RoseCloud,
        Self::SpanishVilla,
        Self::Hazelnut,
        Self::DustyPink,
        Self::CoralPink,
        Self::Salmon,
        Self::PeachBud,
        Self::PeachMelba,
        Self::PeachPearl,
        Self::ApricotBlush,
        Self::RoseSmoke,
        Self::SilverPink,
        Self::PowderPink,
        Self::Blossom,
        Self::PeachesNCream,
        Self::QuartzPink,
        Self::SepiaRose,
        Self::Lotus,
        Self::Peachskin,
        Self::CoralBlush,
        Self::CandyPink,
        Self::PinkNectar,
        Self::CameoPink,
        Self::PrismPink,
        Self::LilacSachet,
        Self::SweetLilac,
        Self::FragrantLilac,
        Self::WinsomeOrchid,
        Self::PinkLavender,
        Self::PastelLavender,
        Self::OrchidBloom,
        Self::OrchidPetal,
        Self::HushedViolet,
        Self::Iris,
        Self::PastelLilac,
        Self::LilacMarble,
        Self::EveningHaze,
        Self::LavenderBlue,
        Self::Raindrops,
        Self::Thistle,
        Self::PurpleHeather,
        Self::ZenBlue,
        Self::XenonBlue,
        Self::WindChime,
        Self::GlacierGray,
        Self::GrayViolet,
        Self::MicroChip,
        Self::GrayDawn,
        Self::QuietGray,
        Self::Heather,
        Self::Skyway,
        Self::CashmereBlue,
        Self::BlueBell,
        Self::AiryBlue,
        Self::LunarRock,
        Self::HarborMist,
        Self::VaporBlue,
        Self::PearlBlue,
        Self::CelestialBlue,
        Self::PowderBlue,
        Self::CloudBlue,
        Self::WinterSky,
        Self::BlueTopaz,
        Self::CorydalisBlue,
        Self::Aquamarine,
        Self::SeaAngel,
        Self::CoolBlue,
        Self::SkyBlue,
        Self::BalticSea,
        Self::Moonstruck,
        Self::SilverLining,
        Self::Mercury,
        Self::Metal,
        Self::SkyGray,
        Self::Smoke,
        Self::Ether,
        Self::Stratosphere,
        Self::Aquatic,
        Self::GulfStream,
        Self::PorcelainBlue,
        Self::PetitFour,
        Self::BachelorButton,
        Self::AntiquaSand,
        Self::IslandParadise,
        Self::TibetanStone,
        Self::BlueFox,
        Self::SurfSpray,
        Self::EggshellBlue,
        Self::CanalBlue,
        Self::AquaSky,
        Self::AquaSplash,
        Self::AngelBlue,
        Self::BlueRadiance,
        Self::HarborGray,
        Self::Silver,
        Self::Holiday,
        Self::Bermuda,
        Self::Cockatoo,
        Self::SiltGreen,
        Self::AquaFoam,
        Self::OceanWave,
        Self::Cascade,
        Self::BeveledGlass,
        Self::Opal,
        Self::ElectricGreen,
        Self::SeaFoam,
        Self::SubtleGreen,
        Self::GrayedJade,
        Self::NeptuneGreen,
        Self::Pelican,
        Self::Alfalfa,
        Self::CameoGreen,
        Self::Sprucestone,
        Self::Meadow,
        Self::Peapod,
        Self::ZephyrGreen,
        Self::AbsintheGreen,
        Self::SpringBud,
        Self::SpringBouquet,
        Self::AbbeyStone,
        Self::Dove,
        Self::GreenFlash,
        Self::SprayGreen,
        Self::SageGreen,
        Self::Tarragon,
        Self::LeafGreen,
        Self::HerbalGarden,
        Self::ParrotGreen,
        Self::Greenery,
        Self::Eucalyptus,
        Self::PaleOliveGreen,
        Self::WinterPear,
        Self::WeepingWillow,
        Self::SweetPea,
        Self::LindenGreen,
        Self::Palm,
        Self::GreenOasis,
        Self::AppleGreen,
        Self::JasmineGreen,
        Self::Citronelle,
        Self::LeekGreen,
        Self::GoldenGreen,
        Self::CressGreen,
        Self::WarmOlive,
        Self::AshesOfRoses,
        Self::SilverFern,
        Self::SouthernMoss,
        Self::Olivenite,
        Self::OilYellow,
        Self::LemonCurry,
        Self::CeylonYellow,
        Self::PaleGold,
        Self::Sauterne,
        Self::GoldenSpice,
        Self::GoldenYellow,
        Self::OldGold,
        Self::MangoMojito,
        Self::GingerRoot,
        Self::IcedCoffee,
        Self::AutumnBlaze,
        Self::MineralYellow,
        Self::ArtisansGold,
        Self::GoldenGlow,
        Self::GoldenOrange,
        Self::CadmiumYellow,
        Self::RadiantYellow,
        Self::GoldFusion,
        Self::Travertine,
        Self::Safari,
        Self::TaosTaupe,
        Self::Porcini,
        Self::FallLeaf,
        Self::HoneyGold,
        Self::Chamois,
        Self::Butterscotch,
        Self::DarkCheddar,
        Self::IcelandPoppy,
        Self::Apricot,
        Self::FlameOrange,
        Self::BlazingOrange,
        Self::BrightMarigold,
        Self::CandiedGinger,
        Self::WarmSand,
        Self::Sesame,
        Self::PaleKhaki,
        Self::MojaveDesert,
        Self::Semolina,
        Self::Latte,
        Self::Sand,
        Self::Clay,
        Self::GoldEarth,
        Self::ApricotTan,
        Self::Cantaloupe,
        Self::Muskmelon,
        Self::Papaya,
        Self::MockOrange,
        Self::Tangerine,
        Self::CarrotCurl,
        Self::AutumnGlory,
        Self::Turmeric,
        Self::Humus,
        Self::FeatherGray,
        Self::OxfordTan,
        Self::WhitePepper,
        Self::Doeskin,
        Self::Moonlight,
        Self::CubanSand,
        Self::RugbyTan,
        Self::MapleSugar,
        Self::Sirocco,
        Self::PinkSand,
        Self::AlmostApricot,
        Self::DustyCoral,
        Self::PeachBloom,
        Self::CoralReef,
        Self::CanyonSunset,
        Self::ShellCoral,
        Self::Tangelo,
        Self::CadmiumOrange,
        Self::CoralCloud,
        Self::PeachAmber,
        Self::Fenugreek,
        Self::DustedClay,
        Self::PastryShell,
        Self::PapayaPunch,
        Self::DesertFlower,
        Self::Etherea,
        Self::MahoganyRose,
        Self::MistyRose,
        Self::MellowRose,
        Self::PeachBeige,
        Self::BloomingDahlia,
        Self::Shrimp,
        Self::PeachPink,
        Self::PaleMauve,
        Self::BridalRose,
        Self::Blush,
        Self::CandlelightPeach,
        Self::ConchShell,
        Self::SalmonRose,
        Self::PinkIcing,
        Self::Peony,
        Self::FlamingoPink,
        Self::BurnishedLilac,
        Self::Zephyr,
        Self::SeaPink,
        Self::MorningGlory,
        Self::GeraniumPink,
        Self::DawnPink,
        Self::OrchidSmoke,
        Self::Rosebloom,
        Self::BegoniaPink,
        Self::SachetPink,
        Self::AuroraPink,
        Self::KeepsakeLilac,
        Self::VioletIce,
        Self::FuchsiaPink,
        Self::LilacChiffon,
        Self::MauveMist,
        Self::Orchid,
        Self::OrchidBouquet,
        Self::LavenderFrost,
        Self::FairOrchid,
        Self::CrocusPetal,
        Self::Lavendula,
        Self::PurpleRose,
        Self::LilacBreeze,
        Self::Porpoise,
        Self::CloudGray,
        Self::MistyLilac,
        Self::Lavender,
        Self::IcelandicBlue,
        Self::CosmicSky,
        Self::LanguidLavender,
        Self::Aleutian,
        Self::KentuckyBlue,
        Self::Serenity,
        Self::PlacidBlue,
        Self::VistaBlue,
        Self::BelAirBlue,
        Self::StormGray,
        Self::DreamBlue,
        Self::BlueFog,
        Self::Cerulean,
        Self::ChambrayBlue,
        Self::HighRise,
        Self::AngelFalls,
        Self::AlaskanBlue,
        Self::Quarry,
        Self::BelgianBlock,
        Self::Tradewinds,
        Self::SterlingBlue,
        Self::ForgetMeNot,
        Self::AirBlue,
        Self::EtherealBlue,
        Self::MilkyBlue,
        Self::BlueGrotto,
        Self::NorseBlue,
        Self::CrystalSeas,
        Self::SilverCloud,
        Self::ChateauGray,
        Self::PuritanGray,
        Self::MirageGray,
        Self::Pigeon,
        Self::GrayMist,
        Self::BlueHaze,
        Self::MarineBlue,
        Self::Aquarelle,
        Self::AquaSea,
        Self::Aqua,
        Self::RiverBlue,
        Self::Capri,
        Self::BlueCuracao,
        Self::AquaGray,
        Self::Aquifer,
        Self::AquaHaze,
        Self::NileBlue,
        Self::BlueTurquoise,
        Self::PoolBlue,
        Self::FloridaKeys,
        Self::AquaGreen,
        Self::Atlantis,
        Self::Waterfall,
        Self::Turquoise,
        Self::BrightGreen,
        Self::MineralGray,
        Self::FrostyGreen,
        Self::DustyJadeGreen,
        Self::BiscayGreen,
        Self::MintLeaf,
        Self::Lichen,
        Self::Spearmint,
        Self::Hemlock,
        Self::Ming,
        Self::JadeCream,
        Self::PussywillowGray,
        Self::AgateGray,
        Self::Swamp,
        Self::LaurelGreen,
        Self::SmokeGreen,
        Self::FairGreen,
        Self::QuietGreen,
        Self::LightGrassGreen,
        Self::IrishGreen,
        Self::MossGray,
        Self::Reseda,
        Self::ForestShade,
        Self::GreenTea,
        Self::Shamrock,
        Self::GrassGreen,
        Self::BudGreen,
        Self::Paloma,
        Self::DesertSage,
        Self::Tendril,
        Self::VintageKhaki,
        Self::LondonFog,
        Self::Tea,
        Self::Mistletoe,
        Self::GreenEyes,
        Self::JadeGreen,
        Self::MacawGreen,
        Self::MeadowGreen,
        Self::Kiwi,
        Self::Foliage,
        Self::Sage,
        Self::Fern,
        Self::DarkCitron,
        Self::SpinachGreen,
        Self::GrayGreen,
        Self::Cedar,
        Self::Moss,
        Self::Oasis,
        Self::GoldenLime,
        Self::SplitPea,
        Self::LentilSprout,
        Self::Elm,
        Self::Willow,
        Self::GoldenOlive,
        Self::SlateGreen,
        Self::Khaki,
        Self::AntiqueGold,
        Self::BurnishedGold,
        Self::GreenSulphur,
        Self::Goat,
        Self::RichGold,
        Self::AntiqueMoss,
        Self::OliveOil,
        Self::SimplyTaupe,
        Self::CurdsAndWhey,
        Self::Croissant,
        Self::Curry,
        Self::Taffy,
        Self::Tinsel,
        Self::Honey,
        Self::BrightGold,
        Self::HarvestGold,
        Self::Narcissus,
        Self::NuggetGold,
        Self::TawnyOlive,
        Self::Arrowwood,
        Self::Incense,
        Self::Sunflower,
        Self::PureCashmere,
        Self::Crockery,
        Self::PlazaTaupe,
        Self::Tuffet,
        Self::Aluminum,
        Self::Twill,
        Self::Greige,
        Self::OliveGray,
        Self::Sponge,
        Self::Starfish,
        Self::Antelope,
        Self::MustardGold,
        Self::AmberGold,
        Self::Yam,
        Self::GoldenNugget,
        Self::HoneyYellow,
        Self::OakBuff,
        Self::Nugget,
        Self::DesertSun,
        Self::Topaz,
        Self::OrangePepper,
        Self::LightTaupe,
        Self::Nomad,
        Self::Tuscany,
        Self::CafeCreme,
        Self::Roebuck,
        Self::Sandstorm,
        Self::OrangeOchre,
        Self::RussetOrange,
        Self::SunOrange,
        Self::HarvestPumpkin,
        Self::String,
        Self::Natural,
        Self::Cornstalk,
        Self::Brush,
        Self::WarmTaupe,
        Self::Nougat,
        Self::Macaroon,
        Self::Lark,
        Self::Copper,
        Self::PrairieSand,
        Self::ToastedNut,
        Self::Sandstone,
        Self::CoralHaze,
        Self::MutedClay,
        Self::Toast,
        Self::Pheasant,
        Self::Doe,
        Self::Tan,
        Self::Biscuit,
        Self::CoralGold,
        Self::CopperTan,
        Self::BrandiedMelon,
        Self::Butterum,
        Self::Buckskin,
        Self::AutumnSunset,
        Self::DustyOrange,
        Self::SunBaked,
        Self::GoldenOchre,
        Self::PeachCaramel,
        Self::TomatoCream,
        Self::CoralRose,
        Self::Amberglow,
        Self::PersimmonOrange,
        Self::BirdOfParadise,
        Self::OrangeTiger,
        Self::OrangePeel,
        Self::Nectarine,
        Self::Carrot,
        Self::VermillionOrange,
        Self::PuffinsBill,
        Self::VibrantOrange,
        Self::Atmosphere,
        Self::Cobblestone,
        Self::Stucco,
        Self::Chanterelle,
        Self::Almondine,
        Self::Cork,
        Self::Sunburn,
        Self::CanyonClay,
        Self::Almond,
        Self::CoralAlmond,
        Self::Carnelian,
        Self::Meerkat,
        Self::Caramel,
        Self::Langoustino,
        Self::Arabesque,
        Self::Melon,
        Self::ApricotBuff,
        Self::BurntOrange,
        Self::GoldFlame,
        Self::Flamingo,
        Self::Nasturtium,
        Self::Firecracker,
        Self::ExoticOrange,
        Self::JaffaOrange,
        Self::MandarinOrange,
        Self::DragonFire,
        Self::GoldenPoppy,
        Self::Bark,
        Self::AdobeRose,
        Self::ShadowGray,
        Self::Fawn,
        Self::RoseTan,
        Self::CameoBrown,
        Self::Rosette,
        Self::LobsterBisque,
        Self::RoseDawn,
        Self::TerraCotta,
        Self::BurntCoral,
        Self::Crabapple,
        Self::Coral,
        Self::Camellia,
        Self::FreshSalmon,
        Self::FusionCoral,
        Self::Persimmon,
        Self::CoralQuartz,
        Self::LivingCoral,
        Self::PeachEcho,
        Self::PurpleDove,
        Self::BrandiedApricot,
        Self::Mauveglow,
        Self::TeaRose,
        Self::Lantana,
        Self::PeachBlossom,
        Self::ShellPink,
        Self::SugarCoral,
        Self::GeorgiaPeach,
        Self::Sphinx,
        Self::DeauvilleMauve,
        Self::Lilas,
        Self::Foxglove,
        Self::Polignac,
        Self::WildRose,
        Self::StrawberryIce,
        Self::Confetti,
        Self::StrawberryPink,
        Self::PinkLemonade,
        Self::Woodrose,
        Self::OrchidHaze,
        Self::MauveOrchid,
        Self::WildOrchid,
        Self::PinkCarnation,
        Self::AzaleaPink,
        Self::CashmereRose,
        Self::MoonliteMauve,
        Self::SmokyGrape,
        Self::Crocus,
        Self::OperaMauve,
        Self::Cyclamen,
        Self::MauveShadows,
        Self::SeaFog,
        Self::LavenderMist,
        Self::LavenderHerb,
        Self::Violet,
        Self::VioletTulle,
        Self::AfricanViolet,
        Self::Lupine,
        Self::RegalOrchid,
        Self::SheerLilac,
        Self::SandVerbena,
        Self::Satellite,
        Self::OpalGray,
        Self::Ash,
        Self::GullGray,
        Self::Wisteria,
        Self::HeirloomLilac,
        Self::Viola,
        Self::Rhapsody,
        Self::VioletTulip,
        Self::SilverSconce,
        Self::LilacGray,
        Self::DappleGray,
        Self::LavenderAura,
        Self::Alloy,
        Self::Sleet,
        Self::Eventide,
        Self::LavenderLustre,
        Self::BlueHeron,
        Self::BrunneraBlue,
        Self::EasterEgg,
        Self::Grapemist,
        Self::ThistleDown,
        Self::SweetLavender,
        Self::DustyBlue,
        Self::AshleyBlue,
        Self::ForeverBlue,
        Self::DellaRobbiaBlue,
        Self::Allure,
        Self::Hydrangea,
        Self::CornflowerBlue,
        Self::Provence,
        Self::GranadaSky,
        Self::Arona,
        Self::StoneBlue,
        Self::DuskBlue,
        Self::HeritageBlue,
        Self::LittleBoyBlue,
        Self::BonnieBlue,
        Self::MourningDove,
        Self::Drizzle,
        Self::Abyss,
        Self::Slate,
        Self::Tourmaline,
        Self::CameoBlue,
        Self::BlueMist,
        Self::HorizonBlue,
        Self::DelphiniumBlue,
        Self::MauiBlue,
        Self::CyanBlue,
        Self::Aquarius,
        Self::BlueAtoll,
        Self::Stillwater,
        Self::ReefWaters,
        Self::Limestone,
        Self::GhostGray,
        Self::SilverBlue,
        Self::MineralBlue,
        Self::Porcelain,
        Self::ScubaBlue,
        Self::PeacockBlue,
        Self::Bluebird,
        Self::WetWeather,
        Self::BlueSurf,
        Self::Wasabi,
        Self::Canton,
        Self::DustyTurquoise,
        Self::SeaBlue,
        Self::Meadowbrook,
        Self::Baltic,
        Self::Ceramic,
        Self::Jadeite,
        Self::AgateGreen,
        Self::Lagoon,
        Self::SeaGreen,
        Self::BrightAqua,
        Self::PoolGreen,
        Self::Billiard,
        Self::PeacockGreen,
        Self::BerylGreen,
        Self::Arcadia,
        Self::MarineGreen,
        Self::FlintGray,
        Self::SlateGray,
        Self::GreenMilieu,
        Self::LilyPad,
        Self::IcebergGreen,
        Self::GreenBay,
        Self::Feldspar,
        Self::GreenSpruce,
        Self::GumdropGreen,
        Self::WroughtIron,
        Self::GraniteGreen,
        Self::MalachiteGreen,
        Self::CremeDeMenthe,
        Self::WinterGreen,
        Self::MingGreen,
        Self::HollyGreen,
        Self::Mint,
        Self::Blarney,
        Self::Seagrass,
        Self::Katydid,
        Self::ShaleGreen,
        Self::Greenbriar,
        Self::KellyGreen,
        Self::Basil,
        Self::IslandGreen,
        Self::MineralGreen,
        Self::Jadesheen,
        Self::Peppermint,
        Self::VibrantGreen,
        Self::ClassicGreen,
        Self::PoisonGreen,
        Self::FrostGray,
        Self::OilGreen,
        Self::TurfGreen,
        Self::StoneGreen,
        Self::FluoriteGreen,
        Self::OnlineLime,
        Self::ElephantSkin,
        Self::RockRidge,
        Self::LodenFrost,
        Self::AspenGreen,
        Self::Watercress,
        Self::ForestGreen,
        Self::PiquantGreen,
        Self::Epsom,
        Self::TurtleGreen,
        Self::Peridot,
        Self::SilverSage,
        Self::DuskyGreen,
        Self::Mosstone,
        Self::Guacamole,
        Self::GreenOlive,
        Self::LaurelOak,
        Self::Vetiver,
        Self::Mermaid,
        Self::Aloe,
        Self::Boa,
        Self::DriedHerb,
        Self::TreeHouse,
        Self::GreenMoss,
        Self::TaupeGray,
        Self::EcruOlive,
        Self::GoldenPalm,
        Self::AmberGreen,
        Self::BronzeMist,
        Self::Fossil,
        Self::FennelSeed,
        Self::DullGold,
        Self::MedalBronze,
        Self::ChaiTea,
        Self::Dune,
        Self::Elmwood,
        Self::Kelp,
        Self::AntiqueBronze,
        Self::Bistre,
        Self::TigersEye,
        Self::SpruceYellow,
        Self::Chipmunk,
        Self::AppleCinnamon,
        Self::GoldenOak,
        Self::HoneyMustard,
        Self::IncaGold,
        Self::RoastedPecan,
        Self::RoastedCashew,
        Self::SenecaRock,
        Self::WinterTwig,
        Self::Chinchilla,
        Self::Coriander,
        Self::PetrifiedOak,
        Self::LeadGray,
        Self::Dijon,
        Self::BoneBrown,
        Self::WoodThrush,
        Self::BrownSugar,
        Self::Cashew,
        Self::Marmalade,
        Self::ArganOil,
        Self::Hazel,
        Self::AutumnMaple,
        Self::AmberBrown,
        Self::MoonRock,
        Self::Fungi,
        Self::Praline,
        Self::Camel,
        Self::TawnyBirch,
        Self::TawnyBrown,
        Self::CafeAuLait,
        Self::MochaMousse,
        Self::TimberWolf,
        Self::DesertTaupe,
        Self::SilverMink,
        Self::SepiaTint,
        Self::Portabella,
        Self::Amphora,
        Self::Tannin,
        Self::Woodsmoke,
        Self::Burro,
        Self::TobaccoBrown,
        Self::IndianTan,
        Self::Lion,
        Self::Bran,
        Self::Adobe,
        Self::TawnyOrange,
        Self::Autumnal,
        Self::SpiceRoute,
        Self::AutumnLeaf,
        Self::Exuberance,
        Self::OrangePopsicle,
        Self::ApricotOrange,
        Self::CelosiaOrange,
        Self::ScarletIbis,
        Self::PineBark,
        Self::BeaverFur,
        Self::GingerSnap,
        Self::RawUmber,
        Self::BrickDust,
        Self::PecanBrown,
        Self::RawSienna,
        Self::Ginger,
        Self::Mango,
        Self::PureedPumpkin,
        Self::SummerFig,
        Self::Koi,
        Self::Tigerlily,
        Self::Orangeade,
        Self::Flame,
        Self::TangerineTango,
        Self::RedOrange,
        Self::SteepleGray,
        Self::WildDove,
        Self::Cloudburst,
        Self::StormFront,
        Self::Quail,
        Self::Cinder,
        Self::Antler,
        Self::WistfulMauve,
        Self::NostalgiaRose,
        Self::AshRose,
        Self::Burlwood,
        Self::OldRose,
        Self::CanyonRose,
        Self::Mauvewood,
        Self::DesertSand,
        Self::CedarWood,
        Self::Aragon,
        Self::MineralRed,
        Self::ApricotBrandy,
        Self::BurntSienna,
        Self::Cranberry,
        Self::Emberglow,
        Self::Paprika,
        Self::Grenadine,
        Self::MandarinRed,
        Self::CherryTomato,
        Self::Fiesta,
        Self::Elderberry,
        Self::HeatherRose,
        Self::MesaRose,
        Self::DuskyOrchid,
        Self::MellowMauve,
        Self::DecoRose,
        Self::RoseWine,
        Self::HollyBerry,
        Self::RoseOfSharon,
        Self::Chrysanthemum,
        Self::PorcelainRose,
        Self::SpicedCoral,
        Self::Dubarry,
        Self::Poinsettia,
        Self::HotCoral,
        Self::Bittersweet,
        Self::PoppyRed,
        Self::Moonscape,
        Self::Bordeaux,
        Self::DustyRose,
        Self::Malaga,
        Self::SunkistCoral,
        Self::ClaretRed,
        Self::CalypsoCoral,
        Self::Geranium,
        Self::ParadisePink,
        Self::RedViolet,
        Self::Carmine,
        Self::Azalea,
        Self::FruitDove,
        Self::DesertRose,
        Self::Bubblegum,
        Self::RaptureRose,
        Self::CamelliaRose,
        Self::HotPink,
        Self::FuchsiaRose,
        Self::FandangoPink,
        Self::PinkYarrow,
        Self::Magenta,
        Self::ChateauRose,
        Self::ShockingPink,
        Self::LilacRose,
        Self::CarmineRose,
        Self::Toadstool,
        Self::IbisRose,
        Self::Zinc,
        Self::DahliaMauve,
        Self::RoseViolet,
        Self::SuperPink,
        Self::PhloxPink,
        Self::Mulberry,
        Self::SpringCrocus,
        Self::Rosebud,
        Self::Bodacious,
        Self::DustyLavender,
        Self::IrisOrchid,
        Self::Valerian,
        Self::DiffusedOrchid,
        Self::OrchidMist,
        Self::ChalkViolet,
        Self::EnglishLavendar,
        Self::Hyacinth,
        Self::AmethystOrchid,
        Self::FairyWren,
        Self::Bougainvillea,
        Self::PaisleyPurple,
        Self::SunlitAllium,
        Self::Gull,
        Self::Nirvana,
        Self::PurpleAsh,
        Self::Dusk,
        Self::Daybreak,
        Self::AsterPurple,
        Self::DahliaPurple,
        Self::MinimalGray,
        Self::Quicksilver,
        Self::LavenderGray,
        Self::SilverFiligree,
        Self::Sharkskin,
        Self::Tempest,
        Self::Stonewash,
        Self::CountryBlue,
        Self::PurpleImpression,
        Self::EnglishManor,
        Self::BlueIce,
        Self::ColonyBlue,
        Self::LavenderViolet,
        Self::PersianViolet,
        Self::PaleIris,
        Self::Jacaranda,
        Self::DeepPeriwinkle,
        Self::SilverBullet,
        Self::PersianJewel,
        Self::BlueBonnet,
        Self::Iolite,
        Self::Titanium,
        Self::Infinity,
        Self::GrayFlannel,
        Self::BlueShadow,
        Self::FadedDenim,
        Self::BlueHeaven,
        Self::Riviera,
        Self::Riverside,
        Self::QuietHarbor,
        Self::SilverLakeBlue,
        Self::LichenBlue,
        Self::PacificCoast,
        Self::Ultramarine,
        Self::Marina,
        Self::Citadel,
        Self::Niagara,
        Self::CendreBlue,
        Self::AzureBlue,
        Self::IbizaBlue,
        Self::DivaBlue,
        Self::AdriaticBlue,
        Self::BlueMoon,
        Self::Blithe,
        Self::NeutralGray,
        Self::Monument,
        Self::Lead,
        Self::SmokeBlue,
        Self::Larkspur,
        Self::Bluejay,
        Self::NavagioBay,
        Self::VividBlue,
        Self::DresdenBlue,
        Self::MalibuBlue,
        Self::BlueDanube,
        Self::BarrierReef,
        Self::HawaiianOcean,
        Self::StormBlue,
        Self::PagodaBlue,
        Self::AlgiersBlue,
        Self::CaneelBay,
        Self::CapriBreeze,
        Self::BristolBlue,
        Self::Arctic,
        Self::Teal,
        Self::LakeBlue,
        Self::TealBlue,
        Self::Navigate,
        Self::DeepPeacockBlue,
        Self::Lapis,
        Self::Griffin,
        Self::ChinoisGreen,
        Self::Trellis,
        Self::OilBlue,
        Self::GreenBlueSlate,
        Self::LatigoBay,
        Self::ViridianGreen,
        Self::Columbia,
        Self::DynastyGreen,
        Self::SpectraGreen,
        Self::PorcelainGreen,
        Self::Alhambra,
        Self::DeepSea,
        Self::Greenlake,
        Self::DeepGreen,
        Self::VividGreen,
        Self::Emerald,
        Self::BottleGreen,
        Self::Viridis,
        Self::Parakeet,
        Self::DarkIvy,
        Self::PineGreen,
        Self::SimplyGreen,
        Self::DeepMint,
        Self::LaurelWreath,
        Self::JellyBean,
        Self::FernGreen,
        Self::Shadow,
        Self::SeaSpray,
        Self::DeepGrassGreen,
        Self::MediumGreen,
        Self::Kashmir,
        Self::HedgeGreen,
        Self::MintGreen,
        Self::SmokedPearl,
        Self::Kale,
        Self::Dill,
        Self::EnglishIvy,
        Self::VineyardGreen,
        Self::WillowBough,
        Self::ElmGreen,
        Self::ArtichokeGreen,
        Self::Cactus,
        Self::Treetop,
        Self::Castlerock,
        Self::Mayfly,
        Self::Pesto,
        Self::Gunmetal,
        Self::DeepLichenGreen,
        Self::Olivine,
        Self::BronzeGreen,
        Self::Cypress,
        Self::CallisteGreen,
        Self::CedarGreen,
        Self::TwistOfLime,
        Self::Grasshopper,
        Self::DarkGullGray,
        Self::FourLeafClover,
        Self::LodenGreen,
        Self::CapuletOlive,
        Self::Avocado,
        Self::CallaGreen,
        Self::Gargoyle,
        Self::CastorGray,
        Self::BungeeCord,
        Self::DustyOlive,
        Self::BurntOlive,
        Self::WinterMoss,
        Self::Iguana,
        Self::OliveBranch,
        Self::Woodbine,
        Self::CharcoalGray,
        Self::StoneGray,
        Self::CovertGreen,
        Self::OliveDrab,
        Self::MartiniOlive,
        Self::FirGreen,
        Self::Lizard,
        Self::GothicOlive,
        Self::Capers,
        Self::Nutria,
        Self::Butternut,
        Self::Plantation,
        Self::DriedTobacco,
        Self::Tapenade,
        Self::Kangaroo,
        Self::Sepia,
        Self::CoffeeLiqueur,
        Self::Rubber,
        Self::BuckthornBrown,
        Self::BronzeBrown,
        Self::Cumin,
        Self::GoldenBrown,
        Self::CathaySpice,
        Self::Shitake,
        Self::Cub,
        Self::Caribou,
        Self::Otter,
        Self::Ermine,
        Self::Bison,
        Self::Emperador,
        Self::ToastedCoconut,
        Self::Thrush,
        Self::Toffee,
        Self::Dachshund,
        Self::MonksRobe,
        Self::ThaiCurry,
        Self::HoneyGinger,
        Self::FallenRock,
        Self::Brindle,
        Self::Walnut,
        Self::Partridge,
        Self::Aztec,
        Self::Rawhide,
        Self::MochaBisque,
        Self::LeatherBrown,
        Self::CaramelCafe,
        Self::GlazedGinger,
        Self::SugarAlmond,
        Self::SudanBrown,
        Self::PumpkinSpice,
        Self::Driftwood,
        Self::CocoaBrown,
        Self::CarobBrown,
        Self::CoconutShell,
        Self::Russet,
        Self::RusticBrown,
        Self::Sierra,
        Self::BrownPatina,
        Self::GingerBread,
        Self::Umber,
        Self::Rust,
        Self::HawaiianSunset,
        Self::BombayBrown,
        Self::Falcon,
        Self::Iron,
        Self::DeepTaupe,
        Self::Acorn,
        Self::Clove,
        Self::Brownie,
        Self::SpicedApple,
        Self::Nutmeg,
        Self::CopperBrown,
        Self::PottersClay,
        Self::Auburn,
        Self::CinnamonStick,
        Self::Bruschetta,
        Self::BurntBrick,
        Self::BurntOchre,
        Self::RooibosTea,
        Self::Sparrow,
        Self::Flint,
        Self::Peppercorn,
        Self::PlumWine,
        Self::Marron,
        Self::CrushedBerry,
        Self::WildGinger,
        Self::Cognac,
        Self::Mahogany,
        Self::AppleButter,
        Self::Chutney,
        Self::EtruscanRed,
        Self::WitheredRose,
        Self::LightMahogany,
        Self::Marsala,
        Self::ChiliOil,
        Self::BakedClay,
        Self::RedOchre,
        Self::Redwood,
        Self::TandoriSpice,
        Self::SpicyOrange,
        Self::OrangeRust,
        Self::Chili,
        Self::Ketchup,
        Self::MeccaOrange,
        Self::AutumnGlaze,
        Self::RedClay,
        Self::PlumTruffle,
        Self::RoseBrown,
        Self::BarnRed,
        Self::GingerSpice,
        Self::HotSauce,
        Self::CopperCoin,
        Self::Cinnabar,
        Self::BrandyBrown,
        Self::BossaNova,
        Self::ValiantPoppy,
        Self::AuroraRed,
        Self::AuraOrange,
        Self::MoltenLava,
        Self::OrangeCom,
        Self::Poinciana,
        Self::RoseTaupe,
        Self::RenaissanceRose,
        Self::Nocturne,
        Self::RoanRouge,
        Self::Maroon,
        Self::FadedRose,
        Self::DustyCedar,
        Self::EarthRed,
        Self::GarnetRose,
        Self::BaroqueRose,
        Self::SlateRose,
        Self::Cardinal,
        Self::BakedApple,
        Self::DeepSeaCoral,
        Self::Cayenne,
        Self::RococcoRed,
        Self::Toreador,
        Self::Lychee,
        Self::MarsRed,
        Self::Salsa,
        Self::PompeianRed,
        Self::GojiBerry,
        Self::Tomato,
        Self::TomatoPuree,
        Self::FlameScarlet,
        Self::ChineseRed,
        Self::FieryRed,
        Self::Shark,
        Self::ArcticDusk,
        Self::BlackPlum,
        Self::Ephemera,
        Self::Tulipwood,
        Self::GrapeNectar,
        Self::Damson,
        Self::HawthornRose,
        Self::VioletQuartz,
        Self::DryRose,
        Self::RaspberryWine,
        Self::Raspberry,
        Self::RougeRed,
        Self::Teaberry,
        Self::JalapenoRed,
        Self::Barberry,
        Self::SkiPatrol,
        Self::Hibiscus,
        Self::HighRiskRed,
        Self::Lollipop,
        Self::TwilightMauve,
        Self::RoseRed,
        Self::VirtualPink,
        Self::BrightRose,
        Self::Jazzy,
        Self::LovePotion,
        Self::Beaujolais,
        Self::RaspberrySorbet,
        Self::PinkPeacock,
        Self::GrapeShake,
        Self::Honeysuckle,
        Self::PinkFlambe,
        Self::Cabaret,
        Self::BeetrootPurple,
        Self::Clover,
        Self::CactusFlower,
        Self::FuchsiaRed,
        Self::RaspberryRose,
        Self::VeryBerry,
        Self::FuchsiaPurple,
        Self::MagentaHaze,
        Self::BatonRouge,
        Self::PurpleWine,
        Self::ArgylePurple,
        Self::PurpleGumdrop,
        Self::BerryConserve,
        Self::GrapeKiss,
        Self::Amethyst,
        Self::DeepOrchid,
        Self::StrikingPurple,
        Self::PurpleOrchid,
        Self::Willowherb,
        Self::Grapeade,
        Self::ConcordGrape,
        Self::VeryGrape,
        Self::RadiantOrchid,
        Self::MeadowMauve,
        Self::Dahlia,
        Self::HyacinthViolet,
        Self::VividViola,
        Self::Charisma,
        Self::VintageViolet,
        Self::GrapeJam,
        Self::ChineseViolet,
        Self::GrapeCompote,
        Self::PatricianPurple,
        Self::PurpleHeart,
        Self::PlumJam,
        Self::CrushedGrape,
        Self::LavenderCrystal,
        Self::RoyalLilac,
        Self::Dewberry,
        Self::PurpleSapphire,
        Self::ImperialPalace,
        Self::Mystical,
        Self::Bellflower,
        Self::DeepLavender,
        Self::ChiveBlossom,
        Self::PicassoLily,
        Self::GrayRidge,
        Self::PurpleSage,
        Self::MulledGrape,
        Self::MontanaGrape,
        Self::PurpleHaze,
        Self::PassionFlower,
        Self::Cadet,
        Self::Heron,
        Self::TwilightPurple,
        Self::CorsicanBlue,
        Self::DustedPeri,
        Self::Veronica,
        Self::UltraViolet,
        Self::PurpleCorallite,
        Self::PurpleOpulence,
        Self::Excalibur,
        Self::Tornado,
        Self::VolcanicGlass,
        Self::FolkstoneGray,
        Self::Grisaille,
        Self::FlintStone,
        Self::GrayBlue,
        Self::ChinaBlue,
        Self::CoastalFjord,
        Self::BijouBlue,
        Self::CoronetBlue,
        Self::VelvetMorning,
        Self::DutchBlue,
        Self::BlueHorizon,
        Self::BleachedDenim,
        Self::Marlin,
        Self::BlueGranite,
        Self::Wedgewood,
        Self::BlueYonder,
        Self::BlueIris,
        Self::VioletStorm,
        Self::AmparoBlue,
        Self::BajaBlue,
        Self::DazzlingBlue,
        Self::IrisBloom,
        Self::SpectrumBlue,
        Self::SteelGray,
        Self::QuietShade,
        Self::GoblinBlue,
        Self::DecemberSky,
        Self::RealTeal,
        Self::CaptainsBlue,
        Self::BlueAshes,
        Self::CopenBlue,
        Self::Stellar,
        Self::MoonlightBlue,
        Self::BeringSea,
        Self::FederalBlue,
        Self::DeepWater,
        Self::VallartaBlue,
        Self::ParisianBlue,
        Self::Regatta,
        Self::StarSapphire,
        Self::PalaceBlue,
        Self::Daphne,
        Self::NebulasBlue,
        Self::StrongBlue,
        Self::MoonMist,
        Self::FrenchBlue,
        Self::Campanula,
        Self::VictoriaBlue,
        Self::StormyWeather,
        Self::BlueMirage,
        Self::Bluestone,
        Self::ProvincialBlue,
        Self::Bluesteel,
        Self::SaxonyBlue,
        Self::BlueSapphire,
        Self::Faience,
        Self::DirectoireBlue,
        Self::BrilliantBlue,
        Self::IndigoBunting,
        Self::BlueAster,
        Self::AegeanBlue,
        Self::SwedishBlue,
        Self::MediterraneanBlue,
        Self::Tapestry,
        Self::FjordBlue,
        Self::TurkishTile,
        Self::MykonosBlue,
        Self::Cloisonne,
        Self::Trooper,
        Self::ColonialBlue,
        Self::CaribbeanSea,
        Self::MosaicBlue,
        Self::Celestial,
        Self::BlueJewel,
        Self::MethylBlue,
        Self::HawaiianSurf,
        Self::NorthAtlantic,
        Self::TahitianTide,
        Self::StormySea,
        Self::Hydro,
        Self::BiscayBay,
        Self::HarborBlue,
        Self::EnamelBlue,
        Self::TileBlue,
        Self::DeepLake,
        Self::TropicalGreen,
        Self::Fanfare,
        Self::Parasailing,
        Self::QuetzalGreen,
        Self::BrushedNickel,
        Self::SedonaSage,
        Self::SeaPine,
        Self::NorthSea,
        Self::Bayou,
        Self::BlueGrass,
        Self::Pewter,
        Self::GraniteGray,
        Self::EiffelTower,
        Self::BlueSpruce,
        Self::Bayberry,
        Self::AlpineGreen,
        Self::UltramarineGreen,
        Self::SilverPine,
        Self::AntiqueGreen,
        Self::CadmiumGreen,
        Self::BalsamGreen,
        Self::BrittanyBlue,
        Self::DarkForest,
        Self::SagebrushGreen,
        Self::PosyGreen,
        Self::DeepJungle,
        Self::Tidepool,
        Self::Ivy,
        Self::Fir,
        Self::FrostySpruce,
        Self::ShadyGlade,
        Self::Bosphorus,
        Self::GolfGreen,
        Self::SmokePine,
        Self::GalapagosGreen,
        Self::AgaveGreen,
        Self::PepperGreen,
        Self::LushMeadow,
        Self::GardenTopiary,
        Self::DuckGreen,
        Self::FoliageGreen,
        Self::Leprechaun,
        Self::Amazon,
        Self::JollyGreen,
        Self::Myrtle,
        Self::Comfrey,
        Self::Fairway,
        Self::Juniper,
        Self::Raven,
        Self::Asphalt,
        Self::GrayPinstripe,
        Self::GardenGreen,
        Self::JetBlack,
        Self::ClimbingIvy,
        Self::Thyme,
        Self::Beetle,
        Self::BlackForest,
        Self::Chive,
        Self::SeaTurtle,
        Self::Beluga,
        Self::DeepDepths,
        Self::ForestNight,
        Self::DuffelBag,
        Self::KombuGreen,
        Self::RifleGreen,
        Self::BlackInk,
        Self::Peat,
        Self::Rosin,
        Self::Kalamata,
        Self::GrapeLeaf,
        Self::IvyGreen,
        Self::OliveNight,
        Self::DarkOlive,
        Self::BlackOlive,
        Self::Wren,
        Self::Teak,
        Self::Beech,
        Self::MilitaryOlive,
        Self::Demitasse,
        Self::Morel,
        Self::ChocolateChip,
        Self::MajorBrown,
        Self::TurkishCoffee,
        Self::SlateBlack,
        Self::DesertPalm,
        Self::Canteen,
        Self::Tarmac,
        Self::Crocodile,
        Self::Delicioso,
        Self::Mulch,
        Self::ChocolateBrown,
        Self::CoffeeBean,
        Self::RainDrum,
        Self::FrenchRoast,
        Self::Bracken,
        Self::Java,
        Self::Ganache,
        Self::DarkEarth,
        Self::Breen,
        Self::AfterDark,
        Self::Licorice,
        Self::Espresso,
        Self::Mole,
        Self::ChocolateTorte,
        Self::ChocolatePlum,
        Self::BlackCoffee,
        Self::Carafe,
        Self::Chestnut,
        Self::Pinecone,
        Self::ShoppingBag,
        Self::ChocolateLab,
        Self::ShavedChocolate,
        Self::Mustang,
        Self::PottingSoil,
        Self::Cappuccino,
        Self::FondueFudge,
        Self::RootBeer,
        Self::FriarBrown,
        Self::Tiramisu,
        Self::RockyRoad,
        Self::Brunette,
        Self::TortoiseShell,
        Self::ArabianSpice,
        Self::Picante,
        Self::SealBrown,
        Self::BitterChocolate,
        Self::Sable,
        Self::RumRaisin,
        Self::BrownStone,
        Self::HotChocolate,
        Self::Andorra,
        Self::MadderBrown,
        Self::Sequoia,
        Self::Henna,
        Self::FiredBrick,
        Self::RussetBrown,
        Self::ChicoryCoffee,
        Self::DeepMahogany,
        Self::SmokedPaprika,
        Self::Mink,
        Self::Fudgesickle,
        Self::ChocolateFondant,
        Self::CherryMahogany,
        Self::Cinnamon,
        Self::Puce,
        Self::RedMahogany,
        Self::Zinfandel,
        Self::OxbloodRed,
        Self::Port,
        Self::ChocolateTruffle,
        Self::WindsorWine,
        Self::BurntRusset,
        Self::SunDriedTomato,
        Self::Rosewood,
        Self::Cowhide,
        Self::Merlot,
        Self::Syrah,
        Self::RedPear,
        Self::BurntHenna,
        Self::BrickRed,
        Self::RedDahlia,
        Self::ChiliPepper,
        Self::ScarletSage,
        Self::Raisin,
        Self::PrunePurple,
        Self::Burgundy,
        Self::Fudge,
        Self::Huckleberry,
        Self::CatawbaGrape,
        Self::VineyardWine,
        Self::Sassafras,
        Self::DecadentChocolate,
        Self::PortRoyale,
        Self::RubyWine,
        Self::BikingRed,
        Self::Rhubarb,
        Self::Garnet,
        Self::RioRed,
        Self::Samba,
        Self::RibbonRed,
        Self::TrueRed,
        Self::MauveWine,
        Self::Fig,
        Self::Cabernet,
        Self::TawnyPort,
        Self::Cordovan,
        Self::BarbadosCherry,
        Self::HauteRed,
        Self::AmericanBeauty,
        Self::Scarlet,
        Self::TangoRed,
        Self::Crimson,
        Self::RacingRed,
        Self::LipstickRed,
        Self::DeepClaret,
        Self::RedBud,
        Self::PersianRed,
        Self::JesterRed,
        Self::Scooter,
        Self::Pomegranate,
        Self::TibetanRed,
        Self::RumbaRed,
        Self::Cerise,
        Self::Hortensia,
        Self::Prune,
        Self::Rhododendron,
        Self::RedPlum,
        Self::BeetRed,
        Self::Anemone,
        Self::Granita,
        Self::CherriesJubilee,
        Self::Vivacious,
        Self::Sangria,
        Self::Winetasting,
        Self::Eggplant,
        Self::CrushedViolets,
        Self::GrapeWine,
        Self::Amaranth,
        Self::PickledBeet,
        Self::MagentaPurple,
        Self::PlumCaspia,
        Self::PurplePotion,
        Self::Boysenberry,
        Self::RaspberryRadiance,
        Self::FestivalFuchsia,
        Self::ItalianPlum,
        Self::PotentPurple,
        Self::DarkPurple,
        Self::WinterBloom,
        Self::WildAster,
        Self::Wineberry,
        Self::BlackberryWine,
        Self::Phlox,
        Self::Hollyhock,
        Self::Gloxinia,
        Self::Byzantium,
        Self::Indigo,
        Self::ShadowPurple,
        Self::PlumPurple,
        Self::Plum,
        Self::PurplePassion,
        Self::GrapeJuice,
        Self::PlumPerfect,
        Self::DeepPurple,
        Self::WoodViolet,
        Self::SparklingGrape,
        Self::SunsetPurple,
        Self::SpicedPlum,
        Self::BrightViolet,
        Self::Majesty,
        Self::GrapeRoyale,
        Self::PurplePennant,
        Self::BlackberryCordial,
        Self::MeadowViolet,
        Self::ImperialPurple,
        Self::AmaranthPurple,
        Self::PurpleMagic,
        Self::Pansy,
        Self::Mysterioso,
        Self::SweetGrape,
        Self::PurpleReign,
        Self::Loganberry,
        Self::Acai,
        Self::Petunia,
        Self::TillandsiaPurple,
        Self::CrownJewel,
        Self::RoyalPurple,
        Self::Nightshade,
        Self::DeepWell,
        Self::NavyCosmos,
        Self::PurplePlumeria,
        Self::GothicGrape,
        Self::MulberryPurple,
        Self::PurpleVelvet,
        Self::Grape,
        Self::GentianViolet,
        Self::ParachutePurple,
        Self::Heliotrope,
        Self::PrismViolet,
        Self::VioletIndigo,
        Self::PlumKitten,
        Self::Eclipse,
        Self::EveningBlue,
        Self::AstralAura,
        Self::MaritimeBlue,
        Self::NavyBlue,
        Self::BlueRibbon,
        Self::DeepWisteria,
        Self::DeepBlue,
        Self::Liberty,
        Self::MazarineBlue,
        Self::Pavement,
        Self::Magnet,
        Self::Obsidian,
        Self::Shale,
        Self::Rabbit,
        Self::DarkShadow,
        Self::ForgedIron,
        Self::NineIron,
        Self::BlackBean,
        Self::IronGate,
        Self::BlackBeauty,
        Self::Graystone,
        Self::BlackenedPearl,
        Self::Periscope,
        Self::NightshadowBlue,
        Self::Peacoat,
        Self::BlackIris,
        Self::SkyCaptain,
        Self::NavyBlazer,
        Self::NightSky,
        Self::PatriotBlue,
        Self::CrownBlue,
        Self::Graphite,
        Self::BlueIndigo,
        Self::VintageIndigo,
        Self::OdysseyGray,
        Self::MedievalBlue,
        Self::DeepCobalt,
        Self::SkipperBlue,
        Self::TwilightBlue,
        Self::Blueprint,
        Self::BlueDepths,
        Self::OrientBlue,
        Self::DeepUltramarine,
        Self::ClematisBlue,
        Self::SurfTheWeb,
        Self::SodaliteBlue,
        Self::RoyalBlue,
        Self::MonacoBlue,
        Self::BlackOnyx,
        Self::TapShoe,
        Self::StretchLimo,
        Self::Caviar,
        Self::Anthracite,
        Self::Meteorite,
        Self::OuterSpace,
        Self::TotalEclipse,
        Self::Salute,
        Self::Carbon,
        Self::DarkNavy,
        Self::OmbreBlue,
        Self::BlueGraphite,
        Self::IndiaInk,
        Self::DarkSapphire,
        Self::Blueberry,
        Self::ParisianNight,
        Self::BlueNights,
        Self::DressBlues,
        Self::MoodIndigo,
        Self::EnsignBlue,
        Self::EstateBlue,
        Self::InsigniaBlue,
        Self::NavyPeony,
        Self::TrueNavy,
        Self::SargassoSea,
        Self::Poseidon,
        Self::SailorBlue,
        Self::DarkBlue,
        Self::BrightCobalt,
        Self::GibraltarSea,
        Self::Delft,
        Self::Limoges,
        Self::LapisBlue,
        Self::BaleineBlue,
        Self::SnorkelBlue,
        Self::NauticalBlue,
        Self::ClassicBlue,
        Self::TurkishSea,
        Self::GalaxyBlue,
        Self::OlympianBlue,
        Self::TrueBlue,
        Self::Ebony,
        Self::MidnightNavy,
        Self::DarkDenim,
        Self::BlueOpal,
        Self::BlueWingTeal,
        Self::MoonlitOcean,
        Self::MajolicaBlue,
        Self::DeepDive,
        Self::Midnight,
        Self::PrincessBlue,
        Self::Skydiver,
        Self::MoonlessNight,
        Self::Phantom,
        Self::Turbulence,
        Self::Vulcan,
        Self::DarkSlate,
        Self::IndianTeal,
        Self::OrionBlue,
        Self::InkBlue,
        Self::MoroccanBlue,
        Self::ImperialBlue,
        Self::PirateBlack,
        Self::Stargazer,
        Self::MallardBlue,
        Self::LegionBlue,
        Self::ReflectingPond,
        Self::Corsair,
        Self::LyonsBlue,
        Self::Seaport,
        Self::Mediterranea,
        Self::ShadedSpruce,
        Self::BlueCoral,
        Self::OceanDepths,
        Self::CrystalTeal,
        Self::DeepLagoon,
        Self::AtlanticDeep,
        Self::MallardGreen,
        Self::Balsam,
        Self::Dragonfly,
        Self::GreenGables,
        Self::DeepTeal,
        Self::Pacific,
        Self::TealGreen,
        Self::UrbanChic,
        Self::SeaMoss,
        Self::DarkestSpruce,
        Self::Storm,
        Self::BotanicalGarden,
        Self::Everglade,
        Self::ForestBiome,
        Self::RainForest,
        Self::PonderosaPine,
        Self::Scarab,
        Self::PineGrove,
        Self::BistroGreen,
        Self::TrekkingGreen,
        Self::Jasper,
        Self::JuneBug,
        Self::Evergreen,
        Self::HunterGreen,
        Self::DarkGreen,
        Self::Cilantro,
        Self::JetSet,
        Self::JungleGreen,
        Self::Sycamore,
        Self::MountainView,
        Self::Pineneedle,
        Self::VerdantGreen,
        Self::Eden,
        Self::DeepForest,
        Self::GreenerPastures,
    ];

    /// The `NN-NNNN` color code (`"19-4057"`).
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Egret => "11-0103",
            Self::VanillaIce => "11-0104",
            Self::AntiqueWhite => "11-0105",
            Self::SweetCream => "11-0106",
            Self::Papyrus => "11-0107",
            Self::Buttercream => "11-0110",
            Self::GlassGreen => "11-0205",
            Self::WaterLily => "11-0304",
            Self::SylvanGreen => "11-0410",
            Self::WinterWhite => "11-0507",
            Self::Afterglow => "11-0510",
            Self::LemonIcing => "11-0515",
            Self::BrightWhite => "11-0601",
            Self::SnowWhite => "11-0602",
            Self::PastelParchment => "11-0603",
            Self::Gardenia => "11-0604",
            Self::JetStream => "11-0605",
            Self::Pristine => "11-0606",
            Self::SugarSwizzle => "11-0607",
            Self::CoconutMilk => "11-0608",
            Self::EtherealGreen => "11-0609",
            Self::PearSorbet => "11-0615",
            Self::PastelYellow => "11-0616",
            Self::TransparentYellow => "11-0617",
            Self::WaxYellow => "11-0618",
            Self::Flan => "11-0619",
            Self::ElfinYellow => "11-0620",
            Self::YellowIris => "11-0622",
            Self::YellowPear => "11-0623",
            Self::WhisperWhite => "11-0701",
            Self::TenderYellow => "11-0710",
            Self::Ecru => "11-0809",
            Self::PearledIvory => "11-0907",
            Self::WhiteAlyssum => "11-1001",
            Self::BridalBlush => "11-1005",
            Self::SeaSalt => "11-1302",
            Self::AngelWing => "11-1305",
            Self::CreamPink => "11-1306",
            Self::PowderPuff => "11-1404",
            Self::Rosewater => "11-1408",
            Self::PetalPink => "11-2309",
            Self::Delicacy => "11-2409",
            Self::ShrinkingViolet => "11-2511",
            Self::BrilliantWhite => "11-4001",
            Self::CloudDancer => "11-4201",
            Self::StarWhite => "11-4202",
            Self::Marshmallow => "11-4300",
            Self::LilyWhite => "11-4301",
            Self::CannoliCream => "11-4302",
            Self::MysticBlue => "11-4303",
            Self::BitOfBlue => "11-4601",
            Self::BillowingSail => "11-4604",
            Self::BlancDeBlanc => "11-4800",
            Self::Tofu => "11-4801",
            Self::SummerShower => "11-4802",
            Self::Ice => "11-4803",
            Self::LightestSky => "11-4804",
            Self::HintOfMint => "11-4805",
            Self::WhiteSwan => "12-0000",
            Self::WhiteAsparagus => "12-0104",
            Self::BoneWhite => "12-0105",
            Self::MeadowMist => "12-0106",
            Self::CanaryGreen => "12-0108",
            Self::Ambrosia => "12-0109",
            Self::PistachioShell => "12-0110",
            Self::PatinaGreen => "12-0225",
            Self::WhitecapGray => "12-0304",
            Self::AsparagusGreen => "12-0311",
            Self::LimeCream => "12-0312",
            Self::SeafoamGreen => "12-0313",
            Self::WhiteJade => "12-0315",
            Self::Gleam => "12-0317",
            Self::Butterfly => "12-0322",
            Self::LightGray => "12-0404",
            Self::Hay => "12-0418",
            Self::MellowGreen => "12-0426",
            Self::DaiquiriGreen => "12-0435",
            Self::PaleLimeYellow => "12-0520",
            Self::YoungWheat => "12-0521",
            Self::Citron => "12-0524",
            Self::LuminaryGreen => "12-0525",
            Self::Charlock => "12-0530",
            Self::Eggnog => "12-0601",
            Self::Angora => "12-0605",
            Self::GreenEssence => "12-0607",
            Self::DustyYellow => "12-0619",
            Self::LemonGrass => "12-0626",
            Self::CanaryYellow => "12-0633",
            Self::Aurora => "12-0642",
            Self::BlazingYellow => "12-0643",
            Self::Celandine => "12-0646",
            Self::Seedpearl => "12-0703",
            Self::WhiteSmoke => "12-0704",
            Self::Macadamia => "12-0709",
            Self::Navajo => "12-0710",
            Self::LemonMeringue => "12-0711",
            Self::Vanilla => "12-0712",
            Self::AlmondOil => "12-0713",
            Self::Cornhusk => "12-0714",
            Self::DoubleCream => "12-0715",
            Self::AniseFlower => "12-0717",
            Self::PineappleSlice => "12-0718",
            Self::MellowYellow => "12-0720",
            Self::Lemonade => "12-0721",
            Self::FrenchVanilla => "12-0722",
            Self::Sunshine => "12-0727",
            Self::Sundress => "12-0729",
            Self::LemonDrop => "12-0736",
            Self::Goldfinch => "12-0737",
            Self::YellowCream => "12-0738",
            Self::Limelight => "12-0740",
            Self::SunnyLime => "12-0741",
            Self::LemonVerbena => "12-0742",
            Self::Buttercup => "12-0752",
            Self::Yarrow => "12-0758",
            Self::CloudCream => "12-0804",
            Self::Rutabaga => "12-0806",
            Self::SunKiss => "12-0807",
            Self::Dawn => "12-0811",
            Self::AlabasterGleam => "12-0812",
            Self::AutumnBlonde => "12-0813",
            Self::VanillaCustard => "12-0815",
            Self::ApricotGelato => "12-0817",
            Self::GoldenFleece => "12-0822",
            Self::PaleBanana => "12-0824",
            Self::Popcorn => "12-0825",
            Self::GoldenHaze => "12-0826",
            Self::LambsWool => "12-0910",
            Self::Nude => "12-0911",
            Self::TenderPeach => "12-0912",
            Self::Alesan => "12-0913",
            Self::PalePeach => "12-0915",
            Self::BleachedApricot => "12-0917",
            Self::GoldenStraw => "12-0921",
            Self::NovellePeach => "12-1005",
            Self::MotherOfPearl => "12-1006",
            Self::PastelRoseTan => "12-1007",
            Self::Linen => "12-1008",
            Self::VanillaCream => "12-1009",
            Self::ScallopShell => "12-1010",
            Self::PeachPuree => "12-1011",
            Self::SheerPink => "12-1106",
            Self::PeachDust => "12-1107",
            Self::Dew => "12-1108",
            Self::CremeDePeche => "12-1110",
            Self::SilverPeony => "12-1206",
            Self::PearlBlush => "12-1207",
            Self::SoftPink => "12-1209",
            Self::VeiledRose => "12-1212",
            Self::Pearl => "12-1304",
            Self::HeavenlyPink => "12-1305",
            Self::BlushingBride => "12-1310",
            Self::Tapioca => "12-1403",
            Self::PinkTint => "12-1404",
            Self::CrystalPink => "12-1605",
            Self::PinkDogwood => "12-1706",
            Self::CrystalRose => "12-1708",
            Self::MarysRose => "12-1813",
            Self::MauveMorn => "12-2102",
            Self::AlmostMauve => "12-2103",
            Self::Morganite => "12-2901",
            Self::MauveChalk => "12-2902",
            Self::LightLilac => "12-2903",
            Self::PrimrosePink => "12-2904",
            Self::CradlePink => "12-2905",
            Self::BarelyPink => "12-2906",
            Self::RoseWater => "12-2907",
            Self::AlmondMilk => "12-4301",
            Self::VaporousGray => "12-4302",
            Self::Bluewash => "12-4304",
            Self::SpaBlue => "12-4305",
            Self::BarelyBlue => "12-4306",
            Self::Skylight => "12-4604",
            Self::PastelBlue => "12-4607",
            Self::Clearwater => "12-4608",
            Self::StarlightBlue => "12-4609",
            Self::WhisperingBlue => "12-4610",
            Self::BlueBlush => "12-4705",
            Self::WanBlue => "12-4805",
            Self::ChalkBlue => "12-4806",
            Self::Icicle => "12-5201",
            Self::Turtledove => "12-5202",
            Self::Murmur => "12-5203",
            Self::MorningMist => "12-5204",
            Self::BlueGlass => "12-5206",
            Self::SoothingSea => "12-5209",
            Self::SproutGreen => "12-5303",
            Self::BlueFlower => "12-5403",
            Self::WhisperGreen => "12-5404",
            Self::OpalBlue => "12-5406",
            Self::AquaGlass => "12-5407",
            Self::MoonlightJade => "12-5408",
            Self::FairAqua => "12-5409",
            Self::BleachedAqua => "12-5410",
            Self::ClearlyAqua => "12-5504",
            Self::Glacier => "12-5505",
            Self::DustyAqua => "12-5506",
            Self::Bay => "12-5507",
            Self::HushedGreen => "12-5508",
            Self::ZephyrBlue => "12-5603",
            Self::Honeydew => "12-5808",
            Self::SilverGreen => "12-6204",
            Self::MilkyGreen => "12-6205",
            Self::FairestJade => "12-6206",
            Self::Frost => "12-6207",
            Self::PhantomGreen => "12-6208",
            Self::Moonbeam => "13-0000",
            Self::WhiteSand => "13-0002",
            Self::Dewkist => "13-0107",
            Self::Seacrest => "13-0111",
            Self::PastelGreen => "13-0116",
            Self::GreenAsh => "13-0117",
            Self::FogGreen => "13-0210",
            Self::TenderGreens => "13-0212",
            Self::Reed => "13-0215",
            Self::ParadiseGreen => "13-0220",
            Self::PistachioGreen => "13-0221",
            Self::LilyGreen => "13-0317",
            Self::ShadowLime => "13-0319",
            Self::LettuceGreen => "13-0324",
            Self::SapGreen => "13-0331",
            Self::LimaBean => "13-0333",
            Self::Oatmeal => "13-0401",
            Self::GrayMorn => "13-0403",
            Self::GreenGlow => "13-0442",
            Self::LimePopsicle => "13-0443",
            Self::FrozenDew => "13-0513",
            Self::PaleGreen => "13-0522",
            Self::LimeSherbet => "13-0530",
            Self::CeleryGreen => "13-0532",
            Self::SharpGreen => "13-0535",
            Self::WildLime => "13-0540",
            Self::LimePunch => "13-0550",
            Self::Fog => "13-0607",
            Self::AloeWash => "13-0608",
            Self::Moth => "13-0611",
            Self::ChinoGreen => "13-0613",
            Self::GardenGlade => "13-0614",
            Self::GoldenMist => "13-0624",
            Self::Endive => "13-0632",
            Self::Chardonnay => "13-0633",
            Self::Acacia => "13-0640",
            Self::GoldenKiwi => "13-0644",
            Self::Limeade => "13-0645",
            Self::Meadowlark => "13-0646",
            Self::GreenSheen => "13-0648",
            Self::SulphurSpring => "13-0650",
            Self::EveningPrimrose => "13-0651",
            Self::Putty => "13-0711",
            Self::SeaMist => "13-0715",
            Self::Custard => "13-0720",
            Self::Raffia => "13-0725",
            Self::CreamGold => "13-0739",
            Self::Maize => "13-0746",
            Self::Lemon => "13-0752",
            Self::PrimroseYellow => "13-0755",
            Self::LemonZest => "13-0756",
            Self::Dandelion => "13-0758",
            Self::SolarPower => "13-0759",
            Self::SummerMelon => "13-0814",
            Self::BananaCrepe => "13-0815",
            Self::Sunlight => "13-0822",
            Self::Snapdragon => "13-0840",
            Self::HabaneroGold => "13-0849",
            Self::AspenGold => "13-0850",
            Self::MinionYellow => "13-0851",
            Self::VibrantYellow => "13-0858",
            Self::LemonChrome => "13-0859",
            Self::Birch => "13-0905",
            Self::Sandshell => "13-0907",
            Self::Parchment => "13-0908",
            Self::ReedYellow => "13-0915",
            Self::Chamomile => "13-0916",
            Self::ItalianStraw => "13-0917",
            Self::Soybean => "13-0919",
            Self::Straw => "13-0922",
            Self::Cornsilk => "13-0932",
            Self::Flax => "13-0935",
            Self::GoldenCream => "13-0939",
            Self::SunsetGold => "13-0940",
            Self::BananaCream => "13-0941",
            Self::AmberYellow => "13-0942",
            Self::PaleMarigold => "13-0945",
            Self::JurassicGold => "13-0946",
            Self::Banana => "13-0947",
            Self::CremeBrulee => "13-1006",
            Self::OysterWhite => "13-1007",
            Self::BleachedSand => "13-1008",
            Self::Biscotti => "13-1009",
            Self::GraySand => "13-1010",
            Self::IvoryCream => "13-1011",
            Self::FrostedAlmond => "13-1012",
            Self::Appleblossom => "13-1013",
            Self::MellowBuff => "13-1014",
            Self::HoneyPeach => "13-1015",
            Self::Wheat => "13-1016",
            Self::AlmondCream => "13-1017",
            Self::DesertDust => "13-1018",
            Self::CreamBlush => "13-1019",
            Self::ApricotIce => "13-1020",
            Self::PrairieSunset => "13-1021",
            Self::CaramelCream => "13-1022",
            Self::PeachFuzz => "13-1023",
            Self::Buff => "13-1024",
            Self::Impala => "13-1025",
            Self::Creampuff => "13-1026",
            Self::ApricotCream => "13-1027",
            Self::Sunburst => "13-1030",
            Self::ApricotSherbet => "13-1031",
            Self::BrownRice => "13-1105",
            Self::SandDollar => "13-1106",
            Self::WhisperPink => "13-1107",
            Self::CreamTan => "13-1108",
            Self::Bisque => "13-1109",
            Self::Bellini => "13-1114",
            Self::PeachQuartz => "13-1125",
            Self::PeachyKeen => "13-1208",
            Self::BrazilianSand => "13-1308",
            Self::EnglishRose => "13-1310",
            Self::TropicalPeach => "13-1318",
            Self::PaleDogwood => "13-1404",
            Self::Shell => "13-1405",
            Self::CloudPink => "13-1406",
            Self::CreolePink => "13-1407",
            Self::ChintzRose => "13-1408",
            Self::SeashellPink => "13-1409",
            Self::PeachBlush => "13-1504",
            Self::ImpatiensPink => "13-1510",
            Self::PinkSalt => "13-1511",
            Self::GossamerPink => "13-1513",
            Self::RoseQuartz => "13-1520",
            Self::ChalkPink => "13-1904",
            Self::RoseShadow => "13-1906",
            Self::Potpourri => "13-2004",
            Self::StrawberryCream => "13-2005",
            Self::AlmondBlossom => "13-2006",
            Self::OrchidPink => "13-2010",
            Self::FairyTale => "13-2802",
            Self::PaleLilac => "13-2803",
            Self::ParfaitPink => "13-2804",
            Self::PinkMist => "13-2805",
            Self::PinkLady => "13-2806",
            Self::Ballerina => "13-2807",
            Self::BalletSlipper => "13-2808",
            Self::CherryBlossom => "13-3207",
            Self::LilacSnow => "13-3405",
            Self::OrchidIce => "13-3406",
            Self::CrystalGray => "13-3801",
            Self::OrchidTint => "13-3802",
            Self::LilacAsh => "13-3803",
            Self::GrayLilac => "13-3804",
            Self::OrchidHush => "13-3805",
            Self::LavenderFog => "13-3820",
            Self::HalogenBlue => "13-3920",
            Self::IllusionBlue => "13-4103",
            Self::Antarctica => "13-4104",
            Self::LilacHint => "13-4105",
            Self::NimbusCloud => "13-4108",
            Self::ArcticIce => "13-4110",
            Self::PleinAir => "13-4111",
            Self::Omphalodes => "13-4200",
            Self::OysterMushroom => "13-4201",
            Self::DawnBlue => "13-4303",
            Self::BalladBlue => "13-4304",
            Self::FoggyDew => "13-4305",
            Self::BabyBlue => "13-4308",
            Self::SilverBirch => "13-4403",
            Self::IceFlow => "13-4404",
            Self::MistyBlue => "13-4405",
            Self::BlueGlow => "13-4409",
            Self::CrystalBlue => "13-4411",
            Self::TanagerTurquoise => "13-4720",
            Self::PaleBlue => "13-4804",
            Self::Plume => "13-4809",
            Self::LimpetShell => "13-4810",
            Self::BlueLight => "13-4909",
            Self::BlueTint => "13-4910",
            Self::RainyDay => "13-5304",
            Self::PaleAqua => "13-5305",
            Self::IcyMorn => "13-5306",
            Self::PastelTurquoise => "13-5309",
            Self::ArubaBlue => "13-5313",
            Self::Yucca => "13-5409",
            Self::IcedAqua => "13-5410",
            Self::BeachGlass => "13-5412",
            Self::IceGreen => "13-5414",
            Self::Cabbage => "13-5714",
            Self::GossamerGreen => "13-5907",
            Self::BirdsEggGreen => "13-5911",
            Self::AlmostAqua => "13-6006",
            Self::Spray => "13-6007",
            Self::MistyJade => "13-6008",
            Self::BrookGreen => "13-6009",
            Self::CeladonTint => "13-6105",
            Self::GreenTint => "13-6106",
            Self::GreenLily => "13-6107",
            Self::Celadon => "13-6108",
            Self::MistGreen => "13-6110",
            Self::BokChoy => "13-6208",
            Self::SilverGray => "14-0000",
            Self::PumiceStone => "14-0002",
            Self::Overcast => "14-0105",
            Self::CastleWall => "14-0108",
            Self::CeladonGreen => "14-0114",
            Self::FoamGreen => "14-0115",
            Self::Margarita => "14-0116",
            Self::NileGreen => "14-0121",
            Self::ArcadianGreen => "14-0123",
            Self::Greengage => "14-0127",
            Self::SummerGreen => "14-0156",
            Self::TidalFoam => "14-0210",
            Self::Lint => "14-0216",
            Self::Seedling => "14-0217",
            Self::Nile => "14-0223",
            Self::OpalineGreen => "14-0226",
            Self::JadeLime => "14-0232",
            Self::BrightLimeGreen => "14-0244",
            Self::AcidLime => "14-0340",
            Self::Bog => "14-0418",
            Self::Beechnut => "14-0425",
            Self::GreenBanana => "14-0434",
            Self::BrightChartreuse => "14-0445",
            Self::TenderShoots => "14-0446",
            Self::LimeGreen => "14-0452",
            Self::GreenHaze => "14-0615",
            Self::DriedMoss => "14-0626",
            Self::ShadowGreen => "14-0627",
            Self::MutedLime => "14-0636",
            Self::Celery => "14-0647",
            Self::Cement => "14-0708",
            Self::Hemp => "14-0721",
            Self::Bamboo => "14-0740",
            Self::SuperLemon => "14-0754",
            Self::Sulphur => "14-0755",
            Self::EmpireYellow => "14-0756",
            Self::CyberYellow => "14-0760",
            Self::Pampas => "14-0826",
            Self::DuskyCitron => "14-0827",
            Self::MistedYellow => "14-0837",
            Self::YolkYellow => "14-0846",
            Self::BuffYellow => "14-0847",
            Self::Mimosa => "14-0848",
            Self::Daffodil => "14-0850",
            Self::SamoanSun => "14-0851",
            Self::Freesia => "14-0852",
            Self::Parsnip => "14-0925",
            Self::Jojoba => "14-0935",
            Self::SaharaSun => "14-0936",
            Self::Beeswax => "14-0941",
            Self::GoldenRod => "14-0951",
            Self::SpicyMustard => "14-0952",
            Self::Citrus => "14-0955",
            Self::SpectraYellow => "14-0957",
            Self::GildedBeige => "14-1012",
            Self::Gravel => "14-1014",
            Self::Cocoon => "14-1025",
            Self::Rattan => "14-1031",
            Self::Ochre => "14-1036",
            Self::NewWheat => "14-1038",
            Self::GoldenApricot => "14-1041",
            Self::Amber => "14-1045",
            Self::Marigold => "14-1050",
            Self::WarmApricot => "14-1051",
            Self::Kumquat => "14-1052",
            Self::Saffron => "14-1064",
            Self::Peyote => "14-1106",
            Self::OysterGray => "14-1107",
            Self::WoodAsh => "14-1108",
            Self::Boulder => "14-1110",
            Self::Pebble => "14-1112",
            Self::Marzipan => "14-1113",
            Self::AlmondBuff => "14-1116",
            Self::Beige => "14-1118",
            Self::WinterWheat => "14-1119",
            Self::ApricotIllusion => "14-1120",
            Self::Sheepskin => "14-1122",
            Self::DesertMist => "14-1127",
            Self::BuffOrange => "14-1128",
            Self::ApricotNectar => "14-1133",
            Self::SalmonBuff => "14-1135",
            Self::Pumpkin => "14-1139",
            Self::Zinnia => "14-1159",
            Self::IrishCream => "14-1208",
            Self::SmokeGray => "14-1209",
            Self::ShiftingSand => "14-1210",
            Self::Frappe => "14-1212",
            Self::ToastedAlmond => "14-1213",
            Self::Amberlight => "14-1217",
            Self::PeachParfait => "14-1219",
            Self::PeachNougat => "14-1220",
            Self::CoralSands => "14-1224",
            Self::BeachSand => "14-1225",
            Self::Peach => "14-1227",
            Self::PeachNectar => "14-1228",
            Self::ApricotWash => "14-1230",
            Self::PeachCobbler => "14-1231",
            Self::OrangeChiffon => "14-1241",
            Self::Mushroom => "14-1305",
            Self::RoseDust => "14-1307",
            Self::PeachWhip => "14-1309",
            Self::CameoRose => "14-1310",
            Self::EveningSand => "14-1311",
            Self::PaleBlush => "14-1312",
            Self::RoseCloud => "14-1313",
            Self::SpanishVilla => "14-1314",
            Self::Hazelnut => "14-1315",
            Self::DustyPink => "14-1316",
            Self::CoralPink => "14-1318",
            Self::Salmon => "14-1323",
            Self::PeachBud => "14-1324",
            Self::PeachMelba => "14-1418",
            Self::PeachPearl => "14-1419",
            Self::ApricotBlush => "14-1420",
            Self::RoseSmoke => "14-1506",
            Self::SilverPink => "14-1508",
            Self::PowderPink => "14-1511",
            Self::Blossom => "14-1513",
            Self::PeachesNCream => "14-1521",
            Self::QuartzPink => "14-1714",
            Self::SepiaRose => "14-1803",
            Self::Lotus => "14-1905",
            Self::Peachskin => "14-1907",
            Self::CoralBlush => "14-1909",
            Self::CandyPink => "14-1911",
            Self::PinkNectar => "14-2305",
            Self::CameoPink => "14-2307",
            Self::PrismPink => "14-2311",
            Self::LilacSachet => "14-2710",
            Self::SweetLilac => "14-2808",
            Self::FragrantLilac => "14-3204",
            Self::WinsomeOrchid => "14-3206",
            Self::PinkLavender => "14-3207",
            Self::PastelLavender => "14-3209",
            Self::OrchidBloom => "14-3612",
            Self::OrchidPetal => "14-3710",
            Self::HushedViolet => "14-3803",
            Self::Iris => "14-3805",
            Self::PastelLilac => "14-3812",
            Self::LilacMarble => "14-3903",
            Self::EveningHaze => "14-3904",
            Self::LavenderBlue => "14-3905",
            Self::Raindrops => "14-3906",
            Self::Thistle => "14-3907",
            Self::PurpleHeather => "14-3911",
            Self::ZenBlue => "14-3912",
            Self::XenonBlue => "14-3949",
            Self::WindChime => "14-4002",
            Self::GlacierGray => "14-4102",
            Self::GrayViolet => "14-4103",
            Self::MicroChip => "14-4105",
            Self::GrayDawn => "14-4106",
            Self::QuietGray => "14-4107",
            Self::Heather => "14-4110",
            Self::Skyway => "14-4112",
            Self::CashmereBlue => "14-4115",
            Self::BlueBell => "14-4121",
            Self::AiryBlue => "14-4122",
            Self::LunarRock => "14-4201",
            Self::HarborMist => "14-4202",
            Self::VaporBlue => "14-4203",
            Self::PearlBlue => "14-4206",
            Self::CelestialBlue => "14-4210",
            Self::PowderBlue => "14-4214",
            Self::CloudBlue => "14-4306",
            Self::WinterSky => "14-4307",
            Self::BlueTopaz => "14-4310",
            Self::CorydalisBlue => "14-4311",
            Self::Aquamarine => "14-4313",
            Self::SeaAngel => "14-4315",
            Self::CoolBlue => "14-4317",
            Self::SkyBlue => "14-4318",
            Self::BalticSea => "14-4320",
            Self::Moonstruck => "14-4500",
            Self::SilverLining => "14-4501",
            Self::Mercury => "14-4502",
            Self::Metal => "14-4503",
            Self::SkyGray => "14-4504",
            Self::Smoke => "14-4505",
            Self::Ether => "14-4506",
            Self::Stratosphere => "14-4508",
            Self::Aquatic => "14-4510",
            Self::GulfStream => "14-4511",
            Self::PorcelainBlue => "14-4512",
            Self::PetitFour => "14-4516",
            Self::BachelorButton => "14-4522",
            Self::AntiquaSand => "14-4615",
            Self::IslandParadise => "14-4620",
            Self::TibetanStone => "14-4710",
            Self::BlueFox => "14-4804",
            Self::SurfSpray => "14-4807",
            Self::EggshellBlue => "14-4809",
            Self::CanalBlue => "14-4810",
            Self::AquaSky => "14-4811",
            Self::AquaSplash => "14-4812",
            Self::AngelBlue => "14-4814",
            Self::BlueRadiance => "14-4816",
            Self::HarborGray => "14-4908",
            Self::Silver => "14-5002",
            Self::Holiday => "14-5413",
            Self::Bermuda => "14-5416",
            Self::Cockatoo => "14-5420",
            Self::SiltGreen => "14-5706",
            Self::AquaFoam => "14-5707",
            Self::OceanWave => "14-5711",
            Self::Cascade => "14-5713",
            Self::BeveledGlass => "14-5714",
            Self::Opal => "14-5718",
            Self::ElectricGreen => "14-5721",
            Self::SeaFoam => "14-6007",
            Self::SubtleGreen => "14-6008",
            Self::GrayedJade => "14-6011",
            Self::NeptuneGreen => "14-6017",
            Self::Pelican => "14-6305",
            Self::Alfalfa => "14-6308",
            Self::CameoGreen => "14-6312",
            Self::Sprucestone => "14-6316",
            Self::Meadow => "14-6319",
            Self::Peapod => "14-6324",
            Self::ZephyrGreen => "14-6327",
            Self::AbsintheGreen => "14-6329",
            Self::SpringBud => "14-6330",
            Self::SpringBouquet => "14-6340",
            Self::AbbeyStone => "14-6408",
            Self::Dove => "15-0000",
            Self::GreenFlash => "15-0146",
            Self::SprayGreen => "15-0309",
            Self::SageGreen => "15-0318",
            Self::Tarragon => "15-0326",
            Self::LeafGreen => "15-0332",
            Self::HerbalGarden => "15-0336",
            Self::ParrotGreen => "15-0341",
            Self::Greenery => "15-0343",
            Self::Eucalyptus => "15-0513",
            Self::PaleOliveGreen => "15-0522",
            Self::WinterPear => "15-0523",
            Self::WeepingWillow => "15-0525",
            Self::SweetPea => "15-0531",
            Self::LindenGreen => "15-0533",
            Self::Palm => "15-0535",
            Self::GreenOasis => "15-0538",
            Self::AppleGreen => "15-0543",
            Self::JasmineGreen => "15-0545",
            Self::Citronelle => "15-0548",
            Self::LeekGreen => "15-0628",
            Self::GoldenGreen => "15-0636",
            Self::CressGreen => "15-0643",
            Self::WarmOlive => "15-0646",
            Self::AshesOfRoses => "15-0703",
            Self::SilverFern => "15-0719",
            Self::SouthernMoss => "15-0730",
            Self::Olivenite => "15-0732",
            Self::OilYellow => "15-0743",
            Self::LemonCurry => "15-0751",
            Self::CeylonYellow => "15-0850",
            Self::PaleGold => "15-0927",
            Self::Sauterne => "15-0942",
            Self::GoldenSpice => "15-0948",
            Self::GoldenYellow => "15-0953",
            Self::OldGold => "15-0955",
            Self::MangoMojito => "15-0960",
            Self::GingerRoot => "15-1020",
            Self::IcedCoffee => "15-1040",
            Self::AutumnBlaze => "15-1045",
            Self::MineralYellow => "15-1046",
            Self::ArtisansGold => "15-1049",
            Self::GoldenGlow => "15-1050",
            Self::GoldenOrange => "15-1051",
            Self::CadmiumYellow => "15-1054",
            Self::RadiantYellow => "15-1058",
            Self::GoldFusion => "15-1062",
            Self::Travertine => "15-1114",
            Self::Safari => "15-1116",
            Self::TaosTaupe => "15-1119",
            Self::Porcini => "15-1125",
            Self::FallLeaf => "15-1132",
            Self::HoneyGold => "15-1142",
            Self::Chamois => "15-1145",
            Self::Butterscotch => "15-1147",
            Self::DarkCheddar => "15-1150",
            Self::IcelandPoppy => "15-1151",
            Self::Apricot => "15-1153",
            Self::FlameOrange => "15-1157",
            Self::BlazingOrange => "15-1160",
            Self::BrightMarigold => "15-1164",
            Self::CandiedGinger => "15-1213",
            Self::WarmSand => "15-1214",
            Self::Sesame => "15-1215",
            Self::PaleKhaki => "15-1216",
            Self::MojaveDesert => "15-1217",
            Self::Semolina => "15-1218",
            Self::Latte => "15-1220",
            Self::Sand => "15-1225",
            Self::Clay => "15-1231",
            Self::GoldEarth => "15-1234",
            Self::ApricotTan => "15-1237",
            Self::Cantaloupe => "15-1239",
            Self::Muskmelon => "15-1242",
            Self::Papaya => "15-1243",
            Self::MockOrange => "15-1245",
            Self::Tangerine => "15-1247",
            Self::CarrotCurl => "15-1262",
            Self::AutumnGlory => "15-1263",
            Self::Turmeric => "15-1264",
            Self::Humus => "15-1304",
            Self::FeatherGray => "15-1305",
            Self::OxfordTan => "15-1306",
            Self::WhitePepper => "15-1307",
            Self::Doeskin => "15-1308",
            Self::Moonlight => "15-1309",
            Self::CubanSand => "15-1314",
            Self::RugbyTan => "15-1315",
            Self::MapleSugar => "15-1316",
            Self::Sirocco => "15-1317",
            Self::PinkSand => "15-1318",
            Self::AlmostApricot => "15-1319",
            Self::DustyCoral => "15-1322",
            Self::PeachBloom => "15-1327",
            Self::CoralReef => "15-1331",
            Self::CanyonSunset => "15-1333",
            Self::ShellCoral => "15-1334",
            Self::Tangelo => "15-1335",
            Self::CadmiumOrange => "15-1340",
            Self::CoralCloud => "15-1415",
            Self::PeachAmber => "15-1423",
            Self::Fenugreek => "15-1425",
            Self::DustedClay => "15-1429",
            Self::PastryShell => "15-1430",
            Self::PapayaPunch => "15-1433",
            Self::DesertFlower => "15-1435",
            Self::Etherea => "15-1506",
            Self::MahoganyRose => "15-1511",
            Self::MistyRose => "15-1512",
            Self::MellowRose => "15-1515",
            Self::PeachBeige => "15-1516",
            Self::BloomingDahlia => "15-1520",
            Self::Shrimp => "15-1523",
            Self::PeachPink => "15-1530",
            Self::PaleMauve => "15-1607",
            Self::BridalRose => "15-1611",
            Self::Blush => "15-1614",
            Self::CandlelightPeach => "15-1621",
            Self::ConchShell => "15-1624",
            Self::SalmonRose => "15-1626",
            Self::PinkIcing => "15-1717",
            Self::Peony => "15-1816",
            Self::FlamingoPink => "15-1821",
            Self::BurnishedLilac => "15-1905",
            Self::Zephyr => "15-1906",
            Self::SeaPink => "15-1912",
            Self::MorningGlory => "15-1920",
            Self::GeraniumPink => "15-1922",
            Self::DawnPink => "15-2205",
            Self::OrchidSmoke => "15-2210",
            Self::Rosebloom => "15-2214",
            Self::BegoniaPink => "15-2215",
            Self::SachetPink => "15-2216",
            Self::AuroraPink => "15-2217",
            Self::KeepsakeLilac => "15-2705",
            Self::VioletIce => "15-2706",
            Self::FuchsiaPink => "15-2718",
            Self::LilacChiffon => "15-2913",
            Self::MauveMist => "15-3207",
            Self::Orchid => "15-3214",
            Self::OrchidBouquet => "15-3412",
            Self::LavenderFrost => "15-3507",
            Self::FairOrchid => "15-3508",
            Self::CrocusPetal => "15-3520",
            Self::Lavendula => "15-3620",
            Self::PurpleRose => "15-3716",
            Self::LilacBreeze => "15-3720",
            Self::Porpoise => "15-3800",
            Self::CloudGray => "15-3802",
            Self::MistyLilac => "15-3807",
            Self::Lavender => "15-3817",
            Self::IcelandicBlue => "15-3908",
            Self::CosmicSky => "15-3909",
            Self::LanguidLavender => "15-3910",
            Self::Aleutian => "15-3912",
            Self::KentuckyBlue => "15-3915",
            Self::Serenity => "15-3919",
            Self::PlacidBlue => "15-3920",
            Self::VistaBlue => "15-3930",
            Self::BelAirBlue => "15-3932",
            Self::StormGray => "15-4003",
            Self::DreamBlue => "15-4005",
            Self::BlueFog => "15-4008",
            Self::Cerulean => "15-4020",
            Self::ChambrayBlue => "15-4030",
            Self::HighRise => "15-4101",
            Self::AngelFalls => "15-4105",
            Self::AlaskanBlue => "15-4225",
            Self::Quarry => "15-4305",
            Self::BelgianBlock => "15-4306",
            Self::Tradewinds => "15-4307",
            Self::SterlingBlue => "15-4309",
            Self::ForgetMeNot => "15-4312",
            Self::AirBlue => "15-4319",
            Self::EtherealBlue => "15-4323",
            Self::MilkyBlue => "15-4415",
            Self::BlueGrotto => "15-4421",
            Self::NorseBlue => "15-4427",
            Self::CrystalSeas => "15-4428",
            Self::SilverCloud => "15-4502",
            Self::ChateauGray => "15-4503",
            Self::PuritanGray => "15-4702",
            Self::MirageGray => "15-4703",
            Self::Pigeon => "15-4704",
            Self::GrayMist => "15-4706",
            Self::BlueHaze => "15-4707",
            Self::MarineBlue => "15-4712",
            Self::Aquarelle => "15-4714",
            Self::AquaSea => "15-4715",
            Self::Aqua => "15-4717",
            Self::RiverBlue => "15-4720",
            Self::Capri => "15-4722",
            Self::BlueCuracao => "15-4825",
            Self::AquaGray => "15-5205",
            Self::Aquifer => "15-5207",
            Self::AquaHaze => "15-5209",
            Self::NileBlue => "15-5210",
            Self::BlueTurquoise => "15-5217",
            Self::PoolBlue => "15-5218",
            Self::FloridaKeys => "15-5416",
            Self::AquaGreen => "15-5421",
            Self::Atlantis => "15-5425",
            Self::Waterfall => "15-5516",
            Self::Turquoise => "15-5519",
            Self::BrightGreen => "15-5534",
            Self::MineralGray => "15-5704",
            Self::FrostyGreen => "15-5706",
            Self::DustyJadeGreen => "15-5711",
            Self::BiscayGreen => "15-5718",
            Self::MintLeaf => "15-5728",
            Self::Lichen => "15-5812",
            Self::Spearmint => "15-5819",
            Self::Hemlock => "15-6114",
            Self::Ming => "15-6120",
            Self::JadeCream => "15-6123",
            Self::PussywillowGray => "15-6304",
            Self::AgateGray => "15-6307",
            Self::Swamp => "15-6310",
            Self::LaurelGreen => "15-6313",
            Self::SmokeGreen => "15-6315",
            Self::FairGreen => "15-6316",
            Self::QuietGreen => "15-6317",
            Self::LightGrassGreen => "15-6322",
            Self::IrishGreen => "15-6340",
            Self::MossGray => "15-6410",
            Self::Reseda => "15-6414",
            Self::ForestShade => "15-6423",
            Self::GreenTea => "15-6428",
            Self::Shamrock => "15-6432",
            Self::GrassGreen => "15-6437",
            Self::BudGreen => "15-6442",
            Self::Paloma => "16-0000",
            Self::DesertSage => "16-0110",
            Self::Tendril => "16-0123",
            Self::VintageKhaki => "16-0205",
            Self::LondonFog => "16-0207",
            Self::Tea => "16-0213",
            Self::Mistletoe => "16-0220",
            Self::GreenEyes => "16-0224",
            Self::JadeGreen => "16-0228",
            Self::MacawGreen => "16-0230",
            Self::MeadowGreen => "16-0233",
            Self::Kiwi => "16-0235",
            Self::Foliage => "16-0237",
            Self::Sage => "16-0421",
            Self::Fern => "16-0430",
            Self::DarkCitron => "16-0435",
            Self::SpinachGreen => "16-0439",
            Self::GrayGreen => "16-0518",
            Self::Cedar => "16-0526",
            Self::Moss => "16-0532",
            Self::Oasis => "16-0540",
            Self::GoldenLime => "16-0543",
            Self::SplitPea => "16-0545",
            Self::LentilSprout => "16-0550",
            Self::Elm => "16-0613",
            Self::Willow => "16-0632",
            Self::GoldenOlive => "16-0639",
            Self::SlateGreen => "16-0713",
            Self::Khaki => "16-0726",
            Self::AntiqueGold => "16-0730",
            Self::BurnishedGold => "16-0737",
            Self::GreenSulphur => "16-0742",
            Self::Goat => "16-0806",
            Self::RichGold => "16-0836",
            Self::AntiqueMoss => "16-0840",
            Self::OliveOil => "16-0847",
            Self::SimplyTaupe => "16-0906",
            Self::CurdsAndWhey => "16-0920",
            Self::Croissant => "16-0924",
            Self::Curry => "16-0928",
            Self::Taffy => "16-0940",
            Self::Tinsel => "16-0945",
            Self::Honey => "16-0946",
            Self::BrightGold => "16-0947",
            Self::HarvestGold => "16-0948",
            Self::Narcissus => "16-0950",
            Self::NuggetGold => "16-0952",
            Self::TawnyOlive => "16-0953",
            Self::Arrowwood => "16-0954",
            Self::Incense => "16-1010",
            Self::Sunflower => "16-1054",
            Self::PureCashmere => "16-1103",
            Self::Crockery => "16-1104",
            Self::PlazaTaupe => "16-1105",
            Self::Tuffet => "16-1106",
            Self::Aluminum => "16-1107",
            Self::Twill => "16-1108",
            Self::Greige => "16-1109",
            Self::OliveGray => "16-1110",
            Self::Sponge => "16-1118",
            Self::Starfish => "16-1120",
            Self::Antelope => "16-1126",
            Self::MustardGold => "16-1133",
            Self::AmberGold => "16-1139",
            Self::Yam => "16-1140",
            Self::GoldenNugget => "16-1142",
            Self::HoneyYellow => "16-1143",
            Self::OakBuff => "16-1144",
            Self::Nugget => "16-1148",
            Self::DesertSun => "16-1149",
            Self::Topaz => "16-1150",
            Self::OrangePepper => "16-1164",
            Self::LightTaupe => "16-1210",
            Self::Nomad => "16-1212",
            Self::Tuscany => "16-1219",
            Self::CafeCreme => "16-1220",
            Self::Roebuck => "16-1221",
            Self::Sandstorm => "16-1235",
            Self::OrangeOchre => "16-1253",
            Self::RussetOrange => "16-1255",
            Self::SunOrange => "16-1257",
            Self::HarvestPumpkin => "16-1260",
            Self::String => "16-1305",
            Self::Natural => "16-1310",
            Self::Cornstalk => "16-1315",
            Self::Brush => "16-1317",
            Self::WarmTaupe => "16-1318",
            Self::Nougat => "16-1320",
            Self::Macaroon => "16-1323",
            Self::Lark => "16-1324",
            Self::Copper => "16-1325",
            Self::PrairieSand => "16-1326",
            Self::ToastedNut => "16-1327",
            Self::Sandstone => "16-1328",
            Self::CoralHaze => "16-1329",
            Self::MutedClay => "16-1330",
            Self::Toast => "16-1331",
            Self::Pheasant => "16-1332",
            Self::Doe => "16-1333",
            Self::Tan => "16-1334",
            Self::Biscuit => "16-1336",
            Self::CoralGold => "16-1337",
            Self::CopperTan => "16-1338",
            Self::BrandiedMelon => "16-1340",
            Self::Butterum => "16-1341",
            Self::Buckskin => "16-1342",
            Self::AutumnSunset => "16-1343",
            Self::DustyOrange => "16-1344",
            Self::SunBaked => "16-1345",
            Self::GoldenOchre => "16-1346",
            Self::PeachCaramel => "16-1347",
            Self::TomatoCream => "16-1348",
            Self::CoralRose => "16-1349",
            Self::Amberglow => "16-1350",
            Self::PersimmonOrange => "16-1356",
            Self::BirdOfParadise => "16-1357",
            Self::OrangeTiger => "16-1358",
            Self::OrangePeel => "16-1359",
            Self::Nectarine => "16-1360",
            Self::Carrot => "16-1361",
            Self::VermillionOrange => "16-1362",
            Self::PuffinsBill => "16-1363",
            Self::VibrantOrange => "16-1364",
            Self::Atmosphere => "16-1406",
            Self::Cobblestone => "16-1407",
            Self::Stucco => "16-1412",
            Self::Chanterelle => "16-1414",
            Self::Almondine => "16-1415",
            Self::Cork => "16-1422",
            Self::Sunburn => "16-1429",
            Self::CanyonClay => "16-1431",
            Self::Almond => "16-1432",
            Self::CoralAlmond => "16-1434",
            Self::Carnelian => "16-1435",
            Self::Meerkat => "16-1438",
            Self::Caramel => "16-1439",
            Self::Langoustino => "16-1440",
            Self::Arabesque => "16-1441",
            Self::Melon => "16-1442",
            Self::ApricotBuff => "16-1443",
            Self::BurntOrange => "16-1448",
            Self::GoldFlame => "16-1449",
            Self::Flamingo => "16-1450",
            Self::Nasturtium => "16-1451",
            Self::Firecracker => "16-1452",
            Self::ExoticOrange => "16-1453",
            Self::JaffaOrange => "16-1454",
            Self::MandarinOrange => "16-1459",
            Self::DragonFire => "16-1460",
            Self::GoldenPoppy => "16-1462",
            Self::Bark => "16-1506",
            Self::AdobeRose => "16-1508",
            Self::ShadowGray => "16-1509",
            Self::Fawn => "16-1510",
            Self::RoseTan => "16-1511",
            Self::CameoBrown => "16-1516",
            Self::Rosette => "16-1518",
            Self::LobsterBisque => "16-1520",
            Self::RoseDawn => "16-1522",
            Self::TerraCotta => "16-1526",
            Self::BurntCoral => "16-1529",
            Self::Crabapple => "16-1532",
            Self::Coral => "16-1539",
            Self::Camellia => "16-1541",
            Self::FreshSalmon => "16-1542",
            Self::FusionCoral => "16-1543",
            Self::Persimmon => "16-1544",
            Self::CoralQuartz => "16-1545",
            Self::LivingCoral => "16-1546",
            Self::PeachEcho => "16-1548",
            Self::PurpleDove => "16-1606",
            Self::BrandiedApricot => "16-1610",
            Self::Mauveglow => "16-1617",
            Self::TeaRose => "16-1620",
            Self::Lantana => "16-1624",
            Self::PeachBlossom => "16-1626",
            Self::ShellPink => "16-1632",
            Self::SugarCoral => "16-1640",
            Self::GeorgiaPeach => "16-1641",
            Self::Sphinx => "16-1703",
            Self::DeauvilleMauve => "16-1707",
            Self::Lilas => "16-1708",
            Self::Foxglove => "16-1710",
            Self::Polignac => "16-1712",
            Self::WildRose => "16-1715",
            Self::StrawberryIce => "16-1720",
            Self::Confetti => "16-1723",
            Self::StrawberryPink => "16-1731",
            Self::PinkLemonade => "16-1735",
            Self::Woodrose => "16-1806",
            Self::OrchidHaze => "16-2107",
            Self::MauveOrchid => "16-2111",
            Self::WildOrchid => "16-2120",
            Self::PinkCarnation => "16-2124",
            Self::AzaleaPink => "16-2126",
            Self::CashmereRose => "16-2215",
            Self::MoonliteMauve => "16-2614",
            Self::SmokyGrape => "16-3110",
            Self::Crocus => "16-3115",
            Self::OperaMauve => "16-3116",
            Self::Cyclamen => "16-3118",
            Self::MauveShadows => "16-3205",
            Self::SeaFog => "16-3304",
            Self::LavenderMist => "16-3307",
            Self::LavenderHerb => "16-3310",
            Self::Violet => "16-3320",
            Self::VioletTulle => "16-3416",
            Self::AfricanViolet => "16-3520",
            Self::Lupine => "16-3521",
            Self::RegalOrchid => "16-3525",
            Self::SheerLilac => "16-3617",
            Self::SandVerbena => "16-3720",
            Self::Satellite => "16-3800",
            Self::OpalGray => "16-3801",
            Self::Ash => "16-3802",
            Self::GullGray => "16-3803",
            Self::Wisteria => "16-3810",
            Self::HeirloomLilac => "16-3812",
            Self::Viola => "16-3815",
            Self::Rhapsody => "16-3817",
            Self::VioletTulip => "16-3823",
            Self::SilverSconce => "16-3850",
            Self::LilacGray => "16-3905",
            Self::DappleGray => "16-3907",
            Self::LavenderAura => "16-3911",
            Self::Alloy => "16-3915",
            Self::Sleet => "16-3916",
            Self::Eventide => "16-3919",
            Self::LavenderLustre => "16-3920",
            Self::BlueHeron => "16-3921",
            Self::BrunneraBlue => "16-3922",
            Self::EasterEgg => "16-3925",
            Self::Grapemist => "16-3929",
            Self::ThistleDown => "16-3930",
            Self::SweetLavender => "16-3931",
            Self::DustyBlue => "16-4010",
            Self::AshleyBlue => "16-4013",
            Self::ForeverBlue => "16-4019",
            Self::DellaRobbiaBlue => "16-4020",
            Self::Allure => "16-4021",
            Self::Hydrangea => "16-4030",
            Self::CornflowerBlue => "16-4031",
            Self::Provence => "16-4032",
            Self::GranadaSky => "16-4033",
            Self::Arona => "16-4109",
            Self::StoneBlue => "16-4114",
            Self::DuskBlue => "16-4120",
            Self::HeritageBlue => "16-4127",
            Self::LittleBoyBlue => "16-4132",
            Self::BonnieBlue => "16-4134",
            Self::MourningDove => "16-4400",
            Self::Drizzle => "16-4402",
            Self::Abyss => "16-4404",
            Self::Slate => "16-4408",
            Self::Tourmaline => "16-4411",
            Self::CameoBlue => "16-4414",
            Self::BlueMist => "16-4421",
            Self::HorizonBlue => "16-4427",
            Self::DelphiniumBlue => "16-4519",
            Self::MauiBlue => "16-4525",
            Self::CyanBlue => "16-4529",
            Self::Aquarius => "16-4530",
            Self::BlueAtoll => "16-4535",
            Self::Stillwater => "16-4610",
            Self::ReefWaters => "16-4612",
            Self::Limestone => "16-4702",
            Self::GhostGray => "16-4703",
            Self::SilverBlue => "16-4706",
            Self::MineralBlue => "16-4712",
            Self::Porcelain => "16-4719",
            Self::ScubaBlue => "16-4725",
            Self::PeacockBlue => "16-4728",
            Self::Bluebird => "16-4834",
            Self::WetWeather => "16-5101",
            Self::BlueSurf => "16-5106",
            Self::Wasabi => "16-5109",
            Self::Canton => "16-5112",
            Self::DustyTurquoise => "16-5114",
            Self::SeaBlue => "16-5119",
            Self::Meadowbrook => "16-5121",
            Self::Baltic => "16-5123",
            Self::Ceramic => "16-5127",
            Self::Jadeite => "16-5304",
            Self::AgateGreen => "16-5412",
            Self::Lagoon => "16-5418",
            Self::SeaGreen => "16-5421",
            Self::BrightAqua => "16-5422",
            Self::PoolGreen => "16-5425",
            Self::Billiard => "16-5427",
            Self::PeacockGreen => "16-5431",
            Self::BerylGreen => "16-5515",
            Self::Arcadia => "16-5533",
            Self::MarineGreen => "16-5721",
            Self::FlintGray => "16-5803",
            Self::SlateGray => "16-5804",
            Self::GreenMilieu => "16-5806",
            Self::LilyPad => "16-5807",
            Self::IcebergGreen => "16-5808",
            Self::GreenBay => "16-5810",
            Self::Feldspar => "16-5815",
            Self::GreenSpruce => "16-5820",
            Self::GumdropGreen => "16-5825",
            Self::WroughtIron => "16-5904",
            Self::GraniteGreen => "16-5907",
            Self::MalachiteGreen => "16-5917",
            Self::CremeDeMenthe => "16-5919",
            Self::WinterGreen => "16-5924",
            Self::MingGreen => "16-5930",
            Self::HollyGreen => "16-5932",
            Self::Mint => "16-5938",
            Self::Blarney => "16-5942",
            Self::Seagrass => "16-6008",
            Self::Katydid => "16-6030",
            Self::ShaleGreen => "16-6116",
            Self::Greenbriar => "16-6127",
            Self::KellyGreen => "16-6138",
            Self::Basil => "16-6216",
            Self::IslandGreen => "16-6240",
            Self::MineralGreen => "16-6318",
            Self::Jadesheen => "16-6324",
            Self::Peppermint => "16-6329",
            Self::VibrantGreen => "16-6339",
            Self::ClassicGreen => "16-6340",
            Self::PoisonGreen => "16-6444",
            Self::FrostGray => "17-0000",
            Self::OilGreen => "17-0115",
            Self::TurfGreen => "17-0119",
            Self::StoneGreen => "17-0123",
            Self::FluoriteGreen => "17-0133",
            Self::OnlineLime => "17-0145",
            Self::ElephantSkin => "17-0205",
            Self::RockRidge => "17-0207",
            Self::LodenFrost => "17-0210",
            Self::AspenGreen => "17-0215",
            Self::Watercress => "17-0220",
            Self::ForestGreen => "17-0230",
            Self::PiquantGreen => "17-0235",
            Self::Epsom => "17-0324",
            Self::TurtleGreen => "17-0330",
            Self::Peridot => "17-0336",
            Self::SilverSage => "17-0510",
            Self::DuskyGreen => "17-0517",
            Self::Mosstone => "17-0525",
            Self::Guacamole => "17-0530",
            Self::GreenOlive => "17-0535",
            Self::LaurelOak => "17-0610",
            Self::Vetiver => "17-0613",
            Self::Mermaid => "17-0618",
            Self::Aloe => "17-0620",
            Self::Boa => "17-0625",
            Self::DriedHerb => "17-0627",
            Self::TreeHouse => "17-0630",
            Self::GreenMoss => "17-0636",
            Self::TaupeGray => "17-0808",
            Self::EcruOlive => "17-0836",
            Self::GoldenPalm => "17-0839",
            Self::AmberGreen => "17-0840",
            Self::BronzeMist => "17-0843",
            Self::Fossil => "17-0909",
            Self::FennelSeed => "17-0929",
            Self::DullGold => "17-0935",
            Self::MedalBronze => "17-0942",
            Self::ChaiTea => "17-0949",
            Self::Dune => "17-1009",
            Self::Elmwood => "17-1019",
            Self::Kelp => "17-1022",
            Self::AntiqueBronze => "17-1028",
            Self::Bistre => "17-1036",
            Self::TigersEye => "17-1038",
            Self::SpruceYellow => "17-1040",
            Self::Chipmunk => "17-1044",
            Self::AppleCinnamon => "17-1045",
            Self::GoldenOak => "17-1046",
            Self::HoneyMustard => "17-1047",
            Self::IncaGold => "17-1048",
            Self::RoastedPecan => "17-1052",
            Self::RoastedCashew => "17-1105",
            Self::SenecaRock => "17-1107",
            Self::WinterTwig => "17-1108",
            Self::Chinchilla => "17-1109",
            Self::Coriander => "17-1113",
            Self::PetrifiedOak => "17-1115",
            Self::LeadGray => "17-1118",
            Self::Dijon => "17-1125",
            Self::BoneBrown => "17-1128",
            Self::WoodThrush => "17-1129",
            Self::BrownSugar => "17-1134",
            Self::Cashew => "17-1137",
            Self::Marmalade => "17-1140",
            Self::ArganOil => "17-1142",
            Self::Hazel => "17-1143",
            Self::AutumnMaple => "17-1145",
            Self::AmberBrown => "17-1147",
            Self::MoonRock => "17-1210",
            Self::Fungi => "17-1212",
            Self::Praline => "17-1223",
            Self::Camel => "17-1224",
            Self::TawnyBirch => "17-1225",
            Self::TawnyBrown => "17-1226",
            Self::CafeAuLait => "17-1227",
            Self::MochaMousse => "17-1230",
            Self::TimberWolf => "17-1310",
            Self::DesertTaupe => "17-1311",
            Self::SilverMink => "17-1312",
            Self::SepiaTint => "17-1314",
            Self::Portabella => "17-1316",
            Self::Amphora => "17-1319",
            Self::Tannin => "17-1320",
            Self::Woodsmoke => "17-1321",
            Self::Burro => "17-1322",
            Self::TobaccoBrown => "17-1327",
            Self::IndianTan => "17-1328",
            Self::Lion => "17-1330",
            Self::Bran => "17-1336",
            Self::Adobe => "17-1340",
            Self::TawnyOrange => "17-1341",
            Self::Autumnal => "17-1342",
            Self::SpiceRoute => "17-1345",
            Self::AutumnLeaf => "17-1347",
            Self::Exuberance => "17-1349",
            Self::OrangePopsicle => "17-1350",
            Self::ApricotOrange => "17-1353",
            Self::CelosiaOrange => "17-1360",
            Self::ScarletIbis => "17-1361",
            Self::PineBark => "17-1410",
            Self::BeaverFur => "17-1417",
            Self::GingerSnap => "17-1418",
            Self::RawUmber => "17-1422",
            Self::BrickDust => "17-1424",
            Self::PecanBrown => "17-1430",
            Self::RawSienna => "17-1436",
            Self::Ginger => "17-1444",
            Self::Mango => "17-1446",
            Self::PureedPumpkin => "17-1449",
            Self::SummerFig => "17-1450",
            Self::Koi => "17-1452",
            Self::Tigerlily => "17-1456",
            Self::Orangeade => "17-1461",
            Self::Flame => "17-1462",
            Self::TangerineTango => "17-1463",
            Self::RedOrange => "17-1464",
            Self::SteepleGray => "17-1500",
            Self::WildDove => "17-1501",
            Self::Cloudburst => "17-1502",
            Self::StormFront => "17-1503",
            Self::Quail => "17-1505",
            Self::Cinder => "17-1506",
            Self::Antler => "17-1510",
            Self::WistfulMauve => "17-1511",
            Self::NostalgiaRose => "17-1512",
            Self::AshRose => "17-1514",
            Self::Burlwood => "17-1516",
            Self::OldRose => "17-1518",
            Self::CanyonRose => "17-1520",
            Self::Mauvewood => "17-1522",
            Self::DesertSand => "17-1524",
            Self::CedarWood => "17-1525",
            Self::Aragon => "17-1532",
            Self::MineralRed => "17-1537",
            Self::ApricotBrandy => "17-1540",
            Self::BurntSienna => "17-1544",
            Self::Cranberry => "17-1545",
            Self::Emberglow => "17-1547",
            Self::Paprika => "17-1553",
            Self::Grenadine => "17-1558",
            Self::MandarinRed => "17-1562",
            Self::CherryTomato => "17-1563",
            Self::Fiesta => "17-1564",
            Self::Elderberry => "17-1605",
            Self::HeatherRose => "17-1608",
            Self::MesaRose => "17-1609",
            Self::DuskyOrchid => "17-1610",
            Self::MellowMauve => "17-1612",
            Self::DecoRose => "17-1614",
            Self::RoseWine => "17-1623",
            Self::HollyBerry => "17-1633",
            Self::RoseOfSharon => "17-1635",
            Self::Chrysanthemum => "17-1641",
            Self::PorcelainRose => "17-1643",
            Self::SpicedCoral => "17-1644",
            Self::Dubarry => "17-1647",
            Self::Poinsettia => "17-1654",
            Self::HotCoral => "17-1656",
            Self::Bittersweet => "17-1663",
            Self::PoppyRed => "17-1664",
            Self::Moonscape => "17-1708",
            Self::Bordeaux => "17-1710",
            Self::DustyRose => "17-1718",
            Self::Malaga => "17-1723",
            Self::SunkistCoral => "17-1736",
            Self::ClaretRed => "17-1740",
            Self::CalypsoCoral => "17-1744",
            Self::Geranium => "17-1753",
            Self::ParadisePink => "17-1755",
            Self::RedViolet => "17-1818",
            Self::Carmine => "17-1831",
            Self::Azalea => "17-1842",
            Self::FruitDove => "17-1926",
            Self::DesertRose => "17-1927",
            Self::Bubblegum => "17-1928",
            Self::RaptureRose => "17-1929",
            Self::CamelliaRose => "17-1930",
            Self::HotPink => "17-1937",
            Self::FuchsiaRose => "17-2031",
            Self::FandangoPink => "17-2033",
            Self::PinkYarrow => "17-2034",
            Self::Magenta => "17-2036",
            Self::ChateauRose => "17-2120",
            Self::ShockingPink => "17-2127",
            Self::LilacRose => "17-2227",
            Self::CarmineRose => "17-2230",
            Self::Toadstool => "17-2411",
            Self::IbisRose => "17-2520",
            Self::Zinc => "17-2601",
            Self::DahliaMauve => "17-2617",
            Self::RoseViolet => "17-2624",
            Self::SuperPink => "17-2625",
            Self::PhloxPink => "17-2627",
            Self::Mulberry => "17-3014",
            Self::SpringCrocus => "17-3020",
            Self::Rosebud => "17-3023",
            Self::Bodacious => "17-3240",
            Self::DustyLavender => "17-3313",
            Self::IrisOrchid => "17-3323",
            Self::Valerian => "17-3410",
            Self::DiffusedOrchid => "17-3520",
            Self::OrchidMist => "17-3612",
            Self::ChalkViolet => "17-3615",
            Self::EnglishLavendar => "17-3617",
            Self::Hyacinth => "17-3619",
            Self::AmethystOrchid => "17-3628",
            Self::FairyWren => "17-3640",
            Self::Bougainvillea => "17-3725",
            Self::PaisleyPurple => "17-3730",
            Self::SunlitAllium => "17-3735",
            Self::Gull => "17-3802",
            Self::Nirvana => "17-3808",
            Self::PurpleAsh => "17-3810",
            Self::Dusk => "17-3812",
            Self::Daybreak => "17-3817",
            Self::AsterPurple => "17-3826",
            Self::DahliaPurple => "17-3834",
            Self::MinimalGray => "17-3906",
            Self::Quicksilver => "17-3907",
            Self::LavenderGray => "17-3910",
            Self::SilverFiligree => "17-3911",
            Self::Sharkskin => "17-3914",
            Self::Tempest => "17-3915",
            Self::Stonewash => "17-3917",
            Self::CountryBlue => "17-3918",
            Self::PurpleImpression => "17-3919",
            Self::EnglishManor => "17-3920",
            Self::BlueIce => "17-3922",
            Self::ColonyBlue => "17-3923",
            Self::LavenderViolet => "17-3924",
            Self::PersianViolet => "17-3925",
            Self::PaleIris => "17-3929",
            Self::Jacaranda => "17-3930",
            Self::DeepPeriwinkle => "17-3932",
            Self::SilverBullet => "17-3933",
            Self::PersianJewel => "17-3934",
            Self::BlueBonnet => "17-3936",
            Self::Iolite => "17-3940",
            Self::Titanium => "17-4014",
            Self::Infinity => "17-4015",
            Self::GrayFlannel => "17-4016",
            Self::BlueShadow => "17-4020",
            Self::FadedDenim => "17-4021",
            Self::BlueHeaven => "17-4023",
            Self::Riviera => "17-4027",
            Self::Riverside => "17-4028",
            Self::QuietHarbor => "17-4029",
            Self::SilverLakeBlue => "17-4030",
            Self::LichenBlue => "17-4032",
            Self::PacificCoast => "17-4033",
            Self::Ultramarine => "17-4037",
            Self::Marina => "17-4041",
            Self::Citadel => "17-4111",
            Self::Niagara => "17-4123",
            Self::CendreBlue => "17-4131",
            Self::AzureBlue => "17-4139",
            Self::IbizaBlue => "17-4245",
            Self::DivaBlue => "17-4247",
            Self::AdriaticBlue => "17-4320",
            Self::BlueMoon => "17-4328",
            Self::Blithe => "17-4336",
            Self::NeutralGray => "17-4402",
            Self::Monument => "17-4405",
            Self::Lead => "17-4408",
            Self::SmokeBlue => "17-4412",
            Self::Larkspur => "17-4421",
            Self::Bluejay => "17-4427",
            Self::NavagioBay => "17-4429",
            Self::VividBlue => "17-4432",
            Self::DresdenBlue => "17-4433",
            Self::MalibuBlue => "17-4435",
            Self::BlueDanube => "17-4440",
            Self::BarrierReef => "17-4530",
            Self::HawaiianOcean => "17-4540",
            Self::StormBlue => "17-4716",
            Self::PagodaBlue => "17-4724",
            Self::AlgiersBlue => "17-4728",
            Self::CaneelBay => "17-4730",
            Self::CapriBreeze => "17-4735",
            Self::BristolBlue => "17-4818",
            Self::Arctic => "17-4911",
            Self::Teal => "17-4919",
            Self::LakeBlue => "17-4928",
            Self::TealBlue => "17-5024",
            Self::Navigate => "17-5025",
            Self::DeepPeacockBlue => "17-5029",
            Self::Lapis => "17-5034",
            Self::Griffin => "17-5102",
            Self::ChinoisGreen => "17-5107",
            Self::Trellis => "17-5110",
            Self::OilBlue => "17-5111",
            Self::GreenBlueSlate => "17-5117",
            Self::LatigoBay => "17-5122",
            Self::ViridianGreen => "17-5126",
            Self::Columbia => "17-5130",
            Self::DynastyGreen => "17-5330",
            Self::SpectraGreen => "17-5335",
            Self::PorcelainGreen => "17-5421",
            Self::Alhambra => "17-5430",
            Self::DeepSea => "17-5513",
            Self::Greenlake => "17-5528",
            Self::DeepGreen => "17-5633",
            Self::VividGreen => "17-5638",
            Self::Emerald => "17-5641",
            Self::BottleGreen => "17-5722",
            Self::Viridis => "17-5734",
            Self::Parakeet => "17-5735",
            Self::DarkIvy => "17-5912",
            Self::PineGreen => "17-5923",
            Self::SimplyGreen => "17-5936",
            Self::DeepMint => "17-5937",
            Self::LaurelWreath => "17-6009",
            Self::JellyBean => "17-6030",
            Self::FernGreen => "17-6153",
            Self::Shadow => "17-6206",
            Self::SeaSpray => "17-6212",
            Self::DeepGrassGreen => "17-6219",
            Self::MediumGreen => "17-6229",
            Self::Kashmir => "17-6319",
            Self::HedgeGreen => "17-6323",
            Self::MintGreen => "17-6333",
            Self::SmokedPearl => "18-0000",
            Self::Kale => "18-0107",
            Self::Dill => "18-0108",
            Self::EnglishIvy => "18-0110",
            Self::VineyardGreen => "18-0117",
            Self::WillowBough => "18-0119",
            Self::ElmGreen => "18-0121",
            Self::ArtichokeGreen => "18-0125",
            Self::Cactus => "18-0130",
            Self::Treetop => "18-0135",
            Self::Castlerock => "18-0201",
            Self::Mayfly => "18-0220",
            Self::Pesto => "18-0228",
            Self::Gunmetal => "18-0306",
            Self::DeepLichenGreen => "18-0312",
            Self::Olivine => "18-0316",
            Self::BronzeGreen => "18-0317",
            Self::Cypress => "18-0322",
            Self::CallisteGreen => "18-0324",
            Self::CedarGreen => "18-0328",
            Self::TwistOfLime => "18-0330",
            Self::Grasshopper => "18-0332",
            Self::DarkGullGray => "18-0403",
            Self::FourLeafClover => "18-0420",
            Self::LodenGreen => "18-0422",
            Self::CapuletOlive => "18-0426",
            Self::Avocado => "18-0430",
            Self::CallaGreen => "18-0435",
            Self::Gargoyle => "18-0503",
            Self::CastorGray => "18-0510",
            Self::BungeeCord => "18-0513",
            Self::DustyOlive => "18-0515",
            Self::BurntOlive => "18-0521",
            Self::WinterMoss => "18-0523",
            Self::Iguana => "18-0525",
            Self::OliveBranch => "18-0527",
            Self::Woodbine => "18-0538",
            Self::CharcoalGray => "18-0601",
            Self::StoneGray => "18-0615",
            Self::CovertGreen => "18-0617",
            Self::OliveDrab => "18-0622",
            Self::MartiniOlive => "18-0625",
            Self::FirGreen => "18-0627",
            Self::Lizard => "18-0629",
            Self::GothicOlive => "18-0724",
            Self::Capers => "18-0820",
            Self::Nutria => "18-0825",
            Self::Butternut => "18-0830",
            Self::Plantation => "18-0832",
            Self::DriedTobacco => "18-0835",
            Self::Tapenade => "18-0840",
            Self::Kangaroo => "18-0920",
            Self::Sepia => "18-0928",
            Self::CoffeeLiqueur => "18-0930",
            Self::Rubber => "18-0933",
            Self::BuckthornBrown => "18-0935",
            Self::BronzeBrown => "18-0937",
            Self::Cumin => "18-0939",
            Self::GoldenBrown => "18-0940",
            Self::CathaySpice => "18-0950",
            Self::Shitake => "18-1015",
            Self::Cub => "18-1016",
            Self::Caribou => "18-1017",
            Self::Otter => "18-1018",
            Self::Ermine => "18-1022",
            Self::Bison => "18-1027",
            Self::Emperador => "18-1028",
            Self::ToastedCoconut => "18-1029",
            Self::Thrush => "18-1030",
            Self::Toffee => "18-1031",
            Self::Dachshund => "18-1033",
            Self::MonksRobe => "18-1048",
            Self::ThaiCurry => "18-1049",
            Self::HoneyGinger => "18-1050",
            Self::FallenRock => "18-1108",
            Self::Brindle => "18-1110",
            Self::Walnut => "18-1112",
            Self::Partridge => "18-1124",
            Self::Aztec => "18-1130",
            Self::Rawhide => "18-1137",
            Self::MochaBisque => "18-1140",
            Self::LeatherBrown => "18-1142",
            Self::CaramelCafe => "18-1148",
            Self::GlazedGinger => "18-1154",
            Self::SugarAlmond => "18-1155",
            Self::SudanBrown => "18-1160",
            Self::PumpkinSpice => "18-1163",
            Self::Driftwood => "18-1210",
            Self::CocoaBrown => "18-1222",
            Self::CarobBrown => "18-1229",
            Self::CoconutShell => "18-1230",
            Self::Russet => "18-1235",
            Self::RusticBrown => "18-1238",
            Self::Sierra => "18-1239",
            Self::BrownPatina => "18-1242",
            Self::GingerBread => "18-1244",
            Self::Umber => "18-1246",
            Self::Rust => "18-1248",
            Self::HawaiianSunset => "18-1249",
            Self::BombayBrown => "18-1250",
            Self::Falcon => "18-1304",
            Self::Iron => "18-1306",
            Self::DeepTaupe => "18-1312",
            Self::Acorn => "18-1314",
            Self::Clove => "18-1320",
            Self::Brownie => "18-1321",
            Self::SpicedApple => "18-1325",
            Self::Nutmeg => "18-1326",
            Self::CopperBrown => "18-1336",
            Self::PottersClay => "18-1340",
            Self::Auburn => "18-1343",
            Self::CinnamonStick => "18-1345",
            Self::Bruschetta => "18-1346",
            Self::BurntBrick => "18-1350",
            Self::BurntOchre => "18-1354",
            Self::RooibosTea => "18-1355",
            Self::Sparrow => "18-1404",
            Self::Flint => "18-1405",
            Self::Peppercorn => "18-1409",
            Self::PlumWine => "18-1411",
            Self::Marron => "18-1415",
            Self::CrushedBerry => "18-1418",
            Self::WildGinger => "18-1420",
            Self::Cognac => "18-1421",
            Self::Mahogany => "18-1425",
            Self::AppleButter => "18-1426",
            Self::Chutney => "18-1433",
            Self::EtruscanRed => "18-1434",
            Self::WitheredRose => "18-1435",
            Self::LightMahogany => "18-1436",
            Self::Marsala => "18-1438",
            Self::ChiliOil => "18-1440",
            Self::BakedClay => "18-1441",
            Self::RedOchre => "18-1442",
            Self::Redwood => "18-1443",
            Self::TandoriSpice => "18-1444",
            Self::SpicyOrange => "18-1445",
            Self::OrangeRust => "18-1447",
            Self::Chili => "18-1448",
            Self::Ketchup => "18-1449",
            Self::MeccaOrange => "18-1450",
            Self::AutumnGlaze => "18-1451",
            Self::RedClay => "18-1454",
            Self::PlumTruffle => "18-1506",
            Self::RoseBrown => "18-1512",
            Self::BarnRed => "18-1531",
            Self::GingerSpice => "18-1535",
            Self::HotSauce => "18-1536",
            Self::CopperCoin => "18-1537",
            Self::Cinnabar => "18-1540",
            Self::BrandyBrown => "18-1541",
            Self::BossaNova => "18-1547",
            Self::ValiantPoppy => "18-1549",
            Self::AuroraRed => "18-1550",
            Self::AuraOrange => "18-1551",
            Self::MoltenLava => "18-1555",
            Self::OrangeCom => "18-1561",
            Self::Poinciana => "18-1564",
            Self::RoseTaupe => "18-1612",
            Self::RenaissanceRose => "18-1613",
            Self::Nocturne => "18-1614",
            Self::RoanRouge => "18-1616",
            Self::Maroon => "18-1619",
            Self::FadedRose => "18-1629",
            Self::DustyCedar => "18-1630",
            Self::EarthRed => "18-1631",
            Self::GarnetRose => "18-1633",
            Self::BaroqueRose => "18-1634",
            Self::SlateRose => "18-1635",
            Self::Cardinal => "18-1643",
            Self::BakedApple => "18-1648",
            Self::DeepSeaCoral => "18-1649",
            Self::Cayenne => "18-1651",
            Self::RococcoRed => "18-1652",
            Self::Toreador => "18-1653",
            Self::Lychee => "18-1654",
            Self::MarsRed => "18-1655",
            Self::Salsa => "18-1657",
            Self::PompeianRed => "18-1658",
            Self::GojiBerry => "18-1659",
            Self::Tomato => "18-1660",
            Self::TomatoPuree => "18-1661",
            Self::FlameScarlet => "18-1662",
            Self::ChineseRed => "18-1663",
            Self::FieryRed => "18-1664",
            Self::Shark => "18-1703",
            Self::ArcticDusk => "18-1705",
            Self::BlackPlum => "18-1706",
            Self::Ephemera => "18-1708",
            Self::Tulipwood => "18-1709",
            Self::GrapeNectar => "18-1710",
            Self::Damson => "18-1716",
            Self::HawthornRose => "18-1718",
            Self::VioletQuartz => "18-1720",
            Self::DryRose => "18-1725",
            Self::RaspberryWine => "18-1741",
            Self::Raspberry => "18-1754",
            Self::RougeRed => "18-1755",
            Self::Teaberry => "18-1756",
            Self::JalapenoRed => "18-1759",
            Self::Barberry => "18-1760",
            Self::SkiPatrol => "18-1761",
            Self::Hibiscus => "18-1762",
            Self::HighRiskRed => "18-1763",
            Self::Lollipop => "18-1764",
            Self::TwilightMauve => "18-1807",
            Self::RoseRed => "18-1852",
            Self::VirtualPink => "18-1856",
            Self::BrightRose => "18-1945",
            Self::Jazzy => "18-1950",
            Self::LovePotion => "18-1951",
            Self::Beaujolais => "18-2027",
            Self::RaspberrySorbet => "18-2043",
            Self::PinkPeacock => "18-2045",
            Self::GrapeShake => "18-2109",
            Self::Honeysuckle => "18-2120",
            Self::PinkFlambe => "18-2133",
            Self::Cabaret => "18-2140",
            Self::BeetrootPurple => "18-2143",
            Self::Clover => "18-2320",
            Self::CactusFlower => "18-2326",
            Self::FuchsiaRed => "18-2328",
            Self::RaspberryRose => "18-2333",
            Self::VeryBerry => "18-2336",
            Self::FuchsiaPurple => "18-2436",
            Self::MagentaHaze => "18-2525",
            Self::BatonRouge => "18-2527",
            Self::PurpleWine => "18-2929",
            Self::ArgylePurple => "18-3011",
            Self::PurpleGumdrop => "18-3012",
            Self::BerryConserve => "18-3013",
            Self::GrapeKiss => "18-3014",
            Self::Amethyst => "18-3015",
            Self::DeepOrchid => "18-3022",
            Self::StrikingPurple => "18-3025",
            Self::PurpleOrchid => "18-3027",
            Self::Willowherb => "18-3120",
            Self::Grapeade => "18-3211",
            Self::ConcordGrape => "18-3218",
            Self::VeryGrape => "18-3220",
            Self::RadiantOrchid => "18-3224",
            Self::MeadowMauve => "18-3230",
            Self::Dahlia => "18-3324",
            Self::HyacinthViolet => "18-3331",
            Self::VividViola => "18-3339",
            Self::Charisma => "18-3340",
            Self::VintageViolet => "18-3410",
            Self::GrapeJam => "18-3415",
            Self::ChineseViolet => "18-3418",
            Self::GrapeCompote => "18-3513",
            Self::PatricianPurple => "18-3518",
            Self::PurpleHeart => "18-3520",
            Self::PlumJam => "18-3521",
            Self::CrushedGrape => "18-3522",
            Self::LavenderCrystal => "18-3530",
            Self::RoyalLilac => "18-3531",
            Self::Dewberry => "18-3533",
            Self::PurpleSapphire => "18-3540",
            Self::ImperialPalace => "18-3615",
            Self::Mystical => "18-3620",
            Self::Bellflower => "18-3628",
            Self::DeepLavender => "18-3633",
            Self::ChiveBlossom => "18-3634",
            Self::PicassoLily => "18-3635",
            Self::GrayRidge => "18-3710",
            Self::PurpleSage => "18-3712",
            Self::MulledGrape => "18-3714",
            Self::MontanaGrape => "18-3715",
            Self::PurpleHaze => "18-3718",
            Self::PassionFlower => "18-3737",
            Self::Cadet => "18-3812",
            Self::Heron => "18-3817",
            Self::TwilightPurple => "18-3820",
            Self::CorsicanBlue => "18-3828",
            Self::DustedPeri => "18-3833",
            Self::Veronica => "18-3834",
            Self::UltraViolet => "18-3838",
            Self::PurpleCorallite => "18-3839",
            Self::PurpleOpulence => "18-3840",
            Self::Excalibur => "18-3905",
            Self::Tornado => "18-3907",
            Self::VolcanicGlass => "18-3908",
            Self::FolkstoneGray => "18-3910",
            Self::Grisaille => "18-3912",
            Self::FlintStone => "18-3916",
            Self::GrayBlue => "18-3917",
            Self::ChinaBlue => "18-3918",
            Self::CoastalFjord => "18-3920",
            Self::BijouBlue => "18-3921",
            Self::CoronetBlue => "18-3922",
            Self::VelvetMorning => "18-3927",
            Self::DutchBlue => "18-3928",
            Self::BlueHorizon => "18-3929",
            Self::BleachedDenim => "18-3930",
            Self::Marlin => "18-3932",
            Self::BlueGranite => "18-3933",
            Self::Wedgewood => "18-3935",
            Self::BlueYonder => "18-3937",
            Self::BlueIris => "18-3943",
            Self::VioletStorm => "18-3944",
            Self::AmparoBlue => "18-3945",
            Self::BajaBlue => "18-3946",
            Self::DazzlingBlue => "18-3949",
            Self::IrisBloom => "18-3950",
            Self::SpectrumBlue => "18-3963",
            Self::SteelGray => "18-4005",
            Self::QuietShade => "18-4006",
            Self::GoblinBlue => "18-4011",
            Self::DecemberSky => "18-4016",
            Self::RealTeal => "18-4018",
            Self::CaptainsBlue => "18-4020",
            Self::BlueAshes => "18-4023",
            Self::CopenBlue => "18-4025",
            Self::Stellar => "18-4026",
            Self::MoonlightBlue => "18-4027",
            Self::BeringSea => "18-4028",
            Self::FederalBlue => "18-4029",
            Self::DeepWater => "18-4032",
            Self::VallartaBlue => "18-4034",
            Self::ParisianBlue => "18-4036",
            Self::Regatta => "18-4039",
            Self::StarSapphire => "18-4041",
            Self::PalaceBlue => "18-4043",
            Self::Daphne => "18-4045",
            Self::NebulasBlue => "18-4048",
            Self::StrongBlue => "18-4051",
            Self::MoonMist => "18-4105",
            Self::FrenchBlue => "18-4140",
            Self::Campanula => "18-4141",
            Self::VictoriaBlue => "18-4148",
            Self::StormyWeather => "18-4214",
            Self::BlueMirage => "18-4215",
            Self::Bluestone => "18-4217",
            Self::ProvincialBlue => "18-4220",
            Self::Bluesteel => "18-4222",
            Self::SaxonyBlue => "18-4225",
            Self::BlueSapphire => "18-4231",
            Self::Faience => "18-4232",
            Self::DirectoireBlue => "18-4244",
            Self::BrilliantBlue => "18-4247",
            Self::IndigoBunting => "18-4250",
            Self::BlueAster => "18-4252",
            Self::AegeanBlue => "18-4320",
            Self::SwedishBlue => "18-4330",
            Self::MediterraneanBlue => "18-4334",
            Self::Tapestry => "18-4417",
            Self::FjordBlue => "18-4430",
            Self::TurkishTile => "18-4432",
            Self::MykonosBlue => "18-4434",
            Self::Cloisonne => "18-4440",
            Self::Trooper => "18-4510",
            Self::ColonialBlue => "18-4522",
            Self::CaribbeanSea => "18-4525",
            Self::MosaicBlue => "18-4528",
            Self::Celestial => "18-4530",
            Self::BlueJewel => "18-4535",
            Self::MethylBlue => "18-4537",
            Self::HawaiianSurf => "18-4538",
            Self::NorthAtlantic => "18-4612",
            Self::TahitianTide => "18-4630",
            Self::StormySea => "18-4711",
            Self::Hydro => "18-4718",
            Self::BiscayBay => "18-4726",
            Self::HarborBlue => "18-4728",
            Self::EnamelBlue => "18-4733",
            Self::TileBlue => "18-4735",
            Self::DeepLake => "18-4834",
            Self::TropicalGreen => "18-4930",
            Self::Fanfare => "18-4936",
            Self::Parasailing => "18-5020",
            Self::QuetzalGreen => "18-5025",
            Self::BrushedNickel => "18-5102",
            Self::SedonaSage => "18-5105",
            Self::SeaPine => "18-5112",
            Self::NorthSea => "18-5115",
            Self::Bayou => "18-5121",
            Self::BlueGrass => "18-5128",
            Self::Pewter => "18-5203",
            Self::GraniteGray => "18-5204",
            Self::EiffelTower => "18-5210",
            Self::BlueSpruce => "18-5308",
            Self::Bayberry => "18-5315",
            Self::AlpineGreen => "18-5322",
            Self::UltramarineGreen => "18-5338",
            Self::SilverPine => "18-5410",
            Self::AntiqueGreen => "18-5418",
            Self::CadmiumGreen => "18-5424",
            Self::BalsamGreen => "18-5606",
            Self::BrittanyBlue => "18-5610",
            Self::DarkForest => "18-5611",
            Self::SagebrushGreen => "18-5612",
            Self::PosyGreen => "18-5616",
            Self::DeepJungle => "18-5618",
            Self::Tidepool => "18-5619",
            Self::Ivy => "18-5620",
            Self::Fir => "18-5621",
            Self::FrostySpruce => "18-5622",
            Self::ShadyGlade => "18-5624",
            Self::Bosphorus => "18-5633",
            Self::GolfGreen => "18-5642",
            Self::SmokePine => "18-5718",
            Self::GalapagosGreen => "18-5725",
            Self::AgaveGreen => "18-5806",
            Self::PepperGreen => "18-5841",
            Self::LushMeadow => "18-5845",
            Self::GardenTopiary => "18-5913",
            Self::DuckGreen => "18-6011",
            Self::FoliageGreen => "18-6018",
            Self::Leprechaun => "18-6022",
            Self::Amazon => "18-6024",
            Self::JollyGreen => "18-6030",
            Self::Myrtle => "18-6114",
            Self::Comfrey => "18-6216",
            Self::Fairway => "18-6320",
            Self::Juniper => "18-6330",
            Self::Raven => "19-0000",
            Self::Asphalt => "19-0201",
            Self::GrayPinstripe => "19-0203",
            Self::GardenGreen => "19-0230",
            Self::JetBlack => "19-0303",
            Self::ClimbingIvy => "19-0307",
            Self::Thyme => "19-0309",
            Self::Beetle => "19-0312",
            Self::BlackForest => "19-0315",
            Self::Chive => "19-0323",
            Self::SeaTurtle => "19-0403",
            Self::Beluga => "19-0405",
            Self::DeepDepths => "19-0413",
            Self::ForestNight => "19-0414",
            Self::DuffelBag => "19-0415",
            Self::KombuGreen => "19-0417",
            Self::RifleGreen => "19-0419",
            Self::BlackInk => "19-0506",
            Self::Peat => "19-0508",
            Self::Rosin => "19-0509",
            Self::Kalamata => "19-0510",
            Self::GrapeLeaf => "19-0511",
            Self::IvyGreen => "19-0512",
            Self::OliveNight => "19-0515",
            Self::DarkOlive => "19-0516",
            Self::BlackOlive => "19-0608",
            Self::Wren => "19-0614",
            Self::Teak => "19-0617",
            Self::Beech => "19-0618",
            Self::MilitaryOlive => "19-0622",
            Self::Demitasse => "19-0712",
            Self::Morel => "19-0808",
            Self::ChocolateChip => "19-0809",
            Self::MajorBrown => "19-0810",
            Self::TurkishCoffee => "19-0812",
            Self::SlateBlack => "19-0814",
            Self::DesertPalm => "19-0815",
            Self::Canteen => "19-0820",
            Self::Tarmac => "19-0822",
            Self::Crocodile => "19-0823",
            Self::Delicioso => "19-0840",
            Self::Mulch => "19-0910",
            Self::ChocolateBrown => "19-0912",
            Self::CoffeeBean => "19-0915",
            Self::RainDrum => "19-0916",
            Self::FrenchRoast => "19-1012",
            Self::Bracken => "19-1015",
            Self::Java => "19-1016",
            Self::Ganache => "19-1018",
            Self::DarkEarth => "19-1020",
            Self::Breen => "19-1034",
            Self::AfterDark => "19-1101",
            Self::Licorice => "19-1102",
            Self::Espresso => "19-1103",
            Self::Mole => "19-1106",
            Self::ChocolateTorte => "19-1109",
            Self::ChocolatePlum => "19-1110",
            Self::BlackCoffee => "19-1111",
            Self::Carafe => "19-1116",
            Self::Chestnut => "19-1118",
            Self::Pinecone => "19-1121",
            Self::ShoppingBag => "19-1213",
            Self::ChocolateLab => "19-1214",
            Self::ShavedChocolate => "19-1215",
            Self::Mustang => "19-1217",
            Self::PottingSoil => "19-1218",
            Self::Cappuccino => "19-1220",
            Self::FondueFudge => "19-1224",
            Self::RootBeer => "19-1228",
            Self::FriarBrown => "19-1230",
            Self::Tiramisu => "19-1233",
            Self::RockyRoad => "19-1234",
            Self::Brunette => "19-1235",
            Self::TortoiseShell => "19-1241",
            Self::ArabianSpice => "19-1245",
            Self::Picante => "19-1250",
            Self::SealBrown => "19-1314",
            Self::BitterChocolate => "19-1317",
            Self::Sable => "19-1320",
            Self::RumRaisin => "19-1321",
            Self::BrownStone => "19-1322",
            Self::HotChocolate => "19-1325",
            Self::Andorra => "19-1327",
            Self::MadderBrown => "19-1331",
            Self::Sequoia => "19-1333",
            Self::Henna => "19-1334",
            Self::FiredBrick => "19-1337",
            Self::RussetBrown => "19-1338",
            Self::ChicoryCoffee => "19-1419",
            Self::DeepMahogany => "19-1420",
            Self::SmokedPaprika => "19-1429",
            Self::Mink => "19-1430",
            Self::Fudgesickle => "19-1431",
            Self::ChocolateFondant => "19-1432",
            Self::CherryMahogany => "19-1435",
            Self::Cinnamon => "19-1436",
            Self::Puce => "19-1518",
            Self::RedMahogany => "19-1521",
            Self::Zinfandel => "19-1522",
            Self::OxbloodRed => "19-1524",
            Self::Port => "19-1525",
            Self::ChocolateTruffle => "19-1526",
            Self::WindsorWine => "19-1528",
            Self::BurntRusset => "19-1530",
            Self::SunDriedTomato => "19-1531",
            Self::Rosewood => "19-1532",
            Self::Cowhide => "19-1533",
            Self::Merlot => "19-1534",
            Self::Syrah => "19-1535",
            Self::RedPear => "19-1536",
            Self::BurntHenna => "19-1540",
            Self::BrickRed => "19-1543",
            Self::RedDahlia => "19-1555",
            Self::ChiliPepper => "19-1557",
            Self::ScarletSage => "19-1559",
            Self::Raisin => "19-1606",
            Self::PrunePurple => "19-1608",
            Self::Burgundy => "19-1617",
            Self::Fudge => "19-1619",
            Self::Huckleberry => "19-1620",
            Self::CatawbaGrape => "19-1621",
            Self::VineyardWine => "19-1623",
            Self::Sassafras => "19-1624",
            Self::DecadentChocolate => "19-1625",
            Self::PortRoyale => "19-1627",
            Self::RubyWine => "19-1629",
            Self::BikingRed => "19-1650",
            Self::Rhubarb => "19-1652",
            Self::Garnet => "19-1655",
            Self::RioRed => "19-1656",
            Self::Samba => "19-1662",
            Self::RibbonRed => "19-1663",
            Self::TrueRed => "19-1664",
            Self::MauveWine => "19-1716",
            Self::Fig => "19-1718",
            Self::Cabernet => "19-1724",
            Self::TawnyPort => "19-1725",
            Self::Cordovan => "19-1726",
            Self::BarbadosCherry => "19-1757",
            Self::HauteRed => "19-1758",
            Self::AmericanBeauty => "19-1759",
            Self::Scarlet => "19-1760",
            Self::TangoRed => "19-1761",
            Self::Crimson => "19-1762",
            Self::RacingRed => "19-1763",
            Self::LipstickRed => "19-1764",
            Self::DeepClaret => "19-1840",
            Self::RedBud => "19-1850",
            Self::PersianRed => "19-1860",
            Self::JesterRed => "19-1862",
            Self::Scooter => "19-1863",
            Self::Pomegranate => "19-1930",
            Self::TibetanRed => "19-1934",
            Self::RumbaRed => "19-1940",
            Self::Cerise => "19-1955",
            Self::Hortensia => "19-2009",
            Self::Prune => "19-2014",
            Self::Rhododendron => "19-2024",
            Self::RedPlum => "19-2025",
            Self::BeetRed => "19-2030",
            Self::Anemone => "19-2033",
            Self::Granita => "19-2039",
            Self::CherriesJubilee => "19-2041",
            Self::Vivacious => "19-2045",
            Self::Sangria => "19-2047",
            Self::Winetasting => "19-2118",
            Self::Eggplant => "19-2311",
            Self::CrushedViolets => "19-2312",
            Self::GrapeWine => "19-2315",
            Self::Amaranth => "19-2410",
            Self::PickledBeet => "19-2420",
            Self::MagentaPurple => "19-2428",
            Self::PlumCaspia => "19-2429",
            Self::PurplePotion => "19-2430",
            Self::Boysenberry => "19-2431",
            Self::RaspberryRadiance => "19-2432",
            Self::FestivalFuchsia => "19-2434",
            Self::ItalianPlum => "19-2514",
            Self::PotentPurple => "19-2520",
            Self::DarkPurple => "19-2524",
            Self::WinterBloom => "19-2620",
            Self::WildAster => "19-2630",
            Self::Wineberry => "19-2814",
            Self::BlackberryWine => "19-2816",
            Self::Phlox => "19-2820",
            Self::Hollyhock => "19-2924",
            Self::Gloxinia => "19-3022",
            Self::Byzantium => "19-3138",
            Self::Indigo => "19-3215",
            Self::ShadowPurple => "19-3217",
            Self::PlumPurple => "19-3218",
            Self::Plum => "19-3220",
            Self::PurplePassion => "19-3223",
            Self::GrapeJuice => "19-3230",
            Self::PlumPerfect => "19-3316",
            Self::DeepPurple => "19-3323",
            Self::WoodViolet => "19-3325",
            Self::SparklingGrape => "19-3336",
            Self::SunsetPurple => "19-3424",
            Self::SpicedPlum => "19-3425",
            Self::BrightViolet => "19-3438",
            Self::Majesty => "19-3514",
            Self::GrapeRoyale => "19-3518",
            Self::PurplePennant => "19-3519",
            Self::BlackberryCordial => "19-3520",
            Self::MeadowViolet => "19-3526",
            Self::ImperialPurple => "19-3528",
            Self::AmaranthPurple => "19-3536",
            Self::PurpleMagic => "19-3540",
            Self::Pansy => "19-3542",
            Self::Mysterioso => "19-3617",
            Self::SweetGrape => "19-3619",
            Self::PurpleReign => "19-3620",
            Self::Loganberry => "19-3622",
            Self::Acai => "19-3628",
            Self::Petunia => "19-3632",
            Self::TillandsiaPurple => "19-3638",
            Self::CrownJewel => "19-3640",
            Self::RoyalPurple => "19-3642",
            Self::Nightshade => "19-3712",
            Self::DeepWell => "19-3713",
            Self::NavyCosmos => "19-3714",
            Self::PurplePlumeria => "19-3716",
            Self::GothicGrape => "19-3720",
            Self::MulberryPurple => "19-3722",
            Self::PurpleVelvet => "19-3725",
            Self::Grape => "19-3728",
            Self::GentianViolet => "19-3730",
            Self::ParachutePurple => "19-3731",
            Self::Heliotrope => "19-3737",
            Self::PrismViolet => "19-3748",
            Self::VioletIndigo => "19-3750",
            Self::PlumKitten => "19-3803",
            Self::Eclipse => "19-3810",
            Self::EveningBlue => "19-3815",
            Self::AstralAura => "19-3830",
            Self::MaritimeBlue => "19-3831",
            Self::NavyBlue => "19-3832",
            Self::BlueRibbon => "19-3839",
            Self::DeepWisteria => "19-3842",
            Self::DeepBlue => "19-3847",
            Self::Liberty => "19-3850",
            Self::MazarineBlue => "19-3864",
            Self::Pavement => "19-3900",
            Self::Magnet => "19-3901",
            Self::Obsidian => "19-3902",
            Self::Shale => "19-3903",
            Self::Rabbit => "19-3905",
            Self::DarkShadow => "19-3906",
            Self::ForgedIron => "19-3907",
            Self::NineIron => "19-3908",
            Self::BlackBean => "19-3909",
            Self::IronGate => "19-3910",
            Self::BlackBeauty => "19-3911",
            Self::Graystone => "19-3915",
            Self::BlackenedPearl => "19-3917",
            Self::Periscope => "19-3918",
            Self::NightshadowBlue => "19-3919",
            Self::Peacoat => "19-3920",
            Self::BlackIris => "19-3921",
            Self::SkyCaptain => "19-3922",
            Self::NavyBlazer => "19-3923",
            Self::NightSky => "19-3924",
            Self::PatriotBlue => "19-3925",
            Self::CrownBlue => "19-3926",
            Self::Graphite => "19-3927",
            Self::BlueIndigo => "19-3928",
            Self::VintageIndigo => "19-3929",
            Self::OdysseyGray => "19-3930",
            Self::MedievalBlue => "19-3933",
            Self::DeepCobalt => "19-3935",
            Self::SkipperBlue => "19-3936",
            Self::TwilightBlue => "19-3938",
            Self::Blueprint => "19-3939",
            Self::BlueDepths => "19-3940",
            Self::OrientBlue => "19-3947",
            Self::DeepUltramarine => "19-3950",
            Self::ClematisBlue => "19-3951",
            Self::SurfTheWeb => "19-3952",
            Self::SodaliteBlue => "19-3953",
            Self::RoyalBlue => "19-3955",
            Self::MonacoBlue => "19-3964",
            Self::BlackOnyx => "19-4003",
            Self::TapShoe => "19-4004",
            Self::StretchLimo => "19-4005",
            Self::Caviar => "19-4006",
            Self::Anthracite => "19-4007",
            Self::Meteorite => "19-4008",
            Self::OuterSpace => "19-4009",
            Self::TotalEclipse => "19-4010",
            Self::Salute => "19-4011",
            Self::Carbon => "19-4012",
            Self::DarkNavy => "19-4013",
            Self::OmbreBlue => "19-4014",
            Self::BlueGraphite => "19-4015",
            Self::IndiaInk => "19-4019",
            Self::DarkSapphire => "19-4020",
            Self::Blueberry => "19-4021",
            Self::ParisianNight => "19-4022",
            Self::BlueNights => "19-4023",
            Self::DressBlues => "19-4024",
            Self::MoodIndigo => "19-4025",
            Self::EnsignBlue => "19-4026",
            Self::EstateBlue => "19-4027",
            Self::InsigniaBlue => "19-4028",
            Self::NavyPeony => "19-4029",
            Self::TrueNavy => "19-4030",
            Self::SargassoSea => "19-4031",
            Self::Poseidon => "19-4033",
            Self::SailorBlue => "19-4034",
            Self::DarkBlue => "19-4035",
            Self::BrightCobalt => "19-4037",
            Self::GibraltarSea => "19-4038",
            Self::Delft => "19-4039",
            Self::Limoges => "19-4044",
            Self::LapisBlue => "19-4045",
            Self::BaleineBlue => "19-4048",
            Self::SnorkelBlue => "19-4049",
            Self::NauticalBlue => "19-4050",
            Self::ClassicBlue => "19-4052",
            Self::TurkishSea => "19-4053",
            Self::GalaxyBlue => "19-4055",
            Self::OlympianBlue => "19-4056",
            Self::TrueBlue => "19-4057",
            Self::Ebony => "19-4104",
            Self::MidnightNavy => "19-4110",
            Self::DarkDenim => "19-4118",
            Self::BlueOpal => "19-4120",
            Self::BlueWingTeal => "19-4121",
            Self::MoonlitOcean => "19-4122",
            Self::MajolicaBlue => "19-4125",
            Self::DeepDive => "19-4126",
            Self::Midnight => "19-4127",
            Self::PrincessBlue => "19-4150",
            Self::Skydiver => "19-4151",
            Self::MoonlessNight => "19-4203",
            Self::Phantom => "19-4205",
            Self::Turbulence => "19-4215",
            Self::Vulcan => "19-4218",
            Self::DarkSlate => "19-4220",
            Self::IndianTeal => "19-4227",
            Self::OrionBlue => "19-4229",
            Self::InkBlue => "19-4234",
            Self::MoroccanBlue => "19-4241",
            Self::ImperialBlue => "19-4245",
            Self::PirateBlack => "19-4305",
            Self::Stargazer => "19-4316",
            Self::MallardBlue => "19-4318",
            Self::LegionBlue => "19-4324",
            Self::ReflectingPond => "19-4326",
            Self::Corsair => "19-4329",
            Self::LyonsBlue => "19-4340",
            Self::Seaport => "19-4342",
            Self::Mediterranea => "19-4517",
            Self::ShadedSpruce => "19-4524",
            Self::BlueCoral => "19-4526",
            Self::OceanDepths => "19-4535",
            Self::CrystalTeal => "19-4536",
            Self::DeepLagoon => "19-4540",
            Self::AtlanticDeep => "19-4726",
            Self::MallardGreen => "19-4818",
            Self::Balsam => "19-4820",
            Self::Dragonfly => "19-4826",
            Self::GreenGables => "19-4906",
            Self::DeepTeal => "19-4914",
            Self::Pacific => "19-4916",
            Self::TealGreen => "19-4922",
            Self::UrbanChic => "19-5004",
            Self::SeaMoss => "19-5030",
            Self::DarkestSpruce => "19-5212",
            Self::Storm => "19-5217",
            Self::BotanicalGarden => "19-5220",
            Self::Everglade => "19-5226",
            Self::ForestBiome => "19-5230",
            Self::RainForest => "19-5232",
            Self::PonderosaPine => "19-5320",
            Self::Scarab => "19-5350",
            Self::PineGrove => "19-5406",
            Self::BistroGreen => "19-5408",
            Self::TrekkingGreen => "19-5411",
            Self::Jasper => "19-5413",
            Self::JuneBug => "19-5414",
            Self::Evergreen => "19-5420",
            Self::HunterGreen => "19-5511",
            Self::DarkGreen => "19-5513",
            Self::Cilantro => "19-5621",
            Self::JetSet => "19-5708",
            Self::JungleGreen => "19-5914",
            Self::Sycamore => "19-5917",
            Self::MountainView => "19-5918",
            Self::Pineneedle => "19-5920",
            Self::VerdantGreen => "19-6026",
            Self::Eden => "19-6050",
            Self::DeepForest => "19-6110",
            Self::GreenerPastures => "19-6311",
        }
    }

    /// The color name without code and suffix (`"True Blue"`).
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Egret => "Egret",
            Self::VanillaIce => "Vanilla Ice",
            Self::AntiqueWhite => "Antique White",
            Self::SweetCream => "Sweet Cream",
            Self::Papyrus => "Papyrus",
            Self::Buttercream => "Buttercream",
            Self::GlassGreen => "Glass Green",
            Self::WaterLily => "Water Lily",
            Self::SylvanGreen => "Sylvan Green",
            Self::WinterWhite => "Winter White",
            Self::Afterglow => "Afterglow",
            Self::LemonIcing => "Lemon Icing",
            Self::BrightWhite => "Bright White",
            Self::SnowWhite => "Snow White",
            Self::PastelParchment => "Pastel Parchment",
            Self::Gardenia => "Gardenia",
            Self::JetStream => "Jet Stream",
            Self::Pristine => "Pristine",
            Self::SugarSwizzle => "Sugar Swizzle",
            Self::CoconutMilk => "Coconut Milk",
            Self::EtherealGreen => "Ethereal Green",
            Self::PearSorbet => "Pear Sorbet",
            Self::PastelYellow => "Pastel Yellow",
            Self::TransparentYellow => "Transparent Yellow",
            Self::WaxYellow => "Wax Yellow",
            Self::Flan => "Flan",
            Self::ElfinYellow => "Elfin Yellow",
            Self::YellowIris => "Yellow Iris",
            Self::YellowPear => "Yellow Pear",
            Self::WhisperWhite => "Whisper White",
            Self::TenderYellow => "Tender Yellow",
            Self::Ecru => "Ecru",
            Self::PearledIvory => "Pearled Ivory",
            Self::WhiteAlyssum => "White Alyssum",
            Self::BridalBlush => "Bridal Blush",
            Self::SeaSalt => "Sea Salt",
            Self::AngelWing => "Angel Wing",
            Self::CreamPink => "Cream Pink",
            Self::PowderPuff => "Powder Puff",
            Self::Rosewater => "Rosewater",
            Self::PetalPink => "Petal Pink",
            Self::Delicacy => "Delicacy",
            Self::ShrinkingViolet => "Shrinking Violet",
            Self::BrilliantWhite => "Brilliant White",
            Self::CloudDancer => "Cloud Dancer",
            Self::StarWhite => "Star White",
            Self::Marshmallow => "Marshmallow",
            Self::LilyWhite => "Lily White",
            Self::CannoliCream => "Cannoli Cream",
            Self::MysticBlue => "Mystic Blue",
            Self::BitOfBlue => "Bit Of Blue",
            Self::BillowingSail => "Billowing Sail",
            Self::BlancDeBlanc => "Blanc De Blanc",
            Self::Tofu => "Tofu",
            Self::SummerShower => "Summer Shower",
            Self::Ice => "Ice",
            Self::LightestSky => "Lightest Sky",
            Self::HintOfMint => "Hint Of Mint",
            Self::WhiteSwan => "White Swan",
            Self::WhiteAsparagus => "White Asparagus",
            Self::BoneWhite => "Bone White",
            Self::MeadowMist => "Meadow Mist",
            Self::CanaryGreen => "Canary Green",
            Self::Ambrosia => "Ambrosia",
            Self::PistachioShell => "Pistachio Shell",
            Self::PatinaGreen => "Patina Green",
            Self::WhitecapGray => "Whitecap Gray",
            Self::AsparagusGreen => "Asparagus Green",
            Self::LimeCream => "Lime Cream",
            Self::SeafoamGreen => "Seafoam Green",
            Self::WhiteJade => "White Jade",
            Self::Gleam => "Gleam",
            Self::Butterfly => "Butterfly",
            Self::LightGray => "Light Gray",
            Self::Hay => "Hay",
            Self::MellowGreen => "Mellow Green",
            Self::DaiquiriGreen => "Daiquiri Green",
            Self::PaleLimeYellow => "Pale Lime Yellow",
            Self::YoungWheat => "Young Wheat",
            Self::Citron => "Citron",
            Self::LuminaryGreen => "Luminary Green",
            Self::Charlock => "Charlock",
            Self::Eggnog => "Eggnog",
            Self::Angora => "Angora",
            Self::GreenEssence => "Green Essence",
            Self::DustyYellow => "Dusty Yellow",
            Self::LemonGrass => "Lemon Grass",
            Self::CanaryYellow => "Canary Yellow",
            Self::Aurora => "Aurora",
            Self::BlazingYellow => "Blazing Yellow",
            Self::Celandine => "Celandine",
            Self::Seedpearl => "Seedpearl",
            Self::WhiteSmoke => "White Smoke",
            Self::Macadamia => "Macadamia",
            Self::Navajo => "Navajo",
            Self::LemonMeringue => "Lemon Meringue",
            Self::Vanilla => "Vanilla",
            Self::AlmondOil => "Almond Oil",
            Self::Cornhusk => "Cornhusk",
            Self::DoubleCream => "Double Cream",
            Self::AniseFlower => "Anise Flower",
            Self::PineappleSlice => "Pineapple Slice",
            Self::MellowYellow => "Mellow Yellow",
            Self::Lemonade => "Lemonade",
            Self::FrenchVanilla => "French Vanilla",
            Self::Sunshine => "Sunshine",
            Self::Sundress => "Sundress",
            Self::LemonDrop => "Lemon Drop",
            Self::Goldfinch => "Goldfinch",
            Self::YellowCream => "Yellow Cream",
            Self::Limelight => "Limelight",
            Self::SunnyLime => "Sunny Lime",
            Self::LemonVerbena => "Lemon Verbena",
            Self::Buttercup => "Buttercup",
            Self::Yarrow => "Yarrow",
            Self::CloudCream => "Cloud Cream",
            Self::Rutabaga => "Rutabaga",
            Self::SunKiss => "Sun Kiss",
            Self::Dawn => "Dawn",
            Self::AlabasterGleam => "Alabaster Gleam",
            Self::AutumnBlonde => "Autumn Blonde",
            Self::VanillaCustard => "Vanilla Custard",
            Self::ApricotGelato => "Apricot Gelato",
            Self::GoldenFleece => "Golden Fleece",
            Self::PaleBanana => "Pale Banana",
            Self::Popcorn => "Popcorn",
            Self::GoldenHaze => "Golden Haze",
            Self::LambsWool => "Lambs Wool",
            Self::Nude => "Nude",
            Self::TenderPeach => "Tender Peach",
            Self::Alesan => "Alesan",
            Self::PalePeach => "Pale Peach",
            Self::BleachedApricot => "Bleached Apricot",
            Self::GoldenStraw => "Golden Straw",
            Self::NovellePeach => "Novelle Peach",
            Self::MotherOfPearl => "Mother Of Pearl",
            Self::PastelRoseTan => "Pastel Rose Tan",
            Self::Linen => "Linen",
            Self::VanillaCream => "Vanilla Cream",
            Self::ScallopShell => "Scallop Shell",
            Self::PeachPuree => "Peach Puree",
            Self::SheerPink => "Sheer Pink",
            Self::PeachDust => "Peach Dust",
            Self::Dew => "Dew",
            Self::CremeDePeche => "Creme De Peche",
            Self::SilverPeony => "Silver Peony",
            Self::PearlBlush => "Pearl Blush",
            Self::SoftPink => "Soft Pink",
            Self::VeiledRose => "Veiled Rose",
            Self::Pearl => "Pearl",
            Self::HeavenlyPink => "Heavenly Pink",
            Self::BlushingBride => "Blushing Bride",
            Self::Tapioca => "Tapioca",
            Self::PinkTint => "Pink Tint",
            Self::CrystalPink => "Crystal Pink",
            Self::PinkDogwood => "Pink Dogwood",
            Self::CrystalRose => "Crystal Rose",
            Self::MarysRose => "Marys Rose",
            Self::MauveMorn => "Mauve Morn",
            Self::AlmostMauve => "Almost Mauve",
            Self::Morganite => "Morganite",
            Self::MauveChalk => "Mauve Chalk",
            Self::LightLilac => "Light Lilac",
            Self::PrimrosePink => "Primrose Pink",
            Self::CradlePink => "Cradle Pink",
            Self::BarelyPink => "Barely Pink",
            Self::RoseWater => "Rose Water",
            Self::AlmondMilk => "Almond Milk",
            Self::VaporousGray => "Vaporous Gray",
            Self::Bluewash => "Bluewash",
            Self::SpaBlue => "Spa Blue",
            Self::BarelyBlue => "Barely Blue",
            Self::Skylight => "Skylight",
            Self::PastelBlue => "Pastel Blue",
            Self::Clearwater => "Clearwater",
            Self::StarlightBlue => "Starlight Blue",
            Self::WhisperingBlue => "Whispering Blue",
            Self::BlueBlush => "Blue Blush",
            Self::WanBlue => "Wan Blue",
            Self::ChalkBlue => "Chalk Blue",
            Self::Icicle => "Icicle",
            Self::Turtledove => "Turtledove",
            Self::Murmur => "Murmur",
            Self::MorningMist => "Morning Mist",
            Self::BlueGlass => "Blue Glass",
            Self::SoothingSea => "Soothing Sea",
            Self::SproutGreen => "Sprout Green",
            Self::BlueFlower => "Blue Flower",
            Self::WhisperGreen => "Whisper Green",
            Self::OpalBlue => "Opal Blue",
            Self::AquaGlass => "Aqua Glass",
            Self::MoonlightJade => "Moonlight Jade",
            Self::FairAqua => "Fair Aqua",
            Self::BleachedAqua => "Bleached Aqua",
            Self::ClearlyAqua => "Clearly Aqua",
            Self::Glacier => "Glacier",
            Self::DustyAqua => "Dusty Aqua",
            Self::Bay => "Bay",
            Self::HushedGreen => "Hushed Green",
            Self::ZephyrBlue => "Zephyr Blue",
            Self::Honeydew => "Honeydew",
            Self::SilverGreen => "Silver Green",
            Self::MilkyGreen => "Milky Green",
            Self::FairestJade => "Fairest Jade",
            Self::Frost => "Frost",
            Self::PhantomGreen => "Phantom Green",
            Self::Moonbeam => "Moonbeam",
            Self::WhiteSand => "White Sand",
            Self::Dewkist => "Dewkist",
            Self::Seacrest => "Seacrest",
            Self::PastelGreen => "Pastel Green",
            Self::GreenAsh => "Green Ash",
            Self::FogGreen => "Fog Green",
            Self::TenderGreens => "Tender Greens",
            Self::Reed => "Reed",
            Self::ParadiseGreen => "Paradise Green",
            Self::PistachioGreen => "Pistachio Green",
            Self::LilyGreen => "Lily Green",
            Self::ShadowLime => "Shadow Lime",
            Self::LettuceGreen => "Lettuce Green",
            Self::SapGreen => "Sap Green",
            Self::LimaBean => "Lima Bean",
            Self::Oatmeal => "Oatmeal",
            Self::GrayMorn => "Gray Morn",
            Self::GreenGlow => "Green Glow",
            Self::LimePopsicle => "Lime Popsicle",
            Self::FrozenDew => "Frozen Dew",
            Self::PaleGreen => "Pale Green",
            Self::LimeSherbet => "Lime Sherbet",
            Self::CeleryGreen => "Celery Green",
            Self::SharpGreen => "Sharp Green",
            Self::WildLime => "Wild Lime",
            Self::LimePunch => "Lime Punch",
            Self::Fog => "Fog",
            Self::AloeWash => "Aloe Wash",
            Self::Moth => "Moth",
            Self::ChinoGreen => "Chino Green",
            Self::GardenGlade => "Garden Glade",
            Self::GoldenMist => "Golden Mist",
            Self::Endive => "Endive",
            Self::Chardonnay => "Chardonnay",
            Self::Acacia => "Acacia",
            Self::GoldenKiwi => "Golden Kiwi",
            Self::Limeade => "Limeade",
            Self::Meadowlark => "Meadowlark",
            Self::GreenSheen => "Green Sheen",
            Self::SulphurSpring => "Sulphur Spring",
            Self::EveningPrimrose => "Evening Primrose",
            Self::Putty => "Putty",
            Self::SeaMist => "Sea Mist",
            Self::Custard => "Custard",
            Self::Raffia => "Raffia",
            Self::CreamGold => "Cream Gold",
            Self::Maize => "Maize",
            Self::Lemon => "Lemon",
            Self::PrimroseYellow => "Primrose Yellow",
            Self::LemonZest => "Lemon Zest",
            Self::Dandelion => "Dandelion",
            Self::SolarPower => "Solar Power",
            Self::SummerMelon => "Summer Melon",
            Self::BananaCrepe => "Banana Crepe",
            Self::Sunlight => "Sunlight",
            Self::Snapdragon => "Snapdragon",
            Self::HabaneroGold => "Habanero Gold",
            Self::AspenGold => "Aspen Gold",
            Self::MinionYellow => "Minion Yellow",
            Self::VibrantYellow => "Vibrant Yellow",
            Self::LemonChrome => "Lemon Chrome",
            Self::Birch => "Birch",
            Self::Sandshell => "Sandshell",
            Self::Parchment => "Parchment",
            Self::ReedYellow => "Reed Yellow",
            Self::Chamomile => "Chamomile",
            Self::ItalianStraw => "Italian Straw",
            Self::Soybean => "Soybean",
            Self::Straw => "Straw",
            Self::Cornsilk => "Cornsilk",
            Self::Flax => "Flax",
            Self::GoldenCream => "Golden Cream",
            Self::SunsetGold => "Sunset Gold",
            Self::BananaCream => "Banana Cream",
            Self::AmberYellow => "Amber Yellow",
            Self::PaleMarigold => "Pale Marigold",
            Self::JurassicGold => "Jurassic Gold",
            Self::Banana => "Banana",
            Self::CremeBrulee => "Creme Brulee",
            Self::OysterWhite => "Oyster White",
            Self::BleachedSand => "Bleached Sand",
            Self::Biscotti => "Biscotti",
            Self::GraySand => "Gray Sand",
            Self::IvoryCream => "Ivory Cream",
            Self::FrostedAlmond => "Frosted Almond",
            Self::Appleblossom => "Appleblossom",
            Self::MellowBuff => "Mellow Buff",
            Self::HoneyPeach => "Honey Peach",
            Self::Wheat => "Wheat",
            Self::AlmondCream => "Almond Cream",
            Self::DesertDust => "Desert Dust",
            Self::CreamBlush => "Cream Blush",
            Self::ApricotIce => "Apricot Ice",
            Self::PrairieSunset => "Prairie Sunset",
            Self::CaramelCream => "Caramel Cream",
            Self::PeachFuzz => "Peach Fuzz",
            Self::Buff => "Buff",
            Self::Impala => "Impala",
            Self::Creampuff => "Creampuff",
            Self::ApricotCream => "Apricot Cream",
            Self::Sunburst => "Sunburst",
            Self::ApricotSherbet => "Apricot Sherbet",
            Self::BrownRice => "Brown Rice",
            Self::SandDollar => "Sand Dollar",
            Self::WhisperPink => "Whisper Pink",
            Self::CreamTan => "Cream Tan",
            Self::Bisque => "Bisque",
            Self::Bellini => "Bellini",
            Self::PeachQuartz => "Peach Quartz",
            Self::PeachyKeen => "Peachy Keen",
            Self::BrazilianSand => "Brazilian Sand",
            Self::EnglishRose => "English Rose",
            Self::TropicalPeach => "Tropical Peach",
            Self::PaleDogwood => "Pale Dogwood",
            Self::Shell => "Shell",
            Self::CloudPink => "Cloud Pink",
            Self::CreolePink => "Creole Pink",
            Self::ChintzRose => "Chintz Rose",
            Self::SeashellPink => "Seashell Pink",
            Self::PeachBlush => "Peach Blush",
            Self::ImpatiensPink => "Impatiens Pink",
            Self::PinkSalt => "Pink Salt",
            Self::GossamerPink => "Gossamer Pink",
            Self::RoseQuartz => "Rose Quartz",
            Self::ChalkPink => "Chalk Pink",
            Self::RoseShadow => "Rose Shadow",
            Self::Potpourri => "Potpourri",
            Self::StrawberryCream => "Strawberry Cream",
            Self::AlmondBlossom => "Almond Blossom",
            Self::OrchidPink => "Orchid Pink",
            Self::FairyTale => "Fairy Tale",
            Self::PaleLilac => "Pale Lilac",
            Self::ParfaitPink => "Parfait Pink",
            Self::PinkMist => "Pink Mist",
            Self::PinkLady => "Pink Lady",
            Self::Ballerina => "Ballerina",
            Self::BalletSlipper => "Ballet Slipper",
            Self::CherryBlossom => "Cherry Blossom",
            Self::LilacSnow => "Lilac Snow",
            Self::OrchidIce => "Orchid Ice",
            Self::CrystalGray => "Crystal Gray",
            Self::OrchidTint => "Orchid Tint",
            Self::LilacAsh => "Lilac Ash",
            Self::GrayLilac => "Gray Lilac",
            Self::OrchidHush => "Orchid Hush",
            Self::LavenderFog => "Lavender Fog",
            Self::HalogenBlue => "Halogen Blue",
            Self::IllusionBlue => "Illusion Blue",
            Self::Antarctica => "Antarctica",
            Self::LilacHint => "Lilac Hint",
            Self::NimbusCloud => "Nimbus Cloud",
            Self::ArcticIce => "Arctic Ice",
            Self::PleinAir => "Plein Air",
            Self::Omphalodes => "Omphalodes",
            Self::OysterMushroom => "Oyster Mushroom",
            Self::DawnBlue => "Dawn Blue",
            Self::BalladBlue => "Ballad Blue",
            Self::FoggyDew => "Foggy Dew",
            Self::BabyBlue => "Baby Blue",
            Self::SilverBirch => "Silver Birch",
            Self::IceFlow => "Ice Flow",
            Self::MistyBlue => "Misty Blue",
            Self::BlueGlow => "Blue Glow",
            Self::CrystalBlue => "Crystal Blue",
            Self::TanagerTurquoise => "Tanager Turquoise",
            Self::PaleBlue => "Pale Blue",
            Self::Plume => "Plume",
            Self::LimpetShell => "Limpet Shell",
            Self::BlueLight => "Blue Light",
            Self::BlueTint => "Blue Tint",
            Self::RainyDay => "Rainy Day",
            Self::PaleAqua => "Pale Aqua",
            Self::IcyMorn => "Icy Morn",
            Self::PastelTurquoise => "Pastel Turquoise",
            Self::ArubaBlue => "Aruba Blue",
            Self::Yucca => "Yucca",
            Self::IcedAqua => "Iced Aqua",
            Self::BeachGlass => "Beach Glass",
            Self::IceGreen => "Ice Green",
            Self::Cabbage => "Cabbage",
            Self::GossamerGreen => "Gossamer Green",
            Self::BirdsEggGreen => "Birds Egg Green",
            Self::AlmostAqua => "Almost Aqua",
            Self::Spray => "Spray",
            Self::MistyJade => "Misty Jade",
            Self::BrookGreen => "Brook Green",
            Self::CeladonTint => "Celadon Tint",
            Self::GreenTint => "Green Tint",
            Self::GreenLily => "Green Lily",
            Self::Celadon => "Celadon",
            Self::MistGreen => "Mist Green",
            Self::BokChoy => "Bok Choy",
            Self::SilverGray => "Silver Gray",
            Self::PumiceStone => "Pumice Stone",
            Self::Overcast => "Overcast",
            Self::CastleWall => "Castle Wall",
            Self::CeladonGreen => "Celadon Green",
            Self::FoamGreen => "Foam Green",
            Self::Margarita => "Margarita",
            Self::NileGreen => "Nile Green",
            Self::ArcadianGreen => "Arcadian Green",
            Self::Greengage => "Greengage",
            Self::SummerGreen => "Summer Green",
            Self::TidalFoam => "Tidal Foam",
            Self::Lint => "Lint",
            Self::Seedling => "Seedling",
            Self::Nile => "Nile",
            Self::OpalineGreen => "Opaline Green",
            Self::JadeLime => "Jade Lime",
            Self::BrightLimeGreen => "Bright Lime Green",
            Self::AcidLime => "Acid Lime",
            Self::Bog => "Bog",
            Self::Beechnut => "Beechnut",
            Self::GreenBanana => "Green Banana",
            Self::BrightChartreuse => "Bright Chartreuse",
            Self::TenderShoots => "Tender Shoots",
            Self::LimeGreen => "Lime Green",
            Self::GreenHaze => "Green Haze",
            Self::DriedMoss => "Dried Moss",
            Self::ShadowGreen => "Shadow Green",
            Self::MutedLime => "Muted Lime",
            Self::Celery => "Celery",
            Self::Cement => "Cement",
            Self::Hemp => "Hemp",
            Self::Bamboo => "Bamboo",
            Self::SuperLemon => "Super Lemon",
            Self::Sulphur => "Sulphur",
            Self::EmpireYellow => "Empire Yellow",
            Self::CyberYellow => "Cyber Yellow",
            Self::Pampas => "Pampas",
            Self::DuskyCitron => "Dusky Citron",
            Self::MistedYellow => "Misted Yellow",
            Self::YolkYellow => "Yolk Yellow",
            Self::BuffYellow => "Buff Yellow",
            Self::Mimosa => "Mimosa",
            Self::Daffodil => "Daffodil",
            Self::SamoanSun => "Samoan Sun",
            Self::Freesia => "Freesia",
            Self::Parsnip => "Parsnip",
            Self::Jojoba => "Jojoba",
            Self::SaharaSun => "Sahara Sun",
            Self::Beeswax => "Beeswax",
            Self::GoldenRod => "Golden Rod",
            Self::SpicyMustard => "Spicy Mustard",
            Self::Citrus => "Citrus",
            Self::SpectraYellow => "Spectra Yellow",
            Self::GildedBeige => "Gilded Beige",
            Self::Gravel => "Gravel",
            Self::Cocoon => "Cocoon",
            Self::Rattan => "Rattan",
            Self::Ochre => "Ochre",
            Self::NewWheat => "New Wheat",
            Self::GoldenApricot => "Golden Apricot",
            Self::Amber => "Amber",
            Self::Marigold => "Marigold",
            Self::WarmApricot => "Warm Apricot",
            Self::Kumquat => "Kumquat",
            Self::Saffron => "Saffron",
            Self::Peyote => "Peyote",
            Self::OysterGray => "Oyster Gray",
            Self::WoodAsh => "Wood Ash",
            Self::Boulder => "Boulder",
            Self::Pebble => "Pebble",
            Self::Marzipan => "Marzipan",
            Self::AlmondBuff => "Almond Buff",
            Self::Beige => "Beige",
            Self::WinterWheat => "Winter Wheat",
            Self::ApricotIllusion => "Apricot Illusion",
            Self::Sheepskin => "Sheepskin",
            Self::DesertMist => "Desert Mist",
            Self::BuffOrange => "Buff Orange",
            Self::ApricotNectar => "Apricot Nectar",
            Self::SalmonBuff => "Salmon Buff",
            Self::Pumpkin => "Pumpkin",
            Self::Zinnia => "Zinnia",
            Self::IrishCream => "Irish Cream",
            Self::SmokeGray => "Smoke Gray",
            Self::ShiftingSand => "Shifting Sand",
            Self::Frappe => "Frappe",
            Self::ToastedAlmond => "Toasted Almond",
            Self::Amberlight => "Amberlight",
            Self::PeachParfait => "Peach Parfait",
            Self::PeachNougat => "Peach Nougat",
            Self::CoralSands => "Coral Sands",
            Self::BeachSand => "Beach Sand",
            Self::Peach => "Peach",
            Self::PeachNectar => "Peach Nectar",
            Self::ApricotWash => "Apricot Wash",
            Self::PeachCobbler => "Peach Cobbler",
            Self::OrangeChiffon => "Orange Chiffon",
            Self::Mushroom => "Mushroom",
            Self::RoseDust => "Rose Dust",
            Self::PeachWhip => "Peach Whip",
            Self::CameoRose => "Cameo Rose",
            Self::EveningSand => "Evening Sand",
            Self::PaleBlush => "Pale Blush",
            Self::RoseCloud => "Rose Cloud",
            Self::SpanishVilla => "Spanish Villa",
            Self::Hazelnut => "Hazelnut",
            Self::DustyPink => "Dusty Pink",
            Self::CoralPink => "Coral Pink",
            Self::Salmon => "Salmon",
            Self::PeachBud => "Peach Bud",
            Self::PeachMelba => "Peach Melba",
            Self::PeachPearl => "Peach Pearl",
            Self::ApricotBlush => "Apricot Blush",
            Self::RoseSmoke => "Rose Smoke",
            Self::SilverPink => "Silver Pink",
            Self::PowderPink => "Powder Pink",
            Self::Blossom => "Blossom",
            Self::PeachesNCream => "Peaches N Cream",
            Self::QuartzPink => "Quartz Pink",
            Self::SepiaRose => "Sepia Rose",
            Self::Lotus => "Lotus",
            Self::Peachskin => "Peachskin",
            Self::CoralBlush => "Coral Blush",
            Self::CandyPink => "Candy Pink",
            Self::PinkNectar => "Pink Nectar",
            Self::CameoPink => "Cameo Pink",
            Self::PrismPink => "Prism Pink",
            Self::LilacSachet => "Lilac Sachet",
            Self::SweetLilac => "Sweet Lilac",
            Self::FragrantLilac => "Fragrant Lilac",
            Self::WinsomeOrchid => "Winsome Orchid",
            Self::PinkLavender => "Pink Lavender",
            Self::PastelLavender => "Pastel Lavender",
            Self::OrchidBloom => "Orchid Bloom",
            Self::OrchidPetal => "Orchid Petal",
            Self::HushedViolet => "Hushed Violet",
            Self::Iris => "Iris",
            Self::PastelLilac => "Pastel Lilac",
            Self::LilacMarble => "Lilac Marble",
            Self::EveningHaze => "Evening Haze",
            Self::LavenderBlue => "Lavender Blue",
            Self::Raindrops => "Raindrops",
            Self::Thistle => "Thistle",
            Self::PurpleHeather => "Purple Heather",
            Self::ZenBlue => "Zen Blue",
            Self::XenonBlue => "Xenon Blue",
            Self::WindChime => "Wind Chime",
            Self::GlacierGray => "Glacier Gray",
            Self::GrayViolet => "Gray Violet",
            Self::MicroChip => "Micro Chip",
            Self::GrayDawn => "Gray Dawn",
            Self::QuietGray => "Quiet Gray",
            Self::Heather => "Heather",
            Self::Skyway => "Skyway",
            Self::CashmereBlue => "Cashmere Blue",
            Self::BlueBell => "Blue Bell",
            Self::AiryBlue => "Airy Blue",
            Self::LunarRock => "Lunar Rock",
            Self::HarborMist => "Harbor Mist",
            Self::VaporBlue => "Vapor Blue",
            Self::PearlBlue => "Pearl Blue",
            Self::CelestialBlue => "Celestial Blue",
            Self::PowderBlue => "Powder Blue",
            Self::CloudBlue => "Cloud Blue",
            Self::WinterSky => "Winter Sky",
            Self::BlueTopaz => "Blue Topaz",
            Self::CorydalisBlue => "Corydalis Blue",
            Self::Aquamarine => "Aquamarine",
            Self::SeaAngel => "Sea Angel",
            Self::CoolBlue => "Cool Blue",
            Self::SkyBlue => "Sky Blue",
            Self::BalticSea => "Baltic Sea",
            Self::Moonstruck => "Moonstruck",
            Self::SilverLining => "Silver Lining",
            Self::Mercury => "Mercury",
            Self::Metal => "Metal",
            Self::SkyGray => "Sky Gray",
            Self::Smoke => "Smoke",
            Self::Ether => "Ether",
            Self::Stratosphere => "Stratosphere",
            Self::Aquatic => "Aquatic",
            Self::GulfStream => "Gulf Stream",
            Self::PorcelainBlue => "Porcelain Blue",
            Self::PetitFour => "Petit Four",
            Self::BachelorButton => "Bachelor Button",
            Self::AntiquaSand => "Antiqua Sand",
            Self::IslandParadise => "Island Paradise",
            Self::TibetanStone => "Tibetan Stone",
            Self::BlueFox => "Blue Fox",
            Self::SurfSpray => "Surf Spray",
            Self::EggshellBlue => "Eggshell Blue",
            Self::CanalBlue => "Canal Blue",
            Self::AquaSky => "Aqua Sky",
            Self::AquaSplash => "Aqua Splash",
            Self::AngelBlue => "Angel Blue",
            Self::BlueRadiance => "Blue Radiance",
            Self::HarborGray => "Harbor Gray",
            Self::Silver => "Silver",
            Self::Holiday => "Holiday",
            Self::Bermuda => "Bermuda",
            Self::Cockatoo => "Cockatoo",
            Self::SiltGreen => "Silt Green",
            Self::AquaFoam => "Aqua Foam",
            Self::OceanWave => "Ocean Wave",
            Self::Cascade => "Cascade",
            Self::BeveledGlass => "Beveled Glass",
            Self::Opal => "Opal",
            Self::ElectricGreen => "Electric Green",
            Self::SeaFoam => "Sea Foam",
            Self::SubtleGreen => "Subtle Green",
            Self::GrayedJade => "Grayed Jade",
            Self::NeptuneGreen => "Neptune Green",
            Self::Pelican => "Pelican",
            Self::Alfalfa => "Alfalfa",
            Self::CameoGreen => "Cameo Green",
            Self::Sprucestone => "Sprucestone",
            Self::Meadow => "Meadow",
            Self::Peapod => "Peapod",
            Self::ZephyrGreen => "Zephyr Green",
            Self::AbsintheGreen => "Absinthe Green",
            Self::SpringBud => "Spring Bud",
            Self::SpringBouquet => "Spring Bouquet",
            Self::AbbeyStone => "Abbey Stone",
            Self::Dove => "Dove",
            Self::GreenFlash => "Green Flash",
            Self::SprayGreen => "Spray Green",
            Self::SageGreen => "Sage Green",
            Self::Tarragon => "Tarragon",
            Self::LeafGreen => "Leaf Green",
            Self::HerbalGarden => "Herbal Garden",
            Self::ParrotGreen => "Parrot Green",
            Self::Greenery => "Greenery",
            Self::Eucalyptus => "Eucalyptus",
            Self::PaleOliveGreen => "Pale Olive Green",
            Self::WinterPear => "Winter Pear",
            Self::WeepingWillow => "Weeping Willow",
            Self::SweetPea => "Sweet Pea",
            Self::LindenGreen => "Linden Green",
            Self::Palm => "Palm",
            Self::GreenOasis => "Green Oasis",
            Self::AppleGreen => "Apple Green",
            Self::JasmineGreen => "Jasmine Green",
            Self::Citronelle => "Citronelle",
            Self::LeekGreen => "Leek Green",
            Self::GoldenGreen => "Golden Green",
            Self::CressGreen => "Cress Green",
            Self::WarmOlive => "Warm Olive",
            Self::AshesOfRoses => "Ashes Of Roses",
            Self::SilverFern => "Silver Fern",
            Self::SouthernMoss => "Southern Moss",
            Self::Olivenite => "Olivenite",
            Self::OilYellow => "Oil Yellow",
            Self::LemonCurry => "Lemon Curry",
            Self::CeylonYellow => "Ceylon Yellow",
            Self::PaleGold => "Pale Gold",
            Self::Sauterne => "Sauterne",
            Self::GoldenSpice => "Golden Spice",
            Self::GoldenYellow => "Golden Yellow",
            Self::OldGold => "Old Gold",
            Self::MangoMojito => "Mango Mojito",
            Self::GingerRoot => "Ginger Root",
            Self::IcedCoffee => "Iced Coffee",
            Self::AutumnBlaze => "Autumn Blaze",
            Self::MineralYellow => "Mineral Yellow",
            Self::ArtisansGold => "Artisans Gold",
            Self::GoldenGlow => "Golden Glow",
            Self::GoldenOrange => "Golden Orange",
            Self::CadmiumYellow => "Cadmium Yellow",
            Self::RadiantYellow => "Radiant Yellow",
            Self::GoldFusion => "Gold Fusion",
            Self::Travertine => "Travertine",
            Self::Safari => "Safari",
            Self::TaosTaupe => "Taos Taupe",
            Self::Porcini => "Porcini",
            Self::FallLeaf => "Fall Leaf",
            Self::HoneyGold => "Honey Gold",
            Self::Chamois => "Chamois",
            Self::Butterscotch => "Butterscotch",
            Self::DarkCheddar => "Dark Cheddar",
            Self::IcelandPoppy => "Iceland Poppy",
            Self::Apricot => "Apricot",
            Self::FlameOrange => "Flame Orange",
            Self::BlazingOrange => "Blazing Orange",
            Self::BrightMarigold => "Bright Marigold",
            Self::CandiedGinger => "Candied Ginger",
            Self::WarmSand => "Warm Sand",
            Self::Sesame => "Sesame",
            Self::PaleKhaki => "Pale Khaki",
            Self::MojaveDesert => "Mojave Desert",
            Self::Semolina => "Semolina",
            Self::Latte => "Latte",
            Self::Sand => "Sand",
            Self::Clay => "Clay",
            Self::GoldEarth => "Gold Earth",
            Self::ApricotTan => "Apricot Tan",
            Self::Cantaloupe => "Cantaloupe",
            Self::Muskmelon => "Muskmelon",
            Self::Papaya => "Papaya",
            Self::MockOrange => "Mock Orange",
            Self::Tangerine => "Tangerine",
            Self::CarrotCurl => "Carrot Curl",
            Self::AutumnGlory => "Autumn Glory",
            Self::Turmeric => "Turmeric",
            Self::Humus => "Humus",
            Self::FeatherGray => "Feather Gray",
            Self::OxfordTan => "Oxford Tan",
            Self::WhitePepper => "White Pepper",
            Self::Doeskin => "Doeskin",
            Self::Moonlight => "Moonlight",
            Self::CubanSand => "Cuban Sand",
            Self::RugbyTan => "Rugby Tan",
            Self::MapleSugar => "Maple Sugar",
            Self::Sirocco => "Sirocco",
            Self::PinkSand => "Pink Sand",
            Self::AlmostApricot => "Almost Apricot",
            Self::DustyCoral => "Dusty Coral",
            Self::PeachBloom => "Peach Bloom",
            Self::CoralReef => "Coral Reef",
            Self::CanyonSunset => "Canyon Sunset",
            Self::ShellCoral => "Shell Coral",
            Self::Tangelo => "Tangelo",
            Self::CadmiumOrange => "Cadmium Orange",
            Self::CoralCloud => "Coral Cloud",
            Self::PeachAmber => "Peach Amber",
            Self::Fenugreek => "Fenugreek",
            Self::DustedClay => "Dusted Clay",
            Self::PastryShell => "Pastry Shell",
            Self::PapayaPunch => "Papaya Punch",
            Self::DesertFlower => "Desert Flower",
            Self::Etherea => "Etherea",
            Self::MahoganyRose => "Mahogany Rose",
            Self::MistyRose => "Misty Rose",
            Self::MellowRose => "Mellow Rose",
            Self::PeachBeige => "Peach Beige",
            Self::BloomingDahlia => "Blooming Dahlia",
            Self::Shrimp => "Shrimp",
            Self::PeachPink => "Peach Pink",
            Self::PaleMauve => "Pale Mauve",
            Self::BridalRose => "Bridal Rose",
            Self::Blush => "Blush",
            Self::CandlelightPeach => "Candlelight Peach",
            Self::ConchShell => "Conch Shell",
            Self::SalmonRose => "Salmon Rose",
            Self::PinkIcing => "Pink Icing",
            Self::Peony => "Peony",
            Self::FlamingoPink => "Flamingo Pink",
            Self::BurnishedLilac => "Burnished Lilac",
            Self::Zephyr => "Zephyr",
            Self::SeaPink => "Sea Pink",
            Self::MorningGlory => "Morning Glory",
            Self::GeraniumPink => "Geranium Pink",
            Self::DawnPink => "Dawn Pink",
            Self::OrchidSmoke => "Orchid Smoke",
            Self::Rosebloom => "Rosebloom",
            Self::BegoniaPink => "Begonia Pink",
            Self::SachetPink => "Sachet Pink",
            Self::AuroraPink => "Aurora Pink",
            Self::KeepsakeLilac => "Keepsake Lilac",
            Self::VioletIce => "Violet Ice",
            Self::FuchsiaPink => "Fuchsia Pink",
            Self::LilacChiffon => "Lilac Chiffon",
            Self::MauveMist => "Mauve Mist",
            Self::Orchid => "Orchid",
            Self::OrchidBouquet => "Orchid Bouquet",
            Self::LavenderFrost => "Lavender Frost",
            Self::FairOrchid => "Fair Orchid",
            Self::CrocusPetal => "Crocus Petal",
            Self::Lavendula => "Lavendula",
            Self::PurpleRose => "Purple Rose",
            Self::LilacBreeze => "Lilac Breeze",
            Self::Porpoise => "Porpoise",
            Self::CloudGray => "Cloud Gray",
            Self::MistyLilac => "Misty Lilac",
            Self::Lavender => "Lavender",
            Self::IcelandicBlue => "Icelandic Blue",
            Self::CosmicSky => "Cosmic Sky",
            Self::LanguidLavender => "Languid Lavender",
            Self::Aleutian => "Aleutian",
            Self::KentuckyBlue => "Kentucky Blue",
            Self::Serenity => "Serenity",
            Self::PlacidBlue => "Placid Blue",
            Self::VistaBlue => "Vista Blue",
            Self::BelAirBlue => "Bel Air Blue",
            Self::StormGray => "Storm Gray",
            Self::DreamBlue => "Dream Blue",
            Self::BlueFog => "Blue Fog",
            Self::Cerulean => "Cerulean",
            Self::ChambrayBlue => "Chambray Blue",
            Self::HighRise => "High Rise",
            Self::AngelFalls => "Angel Falls",
            Self::AlaskanBlue => "Alaskan Blue",
            Self::Quarry => "Quarry",
            Self::BelgianBlock => "Belgian Block",
            Self::Tradewinds => "Tradewinds",
            Self::SterlingBlue => "Sterling Blue",
            Self::ForgetMeNot => "Forget Me Not",
            Self::AirBlue => "Air Blue",
            Self::EtherealBlue => "Ethereal Blue",
            Self::MilkyBlue => "Milky Blue",
            Self::BlueGrotto => "Blue Grotto",
            Self::NorseBlue => "Norse Blue",
            Self::CrystalSeas => "Crystal Seas",
            Self::SilverCloud => "Silver Cloud",
            Self::ChateauGray => "Chateau Gray",
            Self::PuritanGray => "Puritan Gray",
            Self::MirageGray => "Mirage Gray",
            Self::Pigeon => "Pigeon",
            Self::GrayMist => "Gray Mist",
            Self::BlueHaze => "Blue Haze",
            Self::MarineBlue => "Marine Blue",
            Self::Aquarelle => "Aquarelle",
            Self::AquaSea => "Aqua Sea",
            Self::Aqua => "Aqua",
            Self::RiverBlue => "River Blue",
            Self::Capri => "Capri",
            Self::BlueCuracao => "Blue Curacao",
            Self::AquaGray => "Aqua Gray",
            Self::Aquifer => "Aquifer",
            Self::AquaHaze => "Aqua Haze",
            Self::NileBlue => "Nile Blue",
            Self::BlueTurquoise => "Blue Turquoise",
            Self::PoolBlue => "Pool Blue",
            Self::FloridaKeys => "Florida Keys",
            Self::AquaGreen => "Aqua Green",
            Self::Atlantis => "Atlantis",
            Self::Waterfall => "Waterfall",
            Self::Turquoise => "Turquoise",
            Self::BrightGreen => "Bright Green",
            Self::MineralGray => "Mineral Gray",
            Self::FrostyGreen => "Frosty Green",
            Self::DustyJadeGreen => "Dusty Jade Green",
            Self::BiscayGreen => "Biscay Green",
            Self::MintLeaf => "Mint Leaf",
            Self::Lichen => "Lichen",
            Self::Spearmint => "Spearmint",
            Self::Hemlock => "Hemlock",
            Self::Ming => "Ming",
            Self::JadeCream => "Jade Cream",
            Self::PussywillowGray => "Pussywillow Gray",
            Self::AgateGray => "Agate Gray",
            Self::Swamp => "Swamp",
            Self::LaurelGreen => "Laurel Green",
            Self::SmokeGreen => "Smoke Green",
            Self::FairGreen => "Fair Green",
            Self::QuietGreen => "Quiet Green",
            Self::LightGrassGreen => "Light Grass Green",
            Self::IrishGreen => "Irish Green",
            Self::MossGray => "Moss Gray",
            Self::Reseda => "Reseda",
            Self::ForestShade => "Forest Shade",
            Self::GreenTea => "Green Tea",
            Self::Shamrock => "Shamrock",
            Self::GrassGreen => "Grass Green",
            Self::BudGreen => "Bud Green",
            Self::Paloma => "Paloma",
            Self::DesertSage => "Desert Sage",
            Self::Tendril => "Tendril",
            Self::VintageKhaki => "Vintage Khaki",
            Self::LondonFog => "London Fog",
            Self::Tea => "Tea",
            Self::Mistletoe => "Mistletoe",
            Self::GreenEyes => "Green Eyes",
            Self::JadeGreen => "Jade Green",
            Self::MacawGreen => "Macaw Green",
            Self::MeadowGreen => "Meadow Green",
            Self::Kiwi => "Kiwi",
            Self::Foliage => "Foliage",
            Self::Sage => "Sage",
            Self::Fern => "Fern",
            Self::DarkCitron => "Dark Citron",
            Self::SpinachGreen => "Spinach Green",
            Self::GrayGreen => "Gray Green",
            Self::Cedar => "Cedar",
            Self::Moss => "Moss",
            Self::Oasis => "Oasis",
            Self::GoldenLime => "Golden Lime",
            Self::SplitPea => "Split Pea",
            Self::LentilSprout => "Lentil Sprout",
            Self::Elm => "Elm",
            Self::Willow => "Willow",
            Self::GoldenOlive => "Golden Olive",
            Self::SlateGreen => "Slate Green",
            Self::Khaki => "Khaki",
            Self::AntiqueGold => "Antique Gold",
            Self::BurnishedGold => "Burnished Gold",
            Self::GreenSulphur => "Green Sulphur",
            Self::Goat => "Goat",
            Self::RichGold => "Rich Gold",
            Self::AntiqueMoss => "Antique Moss",
            Self::OliveOil => "Olive Oil",
            Self::SimplyTaupe => "Simply Taupe",
            Self::CurdsAndWhey => "Curds And Whey",
            Self::Croissant => "Croissant",
            Self::Curry => "Curry",
            Self::Taffy => "Taffy",
            Self::Tinsel => "Tinsel",
            Self::Honey => "Honey",
            Self::BrightGold => "Bright Gold",
            Self::HarvestGold => "Harvest Gold",
            Self::Narcissus => "Narcissus",
            Self::NuggetGold => "Nugget Gold",
            Self::TawnyOlive => "Tawny Olive",
            Self::Arrowwood => "Arrowwood",
            Self::Incense => "Incense",
            Self::Sunflower => "Sunflower",
            Self::PureCashmere => "Pure Cashmere",
            Self::Crockery => "Crockery",
            Self::PlazaTaupe => "Plaza Taupe",
            Self::Tuffet => "Tuffet",
            Self::Aluminum => "Aluminum",
            Self::Twill => "Twill",
            Self::Greige => "Greige",
            Self::OliveGray => "Olive Gray",
            Self::Sponge => "Sponge",
            Self::Starfish => "Starfish",
            Self::Antelope => "Antelope",
            Self::MustardGold => "Mustard Gold",
            Self::AmberGold => "Amber Gold",
            Self::Yam => "Yam",
            Self::GoldenNugget => "Golden Nugget",
            Self::HoneyYellow => "Honey Yellow",
            Self::OakBuff => "Oak Buff",
            Self::Nugget => "Nugget",
            Self::DesertSun => "Desert Sun",
            Self::Topaz => "Topaz",
            Self::OrangePepper => "Orange Pepper",
            Self::LightTaupe => "Light Taupe",
            Self::Nomad => "Nomad",
            Self::Tuscany => "Tuscany",
            Self::CafeCreme => "Cafe Creme",
            Self::Roebuck => "Roebuck",
            Self::Sandstorm => "Sandstorm",
            Self::OrangeOchre => "Orange Ochre",
            Self::RussetOrange => "Russet Orange",
            Self::SunOrange => "Sun Orange",
            Self::HarvestPumpkin => "Harvest Pumpkin",
            Self::String => "String",
            Self::Natural => "Natural",
            Self::Cornstalk => "Cornstalk",
            Self::Brush => "Brush",
            Self::WarmTaupe => "Warm Taupe",
            Self::Nougat => "Nougat",
            Self::Macaroon => "Macaroon",
            Self::Lark => "Lark",
            Self::Copper => "Copper",
            Self::PrairieSand => "Prairie Sand",
            Self::ToastedNut => "Toasted Nut",
            Self::Sandstone => "Sandstone",
            Self::CoralHaze => "Coral Haze",
            Self::MutedClay => "Muted Clay",
            Self::Toast => "Toast",
            Self::Pheasant => "Pheasant",
            Self::Doe => "Doe",
            Self::Tan => "Tan",
            Self::Biscuit => "Biscuit",
            Self::CoralGold => "Coral Gold",
            Self::CopperTan => "Copper Tan",
            Self::BrandiedMelon => "Brandied Melon",
            Self::Butterum => "Butterum",
            Self::Buckskin => "Buckskin",
            Self::AutumnSunset => "Autumn Sunset",
            Self::DustyOrange => "Dusty Orange",
            Self::SunBaked => "Sun Baked",
            Self::GoldenOchre => "Golden Ochre",
            Self::PeachCaramel => "Peach Caramel",
            Self::TomatoCream => "Tomato Cream",
            Self::CoralRose => "Coral Rose",
            Self::Amberglow => "Amberglow",
            Self::PersimmonOrange => "Persimmon Orange",
            Self::BirdOfParadise => "Bird Of Paradise",
            Self::OrangeTiger => "Orange Tiger",
            Self::OrangePeel => "Orange Peel",
            Self::Nectarine => "Nectarine",
            Self::Carrot => "Carrot",
            Self::VermillionOrange => "Vermillion Orange",
            Self::PuffinsBill => "Puffins Bill",
            Self::VibrantOrange => "Vibrant Orange",
            Self::Atmosphere => "Atmosphere",
            Self::Cobblestone => "Cobblestone",
            Self::Stucco => "Stucco",
            Self::Chanterelle => "Chanterelle",
            Self::Almondine => "Almondine",
            Self::Cork => "Cork",
            Self::Sunburn => "Sunburn",
            Self::CanyonClay => "Canyon Clay",
            Self::Almond => "Almond",
            Self::CoralAlmond => "Coral Almond",
            Self::Carnelian => "Carnelian",
            Self::Meerkat => "Meerkat",
            Self::Caramel => "Caramel",
            Self::Langoustino => "Langoustino",
            Self::Arabesque => "Arabesque",
            Self::Melon => "Melon",
            Self::ApricotBuff => "Apricot Buff",
            Self::BurntOrange => "Burnt Orange",
            Self::GoldFlame => "Gold Flame",
            Self::Flamingo => "Flamingo",
            Self::Nasturtium => "Nasturtium",
            Self::Firecracker => "Firecracker",
            Self::ExoticOrange => "Exotic Orange",
            Self::JaffaOrange => "Jaffa Orange",
            Self::MandarinOrange => "Mandarin Orange",
            Self::DragonFire => "Dragon Fire",
            Self::GoldenPoppy => "Golden Poppy",
            Self::Bark => "Bark",
            Self::AdobeRose => "Adobe Rose",
            Self::ShadowGray => "Shadow Gray",
            Self::Fawn => "Fawn",
            Self::RoseTan => "Rose Tan",
            Self::CameoBrown => "Cameo Brown",
            Self::Rosette => "Rosette",
            Self::LobsterBisque => "Lobster Bisque",
            Self::RoseDawn => "Rose Dawn",
            Self::TerraCotta => "Terra Cotta",
            Self::BurntCoral => "Burnt Coral",
            Self::Crabapple => "Crabapple",
            Self::Coral => "Coral",
            Self::Camellia => "Camellia",
            Self::FreshSalmon => "Fresh Salmon",
            Self::FusionCoral => "Fusion Coral",
            Self::Persimmon => "Persimmon",
            Self::CoralQuartz => "Coral Quartz",
            Self::LivingCoral => "Living Coral",
            Self::PeachEcho => "Peach Echo",
            Self::PurpleDove => "Purple Dove",
            Self::BrandiedApricot => "Brandied Apricot",
            Self::Mauveglow => "Mauveglow",
            Self::TeaRose => "Tea Rose",
            Self::Lantana => "Lantana",
            Self::PeachBlossom => "Peach Blossom",
            Self::ShellPink => "Shell Pink",
            Self::SugarCoral => "Sugar Coral",
            Self::GeorgiaPeach => "Georgia Peach",
            Self::Sphinx => "Sphinx",
            Self::DeauvilleMauve => "Deauville Mauve",
            Self::Lilas => "Lilas",
            Self::Foxglove => "Foxglove",
            Self::Polignac => "Polignac",
            Self::WildRose => "Wild Rose",
            Self::StrawberryIce => "Strawberry Ice",
            Self::Confetti => "Confetti",
            Self::StrawberryPink => "Strawberry Pink",
            Self::PinkLemonade => "Pink Lemonade",
            Self::Woodrose => "Woodrose",
            Self::OrchidHaze => "Orchid Haze",
            Self::MauveOrchid => "Mauve Orchid",
            Self::WildOrchid => "Wild Orchid",
            Self::PinkCarnation => "Pink Carnation",
            Self::AzaleaPink => "Azalea Pink",
            Self::CashmereRose => "Cashmere Rose",
            Self::MoonliteMauve => "Moonlite Mauve",
            Self::SmokyGrape => "Smoky Grape",
            Self::Crocus => "Crocus",
            Self::OperaMauve => "Opera Mauve",
            Self::Cyclamen => "Cyclamen",
            Self::MauveShadows => "Mauve Shadows",
            Self::SeaFog => "Sea Fog",
            Self::LavenderMist => "Lavender Mist",
            Self::LavenderHerb => "Lavender Herb",
            Self::Violet => "Violet",
            Self::VioletTulle => "Violet Tulle",
            Self::AfricanViolet => "African Violet",
            Self::Lupine => "Lupine",
            Self::RegalOrchid => "Regal Orchid",
            Self::SheerLilac => "Sheer Lilac",
            Self::SandVerbena => "Sand Verbena",
            Self::Satellite => "Satellite",
            Self::OpalGray => "Opal Gray",
            Self::Ash => "Ash",
            Self::GullGray => "Gull Gray",
            Self::Wisteria => "Wisteria",
            Self::HeirloomLilac => "Heirloom Lilac",
            Self::Viola => "Viola",
            Self::Rhapsody => "Rhapsody",
            Self::VioletTulip => "Violet Tulip",
            Self::SilverSconce => "Silver Sconce",
            Self::LilacGray => "Lilac Gray",
            Self::DappleGray => "Dapple Gray",
            Self::LavenderAura => "Lavender Aura",
            Self::Alloy => "Alloy",
            Self::Sleet => "Sleet",
            Self::Eventide => "Eventide",
            Self::LavenderLustre => "Lavender Lustre",
            Self::BlueHeron => "Blue Heron",
            Self::BrunneraBlue => "Brunnera Blue",
            Self::EasterEgg => "Easter Egg",
            Self::Grapemist => "Grapemist",
            Self::ThistleDown => "Thistle Down",
            Self::SweetLavender => "Sweet Lavender",
            Self::DustyBlue => "Dusty Blue",
            Self::AshleyBlue => "Ashley Blue",
            Self::ForeverBlue => "Forever Blue",
            Self::DellaRobbiaBlue => "Della Robbia Blue",
            Self::Allure => "Allure",
            Self::Hydrangea => "Hydrangea",
            Self::CornflowerBlue => "Cornflower Blue",
            Self::Provence => "Provence",
            Self::GranadaSky => "Granada Sky",
            Self::Arona => "Arona",
            Self::StoneBlue => "Stone Blue",
            Self::DuskBlue => "Dusk Blue",
            Self::HeritageBlue => "Heritage Blue",
            Self::LittleBoyBlue => "Little Boy Blue",
            Self::BonnieBlue => "Bonnie Blue",
            Self::MourningDove => "Mourning Dove",
            Self::Drizzle => "Drizzle",
            Self::Abyss => "Abyss",
            Self::Slate => "Slate",
            Self::Tourmaline => "Tourmaline",
            Self::CameoBlue => "Cameo Blue",
            Self::BlueMist => "Blue Mist",
            Self::HorizonBlue => "Horizon Blue",
            Self::DelphiniumBlue => "Delphinium Blue",
            Self::MauiBlue => "Maui Blue",
            Self::CyanBlue => "Cyan Blue",
            Self::Aquarius => "Aquarius",
            Self::BlueAtoll => "Blue Atoll",
            Self::Stillwater => "Stillwater",
            Self::ReefWaters => "Reef Waters",
            Self::Limestone => "Limestone",
            Self::GhostGray => "Ghost Gray",
            Self::SilverBlue => "Silver Blue",
            Self::MineralBlue => "Mineral Blue",
            Self::Porcelain => "Porcelain",
            Self::ScubaBlue => "Scuba Blue",
            Self::PeacockBlue => "Peacock Blue",
            Self::Bluebird => "Bluebird",
            Self::WetWeather => "Wet Weather",
            Self::BlueSurf => "Blue Surf",
            Self::Wasabi => "Wasabi",
            Self::Canton => "Canton",
            Self::DustyTurquoise => "Dusty Turquoise",
            Self::SeaBlue => "Sea Blue",
            Self::Meadowbrook => "Meadowbrook",
            Self::Baltic => "Baltic",
            Self::Ceramic => "Ceramic",
            Self::Jadeite => "Jadeite",
            Self::AgateGreen => "Agate Green",
            Self::Lagoon => "Lagoon",
            Self::SeaGreen => "Sea Green",
            Self::BrightAqua => "Bright Aqua",
            Self::PoolGreen => "Pool Green",
            Self::Billiard => "Billiard",
            Self::PeacockGreen => "Peacock Green",
            Self::BerylGreen => "Beryl Green",
            Self::Arcadia => "Arcadia",
            Self::MarineGreen => "Marine Green",
            Self::FlintGray => "Flint Gray",
            Self::SlateGray => "Slate Gray",
            Self::GreenMilieu => "Green Milieu",
            Self::LilyPad => "Lily Pad",
            Self::IcebergGreen => "Iceberg Green",
            Self::GreenBay => "Green Bay",
            Self::Feldspar => "Feldspar",
            Self::GreenSpruce => "Green Spruce",
            Self::GumdropGreen => "Gumdrop Green",
            Self::WroughtIron => "Wrought Iron",
            Self::GraniteGreen => "Granite Green",
            Self::MalachiteGreen => "Malachite Green",
            Self::CremeDeMenthe => "Creme De Menthe",
            Self::WinterGreen => "Winter Green",
            Self::MingGreen => "Ming Green",
            Self::HollyGreen => "Holly Green",
            Self::Mint => "Mint",
            Self::Blarney => "Blarney",
            Self::Seagrass => "Seagrass",
            Self::Katydid => "Katydid",
            Self::ShaleGreen => "Shale Green",
            Self::Greenbriar => "Greenbriar",
            Self::KellyGreen => "Kelly Green",
            Self::Basil => "Basil",
            Self::IslandGreen => "Island Green",
            Self::MineralGreen => "Mineral Green",
            Self::Jadesheen => "Jadesheen",
            Self::Peppermint => "Peppermint",
            Self::VibrantGreen => "Vibrant Green",
            Self::ClassicGreen => "Classic Green",
            Self::PoisonGreen => "Poison Green",
            Self::FrostGray => "Frost Gray",
            Self::OilGreen => "Oil Green",
            Self::TurfGreen => "Turf Green",
            Self::StoneGreen => "Stone Green",
            Self::FluoriteGreen => "Fluorite Green",
            Self::OnlineLime => "Online Lime",
            Self::ElephantSkin => "Elephant Skin",
            Self::RockRidge => "Rock Ridge",
            Self::LodenFrost => "Loden Frost",
            Self::AspenGreen => "Aspen Green",
            Self::Watercress => "Watercress",
            Self::ForestGreen => "Forest Green",
            Self::PiquantGreen => "Piquant Green",
            Self::Epsom => "Epsom",
            Self::TurtleGreen => "Turtle Green",
            Self::Peridot => "Peridot",
            Self::SilverSage => "Silver Sage",
            Self::DuskyGreen => "Dusky Green",
            Self::Mosstone => "Mosstone",
            Self::Guacamole => "Guacamole",
            Self::GreenOlive => "Green Olive",
            Self::LaurelOak => "Laurel Oak",
            Self::Vetiver => "Vetiver",
            Self::Mermaid => "Mermaid",
            Self::Aloe => "Aloe",
            Self::Boa => "Boa",
            Self::DriedHerb => "Dried Herb",
            Self::TreeHouse => "Tree House",
            Self::GreenMoss => "Green Moss",
            Self::TaupeGray => "Taupe Gray",
            Self::EcruOlive => "Ecru Olive",
            Self::GoldenPalm => "Golden Palm",
            Self::AmberGreen => "Amber Green",
            Self::BronzeMist => "Bronze Mist",
            Self::Fossil => "Fossil",
            Self::FennelSeed => "Fennel Seed",
            Self::DullGold => "Dull Gold",
            Self::MedalBronze => "Medal Bronze",
            Self::ChaiTea => "Chai Tea",
            Self::Dune => "Dune",
            Self::Elmwood => "Elmwood",
            Self::Kelp => "Kelp",
            Self::AntiqueBronze => "Antique Bronze",
            Self::Bistre => "Bistre",
            Self::TigersEye => "Tigers Eye",
            Self::SpruceYellow => "Spruce Yellow",
            Self::Chipmunk => "Chipmunk",
            Self::AppleCinnamon => "Apple Cinnamon",
            Self::GoldenOak => "Golden Oak",
            Self::HoneyMustard => "Honey Mustard",
            Self::IncaGold => "Inca Gold",
            Self::RoastedPecan => "Roasted Pecan",
            Self::RoastedCashew => "Roasted Cashew",
            Self::SenecaRock => "Seneca Rock",
            Self::WinterTwig => "Winter Twig",
            Self::Chinchilla => "Chinchilla",
            Self::Coriander => "Coriander",
            Self::PetrifiedOak => "Petrified Oak",
            Self::LeadGray => "Lead Gray",
            Self::Dijon => "Dijon",
            Self::BoneBrown => "Bone Brown",
            Self::WoodThrush => "Wood Thrush",
            Self::BrownSugar => "Brown Sugar",
            Self::Cashew => "Cashew",
            Self::Marmalade => "Marmalade",
            Self::ArganOil => "Argan Oil",
            Self::Hazel => "Hazel",
            Self::AutumnMaple => "Autumn Maple",
            Self::AmberBrown => "Amber Brown",
            Self::MoonRock => "Moon Rock",
            Self::Fungi => "Fungi",
            Self::Praline => "Praline",
            Self::Camel => "Camel",
            Self::TawnyBirch => "Tawny Birch",
            Self::TawnyBrown => "Tawny Brown",
            Self::CafeAuLait => "Cafe Au Lait",
            Self::MochaMousse => "Mocha Mousse",
            Self::TimberWolf => "Timber Wolf",
            Self::DesertTaupe => "Desert Taupe",
            Self::SilverMink => "Silver Mink",
            Self::SepiaTint => "Sepia Tint",
            Self::Portabella => "Portabella",
            Self::Amphora => "Amphora",
            Self::Tannin => "Tannin",
            Self::Woodsmoke => "Woodsmoke",
            Self::Burro => "Burro",
            Self::TobaccoBrown => "Tobacco Brown",
            Self::IndianTan => "Indian Tan",
            Self::Lion => "Lion",
            Self::Bran => "Bran",
            Self::Adobe => "Adobe",
            Self::TawnyOrange => "Tawny Orange",
            Self::Autumnal => "Autumnal",
            Self::SpiceRoute => "Spice Route",
            Self::AutumnLeaf => "Autumn Leaf",
            Self::Exuberance => "Exuberance",
            Self::OrangePopsicle => "Orange Popsicle",
            Self::ApricotOrange => "Apricot Orange",
            Self::CelosiaOrange => "Celosia Orange",
            Self::ScarletIbis => "Scarlet Ibis",
            Self::PineBark => "Pine Bark",
            Self::BeaverFur => "Beaver Fur",
            Self::GingerSnap => "Ginger Snap",
            Self::RawUmber => "Raw Umber",
            Self::BrickDust => "Brick Dust",
            Self::PecanBrown => "Pecan Brown",
            Self::RawSienna => "Raw Sienna",
            Self::Ginger => "Ginger",
            Self::Mango => "Mango",
            Self::PureedPumpkin => "Pureed Pumpkin",
            Self::SummerFig => "Summer Fig",
            Self::Koi => "Koi",
            Self::Tigerlily => "Tigerlily",
            Self::Orangeade => "Orangeade",
            Self::Flame => "Flame",
            Self::TangerineTango => "Tangerine Tango",
            Self::RedOrange => "Red Orange",
            Self::SteepleGray => "Steeple Gray",
            Self::WildDove => "Wild Dove",
            Self::Cloudburst => "Cloudburst",
            Self::StormFront => "Storm Front",
            Self::Quail => "Quail",
            Self::Cinder => "Cinder",
            Self::Antler => "Antler",
            Self::WistfulMauve => "Wistful Mauve",
            Self::NostalgiaRose => "Nostalgia Rose",
            Self::AshRose => "Ash Rose",
            Self::Burlwood => "Burlwood",
            Self::OldRose => "Old Rose",
            Self::CanyonRose => "Canyon Rose",
            Self::Mauvewood => "Mauvewood",
            Self::DesertSand => "Desert Sand",
            Self::CedarWood => "Cedar Wood",
            Self::Aragon => "Aragon",
            Self::MineralRed => "Mineral Red",
            Self::ApricotBrandy => "Apricot Brandy",
            Self::BurntSienna => "Burnt Sienna",
            Self::Cranberry => "Cranberry",
            Self::Emberglow => "Emberglow",
            Self::Paprika => "Paprika",
            Self::Grenadine => "Grenadine",
            Self::MandarinRed => "Mandarin Red",
            Self::CherryTomato => "Cherry Tomato",
            Self::Fiesta => "Fiesta",
            Self::Elderberry => "Elderberry",
            Self::HeatherRose => "Heather Rose",
            Self::MesaRose => "Mesa Rose",
            Self::DuskyOrchid => "Dusky Orchid",
            Self::MellowMauve => "Mellow Mauve",
            Self::DecoRose => "Deco Rose",
            Self::RoseWine => "Rose Wine",
            Self::HollyBerry => "Holly Berry",
            Self::RoseOfSharon => "Rose Of Sharon",
            Self::Chrysanthemum => "Chrysanthemum",
            Self::PorcelainRose => "Porcelain Rose",
            Self::SpicedCoral => "Spiced Coral",
            Self::Dubarry => "Dubarry",
            Self::Poinsettia => "Poinsettia",
            Self::HotCoral => "Hot Coral",
            Self::Bittersweet => "Bittersweet",
            Self::PoppyRed => "Poppy Red",
            Self::Moonscape => "Moonscape",
            Self::Bordeaux => "Bordeaux",
            Self::DustyRose => "Dusty Rose",
            Self::Malaga => "Malaga",
            Self::SunkistCoral => "Sunkist Coral",
            Self::ClaretRed => "Claret Red",
            Self::CalypsoCoral => "Calypso Coral",
            Self::Geranium => "Geranium",
            Self::ParadisePink => "Paradise Pink",
            Self::RedViolet => "Red Violet",
            Self::Carmine => "Carmine",
            Self::Azalea => "Azalea",
            Self::FruitDove => "Fruit Dove",
            Self::DesertRose => "Desert Rose",
            Self::Bubblegum => "Bubblegum",
            Self::RaptureRose => "Rapture Rose",
            Self::CamelliaRose => "Camellia Rose",
            Self::HotPink => "Hot Pink",
            Self::FuchsiaRose => "Fuchsia Rose",
            Self::FandangoPink => "Fandango Pink",
            Self::PinkYarrow => "Pink Yarrow",
            Self::Magenta => "Magenta",
            Self::ChateauRose => "Chateau Rose",
            Self::ShockingPink => "Shocking Pink",
            Self::LilacRose => "Lilac Rose",
            Self::CarmineRose => "Carmine Rose",
            Self::Toadstool => "Toadstool",
            Self::IbisRose => "Ibis Rose",
            Self::Zinc => "Zinc",
            Self::DahliaMauve => "Dahlia Mauve",
            Self::RoseViolet => "Rose Violet",
            Self::SuperPink => "Super Pink",
            Self::PhloxPink => "Phlox Pink",
            Self::Mulberry => "Mulberry",
            Self::SpringCrocus => "Spring Crocus",
            Self::Rosebud => "Rosebud",
            Self::Bodacious => "Bodacious",
            Self::DustyLavender => "Dusty Lavender",
            Self::IrisOrchid => "Iris Orchid",
            Self::Valerian => "Valerian",
            Self::DiffusedOrchid => "Diffused Orchid",
            Self::OrchidMist => "Orchid Mist",
            Self::ChalkViolet => "Chalk Violet",
            Self::EnglishLavendar => "English Lavendar",
            Self::Hyacinth => "Hyacinth",
            Self::AmethystOrchid => "Amethyst Orchid",
            Self::FairyWren => "Fairy Wren",
            Self::Bougainvillea => "Bougainvillea",
            Self::PaisleyPurple => "Paisley Purple",
            Self::SunlitAllium => "Sunlit Allium",
            Self::Gull => "Gull",
            Self::Nirvana => "Nirvana",
            Self::PurpleAsh => "Purple Ash",
            Self::Dusk => "Dusk",
            Self::Daybreak => "Daybreak",
            Self::AsterPurple => "Aster Purple",
            Self::DahliaPurple => "Dahlia Purple",
            Self::MinimalGray => "Minimal Gray",
            Self::Quicksilver => "Quicksilver",
            Self::LavenderGray => "Lavender Gray",
            Self::SilverFiligree => "Silver Filigree",
            Self::Sharkskin => "Sharkskin",
            Self::Tempest => "Tempest",
            Self::Stonewash => "Stonewash",
            Self::CountryBlue => "Country Blue",
            Self::PurpleImpression => "Purple Impression",
            Self::EnglishManor => "English Manor",
            Self::BlueIce => "Blue Ice",
            Self::ColonyBlue => "Colony Blue",
            Self::LavenderViolet => "Lavender Violet",
            Self::PersianViolet => "Persian Violet",
            Self::PaleIris => "Pale Iris",
            Self::Jacaranda => "Jacaranda",
            Self::DeepPeriwinkle => "Deep Periwinkle",
            Self::SilverBullet => "Silver Bullet",
            Self::PersianJewel => "Persian Jewel",
            Self::BlueBonnet => "Blue Bonnet",
            Self::Iolite => "Iolite",
            Self::Titanium => "Titanium",
            Self::Infinity => "Infinity",
            Self::GrayFlannel => "Gray Flannel",
            Self::BlueShadow => "Blue Shadow",
            Self::FadedDenim => "Faded Denim",
            Self::BlueHeaven => "Blue Heaven",
            Self::Riviera => "Riviera",
            Self::Riverside => "Riverside",
            Self::QuietHarbor => "Quiet Harbor",
            Self::SilverLakeBlue => "Silver Lake Blue",
            Self::LichenBlue => "Lichen Blue",
            Self::PacificCoast => "Pacific Coast",
            Self::Ultramarine => "Ultramarine",
            Self::Marina => "Marina",
            Self::Citadel => "Citadel",
            Self::Niagara => "Niagara",
            Self::CendreBlue => "Cendre Blue",
            Self::AzureBlue => "Azure Blue",
            Self::IbizaBlue => "Ibiza Blue",
            Self::DivaBlue => "Diva Blue",
            Self::AdriaticBlue => "Adriatic Blue",
            Self::BlueMoon => "Blue Moon",
            Self::Blithe => "Blithe",
            Self::NeutralGray => "Neutral Gray",
            Self::Monument => "Monument",
            Self::Lead => "Lead",
            Self::SmokeBlue => "Smoke Blue",
            Self::Larkspur => "Larkspur",
            Self::Bluejay => "Bluejay",
            Self::NavagioBay => "Navagio Bay",
            Self::VividBlue => "Vivid Blue",
            Self::DresdenBlue => "Dresden Blue",
            Self::MalibuBlue => "Malibu Blue",
            Self::BlueDanube => "Blue Danube",
            Self::BarrierReef => "Barrier Reef",
            Self::HawaiianOcean => "Hawaiian Ocean",
            Self::StormBlue => "Storm Blue",
            Self::PagodaBlue => "Pagoda Blue",
            Self::AlgiersBlue => "Algiers Blue",
            Self::CaneelBay => "Caneel Bay",
            Self::CapriBreeze => "Capri Breeze",
            Self::BristolBlue => "Bristol Blue",
            Self::Arctic => "Arctic",
            Self::Teal => "Teal",
            Self::LakeBlue => "Lake Blue",
            Self::TealBlue => "Teal Blue",
            Self::Navigate => "Navigate",
            Self::DeepPeacockBlue => "Deep Peacock Blue",
            Self::Lapis => "Lapis",
            Self::Griffin => "Griffin",
            Self::ChinoisGreen => "Chinois Green",
            Self::Trellis => "Trellis",
            Self::OilBlue => "Oil Blue",
            Self::GreenBlueSlate => "Green Blue Slate",
            Self::LatigoBay => "Latigo Bay",
            Self::ViridianGreen => "Viridian Green",
            Self::Columbia => "Columbia",
            Self::DynastyGreen => "Dynasty Green",
            Self::SpectraGreen => "Spectra Green",
            Self::PorcelainGreen => "Porcelain Green",
            Self::Alhambra => "Alhambra",
            Self::DeepSea => "Deep Sea",
            Self::Greenlake => "Greenlake",
            Self::DeepGreen => "Deep Green",
            Self::VividGreen => "Vivid Green",
            Self::Emerald => "Emerald",
            Self::BottleGreen => "Bottle Green",
            Self::Viridis => "Viridis",
            Self::Parakeet => "Parakeet",
            Self::DarkIvy => "Dark Ivy",
            Self::PineGreen => "Pine Green",
            Self::SimplyGreen => "Simply Green",
            Self::DeepMint => "Deep Mint",
            Self::LaurelWreath => "Laurel Wreath",
            Self::JellyBean => "Jelly Bean",
            Self::FernGreen => "Fern Green",
            Self::Shadow => "Shadow",
            Self::SeaSpray => "Sea Spray",
            Self::DeepGrassGreen => "Deep Grass Green",
            Self::MediumGreen => "Medium Green",
            Self::Kashmir => "Kashmir",
            Self::HedgeGreen => "Hedge Green",
            Self::MintGreen => "Mint Green",
            Self::SmokedPearl => "Smoked Pearl",
            Self::Kale => "Kale",
            Self::Dill => "Dill",
            Self::EnglishIvy => "English Ivy",
            Self::VineyardGreen => "Vineyard Green",
            Self::WillowBough => "Willow Bough",
            Self::ElmGreen => "Elm Green",
            Self::ArtichokeGreen => "Artichoke Green",
            Self::Cactus => "Cactus",
            Self::Treetop => "Treetop",
            Self::Castlerock => "Castlerock",
            Self::Mayfly => "Mayfly",
            Self::Pesto => "Pesto",
            Self::Gunmetal => "Gunmetal",
            Self::DeepLichenGreen => "Deep Lichen Green",
            Self::Olivine => "Olivine",
            Self::BronzeGreen => "Bronze Green",
            Self::Cypress => "Cypress",
            Self::CallisteGreen => "Calliste Green",
            Self::CedarGreen => "Cedar Green",
            Self::TwistOfLime => "Twist Of Lime",
            Self::Grasshopper => "Grasshopper",
            Self::DarkGullGray => "Dark Gull Gray",
            Self::FourLeafClover => "Four Leaf Clover",
            Self::LodenGreen => "Loden Green",
            Self::CapuletOlive => "Capulet Olive",
            Self::Avocado => "Avocado",
            Self::CallaGreen => "Calla Green",
            Self::Gargoyle => "Gargoyle",
            Self::CastorGray => "Castor Gray",
            Self::BungeeCord => "Bungee Cord",
            Self::DustyOlive => "Dusty Olive",
            Self::BurntOlive => "Burnt Olive",
            Self::WinterMoss => "Winter Moss",
            Self::Iguana => "Iguana",
            Self::OliveBranch => "Olive Branch",
            Self::Woodbine => "Woodbine",
            Self::CharcoalGray => "Charcoal Gray",
            Self::StoneGray => "Stone Gray",
            Self::CovertGreen => "Covert Green",
            Self::OliveDrab => "Olive Drab",
            Self::MartiniOlive => "Martini Olive",
            Self::FirGreen => "Fir Green",
            Self::Lizard => "Lizard",
            Self::GothicOlive => "Gothic Olive",
            Self::Capers => "Capers",
            Self::Nutria => "Nutria",
            Self::Butternut => "Butternut",
            Self::Plantation => "Plantation",
            Self::DriedTobacco => "Dried Tobacco",
            Self::Tapenade => "Tapenade",
            Self::Kangaroo => "Kangaroo",
            Self::Sepia => "Sepia",
            Self::CoffeeLiqueur => "Coffee Liqueur",
            Self::Rubber => "Rubber",
            Self::BuckthornBrown => "Buckthorn Brown",
            Self::BronzeBrown => "Bronze Brown",
            Self::Cumin => "Cumin",
            Self::GoldenBrown => "Golden Brown",
            Self::CathaySpice => "Cathay Spice",
            Self::Shitake => "Shitake",
            Self::Cub => "Cub",
            Self::Caribou => "Caribou",
            Self::Otter => "Otter",
            Self::Ermine => "Ermine",
            Self::Bison => "Bison",
            Self::Emperador => "Emperador",
            Self::ToastedCoconut => "Toasted Coconut",
            Self::Thrush => "Thrush",
            Self::Toffee => "Toffee",
            Self::Dachshund => "Dachshund",
            Self::MonksRobe => "Monks Robe",
            Self::ThaiCurry => "Thai Curry",
            Self::HoneyGinger => "Honey Ginger",
            Self::FallenRock => "Fallen Rock",
            Self::Brindle => "Brindle",
            Self::Walnut => "Walnut",
            Self::Partridge => "Partridge",
            Self::Aztec => "Aztec",
            Self::Rawhide => "Rawhide",
            Self::MochaBisque => "Mocha Bisque",
            Self::LeatherBrown => "Leather Brown",
            Self::CaramelCafe => "Caramel Cafe",
            Self::GlazedGinger => "Glazed Ginger",
            Self::SugarAlmond => "Sugar Almond",
            Self::SudanBrown => "Sudan Brown",
            Self::PumpkinSpice => "Pumpkin Spice",
            Self::Driftwood => "Driftwood",
            Self::CocoaBrown => "Cocoa Brown",
            Self::CarobBrown => "Carob Brown",
            Self::CoconutShell => "Coconut Shell",
            Self::Russet => "Russet",
            Self::RusticBrown => "Rustic Brown",
            Self::Sierra => "Sierra",
            Self::BrownPatina => "Brown Patina",
            Self::GingerBread => "Ginger Bread",
            Self::Umber => "Umber",
            Self::Rust => "Rust",
            Self::HawaiianSunset => "Hawaiian Sunset",
            Self::BombayBrown => "Bombay Brown",
            Self::Falcon => "Falcon",
            Self::Iron => "Iron",
            Self::DeepTaupe => "Deep Taupe",
            Self::Acorn => "Acorn",
            Self::Clove => "Clove",
            Self::Brownie => "Brownie",
            Self::SpicedApple => "Spiced Apple",
            Self::Nutmeg => "Nutmeg",
            Self::CopperBrown => "Copper Brown",
            Self::PottersClay => "Potters Clay",
            Self::Auburn => "Auburn",
            Self::CinnamonStick => "Cinnamon Stick",
            Self::Bruschetta => "Bruschetta",
            Self::BurntBrick => "Burnt Brick",
            Self::BurntOchre => "Burnt Ochre",
            Self::RooibosTea => "Rooibos Tea",
            Self::Sparrow => "Sparrow",
            Self::Flint => "Flint",
            Self::Peppercorn => "Peppercorn",
            Self::PlumWine => "Plum Wine",
            Self::Marron => "Marron",
            Self::CrushedBerry => "Crushed Berry",
            Self::WildGinger => "Wild Ginger",
            Self::Cognac => "Cognac",
            Self::Mahogany => "Mahogany",
            Self::AppleButter => "Apple Butter",
            Self::Chutney => "Chutney",
            Self::EtruscanRed => "Etruscan Red",
            Self::WitheredRose => "Withered Rose",
            Self::LightMahogany => "Light Mahogany",
            Self::Marsala => "Marsala",
            Self::ChiliOil => "Chili Oil",
            Self::BakedClay => "Baked Clay",
            Self::RedOchre => "Red Ochre",
            Self::Redwood => "Redwood",
            Self::TandoriSpice => "Tandori Spice",
            Self::SpicyOrange => "Spicy Orange",
            Self::OrangeRust => "Orange Rust",
            Self::Chili => "Chili",
            Self::Ketchup => "Ketchup",
            Self::MeccaOrange => "Mecca Orange",
            Self::AutumnGlaze => "Autumn Glaze",
            Self::RedClay => "Red Clay",
            Self::PlumTruffle => "Plum Truffle",
            Self::RoseBrown => "Rose Brown",
            Self::BarnRed => "Barn Red",
            Self::GingerSpice => "Ginger Spice",
            Self::HotSauce => "Hot Sauce",
            Self::CopperCoin => "Copper Coin",
            Self::Cinnabar => "Cinnabar",
            Self::BrandyBrown => "Brandy Brown",
            Self::BossaNova => "Bossa Nova",
            Self::ValiantPoppy => "Valiant Poppy",
            Self::AuroraRed => "Aurora Red",
            Self::AuraOrange => "Aura Orange",
            Self::MoltenLava => "Molten Lava",
            Self::OrangeCom => "Orange Com",
            Self::Poinciana => "Poinciana",
            Self::RoseTaupe => "Rose Taupe",
            Self::RenaissanceRose => "Renaissance Rose",
            Self::Nocturne => "Nocturne",
            Self::RoanRouge => "Roan Rouge",
            Self::Maroon => "Maroon",
            Self::FadedRose => "Faded Rose",
            Self::DustyCedar => "Dusty Cedar",
            Self::EarthRed => "Earth Red",
            Self::GarnetRose => "Garnet Rose",
            Self::BaroqueRose => "Baroque Rose",
            Self::SlateRose => "Slate Rose",
            Self::Cardinal => "Cardinal",
            Self::BakedApple => "Baked Apple",
            Self::DeepSeaCoral => "Deep Sea Coral",
            Self::Cayenne => "Cayenne",
            Self::RococcoRed => "Rococco Red",
            Self::Toreador => "Toreador",
            Self::Lychee => "Lychee",
            Self::MarsRed => "Mars Red",
            Self::Salsa => "Salsa",
            Self::PompeianRed => "Pompeian Red",
            Self::GojiBerry => "Goji Berry",
            Self::Tomato => "Tomato",
            Self::TomatoPuree => "Tomato Puree",
            Self::FlameScarlet => "Flame Scarlet",
            Self::ChineseRed => "Chinese Red",
            Self::FieryRed => "Fiery Red",
            Self::Shark => "Shark",
            Self::ArcticDusk => "Arctic Dusk",
            Self::BlackPlum => "Black Plum",
            Self::Ephemera => "Ephemera",
            Self::Tulipwood => "Tulipwood",
            Self::GrapeNectar => "Grape Nectar",
            Self::Damson => "Damson",
            Self::HawthornRose => "Hawthorn Rose",
            Self::VioletQuartz => "Violet Quartz",
            Self::DryRose => "Dry Rose",
            Self::RaspberryWine => "Raspberry Wine",
            Self::Raspberry => "Raspberry",
            Self::RougeRed => "Rouge Red",
            Self::Teaberry => "Teaberry",
            Self::JalapenoRed => "Jalapeno Red",
            Self::Barberry => "Barberry",
            Self::SkiPatrol => "Ski Patrol",
            Self::Hibiscus => "Hibiscus",
            Self::HighRiskRed => "High Risk Red",
            Self::Lollipop => "Lollipop",
            Self::TwilightMauve => "Twilight Mauve",
            Self::RoseRed => "Rose Red",
            Self::VirtualPink => "Virtual Pink",
            Self::BrightRose => "Bright Rose",
            Self::Jazzy => "Jazzy",
            Self::LovePotion => "Love Potion",
            Self::Beaujolais => "Beaujolais",
            Self::RaspberrySorbet => "Raspberry Sorbet",
            Self::PinkPeacock => "Pink Peacock",
            Self::GrapeShake => "Grape Shake",
            Self::Honeysuckle => "Honeysuckle",
            Self::PinkFlambe => "Pink Flambe",
            Self::Cabaret => "Cabaret",
            Self::BeetrootPurple => "Beetroot Purple",
            Self::Clover => "Clover",
            Self::CactusFlower => "Cactus Flower",
            Self::FuchsiaRed => "Fuchsia Red",
            Self::RaspberryRose => "Raspberry Rose",
            Self::VeryBerry => "Very Berry",
            Self::FuchsiaPurple => "Fuchsia Purple",
            Self::MagentaHaze => "Magenta Haze",
            Self::BatonRouge => "Baton Rouge",
            Self::PurpleWine => "Purple Wine",
            Self::ArgylePurple => "Argyle Purple",
            Self::PurpleGumdrop => "Purple Gumdrop",
            Self::BerryConserve => "Berry Conserve",
            Self::GrapeKiss => "Grape Kiss",
            Self::Amethyst => "Amethyst",
            Self::DeepOrchid => "Deep Orchid",
            Self::StrikingPurple => "Striking Purple",
            Self::PurpleOrchid => "Purple Orchid",
            Self::Willowherb => "Willowherb",
            Self::Grapeade => "Grapeade",
            Self::ConcordGrape => "Concord Grape",
            Self::VeryGrape => "Very Grape",
            Self::RadiantOrchid => "Radiant Orchid",
            Self::MeadowMauve => "Meadow Mauve",
            Self::Dahlia => "Dahlia",
            Self::HyacinthViolet => "Hyacinth Violet",
            Self::VividViola => "Vivid Viola",
            Self::Charisma => "Charisma",
            Self::VintageViolet => "Vintage Violet",
            Self::GrapeJam => "Grape Jam",
            Self::ChineseViolet => "Chinese Violet",
            Self::GrapeCompote => "Grape Compote",
            Self::PatricianPurple => "Patrician Purple",
            Self::PurpleHeart => "Purple Heart",
            Self::PlumJam => "Plum Jam",
            Self::CrushedGrape => "Crushed Grape",
            Self::LavenderCrystal => "Lavender Crystal",
            Self::RoyalLilac => "Royal Lilac",
            Self::Dewberry => "Dewberry",
            Self::PurpleSapphire => "Purple Sapphire",
            Self::ImperialPalace => "Imperial Palace",
            Self::Mystical => "Mystical",
            Self::Bellflower => "Bellflower",
            Self::DeepLavender => "Deep Lavender",
            Self::ChiveBlossom => "Chive Blossom",
            Self::PicassoLily => "Picasso Lily",
            Self::GrayRidge => "Gray Ridge",
            Self::PurpleSage => "Purple Sage",
            Self::MulledGrape => "Mulled Grape",
            Self::MontanaGrape => "Montana Grape",
            Self::PurpleHaze => "Purple Haze",
            Self::PassionFlower => "Passion Flower",
            Self::Cadet => "Cadet",
            Self::Heron => "Heron",
            Self::TwilightPurple => "Twilight Purple",
            Self::CorsicanBlue => "Corsican Blue",
            Self::DustedPeri => "Dusted Peri",
            Self::Veronica => "Veronica",
            Self::UltraViolet => "Ultra Violet",
            Self::PurpleCorallite => "Purple Corallite",
            Self::PurpleOpulence => "Purple Opulence",
            Self::Excalibur => "Excalibur",
            Self::Tornado => "Tornado",
            Self::VolcanicGlass => "Volcanic Glass",
            Self::FolkstoneGray => "Folkstone Gray",
            Self::Grisaille => "Grisaille",
            Self::FlintStone => "Flint Stone",
            Self::GrayBlue => "Gray Blue",
            Self::ChinaBlue => "China Blue",
            Self::CoastalFjord => "Coastal Fjord",
            Self::BijouBlue => "Bijou Blue",
            Self::CoronetBlue => "Coronet Blue",
            Self::VelvetMorning => "Velvet Morning",
            Self::DutchBlue => "Dutch Blue",
            Self::BlueHorizon => "Blue Horizon",
            Self::BleachedDenim => "Bleached Denim",
            Self::Marlin => "Marlin",
            Self::BlueGranite => "Blue Granite",
            Self::Wedgewood => "Wedgewood",
            Self::BlueYonder => "Blue Yonder",
            Self::BlueIris => "Blue Iris",
            Self::VioletStorm => "Violet Storm",
            Self::AmparoBlue => "Amparo Blue",
            Self::BajaBlue => "Baja Blue",
            Self::DazzlingBlue => "Dazzling Blue",
            Self::IrisBloom => "Iris Bloom",
            Self::SpectrumBlue => "Spectrum Blue",
            Self::SteelGray => "Steel Gray",
            Self::QuietShade => "Quiet Shade",
            Self::GoblinBlue => "Goblin Blue",
            Self::DecemberSky => "December Sky",
            Self::RealTeal => "Real Teal",
            Self::CaptainsBlue => "Captains Blue",
            Self::BlueAshes => "Blue Ashes",
            Self::CopenBlue => "Copen Blue",
            Self::Stellar => "Stellar",
            Self::MoonlightBlue => "Moonlight Blue",
            Self::BeringSea => "Bering Sea",
            Self::FederalBlue => "Federal Blue",
            Self::DeepWater => "Deep Water",
            Self::VallartaBlue => "Vallarta Blue",
            Self::ParisianBlue => "Parisian Blue",
            Self::Regatta => "Regatta",
            Self::StarSapphire => "Star Sapphire",
            Self::PalaceBlue => "Palace Blue",
            Self::Daphne => "Daphne",
            Self::NebulasBlue => "Nebulas Blue",
            Self::StrongBlue => "Strong Blue",
            Self::MoonMist => "Moon Mist",
            Self::FrenchBlue => "French Blue",
            Self::Campanula => "Campanula",
            Self::VictoriaBlue => "Victoria Blue",
            Self::StormyWeather => "Stormy Weather",
            Self::BlueMirage => "Blue Mirage",
            Self::Bluestone => "Bluestone",
            Self::ProvincialBlue => "Provincial Blue",
            Self::Bluesteel => "Bluesteel",
            Self::SaxonyBlue => "Saxony Blue",
            Self::BlueSapphire => "Blue Sapphire",
            Self::Faience => "Faience",
            Self::DirectoireBlue => "Directoire Blue",
            Self::BrilliantBlue => "Brilliant Blue",
            Self::IndigoBunting => "Indigo Bunting",
            Self::BlueAster => "Blue Aster",
            Self::AegeanBlue => "Aegean Blue",
            Self::SwedishBlue => "Swedish Blue",
            Self::MediterraneanBlue => "Mediterranean Blue",
            Self::Tapestry => "Tapestry",
            Self::FjordBlue => "Fjord Blue",
            Self::TurkishTile => "Turkish Tile",
            Self::MykonosBlue => "Mykonos Blue",
            Self::Cloisonne => "Cloisonne",
            Self::Trooper => "Trooper",
            Self::ColonialBlue => "Colonial Blue",
            Self::CaribbeanSea => "Caribbean Sea",
            Self::MosaicBlue => "Mosaic Blue",
            Self::Celestial => "Celestial",
            Self::BlueJewel => "Blue Jewel",
            Self::MethylBlue => "Methyl Blue",
            Self::HawaiianSurf => "Hawaiian Surf",
            Self::NorthAtlantic => "North Atlantic",
            Self::TahitianTide => "Tahitian Tide",
            Self::StormySea => "Stormy Sea",
            Self::Hydro => "Hydro",
            Self::BiscayBay => "Biscay Bay",
            Self::HarborBlue => "Harbor Blue",
            Self::EnamelBlue => "Enamel Blue",
            Self::TileBlue => "Tile Blue",
            Self::DeepLake => "Deep Lake",
            Self::TropicalGreen => "Tropical Green",
            Self::Fanfare => "Fanfare",
            Self::Parasailing => "Parasailing",
            Self::QuetzalGreen => "Quetzal Green",
            Self::BrushedNickel => "Brushed Nickel",
            Self::SedonaSage => "Sedona Sage",
            Self::SeaPine => "Sea Pine",
            Self::NorthSea => "North Sea",
            Self::Bayou => "Bayou",
            Self::BlueGrass => "Blue Grass",
            Self::Pewter => "Pewter",
            Self::GraniteGray => "Granite Gray",
            Self::EiffelTower => "Eiffel Tower",
            Self::BlueSpruce => "Blue Spruce",
            Self::Bayberry => "Bayberry",
            Self::AlpineGreen => "Alpine Green",
            Self::UltramarineGreen => "Ultramarine Green",
            Self::SilverPine => "Silver Pine",
            Self::AntiqueGreen => "Antique Green",
            Self::CadmiumGreen => "Cadmium Green",
            Self::BalsamGreen => "Balsam Green",
            Self::BrittanyBlue => "Brittany Blue",
            Self::DarkForest => "Dark Forest",
            Self::SagebrushGreen => "Sagebrush Green",
            Self::PosyGreen => "Posy Green",
            Self::DeepJungle => "Deep Jungle",
            Self::Tidepool => "Tidepool",
            Self::Ivy => "Ivy",
            Self::Fir => "Fir",
            Self::FrostySpruce => "Frosty Spruce",
            Self::ShadyGlade => "Shady Glade",
            Self::Bosphorus => "Bosphorus",
            Self::GolfGreen => "Golf Green",
            Self::SmokePine => "Smoke Pine",
            Self::GalapagosGreen => "Galapagos Green",
            Self::AgaveGreen => "Agave Green",
            Self::PepperGreen => "Pepper Green",
            Self::LushMeadow => "Lush Meadow",
            Self::GardenTopiary => "Garden Topiary",
            Self::DuckGreen => "Duck Green",
            Self::FoliageGreen => "Foliage Green",
            Self::Leprechaun => "Leprechaun",
            Self::Amazon => "Amazon",
            Self::JollyGreen => "Jolly Green",
            Self::Myrtle => "Myrtle",
            Self::Comfrey => "Comfrey",
            Self::Fairway => "Fairway",
            Self::Juniper => "Juniper",
            Self::Raven => "Raven",
            Self::Asphalt => "Asphalt",
            Self::GrayPinstripe => "Gray Pinstripe",
            Self::GardenGreen => "Garden Green",
            Self::JetBlack => "Jet Black",
            Self::ClimbingIvy => "Climbing Ivy",
            Self::Thyme => "Thyme",
            Self::Beetle => "Beetle",
            Self::BlackForest => "Black Forest",
            Self::Chive => "Chive",
            Self::SeaTurtle => "Sea Turtle",
            Self::Beluga => "Beluga",
            Self::DeepDepths => "Deep Depths",
            Self::ForestNight => "Forest Night",
            Self::DuffelBag => "Duffel Bag",
            Self::KombuGreen => "Kombu Green",
            Self::RifleGreen => "Rifle Green",
            Self::BlackInk => "Black Ink",
            Self::Peat => "Peat",
            Self::Rosin => "Rosin",
            Self::Kalamata => "Kalamata",
            Self::GrapeLeaf => "Grape Leaf",
            Self::IvyGreen => "Ivy Green",
            Self::OliveNight => "Olive Night",
            Self::DarkOlive => "Dark Olive",
            Self::BlackOlive => "Black Olive",
            Self::Wren => "Wren",
            Self::Teak => "Teak",
            Self::Beech => "Beech",
            Self::MilitaryOlive => "Military Olive",
            Self::Demitasse => "Demitasse",
            Self::Morel => "Morel",
            Self::ChocolateChip => "Chocolate Chip",
            Self::MajorBrown => "Major Brown",
            Self::TurkishCoffee => "Turkish Coffee",
            Self::SlateBlack => "Slate Black",
            Self::DesertPalm => "Desert Palm",
            Self::Canteen => "Canteen",
            Self::Tarmac => "Tarmac",
            Self::Crocodile => "Crocodile",
            Self::Delicioso => "Delicioso",
            Self::Mulch => "Mulch",
            Self::ChocolateBrown => "Chocolate Brown",
            Self::CoffeeBean => "Coffee Bean",
            Self::RainDrum => "Rain Drum",
            Self::FrenchRoast => "French Roast",
            Self::Bracken => "Bracken",
            Self::Java => "Java",
            Self::Ganache => "Ganache",
            Self::DarkEarth => "Dark Earth",
            Self::Breen => "Breen",
            Self::AfterDark => "After Dark",
            Self::Licorice => "Licorice",
            Self::Espresso => "Espresso",
            Self::Mole => "Mole",
            Self::ChocolateTorte => "Chocolate Torte",
            Self::ChocolatePlum => "Chocolate Plum",
            Self::BlackCoffee => "Black Coffee",
            Self::Carafe => "Carafe",
            Self::Chestnut => "Chestnut",
            Self::Pinecone => "Pinecone",
            Self::ShoppingBag => "Shopping Bag",
            Self::ChocolateLab => "Chocolate Lab",
            Self::ShavedChocolate => "Shaved Chocolate",
            Self::Mustang => "Mustang",
            Self::PottingSoil => "Potting Soil",
            Self::Cappuccino => "Cappuccino",
            Self::FondueFudge => "Fondue Fudge",
            Self::RootBeer => "Root Beer",
            Self::FriarBrown => "Friar Brown",
            Self::Tiramisu => "Tiramisu",
            Self::RockyRoad => "Rocky Road",
            Self::Brunette => "Brunette",
            Self::TortoiseShell => "Tortoise Shell",
            Self::ArabianSpice => "Arabian Spice",
            Self::Picante => "Picante",
            Self::SealBrown => "Seal Brown",
            Self::BitterChocolate => "Bitter Chocolate",
            Self::Sable => "Sable",
            Self::RumRaisin => "Rum Raisin",
            Self::BrownStone => "Brown Stone",
            Self::HotChocolate => "Hot Chocolate",
            Self::Andorra => "Andorra",
            Self::MadderBrown => "Madder Brown",
            Self::Sequoia => "Sequoia",
            Self::Henna => "Henna",
            Self::FiredBrick => "Fired Brick",
            Self::RussetBrown => "Russet Brown",
            Self::ChicoryCoffee => "Chicory Coffee",
            Self::DeepMahogany => "Deep Mahogany",
            Self::SmokedPaprika => "Smoked Paprika",
            Self::Mink => "Mink",
            Self::Fudgesickle => "Fudgesickle",
            Self::ChocolateFondant => "Chocolate Fondant",
            Self::CherryMahogany => "Cherry Mahogany",
            Self::Cinnamon => "Cinnamon",
            Self::Puce => "Puce",
            Self::RedMahogany => "Red Mahogany",
            Self::Zinfandel => "Zinfandel",
            Self::OxbloodRed => "Oxblood Red",
            Self::Port => "Port",
            Self::ChocolateTruffle => "Chocolate Truffle",
            Self::WindsorWine => "Windsor Wine",
            Self::BurntRusset => "Burnt Russet",
            Self::SunDriedTomato => "Sun Dried Tomato",
            Self::Rosewood => "Rosewood",
            Self::Cowhide => "Cowhide",
            Self::Merlot => "Merlot",
            Self::Syrah => "Syrah",
            Self::RedPear => "Red Pear",
            Self::BurntHenna => "Burnt Henna",
            Self::BrickRed => "Brick Red",
            Self::RedDahlia => "Red Dahlia",
            Self::ChiliPepper => "Chili Pepper",
            Self::ScarletSage => "Scarlet Sage",
            Self::Raisin => "Raisin",
            Self::PrunePurple => "Prune Purple",
            Self::Burgundy => "Burgundy",
            Self::Fudge => "Fudge",
            Self::Huckleberry => "Huckleberry",
            Self::CatawbaGrape => "Catawba Grape",
            Self::VineyardWine => "Vineyard Wine",
            Self::Sassafras => "Sassafras",
            Self::DecadentChocolate => "Decadent Chocolate",
            Self::PortRoyale => "Port Royale",
            Self::RubyWine => "Ruby Wine",
            Self::BikingRed => "Biking Red",
            Self::Rhubarb => "Rhubarb",
            Self::Garnet => "Garnet",
            Self::RioRed => "Rio Red",
            Self::Samba => "Samba",
            Self::RibbonRed => "Ribbon Red",
            Self::TrueRed => "True Red",
            Self::MauveWine => "Mauve Wine",
            Self::Fig => "Fig",
            Self::Cabernet => "Cabernet",
            Self::TawnyPort => "Tawny Port",
            Self::Cordovan => "Cordovan",
            Self::BarbadosCherry => "Barbados Cherry",
            Self::HauteRed => "Haute Red",
            Self::AmericanBeauty => "American Beauty",
            Self::Scarlet => "Scarlet",
            Self::TangoRed => "Tango Red",
            Self::Crimson => "Crimson",
            Self::RacingRed => "Racing Red",
            Self::LipstickRed => "Lipstick Red",
            Self::DeepClaret => "Deep Claret",
            Self::RedBud => "Red Bud",
            Self::PersianRed => "Persian Red",
            Self::JesterRed => "Jester Red",
            Self::Scooter => "Scooter",
            Self::Pomegranate => "Pomegranate",
            Self::TibetanRed => "Tibetan Red",
            Self::RumbaRed => "Rumba Red",
            Self::Cerise => "Cerise",
            Self::Hortensia => "Hortensia",
            Self::Prune => "Prune",
            Self::Rhododendron => "Rhododendron",
            Self::RedPlum => "Red Plum",
            Self::BeetRed => "Beet Red",
            Self::Anemone => "Anemone",
            Self::Granita => "Granita",
            Self::CherriesJubilee => "Cherries Jubilee",
            Self::Vivacious => "Vivacious",
            Self::Sangria => "Sangria",
            Self::Winetasting => "Winetasting",
            Self::Eggplant => "Eggplant",
            Self::CrushedViolets => "Crushed Violets",
            Self::GrapeWine => "Grape Wine",
            Self::Amaranth => "Amaranth",
            Self::PickledBeet => "Pickled Beet",
            Self::MagentaPurple => "Magenta Purple",
            Self::PlumCaspia => "Plum Caspia",
            Self::PurplePotion => "Purple Potion",
            Self::Boysenberry => "Boysenberry",
            Self::RaspberryRadiance => "Raspberry Radiance",
            Self::FestivalFuchsia => "Festival Fuchsia",
            Self::ItalianPlum => "Italian Plum",
            Self::PotentPurple => "Potent Purple",
            Self::DarkPurple => "Dark Purple",
            Self::WinterBloom => "Winter Bloom",
            Self::WildAster => "Wild Aster",
            Self::Wineberry => "Wineberry",
            Self::BlackberryWine => "Blackberry Wine",
            Self::Phlox => "Phlox",
            Self::Hollyhock => "Hollyhock",
            Self::Gloxinia => "Gloxinia",
            Self::Byzantium => "Byzantium",
            Self::Indigo => "Indigo",
            Self::ShadowPurple => "Shadow Purple",
            Self::PlumPurple => "Plum Purple",
            Self::Plum => "Plum",
            Self::PurplePassion => "Purple Passion",
            Self::GrapeJuice => "Grape Juice",
            Self::PlumPerfect => "Plum Perfect",
            Self::DeepPurple => "Deep Purple",
            Self::WoodViolet => "Wood Violet",
            Self::SparklingGrape => "Sparkling Grape",
            Self::SunsetPurple => "Sunset Purple",
            Self::SpicedPlum => "Spiced Plum",
            Self::BrightViolet => "Bright Violet",
            Self::Majesty => "Majesty",
            Self::GrapeRoyale => "Grape Royale",
            Self::PurplePennant => "Purple Pennant",
            Self::BlackberryCordial => "Blackberry Cordial",
            Self::MeadowViolet => "Meadow Violet",
            Self::ImperialPurple => "Imperial Purple",
            Self::AmaranthPurple => "Amaranth Purple",
            Self::PurpleMagic => "Purple Magic",
            Self::Pansy => "Pansy",
            Self::Mysterioso => "Mysterioso",
            Self::SweetGrape => "Sweet Grape",
            Self::PurpleReign => "Purple Reign",
            Self::Loganberry => "Loganberry",
            Self::Acai => "Acai",
            Self::Petunia => "Petunia",
            Self::TillandsiaPurple => "Tillandsia Purple",
            Self::CrownJewel => "Crown Jewel",
            Self::RoyalPurple => "Royal Purple",
            Self::Nightshade => "Nightshade",
            Self::DeepWell => "Deep Well",
            Self::NavyCosmos => "Navy Cosmos",
            Self::PurplePlumeria => "Purple Plumeria",
            Self::GothicGrape => "Gothic Grape",
            Self::MulberryPurple => "Mulberry Purple",
            Self::PurpleVelvet => "Purple Velvet",
            Self::Grape => "Grape",
            Self::GentianViolet => "Gentian Violet",
            Self::ParachutePurple => "Parachute Purple",
            Self::Heliotrope => "Heliotrope",
            Self::PrismViolet => "Prism Violet",
            Self::VioletIndigo => "Violet Indigo",
            Self::PlumKitten => "Plum Kitten",
            Self::Eclipse => "Eclipse",
            Self::EveningBlue => "Evening Blue",
            Self::AstralAura => "Astral Aura",
            Self::MaritimeBlue => "Maritime Blue",
            Self::NavyBlue => "Navy Blue",
            Self::BlueRibbon => "Blue Ribbon",
            Self::DeepWisteria => "Deep Wisteria",
            Self::DeepBlue => "Deep Blue",
            Self::Liberty => "Liberty",
            Self::MazarineBlue => "Mazarine Blue",
            Self::Pavement => "Pavement",
            Self::Magnet => "Magnet",
            Self::Obsidian => "Obsidian",
            Self::Shale => "Shale",
            Self::Rabbit => "Rabbit",
            Self::DarkShadow => "Dark Shadow",
            Self::ForgedIron => "Forged Iron",
            Self::NineIron => "Nine Iron",
            Self::BlackBean => "Black Bean",
            Self::IronGate => "Iron Gate",
            Self::BlackBeauty => "Black Beauty",
            Self::Graystone => "Graystone",
            Self::BlackenedPearl => "Blackened Pearl",
            Self::Periscope => "Periscope",
            Self::NightshadowBlue => "Nightshadow Blue",
            Self::Peacoat => "Peacoat",
            Self::BlackIris => "Black Iris",
            Self::SkyCaptain => "Sky Captain",
            Self::NavyBlazer => "Navy Blazer",
            Self::NightSky => "Night Sky",
            Self::PatriotBlue => "Patriot Blue",
            Self::CrownBlue => "Crown Blue",
            Self::Graphite => "Graphite",
            Self::BlueIndigo => "Blue Indigo",
            Self::VintageIndigo => "Vintage Indigo",
            Self::OdysseyGray => "Odyssey Gray",
            Self::MedievalBlue => "Medieval Blue",
            Self::DeepCobalt => "Deep Cobalt",
            Self::SkipperBlue => "Skipper Blue",
            Self::TwilightBlue => "Twilight Blue",
            Self::Blueprint => "Blueprint",
            Self::BlueDepths => "Blue Depths",
            Self::OrientBlue => "Orient Blue",
            Self::DeepUltramarine => "Deep Ultramarine",
            Self::ClematisBlue => "Clematis Blue",
            Self::SurfTheWeb => "Surf The Web",
            Self::SodaliteBlue => "Sodalite Blue",
            Self::RoyalBlue => "Royal Blue",
            Self::MonacoBlue => "Monaco Blue",
            Self::BlackOnyx => "Black Onyx",
            Self::TapShoe => "Tap Shoe",
            Self::StretchLimo => "Stretch Limo",
            Self::Caviar => "Caviar",
            Self::Anthracite => "Anthracite",
            Self::Meteorite => "Meteorite",
            Self::OuterSpace => "Outer Space",
            Self::TotalEclipse => "Total Eclipse",
            Self::Salute => "Salute",
            Self::Carbon => "Carbon",
            Self::DarkNavy => "Dark Navy",
            Self::OmbreBlue => "Ombre Blue",
            Self::BlueGraphite => "Blue Graphite",
            Self::IndiaInk => "India Ink",
            Self::DarkSapphire => "Dark Sapphire",
            Self::Blueberry => "Blueberry",
            Self::ParisianNight => "Parisian Night",
            Self::BlueNights => "Blue Nights",
            Self::DressBlues => "Dress Blues",
            Self::MoodIndigo => "Mood Indigo",
            Self::EnsignBlue => "Ensign Blue",
            Self::EstateBlue => "Estate Blue",
            Self::InsigniaBlue => "Insignia Blue",
            Self::NavyPeony => "Navy Peony",
            Self::TrueNavy => "True Navy",
            Self::SargassoSea => "Sargasso Sea",
            Self::Poseidon => "Poseidon",
            Self::SailorBlue => "Sailor Blue",
            Self::DarkBlue => "Dark Blue",
            Self::BrightCobalt => "Bright Cobalt",
            Self::GibraltarSea => "Gibraltar Sea",
            Self::Delft => "Delft",
            Self::Limoges => "Limoges",
            Self::LapisBlue => "Lapis Blue",
            Self::BaleineBlue => "Baleine Blue",
            Self::SnorkelBlue => "Snorkel Blue",
            Self::NauticalBlue => "Nautical Blue",
            Self::ClassicBlue => "Classic Blue",
            Self::TurkishSea => "Turkish Sea",
            Self::GalaxyBlue => "Galaxy Blue",
            Self::OlympianBlue => "Olympian Blue",
            Self::TrueBlue => "True Blue",
            Self::Ebony => "Ebony",
            Self::MidnightNavy => "Midnight Navy",
            Self::DarkDenim => "Dark Denim",
            Self::BlueOpal => "Blue Opal",
            Self::BlueWingTeal => "Blue Wing Teal",
            Self::MoonlitOcean => "Moonlit Ocean",
            Self::MajolicaBlue => "Majolica Blue",
            Self::DeepDive => "Deep Dive",
            Self::Midnight => "Midnight",
            Self::PrincessBlue => "Princess Blue",
            Self::Skydiver => "Skydiver",
            Self::MoonlessNight => "Moonless Night",
            Self::Phantom => "Phantom",
            Self::Turbulence => "Turbulence",
            Self::Vulcan => "Vulcan",
            Self::DarkSlate => "Dark Slate",
            Self::IndianTeal => "Indian Teal",
            Self::OrionBlue => "Orion Blue",
            Self::InkBlue => "Ink Blue",
            Self::MoroccanBlue => "Moroccan Blue",
            Self::ImperialBlue => "Imperial Blue",
            Self::PirateBlack => "Pirate Black",
            Self::Stargazer => "Stargazer",
            Self::MallardBlue => "Mallard Blue",
            Self::LegionBlue => "Legion Blue",
            Self::ReflectingPond => "Reflecting Pond",
            Self::Corsair => "Corsair",
            Self::LyonsBlue => "Lyons Blue",
            Self::Seaport => "Seaport",
            Self::Mediterranea => "Mediterranea",
            Self::ShadedSpruce => "Shaded Spruce",
            Self::BlueCoral => "Blue Coral",
            Self::OceanDepths => "Ocean Depths",
            Self::CrystalTeal => "Crystal Teal",
            Self::DeepLagoon => "Deep Lagoon",
            Self::AtlanticDeep => "Atlantic Deep",
            Self::MallardGreen => "Mallard Green",
            Self::Balsam => "Balsam",
            Self::Dragonfly => "Dragonfly",
            Self::GreenGables => "Green Gables",
            Self::DeepTeal => "Deep Teal",
            Self::Pacific => "Pacific",
            Self::TealGreen => "Teal Green",
            Self::UrbanChic => "Urban Chic",
            Self::SeaMoss => "Sea Moss",
            Self::DarkestSpruce => "Darkest Spruce",
            Self::Storm => "Storm",
            Self::BotanicalGarden => "Botanical Garden",
            Self::Everglade => "Everglade",
            Self::ForestBiome => "Forest Biome",
            Self::RainForest => "Rain Forest",
            Self::PonderosaPine => "Ponderosa Pine",
            Self::Scarab => "Scarab",
            Self::PineGrove => "Pine Grove",
            Self::BistroGreen => "Bistro Green",
            Self::TrekkingGreen => "Trekking Green",
            Self::Jasper => "Jasper",
            Self::JuneBug => "June Bug",
            Self::Evergreen => "Evergreen",
            Self::HunterGreen => "Hunter Green",
            Self::DarkGreen => "Dark Green",
            Self::Cilantro => "Cilantro",
            Self::JetSet => "Jet Set",
            Self::JungleGreen => "Jungle Green",
            Self::Sycamore => "Sycamore",
            Self::MountainView => "Mountain View",
            Self::Pineneedle => "Pineneedle",
            Self::VerdantGreen => "Verdant Green",
            Self::Eden => "Eden",
            Self::DeepForest => "Deep Forest",
            Self::GreenerPastures => "Greener Pastures",
        }
    }

    /// The canonical substrate suffix used by [`Self::as_str`].
    ///
    /// FHI colors are material-independent names; the canonical
    /// string resolves the cotton `TCX` guide, which is the dataset
    /// these sRGB approximations are based on.
    pub const fn suffix(&self) -> PantoneFhiSuffix {
        PantoneFhiSuffix::Cotton
    }

    /// The canonical `Pantone <code> TCX <name>` string.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Egret => "Pantone 11-0103 TCX Egret",
            Self::VanillaIce => "Pantone 11-0104 TCX Vanilla Ice",
            Self::AntiqueWhite => "Pantone 11-0105 TCX Antique White",
            Self::SweetCream => "Pantone 11-0106 TCX Sweet Cream",
            Self::Papyrus => "Pantone 11-0107 TCX Papyrus",
            Self::Buttercream => "Pantone 11-0110 TCX Buttercream",
            Self::GlassGreen => "Pantone 11-0205 TCX Glass Green",
            Self::WaterLily => "Pantone 11-0304 TCX Water Lily",
            Self::SylvanGreen => "Pantone 11-0410 TCX Sylvan Green",
            Self::WinterWhite => "Pantone 11-0507 TCX Winter White",
            Self::Afterglow => "Pantone 11-0510 TCX Afterglow",
            Self::LemonIcing => "Pantone 11-0515 TCX Lemon Icing",
            Self::BrightWhite => "Pantone 11-0601 TCX Bright White",
            Self::SnowWhite => "Pantone 11-0602 TCX Snow White",
            Self::PastelParchment => "Pantone 11-0603 TCX Pastel Parchment",
            Self::Gardenia => "Pantone 11-0604 TCX Gardenia",
            Self::JetStream => "Pantone 11-0605 TCX Jet Stream",
            Self::Pristine => "Pantone 11-0606 TCX Pristine",
            Self::SugarSwizzle => "Pantone 11-0607 TCX Sugar Swizzle",
            Self::CoconutMilk => "Pantone 11-0608 TCX Coconut Milk",
            Self::EtherealGreen => "Pantone 11-0609 TCX Ethereal Green",
            Self::PearSorbet => "Pantone 11-0615 TCX Pear Sorbet",
            Self::PastelYellow => "Pantone 11-0616 TCX Pastel Yellow",
            Self::TransparentYellow => "Pantone 11-0617 TCX Transparent Yellow",
            Self::WaxYellow => "Pantone 11-0618 TCX Wax Yellow",
            Self::Flan => "Pantone 11-0619 TCX Flan",
            Self::ElfinYellow => "Pantone 11-0620 TCX Elfin Yellow",
            Self::YellowIris => "Pantone 11-0622 TCX Yellow Iris",
            Self::YellowPear => "Pantone 11-0623 TCX Yellow Pear",
            Self::WhisperWhite => "Pantone 11-0701 TCX Whisper White",
            Self::TenderYellow => "Pantone 11-0710 TCX Tender Yellow",
            Self::Ecru => "Pantone 11-0809 TCX Ecru",
            Self::PearledIvory => "Pantone 11-0907 TCX Pearled Ivory",
            Self::WhiteAlyssum => "Pantone 11-1001 TCX White Alyssum",
            Self::BridalBlush => "Pantone 11-1005 TCX Bridal Blush",
            Self::SeaSalt => "Pantone 11-1302 TCX Sea Salt",
            Self::AngelWing => "Pantone 11-1305 TCX Angel Wing",
            Self::CreamPink => "Pantone 11-1306 TCX Cream Pink",
            Self::PowderPuff => "Pantone 11-1404 TCX Powder Puff",
            Self::Rosewater => "Pantone 11-1408 TCX Rosewater",
            Self::PetalPink => "Pantone 11-2309 TCX Petal Pink",
            Self::Delicacy => "Pantone 11-2409 TCX Delicacy",
            Self::ShrinkingViolet => "Pantone 11-2511 TCX Shrinking Violet",
            Self::BrilliantWhite => "Pantone 11-4001 TCX Brilliant White",
            Self::CloudDancer => "Pantone 11-4201 TCX Cloud Dancer",
            Self::StarWhite => "Pantone 11-4202 TCX Star White",
            Self::Marshmallow => "Pantone 11-4300 TCX Marshmallow",
            Self::LilyWhite => "Pantone 11-4301 TCX Lily White",
            Self::CannoliCream => "Pantone 11-4302 TCX Cannoli Cream",
            Self::MysticBlue => "Pantone 11-4303 TCX Mystic Blue",
            Self::BitOfBlue => "Pantone 11-4601 TCX Bit Of Blue",
            Self::BillowingSail => "Pantone 11-4604 TCX Billowing Sail",
            Self::BlancDeBlanc => "Pantone 11-4800 TCX Blanc De Blanc",
            Self::Tofu => "Pantone 11-4801 TCX Tofu",
            Self::SummerShower => "Pantone 11-4802 TCX Summer Shower",
            Self::Ice => "Pantone 11-4803 TCX Ice",
            Self::LightestSky => "Pantone 11-4804 TCX Lightest Sky",
            Self::HintOfMint => "Pantone 11-4805 TCX Hint Of Mint",
            Self::WhiteSwan => "Pantone 12-0000 TCX White Swan",
            Self::WhiteAsparagus => "Pantone 12-0104 TCX White Asparagus",
            Self::BoneWhite => "Pantone 12-0105 TCX Bone White",
            Self::MeadowMist => "Pantone 12-0106 TCX Meadow Mist",
            Self::CanaryGreen => "Pantone 12-0108 TCX Canary Green",
            Self::Ambrosia => "Pantone 12-0109 TCX Ambrosia",
            Self::PistachioShell => "Pantone 12-0110 TCX Pistachio Shell",
            Self::PatinaGreen => "Pantone 12-0225 TCX Patina Green",
            Self::WhitecapGray => "Pantone 12-0304 TCX Whitecap Gray",
            Self::AsparagusGreen => "Pantone 12-0311 TCX Asparagus Green",
            Self::LimeCream => "Pantone 12-0312 TCX Lime Cream",
            Self::SeafoamGreen => "Pantone 12-0313 TCX Seafoam Green",
            Self::WhiteJade => "Pantone 12-0315 TCX White Jade",
            Self::Gleam => "Pantone 12-0317 TCX Gleam",
            Self::Butterfly => "Pantone 12-0322 TCX Butterfly",
            Self::LightGray => "Pantone 12-0404 TCX Light Gray",
            Self::Hay => "Pantone 12-0418 TCX Hay",
            Self::MellowGreen => "Pantone 12-0426 TCX Mellow Green",
            Self::DaiquiriGreen => "Pantone 12-0435 TCX Daiquiri Green",
            Self::PaleLimeYellow => "Pantone 12-0520 TCX Pale Lime Yellow",
            Self::YoungWheat => "Pantone 12-0521 TCX Young Wheat",
            Self::Citron => "Pantone 12-0524 TCX Citron",
            Self::LuminaryGreen => "Pantone 12-0525 TCX Luminary Green",
            Self::Charlock => "Pantone 12-0530 TCX Charlock",
            Self::Eggnog => "Pantone 12-0601 TCX Eggnog",
            Self::Angora => "Pantone 12-0605 TCX Angora",
            Self::GreenEssence => "Pantone 12-0607 TCX Green Essence",
            Self::DustyYellow => "Pantone 12-0619 TCX Dusty Yellow",
            Self::LemonGrass => "Pantone 12-0626 TCX Lemon Grass",
            Self::CanaryYellow => "Pantone 12-0633 TCX Canary Yellow",
            Self::Aurora => "Pantone 12-0642 TCX Aurora",
            Self::BlazingYellow => "Pantone 12-0643 TCX Blazing Yellow",
            Self::Celandine => "Pantone 12-0646 TCX Celandine",
            Self::Seedpearl => "Pantone 12-0703 TCX Seedpearl",
            Self::WhiteSmoke => "Pantone 12-0704 TCX White Smoke",
            Self::Macadamia => "Pantone 12-0709 TCX Macadamia",
            Self::Navajo => "Pantone 12-0710 TCX Navajo",
            Self::LemonMeringue => "Pantone 12-0711 TCX Lemon Meringue",
            Self::Vanilla => "Pantone 12-0712 TCX Vanilla",
            Self::AlmondOil => "Pantone 12-0713 TCX Almond Oil",
            Self::Cornhusk => "Pantone 12-0714 TCX Cornhusk",
            Self::DoubleCream => "Pantone 12-0715 TCX Double Cream",
            Self::AniseFlower => "Pantone 12-0717 TCX Anise Flower",
            Self::PineappleSlice => "Pantone 12-0718 TCX Pineapple Slice",
            Self::MellowYellow => "Pantone 12-0720 TCX Mellow Yellow",
            Self::Lemonade => "Pantone 12-0721 TCX Lemonade",
            Self::FrenchVanilla => "Pantone 12-0722 TCX French Vanilla",
            Self::Sunshine => "Pantone 12-0727 TCX Sunshine",
            Self::Sundress => "Pantone 12-0729 TCX Sundress",
            Self::LemonDrop => "Pantone 12-0736 TCX Lemon Drop",
            Self::Goldfinch => "Pantone 12-0737 TCX Goldfinch",
            Self::YellowCream => "Pantone 12-0738 TCX Yellow Cream",
            Self::Limelight => "Pantone 12-0740 TCX Limelight",
            Self::SunnyLime => "Pantone 12-0741 TCX Sunny Lime",
            Self::LemonVerbena => "Pantone 12-0742 TCX Lemon Verbena",
            Self::Buttercup => "Pantone 12-0752 TCX Buttercup",
            Self::Yarrow => "Pantone 12-0758 TCX Yarrow",
            Self::CloudCream => "Pantone 12-0804 TCX Cloud Cream",
            Self::Rutabaga => "Pantone 12-0806 TCX Rutabaga",
            Self::SunKiss => "Pantone 12-0807 TCX Sun Kiss",
            Self::Dawn => "Pantone 12-0811 TCX Dawn",
            Self::AlabasterGleam => "Pantone 12-0812 TCX Alabaster Gleam",
            Self::AutumnBlonde => "Pantone 12-0813 TCX Autumn Blonde",
            Self::VanillaCustard => "Pantone 12-0815 TCX Vanilla Custard",
            Self::ApricotGelato => "Pantone 12-0817 TCX Apricot Gelato",
            Self::GoldenFleece => "Pantone 12-0822 TCX Golden Fleece",
            Self::PaleBanana => "Pantone 12-0824 TCX Pale Banana",
            Self::Popcorn => "Pantone 12-0825 TCX Popcorn",
            Self::GoldenHaze => "Pantone 12-0826 TCX Golden Haze",
            Self::LambsWool => "Pantone 12-0910 TCX Lambs Wool",
            Self::Nude => "Pantone 12-0911 TCX Nude",
            Self::TenderPeach => "Pantone 12-0912 TCX Tender Peach",
            Self::Alesan => "Pantone 12-0913 TCX Alesan",
            Self::PalePeach => "Pantone 12-0915 TCX Pale Peach",
            Self::BleachedApricot => "Pantone 12-0917 TCX Bleached Apricot",
            Self::GoldenStraw => "Pantone 12-0921 TCX Golden Straw",
            Self::NovellePeach => "Pantone 12-1005 TCX Novelle Peach",
            Self::MotherOfPearl => "Pantone 12-1006 TCX Mother Of Pearl",
            Self::PastelRoseTan => "Pantone 12-1007 TCX Pastel Rose Tan",
            Self::Linen => "Pantone 12-1008 TCX Linen",
            Self::VanillaCream => "Pantone 12-1009 TCX Vanilla Cream",
            Self::ScallopShell => "Pantone 12-1010 TCX Scallop Shell",
            Self::PeachPuree => "Pantone 12-1011 TCX Peach Puree",
            Self::SheerPink => "Pantone 12-1106 TCX Sheer Pink",
            Self::PeachDust => "Pantone 12-1107 TCX Peach Dust",
            Self::Dew => "Pantone 12-1108 TCX Dew",
            Self::CremeDePeche => "Pantone 12-1110 TCX Creme De Peche",
            Self::SilverPeony => "Pantone 12-1206 TCX Silver Peony",
            Self::PearlBlush => "Pantone 12-1207 TCX Pearl Blush",
            Self::SoftPink => "Pantone 12-1209 TCX Soft Pink",
            Self::VeiledRose => "Pantone 12-1212 TCX Veiled Rose",
            Self::Pearl => "Pantone 12-1304 TCX Pearl",
            Self::HeavenlyPink => "Pantone 12-1305 TCX Heavenly Pink",
            Self::BlushingBride => "Pantone 12-1310 TCX Blushing Bride",
            Self::Tapioca => "Pantone 12-1403 TCX Tapioca",
            Self::PinkTint => "Pantone 12-1404 TCX Pink Tint",
            Self::CrystalPink => "Pantone 12-1605 TCX Crystal Pink",
            Self::PinkDogwood => "Pantone 12-1706 TCX Pink Dogwood",
            Self::CrystalRose => "Pantone 12-1708 TCX Crystal Rose",
            Self::MarysRose => "Pantone 12-1813 TCX Marys Rose",
            Self::MauveMorn => "Pantone 12-2102 TCX Mauve Morn",
            Self::AlmostMauve => "Pantone 12-2103 TCX Almost Mauve",
            Self::Morganite => "Pantone 12-2901 TCX Morganite",
            Self::MauveChalk => "Pantone 12-2902 TCX Mauve Chalk",
            Self::LightLilac => "Pantone 12-2903 TCX Light Lilac",
            Self::PrimrosePink => "Pantone 12-2904 TCX Primrose Pink",
            Self::CradlePink => "Pantone 12-2905 TCX Cradle Pink",
            Self::BarelyPink => "Pantone 12-2906 TCX Barely Pink",
            Self::RoseWater => "Pantone 12-2907 TCX Rose Water",
            Self::AlmondMilk => "Pantone 12-4301 TCX Almond Milk",
            Self::VaporousGray => "Pantone 12-4302 TCX Vaporous Gray",
            Self::Bluewash => "Pantone 12-4304 TCX Bluewash",
            Self::SpaBlue => "Pantone 12-4305 TCX Spa Blue",
            Self::BarelyBlue => "Pantone 12-4306 TCX Barely Blue",
            Self::Skylight => "Pantone 12-4604 TCX Skylight",
            Self::PastelBlue => "Pantone 12-4607 TCX Pastel Blue",
            Self::Clearwater => "Pantone 12-4608 TCX Clearwater",
            Self::StarlightBlue => "Pantone 12-4609 TCX Starlight Blue",
            Self::WhisperingBlue => "Pantone 12-4610 TCX Whispering Blue",
            Self::BlueBlush => "Pantone 12-4705 TCX Blue Blush",
            Self::WanBlue => "Pantone 12-4805 TCX Wan Blue",
            Self::ChalkBlue => "Pantone 12-4806 TCX Chalk Blue",
            Self::Icicle => "Pantone 12-5201 TCX Icicle",
            Self::Turtledove => "Pantone 12-5202 TCX Turtledove",
            Self::Murmur => "Pantone 12-5203 TCX Murmur",
            Self::MorningMist => "Pantone 12-5204 TCX Morning Mist",
            Self::BlueGlass => "Pantone 12-5206 TCX Blue Glass",
            Self::SoothingSea => "Pantone 12-5209 TCX Soothing Sea",
            Self::SproutGreen => "Pantone 12-5303 TCX Sprout Green",
            Self::BlueFlower => "Pantone 12-5403 TCX Blue Flower",
            Self::WhisperGreen => "Pantone 12-5404 TCX Whisper Green",
            Self::OpalBlue => "Pantone 12-5406 TCX Opal Blue",
            Self::AquaGlass => "Pantone 12-5407 TCX Aqua Glass",
            Self::MoonlightJade => "Pantone 12-5408 TCX Moonlight Jade",
            Self::FairAqua => "Pantone 12-5409 TCX Fair Aqua",
            Self::BleachedAqua => "Pantone 12-5410 TCX Bleached Aqua",
            Self::ClearlyAqua => "Pantone 12-5504 TCX Clearly Aqua",
            Self::Glacier => "Pantone 12-5505 TCX Glacier",
            Self::DustyAqua => "Pantone 12-5506 TCX Dusty Aqua",
            Self::Bay => "Pantone 12-5507 TCX Bay",
            Self::HushedGreen => "Pantone 12-5508 TCX Hushed Green",
            Self::ZephyrBlue => "Pantone 12-5603 TCX Zephyr Blue",
            Self::Honeydew => "Pantone 12-5808 TCX Honeydew",
            Self::SilverGreen => "Pantone 12-6204 TCX Silver Green",
            Self::MilkyGreen => "Pantone 12-6205 TCX Milky Green",
            Self::FairestJade => "Pantone 12-6206 TCX Fairest Jade",
            Self::Frost => "Pantone 12-6207 TCX Frost",
            Self::PhantomGreen => "Pantone 12-6208 TCX Phantom Green",
            Self::Moonbeam => "Pantone 13-0000 TCX Moonbeam",
            Self::WhiteSand => "Pantone 13-0002 TCX White Sand",
            Self::Dewkist => "Pantone 13-0107 TCX Dewkist",
            Self::Seacrest => "Pantone 13-0111 TCX Seacrest",
            Self::PastelGreen => "Pantone 13-0116 TCX Pastel Green",
            Self::GreenAsh => "Pantone 13-0117 TCX Green Ash",
            Self::FogGreen => "Pantone 13-0210 TCX Fog Green",
            Self::TenderGreens => "Pantone 13-0212 TCX Tender Greens",
            Self::Reed => "Pantone 13-0215 TCX Reed",
            Self::ParadiseGreen => "Pantone 13-0220 TCX Paradise Green",
            Self::PistachioGreen => "Pantone 13-0221 TCX Pistachio Green",
            Self::LilyGreen => "Pantone 13-0317 TCX Lily Green",
            Self::ShadowLime => "Pantone 13-0319 TCX Shadow Lime",
            Self::LettuceGreen => "Pantone 13-0324 TCX Lettuce Green",
            Self::SapGreen => "Pantone 13-0331 TCX Sap Green",
            Self::LimaBean => "Pantone 13-0333 TCX Lima Bean",
            Self::Oatmeal => "Pantone 13-0401 TCX Oatmeal",
            Self::GrayMorn => "Pantone 13-0403 TCX Gray Morn",
            Self::GreenGlow => "Pantone 13-0442 TCX Green Glow",
            Self::LimePopsicle => "Pantone 13-0443 TCX Lime Popsicle",
            Self::FrozenDew => "Pantone 13-0513 TCX Frozen Dew",
            Self::PaleGreen => "Pantone 13-0522 TCX Pale Green",
            Self::LimeSherbet => "Pantone 13-0530 TCX Lime Sherbet",
            Self::CeleryGreen => "Pantone 13-0532 TCX Celery Green",
            Self::SharpGreen => "Pantone 13-0535 TCX Sharp Green",
            Self::WildLime => "Pantone 13-0540 TCX Wild Lime",
            Self::LimePunch => "Pantone 13-0550 TCX Lime Punch",
            Self::Fog => "Pantone 13-0607 TCX Fog",
            Self::AloeWash => "Pantone 13-0608 TCX Aloe Wash",
            Self::Moth => "Pantone 13-0611 TCX Moth",
            Self::ChinoGreen => "Pantone 13-0613 TCX Chino Green",
            Self::GardenGlade => "Pantone 13-0614 TCX Garden Glade",
            Self::GoldenMist => "Pantone 13-0624 TCX Golden Mist",
            Self::Endive => "Pantone 13-0632 TCX Endive",
            Self::Chardonnay => "Pantone 13-0633 TCX Chardonnay",
            Self::Acacia => "Pantone 13-0640 TCX Acacia",
            Self::GoldenKiwi => "Pantone 13-0644 TCX Golden Kiwi",
            Self::Limeade => "Pantone 13-0645 TCX Limeade",
            Self::Meadowlark => "Pantone 13-0646 TCX Meadowlark",
            Self::GreenSheen => "Pantone 13-0648 TCX Green Sheen",
            Self::SulphurSpring => "Pantone 13-0650 TCX Sulphur Spring",
            Self::EveningPrimrose => "Pantone 13-0651 TCX Evening Primrose",
            Self::Putty => "Pantone 13-0711 TCX Putty",
            Self::SeaMist => "Pantone 13-0715 TCX Sea Mist",
            Self::Custard => "Pantone 13-0720 TCX Custard",
            Self::Raffia => "Pantone 13-0725 TCX Raffia",
            Self::CreamGold => "Pantone 13-0739 TCX Cream Gold",
            Self::Maize => "Pantone 13-0746 TCX Maize",
            Self::Lemon => "Pantone 13-0752 TCX Lemon",
            Self::PrimroseYellow => "Pantone 13-0755 TCX Primrose Yellow",
            Self::LemonZest => "Pantone 13-0756 TCX Lemon Zest",
            Self::Dandelion => "Pantone 13-0758 TCX Dandelion",
            Self::SolarPower => "Pantone 13-0759 TCX Solar Power",
            Self::SummerMelon => "Pantone 13-0814 TCX Summer Melon",
            Self::BananaCrepe => "Pantone 13-0815 TCX Banana Crepe",
            Self::Sunlight => "Pantone 13-0822 TCX Sunlight",
            Self::Snapdragon => "Pantone 13-0840 TCX Snapdragon",
            Self::HabaneroGold => "Pantone 13-0849 TCX Habanero Gold",
            Self::AspenGold => "Pantone 13-0850 TCX Aspen Gold",
            Self::MinionYellow => "Pantone 13-0851 TCX Minion Yellow",
            Self::VibrantYellow => "Pantone 13-0858 TCX Vibrant Yellow",
            Self::LemonChrome => "Pantone 13-0859 TCX Lemon Chrome",
            Self::Birch => "Pantone 13-0905 TCX Birch",
            Self::Sandshell => "Pantone 13-0907 TCX Sandshell",
            Self::Parchment => "Pantone 13-0908 TCX Parchment",
            Self::ReedYellow => "Pantone 13-0915 TCX Reed Yellow",
            Self::Chamomile => "Pantone 13-0916 TCX Chamomile",
            Self::ItalianStraw => "Pantone 13-0917 TCX Italian Straw",
            Self::Soybean => "Pantone 13-0919 TCX Soybean",
            Self::Straw => "Pantone 13-0922 TCX Straw",
            Self::Cornsilk => "Pantone 13-0932 TCX Cornsilk",
            Self::Flax => "Pantone 13-0935 TCX Flax",
            Self::GoldenCream => "Pantone 13-0939 TCX Golden Cream",
            Self::SunsetGold => "Pantone 13-0940 TCX Sunset Gold",
            Self::BananaCream => "Pantone 13-0941 TCX Banana Cream",
            Self::AmberYellow => "Pantone 13-0942 TCX Amber Yellow",
            Self::PaleMarigold => "Pantone 13-0945 TCX Pale Marigold",
            Self::JurassicGold => "Pantone 13-0946 TCX Jurassic Gold",
            Self::Banana => "Pantone 13-0947 TCX Banana",
            Self::CremeBrulee => "Pantone 13-1006 TCX Creme Brulee",
            Self::OysterWhite => "Pantone 13-1007 TCX Oyster White",
            Self::BleachedSand => "Pantone 13-1008 TCX Bleached Sand",
            Self::Biscotti => "Pantone 13-1009 TCX Biscotti",
            Self::GraySand => "Pantone 13-1010 TCX Gray Sand",
            Self::IvoryCream => "Pantone 13-1011 TCX Ivory Cream",
            Self::FrostedAlmond => "Pantone 13-1012 TCX Frosted Almond",
            Self::Appleblossom => "Pantone 13-1013 TCX Appleblossom",
            Self::MellowBuff => "Pantone 13-1014 TCX Mellow Buff",
            Self::HoneyPeach => "Pantone 13-1015 TCX Honey Peach",
            Self::Wheat => "Pantone 13-1016 TCX Wheat",
            Self::AlmondCream => "Pantone 13-1017 TCX Almond Cream",
            Self::DesertDust => "Pantone 13-1018 TCX Desert Dust",
            Self::CreamBlush => "Pantone 13-1019 TCX Cream Blush",
            Self::ApricotIce => "Pantone 13-1020 TCX Apricot Ice",
            Self::PrairieSunset => "Pantone 13-1021 TCX Prairie Sunset",
            Self::CaramelCream => "Pantone 13-1022 TCX Caramel Cream",
            Self::PeachFuzz => "Pantone 13-1023 TCX Peach Fuzz",
            Self::Buff => "Pantone 13-1024 TCX Buff",
            Self::Impala => "Pantone 13-1025 TCX Impala",
            Self::Creampuff => "Pantone 13-1026 TCX Creampuff",
            Self::ApricotCream => "Pantone 13-1027 TCX Apricot Cream",
            Self::Sunburst => "Pantone 13-1030 TCX Sunburst",
            Self::ApricotSherbet => "Pantone 13-1031 TCX Apricot Sherbet",
            Self::BrownRice => "Pantone 13-1105 TCX Brown Rice",
            Self::SandDollar => "Pantone 13-1106 TCX Sand Dollar",
            Self::WhisperPink => "Pantone 13-1107 TCX Whisper Pink",
            Self::CreamTan => "Pantone 13-1108 TCX Cream Tan",
            Self::Bisque => "Pantone 13-1109 TCX Bisque",
            Self::Bellini => "Pantone 13-1114 TCX Bellini",
            Self::PeachQuartz => "Pantone 13-1125 TCX Peach Quartz",
            Self::PeachyKeen => "Pantone 13-1208 TCX Peachy Keen",
            Self::BrazilianSand => "Pantone 13-1308 TCX Brazilian Sand",
            Self::EnglishRose => "Pantone 13-1310 TCX English Rose",
            Self::TropicalPeach => "Pantone 13-1318 TCX Tropical Peach",
            Self::PaleDogwood => "Pantone 13-1404 TCX Pale Dogwood",
            Self::Shell => "Pantone 13-1405 TCX Shell",
            Self::CloudPink => "Pantone 13-1406 TCX Cloud Pink",
            Self::CreolePink => "Pantone 13-1407 TCX Creole Pink",
            Self::ChintzRose => "Pantone 13-1408 TCX Chintz Rose",
            Self::SeashellPink => "Pantone 13-1409 TCX Seashell Pink",
            Self::PeachBlush => "Pantone 13-1504 TCX Peach Blush",
            Self::ImpatiensPink => "Pantone 13-1510 TCX Impatiens Pink",
            Self::PinkSalt => "Pantone 13-1511 TCX Pink Salt",
            Self::GossamerPink => "Pantone 13-1513 TCX Gossamer Pink",
            Self::RoseQuartz => "Pantone 13-1520 TCX Rose Quartz",
            Self::ChalkPink => "Pantone 13-1904 TCX Chalk Pink",
            Self::RoseShadow => "Pantone 13-1906 TCX Rose Shadow",
            Self::Potpourri => "Pantone 13-2004 TCX Potpourri",
            Self::StrawberryCream => "Pantone 13-2005 TCX Strawberry Cream",
            Self::AlmondBlossom => "Pantone 13-2006 TCX Almond Blossom",
            Self::OrchidPink => "Pantone 13-2010 TCX Orchid Pink",
            Self::FairyTale => "Pantone 13-2802 TCX Fairy Tale",
            Self::PaleLilac => "Pantone 13-2803 TCX Pale Lilac",
            Self::ParfaitPink => "Pantone 13-2804 TCX Parfait Pink",
            Self::PinkMist => "Pantone 13-2805 TCX Pink Mist",
            Self::PinkLady => "Pantone 13-2806 TCX Pink Lady",
            Self::Ballerina => "Pantone 13-2807 TCX Ballerina",
            Self::BalletSlipper => "Pantone 13-2808 TCX Ballet Slipper",
            Self::CherryBlossom => "Pantone 13-3207 TCX Cherry Blossom",
            Self::LilacSnow => "Pantone 13-3405 TCX Lilac Snow",
            Self::OrchidIce => "Pantone 13-3406 TCX Orchid Ice",
            Self::CrystalGray => "Pantone 13-3801 TCX Crystal Gray",
            Self::OrchidTint => "Pantone 13-3802 TCX Orchid Tint",
            Self::LilacAsh => "Pantone 13-3803 TCX Lilac Ash",
            Self::GrayLilac => "Pantone 13-3804 TCX Gray Lilac",
            Self::OrchidHush => "Pantone 13-3805 TCX Orchid Hush",
            Self::LavenderFog => "Pantone 13-3820 TCX Lavender Fog",
            Self::HalogenBlue => "Pantone 13-3920 TCX Halogen Blue",
            Self::IllusionBlue => "Pantone 13-4103 TCX Illusion Blue",
            Self::Antarctica => "Pantone 13-4104 TCX Antarctica",
            Self::LilacHint => "Pantone 13-4105 TCX Lilac Hint",
            Self::NimbusCloud => "Pantone 13-4108 TCX Nimbus Cloud",
            Self::ArcticIce => "Pantone 13-4110 TCX Arctic Ice",
            Self::PleinAir => "Pantone 13-4111 TCX Plein Air",
            Self::Omphalodes => "Pantone 13-4200 TCX Omphalodes",
            Self::OysterMushroom => "Pantone 13-4201 TCX Oyster Mushroom",
            Self::DawnBlue => "Pantone 13-4303 TCX Dawn Blue",
            Self::BalladBlue => "Pantone 13-4304 TCX Ballad Blue",
            Self::FoggyDew => "Pantone 13-4305 TCX Foggy Dew",
            Self::BabyBlue => "Pantone 13-4308 TCX Baby Blue",
            Self::SilverBirch => "Pantone 13-4403 TCX Silver Birch",
            Self::IceFlow => "Pantone 13-4404 TCX Ice Flow",
            Self::MistyBlue => "Pantone 13-4405 TCX Misty Blue",
            Self::BlueGlow => "Pantone 13-4409 TCX Blue Glow",
            Self::CrystalBlue => "Pantone 13-4411 TCX Crystal Blue",
            Self::TanagerTurquoise => "Pantone 13-4720 TCX Tanager Turquoise",
            Self::PaleBlue => "Pantone 13-4804 TCX Pale Blue",
            Self::Plume => "Pantone 13-4809 TCX Plume",
            Self::LimpetShell => "Pantone 13-4810 TCX Limpet Shell",
            Self::BlueLight => "Pantone 13-4909 TCX Blue Light",
            Self::BlueTint => "Pantone 13-4910 TCX Blue Tint",
            Self::RainyDay => "Pantone 13-5304 TCX Rainy Day",
            Self::PaleAqua => "Pantone 13-5305 TCX Pale Aqua",
            Self::IcyMorn => "Pantone 13-5306 TCX Icy Morn",
            Self::PastelTurquoise => "Pantone 13-5309 TCX Pastel Turquoise",
            Self::ArubaBlue => "Pantone 13-5313 TCX Aruba Blue",
            Self::Yucca => "Pantone 13-5409 TCX Yucca",
            Self::IcedAqua => "Pantone 13-5410 TCX Iced Aqua",
            Self::BeachGlass => "Pantone 13-5412 TCX Beach Glass",
            Self::IceGreen => "Pantone 13-5414 TCX Ice Green",
            Self::Cabbage => "Pantone 13-5714 TCX Cabbage",
            Self::GossamerGreen => "Pantone 13-5907 TCX Gossamer Green",
            Self::BirdsEggGreen => "Pantone 13-5911 TCX Birds Egg Green",
            Self::AlmostAqua => "Pantone 13-6006 TCX Almost Aqua",
            Self::Spray => "Pantone 13-6007 TCX Spray",
            Self::MistyJade => "Pantone 13-6008 TCX Misty Jade",
            Self::BrookGreen => "Pantone 13-6009 TCX Brook Green",
            Self::CeladonTint => "Pantone 13-6105 TCX Celadon Tint",
            Self::GreenTint => "Pantone 13-6106 TCX Green Tint",
            Self::GreenLily => "Pantone 13-6107 TCX Green Lily",
            Self::Celadon => "Pantone 13-6108 TCX Celadon",
            Self::MistGreen => "Pantone 13-6110 TCX Mist Green",
            Self::BokChoy => "Pantone 13-6208 TCX Bok Choy",
            Self::SilverGray => "Pantone 14-0000 TCX Silver Gray",
            Self::PumiceStone => "Pantone 14-0002 TCX Pumice Stone",
            Self::Overcast => "Pantone 14-0105 TCX Overcast",
            Self::CastleWall => "Pantone 14-0108 TCX Castle Wall",
            Self::CeladonGreen => "Pantone 14-0114 TCX Celadon Green",
            Self::FoamGreen => "Pantone 14-0115 TCX Foam Green",
            Self::Margarita => "Pantone 14-0116 TCX Margarita",
            Self::NileGreen => "Pantone 14-0121 TCX Nile Green",
            Self::ArcadianGreen => "Pantone 14-0123 TCX Arcadian Green",
            Self::Greengage => "Pantone 14-0127 TCX Greengage",
            Self::SummerGreen => "Pantone 14-0156 TCX Summer Green",
            Self::TidalFoam => "Pantone 14-0210 TCX Tidal Foam",
            Self::Lint => "Pantone 14-0216 TCX Lint",
            Self::Seedling => "Pantone 14-0217 TCX Seedling",
            Self::Nile => "Pantone 14-0223 TCX Nile",
            Self::OpalineGreen => "Pantone 14-0226 TCX Opaline Green",
            Self::JadeLime => "Pantone 14-0232 TCX Jade Lime",
            Self::BrightLimeGreen => "Pantone 14-0244 TCX Bright Lime Green",
            Self::AcidLime => "Pantone 14-0340 TCX Acid Lime",
            Self::Bog => "Pantone 14-0418 TCX Bog",
            Self::Beechnut => "Pantone 14-0425 TCX Beechnut",
            Self::GreenBanana => "Pantone 14-0434 TCX Green Banana",
            Self::BrightChartreuse => "Pantone 14-0445 TCX Bright Chartreuse",
            Self::TenderShoots => "Pantone 14-0446 TCX Tender Shoots",
            Self::LimeGreen => "Pantone 14-0452 TCX Lime Green",
            Self::GreenHaze => "Pantone 14-0615 TCX Green Haze",
            Self::DriedMoss => "Pantone 14-0626 TCX Dried Moss",
            Self::ShadowGreen => "Pantone 14-0627 TCX Shadow Green",
            Self::MutedLime => "Pantone 14-0636 TCX Muted Lime",
            Self::Celery => "Pantone 14-0647 TCX Celery",
            Self::Cement => "Pantone 14-0708 TCX Cement",
            Self::Hemp => "Pantone 14-0721 TCX Hemp",
            Self::Bamboo => "Pantone 14-0740 TCX Bamboo",
            Self::SuperLemon => "Pantone 14-0754 TCX Super Lemon",
            Self::Sulphur => "Pantone 14-0755 TCX Sulphur",
            Self::EmpireYellow => "Pantone 14-0756 TCX Empire Yellow",
            Self::CyberYellow => "Pantone 14-0760 TCX Cyber Yellow",
            Self::Pampas => "Pantone 14-0826 TCX Pampas",
            Self::DuskyCitron => "Pantone 14-0827 TCX Dusky Citron",
            Self::MistedYellow => "Pantone 14-0837 TCX Misted Yellow",
            Self::YolkYellow => "Pantone 14-0846 TCX Yolk Yellow",
            Self::BuffYellow => "Pantone 14-0847 TCX Buff Yellow",
            Self::Mimosa => "Pantone 14-0848 TCX Mimosa",
            Self::Daffodil => "Pantone 14-0850 TCX Daffodil",
            Self::SamoanSun => "Pantone 14-0851 TCX Samoan Sun",
            Self::Freesia => "Pantone 14-0852 TCX Freesia",
            Self::Parsnip => "Pantone 14-0925 TCX Parsnip",
            Self::Jojoba => "Pantone 14-0935 TCX Jojoba",
            Self::SaharaSun => "Pantone 14-0936 TCX Sahara Sun",
            Self::Beeswax => "Pantone 14-0941 TCX Beeswax",
            Self::GoldenRod => "Pantone 14-0951 TCX Golden Rod",
            Self::SpicyMustard => "Pantone 14-0952 TCX Spicy Mustard",
            Self::Citrus => "Pantone 14-0955 TCX Citrus",
            Self::SpectraYellow => "Pantone 14-0957 TCX Spectra Yellow",
            Self::GildedBeige => "Pantone 14-1012 TCX Gilded Beige",
            Self::Gravel => "Pantone 14-1014 TCX Gravel",
            Self::Cocoon => "Pantone 14-1025 TCX Cocoon",
            Self::Rattan => "Pantone 14-1031 TCX Rattan",
            Self::Ochre => "Pantone 14-1036 TCX Ochre",
            Self::NewWheat => "Pantone 14-1038 TCX New Wheat",
            Self::GoldenApricot => "Pantone 14-1041 TCX Golden Apricot",
            Self::Amber => "Pantone 14-1045 TCX Amber",
            Self::Marigold => "Pantone 14-1050 TCX Marigold",
            Self::WarmApricot => "Pantone 14-1051 TCX Warm Apricot",
            Self::Kumquat => "Pantone 14-1052 TCX Kumquat",
            Self::Saffron => "Pantone 14-1064 TCX Saffron",
            Self::Peyote => "Pantone 14-1106 TCX Peyote",
            Self::OysterGray => "Pantone 14-1107 TCX Oyster Gray",
            Self::WoodAsh => "Pantone 14-1108 TCX Wood Ash",
            Self::Boulder => "Pantone 14-1110 TCX Boulder",
            Self::Pebble => "Pantone 14-1112 TCX Pebble",
            Self::Marzipan => "Pantone 14-1113 TCX Marzipan",
            Self::AlmondBuff => "Pantone 14-1116 TCX Almond Buff",
            Self::Beige => "Pantone 14-1118 TCX Beige",
            Self::WinterWheat => "Pantone 14-1119 TCX Winter Wheat",
            Self::ApricotIllusion => "Pantone 14-1120 TCX Apricot Illusion",
            Self::Sheepskin => "Pantone 14-1122 TCX Sheepskin",
            Self::DesertMist => "Pantone 14-1127 TCX Desert Mist",
            Self::BuffOrange => "Pantone 14-1128 TCX Buff Orange",
            Self::ApricotNectar => "Pantone 14-1133 TCX Apricot Nectar",
            Self::SalmonBuff => "Pantone 14-1135 TCX Salmon Buff",
            Self::Pumpkin => "Pantone 14-1139 TCX Pumpkin",
            Self::Zinnia => "Pantone 14-1159 TCX Zinnia",
            Self::IrishCream => "Pantone 14-1208 TCX Irish Cream",
            Self::SmokeGray => "Pantone 14-1209 TCX Smoke Gray",
            Self::ShiftingSand => "Pantone 14-1210 TCX Shifting Sand",
            Self::Frappe => "Pantone 14-1212 TCX Frappe",
            Self::ToastedAlmond => "Pantone 14-1213 TCX Toasted Almond",
            Self::Amberlight => "Pantone 14-1217 TCX Amberlight",
            Self::PeachParfait => "Pantone 14-1219 TCX Peach Parfait",
            Self::PeachNougat => "Pantone 14-1220 TCX Peach Nougat",
            Self::CoralSands => "Pantone 14-1224 TCX Coral Sands",
            Self::BeachSand => "Pantone 14-1225 TCX Beach Sand",
            Self::Peach => "Pantone 14-1227 TCX Peach",
            Self::PeachNectar => "Pantone 14-1228 TCX Peach Nectar",
            Self::ApricotWash => "Pantone 14-1230 TCX Apricot Wash",
            Self::PeachCobbler => "Pantone 14-1231 TCX Peach Cobbler",
            Self::OrangeChiffon => "Pantone 14-1241 TCX Orange Chiffon",
            Self::Mushroom => "Pantone 14-1305 TCX Mushroom",
            Self::RoseDust => "Pantone 14-1307 TCX Rose Dust",
            Self::PeachWhip => "Pantone 14-1309 TCX Peach Whip",
            Self::CameoRose => "Pantone 14-1310 TCX Cameo Rose",
            Self::EveningSand => "Pantone 14-1311 TCX Evening Sand",
            Self::PaleBlush => "Pantone 14-1312 TCX Pale Blush",
            Self::RoseCloud => "Pantone 14-1313 TCX Rose Cloud",
            Self::SpanishVilla => "Pantone 14-1314 TCX Spanish Villa",
            Self::Hazelnut => "Pantone 14-1315 TCX Hazelnut",
            Self::DustyPink => "Pantone 14-1316 TCX Dusty Pink",
            Self::CoralPink => "Pantone 14-1318 TCX Coral Pink",
            Self::Salmon => "Pantone 14-1323 TCX Salmon",
            Self::PeachBud => "Pantone 14-1324 TCX Peach Bud",
            Self::PeachMelba => "Pantone 14-1418 TCX Peach Melba",
            Self::PeachPearl => "Pantone 14-1419 TCX Peach Pearl",
            Self::ApricotBlush => "Pantone 14-1420 TCX Apricot Blush",
            Self::RoseSmoke => "Pantone 14-1506 TCX Rose Smoke",
            Self::SilverPink => "Pantone 14-1508 TCX Silver Pink",
            Self::PowderPink => "Pantone 14-1511 TCX Powder Pink",
            Self::Blossom => "Pantone 14-1513 TCX Blossom",
            Self::PeachesNCream => "Pantone 14-1521 TCX Peaches N Cream",
            Self::QuartzPink => "Pantone 14-1714 TCX Quartz Pink",
            Self::SepiaRose => "Pantone 14-1803 TCX Sepia Rose",
            Self::Lotus => "Pantone 14-1905 TCX Lotus",
            Self::Peachskin => "Pantone 14-1907 TCX Peachskin",
            Self::CoralBlush => "Pantone 14-1909 TCX Coral Blush",
            Self::CandyPink => "Pantone 14-1911 TCX Candy Pink",
            Self::PinkNectar => "Pantone 14-2305 TCX Pink Nectar",
            Self::CameoPink => "Pantone 14-2307 TCX Cameo Pink",
            Self::PrismPink => "Pantone 14-2311 TCX Prism Pink",
            Self::LilacSachet => "Pantone 14-2710 TCX Lilac Sachet",
            Self::SweetLilac => "Pantone 14-2808 TCX Sweet Lilac",
            Self::FragrantLilac => "Pantone 14-3204 TCX Fragrant Lilac",
            Self::WinsomeOrchid => "Pantone 14-3206 TCX Winsome Orchid",
            Self::PinkLavender => "Pantone 14-3207 TCX Pink Lavender",
            Self::PastelLavender => "Pantone 14-3209 TCX Pastel Lavender",
            Self::OrchidBloom => "Pantone 14-3612 TCX Orchid Bloom",
            Self::OrchidPetal => "Pantone 14-3710 TCX Orchid Petal",
            Self::HushedViolet => "Pantone 14-3803 TCX Hushed Violet",
            Self::Iris => "Pantone 14-3805 TCX Iris",
            Self::PastelLilac => "Pantone 14-3812 TCX Pastel Lilac",
            Self::LilacMarble => "Pantone 14-3903 TCX Lilac Marble",
            Self::EveningHaze => "Pantone 14-3904 TCX Evening Haze",
            Self::LavenderBlue => "Pantone 14-3905 TCX Lavender Blue",
            Self::Raindrops => "Pantone 14-3906 TCX Raindrops",
            Self::Thistle => "Pantone 14-3907 TCX Thistle",
            Self::PurpleHeather => "Pantone 14-3911 TCX Purple Heather",
            Self::ZenBlue => "Pantone 14-3912 TCX Zen Blue",
            Self::XenonBlue => "Pantone 14-3949 TCX Xenon Blue",
            Self::WindChime => "Pantone 14-4002 TCX Wind Chime",
            Self::GlacierGray => "Pantone 14-4102 TCX Glacier Gray",
            Self::GrayViolet => "Pantone 14-4103 TCX Gray Violet",
            Self::MicroChip => "Pantone 14-4105 TCX Micro Chip",
            Self::GrayDawn => "Pantone 14-4106 TCX Gray Dawn",
            Self::QuietGray => "Pantone 14-4107 TCX Quiet Gray",
            Self::Heather => "Pantone 14-4110 TCX Heather",
            Self::Skyway => "Pantone 14-4112 TCX Skyway",
            Self::CashmereBlue => "Pantone 14-4115 TCX Cashmere Blue",
            Self::BlueBell => "Pantone 14-4121 TCX Blue Bell",
            Self::AiryBlue => "Pantone 14-4122 TCX Airy Blue",
            Self::LunarRock => "Pantone 14-4201 TCX Lunar Rock",
            Self::HarborMist => "Pantone 14-4202 TCX Harbor Mist",
            Self::VaporBlue => "Pantone 14-4203 TCX Vapor Blue",
            Self::PearlBlue => "Pantone 14-4206 TCX Pearl Blue",
            Self::CelestialBlue => "Pantone 14-4210 TCX Celestial Blue",
            Self::PowderBlue => "Pantone 14-4214 TCX Powder Blue",
            Self::CloudBlue => "Pantone 14-4306 TCX Cloud Blue",
            Self::WinterSky => "Pantone 14-4307 TCX Winter Sky",
            Self::BlueTopaz => "Pantone 14-4310 TCX Blue Topaz",
            Self::CorydalisBlue => "Pantone 14-4311 TCX Corydalis Blue",
            Self::Aquamarine => "Pantone 14-4313 TCX Aquamarine",
            Self::SeaAngel => "Pantone 14-4315 TCX Sea Angel",
            Self::CoolBlue => "Pantone 14-4317 TCX Cool Blue",
            Self::SkyBlue => "Pantone 14-4318 TCX Sky Blue",
            Self::BalticSea => "Pantone 14-4320 TCX Baltic Sea",
            Self::Moonstruck => "Pantone 14-4500 TCX Moonstruck",
            Self::SilverLining => "Pantone 14-4501 TCX Silver Lining",
            Self::Mercury => "Pantone 14-4502 TCX Mercury",
            Self::Metal => "Pantone 14-4503 TCX Metal",
            Self::SkyGray => "Pantone 14-4504 TCX Sky Gray",
            Self::Smoke => "Pantone 14-4505 TCX Smoke",
            Self::Ether => "Pantone 14-4506 TCX Ether",
            Self::Stratosphere => "Pantone 14-4508 TCX Stratosphere",
            Self::Aquatic => "Pantone 14-4510 TCX Aquatic",
            Self::GulfStream => "Pantone 14-4511 TCX Gulf Stream",
            Self::PorcelainBlue => "Pantone 14-4512 TCX Porcelain Blue",
            Self::PetitFour => "Pantone 14-4516 TCX Petit Four",
            Self::BachelorButton => "Pantone 14-4522 TCX Bachelor Button",
            Self::AntiquaSand => "Pantone 14-4615 TCX Antiqua Sand",
            Self::IslandParadise => "Pantone 14-4620 TCX Island Paradise",
            Self::TibetanStone => "Pantone 14-4710 TCX Tibetan Stone",
            Self::BlueFox => "Pantone 14-4804 TCX Blue Fox",
            Self::SurfSpray => "Pantone 14-4807 TCX Surf Spray",
            Self::EggshellBlue => "Pantone 14-4809 TCX Eggshell Blue",
            Self::CanalBlue => "Pantone 14-4810 TCX Canal Blue",
            Self::AquaSky => "Pantone 14-4811 TCX Aqua Sky",
            Self::AquaSplash => "Pantone 14-4812 TCX Aqua Splash",
            Self::AngelBlue => "Pantone 14-4814 TCX Angel Blue",
            Self::BlueRadiance => "Pantone 14-4816 TCX Blue Radiance",
            Self::HarborGray => "Pantone 14-4908 TCX Harbor Gray",
            Self::Silver => "Pantone 14-5002 TCX Silver",
            Self::Holiday => "Pantone 14-5413 TCX Holiday",
            Self::Bermuda => "Pantone 14-5416 TCX Bermuda",
            Self::Cockatoo => "Pantone 14-5420 TCX Cockatoo",
            Self::SiltGreen => "Pantone 14-5706 TCX Silt Green",
            Self::AquaFoam => "Pantone 14-5707 TCX Aqua Foam",
            Self::OceanWave => "Pantone 14-5711 TCX Ocean Wave",
            Self::Cascade => "Pantone 14-5713 TCX Cascade",
            Self::BeveledGlass => "Pantone 14-5714 TCX Beveled Glass",
            Self::Opal => "Pantone 14-5718 TCX Opal",
            Self::ElectricGreen => "Pantone 14-5721 TCX Electric Green",
            Self::SeaFoam => "Pantone 14-6007 TCX Sea Foam",
            Self::SubtleGreen => "Pantone 14-6008 TCX Subtle Green",
            Self::GrayedJade => "Pantone 14-6011 TCX Grayed Jade",
            Self::NeptuneGreen => "Pantone 14-6017 TCX Neptune Green",
            Self::Pelican => "Pantone 14-6305 TCX Pelican",
            Self::Alfalfa => "Pantone 14-6308 TCX Alfalfa",
            Self::CameoGreen => "Pantone 14-6312 TCX Cameo Green",
            Self::Sprucestone => "Pantone 14-6316 TCX Sprucestone",
            Self::Meadow => "Pantone 14-6319 TCX Meadow",
            Self::Peapod => "Pantone 14-6324 TCX Peapod",
            Self::ZephyrGreen => "Pantone 14-6327 TCX Zephyr Green",
            Self::AbsintheGreen => "Pantone 14-6329 TCX Absinthe Green",
            Self::SpringBud => "Pantone 14-6330 TCX Spring Bud",
            Self::SpringBouquet => "Pantone 14-6340 TCX Spring Bouquet",
            Self::AbbeyStone => "Pantone 14-6408 TCX Abbey Stone",
            Self::Dove => "Pantone 15-0000 TCX Dove",
            Self::GreenFlash => "Pantone 15-0146 TCX Green Flash",
            Self::SprayGreen => "Pantone 15-0309 TCX Spray Green",
            Self::SageGreen => "Pantone 15-0318 TCX Sage Green",
            Self::Tarragon => "Pantone 15-0326 TCX Tarragon",
            Self::LeafGreen => "Pantone 15-0332 TCX Leaf Green",
            Self::HerbalGarden => "Pantone 15-0336 TCX Herbal Garden",
            Self::ParrotGreen => "Pantone 15-0341 TCX Parrot Green",
            Self::Greenery => "Pantone 15-0343 TCX Greenery",
            Self::Eucalyptus => "Pantone 15-0513 TCX Eucalyptus",
            Self::PaleOliveGreen => "Pantone 15-0522 TCX Pale Olive Green",
            Self::WinterPear => "Pantone 15-0523 TCX Winter Pear",
            Self::WeepingWillow => "Pantone 15-0525 TCX Weeping Willow",
            Self::SweetPea => "Pantone 15-0531 TCX Sweet Pea",
            Self::LindenGreen => "Pantone 15-0533 TCX Linden Green",
            Self::Palm => "Pantone 15-0535 TCX Palm",
            Self::GreenOasis => "Pantone 15-0538 TCX Green Oasis",
            Self::AppleGreen => "Pantone 15-0543 TCX Apple Green",
            Self::JasmineGreen => "Pantone 15-0545 TCX Jasmine Green",
            Self::Citronelle => "Pantone 15-0548 TCX Citronelle",
            Self::LeekGreen => "Pantone 15-0628 TCX Leek Green",
            Self::GoldenGreen => "Pantone 15-0636 TCX Golden Green",
            Self::CressGreen => "Pantone 15-0643 TCX Cress Green",
            Self::WarmOlive => "Pantone 15-0646 TCX Warm Olive",
            Self::AshesOfRoses => "Pantone 15-0703 TCX Ashes Of Roses",
            Self::SilverFern => "Pantone 15-0719 TCX Silver Fern",
            Self::SouthernMoss => "Pantone 15-0730 TCX Southern Moss",
            Self::Olivenite => "Pantone 15-0732 TCX Olivenite",
            Self::OilYellow => "Pantone 15-0743 TCX Oil Yellow",
            Self::LemonCurry => "Pantone 15-0751 TCX Lemon Curry",
            Self::CeylonYellow => "Pantone 15-0850 TCX Ceylon Yellow",
            Self::PaleGold => "Pantone 15-0927 TCX Pale Gold",
            Self::Sauterne => "Pantone 15-0942 TCX Sauterne",
            Self::GoldenSpice => "Pantone 15-0948 TCX Golden Spice",
            Self::GoldenYellow => "Pantone 15-0953 TCX Golden Yellow",
            Self::OldGold => "Pantone 15-0955 TCX Old Gold",
            Self::MangoMojito => "Pantone 15-0960 TCX Mango Mojito",
            Self::GingerRoot => "Pantone 15-1020 TCX Ginger Root",
            Self::IcedCoffee => "Pantone 15-1040 TCX Iced Coffee",
            Self::AutumnBlaze => "Pantone 15-1045 TCX Autumn Blaze",
            Self::MineralYellow => "Pantone 15-1046 TCX Mineral Yellow",
            Self::ArtisansGold => "Pantone 15-1049 TCX Artisans Gold",
            Self::GoldenGlow => "Pantone 15-1050 TCX Golden Glow",
            Self::GoldenOrange => "Pantone 15-1051 TCX Golden Orange",
            Self::CadmiumYellow => "Pantone 15-1054 TCX Cadmium Yellow",
            Self::RadiantYellow => "Pantone 15-1058 TCX Radiant Yellow",
            Self::GoldFusion => "Pantone 15-1062 TCX Gold Fusion",
            Self::Travertine => "Pantone 15-1114 TCX Travertine",
            Self::Safari => "Pantone 15-1116 TCX Safari",
            Self::TaosTaupe => "Pantone 15-1119 TCX Taos Taupe",
            Self::Porcini => "Pantone 15-1125 TCX Porcini",
            Self::FallLeaf => "Pantone 15-1132 TCX Fall Leaf",
            Self::HoneyGold => "Pantone 15-1142 TCX Honey Gold",
            Self::Chamois => "Pantone 15-1145 TCX Chamois",
            Self::Butterscotch => "Pantone 15-1147 TCX Butterscotch",
            Self::DarkCheddar => "Pantone 15-1150 TCX Dark Cheddar",
            Self::IcelandPoppy => "Pantone 15-1151 TCX Iceland Poppy",
            Self::Apricot => "Pantone 15-1153 TCX Apricot",
            Self::FlameOrange => "Pantone 15-1157 TCX Flame Orange",
            Self::BlazingOrange => "Pantone 15-1160 TCX Blazing Orange",
            Self::BrightMarigold => "Pantone 15-1164 TCX Bright Marigold",
            Self::CandiedGinger => "Pantone 15-1213 TCX Candied Ginger",
            Self::WarmSand => "Pantone 15-1214 TCX Warm Sand",
            Self::Sesame => "Pantone 15-1215 TCX Sesame",
            Self::PaleKhaki => "Pantone 15-1216 TCX Pale Khaki",
            Self::MojaveDesert => "Pantone 15-1217 TCX Mojave Desert",
            Self::Semolina => "Pantone 15-1218 TCX Semolina",
            Self::Latte => "Pantone 15-1220 TCX Latte",
            Self::Sand => "Pantone 15-1225 TCX Sand",
            Self::Clay => "Pantone 15-1231 TCX Clay",
            Self::GoldEarth => "Pantone 15-1234 TCX Gold Earth",
            Self::ApricotTan => "Pantone 15-1237 TCX Apricot Tan",
            Self::Cantaloupe => "Pantone 15-1239 TCX Cantaloupe",
            Self::Muskmelon => "Pantone 15-1242 TCX Muskmelon",
            Self::Papaya => "Pantone 15-1243 TCX Papaya",
            Self::MockOrange => "Pantone 15-1245 TCX Mock Orange",
            Self::Tangerine => "Pantone 15-1247 TCX Tangerine",
            Self::CarrotCurl => "Pantone 15-1262 TCX Carrot Curl",
            Self::AutumnGlory => "Pantone 15-1263 TCX Autumn Glory",
            Self::Turmeric => "Pantone 15-1264 TCX Turmeric",
            Self::Humus => "Pantone 15-1304 TCX Humus",
            Self::FeatherGray => "Pantone 15-1305 TCX Feather Gray",
            Self::OxfordTan => "Pantone 15-1306 TCX Oxford Tan",
            Self::WhitePepper => "Pantone 15-1307 TCX White Pepper",
            Self::Doeskin => "Pantone 15-1308 TCX Doeskin",
            Self::Moonlight => "Pantone 15-1309 TCX Moonlight",
            Self::CubanSand => "Pantone 15-1314 TCX Cuban Sand",
            Self::RugbyTan => "Pantone 15-1315 TCX Rugby Tan",
            Self::MapleSugar => "Pantone 15-1316 TCX Maple Sugar",
            Self::Sirocco => "Pantone 15-1317 TCX Sirocco",
            Self::PinkSand => "Pantone 15-1318 TCX Pink Sand",
            Self::AlmostApricot => "Pantone 15-1319 TCX Almost Apricot",
            Self::DustyCoral => "Pantone 15-1322 TCX Dusty Coral",
            Self::PeachBloom => "Pantone 15-1327 TCX Peach Bloom",
            Self::CoralReef => "Pantone 15-1331 TCX Coral Reef",
            Self::CanyonSunset => "Pantone 15-1333 TCX Canyon Sunset",
            Self::ShellCoral => "Pantone 15-1334 TCX Shell Coral",
            Self::Tangelo => "Pantone 15-1335 TCX Tangelo",
            Self::CadmiumOrange => "Pantone 15-1340 TCX Cadmium Orange",
            Self::CoralCloud => "Pantone 15-1415 TCX Coral Cloud",
            Self::PeachAmber => "Pantone 15-1423 TCX Peach Amber",
            Self::Fenugreek => "Pantone 15-1425 TCX Fenugreek",
            Self::DustedClay => "Pantone 15-1429 TCX Dusted Clay",
            Self::PastryShell => "Pantone 15-1430 TCX Pastry Shell",
            Self::PapayaPunch => "Pantone 15-1433 TCX Papaya Punch",
            Self::DesertFlower => "Pantone 15-1435 TCX Desert Flower",
            Self::Etherea => "Pantone 15-1506 TCX Etherea",
            Self::MahoganyRose => "Pantone 15-1511 TCX Mahogany Rose",
            Self::MistyRose => "Pantone 15-1512 TCX Misty Rose",
            Self::MellowRose => "Pantone 15-1515 TCX Mellow Rose",
            Self::PeachBeige => "Pantone 15-1516 TCX Peach Beige",
            Self::BloomingDahlia => "Pantone 15-1520 TCX Blooming Dahlia",
            Self::Shrimp => "Pantone 15-1523 TCX Shrimp",
            Self::PeachPink => "Pantone 15-1530 TCX Peach Pink",
            Self::PaleMauve => "Pantone 15-1607 TCX Pale Mauve",
            Self::BridalRose => "Pantone 15-1611 TCX Bridal Rose",
            Self::Blush => "Pantone 15-1614 TCX Blush",
            Self::CandlelightPeach => "Pantone 15-1621 TCX Candlelight Peach",
            Self::ConchShell => "Pantone 15-1624 TCX Conch Shell",
            Self::SalmonRose => "Pantone 15-1626 TCX Salmon Rose",
            Self::PinkIcing => "Pantone 15-1717 TCX Pink Icing",
            Self::Peony => "Pantone 15-1816 TCX Peony",
            Self::FlamingoPink => "Pantone 15-1821 TCX Flamingo Pink",
            Self::BurnishedLilac => "Pantone 15-1905 TCX Burnished Lilac",
            Self::Zephyr => "Pantone 15-1906 TCX Zephyr",
            Self::SeaPink => "Pantone 15-1912 TCX Sea Pink",
            Self::MorningGlory => "Pantone 15-1920 TCX Morning Glory",
            Self::GeraniumPink => "Pantone 15-1922 TCX Geranium Pink",
            Self::DawnPink => "Pantone 15-2205 TCX Dawn Pink",
            Self::OrchidSmoke => "Pantone 15-2210 TCX Orchid Smoke",
            Self::Rosebloom => "Pantone 15-2214 TCX Rosebloom",
            Self::BegoniaPink => "Pantone 15-2215 TCX Begonia Pink",
            Self::SachetPink => "Pantone 15-2216 TCX Sachet Pink",
            Self::AuroraPink => "Pantone 15-2217 TCX Aurora Pink",
            Self::KeepsakeLilac => "Pantone 15-2705 TCX Keepsake Lilac",
            Self::VioletIce => "Pantone 15-2706 TCX Violet Ice",
            Self::FuchsiaPink => "Pantone 15-2718 TCX Fuchsia Pink",
            Self::LilacChiffon => "Pantone 15-2913 TCX Lilac Chiffon",
            Self::MauveMist => "Pantone 15-3207 TCX Mauve Mist",
            Self::Orchid => "Pantone 15-3214 TCX Orchid",
            Self::OrchidBouquet => "Pantone 15-3412 TCX Orchid Bouquet",
            Self::LavenderFrost => "Pantone 15-3507 TCX Lavender Frost",
            Self::FairOrchid => "Pantone 15-3508 TCX Fair Orchid",
            Self::CrocusPetal => "Pantone 15-3520 TCX Crocus Petal",
            Self::Lavendula => "Pantone 15-3620 TCX Lavendula",
            Self::PurpleRose => "Pantone 15-3716 TCX Purple Rose",
            Self::LilacBreeze => "Pantone 15-3720 TCX Lilac Breeze",
            Self::Porpoise => "Pantone 15-3800 TCX Porpoise",
            Self::CloudGray => "Pantone 15-3802 TCX Cloud Gray",
            Self::MistyLilac => "Pantone 15-3807 TCX Misty Lilac",
            Self::Lavender => "Pantone 15-3817 TCX Lavender",
            Self::IcelandicBlue => "Pantone 15-3908 TCX Icelandic Blue",
            Self::CosmicSky => "Pantone 15-3909 TCX Cosmic Sky",
            Self::LanguidLavender => "Pantone 15-3910 TCX Languid Lavender",
            Self::Aleutian => "Pantone 15-3912 TCX Aleutian",
            Self::KentuckyBlue => "Pantone 15-3915 TCX Kentucky Blue",
            Self::Serenity => "Pantone 15-3919 TCX Serenity",
            Self::PlacidBlue => "Pantone 15-3920 TCX Placid Blue",
            Self::VistaBlue => "Pantone 15-3930 TCX Vista Blue",
            Self::BelAirBlue => "Pantone 15-3932 TCX Bel Air Blue",
            Self::StormGray => "Pantone 15-4003 TCX Storm Gray",
            Self::DreamBlue => "Pantone 15-4005 TCX Dream Blue",
            Self::BlueFog => "Pantone 15-4008 TCX Blue Fog",
            Self::Cerulean => "Pantone 15-4020 TCX Cerulean",
            Self::ChambrayBlue => "Pantone 15-4030 TCX Chambray Blue",
            Self::HighRise => "Pantone 15-4101 TCX High Rise",
            Self::AngelFalls => "Pantone 15-4105 TCX Angel Falls",
            Self::AlaskanBlue => "Pantone 15-4225 TCX Alaskan Blue",
            Self::Quarry => "Pantone 15-4305 TCX Quarry",
            Self::BelgianBlock => "Pantone 15-4306 TCX Belgian Block",
            Self::Tradewinds => "Pantone 15-4307 TCX Tradewinds",
            Self::SterlingBlue => "Pantone 15-4309 TCX Sterling Blue",
            Self::ForgetMeNot => "Pantone 15-4312 TCX Forget Me Not",
            Self::AirBlue => "Pantone 15-4319 TCX Air Blue",
            Self::EtherealBlue => "Pantone 15-4323 TCX Ethereal Blue",
            Self::MilkyBlue => "Pantone 15-4415 TCX Milky Blue",
            Self::BlueGrotto => "Pantone 15-4421 TCX Blue Grotto",
            Self::NorseBlue => "Pantone 15-4427 TCX Norse Blue",
            Self::CrystalSeas => "Pantone 15-4428 TCX Crystal Seas",
            Self::SilverCloud => "Pantone 15-4502 TCX Silver Cloud",
            Self::ChateauGray => "Pantone 15-4503 TCX Chateau Gray",
            Self::PuritanGray => "Pantone 15-4702 TCX Puritan Gray",
            Self::MirageGray => "Pantone 15-4703 TCX Mirage Gray",
            Self::Pigeon => "Pantone 15-4704 TCX Pigeon",
            Self::GrayMist => "Pantone 15-4706 TCX Gray Mist",
            Self::BlueHaze => "Pantone 15-4707 TCX Blue Haze",
            Self::MarineBlue => "Pantone 15-4712 TCX Marine Blue",
            Self::Aquarelle => "Pantone 15-4714 TCX Aquarelle",
            Self::AquaSea => "Pantone 15-4715 TCX Aqua Sea",
            Self::Aqua => "Pantone 15-4717 TCX Aqua",
            Self::RiverBlue => "Pantone 15-4720 TCX River Blue",
            Self::Capri => "Pantone 15-4722 TCX Capri",
            Self::BlueCuracao => "Pantone 15-4825 TCX Blue Curacao",
            Self::AquaGray => "Pantone 15-5205 TCX Aqua Gray",
            Self::Aquifer => "Pantone 15-5207 TCX Aquifer",
            Self::AquaHaze => "Pantone 15-5209 TCX Aqua Haze",
            Self::NileBlue => "Pantone 15-5210 TCX Nile Blue",
            Self::BlueTurquoise => "Pantone 15-5217 TCX Blue Turquoise",
            Self::PoolBlue => "Pantone 15-5218 TCX Pool Blue",
            Self::FloridaKeys => "Pantone 15-5416 TCX Florida Keys",
            Self::AquaGreen => "Pantone 15-5421 TCX Aqua Green",
            Self::Atlantis => "Pantone 15-5425 TCX Atlantis",
            Self::Waterfall => "Pantone 15-5516 TCX Waterfall",
            Self::Turquoise => "Pantone 15-5519 TCX Turquoise",
            Self::BrightGreen => "Pantone 15-5534 TCX Bright Green",
            Self::MineralGray => "Pantone 15-5704 TCX Mineral Gray",
            Self::FrostyGreen => "Pantone 15-5706 TCX Frosty Green",
            Self::DustyJadeGreen => "Pantone 15-5711 TCX Dusty Jade Green",
            Self::BiscayGreen => "Pantone 15-5718 TCX Biscay Green",
            Self::MintLeaf => "Pantone 15-5728 TCX Mint Leaf",
            Self::Lichen => "Pantone 15-5812 TCX Lichen",
            Self::Spearmint => "Pantone 15-5819 TCX Spearmint",
            Self::Hemlock => "Pantone 15-6114 TCX Hemlock",
            Self::Ming => "Pantone 15-6120 TCX Ming",
            Self::JadeCream => "Pantone 15-6123 TCX Jade Cream",
            Self::PussywillowGray => "Pantone 15-6304 TCX Pussywillow Gray",
            Self::AgateGray => "Pantone 15-6307 TCX Agate Gray",
            Self::Swamp => "Pantone 15-6310 TCX Swamp",
            Self::LaurelGreen => "Pantone 15-6313 TCX Laurel Green",
            Self::SmokeGreen => "Pantone 15-6315 TCX Smoke Green",
            Self::FairGreen => "Pantone 15-6316 TCX Fair Green",
            Self::QuietGreen => "Pantone 15-6317 TCX Quiet Green",
            Self::LightGrassGreen => "Pantone 15-6322 TCX Light Grass Green",
            Self::IrishGreen => "Pantone 15-6340 TCX Irish Green",
            Self::MossGray => "Pantone 15-6410 TCX Moss Gray",
            Self::Reseda => "Pantone 15-6414 TCX Reseda",
            Self::ForestShade => "Pantone 15-6423 TCX Forest Shade",
            Self::GreenTea => "Pantone 15-6428 TCX Green Tea",
            Self::Shamrock => "Pantone 15-6432 TCX Shamrock",
            Self::GrassGreen => "Pantone 15-6437 TCX Grass Green",
            Self::BudGreen => "Pantone 15-6442 TCX Bud Green",
            Self::Paloma => "Pantone 16-0000 TCX Paloma",
            Self::DesertSage => "Pantone 16-0110 TCX Desert Sage",
            Self::Tendril => "Pantone 16-0123 TCX Tendril",
            Self::VintageKhaki => "Pantone 16-0205 TCX Vintage Khaki",
            Self::LondonFog => "Pantone 16-0207 TCX London Fog",
            Self::Tea => "Pantone 16-0213 TCX Tea",
            Self::Mistletoe => "Pantone 16-0220 TCX Mistletoe",
            Self::GreenEyes => "Pantone 16-0224 TCX Green Eyes",
            Self::JadeGreen => "Pantone 16-0228 TCX Jade Green",
            Self::MacawGreen => "Pantone 16-0230 TCX Macaw Green",
            Self::MeadowGreen => "Pantone 16-0233 TCX Meadow Green",
            Self::Kiwi => "Pantone 16-0235 TCX Kiwi",
            Self::Foliage => "Pantone 16-0237 TCX Foliage",
            Self::Sage => "Pantone 16-0421 TCX Sage",
            Self::Fern => "Pantone 16-0430 TCX Fern",
            Self::DarkCitron => "Pantone 16-0435 TCX Dark Citron",
            Self::SpinachGreen => "Pantone 16-0439 TCX Spinach Green",
            Self::GrayGreen => "Pantone 16-0518 TCX Gray Green",
            Self::Cedar => "Pantone 16-0526 TCX Cedar",
            Self::Moss => "Pantone 16-0532 TCX Moss",
            Self::Oasis => "Pantone 16-0540 TCX Oasis",
            Self::GoldenLime => "Pantone 16-0543 TCX Golden Lime",
            Self::SplitPea => "Pantone 16-0545 TCX Split Pea",
            Self::LentilSprout => "Pantone 16-0550 TCX Lentil Sprout",
            Self::Elm => "Pantone 16-0613 TCX Elm",
            Self::Willow => "Pantone 16-0632 TCX Willow",
            Self::GoldenOlive => "Pantone 16-0639 TCX Golden Olive",
            Self::SlateGreen => "Pantone 16-0713 TCX Slate Green",
            Self::Khaki => "Pantone 16-0726 TCX Khaki",
            Self::AntiqueGold => "Pantone 16-0730 TCX Antique Gold",
            Self::BurnishedGold => "Pantone 16-0737 TCX Burnished Gold",
            Self::GreenSulphur => "Pantone 16-0742 TCX Green Sulphur",
            Self::Goat => "Pantone 16-0806 TCX Goat",
            Self::RichGold => "Pantone 16-0836 TCX Rich Gold",
            Self::AntiqueMoss => "Pantone 16-0840 TCX Antique Moss",
            Self::OliveOil => "Pantone 16-0847 TCX Olive Oil",
            Self::SimplyTaupe => "Pantone 16-0906 TCX Simply Taupe",
            Self::CurdsAndWhey => "Pantone 16-0920 TCX Curds And Whey",
            Self::Croissant => "Pantone 16-0924 TCX Croissant",
            Self::Curry => "Pantone 16-0928 TCX Curry",
            Self::Taffy => "Pantone 16-0940 TCX Taffy",
            Self::Tinsel => "Pantone 16-0945 TCX Tinsel",
            Self::Honey => "Pantone 16-0946 TCX Honey",
            Self::BrightGold => "Pantone 16-0947 TCX Bright Gold",
            Self::HarvestGold => "Pantone 16-0948 TCX Harvest Gold",
            Self::Narcissus => "Pantone 16-0950 TCX Narcissus",
            Self::NuggetGold => "Pantone 16-0952 TCX Nugget Gold",
            Self::TawnyOlive => "Pantone 16-0953 TCX Tawny Olive",
            Self::Arrowwood => "Pantone 16-0954 TCX Arrowwood",
            Self::Incense => "Pantone 16-1010 TCX Incense",
            Self::Sunflower => "Pantone 16-1054 TCX Sunflower",
            Self::PureCashmere => "Pantone 16-1103 TCX Pure Cashmere",
            Self::Crockery => "Pantone 16-1104 TCX Crockery",
            Self::PlazaTaupe => "Pantone 16-1105 TCX Plaza Taupe",
            Self::Tuffet => "Pantone 16-1106 TCX Tuffet",
            Self::Aluminum => "Pantone 16-1107 TCX Aluminum",
            Self::Twill => "Pantone 16-1108 TCX Twill",
            Self::Greige => "Pantone 16-1109 TCX Greige",
            Self::OliveGray => "Pantone 16-1110 TCX Olive Gray",
            Self::Sponge => "Pantone 16-1118 TCX Sponge",
            Self::Starfish => "Pantone 16-1120 TCX Starfish",
            Self::Antelope => "Pantone 16-1126 TCX Antelope",
            Self::MustardGold => "Pantone 16-1133 TCX Mustard Gold",
            Self::AmberGold => "Pantone 16-1139 TCX Amber Gold",
            Self::Yam => "Pantone 16-1140 TCX Yam",
            Self::GoldenNugget => "Pantone 16-1142 TCX Golden Nugget",
            Self::HoneyYellow => "Pantone 16-1143 TCX Honey Yellow",
            Self::OakBuff => "Pantone 16-1144 TCX Oak Buff",
            Self::Nugget => "Pantone 16-1148 TCX Nugget",
            Self::DesertSun => "Pantone 16-1149 TCX Desert Sun",
            Self::Topaz => "Pantone 16-1150 TCX Topaz",
            Self::OrangePepper => "Pantone 16-1164 TCX Orange Pepper",
            Self::LightTaupe => "Pantone 16-1210 TCX Light Taupe",
            Self::Nomad => "Pantone 16-1212 TCX Nomad",
            Self::Tuscany => "Pantone 16-1219 TCX Tuscany",
            Self::CafeCreme => "Pantone 16-1220 TCX Cafe Creme",
            Self::Roebuck => "Pantone 16-1221 TCX Roebuck",
            Self::Sandstorm => "Pantone 16-1235 TCX Sandstorm",
            Self::OrangeOchre => "Pantone 16-1253 TCX Orange Ochre",
            Self::RussetOrange => "Pantone 16-1255 TCX Russet Orange",
            Self::SunOrange => "Pantone 16-1257 TCX Sun Orange",
            Self::HarvestPumpkin => "Pantone 16-1260 TCX Harvest Pumpkin",
            Self::String => "Pantone 16-1305 TCX String",
            Self::Natural => "Pantone 16-1310 TCX Natural",
            Self::Cornstalk => "Pantone 16-1315 TCX Cornstalk",
            Self::Brush => "Pantone 16-1317 TCX Brush",
            Self::WarmTaupe => "Pantone 16-1318 TCX Warm Taupe",
            Self::Nougat => "Pantone 16-1320 TCX Nougat",
            Self::Macaroon => "Pantone 16-1323 TCX Macaroon",
            Self::Lark => "Pantone 16-1324 TCX Lark",
            Self::Copper => "Pantone 16-1325 TCX Copper",
            Self::PrairieSand => "Pantone 16-1326 TCX Prairie Sand",
            Self::ToastedNut => "Pantone 16-1327 TCX Toasted Nut",
            Self::Sandstone => "Pantone 16-1328 TCX Sandstone",
            Self::CoralHaze => "Pantone 16-1329 TCX Coral Haze",
            Self::MutedClay => "Pantone 16-1330 TCX Muted Clay",
            Self::Toast => "Pantone 16-1331 TCX Toast",
            Self::Pheasant => "Pantone 16-1332 TCX Pheasant",
            Self::Doe => "Pantone 16-1333 TCX Doe",
            Self::Tan => "Pantone 16-1334 TCX Tan",
            Self::Biscuit => "Pantone 16-1336 TCX Biscuit",
            Self::CoralGold => "Pantone 16-1337 TCX Coral Gold",
            Self::CopperTan => "Pantone 16-1338 TCX Copper Tan",
            Self::BrandiedMelon => "Pantone 16-1340 TCX Brandied Melon",
            Self::Butterum => "Pantone 16-1341 TCX Butterum",
            Self::Buckskin => "Pantone 16-1342 TCX Buckskin",
            Self::AutumnSunset => "Pantone 16-1343 TCX Autumn Sunset",
            Self::DustyOrange => "Pantone 16-1344 TCX Dusty Orange",
            Self::SunBaked => "Pantone 16-1345 TCX Sun Baked",
            Self::GoldenOchre => "Pantone 16-1346 TCX Golden Ochre",
            Self::PeachCaramel => "Pantone 16-1347 TCX Peach Caramel",
            Self::TomatoCream => "Pantone 16-1348 TCX Tomato Cream",
            Self::CoralRose => "Pantone 16-1349 TCX Coral Rose",
            Self::Amberglow => "Pantone 16-1350 TCX Amberglow",
            Self::PersimmonOrange => "Pantone 16-1356 TCX Persimmon Orange",
            Self::BirdOfParadise => "Pantone 16-1357 TCX Bird Of Paradise",
            Self::OrangeTiger => "Pantone 16-1358 TCX Orange Tiger",
            Self::OrangePeel => "Pantone 16-1359 TCX Orange Peel",
            Self::Nectarine => "Pantone 16-1360 TCX Nectarine",
            Self::Carrot => "Pantone 16-1361 TCX Carrot",
            Self::VermillionOrange => "Pantone 16-1362 TCX Vermillion Orange",
            Self::PuffinsBill => "Pantone 16-1363 TCX Puffins Bill",
            Self::VibrantOrange => "Pantone 16-1364 TCX Vibrant Orange",
            Self::Atmosphere => "Pantone 16-1406 TCX Atmosphere",
            Self::Cobblestone => "Pantone 16-1407 TCX Cobblestone",
            Self::Stucco => "Pantone 16-1412 TCX Stucco",
            Self::Chanterelle => "Pantone 16-1414 TCX Chanterelle",
            Self::Almondine => "Pantone 16-1415 TCX Almondine",
            Self::Cork => "Pantone 16-1422 TCX Cork",
            Self::Sunburn => "Pantone 16-1429 TCX Sunburn",
            Self::CanyonClay => "Pantone 16-1431 TCX Canyon Clay",
            Self::Almond => "Pantone 16-1432 TCX Almond",
            Self::CoralAlmond => "Pantone 16-1434 TCX Coral Almond",
            Self::Carnelian => "Pantone 16-1435 TCX Carnelian",
            Self::Meerkat => "Pantone 16-1438 TCX Meerkat",
            Self::Caramel => "Pantone 16-1439 TCX Caramel",
            Self::Langoustino => "Pantone 16-1440 TCX Langoustino",
            Self::Arabesque => "Pantone 16-1441 TCX Arabesque",
            Self::Melon => "Pantone 16-1442 TCX Melon",
            Self::ApricotBuff => "Pantone 16-1443 TCX Apricot Buff",
            Self::BurntOrange => "Pantone 16-1448 TCX Burnt Orange",
            Self::GoldFlame => "Pantone 16-1449 TCX Gold Flame",
            Self::Flamingo => "Pantone 16-1450 TCX Flamingo",
            Self::Nasturtium => "Pantone 16-1451 TCX Nasturtium",
            Self::Firecracker => "Pantone 16-1452 TCX Firecracker",
            Self::ExoticOrange => "Pantone 16-1453 TCX Exotic Orange",
            Self::JaffaOrange => "Pantone 16-1454 TCX Jaffa Orange",
            Self::MandarinOrange => "Pantone 16-1459 TCX Mandarin Orange",
            Self::DragonFire => "Pantone 16-1460 TCX Dragon Fire",
            Self::GoldenPoppy => "Pantone 16-1462 TCX Golden Poppy",
            Self::Bark => "Pantone 16-1506 TCX Bark",
            Self::AdobeRose => "Pantone 16-1508 TCX Adobe Rose",
            Self::ShadowGray => "Pantone 16-1509 TCX Shadow Gray",
            Self::Fawn => "Pantone 16-1510 TCX Fawn",
            Self::RoseTan => "Pantone 16-1511 TCX Rose Tan",
            Self::CameoBrown => "Pantone 16-1516 TCX Cameo Brown",
            Self::Rosette => "Pantone 16-1518 TCX Rosette",
            Self::LobsterBisque => "Pantone 16-1520 TCX Lobster Bisque",
            Self::RoseDawn => "Pantone 16-1522 TCX Rose Dawn",
            Self::TerraCotta => "Pantone 16-1526 TCX Terra Cotta",
            Self::BurntCoral => "Pantone 16-1529 TCX Burnt Coral",
            Self::Crabapple => "Pantone 16-1532 TCX Crabapple",
            Self::Coral => "Pantone 16-1539 TCX Coral",
            Self::Camellia => "Pantone 16-1541 TCX Camellia",
            Self::FreshSalmon => "Pantone 16-1542 TCX Fresh Salmon",
            Self::FusionCoral => "Pantone 16-1543 TCX Fusion Coral",
            Self::Persimmon => "Pantone 16-1544 TCX Persimmon",
            Self::CoralQuartz => "Pantone 16-1545 TCX Coral Quartz",
            Self::LivingCoral => "Pantone 16-1546 TCX Living Coral",
            Self::PeachEcho => "Pantone 16-1548 TCX Peach Echo",
            Self::PurpleDove => "Pantone 16-1606 TCX Purple Dove",
            Self::BrandiedApricot => "Pantone 16-1610 TCX Brandied Apricot",
            Self::Mauveglow => "Pantone 16-1617 TCX Mauveglow",
            Self::TeaRose => "Pantone 16-1620 TCX Tea Rose",
            Self::Lantana => "Pantone 16-1624 TCX Lantana",
            Self::PeachBlossom => "Pantone 16-1626 TCX Peach Blossom",
            Self::ShellPink => "Pantone 16-1632 TCX Shell Pink",
            Self::SugarCoral => "Pantone 16-1640 TCX Sugar Coral",
            Self::GeorgiaPeach => "Pantone 16-1641 TCX Georgia Peach",
            Self::Sphinx => "Pantone 16-1703 TCX Sphinx",
            Self::DeauvilleMauve => "Pantone 16-1707 TCX Deauville Mauve",
            Self::Lilas => "Pantone 16-1708 TCX Lilas",
            Self::Foxglove => "Pantone 16-1710 TCX Foxglove",
            Self::Polignac => "Pantone 16-1712 TCX Polignac",
            Self::WildRose => "Pantone 16-1715 TCX Wild Rose",
            Self::StrawberryIce => "Pantone 16-1720 TCX Strawberry Ice",
            Self::Confetti => "Pantone 16-1723 TCX Confetti",
            Self::StrawberryPink => "Pantone 16-1731 TCX Strawberry Pink",
            Self::PinkLemonade => "Pantone 16-1735 TCX Pink Lemonade",
            Self::Woodrose => "Pantone 16-1806 TCX Woodrose",
            Self::OrchidHaze => "Pantone 16-2107 TCX Orchid Haze",
            Self::MauveOrchid => "Pantone 16-2111 TCX Mauve Orchid",
            Self::WildOrchid => "Pantone 16-2120 TCX Wild Orchid",
            Self::PinkCarnation => "Pantone 16-2124 TCX Pink Carnation",
            Self::AzaleaPink => "Pantone 16-2126 TCX Azalea Pink",
            Self::CashmereRose => "Pantone 16-2215 TCX Cashmere Rose",
            Self::MoonliteMauve => "Pantone 16-2614 TCX Moonlite Mauve",
            Self::SmokyGrape => "Pantone 16-3110 TCX Smoky Grape",
            Self::Crocus => "Pantone 16-3115 TCX Crocus",
            Self::OperaMauve => "Pantone 16-3116 TCX Opera Mauve",
            Self::Cyclamen => "Pantone 16-3118 TCX Cyclamen",
            Self::MauveShadows => "Pantone 16-3205 TCX Mauve Shadows",
            Self::SeaFog => "Pantone 16-3304 TCX Sea Fog",
            Self::LavenderMist => "Pantone 16-3307 TCX Lavender Mist",
            Self::LavenderHerb => "Pantone 16-3310 TCX Lavender Herb",
            Self::Violet => "Pantone 16-3320 TCX Violet",
            Self::VioletTulle => "Pantone 16-3416 TCX Violet Tulle",
            Self::AfricanViolet => "Pantone 16-3520 TCX African Violet",
            Self::Lupine => "Pantone 16-3521 TCX Lupine",
            Self::RegalOrchid => "Pantone 16-3525 TCX Regal Orchid",
            Self::SheerLilac => "Pantone 16-3617 TCX Sheer Lilac",
            Self::SandVerbena => "Pantone 16-3720 TCX Sand Verbena",
            Self::Satellite => "Pantone 16-3800 TCX Satellite",
            Self::OpalGray => "Pantone 16-3801 TCX Opal Gray",
            Self::Ash => "Pantone 16-3802 TCX Ash",
            Self::GullGray => "Pantone 16-3803 TCX Gull Gray",
            Self::Wisteria => "Pantone 16-3810 TCX Wisteria",
            Self::HeirloomLilac => "Pantone 16-3812 TCX Heirloom Lilac",
            Self::Viola => "Pantone 16-3815 TCX Viola",
            Self::Rhapsody => "Pantone 16-3817 TCX Rhapsody",
            Self::VioletTulip => "Pantone 16-3823 TCX Violet Tulip",
            Self::SilverSconce => "Pantone 16-3850 TCX Silver Sconce",
            Self::LilacGray => "Pantone 16-3905 TCX Lilac Gray",
            Self::DappleGray => "Pantone 16-3907 TCX Dapple Gray",
            Self::LavenderAura => "Pantone 16-3911 TCX Lavender Aura",
            Self::Alloy => "Pantone 16-3915 TCX Alloy",
            Self::Sleet => "Pantone 16-3916 TCX Sleet",
            Self::Eventide => "Pantone 16-3919 TCX Eventide",
            Self::LavenderLustre => "Pantone 16-3920 TCX Lavender Lustre",
            Self::BlueHeron => "Pantone 16-3921 TCX Blue Heron",
            Self::BrunneraBlue => "Pantone 16-3922 TCX Brunnera Blue",
            Self::EasterEgg => "Pantone 16-3925 TCX Easter Egg",
            Self::Grapemist => "Pantone 16-3929 TCX Grapemist",
            Self::ThistleDown => "Pantone 16-3930 TCX Thistle Down",
            Self::SweetLavender => "Pantone 16-3931 TCX Sweet Lavender",
            Self::DustyBlue => "Pantone 16-4010 TCX Dusty Blue",
            Self::AshleyBlue => "Pantone 16-4013 TCX Ashley Blue",
            Self::ForeverBlue => "Pantone 16-4019 TCX Forever Blue",
            Self::DellaRobbiaBlue => "Pantone 16-4020 TCX Della Robbia Blue",
            Self::Allure => "Pantone 16-4021 TCX Allure",
            Self::Hydrangea => "Pantone 16-4030 TCX Hydrangea",
            Self::CornflowerBlue => "Pantone 16-4031 TCX Cornflower Blue",
            Self::Provence => "Pantone 16-4032 TCX Provence",
            Self::GranadaSky => "Pantone 16-4033 TCX Granada Sky",
            Self::Arona => "Pantone 16-4109 TCX Arona",
            Self::StoneBlue => "Pantone 16-4114 TCX Stone Blue",
            Self::DuskBlue => "Pantone 16-4120 TCX Dusk Blue",
            Self::HeritageBlue => "Pantone 16-4127 TCX Heritage Blue",
            Self::LittleBoyBlue => "Pantone 16-4132 TCX Little Boy Blue",
            Self::BonnieBlue => "Pantone 16-4134 TCX Bonnie Blue",
            Self::MourningDove => "Pantone 16-4400 TCX Mourning Dove",
            Self::Drizzle => "Pantone 16-4402 TCX Drizzle",
            Self::Abyss => "Pantone 16-4404 TCX Abyss",
            Self::Slate => "Pantone 16-4408 TCX Slate",
            Self::Tourmaline => "Pantone 16-4411 TCX Tourmaline",
            Self::CameoBlue => "Pantone 16-4414 TCX Cameo Blue",
            Self::BlueMist => "Pantone 16-4421 TCX Blue Mist",
            Self::HorizonBlue => "Pantone 16-4427 TCX Horizon Blue",
            Self::DelphiniumBlue => "Pantone 16-4519 TCX Delphinium Blue",
            Self::MauiBlue => "Pantone 16-4525 TCX Maui Blue",
            Self::CyanBlue => "Pantone 16-4529 TCX Cyan Blue",
            Self::Aquarius => "Pantone 16-4530 TCX Aquarius",
            Self::BlueAtoll => "Pantone 16-4535 TCX Blue Atoll",
            Self::Stillwater => "Pantone 16-4610 TCX Stillwater",
            Self::ReefWaters => "Pantone 16-4612 TCX Reef Waters",
            Self::Limestone => "Pantone 16-4702 TCX Limestone",
            Self::GhostGray => "Pantone 16-4703 TCX Ghost Gray",
            Self::SilverBlue => "Pantone 16-4706 TCX Silver Blue",
            Self::MineralBlue => "Pantone 16-4712 TCX Mineral Blue",
            Self::Porcelain => "Pantone 16-4719 TCX Porcelain",
            Self::ScubaBlue => "Pantone 16-4725 TCX Scuba Blue",
            Self::PeacockBlue => "Pantone 16-4728 TCX Peacock Blue",
            Self::Bluebird => "Pantone 16-4834 TCX Bluebird",
            Self::WetWeather => "Pantone 16-5101 TCX Wet Weather",
            Self::BlueSurf => "Pantone 16-5106 TCX Blue Surf",
            Self::Wasabi => "Pantone 16-5109 TCX Wasabi",
            Self::Canton => "Pantone 16-5112 TCX Canton",
            Self::DustyTurquoise => "Pantone 16-5114 TCX Dusty Turquoise",
            Self::SeaBlue => "Pantone 16-5119 TCX Sea Blue",
            Self::Meadowbrook => "Pantone 16-5121 TCX Meadowbrook",
            Self::Baltic => "Pantone 16-5123 TCX Baltic",
            Self::Ceramic => "Pantone 16-5127 TCX Ceramic",
            Self::Jadeite => "Pantone 16-5304 TCX Jadeite",
            Self::AgateGreen => "Pantone 16-5412 TCX Agate Green",
            Self::Lagoon => "Pantone 16-5418 TCX Lagoon",
            Self::SeaGreen => "Pantone 16-5421 TCX Sea Green",
            Self::BrightAqua => "Pantone 16-5422 TCX Bright Aqua",
            Self::PoolGreen => "Pantone 16-5425 TCX Pool Green",
            Self::Billiard => "Pantone 16-5427 TCX Billiard",
            Self::PeacockGreen => "Pantone 16-5431 TCX Peacock Green",
            Self::BerylGreen => "Pantone 16-5515 TCX Beryl Green",
            Self::Arcadia => "Pantone 16-5533 TCX Arcadia",
            Self::MarineGreen => "Pantone 16-5721 TCX Marine Green",
            Self::FlintGray => "Pantone 16-5803 TCX Flint Gray",
            Self::SlateGray => "Pantone 16-5804 TCX Slate Gray",
            Self::GreenMilieu => "Pantone 16-5806 TCX Green Milieu",
            Self::LilyPad => "Pantone 16-5807 TCX Lily Pad",
            Self::IcebergGreen => "Pantone 16-5808 TCX Iceberg Green",
            Self::GreenBay => "Pantone 16-5810 TCX Green Bay",
            Self::Feldspar => "Pantone 16-5815 TCX Feldspar",
            Self::GreenSpruce => "Pantone 16-5820 TCX Green Spruce",
            Self::GumdropGreen => "Pantone 16-5825 TCX Gumdrop Green",
            Self::WroughtIron => "Pantone 16-5904 TCX Wrought Iron",
            Self::GraniteGreen => "Pantone 16-5907 TCX Granite Green",
            Self::MalachiteGreen => "Pantone 16-5917 TCX Malachite Green",
            Self::CremeDeMenthe => "Pantone 16-5919 TCX Creme De Menthe",
            Self::WinterGreen => "Pantone 16-5924 TCX Winter Green",
            Self::MingGreen => "Pantone 16-5930 TCX Ming Green",
            Self::HollyGreen => "Pantone 16-5932 TCX Holly Green",
            Self::Mint => "Pantone 16-5938 TCX Mint",
            Self::Blarney => "Pantone 16-5942 TCX Blarney",
            Self::Seagrass => "Pantone 16-6008 TCX Seagrass",
            Self::Katydid => "Pantone 16-6030 TCX Katydid",
            Self::ShaleGreen => "Pantone 16-6116 TCX Shale Green",
            Self::Greenbriar => "Pantone 16-6127 TCX Greenbriar",
            Self::KellyGreen => "Pantone 16-6138 TCX Kelly Green",
            Self::Basil => "Pantone 16-6216 TCX Basil",
            Self::IslandGreen => "Pantone 16-6240 TCX Island Green",
            Self::MineralGreen => "Pantone 16-6318 TCX Mineral Green",
            Self::Jadesheen => "Pantone 16-6324 TCX Jadesheen",
            Self::Peppermint => "Pantone 16-6329 TCX Peppermint",
            Self::VibrantGreen => "Pantone 16-6339 TCX Vibrant Green",
            Self::ClassicGreen => "Pantone 16-6340 TCX Classic Green",
            Self::PoisonGreen => "Pantone 16-6444 TCX Poison Green",
            Self::FrostGray => "Pantone 17-0000 TCX Frost Gray",
            Self::OilGreen => "Pantone 17-0115 TCX Oil Green",
            Self::TurfGreen => "Pantone 17-0119 TCX Turf Green",
            Self::StoneGreen => "Pantone 17-0123 TCX Stone Green",
            Self::FluoriteGreen => "Pantone 17-0133 TCX Fluorite Green",
            Self::OnlineLime => "Pantone 17-0145 TCX Online Lime",
            Self::ElephantSkin => "Pantone 17-0205 TCX Elephant Skin",
            Self::RockRidge => "Pantone 17-0207 TCX Rock Ridge",
            Self::LodenFrost => "Pantone 17-0210 TCX Loden Frost",
            Self::AspenGreen => "Pantone 17-0215 TCX Aspen Green",
            Self::Watercress => "Pantone 17-0220 TCX Watercress",
            Self::ForestGreen => "Pantone 17-0230 TCX Forest Green",
            Self::PiquantGreen => "Pantone 17-0235 TCX Piquant Green",
            Self::Epsom => "Pantone 17-0324 TCX Epsom",
            Self::TurtleGreen => "Pantone 17-0330 TCX Turtle Green",
            Self::Peridot => "Pantone 17-0336 TCX Peridot",
            Self::SilverSage => "Pantone 17-0510 TCX Silver Sage",
            Self::DuskyGreen => "Pantone 17-0517 TCX Dusky Green",
            Self::Mosstone => "Pantone 17-0525 TCX Mosstone",
            Self::Guacamole => "Pantone 17-0530 TCX Guacamole",
            Self::GreenOlive => "Pantone 17-0535 TCX Green Olive",
            Self::LaurelOak => "Pantone 17-0610 TCX Laurel Oak",
            Self::Vetiver => "Pantone 17-0613 TCX Vetiver",
            Self::Mermaid => "Pantone 17-0618 TCX Mermaid",
            Self::Aloe => "Pantone 17-0620 TCX Aloe",
            Self::Boa => "Pantone 17-0625 TCX Boa",
            Self::DriedHerb => "Pantone 17-0627 TCX Dried Herb",
            Self::TreeHouse => "Pantone 17-0630 TCX Tree House",
            Self::GreenMoss => "Pantone 17-0636 TCX Green Moss",
            Self::TaupeGray => "Pantone 17-0808 TCX Taupe Gray",
            Self::EcruOlive => "Pantone 17-0836 TCX Ecru Olive",
            Self::GoldenPalm => "Pantone 17-0839 TCX Golden Palm",
            Self::AmberGreen => "Pantone 17-0840 TCX Amber Green",
            Self::BronzeMist => "Pantone 17-0843 TCX Bronze Mist",
            Self::Fossil => "Pantone 17-0909 TCX Fossil",
            Self::FennelSeed => "Pantone 17-0929 TCX Fennel Seed",
            Self::DullGold => "Pantone 17-0935 TCX Dull Gold",
            Self::MedalBronze => "Pantone 17-0942 TCX Medal Bronze",
            Self::ChaiTea => "Pantone 17-0949 TCX Chai Tea",
            Self::Dune => "Pantone 17-1009 TCX Dune",
            Self::Elmwood => "Pantone 17-1019 TCX Elmwood",
            Self::Kelp => "Pantone 17-1022 TCX Kelp",
            Self::AntiqueBronze => "Pantone 17-1028 TCX Antique Bronze",
            Self::Bistre => "Pantone 17-1036 TCX Bistre",
            Self::TigersEye => "Pantone 17-1038 TCX Tigers Eye",
            Self::SpruceYellow => "Pantone 17-1040 TCX Spruce Yellow",
            Self::Chipmunk => "Pantone 17-1044 TCX Chipmunk",
            Self::AppleCinnamon => "Pantone 17-1045 TCX Apple Cinnamon",
            Self::GoldenOak => "Pantone 17-1046 TCX Golden Oak",
            Self::HoneyMustard => "Pantone 17-1047 TCX Honey Mustard",
            Self::IncaGold => "Pantone 17-1048 TCX Inca Gold",
            Self::RoastedPecan => "Pantone 17-1052 TCX Roasted Pecan",
            Self::RoastedCashew => "Pantone 17-1105 TCX Roasted Cashew",
            Self::SenecaRock => "Pantone 17-1107 TCX Seneca Rock",
            Self::WinterTwig => "Pantone 17-1108 TCX Winter Twig",
            Self::Chinchilla => "Pantone 17-1109 TCX Chinchilla",
            Self::Coriander => "Pantone 17-1113 TCX Coriander",
            Self::PetrifiedOak => "Pantone 17-1115 TCX Petrified Oak",
            Self::LeadGray => "Pantone 17-1118 TCX Lead Gray",
            Self::Dijon => "Pantone 17-1125 TCX Dijon",
            Self::BoneBrown => "Pantone 17-1128 TCX Bone Brown",
            Self::WoodThrush => "Pantone 17-1129 TCX Wood Thrush",
            Self::BrownSugar => "Pantone 17-1134 TCX Brown Sugar",
            Self::Cashew => "Pantone 17-1137 TCX Cashew",
            Self::Marmalade => "Pantone 17-1140 TCX Marmalade",
            Self::ArganOil => "Pantone 17-1142 TCX Argan Oil",
            Self::Hazel => "Pantone 17-1143 TCX Hazel",
            Self::AutumnMaple => "Pantone 17-1145 TCX Autumn Maple",
            Self::AmberBrown => "Pantone 17-1147 TCX Amber Brown",
            Self::MoonRock => "Pantone 17-1210 TCX Moon Rock",
            Self::Fungi => "Pantone 17-1212 TCX Fungi",
            Self::Praline => "Pantone 17-1223 TCX Praline",
            Self::Camel => "Pantone 17-1224 TCX Camel",
            Self::TawnyBirch => "Pantone 17-1225 TCX Tawny Birch",
            Self::TawnyBrown => "Pantone 17-1226 TCX Tawny Brown",
            Self::CafeAuLait => "Pantone 17-1227 TCX Cafe Au Lait",
            Self::MochaMousse => "Pantone 17-1230 TCX Mocha Mousse",
            Self::TimberWolf => "Pantone 17-1310 TCX Timber Wolf",
            Self::DesertTaupe => "Pantone 17-1311 TCX Desert Taupe",
            Self::SilverMink => "Pantone 17-1312 TCX Silver Mink",
            Self::SepiaTint => "Pantone 17-1314 TCX Sepia Tint",
            Self::Portabella => "Pantone 17-1316 TCX Portabella",
            Self::Amphora => "Pantone 17-1319 TCX Amphora",
            Self::Tannin => "Pantone 17-1320 TCX Tannin",
            Self::Woodsmoke => "Pantone 17-1321 TCX Woodsmoke",
            Self::Burro => "Pantone 17-1322 TCX Burro",
            Self::TobaccoBrown => "Pantone 17-1327 TCX Tobacco Brown",
            Self::IndianTan => "Pantone 17-1328 TCX Indian Tan",
            Self::Lion => "Pantone 17-1330 TCX Lion",
            Self::Bran => "Pantone 17-1336 TCX Bran",
            Self::Adobe => "Pantone 17-1340 TCX Adobe",
            Self::TawnyOrange => "Pantone 17-1341 TCX Tawny Orange",
            Self::Autumnal => "Pantone 17-1342 TCX Autumnal",
            Self::SpiceRoute => "Pantone 17-1345 TCX Spice Route",
            Self::AutumnLeaf => "Pantone 17-1347 TCX Autumn Leaf",
            Self::Exuberance => "Pantone 17-1349 TCX Exuberance",
            Self::OrangePopsicle => "Pantone 17-1350 TCX Orange Popsicle",
            Self::ApricotOrange => "Pantone 17-1353 TCX Apricot Orange",
            Self::CelosiaOrange => "Pantone 17-1360 TCX Celosia Orange",
            Self::ScarletIbis => "Pantone 17-1361 TCX Scarlet Ibis",
            Self::PineBark => "Pantone 17-1410 TCX Pine Bark",
            Self::BeaverFur => "Pantone 17-1417 TCX Beaver Fur",
            Self::GingerSnap => "Pantone 17-1418 TCX Ginger Snap",
            Self::RawUmber => "Pantone 17-1422 TCX Raw Umber",
            Self::BrickDust => "Pantone 17-1424 TCX Brick Dust",
            Self::PecanBrown => "Pantone 17-1430 TCX Pecan Brown",
            Self::RawSienna => "Pantone 17-1436 TCX Raw Sienna",
            Self::Ginger => "Pantone 17-1444 TCX Ginger",
            Self::Mango => "Pantone 17-1446 TCX Mango",
            Self::PureedPumpkin => "Pantone 17-1449 TCX Pureed Pumpkin",
            Self::SummerFig => "Pantone 17-1450 TCX Summer Fig",
            Self::Koi => "Pantone 17-1452 TCX Koi",
            Self::Tigerlily => "Pantone 17-1456 TCX Tigerlily",
            Self::Orangeade => "Pantone 17-1461 TCX Orangeade",
            Self::Flame => "Pantone 17-1462 TCX Flame",
            Self::TangerineTango => "Pantone 17-1463 TCX Tangerine Tango",
            Self::RedOrange => "Pantone 17-1464 TCX Red Orange",
            Self::SteepleGray => "Pantone 17-1500 TCX Steeple Gray",
            Self::WildDove => "Pantone 17-1501 TCX Wild Dove",
            Self::Cloudburst => "Pantone 17-1502 TCX Cloudburst",
            Self::StormFront => "Pantone 17-1503 TCX Storm Front",
            Self::Quail => "Pantone 17-1505 TCX Quail",
            Self::Cinder => "Pantone 17-1506 TCX Cinder",
            Self::Antler => "Pantone 17-1510 TCX Antler",
            Self::WistfulMauve => "Pantone 17-1511 TCX Wistful Mauve",
            Self::NostalgiaRose => "Pantone 17-1512 TCX Nostalgia Rose",
            Self::AshRose => "Pantone 17-1514 TCX Ash Rose",
            Self::Burlwood => "Pantone 17-1516 TCX Burlwood",
            Self::OldRose => "Pantone 17-1518 TCX Old Rose",
            Self::CanyonRose => "Pantone 17-1520 TCX Canyon Rose",
            Self::Mauvewood => "Pantone 17-1522 TCX Mauvewood",
            Self::DesertSand => "Pantone 17-1524 TCX Desert Sand",
            Self::CedarWood => "Pantone 17-1525 TCX Cedar Wood",
            Self::Aragon => "Pantone 17-1532 TCX Aragon",
            Self::MineralRed => "Pantone 17-1537 TCX Mineral Red",
            Self::ApricotBrandy => "Pantone 17-1540 TCX Apricot Brandy",
            Self::BurntSienna => "Pantone 17-1544 TCX Burnt Sienna",
            Self::Cranberry => "Pantone 17-1545 TCX Cranberry",
            Self::Emberglow => "Pantone 17-1547 TCX Emberglow",
            Self::Paprika => "Pantone 17-1553 TCX Paprika",
            Self::Grenadine => "Pantone 17-1558 TCX Grenadine",
            Self::MandarinRed => "Pantone 17-1562 TCX Mandarin Red",
            Self::CherryTomato => "Pantone 17-1563 TCX Cherry Tomato",
            Self::Fiesta => "Pantone 17-1564 TCX Fiesta",
            Self::Elderberry => "Pantone 17-1605 TCX Elderberry",
            Self::HeatherRose => "Pantone 17-1608 TCX Heather Rose",
            Self::MesaRose => "Pantone 17-1609 TCX Mesa Rose",
            Self::DuskyOrchid => "Pantone 17-1610 TCX Dusky Orchid",
            Self::MellowMauve => "Pantone 17-1612 TCX Mellow Mauve",
            Self::DecoRose => "Pantone 17-1614 TCX Deco Rose",
            Self::RoseWine => "Pantone 17-1623 TCX Rose Wine",
            Self::HollyBerry => "Pantone 17-1633 TCX Holly Berry",
            Self::RoseOfSharon => "Pantone 17-1635 TCX Rose Of Sharon",
            Self::Chrysanthemum => "Pantone 17-1641 TCX Chrysanthemum",
            Self::PorcelainRose => "Pantone 17-1643 TCX Porcelain Rose",
            Self::SpicedCoral => "Pantone 17-1644 TCX Spiced Coral",
            Self::Dubarry => "Pantone 17-1647 TCX Dubarry",
            Self::Poinsettia => "Pantone 17-1654 TCX Poinsettia",
            Self::HotCoral => "Pantone 17-1656 TCX Hot Coral",
            Self::Bittersweet => "Pantone 17-1663 TCX Bittersweet",
            Self::PoppyRed => "Pantone 17-1664 TCX Poppy Red",
            Self::Moonscape => "Pantone 17-1708 TCX Moonscape",
            Self::Bordeaux => "Pantone 17-1710 TCX Bordeaux",
            Self::DustyRose => "Pantone 17-1718 TCX Dusty Rose",
            Self::Malaga => "Pantone 17-1723 TCX Malaga",
            Self::SunkistCoral => "Pantone 17-1736 TCX Sunkist Coral",
            Self::ClaretRed => "Pantone 17-1740 TCX Claret Red",
            Self::CalypsoCoral => "Pantone 17-1744 TCX Calypso Coral",
            Self::Geranium => "Pantone 17-1753 TCX Geranium",
            Self::ParadisePink => "Pantone 17-1755 TCX Paradise Pink",
            Self::RedViolet => "Pantone 17-1818 TCX Red Violet",
            Self::Carmine => "Pantone 17-1831 TCX Carmine",
            Self::Azalea => "Pantone 17-1842 TCX Azalea",
            Self::FruitDove => "Pantone 17-1926 TCX Fruit Dove",
            Self::DesertRose => "Pantone 17-1927 TCX Desert Rose",
            Self::Bubblegum => "Pantone 17-1928 TCX Bubblegum",
            Self::RaptureRose => "Pantone 17-1929 TCX Rapture Rose",
            Self::CamelliaRose => "Pantone 17-1930 TCX Camellia Rose",
            Self::HotPink => "Pantone 17-1937 TCX Hot Pink",
            Self::FuchsiaRose => "Pantone 17-2031 TCX Fuchsia Rose",
            Self::FandangoPink => "Pantone 17-2033 TCX Fandango Pink",
            Self::PinkYarrow => "Pantone 17-2034 TCX Pink Yarrow",
            Self::Magenta => "Pantone 17-2036 TCX Magenta",
            Self::ChateauRose => "Pantone 17-2120 TCX Chateau Rose",
            Self::ShockingPink => "Pantone 17-2127 TCX Shocking Pink",
            Self::LilacRose => "Pantone 17-2227 TCX Lilac Rose",
            Self::CarmineRose => "Pantone 17-2230 TCX Carmine Rose",
            Self::Toadstool => "Pantone 17-2411 TCX Toadstool",
            Self::IbisRose => "Pantone 17-2520 TCX Ibis Rose",
            Self::Zinc => "Pantone 17-2601 TCX Zinc",
            Self::DahliaMauve => "Pantone 17-2617 TCX Dahlia Mauve",
            Self::RoseViolet => "Pantone 17-2624 TCX Rose Violet",
            Self::SuperPink => "Pantone 17-2625 TCX Super Pink",
            Self::PhloxPink => "Pantone 17-2627 TCX Phlox Pink",
            Self::Mulberry => "Pantone 17-3014 TCX Mulberry",
            Self::SpringCrocus => "Pantone 17-3020 TCX Spring Crocus",
            Self::Rosebud => "Pantone 17-3023 TCX Rosebud",
            Self::Bodacious => "Pantone 17-3240 TCX Bodacious",
            Self::DustyLavender => "Pantone 17-3313 TCX Dusty Lavender",
            Self::IrisOrchid => "Pantone 17-3323 TCX Iris Orchid",
            Self::Valerian => "Pantone 17-3410 TCX Valerian",
            Self::DiffusedOrchid => "Pantone 17-3520 TCX Diffused Orchid",
            Self::OrchidMist => "Pantone 17-3612 TCX Orchid Mist",
            Self::ChalkViolet => "Pantone 17-3615 TCX Chalk Violet",
            Self::EnglishLavendar => "Pantone 17-3617 TCX English Lavendar",
            Self::Hyacinth => "Pantone 17-3619 TCX Hyacinth",
            Self::AmethystOrchid => "Pantone 17-3628 TCX Amethyst Orchid",
            Self::FairyWren => "Pantone 17-3640 TCX Fairy Wren",
            Self::Bougainvillea => "Pantone 17-3725 TCX Bougainvillea",
            Self::PaisleyPurple => "Pantone 17-3730 TCX Paisley Purple",
            Self::SunlitAllium => "Pantone 17-3735 TCX Sunlit Allium",
            Self::Gull => "Pantone 17-3802 TCX Gull",
            Self::Nirvana => "Pantone 17-3808 TCX Nirvana",
            Self::PurpleAsh => "Pantone 17-3810 TCX Purple Ash",
            Self::Dusk => "Pantone 17-3812 TCX Dusk",
            Self::Daybreak => "Pantone 17-3817 TCX Daybreak",
            Self::AsterPurple => "Pantone 17-3826 TCX Aster Purple",
            Self::DahliaPurple => "Pantone 17-3834 TCX Dahlia Purple",
            Self::MinimalGray => "Pantone 17-3906 TCX Minimal Gray",
            Self::Quicksilver => "Pantone 17-3907 TCX Quicksilver",
            Self::LavenderGray => "Pantone 17-3910 TCX Lavender Gray",
            Self::SilverFiligree => "Pantone 17-3911 TCX Silver Filigree",
            Self::Sharkskin => "Pantone 17-3914 TCX Sharkskin",
            Self::Tempest => "Pantone 17-3915 TCX Tempest",
            Self::Stonewash => "Pantone 17-3917 TCX Stonewash",
            Self::CountryBlue => "Pantone 17-3918 TCX Country Blue",
            Self::PurpleImpression => "Pantone 17-3919 TCX Purple Impression",
            Self::EnglishManor => "Pantone 17-3920 TCX English Manor",
            Self::BlueIce => "Pantone 17-3922 TCX Blue Ice",
            Self::ColonyBlue => "Pantone 17-3923 TCX Colony Blue",
            Self::LavenderViolet => "Pantone 17-3924 TCX Lavender Violet",
            Self::PersianViolet => "Pantone 17-3925 TCX Persian Violet",
            Self::PaleIris => "Pantone 17-3929 TCX Pale Iris",
            Self::Jacaranda => "Pantone 17-3930 TCX Jacaranda",
            Self::DeepPeriwinkle => "Pantone 17-3932 TCX Deep Periwinkle",
            Self::SilverBullet => "Pantone 17-3933 TCX Silver Bullet",
            Self::PersianJewel => "Pantone 17-3934 TCX Persian Jewel",
            Self::BlueBonnet => "Pantone 17-3936 TCX Blue Bonnet",
            Self::Iolite => "Pantone 17-3940 TCX Iolite",
            Self::Titanium => "Pantone 17-4014 TCX Titanium",
            Self::Infinity => "Pantone 17-4015 TCX Infinity",
            Self::GrayFlannel => "Pantone 17-4016 TCX Gray Flannel",
            Self::BlueShadow => "Pantone 17-4020 TCX Blue Shadow",
            Self::FadedDenim => "Pantone 17-4021 TCX Faded Denim",
            Self::BlueHeaven => "Pantone 17-4023 TCX Blue Heaven",
            Self::Riviera => "Pantone 17-4027 TCX Riviera",
            Self::Riverside => "Pantone 17-4028 TCX Riverside",
            Self::QuietHarbor => "Pantone 17-4029 TCX Quiet Harbor",
            Self::SilverLakeBlue => "Pantone 17-4030 TCX Silver Lake Blue",
            Self::LichenBlue => "Pantone 17-4032 TCX Lichen Blue",
            Self::PacificCoast => "Pantone 17-4033 TCX Pacific Coast",
            Self::Ultramarine => "Pantone 17-4037 TCX Ultramarine",
            Self::Marina => "Pantone 17-4041 TCX Marina",
            Self::Citadel => "Pantone 17-4111 TCX Citadel",
            Self::Niagara => "Pantone 17-4123 TCX Niagara",
            Self::CendreBlue => "Pantone 17-4131 TCX Cendre Blue",
            Self::AzureBlue => "Pantone 17-4139 TCX Azure Blue",
            Self::IbizaBlue => "Pantone 17-4245 TCX Ibiza Blue",
            Self::DivaBlue => "Pantone 17-4247 TCX Diva Blue",
            Self::AdriaticBlue => "Pantone 17-4320 TCX Adriatic Blue",
            Self::BlueMoon => "Pantone 17-4328 TCX Blue Moon",
            Self::Blithe => "Pantone 17-4336 TCX Blithe",
            Self::NeutralGray => "Pantone 17-4402 TCX Neutral Gray",
            Self::Monument => "Pantone 17-4405 TCX Monument",
            Self::Lead => "Pantone 17-4408 TCX Lead",
            Self::SmokeBlue => "Pantone 17-4412 TCX Smoke Blue",
            Self::Larkspur => "Pantone 17-4421 TCX Larkspur",
            Self::Bluejay => "Pantone 17-4427 TCX Bluejay",
            Self::NavagioBay => "Pantone 17-4429 TCX Navagio Bay",
            Self::VividBlue => "Pantone 17-4432 TCX Vivid Blue",
            Self::DresdenBlue => "Pantone 17-4433 TCX Dresden Blue",
            Self::MalibuBlue => "Pantone 17-4435 TCX Malibu Blue",
            Self::BlueDanube => "Pantone 17-4440 TCX Blue Danube",
            Self::BarrierReef => "Pantone 17-4530 TCX Barrier Reef",
            Self::HawaiianOcean => "Pantone 17-4540 TCX Hawaiian Ocean",
            Self::StormBlue => "Pantone 17-4716 TCX Storm Blue",
            Self::PagodaBlue => "Pantone 17-4724 TCX Pagoda Blue",
            Self::AlgiersBlue => "Pantone 17-4728 TCX Algiers Blue",
            Self::CaneelBay => "Pantone 17-4730 TCX Caneel Bay",
            Self::CapriBreeze => "Pantone 17-4735 TCX Capri Breeze",
            Self::BristolBlue => "Pantone 17-4818 TCX Bristol Blue",
            Self::Arctic => "Pantone 17-4911 TCX Arctic",
            Self::Teal => "Pantone 17-4919 TCX Teal",
            Self::LakeBlue => "Pantone 17-4928 TCX Lake Blue",
            Self::TealBlue => "Pantone 17-5024 TCX Teal Blue",
            Self::Navigate => "Pantone 17-5025 TCX Navigate",
            Self::DeepPeacockBlue => "Pantone 17-5029 TCX Deep Peacock Blue",
            Self::Lapis => "Pantone 17-5034 TCX Lapis",
            Self::Griffin => "Pantone 17-5102 TCX Griffin",
            Self::ChinoisGreen => "Pantone 17-5107 TCX Chinois Green",
            Self::Trellis => "Pantone 17-5110 TCX Trellis",
            Self::OilBlue => "Pantone 17-5111 TCX Oil Blue",
            Self::GreenBlueSlate => "Pantone 17-5117 TCX Green Blue Slate",
            Self::LatigoBay => "Pantone 17-5122 TCX Latigo Bay",
            Self::ViridianGreen => "Pantone 17-5126 TCX Viridian Green",
            Self::Columbia => "Pantone 17-5130 TCX Columbia",
            Self::DynastyGreen => "Pantone 17-5330 TCX Dynasty Green",
            Self::SpectraGreen => "Pantone 17-5335 TCX Spectra Green",
            Self::PorcelainGreen => "Pantone 17-5421 TCX Porcelain Green",
            Self::Alhambra => "Pantone 17-5430 TCX Alhambra",
            Self::DeepSea => "Pantone 17-5513 TCX Deep Sea",
            Self::Greenlake => "Pantone 17-5528 TCX Greenlake",
            Self::DeepGreen => "Pantone 17-5633 TCX Deep Green",
            Self::VividGreen => "Pantone 17-5638 TCX Vivid Green",
            Self::Emerald => "Pantone 17-5641 TCX Emerald",
            Self::BottleGreen => "Pantone 17-5722 TCX Bottle Green",
            Self::Viridis => "Pantone 17-5734 TCX Viridis",
            Self::Parakeet => "Pantone 17-5735 TCX Parakeet",
            Self::DarkIvy => "Pantone 17-5912 TCX Dark Ivy",
            Self::PineGreen => "Pantone 17-5923 TCX Pine Green",
            Self::SimplyGreen => "Pantone 17-5936 TCX Simply Green",
            Self::DeepMint => "Pantone 17-5937 TCX Deep Mint",
            Self::LaurelWreath => "Pantone 17-6009 TCX Laurel Wreath",
            Self::JellyBean => "Pantone 17-6030 TCX Jelly Bean",
            Self::FernGreen => "Pantone 17-6153 TCX Fern Green",
            Self::Shadow => "Pantone 17-6206 TCX Shadow",
            Self::SeaSpray => "Pantone 17-6212 TCX Sea Spray",
            Self::DeepGrassGreen => "Pantone 17-6219 TCX Deep Grass Green",
            Self::MediumGreen => "Pantone 17-6229 TCX Medium Green",
            Self::Kashmir => "Pantone 17-6319 TCX Kashmir",
            Self::HedgeGreen => "Pantone 17-6323 TCX Hedge Green",
            Self::MintGreen => "Pantone 17-6333 TCX Mint Green",
            Self::SmokedPearl => "Pantone 18-0000 TCX Smoked Pearl",
            Self::Kale => "Pantone 18-0107 TCX Kale",
            Self::Dill => "Pantone 18-0108 TCX Dill",
            Self::EnglishIvy => "Pantone 18-0110 TCX English Ivy",
            Self::VineyardGreen => "Pantone 18-0117 TCX Vineyard Green",
            Self::WillowBough => "Pantone 18-0119 TCX Willow Bough",
            Self::ElmGreen => "Pantone 18-0121 TCX Elm Green",
            Self::ArtichokeGreen => "Pantone 18-0125 TCX Artichoke Green",
            Self::Cactus => "Pantone 18-0130 TCX Cactus",
            Self::Treetop => "Pantone 18-0135 TCX Treetop",
            Self::Castlerock => "Pantone 18-0201 TCX Castlerock",
            Self::Mayfly => "Pantone 18-0220 TCX Mayfly",
            Self::Pesto => "Pantone 18-0228 TCX Pesto",
            Self::Gunmetal => "Pantone 18-0306 TCX Gunmetal",
            Self::DeepLichenGreen => "Pantone 18-0312 TCX Deep Lichen Green",
            Self::Olivine => "Pantone 18-0316 TCX Olivine",
            Self::BronzeGreen => "Pantone 18-0317 TCX Bronze Green",
            Self::Cypress => "Pantone 18-0322 TCX Cypress",
            Self::CallisteGreen => "Pantone 18-0324 TCX Calliste Green",
            Self::CedarGreen => "Pantone 18-0328 TCX Cedar Green",
            Self::TwistOfLime => "Pantone 18-0330 TCX Twist Of Lime",
            Self::Grasshopper => "Pantone 18-0332 TCX Grasshopper",
            Self::DarkGullGray => "Pantone 18-0403 TCX Dark Gull Gray",
            Self::FourLeafClover => "Pantone 18-0420 TCX Four Leaf Clover",
            Self::LodenGreen => "Pantone 18-0422 TCX Loden Green",
            Self::CapuletOlive => "Pantone 18-0426 TCX Capulet Olive",
            Self::Avocado => "Pantone 18-0430 TCX Avocado",
            Self::CallaGreen => "Pantone 18-0435 TCX Calla Green",
            Self::Gargoyle => "Pantone 18-0503 TCX Gargoyle",
            Self::CastorGray => "Pantone 18-0510 TCX Castor Gray",
            Self::BungeeCord => "Pantone 18-0513 TCX Bungee Cord",
            Self::DustyOlive => "Pantone 18-0515 TCX Dusty Olive",
            Self::BurntOlive => "Pantone 18-0521 TCX Burnt Olive",
            Self::WinterMoss => "Pantone 18-0523 TCX Winter Moss",
            Self::Iguana => "Pantone 18-0525 TCX Iguana",
            Self::OliveBranch => "Pantone 18-0527 TCX Olive Branch",
            Self::Woodbine => "Pantone 18-0538 TCX Woodbine",
            Self::CharcoalGray => "Pantone 18-0601 TCX Charcoal Gray",
            Self::StoneGray => "Pantone 18-0615 TCX Stone Gray",
            Self::CovertGreen => "Pantone 18-0617 TCX Covert Green",
            Self::OliveDrab => "Pantone 18-0622 TCX Olive Drab",
            Self::MartiniOlive => "Pantone 18-0625 TCX Martini Olive",
            Self::FirGreen => "Pantone 18-0627 TCX Fir Green",
            Self::Lizard => "Pantone 18-0629 TCX Lizard",
            Self::GothicOlive => "Pantone 18-0724 TCX Gothic Olive",
            Self::Capers => "Pantone 18-0820 TCX Capers",
            Self::Nutria => "Pantone 18-0825 TCX Nutria",
            Self::Butternut => "Pantone 18-0830 TCX Butternut",
            Self::Plantation => "Pantone 18-0832 TCX Plantation",
            Self::DriedTobacco => "Pantone 18-0835 TCX Dried Tobacco",
            Self::Tapenade => "Pantone 18-0840 TCX Tapenade",
            Self::Kangaroo => "Pantone 18-0920 TCX Kangaroo",
            Self::Sepia => "Pantone 18-0928 TCX Sepia",
            Self::CoffeeLiqueur => "Pantone 18-0930 TCX Coffee Liqueur",
            Self::Rubber => "Pantone 18-0933 TCX Rubber",
            Self::BuckthornBrown => "Pantone 18-0935 TCX Buckthorn Brown",
            Self::BronzeBrown => "Pantone 18-0937 TCX Bronze Brown",
            Self::Cumin => "Pantone 18-0939 TCX Cumin",
            Self::GoldenBrown => "Pantone 18-0940 TCX Golden Brown",
            Self::CathaySpice => "Pantone 18-0950 TCX Cathay Spice",
            Self::Shitake => "Pantone 18-1015 TCX Shitake",
            Self::Cub => "Pantone 18-1016 TCX Cub",
            Self::Caribou => "Pantone 18-1017 TCX Caribou",
            Self::Otter => "Pantone 18-1018 TCX Otter",
            Self::Ermine => "Pantone 18-1022 TCX Ermine",
            Self::Bison => "Pantone 18-1027 TCX Bison",
            Self::Emperador => "Pantone 18-1028 TCX Emperador",
            Self::ToastedCoconut => "Pantone 18-1029 TCX Toasted Coconut",
            Self::Thrush => "Pantone 18-1030 TCX Thrush",
            Self::Toffee => "Pantone 18-1031 TCX Toffee",
            Self::Dachshund => "Pantone 18-1033 TCX Dachshund",
            Self::MonksRobe => "Pantone 18-1048 TCX Monks Robe",
            Self::ThaiCurry => "Pantone 18-1049 TCX Thai Curry",
            Self::HoneyGinger => "Pantone 18-1050 TCX Honey Ginger",
            Self::FallenRock => "Pantone 18-1108 TCX Fallen Rock",
            Self::Brindle => "Pantone 18-1110 TCX Brindle",
            Self::Walnut => "Pantone 18-1112 TCX Walnut",
            Self::Partridge => "Pantone 18-1124 TCX Partridge",
            Self::Aztec => "Pantone 18-1130 TCX Aztec",
            Self::Rawhide => "Pantone 18-1137 TCX Rawhide",
            Self::MochaBisque => "Pantone 18-1140 TCX Mocha Bisque",
            Self::LeatherBrown => "Pantone 18-1142 TCX Leather Brown",
            Self::CaramelCafe => "Pantone 18-1148 TCX Caramel Cafe",
            Self::GlazedGinger => "Pantone 18-1154 TCX Glazed Ginger",
            Self::SugarAlmond => "Pantone 18-1155 TCX Sugar Almond",
            Self::SudanBrown => "Pantone 18-1160 TCX Sudan Brown",
            Self::PumpkinSpice => "Pantone 18-1163 TCX Pumpkin Spice",
            Self::Driftwood => "Pantone 18-1210 TCX Driftwood",
            Self::CocoaBrown => "Pantone 18-1222 TCX Cocoa Brown",
            Self::CarobBrown => "Pantone 18-1229 TCX Carob Brown",
            Self::CoconutShell => "Pantone 18-1230 TCX Coconut Shell",
            Self::Russet => "Pantone 18-1235 TCX Russet",
            Self::RusticBrown => "Pantone 18-1238 TCX Rustic Brown",
            Self::Sierra => "Pantone 18-1239 TCX Sierra",
            Self::BrownPatina => "Pantone 18-1242 TCX Brown Patina",
            Self::GingerBread => "Pantone 18-1244 TCX Ginger Bread",
            Self::Umber => "Pantone 18-1246 TCX Umber",
            Self::Rust => "Pantone 18-1248 TCX Rust",
            Self::HawaiianSunset => "Pantone 18-1249 TCX Hawaiian Sunset",
            Self::BombayBrown => "Pantone 18-1250 TCX Bombay Brown",
            Self::Falcon => "Pantone 18-1304 TCX Falcon",
            Self::Iron => "Pantone 18-1306 TCX Iron",
            Self::DeepTaupe => "Pantone 18-1312 TCX Deep Taupe",
            Self::Acorn => "Pantone 18-1314 TCX Acorn",
            Self::Clove => "Pantone 18-1320 TCX Clove",
            Self::Brownie => "Pantone 18-1321 TCX Brownie",
            Self::SpicedApple => "Pantone 18-1325 TCX Spiced Apple",
            Self::Nutmeg => "Pantone 18-1326 TCX Nutmeg",
            Self::CopperBrown => "Pantone 18-1336 TCX Copper Brown",
            Self::PottersClay => "Pantone 18-1340 TCX Potters Clay",
            Self::Auburn => "Pantone 18-1343 TCX Auburn",
            Self::CinnamonStick => "Pantone 18-1345 TCX Cinnamon Stick",
            Self::Bruschetta => "Pantone 18-1346 TCX Bruschetta",
            Self::BurntBrick => "Pantone 18-1350 TCX Burnt Brick",
            Self::BurntOchre => "Pantone 18-1354 TCX Burnt Ochre",
            Self::RooibosTea => "Pantone 18-1355 TCX Rooibos Tea",
            Self::Sparrow => "Pantone 18-1404 TCX Sparrow",
            Self::Flint => "Pantone 18-1405 TCX Flint",
            Self::Peppercorn => "Pantone 18-1409 TCX Peppercorn",
            Self::PlumWine => "Pantone 18-1411 TCX Plum Wine",
            Self::Marron => "Pantone 18-1415 TCX Marron",
            Self::CrushedBerry => "Pantone 18-1418 TCX Crushed Berry",
            Self::WildGinger => "Pantone 18-1420 TCX Wild Ginger",
            Self::Cognac => "Pantone 18-1421 TCX Cognac",
            Self::Mahogany => "Pantone 18-1425 TCX Mahogany",
            Self::AppleButter => "Pantone 18-1426 TCX Apple Butter",
            Self::Chutney => "Pantone 18-1433 TCX Chutney",
            Self::EtruscanRed => "Pantone 18-1434 TCX Etruscan Red",
            Self::WitheredRose => "Pantone 18-1435 TCX Withered Rose",
            Self::LightMahogany => "Pantone 18-1436 TCX Light Mahogany",
            Self::Marsala => "Pantone 18-1438 TCX Marsala",
            Self::ChiliOil => "Pantone 18-1440 TCX Chili Oil",
            Self::BakedClay => "Pantone 18-1441 TCX Baked Clay",
            Self::RedOchre => "Pantone 18-1442 TCX Red Ochre",
            Self::Redwood => "Pantone 18-1443 TCX Redwood",
            Self::TandoriSpice => "Pantone 18-1444 TCX Tandori Spice",
            Self::SpicyOrange => "Pantone 18-1445 TCX Spicy Orange",
            Self::OrangeRust => "Pantone 18-1447 TCX Orange Rust",
            Self::Chili => "Pantone 18-1448 TCX Chili",
            Self::Ketchup => "Pantone 18-1449 TCX Ketchup",
            Self::MeccaOrange => "Pantone 18-1450 TCX Mecca Orange",
            Self::AutumnGlaze => "Pantone 18-1451 TCX Autumn Glaze",
            Self::RedClay => "Pantone 18-1454 TCX Red Clay",
            Self::PlumTruffle => "Pantone 18-1506 TCX Plum Truffle",
            Self::RoseBrown => "Pantone 18-1512 TCX Rose Brown",
            Self::BarnRed => "Pantone 18-1531 TCX Barn Red",
            Self::GingerSpice => "Pantone 18-1535 TCX Ginger Spice",
            Self::HotSauce => "Pantone 18-1536 TCX Hot Sauce",
            Self::CopperCoin => "Pantone 18-1537 TCX Copper Coin",
            Self::Cinnabar => "Pantone 18-1540 TCX Cinnabar",
            Self::BrandyBrown => "Pantone 18-1541 TCX Brandy Brown",
            Self::BossaNova => "Pantone 18-1547 TCX Bossa Nova",
            Self::ValiantPoppy => "Pantone 18-1549 TCX Valiant Poppy",
            Self::AuroraRed => "Pantone 18-1550 TCX Aurora Red",
            Self::AuraOrange => "Pantone 18-1551 TCX Aura Orange",
            Self::MoltenLava => "Pantone 18-1555 TCX Molten Lava",
            Self::OrangeCom => "Pantone 18-1561 TCX Orange Com",
            Self::Poinciana => "Pantone 18-1564 TCX Poinciana",
            Self::RoseTaupe => "Pantone 18-1612 TCX Rose Taupe",
            Self::RenaissanceRose => "Pantone 18-1613 TCX Renaissance Rose",
            Self::Nocturne => "Pantone 18-1614 TCX Nocturne",
            Self::RoanRouge => "Pantone 18-1616 TCX Roan Rouge",
            Self::Maroon => "Pantone 18-1619 TCX Maroon",
            Self::FadedRose => "Pantone 18-1629 TCX Faded Rose",
            Self::DustyCedar => "Pantone 18-1630 TCX Dusty Cedar",
            Self::EarthRed => "Pantone 18-1631 TCX Earth Red",
            Self::GarnetRose => "Pantone 18-1633 TCX Garnet Rose",
            Self::BaroqueRose => "Pantone 18-1634 TCX Baroque Rose",
            Self::SlateRose => "Pantone 18-1635 TCX Slate Rose",
            Self::Cardinal => "Pantone 18-1643 TCX Cardinal",
            Self::BakedApple => "Pantone 18-1648 TCX Baked Apple",
            Self::DeepSeaCoral => "Pantone 18-1649 TCX Deep Sea Coral",
            Self::Cayenne => "Pantone 18-1651 TCX Cayenne",
            Self::RococcoRed => "Pantone 18-1652 TCX Rococco Red",
            Self::Toreador => "Pantone 18-1653 TCX Toreador",
            Self::Lychee => "Pantone 18-1654 TCX Lychee",
            Self::MarsRed => "Pantone 18-1655 TCX Mars Red",
            Self::Salsa => "Pantone 18-1657 TCX Salsa",
            Self::PompeianRed => "Pantone 18-1658 TCX Pompeian Red",
            Self::GojiBerry => "Pantone 18-1659 TCX Goji Berry",
            Self::Tomato => "Pantone 18-1660 TCX Tomato",
            Self::TomatoPuree => "Pantone 18-1661 TCX Tomato Puree",
            Self::FlameScarlet => "Pantone 18-1662 TCX Flame Scarlet",
            Self::ChineseRed => "Pantone 18-1663 TCX Chinese Red",
            Self::FieryRed => "Pantone 18-1664 TCX Fiery Red",
            Self::Shark => "Pantone 18-1703 TCX Shark",
            Self::ArcticDusk => "Pantone 18-1705 TCX Arctic Dusk",
            Self::BlackPlum => "Pantone 18-1706 TCX Black Plum",
            Self::Ephemera => "Pantone 18-1708 TCX Ephemera",
            Self::Tulipwood => "Pantone 18-1709 TCX Tulipwood",
            Self::GrapeNectar => "Pantone 18-1710 TCX Grape Nectar",
            Self::Damson => "Pantone 18-1716 TCX Damson",
            Self::HawthornRose => "Pantone 18-1718 TCX Hawthorn Rose",
            Self::VioletQuartz => "Pantone 18-1720 TCX Violet Quartz",
            Self::DryRose => "Pantone 18-1725 TCX Dry Rose",
            Self::RaspberryWine => "Pantone 18-1741 TCX Raspberry Wine",
            Self::Raspberry => "Pantone 18-1754 TCX Raspberry",
            Self::RougeRed => "Pantone 18-1755 TCX Rouge Red",
            Self::Teaberry => "Pantone 18-1756 TCX Teaberry",
            Self::JalapenoRed => "Pantone 18-1759 TCX Jalapeno Red",
            Self::Barberry => "Pantone 18-1760 TCX Barberry",
            Self::SkiPatrol => "Pantone 18-1761 TCX Ski Patrol",
            Self::Hibiscus => "Pantone 18-1762 TCX Hibiscus",
            Self::HighRiskRed => "Pantone 18-1763 TCX High Risk Red",
            Self::Lollipop => "Pantone 18-1764 TCX Lollipop",
            Self::TwilightMauve => "Pantone 18-1807 TCX Twilight Mauve",
            Self::RoseRed => "Pantone 18-1852 TCX Rose Red",
            Self::VirtualPink => "Pantone 18-1856 TCX Virtual Pink",
            Self::BrightRose => "Pantone 18-1945 TCX Bright Rose",
            Self::Jazzy => "Pantone 18-1950 TCX Jazzy",
            Self::LovePotion => "Pantone 18-1951 TCX Love Potion",
            Self::Beaujolais => "Pantone 18-2027 TCX Beaujolais",
            Self::RaspberrySorbet => "Pantone 18-2043 TCX Raspberry Sorbet",
            Self::PinkPeacock => "Pantone 18-2045 TCX Pink Peacock",
            Self::GrapeShake => "Pantone 18-2109 TCX Grape Shake",
            Self::Honeysuckle => "Pantone 18-2120 TCX Honeysuckle",
            Self::PinkFlambe => "Pantone 18-2133 TCX Pink Flambe",
            Self::Cabaret => "Pantone 18-2140 TCX Cabaret",
            Self::BeetrootPurple => "Pantone 18-2143 TCX Beetroot Purple",
            Self::Clover => "Pantone 18-2320 TCX Clover",
            Self::CactusFlower => "Pantone 18-2326 TCX Cactus Flower",
            Self::FuchsiaRed => "Pantone 18-2328 TCX Fuchsia Red",
            Self::RaspberryRose => "Pantone 18-2333 TCX Raspberry Rose",
            Self::VeryBerry => "Pantone 18-2336 TCX Very Berry",
            Self::FuchsiaPurple => "Pantone 18-2436 TCX Fuchsia Purple",
            Self::MagentaHaze => "Pantone 18-2525 TCX Magenta Haze",
            Self::BatonRouge => "Pantone 18-2527 TCX Baton Rouge",
            Self::PurpleWine => "Pantone 18-2929 TCX Purple Wine",
            Self::ArgylePurple => "Pantone 18-3011 TCX Argyle Purple",
            Self::PurpleGumdrop => "Pantone 18-3012 TCX Purple Gumdrop",
            Self::BerryConserve => "Pantone 18-3013 TCX Berry Conserve",
            Self::GrapeKiss => "Pantone 18-3014 TCX Grape Kiss",
            Self::Amethyst => "Pantone 18-3015 TCX Amethyst",
            Self::DeepOrchid => "Pantone 18-3022 TCX Deep Orchid",
            Self::StrikingPurple => "Pantone 18-3025 TCX Striking Purple",
            Self::PurpleOrchid => "Pantone 18-3027 TCX Purple Orchid",
            Self::Willowherb => "Pantone 18-3120 TCX Willowherb",
            Self::Grapeade => "Pantone 18-3211 TCX Grapeade",
            Self::ConcordGrape => "Pantone 18-3218 TCX Concord Grape",
            Self::VeryGrape => "Pantone 18-3220 TCX Very Grape",
            Self::RadiantOrchid => "Pantone 18-3224 TCX Radiant Orchid",
            Self::MeadowMauve => "Pantone 18-3230 TCX Meadow Mauve",
            Self::Dahlia => "Pantone 18-3324 TCX Dahlia",
            Self::HyacinthViolet => "Pantone 18-3331 TCX Hyacinth Violet",
            Self::VividViola => "Pantone 18-3339 TCX Vivid Viola",
            Self::Charisma => "Pantone 18-3340 TCX Charisma",
            Self::VintageViolet => "Pantone 18-3410 TCX Vintage Violet",
            Self::GrapeJam => "Pantone 18-3415 TCX Grape Jam",
            Self::ChineseViolet => "Pantone 18-3418 TCX Chinese Violet",
            Self::GrapeCompote => "Pantone 18-3513 TCX Grape Compote",
            Self::PatricianPurple => "Pantone 18-3518 TCX Patrician Purple",
            Self::PurpleHeart => "Pantone 18-3520 TCX Purple Heart",
            Self::PlumJam => "Pantone 18-3521 TCX Plum Jam",
            Self::CrushedGrape => "Pantone 18-3522 TCX Crushed Grape",
            Self::LavenderCrystal => "Pantone 18-3530 TCX Lavender Crystal",
            Self::RoyalLilac => "Pantone 18-3531 TCX Royal Lilac",
            Self::Dewberry => "Pantone 18-3533 TCX Dewberry",
            Self::PurpleSapphire => "Pantone 18-3540 TCX Purple Sapphire",
            Self::ImperialPalace => "Pantone 18-3615 TCX Imperial Palace",
            Self::Mystical => "Pantone 18-3620 TCX Mystical",
            Self::Bellflower => "Pantone 18-3628 TCX Bellflower",
            Self::DeepLavender => "Pantone 18-3633 TCX Deep Lavender",
            Self::ChiveBlossom => "Pantone 18-3634 TCX Chive Blossom",
            Self::PicassoLily => "Pantone 18-3635 TCX Picasso Lily",
            Self::GrayRidge => "Pantone 18-3710 TCX Gray Ridge",
            Self::PurpleSage => "Pantone 18-3712 TCX Purple Sage",
            Self::MulledGrape => "Pantone 18-3714 TCX Mulled Grape",
            Self::MontanaGrape => "Pantone 18-3715 TCX Montana Grape",
            Self::PurpleHaze => "Pantone 18-3718 TCX Purple Haze",
            Self::PassionFlower => "Pantone 18-3737 TCX Passion Flower",
            Self::Cadet => "Pantone 18-3812 TCX Cadet",
            Self::Heron => "Pantone 18-3817 TCX Heron",
            Self::TwilightPurple => "Pantone 18-3820 TCX Twilight Purple",
            Self::CorsicanBlue => "Pantone 18-3828 TCX Corsican Blue",
            Self::DustedPeri => "Pantone 18-3833 TCX Dusted Peri",
            Self::Veronica => "Pantone 18-3834 TCX Veronica",
            Self::UltraViolet => "Pantone 18-3838 TCX Ultra Violet",
            Self::PurpleCorallite => "Pantone 18-3839 TCX Purple Corallite",
            Self::PurpleOpulence => "Pantone 18-3840 TCX Purple Opulence",
            Self::Excalibur => "Pantone 18-3905 TCX Excalibur",
            Self::Tornado => "Pantone 18-3907 TCX Tornado",
            Self::VolcanicGlass => "Pantone 18-3908 TCX Volcanic Glass",
            Self::FolkstoneGray => "Pantone 18-3910 TCX Folkstone Gray",
            Self::Grisaille => "Pantone 18-3912 TCX Grisaille",
            Self::FlintStone => "Pantone 18-3916 TCX Flint Stone",
            Self::GrayBlue => "Pantone 18-3917 TCX Gray Blue",
            Self::ChinaBlue => "Pantone 18-3918 TCX China Blue",
            Self::CoastalFjord => "Pantone 18-3920 TCX Coastal Fjord",
            Self::BijouBlue => "Pantone 18-3921 TCX Bijou Blue",
            Self::CoronetBlue => "Pantone 18-3922 TCX Coronet Blue",
            Self::VelvetMorning => "Pantone 18-3927 TCX Velvet Morning",
            Self::DutchBlue => "Pantone 18-3928 TCX Dutch Blue",
            Self::BlueHorizon => "Pantone 18-3929 TCX Blue Horizon",
            Self::BleachedDenim => "Pantone 18-3930 TCX Bleached Denim",
            Self::Marlin => "Pantone 18-3932 TCX Marlin",
            Self::BlueGranite => "Pantone 18-3933 TCX Blue Granite",
            Self::Wedgewood => "Pantone 18-3935 TCX Wedgewood",
            Self::BlueYonder => "Pantone 18-3937 TCX Blue Yonder",
            Self::BlueIris => "Pantone 18-3943 TCX Blue Iris",
            Self::VioletStorm => "Pantone 18-3944 TCX Violet Storm",
            Self::AmparoBlue => "Pantone 18-3945 TCX Amparo Blue",
            Self::BajaBlue => "Pantone 18-3946 TCX Baja Blue",
            Self::DazzlingBlue => "Pantone 18-3949 TCX Dazzling Blue",
            Self::IrisBloom => "Pantone 18-3950 TCX Iris Bloom",
            Self::SpectrumBlue => "Pantone 18-3963 TCX Spectrum Blue",
            Self::SteelGray => "Pantone 18-4005 TCX Steel Gray",
            Self::QuietShade => "Pantone 18-4006 TCX Quiet Shade",
            Self::GoblinBlue => "Pantone 18-4011 TCX Goblin Blue",
            Self::DecemberSky => "Pantone 18-4016 TCX December Sky",
            Self::RealTeal => "Pantone 18-4018 TCX Real Teal",
            Self::CaptainsBlue => "Pantone 18-4020 TCX Captains Blue",
            Self::BlueAshes => "Pantone 18-4023 TCX Blue Ashes",
            Self::CopenBlue => "Pantone 18-4025 TCX Copen Blue",
            Self::Stellar => "Pantone 18-4026 TCX Stellar",
            Self::MoonlightBlue => "Pantone 18-4027 TCX Moonlight Blue",
            Self::BeringSea => "Pantone 18-4028 TCX Bering Sea",
            Self::FederalBlue => "Pantone 18-4029 TCX Federal Blue",
            Self::DeepWater => "Pantone 18-4032 TCX Deep Water",
            Self::VallartaBlue => "Pantone 18-4034 TCX Vallarta Blue",
            Self::ParisianBlue => "Pantone 18-4036 TCX Parisian Blue",
            Self::Regatta => "Pantone 18-4039 TCX Regatta",
            Self::StarSapphire => "Pantone 18-4041 TCX Star Sapphire",
            Self::PalaceBlue => "Pantone 18-4043 TCX Palace Blue",
            Self::Daphne => "Pantone 18-4045 TCX Daphne",
            Self::NebulasBlue => "Pantone 18-4048 TCX Nebulas Blue",
            Self::StrongBlue => "Pantone 18-4051 TCX Strong Blue",
            Self::MoonMist => "Pantone 18-4105 TCX Moon Mist",
            Self::FrenchBlue => "Pantone 18-4140 TCX French Blue",
            Self::Campanula => "Pantone 18-4141 TCX Campanula",
            Self::VictoriaBlue => "Pantone 18-4148 TCX Victoria Blue",
            Self::StormyWeather => "Pantone 18-4214 TCX Stormy Weather",
            Self::BlueMirage => "Pantone 18-4215 TCX Blue Mirage",
            Self::Bluestone => "Pantone 18-4217 TCX Bluestone",
            Self::ProvincialBlue => "Pantone 18-4220 TCX Provincial Blue",
            Self::Bluesteel => "Pantone 18-4222 TCX Bluesteel",
            Self::SaxonyBlue => "Pantone 18-4225 TCX Saxony Blue",
            Self::BlueSapphire => "Pantone 18-4231 TCX Blue Sapphire",
            Self::Faience => "Pantone 18-4232 TCX Faience",
            Self::DirectoireBlue => "Pantone 18-4244 TCX Directoire Blue",
            Self::BrilliantBlue => "Pantone 18-4247 TCX Brilliant Blue",
            Self::IndigoBunting => "Pantone 18-4250 TCX Indigo Bunting",
            Self::BlueAster => "Pantone 18-4252 TCX Blue Aster",
            Self::AegeanBlue => "Pantone 18-4320 TCX Aegean Blue",
            Self::SwedishBlue => "Pantone 18-4330 TCX Swedish Blue",
            Self::MediterraneanBlue => "Pantone 18-4334 TCX Mediterranean Blue",
            Self::Tapestry => "Pantone 18-4417 TCX Tapestry",
            Self::FjordBlue => "Pantone 18-4430 TCX Fjord Blue",
            Self::TurkishTile => "Pantone 18-4432 TCX Turkish Tile",
            Self::MykonosBlue => "Pantone 18-4434 TCX Mykonos Blue",
            Self::Cloisonne => "Pantone 18-4440 TCX Cloisonne",
            Self::Trooper => "Pantone 18-4510 TCX Trooper",
            Self::ColonialBlue => "Pantone 18-4522 TCX Colonial Blue",
            Self::CaribbeanSea => "Pantone 18-4525 TCX Caribbean Sea",
            Self::MosaicBlue => "Pantone 18-4528 TCX Mosaic Blue",
            Self::Celestial => "Pantone 18-4530 TCX Celestial",
            Self::BlueJewel => "Pantone 18-4535 TCX Blue Jewel",
            Self::MethylBlue => "Pantone 18-4537 TCX Methyl Blue",
            Self::HawaiianSurf => "Pantone 18-4538 TCX Hawaiian Surf",
            Self::NorthAtlantic => "Pantone 18-4612 TCX North Atlantic",
            Self::TahitianTide => "Pantone 18-4630 TCX Tahitian Tide",
            Self::StormySea => "Pantone 18-4711 TCX Stormy Sea",
            Self::Hydro => "Pantone 18-4718 TCX Hydro",
            Self::BiscayBay => "Pantone 18-4726 TCX Biscay Bay",
            Self::HarborBlue => "Pantone 18-4728 TCX Harbor Blue",
            Self::EnamelBlue => "Pantone 18-4733 TCX Enamel Blue",
            Self::TileBlue => "Pantone 18-4735 TCX Tile Blue",
            Self::DeepLake => "Pantone 18-4834 TCX Deep Lake",
            Self::TropicalGreen => "Pantone 18-4930 TCX Tropical Green",
            Self::Fanfare => "Pantone 18-4936 TCX Fanfare",
            Self::Parasailing => "Pantone 18-5020 TCX Parasailing",
            Self::QuetzalGreen => "Pantone 18-5025 TCX Quetzal Green",
            Self::BrushedNickel => "Pantone 18-5102 TCX Brushed Nickel",
            Self::SedonaSage => "Pantone 18-5105 TCX Sedona Sage",
            Self::SeaPine => "Pantone 18-5112 TCX Sea Pine",
            Self::NorthSea => "Pantone 18-5115 TCX North Sea",
            Self::Bayou => "Pantone 18-5121 TCX Bayou",
            Self::BlueGrass => "Pantone 18-5128 TCX Blue Grass",
            Self::Pewter => "Pantone 18-5203 TCX Pewter",
            Self::GraniteGray => "Pantone 18-5204 TCX Granite Gray",
            Self::EiffelTower => "Pantone 18-5210 TCX Eiffel Tower",
            Self::BlueSpruce => "Pantone 18-5308 TCX Blue Spruce",
            Self::Bayberry => "Pantone 18-5315 TCX Bayberry",
            Self::AlpineGreen => "Pantone 18-5322 TCX Alpine Green",
            Self::UltramarineGreen => "Pantone 18-5338 TCX Ultramarine Green",
            Self::SilverPine => "Pantone 18-5410 TCX Silver Pine",
            Self::AntiqueGreen => "Pantone 18-5418 TCX Antique Green",
            Self::CadmiumGreen => "Pantone 18-5424 TCX Cadmium Green",
            Self::BalsamGreen => "Pantone 18-5606 TCX Balsam Green",
            Self::BrittanyBlue => "Pantone 18-5610 TCX Brittany Blue",
            Self::DarkForest => "Pantone 18-5611 TCX Dark Forest",
            Self::SagebrushGreen => "Pantone 18-5612 TCX Sagebrush Green",
            Self::PosyGreen => "Pantone 18-5616 TCX Posy Green",
            Self::DeepJungle => "Pantone 18-5618 TCX Deep Jungle",
            Self::Tidepool => "Pantone 18-5619 TCX Tidepool",
            Self::Ivy => "Pantone 18-5620 TCX Ivy",
            Self::Fir => "Pantone 18-5621 TCX Fir",
            Self::FrostySpruce => "Pantone 18-5622 TCX Frosty Spruce",
            Self::ShadyGlade => "Pantone 18-5624 TCX Shady Glade",
            Self::Bosphorus => "Pantone 18-5633 TCX Bosphorus",
            Self::GolfGreen => "Pantone 18-5642 TCX Golf Green",
            Self::SmokePine => "Pantone 18-5718 TCX Smoke Pine",
            Self::GalapagosGreen => "Pantone 18-5725 TCX Galapagos Green",
            Self::AgaveGreen => "Pantone 18-5806 TCX Agave Green",
            Self::PepperGreen => "Pantone 18-5841 TCX Pepper Green",
            Self::LushMeadow => "Pantone 18-5845 TCX Lush Meadow",
            Self::GardenTopiary => "Pantone 18-5913 TCX Garden Topiary",
            Self::DuckGreen => "Pantone 18-6011 TCX Duck Green",
            Self::FoliageGreen => "Pantone 18-6018 TCX Foliage Green",
            Self::Leprechaun => "Pantone 18-6022 TCX Leprechaun",
            Self::Amazon => "Pantone 18-6024 TCX Amazon",
            Self::JollyGreen => "Pantone 18-6030 TCX Jolly Green",
            Self::Myrtle => "Pantone 18-6114 TCX Myrtle",
            Self::Comfrey => "Pantone 18-6216 TCX Comfrey",
            Self::Fairway => "Pantone 18-6320 TCX Fairway",
            Self::Juniper => "Pantone 18-6330 TCX Juniper",
            Self::Raven => "Pantone 19-0000 TCX Raven",
            Self::Asphalt => "Pantone 19-0201 TCX Asphalt",
            Self::GrayPinstripe => "Pantone 19-0203 TCX Gray Pinstripe",
            Self::GardenGreen => "Pantone 19-0230 TCX Garden Green",
            Self::JetBlack => "Pantone 19-0303 TCX Jet Black",
            Self::ClimbingIvy => "Pantone 19-0307 TCX Climbing Ivy",
            Self::Thyme => "Pantone 19-0309 TCX Thyme",
            Self::Beetle => "Pantone 19-0312 TCX Beetle",
            Self::BlackForest => "Pantone 19-0315 TCX Black Forest",
            Self::Chive => "Pantone 19-0323 TCX Chive",
            Self::SeaTurtle => "Pantone 19-0403 TCX Sea Turtle",
            Self::Beluga => "Pantone 19-0405 TCX Beluga",
            Self::DeepDepths => "Pantone 19-0413 TCX Deep Depths",
            Self::ForestNight => "Pantone 19-0414 TCX Forest Night",
            Self::DuffelBag => "Pantone 19-0415 TCX Duffel Bag",
            Self::KombuGreen => "Pantone 19-0417 TCX Kombu Green",
            Self::RifleGreen => "Pantone 19-0419 TCX Rifle Green",
            Self::BlackInk => "Pantone 19-0506 TCX Black Ink",
            Self::Peat => "Pantone 19-0508 TCX Peat",
            Self::Rosin => "Pantone 19-0509 TCX Rosin",
            Self::Kalamata => "Pantone 19-0510 TCX Kalamata",
            Self::GrapeLeaf => "Pantone 19-0511 TCX Grape Leaf",
            Self::IvyGreen => "Pantone 19-0512 TCX Ivy Green",
            Self::OliveNight => "Pantone 19-0515 TCX Olive Night",
            Self::DarkOlive => "Pantone 19-0516 TCX Dark Olive",
            Self::BlackOlive => "Pantone 19-0608 TCX Black Olive",
            Self::Wren => "Pantone 19-0614 TCX Wren",
            Self::Teak => "Pantone 19-0617 TCX Teak",
            Self::Beech => "Pantone 19-0618 TCX Beech",
            Self::MilitaryOlive => "Pantone 19-0622 TCX Military Olive",
            Self::Demitasse => "Pantone 19-0712 TCX Demitasse",
            Self::Morel => "Pantone 19-0808 TCX Morel",
            Self::ChocolateChip => "Pantone 19-0809 TCX Chocolate Chip",
            Self::MajorBrown => "Pantone 19-0810 TCX Major Brown",
            Self::TurkishCoffee => "Pantone 19-0812 TCX Turkish Coffee",
            Self::SlateBlack => "Pantone 19-0814 TCX Slate Black",
            Self::DesertPalm => "Pantone 19-0815 TCX Desert Palm",
            Self::Canteen => "Pantone 19-0820 TCX Canteen",
            Self::Tarmac => "Pantone 19-0822 TCX Tarmac",
            Self::Crocodile => "Pantone 19-0823 TCX Crocodile",
            Self::Delicioso => "Pantone 19-0840 TCX Delicioso",
            Self::Mulch => "Pantone 19-0910 TCX Mulch",
            Self::ChocolateBrown => "Pantone 19-0912 TCX Chocolate Brown",
            Self::CoffeeBean => "Pantone 19-0915 TCX Coffee Bean",
            Self::RainDrum => "Pantone 19-0916 TCX Rain Drum",
            Self::FrenchRoast => "Pantone 19-1012 TCX French Roast",
            Self::Bracken => "Pantone 19-1015 TCX Bracken",
            Self::Java => "Pantone 19-1016 TCX Java",
            Self::Ganache => "Pantone 19-1018 TCX Ganache",
            Self::DarkEarth => "Pantone 19-1020 TCX Dark Earth",
            Self::Breen => "Pantone 19-1034 TCX Breen",
            Self::AfterDark => "Pantone 19-1101 TCX After Dark",
            Self::Licorice => "Pantone 19-1102 TCX Licorice",
            Self::Espresso => "Pantone 19-1103 TCX Espresso",
            Self::Mole => "Pantone 19-1106 TCX Mole",
            Self::ChocolateTorte => "Pantone 19-1109 TCX Chocolate Torte",
            Self::ChocolatePlum => "Pantone 19-1110 TCX Chocolate Plum",
            Self::BlackCoffee => "Pantone 19-1111 TCX Black Coffee",
            Self::Carafe => "Pantone 19-1116 TCX Carafe",
            Self::Chestnut => "Pantone 19-1118 TCX Chestnut",
            Self::Pinecone => "Pantone 19-1121 TCX Pinecone",
            Self::ShoppingBag => "Pantone 19-1213 TCX Shopping Bag",
            Self::ChocolateLab => "Pantone 19-1214 TCX Chocolate Lab",
            Self::ShavedChocolate => "Pantone 19-1215 TCX Shaved Chocolate",
            Self::Mustang => "Pantone 19-1217 TCX Mustang",
            Self::PottingSoil => "Pantone 19-1218 TCX Potting Soil",
            Self::Cappuccino => "Pantone 19-1220 TCX Cappuccino",
            Self::FondueFudge => "Pantone 19-1224 TCX Fondue Fudge",
            Self::RootBeer => "Pantone 19-1228 TCX Root Beer",
            Self::FriarBrown => "Pantone 19-1230 TCX Friar Brown",
            Self::Tiramisu => "Pantone 19-1233 TCX Tiramisu",
            Self::RockyRoad => "Pantone 19-1234 TCX Rocky Road",
            Self::Brunette => "Pantone 19-1235 TCX Brunette",
            Self::TortoiseShell => "Pantone 19-1241 TCX Tortoise Shell",
            Self::ArabianSpice => "Pantone 19-1245 TCX Arabian Spice",
            Self::Picante => "Pantone 19-1250 TCX Picante",
            Self::SealBrown => "Pantone 19-1314 TCX Seal Brown",
            Self::BitterChocolate => "Pantone 19-1317 TCX Bitter Chocolate",
            Self::Sable => "Pantone 19-1320 TCX Sable",
            Self::RumRaisin => "Pantone 19-1321 TCX Rum Raisin",
            Self::BrownStone => "Pantone 19-1322 TCX Brown Stone",
            Self::HotChocolate => "Pantone 19-1325 TCX Hot Chocolate",
            Self::Andorra => "Pantone 19-1327 TCX Andorra",
            Self::MadderBrown => "Pantone 19-1331 TCX Madder Brown",
            Self::Sequoia => "Pantone 19-1333 TCX Sequoia",
            Self::Henna => "Pantone 19-1334 TCX Henna",
            Self::FiredBrick => "Pantone 19-1337 TCX Fired Brick",
            Self::RussetBrown => "Pantone 19-1338 TCX Russet Brown",
            Self::ChicoryCoffee => "Pantone 19-1419 TCX Chicory Coffee",
            Self::DeepMahogany => "Pantone 19-1420 TCX Deep Mahogany",
            Self::SmokedPaprika => "Pantone 19-1429 TCX Smoked Paprika",
            Self::Mink => "Pantone 19-1430 TCX Mink",
            Self::Fudgesickle => "Pantone 19-1431 TCX Fudgesickle",
            Self::ChocolateFondant => "Pantone 19-1432 TCX Chocolate Fondant",
            Self::CherryMahogany => "Pantone 19-1435 TCX Cherry Mahogany",
            Self::Cinnamon => "Pantone 19-1436 TCX Cinnamon",
            Self::Puce => "Pantone 19-1518 TCX Puce",
            Self::RedMahogany => "Pantone 19-1521 TCX Red Mahogany",
            Self::Zinfandel => "Pantone 19-1522 TCX Zinfandel",
            Self::OxbloodRed => "Pantone 19-1524 TCX Oxblood Red",
            Self::Port => "Pantone 19-1525 TCX Port",
            Self::ChocolateTruffle => "Pantone 19-1526 TCX Chocolate Truffle",
            Self::WindsorWine => "Pantone 19-1528 TCX Windsor Wine",
            Self::BurntRusset => "Pantone 19-1530 TCX Burnt Russet",
            Self::SunDriedTomato => "Pantone 19-1531 TCX Sun Dried Tomato",
            Self::Rosewood => "Pantone 19-1532 TCX Rosewood",
            Self::Cowhide => "Pantone 19-1533 TCX Cowhide",
            Self::Merlot => "Pantone 19-1534 TCX Merlot",
            Self::Syrah => "Pantone 19-1535 TCX Syrah",
            Self::RedPear => "Pantone 19-1536 TCX Red Pear",
            Self::BurntHenna => "Pantone 19-1540 TCX Burnt Henna",
            Self::BrickRed => "Pantone 19-1543 TCX Brick Red",
            Self::RedDahlia => "Pantone 19-1555 TCX Red Dahlia",
            Self::ChiliPepper => "Pantone 19-1557 TCX Chili Pepper",
            Self::ScarletSage => "Pantone 19-1559 TCX Scarlet Sage",
            Self::Raisin => "Pantone 19-1606 TCX Raisin",
            Self::PrunePurple => "Pantone 19-1608 TCX Prune Purple",
            Self::Burgundy => "Pantone 19-1617 TCX Burgundy",
            Self::Fudge => "Pantone 19-1619 TCX Fudge",
            Self::Huckleberry => "Pantone 19-1620 TCX Huckleberry",
            Self::CatawbaGrape => "Pantone 19-1621 TCX Catawba Grape",
            Self::VineyardWine => "Pantone 19-1623 TCX Vineyard Wine",
            Self::Sassafras => "Pantone 19-1624 TCX Sassafras",
            Self::DecadentChocolate => "Pantone 19-1625 TCX Decadent Chocolate",
            Self::PortRoyale => "Pantone 19-1627 TCX Port Royale",
            Self::RubyWine => "Pantone 19-1629 TCX Ruby Wine",
            Self::BikingRed => "Pantone 19-1650 TCX Biking Red",
            Self::Rhubarb => "Pantone 19-1652 TCX Rhubarb",
            Self::Garnet => "Pantone 19-1655 TCX Garnet",
            Self::RioRed => "Pantone 19-1656 TCX Rio Red",
            Self::Samba => "Pantone 19-1662 TCX Samba",
            Self::RibbonRed => "Pantone 19-1663 TCX Ribbon Red",
            Self::TrueRed => "Pantone 19-1664 TCX True Red",
            Self::MauveWine => "Pantone 19-1716 TCX Mauve Wine",
            Self::Fig => "Pantone 19-1718 TCX Fig",
            Self::Cabernet => "Pantone 19-1724 TCX Cabernet",
            Self::TawnyPort => "Pantone 19-1725 TCX Tawny Port",
            Self::Cordovan => "Pantone 19-1726 TCX Cordovan",
            Self::BarbadosCherry => "Pantone 19-1757 TCX Barbados Cherry",
            Self::HauteRed => "Pantone 19-1758 TCX Haute Red",
            Self::AmericanBeauty => "Pantone 19-1759 TCX American Beauty",
            Self::Scarlet => "Pantone 19-1760 TCX Scarlet",
            Self::TangoRed => "Pantone 19-1761 TCX Tango Red",
            Self::Crimson => "Pantone 19-1762 TCX Crimson",
            Self::RacingRed => "Pantone 19-1763 TCX Racing Red",
            Self::LipstickRed => "Pantone 19-1764 TCX Lipstick Red",
            Self::DeepClaret => "Pantone 19-1840 TCX Deep Claret",
            Self::RedBud => "Pantone 19-1850 TCX Red Bud",
            Self::PersianRed => "Pantone 19-1860 TCX Persian Red",
            Self::JesterRed => "Pantone 19-1862 TCX Jester Red",
            Self::Scooter => "Pantone 19-1863 TCX Scooter",
            Self::Pomegranate => "Pantone 19-1930 TCX Pomegranate",
            Self::TibetanRed => "Pantone 19-1934 TCX Tibetan Red",
            Self::RumbaRed => "Pantone 19-1940 TCX Rumba Red",
            Self::Cerise => "Pantone 19-1955 TCX Cerise",
            Self::Hortensia => "Pantone 19-2009 TCX Hortensia",
            Self::Prune => "Pantone 19-2014 TCX Prune",
            Self::Rhododendron => "Pantone 19-2024 TCX Rhododendron",
            Self::RedPlum => "Pantone 19-2025 TCX Red Plum",
            Self::BeetRed => "Pantone 19-2030 TCX Beet Red",
            Self::Anemone => "Pantone 19-2033 TCX Anemone",
            Self::Granita => "Pantone 19-2039 TCX Granita",
            Self::CherriesJubilee => "Pantone 19-2041 TCX Cherries Jubilee",
            Self::Vivacious => "Pantone 19-2045 TCX Vivacious",
            Self::Sangria => "Pantone 19-2047 TCX Sangria",
            Self::Winetasting => "Pantone 19-2118 TCX Winetasting",
            Self::Eggplant => "Pantone 19-2311 TCX Eggplant",
            Self::CrushedViolets => "Pantone 19-2312 TCX Crushed Violets",
            Self::GrapeWine => "Pantone 19-2315 TCX Grape Wine",
            Self::Amaranth => "Pantone 19-2410 TCX Amaranth",
            Self::PickledBeet => "Pantone 19-2420 TCX Pickled Beet",
            Self::MagentaPurple => "Pantone 19-2428 TCX Magenta Purple",
            Self::PlumCaspia => "Pantone 19-2429 TCX Plum Caspia",
            Self::PurplePotion => "Pantone 19-2430 TCX Purple Potion",
            Self::Boysenberry => "Pantone 19-2431 TCX Boysenberry",
            Self::RaspberryRadiance => "Pantone 19-2432 TCX Raspberry Radiance",
            Self::FestivalFuchsia => "Pantone 19-2434 TCX Festival Fuchsia",
            Self::ItalianPlum => "Pantone 19-2514 TCX Italian Plum",
            Self::PotentPurple => "Pantone 19-2520 TCX Potent Purple",
            Self::DarkPurple => "Pantone 19-2524 TCX Dark Purple",
            Self::WinterBloom => "Pantone 19-2620 TCX Winter Bloom",
            Self::WildAster => "Pantone 19-2630 TCX Wild Aster",
            Self::Wineberry => "Pantone 19-2814 TCX Wineberry",
            Self::BlackberryWine => "Pantone 19-2816 TCX Blackberry Wine",
            Self::Phlox => "Pantone 19-2820 TCX Phlox",
            Self::Hollyhock => "Pantone 19-2924 TCX Hollyhock",
            Self::Gloxinia => "Pantone 19-3022 TCX Gloxinia",
            Self::Byzantium => "Pantone 19-3138 TCX Byzantium",
            Self::Indigo => "Pantone 19-3215 TCX Indigo",
            Self::ShadowPurple => "Pantone 19-3217 TCX Shadow Purple",
            Self::PlumPurple => "Pantone 19-3218 TCX Plum Purple",
            Self::Plum => "Pantone 19-3220 TCX Plum",
            Self::PurplePassion => "Pantone 19-3223 TCX Purple Passion",
            Self::GrapeJuice => "Pantone 19-3230 TCX Grape Juice",
            Self::PlumPerfect => "Pantone 19-3316 TCX Plum Perfect",
            Self::DeepPurple => "Pantone 19-3323 TCX Deep Purple",
            Self::WoodViolet => "Pantone 19-3325 TCX Wood Violet",
            Self::SparklingGrape => "Pantone 19-3336 TCX Sparkling Grape",
            Self::SunsetPurple => "Pantone 19-3424 TCX Sunset Purple",
            Self::SpicedPlum => "Pantone 19-3425 TCX Spiced Plum",
            Self::BrightViolet => "Pantone 19-3438 TCX Bright Violet",
            Self::Majesty => "Pantone 19-3514 TCX Majesty",
            Self::GrapeRoyale => "Pantone 19-3518 TCX Grape Royale",
            Self::PurplePennant => "Pantone 19-3519 TCX Purple Pennant",
            Self::BlackberryCordial => "Pantone 19-3520 TCX Blackberry Cordial",
            Self::MeadowViolet => "Pantone 19-3526 TCX Meadow Violet",
            Self::ImperialPurple => "Pantone 19-3528 TCX Imperial Purple",
            Self::AmaranthPurple => "Pantone 19-3536 TCX Amaranth Purple",
            Self::PurpleMagic => "Pantone 19-3540 TCX Purple Magic",
            Self::Pansy => "Pantone 19-3542 TCX Pansy",
            Self::Mysterioso => "Pantone 19-3617 TCX Mysterioso",
            Self::SweetGrape => "Pantone 19-3619 TCX Sweet Grape",
            Self::PurpleReign => "Pantone 19-3620 TCX Purple Reign",
            Self::Loganberry => "Pantone 19-3622 TCX Loganberry",
            Self::Acai => "Pantone 19-3628 TCX Acai",
            Self::Petunia => "Pantone 19-3632 TCX Petunia",
            Self::TillandsiaPurple => "Pantone 19-3638 TCX Tillandsia Purple",
            Self::CrownJewel => "Pantone 19-3640 TCX Crown Jewel",
            Self::RoyalPurple => "Pantone 19-3642 TCX Royal Purple",
            Self::Nightshade => "Pantone 19-3712 TCX Nightshade",
            Self::DeepWell => "Pantone 19-3713 TCX Deep Well",
            Self::NavyCosmos => "Pantone 19-3714 TCX Navy Cosmos",
            Self::PurplePlumeria => "Pantone 19-3716 TCX Purple Plumeria",
            Self::GothicGrape => "Pantone 19-3720 TCX Gothic Grape",
            Self::MulberryPurple => "Pantone 19-3722 TCX Mulberry Purple",
            Self::PurpleVelvet => "Pantone 19-3725 TCX Purple Velvet",
            Self::Grape => "Pantone 19-3728 TCX Grape",
            Self::GentianViolet => "Pantone 19-3730 TCX Gentian Violet",
            Self::ParachutePurple => "Pantone 19-3731 TCX Parachute Purple",
            Self::Heliotrope => "Pantone 19-3737 TCX Heliotrope",
            Self::PrismViolet => "Pantone 19-3748 TCX Prism Violet",
            Self::VioletIndigo => "Pantone 19-3750 TCX Violet Indigo",
            Self::PlumKitten => "Pantone 19-3803 TCX Plum Kitten",
            Self::Eclipse => "Pantone 19-3810 TCX Eclipse",
            Self::EveningBlue => "Pantone 19-3815 TCX Evening Blue",
            Self::AstralAura => "Pantone 19-3830 TCX Astral Aura",
            Self::MaritimeBlue => "Pantone 19-3831 TCX Maritime Blue",
            Self::NavyBlue => "Pantone 19-3832 TCX Navy Blue",
            Self::BlueRibbon => "Pantone 19-3839 TCX Blue Ribbon",
            Self::DeepWisteria => "Pantone 19-3842 TCX Deep Wisteria",
            Self::DeepBlue => "Pantone 19-3847 TCX Deep Blue",
            Self::Liberty => "Pantone 19-3850 TCX Liberty",
            Self::MazarineBlue => "Pantone 19-3864 TCX Mazarine Blue",
            Self::Pavement => "Pantone 19-3900 TCX Pavement",
            Self::Magnet => "Pantone 19-3901 TCX Magnet",
            Self::Obsidian => "Pantone 19-3902 TCX Obsidian",
            Self::Shale => "Pantone 19-3903 TCX Shale",
            Self::Rabbit => "Pantone 19-3905 TCX Rabbit",
            Self::DarkShadow => "Pantone 19-3906 TCX Dark Shadow",
            Self::ForgedIron => "Pantone 19-3907 TCX Forged Iron",
            Self::NineIron => "Pantone 19-3908 TCX Nine Iron",
            Self::BlackBean => "Pantone 19-3909 TCX Black Bean",
            Self::IronGate => "Pantone 19-3910 TCX Iron Gate",
            Self::BlackBeauty => "Pantone 19-3911 TCX Black Beauty",
            Self::Graystone => "Pantone 19-3915 TCX Graystone",
            Self::BlackenedPearl => "Pantone 19-3917 TCX Blackened Pearl",
            Self::Periscope => "Pantone 19-3918 TCX Periscope",
            Self::NightshadowBlue => "Pantone 19-3919 TCX Nightshadow Blue",
            Self::Peacoat => "Pantone 19-3920 TCX Peacoat",
            Self::BlackIris => "Pantone 19-3921 TCX Black Iris",
            Self::SkyCaptain => "Pantone 19-3922 TCX Sky Captain",
            Self::NavyBlazer => "Pantone 19-3923 TCX Navy Blazer",
            Self::NightSky => "Pantone 19-3924 TCX Night Sky",
            Self::PatriotBlue => "Pantone 19-3925 TCX Patriot Blue",
            Self::CrownBlue => "Pantone 19-3926 TCX Crown Blue",
            Self::Graphite => "Pantone 19-3927 TCX Graphite",
            Self::BlueIndigo => "Pantone 19-3928 TCX Blue Indigo",
            Self::VintageIndigo => "Pantone 19-3929 TCX Vintage Indigo",
            Self::OdysseyGray => "Pantone 19-3930 TCX Odyssey Gray",
            Self::MedievalBlue => "Pantone 19-3933 TCX Medieval Blue",
            Self::DeepCobalt => "Pantone 19-3935 TCX Deep Cobalt",
            Self::SkipperBlue => "Pantone 19-3936 TCX Skipper Blue",
            Self::TwilightBlue => "Pantone 19-3938 TCX Twilight Blue",
            Self::Blueprint => "Pantone 19-3939 TCX Blueprint",
            Self::BlueDepths => "Pantone 19-3940 TCX Blue Depths",
            Self::OrientBlue => "Pantone 19-3947 TCX Orient Blue",
            Self::DeepUltramarine => "Pantone 19-3950 TCX Deep Ultramarine",
            Self::ClematisBlue => "Pantone 19-3951 TCX Clematis Blue",
            Self::SurfTheWeb => "Pantone 19-3952 TCX Surf The Web",
            Self::SodaliteBlue => "Pantone 19-3953 TCX Sodalite Blue",
            Self::RoyalBlue => "Pantone 19-3955 TCX Royal Blue",
            Self::MonacoBlue => "Pantone 19-3964 TCX Monaco Blue",
            Self::BlackOnyx => "Pantone 19-4003 TCX Black Onyx",
            Self::TapShoe => "Pantone 19-4004 TCX Tap Shoe",
            Self::StretchLimo => "Pantone 19-4005 TCX Stretch Limo",
            Self::Caviar => "Pantone 19-4006 TCX Caviar",
            Self::Anthracite => "Pantone 19-4007 TCX Anthracite",
            Self::Meteorite => "Pantone 19-4008 TCX Meteorite",
            Self::OuterSpace => "Pantone 19-4009 TCX Outer Space",
            Self::TotalEclipse => "Pantone 19-4010 TCX Total Eclipse",
            Self::Salute => "Pantone 19-4011 TCX Salute",
            Self::Carbon => "Pantone 19-4012 TCX Carbon",
            Self::DarkNavy => "Pantone 19-4013 TCX Dark Navy",
            Self::OmbreBlue => "Pantone 19-4014 TCX Ombre Blue",
            Self::BlueGraphite => "Pantone 19-4015 TCX Blue Graphite",
            Self::IndiaInk => "Pantone 19-4019 TCX India Ink",
            Self::DarkSapphire => "Pantone 19-4020 TCX Dark Sapphire",
            Self::Blueberry => "Pantone 19-4021 TCX Blueberry",
            Self::ParisianNight => "Pantone 19-4022 TCX Parisian Night",
            Self::BlueNights => "Pantone 19-4023 TCX Blue Nights",
            Self::DressBlues => "Pantone 19-4024 TCX Dress Blues",
            Self::MoodIndigo => "Pantone 19-4025 TCX Mood Indigo",
            Self::EnsignBlue => "Pantone 19-4026 TCX Ensign Blue",
            Self::EstateBlue => "Pantone 19-4027 TCX Estate Blue",
            Self::InsigniaBlue => "Pantone 19-4028 TCX Insignia Blue",
            Self::NavyPeony => "Pantone 19-4029 TCX Navy Peony",
            Self::TrueNavy => "Pantone 19-4030 TCX True Navy",
            Self::SargassoSea => "Pantone 19-4031 TCX Sargasso Sea",
            Self::Poseidon => "Pantone 19-4033 TCX Poseidon",
            Self::SailorBlue => "Pantone 19-4034 TCX Sailor Blue",
            Self::DarkBlue => "Pantone 19-4035 TCX Dark Blue",
            Self::BrightCobalt => "Pantone 19-4037 TCX Bright Cobalt",
            Self::GibraltarSea => "Pantone 19-4038 TCX Gibraltar Sea",
            Self::Delft => "Pantone 19-4039 TCX Delft",
            Self::Limoges => "Pantone 19-4044 TCX Limoges",
            Self::LapisBlue => "Pantone 19-4045 TCX Lapis Blue",
            Self::BaleineBlue => "Pantone 19-4048 TCX Baleine Blue",
            Self::SnorkelBlue => "Pantone 19-4049 TCX Snorkel Blue",
            Self::NauticalBlue => "Pantone 19-4050 TCX Nautical Blue",
            Self::ClassicBlue => "Pantone 19-4052 TCX Classic Blue",
            Self::TurkishSea => "Pantone 19-4053 TCX Turkish Sea",
            Self::GalaxyBlue => "Pantone 19-4055 TCX Galaxy Blue",
            Self::OlympianBlue => "Pantone 19-4056 TCX Olympian Blue",
            Self::TrueBlue => "Pantone 19-4057 TCX True Blue",
            Self::Ebony => "Pantone 19-4104 TCX Ebony",
            Self::MidnightNavy => "Pantone 19-4110 TCX Midnight Navy",
            Self::DarkDenim => "Pantone 19-4118 TCX Dark Denim",
            Self::BlueOpal => "Pantone 19-4120 TCX Blue Opal",
            Self::BlueWingTeal => "Pantone 19-4121 TCX Blue Wing Teal",
            Self::MoonlitOcean => "Pantone 19-4122 TCX Moonlit Ocean",
            Self::MajolicaBlue => "Pantone 19-4125 TCX Majolica Blue",
            Self::DeepDive => "Pantone 19-4126 TCX Deep Dive",
            Self::Midnight => "Pantone 19-4127 TCX Midnight",
            Self::PrincessBlue => "Pantone 19-4150 TCX Princess Blue",
            Self::Skydiver => "Pantone 19-4151 TCX Skydiver",
            Self::MoonlessNight => "Pantone 19-4203 TCX Moonless Night",
            Self::Phantom => "Pantone 19-4205 TCX Phantom",
            Self::Turbulence => "Pantone 19-4215 TCX Turbulence",
            Self::Vulcan => "Pantone 19-4218 TCX Vulcan",
            Self::DarkSlate => "Pantone 19-4220 TCX Dark Slate",
            Self::IndianTeal => "Pantone 19-4227 TCX Indian Teal",
            Self::OrionBlue => "Pantone 19-4229 TCX Orion Blue",
            Self::InkBlue => "Pantone 19-4234 TCX Ink Blue",
            Self::MoroccanBlue => "Pantone 19-4241 TCX Moroccan Blue",
            Self::ImperialBlue => "Pantone 19-4245 TCX Imperial Blue",
            Self::PirateBlack => "Pantone 19-4305 TCX Pirate Black",
            Self::Stargazer => "Pantone 19-4316 TCX Stargazer",
            Self::MallardBlue => "Pantone 19-4318 TCX Mallard Blue",
            Self::LegionBlue => "Pantone 19-4324 TCX Legion Blue",
            Self::ReflectingPond => "Pantone 19-4326 TCX Reflecting Pond",
            Self::Corsair => "Pantone 19-4329 TCX Corsair",
            Self::LyonsBlue => "Pantone 19-4340 TCX Lyons Blue",
            Self::Seaport => "Pantone 19-4342 TCX Seaport",
            Self::Mediterranea => "Pantone 19-4517 TCX Mediterranea",
            Self::ShadedSpruce => "Pantone 19-4524 TCX Shaded Spruce",
            Self::BlueCoral => "Pantone 19-4526 TCX Blue Coral",
            Self::OceanDepths => "Pantone 19-4535 TCX Ocean Depths",
            Self::CrystalTeal => "Pantone 19-4536 TCX Crystal Teal",
            Self::DeepLagoon => "Pantone 19-4540 TCX Deep Lagoon",
            Self::AtlanticDeep => "Pantone 19-4726 TCX Atlantic Deep",
            Self::MallardGreen => "Pantone 19-4818 TCX Mallard Green",
            Self::Balsam => "Pantone 19-4820 TCX Balsam",
            Self::Dragonfly => "Pantone 19-4826 TCX Dragonfly",
            Self::GreenGables => "Pantone 19-4906 TCX Green Gables",
            Self::DeepTeal => "Pantone 19-4914 TCX Deep Teal",
            Self::Pacific => "Pantone 19-4916 TCX Pacific",
            Self::TealGreen => "Pantone 19-4922 TCX Teal Green",
            Self::UrbanChic => "Pantone 19-5004 TCX Urban Chic",
            Self::SeaMoss => "Pantone 19-5030 TCX Sea Moss",
            Self::DarkestSpruce => "Pantone 19-5212 TCX Darkest Spruce",
            Self::Storm => "Pantone 19-5217 TCX Storm",
            Self::BotanicalGarden => "Pantone 19-5220 TCX Botanical Garden",
            Self::Everglade => "Pantone 19-5226 TCX Everglade",
            Self::ForestBiome => "Pantone 19-5230 TCX Forest Biome",
            Self::RainForest => "Pantone 19-5232 TCX Rain Forest",
            Self::PonderosaPine => "Pantone 19-5320 TCX Ponderosa Pine",
            Self::Scarab => "Pantone 19-5350 TCX Scarab",
            Self::PineGrove => "Pantone 19-5406 TCX Pine Grove",
            Self::BistroGreen => "Pantone 19-5408 TCX Bistro Green",
            Self::TrekkingGreen => "Pantone 19-5411 TCX Trekking Green",
            Self::Jasper => "Pantone 19-5413 TCX Jasper",
            Self::JuneBug => "Pantone 19-5414 TCX June Bug",
            Self::Evergreen => "Pantone 19-5420 TCX Evergreen",
            Self::HunterGreen => "Pantone 19-5511 TCX Hunter Green",
            Self::DarkGreen => "Pantone 19-5513 TCX Dark Green",
            Self::Cilantro => "Pantone 19-5621 TCX Cilantro",
            Self::JetSet => "Pantone 19-5708 TCX Jet Set",
            Self::JungleGreen => "Pantone 19-5914 TCX Jungle Green",
            Self::Sycamore => "Pantone 19-5917 TCX Sycamore",
            Self::MountainView => "Pantone 19-5918 TCX Mountain View",
            Self::Pineneedle => "Pantone 19-5920 TCX Pineneedle",
            Self::VerdantGreen => "Pantone 19-6026 TCX Verdant Green",
            Self::Eden => "Pantone 19-6050 TCX Eden",
            Self::DeepForest => "Pantone 19-6110 TCX Deep Forest",
            Self::GreenerPastures => "Pantone 19-6311 TCX Greener Pastures",
        }
    }

    /// The color as opaque byte components.
    pub const fn to_color(&self) -> Color<u8> {
        Color::from_rgb_u32(self.packed_rgb())
    }

    /// The nearest FHI color to `color` in the sRGB space - squared
    /// Euclidean distance over the RGB channels, alpha ignored.
    pub fn nearest(color: &Color<u8>) -> Self {
        // `ALL` is a non-empty const table, so the minimum always exists.
        Self::ALL
            .iter()
            .min_by_key(|candidate| color.squared_rgb_distance(&candidate.to_color()))
            .copied()
            .unwrap_or(Self::Egret)
    }

    /// The packed `0xRRGGBB` value of the color.
    const fn packed_rgb(&self) -> u32 {
        match self {
            Self::Egret => 0xF3ECE0,
            Self::VanillaIce => 0xF0EADA,
            Self::AntiqueWhite => 0xEDE3D2,
            Self::SweetCream => 0xF0EAD6,
            Self::Papyrus => 0xF5EDD6,
            Self::Buttercream => 0xEFE0CD,
            Self::GlassGreen => 0xECEAD0,
            Self::WaterLily => 0xDDE3D5,
            Self::SylvanGreen => 0xE7EACB,
            Self::WinterWhite => 0xF5ECD2,
            Self::Afterglow => 0xF3E6C9,
            Self::LemonIcing => 0xF6EBC8,
            Self::BrightWhite => 0xF4F5F0,
            Self::SnowWhite => 0xF2F0EB,
            Self::PastelParchment => 0xE5D9D3,
            Self::Gardenia => 0xF1E8DF,
            Self::JetStream => 0xEDE6DE,
            Self::Pristine => 0xF2E8DA,
            Self::SugarSwizzle => 0xF3EEE7,
            Self::CoconutMilk => 0xF0EDE5,
            Self::EtherealGreen => 0xF1ECCA,
            Self::PearSorbet => 0xF3EAC3,
            Self::PastelYellow => 0xF2E6B1,
            Self::TransparentYellow => 0xF4ECC2,
            Self::WaxYellow => 0xEDE9AD,
            Self::Flan => 0xF6E3B4,
            Self::ElfinYellow => 0xEEEA97,
            Self::YellowIris => 0xEEE78E,
            Self::YellowPear => 0xECE99B,
            Self::WhisperWhite => 0xEDE6DB,
            Self::TenderYellow => 0xEDEDB7,
            Self::Ecru => 0xF3DFCA,
            Self::PearledIvory => 0xF0DFCC,
            Self::WhiteAlyssum => 0xEFEBE7,
            Self::BridalBlush => 0xEEE2DD,
            Self::SeaSalt => 0xF1E6DE,
            Self::AngelWing => 0xF3DFD7,
            Self::CreamPink => 0xF6E4D9,
            Self::PowderPuff => 0xF3E0D6,
            Self::Rosewater => 0xF6DBD8,
            Self::PetalPink => 0xF2E2E0,
            Self::Delicacy => 0xF5E3E2,
            Self::ShrinkingViolet => 0xF4E1E6,
            Self::BrilliantWhite => 0xEDF1FE,
            Self::CloudDancer => 0xF0EEE9,
            Self::StarWhite => 0xEFEFE8,
            Self::Marshmallow => 0xF0EEE4,
            Self::LilyWhite => 0xE2E2DA,
            Self::CannoliCream => 0xF0EFE2,
            Self::MysticBlue => 0xE1E3DE,
            Self::BitOfBlue => 0xE2EAEB,
            Self::BillowingSail => 0xD8E7E7,
            Self::BlancDeBlanc => 0xE7E9E7,
            Self::Tofu => 0xE8E3D9,
            Self::SummerShower => 0xE5EBE3,
            Self::Ice => 0xE0E4D9,
            Self::LightestSky => 0xE4EADF,
            Self::HintOfMint => 0xD8E8E6,
            Self::WhiteSwan => 0xE4D7C5,
            Self::WhiteAsparagus => 0xE1DBC8,
            Self::BoneWhite => 0xD7D0C0,
            Self::MeadowMist => 0xD3DEC4,
            Self::CanaryGreen => 0xD6DEC9,
            Self::Ambrosia => 0xD2E7CA,
            Self::PistachioShell => 0xD7CFBB,
            Self::PatinaGreen => 0xB9EAB3,
            Self::WhitecapGray => 0xE0D5C6,
            Self::AsparagusGreen => 0xD2CDB4,
            Self::LimeCream => 0xD7E8BC,
            Self::SeafoamGreen => 0xCBD5B1,
            Self::WhiteJade => 0xD4DBB2,
            Self::Gleam => 0xBFD1AD,
            Self::Butterfly => 0xCADEA5,
            Self::LightGray => 0xDAD8C9,
            Self::Hay => 0xD3CCA3,
            Self::MellowGreen => 0xD5D593,
            Self::DaiquiriGreen => 0xC9D77E,
            Self::PaleLimeYellow => 0xDFE69F,
            Self::YoungWheat => 0xE1E3A9,
            Self::Citron => 0xDFDE9B,
            Self::LuminaryGreen => 0xE3EAA5,
            Self::Charlock => 0xE5E790,
            Self::Eggnog => 0xECE1D3,
            Self::Angora => 0xDFD1BB,
            Self::GreenEssence => 0xE9EAC8,
            Self::DustyYellow => 0xD4CC9A,
            Self::LemonGrass => 0xDCD494,
            Self::CanaryYellow => 0xDFD87E,
            Self::Aurora => 0xEDDD59,
            Self::BlazingYellow => 0xFEE715,
            Self::Celandine => 0xEBDF67,
            Self::Seedpearl => 0xE6DAC4,
            Self::WhiteSmoke => 0xEDDCC9,
            Self::Macadamia => 0xE4CFB6,
            Self::Navajo => 0xEFDCC3,
            Self::LemonMeringue => 0xF6E199,
            Self::Vanilla => 0xF4E1C1,
            Self::AlmondOil => 0xF4EFC1,
            Self::Cornhusk => 0xF2D6AE,
            Self::DoubleCream => 0xF3E0AC,
            Self::AniseFlower => 0xF4E3B5,
            Self::PineappleSlice => 0xE7D391,
            Self::MellowYellow => 0xF0DD9D,
            Self::Lemonade => 0xF0E79D,
            Self::FrenchVanilla => 0xEFE1A7,
            Self::Sunshine => 0xFADE85,
            Self::Sundress => 0xEBCF89,
            Self::LemonDrop => 0xFDD878,
            Self::Goldfinch => 0xF8DC6C,
            Self::YellowCream => 0xEFDC75,
            Self::Limelight => 0xF0E87D,
            Self::SunnyLime => 0xDFEF87,
            Self::LemonVerbena => 0xF3E779,
            Self::Buttercup => 0xFAE03C,
            Self::Yarrow => 0xFACE6D,
            Self::CloudCream => 0xE6DDC5,
            Self::Rutabaga => 0xECDDBE,
            Self::SunKiss => 0xEBD1BB,
            Self::Dawn => 0xEBD2B7,
            Self::AlabasterGleam => 0xF0DEBD,
            Self::AutumnBlonde => 0xEED0AE,
            Self::VanillaCustard => 0xF3E0BE,
            Self::ApricotGelato => 0xF5D7AF,
            Self::GoldenFleece => 0xF2D1A0,
            Self::PaleBanana => 0xFAE199,
            Self::Popcorn => 0xF8DE8D,
            Self::GoldenHaze => 0xFBD897,
            Self::LambsWool => 0xE5D0B1,
            Self::Nude => 0xF2D3BC,
            Self::TenderPeach => 0xF8D5B8,
            Self::Alesan => 0xF1CEB3,
            Self::PalePeach => 0xFED1BD,
            Self::BleachedApricot => 0xFCCAAC,
            Self::GoldenStraw => 0xE6BD8F,
            Self::NovellePeach => 0xE7CFBD,
            Self::MotherOfPearl => 0xE9D4C3,
            Self::PastelRoseTan => 0xE9D1BF,
            Self::Linen => 0xEDD2C0,
            Self::VanillaCream => 0xF4D8C6,
            Self::ScallopShell => 0xFBD8C9,
            Self::PeachPuree => 0xEFCFBA,
            Self::SheerPink => 0xF6E5DB,
            Self::PeachDust => 0xF0D8CC,
            Self::Dew => 0xEEDED1,
            Self::CremeDePeche => 0xF5D6C6,
            Self::SilverPeony => 0xE7CFC7,
            Self::PearlBlush => 0xF4CEC5,
            Self::SoftPink => 0xF2D8CD,
            Self::VeiledRose => 0xF8CDC9,
            Self::Pearl => 0xF9DBD8,
            Self::HeavenlyPink => 0xF4DEDE,
            Self::BlushingBride => 0xFBD3D9,
            Self::Tapioca => 0xDCCDBC,
            Self::PinkTint => 0xDBCBBD,
            Self::CrystalPink => 0xEDD0CE,
            Self::PinkDogwood => 0xF7D1D1,
            Self::CrystalRose => 0xFDC3C6,
            Self::MarysRose => 0xF7D1D4,
            Self::MauveMorn => 0xECD6D6,
            Self::AlmostMauve => 0xE7DCD9,
            Self::Morganite => 0xDFCDC6,
            Self::MauveChalk => 0xE5D0CF,
            Self::LightLilac => 0xDEC6D3,
            Self::PrimrosePink => 0xEED4D9,
            Self::CradlePink => 0xEDD0DD,
            Self::BarelyPink => 0xF8D7DD,
            Self::RoseWater => 0xF8E0E7,
            Self::AlmondMilk => 0xD6CEBE,
            Self::VaporousGray => 0xDFDDD7,
            Self::Bluewash => 0xE2E6E0,
            Self::SpaBlue => 0xD3DEDF,
            Self::BarelyBlue => 0xDDE0DF,
            Self::Skylight => 0xC8E0E0,
            Self::PastelBlue => 0xBCD3D5,
            Self::Clearwater => 0xAAD5DB,
            Self::StarlightBlue => 0xB5CED4,
            Self::WhisperingBlue => 0xC9DCDC,
            Self::BlueBlush => 0xD6DBD9,
            Self::WanBlue => 0xCBDCDF,
            Self::ChalkBlue => 0xCCDAD7,
            Self::Icicle => 0xDADCD0,
            Self::Turtledove => 0xDED7C8,
            Self::Murmur => 0xD2D8D2,
            Self::MorningMist => 0xCFDFDB,
            Self::BlueGlass => 0xC6E3E1,
            Self::SoothingSea => 0xC3E9E4,
            Self::SproutGreen => 0xCBD7D2,
            Self::BlueFlower => 0xD0D9D4,
            Self::WhisperGreen => 0xE0E6D7,
            Self::OpalBlue => 0xC3DDD6,
            Self::AquaGlass => 0xD2E8E0,
            Self::MoonlightJade => 0xC7E5DF,
            Self::FairAqua => 0xB8E2DC,
            Self::BleachedAqua => 0xBCE3DF,
            Self::ClearlyAqua => 0xCEE1D4,
            Self::Glacier => 0xC3DBD4,
            Self::DustyAqua => 0xC0DCCD,
            Self::Bay => 0xBAE5D6,
            Self::HushedGreen => 0xD8E9E5,
            Self::ZephyrBlue => 0xD3D9D1,
            Self::Honeydew => 0xBAE1D3,
            Self::SilverGreen => 0xD7D7C7,
            Self::MilkyGreen => 0xCFDBD1,
            Self::FairestJade => 0xD8E3D7,
            Self::Frost => 0xDDE2D6,
            Self::PhantomGreen => 0xDCE4D7,
            Self::Moonbeam => 0xCDC6BD,
            Self::WhiteSand => 0xDBD5D1,
            Self::Dewkist => 0xC4D1C2,
            Self::Seacrest => 0xBFD1B3,
            Self::PastelGreen => 0xB4D3B2,
            Self::GreenAsh => 0xA0DAA9,
            Self::FogGreen => 0xC2CBB4,
            Self::TenderGreens => 0xC5CFB6,
            Self::Reed => 0xC3D3A8,
            Self::ParadiseGreen => 0xB2E79F,
            Self::PistachioGreen => 0xA9D39E,
            Self::LilyGreen => 0xC5CF98,
            Self::ShadowLime => 0xCFE09D,
            Self::LettuceGreen => 0xBED38E,
            Self::SapGreen => 0xAFCB80,
            Self::LimaBean => 0xE1D590,
            Self::Oatmeal => 0xCBC3B4,
            Self::GrayMorn => 0xCABEB5,
            Self::GreenGlow => 0xB0C965,
            Self::LimePopsicle => 0xC0DB3A,
            Self::FrozenDew => 0xD8CFB2,
            Self::PaleGreen => 0xCBCE91,
            Self::LimeSherbet => 0xCDD78A,
            Self::CeleryGreen => 0xC5CC7B,
            Self::SharpGreen => 0xC6EC7A,
            Self::WildLime => 0xC3D363,
            Self::LimePunch => 0xC0D725,
            Self::Fog => 0xD0C5B1,
            Self::AloeWash => 0xD0D3B7,
            Self::Moth => 0xD2CBAF,
            Self::ChinoGreen => 0xD9CAA5,
            Self::GardenGlade => 0xDCD8A8,
            Self::GoldenMist => 0xD5CD94,
            Self::Endive => 0xD2CC81,
            Self::Chardonnay => 0xE7DF99,
            Self::Acacia => 0xDACD65,
            Self::GoldenKiwi => 0xF3DD3E,
            Self::Limeade => 0xD3D95F,
            Self::Meadowlark => 0xEAD94E,
            Self::GreenSheen => 0xD9CE52,
            Self::SulphurSpring => 0xD5D717,
            Self::EveningPrimrose => 0xCCDB1E,
            Self::Putty => 0xD4CAB0,
            Self::SeaMist => 0xD8C9A3,
            Self::Custard => 0xE5D68E,
            Self::Raffia => 0xDAC483,
            Self::CreamGold => 0xDEC05F,
            Self::Maize => 0xEEC843,
            Self::Lemon => 0xF3BF08,
            Self::PrimroseYellow => 0xF6D155,
            Self::LemonZest => 0xF9D857,
            Self::Dandelion => 0xFFD02E,
            Self::SolarPower => 0xF4BF3A,
            Self::SummerMelon => 0xEAD3AE,
            Self::BananaCrepe => 0xE7D3AD,
            Self::Sunlight => 0xEDD59E,
            Self::Snapdragon => 0xFED777,
            Self::HabaneroGold => 0xFED450,
            Self::AspenGold => 0xFFD662,
            Self::MinionYellow => 0xFED55D,
            Self::VibrantYellow => 0xFFDA29,
            Self::LemonChrome => 0xFFC300,
            Self::Birch => 0xDDD5C7,
            Self::Sandshell => 0xD8CCBB,
            Self::Parchment => 0xDFD1BE,
            Self::ReedYellow => 0xDCC99E,
            Self::Chamomile => 0xE8D0A7,
            Self::ItalianStraw => 0xE7D1A1,
            Self::Soybean => 0xD2C29D,
            Self::Straw => 0xE0C992,
            Self::Cornsilk => 0xEDC373,
            Self::Flax => 0xFFC87D,
            Self::GoldenCream => 0xF7B768,
            Self::SunsetGold => 0xF7C46C,
            Self::BananaCream => 0xFFCF73,
            Self::AmberYellow => 0xFAB75A,
            Self::PaleMarigold => 0xFFC66E,
            Self::JurassicGold => 0xE7AA56,
            Self::Banana => 0xFCB953,
            Self::CremeBrulee => 0xDBCCB5,
            Self::OysterWhite => 0xD2CAAF,
            Self::BleachedSand => 0xDACCB4,
            Self::Biscotti => 0xDAC7AB,
            Self::GraySand => 0xE5CCAF,
            Self::IvoryCream => 0xDAC0A7,
            Self::FrostedAlmond => 0xD2C2AC,
            Self::Appleblossom => 0xDDBCA0,
            Self::MellowBuff => 0xD8B998,
            Self::HoneyPeach => 0xDCBD9E,
            Self::Wheat => 0xDEC5A5,
            Self::AlmondCream => 0xF4C29F,
            Self::DesertDust => 0xE3BC8E,
            Self::CreamBlush => 0xF8C19A,
            Self::ApricotIce => 0xFBBE99,
            Self::PrairieSunset => 0xFFBB9E,
            Self::CaramelCream => 0xF4BA94,
            Self::PeachFuzz => 0xFFBE98,
            Self::Buff => 0xEBC396,
            Self::Impala => 0xF8CE97,
            Self::Creampuff => 0xFFCDA8,
            Self::ApricotCream => 0xF1BD89,
            Self::Sunburst => 0xF6C289,
            Self::ApricotSherbet => 0xFACD9E,
            Self::BrownRice => 0xC7BBA4,
            Self::SandDollar => 0xDECDBE,
            Self::WhisperPink => 0xDACBBE,
            Self::CreamTan => 0xE4C7B8,
            Self::Bisque => 0xEDCAB5,
            Self::Bellini => 0xF4C9B1,
            Self::PeachQuartz => 0xF5B895,
            Self::PeachyKeen => 0xE2BDB3,
            Self::BrazilianSand => 0xDACAB7,
            Self::EnglishRose => 0xF4C6C3,
            Self::TropicalPeach => 0xFFC4B2,
            Self::PaleDogwood => 0xEDCDC2,
            Self::Shell => 0xE1CFC6,
            Self::CloudPink => 0xF5D1C8,
            Self::CreolePink => 0xF7D5CC,
            Self::ChintzRose => 0xEEC4BE,
            Self::SeashellPink => 0xF7C8C2,
            Self::PeachBlush => 0xE4CCC6,
            Self::ImpatiensPink => 0xFFC4BC,
            Self::PinkSalt => 0xF7CDC7,
            Self::GossamerPink => 0xFAC8C3,
            Self::RoseQuartz => 0xF7CAC9,
            Self::ChalkPink => 0xE6C5CA,
            Self::RoseShadow => 0xF9C2CD,
            Self::Potpourri => 0xE7C9CA,
            Self::StrawberryCream => 0xF4C3C4,
            Self::AlmondBlossom => 0xF5BEC7,
            Self::OrchidPink => 0xF3BBCA,
            Self::FairyTale => 0xF2C1D1,
            Self::PaleLilac => 0xE1C6CC,
            Self::ParfaitPink => 0xE9C3CF,
            Self::PinkMist => 0xE6BCCD,
            Self::PinkLady => 0xEFC1D6,
            Self::Ballerina => 0xF2CFDC,
            Self::BalletSlipper => 0xEBCED5,
            Self::CherryBlossom => 0xF7CEE0,
            Self::LilacSnow => 0xE0C7D7,
            Self::OrchidIce => 0xE0D0DB,
            Self::CrystalGray => 0xD7CBC4,
            Self::OrchidTint => 0xDBD2DB,
            Self::LilacAsh => 0xD7CDCD,
            Self::GrayLilac => 0xD4CACD,
            Self::OrchidHush => 0xCEC3D2,
            Self::LavenderFog => 0xD2C4D6,
            Self::HalogenBlue => 0xBDC6DC,
            Self::IllusionBlue => 0xC9D3DC,
            Self::Antarctica => 0xC6C5C6,
            Self::LilacHint => 0xD0D0DA,
            Self::NimbusCloud => 0xD5D5D8,
            Self::ArcticIce => 0xBFC7D6,
            Self::PleinAir => 0xBFCAD6,
            Self::Omphalodes => 0xB5CEDF,
            Self::OysterMushroom => 0xC3C6C8,
            Self::DawnBlue => 0xCACCCB,
            Self::BalladBlue => 0xC0CEDA,
            Self::FoggyDew => 0xD1D5D0,
            Self::BabyBlue => 0xB5C7D3,
            Self::SilverBirch => 0xD2CFC4,
            Self::IceFlow => 0xC6D2D2,
            Self::MistyBlue => 0xBFCDCC,
            Self::BlueGlow => 0xB2D4DD,
            Self::CrystalBlue => 0xA1C8DB,
            Self::TanagerTurquoise => 0x91DCE8,
            Self::PaleBlue => 0xC4D6D3,
            Self::Plume => 0xA5CFD5,
            Self::LimpetShell => 0x98DDDE,
            Self::BlueLight => 0xACDFDD,
            Self::BlueTint => 0x9FD9D7,
            Self::RainyDay => 0xCFC8BD,
            Self::PaleAqua => 0xC1CCC2,
            Self::IcyMorn => 0xB0D3D1,
            Self::PastelTurquoise => 0x99C5C4,
            Self::ArubaBlue => 0x81D7D3,
            Self::Yucca => 0xA1D7C9,
            Self::IcedAqua => 0xABD3DB,
            Self::BeachGlass => 0x96DFCE,
            Self::IceGreen => 0x87D8C3,
            Self::Cabbage => 0x87D7BE,
            Self::GossamerGreen => 0xB2CFBE,
            Self::BirdsEggGreen => 0xAACCB9,
            Self::AlmostAqua => 0xCAD3C1,
            Self::Spray => 0xBED3BB,
            Self::MistyJade => 0xBCD9C8,
            Self::BrookGreen => 0xAFDDCC,
            Self::CeladonTint => 0xCBCEBE,
            Self::GreenTint => 0xC5CCC0,
            Self::GreenLily => 0xC1CEC1,
            Self::Celadon => 0xB8CCBA,
            Self::MistGreen => 0xAACEBC,
            Self::BokChoy => 0xBCCAB3,
            Self::SilverGray => 0xC1B7B0,
            Self::PumiceStone => 0xCAC2B9,
            Self::Overcast => 0xC3BDAB,
            Self::CastleWall => 0xC8C1AB,
            Self::CeladonGreen => 0xB5C1A5,
            Self::FoamGreen => 0xB4C79C,
            Self::Margarita => 0xB5C38E,
            Self::NileGreen => 0xA7C796,
            Self::ArcadianGreen => 0xA3C893,
            Self::Greengage => 0x8BC28C,
            Self::SummerGreen => 0x7ED37F,
            Self::TidalFoam => 0xBFB9A3,
            Self::Lint => 0xB6BA99,
            Self::Seedling => 0xC0CBA1,
            Self::Nile => 0xB4BB85,
            Self::OpalineGreen => 0xA3C57D,
            Self::JadeLime => 0xA1CA7B,
            Self::BrightLimeGreen => 0x97BC62,
            Self::AcidLime => 0xBADF30,
            Self::Bog => 0xBAB696,
            Self::Beechnut => 0xC2C18D,
            Self::GreenBanana => 0xBABC72,
            Self::BrightChartreuse => 0xB5BF50,
            Self::TenderShoots => 0xB5CC39,
            Self::LimeGreen => 0x9FC131,
            Self::GreenHaze => 0xCAC4A4,
            Self::DriedMoss => 0xCCB97E,
            Self::ShadowGreen => 0xCFC486,
            Self::MutedLime => 0xD1C87C,
            Self::Celery => 0xCEC153,
            Self::Cement => 0xC4B6A6,
            Self::Hemp => 0xC0AD7C,
            Self::Bamboo => 0xD2B04C,
            Self::SuperLemon => 0xE4BF45,
            Self::Sulphur => 0xDDB614,
            Self::EmpireYellow => 0xF7D000,
            Self::CyberYellow => 0xFFD400,
            Self::Pampas => 0xCFBB7B,
            Self::DuskyCitron => 0xE3CC81,
            Self::MistedYellow => 0xDAB965,
            Self::YolkYellow => 0xE2B051,
            Self::BuffYellow => 0xF1BF70,
            Self::Mimosa => 0xF0C05A,
            Self::Daffodil => 0xFDC04E,
            Self::SamoanSun => 0xFBC85F,
            Self::Freesia => 0xF3C12C,
            Self::Parsnip => 0xD6C69A,
            Self::Jojoba => 0xDABE81,
            Self::SaharaSun => 0xDFC08A,
            Self::Beeswax => 0xEBA851,
            Self::GoldenRod => 0xE2A829,
            Self::SpicyMustard => 0xD8AE47,
            Self::Citrus => 0xF9AC2F,
            Self::SpectraYellow => 0xF7B718,
            Self::GildedBeige => 0xB39F8D,
            Self::Gravel => 0xCBBFA2,
            Self::Cocoon => 0xC9B27C,
            Self::Rattan => 0xD1B272,
            Self::Ochre => 0xD6AF66,
            Self::NewWheat => 0xD7B57F,
            Self::GoldenApricot => 0xDDA758,
            Self::Amber => 0xEFAD55,
            Self::Marigold => 0xFADC53,
            Self::WarmApricot => 0xFFB865,
            Self::Kumquat => 0xFBAA4C,
            Self::Saffron => 0xFFA500,
            Self::Peyote => 0xC5BBAE,
            Self::OysterGray => 0xCBC1AE,
            Self::WoodAsh => 0xD7CAB0,
            Self::Boulder => 0xD1BE9B,
            Self::Pebble => 0xCAB698,
            Self::Marzipan => 0xD8C09D,
            Self::AlmondBuff => 0xCCB390,
            Self::Beige => 0xD5BA98,
            Self::WinterWheat => 0xDFC09F,
            Self::ApricotIllusion => 0xE2C4A6,
            Self::Sheepskin => 0xDAB58F,
            Self::DesertMist => 0xE0B589,
            Self::BuffOrange => 0xFFBB7C,
            Self::ApricotNectar => 0xECAA79,
            Self::SalmonBuff => 0xFEAA7B,
            Self::Pumpkin => 0xF5A26F,
            Self::Zinnia => 0xFFA010,
            Self::IrishCream => 0xC0AC92,
            Self::SmokeGray => 0xCEBAA8,
            Self::ShiftingSand => 0xD8C0AD,
            Self::Frappe => 0xD1B7A0,
            Self::ToastedAlmond => 0xD2B49C,
            Self::Amberlight => 0xE2BEA2,
            Self::PeachParfait => 0xF8BFA8,
            Self::PeachNougat => 0xE6AF91,
            Self::CoralSands => 0xEDAA86,
            Self::BeachSand => 0xFBB995,
            Self::Peach => 0xF2A987,
            Self::PeachNectar => 0xFFB59B,
            Self::ApricotWash => 0xFBAC82,
            Self::PeachCobbler => 0xFFB181,
            Self::OrangeChiffon => 0xF9AA7D,
            Self::Mushroom => 0xBDACA3,
            Self::RoseDust => 0xCDB2A5,
            Self::PeachWhip => 0xDBBEB7,
            Self::CameoRose => 0xD7B8AB,
            Self::EveningSand => 0xDDB6AB,
            Self::PaleBlush => 0xE4BFB3,
            Self::RoseCloud => 0xDBB0A2,
            Self::SpanishVilla => 0xDFBAA9,
            Self::Hazelnut => 0xCFB095,
            Self::DustyPink => 0xDEAA9B,
            Self::CoralPink => 0xE8A798,
            Self::Salmon => 0xFAAA94,
            Self::PeachBud => 0xFDB2AB,
            Self::PeachMelba => 0xFBBDAF,
            Self::PeachPearl => 0xFFB2A5,
            Self::ApricotBlush => 0xFEAEA5,
            Self::RoseSmoke => 0xD3B4AD,
            Self::SilverPink => 0xDCB1AF,
            Self::PowderPink => 0xECB2B3,
            Self::Blossom => 0xF2B2AE,
            Self::PeachesNCream => 0xF4A6A3,
            Self::QuartzPink => 0xEFA6AA,
            Self::SepiaRose => 0xD4BAB6,
            Self::Lotus => 0xE2C1C0,
            Self::Peachskin => 0xDFB8B6,
            Self::CoralBlush => 0xE6B2B8,
            Self::CandyPink => 0xF5B0BD,
            Self::PinkNectar => 0xD8AAB7,
            Self::CameoPink => 0xDBA9B8,
            Self::PrismPink => 0xF0A1BF,
            Self::LilacSachet => 0xE9ADCA,
            Self::SweetLilac => 0xE8B5CE,
            Self::FragrantLilac => 0xCEADBE,
            Self::WinsomeOrchid => 0xD4B9CB,
            Self::PinkLavender => 0xD9AFCA,
            Self::PastelLavender => 0xD8A1C4,
            Self::OrchidBloom => 0xC5AECF,
            Self::OrchidPetal => 0xBFB4CB,
            Self::HushedViolet => 0xD1C0BF,
            Self::Iris => 0xBAAFBC,
            Self::PastelLilac => 0xBDB0D0,
            Self::LilacMarble => 0xC3BABF,
            Self::EveningHaze => 0xBDB8C7,
            Self::LavenderBlue => 0xC5C0D0,
            Self::Raindrops => 0xB1AAB3,
            Self::Thistle => 0xB9B3C5,
            Self::PurpleHeather => 0xBAB8D3,
            Self::ZenBlue => 0x9FA9BE,
            Self::XenonBlue => 0xB7C0D7,
            Self::WindChime => 0xCAC5C2,
            Self::GlacierGray => 0xC5C6C7,
            Self::GrayViolet => 0xBBBCBC,
            Self::MicroChip => 0xBABCC0,
            Self::GrayDawn => 0xBBC1CC,
            Self::QuietGray => 0xB9BABD,
            Self::Heather => 0xB7C0D6,
            Self::Skyway => 0xADBED3,
            Self::CashmereBlue => 0xA5B8D0,
            Self::BlueBell => 0x93B4D7,
            Self::AiryBlue => 0x92B6D5,
            Self::LunarRock => 0xC5C5C5,
            Self::HarborMist => 0xAFB1B4,
            Self::VaporBlue => 0xBEBDBD,
            Self::PearlBlue => 0xB0B7BE,
            Self::CelestialBlue => 0xA3B4C4,
            Self::PowderBlue => 0x96B3D2,
            Self::CloudBlue => 0xA2B6B9,
            Self::WinterSky => 0xA9C0CB,
            Self::BlueTopaz => 0x78BDD4,
            Self::CorydalisBlue => 0xA9CADA,
            Self::Aquamarine => 0x9DC3D4,
            Self::SeaAngel => 0x98BFCA,
            Self::CoolBlue => 0xA5C5D9,
            Self::SkyBlue => 0x8ABAD3,
            Self::BalticSea => 0x79B5DB,
            Self::Moonstruck => 0xC2BEB6,
            Self::SilverLining => 0xBDB6AB,
            Self::Mercury => 0xBAC2BA,
            Self::Metal => 0xBABFBC,
            Self::SkyGray => 0xBCC8C6,
            Self::Smoke => 0xBFC8C3,
            Self::Ether => 0x9EB6B8,
            Self::Stratosphere => 0x9EC1CC,
            Self::Aquatic => 0x99C1CC,
            Self::GulfStream => 0x88C3D0,
            Self::PorcelainBlue => 0x95C0CB,
            Self::PetitFour => 0x87C2D4,
            Self::BachelorButton => 0x4ABBD5,
            Self::AntiquaSand => 0x83C2CD,
            Self::IslandParadise => 0x95DEE3,
            Self::TibetanStone => 0x82C2C7,
            Self::BlueFox => 0xB9BCB6,
            Self::SurfSpray => 0xB4C8C2,
            Self::EggshellBlue => 0xA3CCC9,
            Self::CanalBlue => 0x9CC2C5,
            Self::AquaSky => 0x7BC4C4,
            Self::AquaSplash => 0x85CED1,
            Self::AngelBlue => 0x83C5CD,
            Self::BlueRadiance => 0x58C9D4,
            Self::HarborGray => 0xA8C0BB,
            Self::Silver => 0xA2A2A1,
            Self::Holiday => 0x81C3B4,
            Self::Bermuda => 0x60C9B3,
            Self::Cockatoo => 0x58C8B6,
            Self::SiltGreen => 0xA9BDB1,
            Self::AquaFoam => 0xADC3B4,
            Self::OceanWave => 0x8EC5B6,
            Self::Cascade => 0x76C1B2,
            Self::BeveledGlass => 0x7ACCB8,
            Self::Opal => 0x77CFB7,
            Self::ElectricGreen => 0x4BC3A8,
            Self::SeaFoam => 0xB7C2B2,
            Self::SubtleGreen => 0xB5CBBB,
            Self::GrayedJade => 0x9BBEA9,
            Self::NeptuneGreen => 0x7FBB9E,
            Self::Pelican => 0xC1BCAC,
            Self::Alfalfa => 0xB7B59F,
            Self::CameoGreen => 0xAAC0AD,
            Self::Sprucestone => 0x9FC09C,
            Self::Meadow => 0x8BBA94,
            Self::Peapod => 0x82B185,
            Self::ZephyrGreen => 0x7CB083,
            Self::AbsintheGreen => 0x76B583,
            Self::SpringBud => 0x6BCD9C,
            Self::SpringBouquet => 0x6DCE87,
            Self::AbbeyStone => 0xABA798,
            Self::Dove => 0xB3ADA7,
            Self::GreenFlash => 0x79C753,
            Self::SprayGreen => 0xAEA692,
            Self::SageGreen => 0xB2AC88,
            Self::Tarragon => 0xA4AE77,
            Self::LeafGreen => 0x9FAF6C,
            Self::HerbalGarden => 0x9CAD60,
            Self::ParrotGreen => 0x8DB051,
            Self::Greenery => 0x88B04B,
            Self::Eucalyptus => 0xB1A992,
            Self::PaleOliveGreen => 0xB5AD88,
            Self::WinterPear => 0xB0B487,
            Self::WeepingWillow => 0xB3B17B,
            Self::SweetPea => 0xA3A969,
            Self::LindenGreen => 0xC4BF71,
            Self::Palm => 0xAFAF5E,
            Self::GreenOasis => 0xB0B454,
            Self::AppleGreen => 0xB5B644,
            Self::JasmineGreen => 0x7EC845,
            Self::Citronelle => 0xB8AF23,
            Self::LeekGreen => 0xB7B17A,
            Self::GoldenGreen => 0xBDB369,
            Self::CressGreen => 0xBCA949,
            Self::WarmOlive => 0xC7B63C,
            Self::AshesOfRoses => 0xB5ACAB,
            Self::SilverFern => 0xBBAA7E,
            Self::SouthernMoss => 0xBCA66A,
            Self::Olivenite => 0xC1A65C,
            Self::OilYellow => 0xC4A647,
            Self::LemonCurry => 0xCDA323,
            Self::CeylonYellow => 0xD4AE40,
            Self::PaleGold => 0xBD9865,
            Self::Sauterne => 0xC5A253,
            Self::GoldenSpice => 0xC6973F,
            Self::GoldenYellow => 0xCB8E16,
            Self::OldGold => 0xECA825,
            Self::MangoMojito => 0xD69C2F,
            Self::GingerRoot => 0xBFA58A,
            Self::IcedCoffee => 0xB18F6A,
            Self::AutumnBlaze => 0xD9922E,
            Self::MineralYellow => 0xD39C43,
            Self::ArtisansGold => 0xF2AB46,
            Self::GoldenGlow => 0xD99938,
            Self::GoldenOrange => 0xD7942D,
            Self::CadmiumYellow => 0xEE9626,
            Self::RadiantYellow => 0xFC9E21,
            Self::GoldFusion => 0xFFB000,
            Self::Travertine => 0xAE997D,
            Self::Safari => 0xBAAA91,
            Self::TaosTaupe => 0xBFA77F,
            Self::Porcini => 0xCCA580,
            Self::FallLeaf => 0xC9A86A,
            Self::HoneyGold => 0xD1A054,
            Self::Chamois => 0xF7B26A,
            Self::Butterscotch => 0xE19640,
            Self::DarkCheddar => 0xE08119,
            Self::IcelandPoppy => 0xF4963A,
            Self::Apricot => 0xF19035,
            Self::FlameOrange => 0xFB8B23,
            Self::BlazingOrange => 0xFFA64F,
            Self::BrightMarigold => 0xFF8D00,
            Self::CandiedGinger => 0xBFA387,
            Self::WarmSand => 0xC5AE91,
            Self::Sesame => 0xBAA38B,
            Self::PaleKhaki => 0xBFAF92,
            Self::MojaveDesert => 0xC7B595,
            Self::Semolina => 0xCEB899,
            Self::Latte => 0xC5A582,
            Self::Sand => 0xCCA67F,
            Self::Clay => 0xD2A172,
            Self::GoldEarth => 0xDD9C6B,
            Self::ApricotTan => 0xDD9760,
            Self::Cantaloupe => 0xFFA177,
            Self::Muskmelon => 0xEC935E,
            Self::Papaya => 0xFEA166,
            Self::MockOrange => 0xFFA368,
            Self::Tangerine => 0xF88F58,
            Self::CarrotCurl => 0xFE8C18,
            Self::AutumnGlory => 0xFF8812,
            Self::Turmeric => 0xFE840E,
            Self::Humus => 0xB7A793,
            Self::FeatherGray => 0xB8AD9E,
            Self::OxfordTan => 0xB8A99A,
            Self::WhitePepper => 0xB6A893,
            Self::Doeskin => 0xBDAB9B,
            Self::Moonlight => 0xC5B1A0,
            Self::CubanSand => 0xC1A68D,
            Self::RugbyTan => 0xC2A594,
            Self::MapleSugar => 0xC9A38D,
            Self::Sirocco => 0xC39D88,
            Self::PinkSand => 0xDFB19B,
            Self::AlmostApricot => 0xE5B39B,
            Self::DustyCoral => 0xD29B83,
            Self::PeachBloom => 0xD99B7C,
            Self::CoralReef => 0xFAA181,
            Self::CanyonSunset => 0xE1927A,
            Self::ShellCoral => 0xEA9575,
            Self::Tangelo => 0xFE7E03,
            Self::CadmiumOrange => 0xF99471,
            Self::CoralCloud => 0xE2A9A1,
            Self::PeachAmber => 0xFB9F93,
            Self::Fenugreek => 0xC0916C,
            Self::DustedClay => 0xCC7357,
            Self::PastryShell => 0xBD8C66,
            Self::PapayaPunch => 0xFCA289,
            Self::DesertFlower => 0xFF9687,
            Self::Etherea => 0xA5958F,
            Self::MahoganyRose => 0xC5A193,
            Self::MistyRose => 0xCAA39A,
            Self::MellowRose => 0xD9A6A1,
            Self::PeachBeige => 0xD3A297,
            Self::BloomingDahlia => 0xEB9687,
            Self::Shrimp => 0xE29A86,
            Self::PeachPink => 0xFA9A85,
            Self::PaleMauve => 0xC6A4A4,
            Self::BridalRose => 0xD69FA2,
            Self::Blush => 0xD1969A,
            Self::CandlelightPeach => 0xF8A39D,
            Self::ConchShell => 0xFC8F9B,
            Self::SalmonRose => 0xFF8D94,
            Self::PinkIcing => 0xEEA0A6,
            Self::Peony => 0xED9CA8,
            Self::FlamingoPink => 0xF7969E,
            Self::BurnishedLilac => 0xC5AEB1,
            Self::Zephyr => 0xC89FA5,
            Self::SeaPink => 0xDE98AB,
            Self::MorningGlory => 0xEE819F,
            Self::GeraniumPink => 0xF6909D,
            Self::DawnPink => 0xBFA3AF,
            Self::OrchidSmoke => 0xD294AA,
            Self::Rosebloom => 0xE290B2,
            Self::BegoniaPink => 0xEC9ABE,
            Self::SachetPink => 0xF18AAD,
            Self::AuroraPink => 0xE881A6,
            Self::KeepsakeLilac => 0xC0A5AE,
            Self::VioletIce => 0xC2ACB1,
            Self::FuchsiaPink => 0xDF88B7,
            Self::LilacChiffon => 0xDE9BC4,
            Self::MauveMist => 0xC49BD4,
            Self::Orchid => 0xD198C5,
            Self::OrchidBouquet => 0xD1ACCE,
            Self::LavenderFrost => 0xBDABBE,
            Self::FairOrchid => 0xC0AAC0,
            Self::CrocusPetal => 0xB99BC5,
            Self::Lavendula => 0xBCA4CB,
            Self::PurpleRose => 0xB09FCA,
            Self::LilacBreeze => 0xB3A0C9,
            Self::Porpoise => 0xA7A19E,
            Self::CloudGray => 0xB7A9AC,
            Self::MistyLilac => 0xBCB4C4,
            Self::Lavender => 0xAFA4CE,
            Self::IcelandicBlue => 0xA9ADC2,
            Self::CosmicSky => 0xAAAAC4,
            Self::LanguidLavender => 0xA2A1BA,
            Self::Aleutian => 0x9A9EB3,
            Self::KentuckyBlue => 0xA5B3CC,
            Self::Serenity => 0x91A8D0,
            Self::PlacidBlue => 0x8CADD3,
            Self::VistaBlue => 0x81A0D4,
            Self::BelAirBlue => 0x819AC1,
            Self::StormGray => 0xB5BAB6,
            Self::DreamBlue => 0xA0BCD0,
            Self::BlueFog => 0x9BABBB,
            Self::Cerulean => 0x9BB7D4,
            Self::ChambrayBlue => 0x9EB4D3,
            Self::HighRise => 0xAEB2B5,
            Self::AngelFalls => 0xA3BDD3,
            Self::AlaskanBlue => 0x6DA9D2,
            Self::Quarry => 0x98A0A5,
            Self::BelgianBlock => 0xA3A9A6,
            Self::Tradewinds => 0x7F8793,
            Self::SterlingBlue => 0xA2B9C2,
            Self::ForgetMeNot => 0x8FADBD,
            Self::AirBlue => 0x77ACC7,
            Self::EtherealBlue => 0x5CA6CE,
            Self::MilkyBlue => 0x72A8BA,
            Self::BlueGrotto => 0x5CACCE,
            Self::NorseBlue => 0x4CA5C7,
            Self::CrystalSeas => 0x5DAFCE,
            Self::SilverCloud => 0xBEB7B0,
            Self::ChateauGray => 0xBBB1A8,
            Self::PuritanGray => 0xA8B0AE,
            Self::MirageGray => 0xABAFAE,
            Self::Pigeon => 0xA9AFAA,
            Self::GrayMist => 0x99AEAE,
            Self::BlueHaze => 0xA5BCBB,
            Self::MarineBlue => 0x76AFB6,
            Self::Aquarelle => 0x61AAB1,
            Self::AquaSea => 0x6BAAAE,
            Self::Aqua => 0x64A1AD,
            Self::RiverBlue => 0x38AFCD,
            Self::Capri => 0x44BBCA,
            Self::BlueCuracao => 0x32BECC,
            Self::AquaGray => 0xA5B2AA,
            Self::Aquifer => 0x89ACAC,
            Self::AquaHaze => 0x87B9BC,
            Self::NileBlue => 0x76A7AB,
            Self::BlueTurquoise => 0x53B0AE,
            Self::PoolBlue => 0x67BCB3,
            Self::FloridaKeys => 0x56BEAB,
            Self::AquaGreen => 0x00B89F,
            Self::Atlantis => 0x00AF9F,
            Self::Waterfall => 0x3AB0A2,
            Self::Turquoise => 0x45B5AA,
            Self::BrightGreen => 0x009B5C,
            Self::MineralGray => 0xB2B6AC,
            Self::FrostyGreen => 0xA3B5A6,
            Self::DustyJadeGreen => 0x7BB5A3,
            Self::BiscayGreen => 0x55C6A9,
            Self::MintLeaf => 0x00B694,
            Self::Lichen => 0x9BC2B1,
            Self::Spearmint => 0x64BFA4,
            Self::Hemlock => 0x97C1A1,
            Self::Ming => 0x7CB08A,
            Self::JadeCream => 0x60B892,
            Self::PussywillowGray => 0xAEACA1,
            Self::AgateGray => 0xB1B09F,
            Self::Swamp => 0xA8B197,
            Self::LaurelGreen => 0xADBBA1,
            Self::SmokeGreen => 0xA8BBA2,
            Self::FairGreen => 0x92AF88,
            Self::QuietGreen => 0x9EBC97,
            Self::LightGrassGreen => 0x7CB68E,
            Self::IrishGreen => 0x45BE76,
            Self::MossGray => 0xAFAB97,
            Self::Reseda => 0xA1AD92,
            Self::ForestShade => 0x91AC80,
            Self::GreenTea => 0x86A96F,
            Self::Shamrock => 0x6FA26B,
            Self::GrassGreen => 0x7BB369,
            Self::BudGreen => 0x79B465,
            Self::Paloma => 0x9F9C99,
            Self::DesertSage => 0xA7AE9E,
            Self::Tendril => 0x89A06B,
            Self::VintageKhaki => 0x9A9186,
            Self::LondonFog => 0xA29E92,
            Self::Tea => 0x999B85,
            Self::Mistletoe => 0x8AA282,
            Self::GreenEyes => 0x7D956D,
            Self::JadeGreen => 0x759465,
            Self::MacawGreen => 0x9BB53E,
            Self::MeadowGreen => 0x739957,
            Self::Kiwi => 0x7AAB55,
            Self::Foliage => 0x75A14F,
            Self::Sage => 0x91946E,
            Self::Fern => 0x9AA067,
            Self::DarkCitron => 0xA0AC4F,
            Self::SpinachGreen => 0x909B4C,
            Self::GrayGreen => 0xA49A79,
            Self::Cedar => 0x928E64,
            Self::Moss => 0xA09D59,
            Self::Oasis => 0xA3A04E,
            Self::GoldenLime => 0x9A9738,
            Self::SplitPea => 0x9C9A40,
            Self::LentilSprout => 0xABA44D,
            Self::Elm => 0xA39F86,
            Self::Willow => 0x9A8B4F,
            Self::GoldenOlive => 0xAF9841,
            Self::SlateGreen => 0xA0987C,
            Self::Khaki => 0xA39264,
            Self::AntiqueGold => 0xB59E5F,
            Self::BurnishedGold => 0xAA9855,
            Self::GreenSulphur => 0xAE8E2C,
            Self::Goat => 0xA89A91,
            Self::RichGold => 0xC8B273,
            Self::AntiqueMoss => 0xB9A023,
            Self::OliveOil => 0xA98B2D,
            Self::SimplyTaupe => 0xAD9F93,
            Self::CurdsAndWhey => 0xBCA483,
            Self::Croissant => 0xC4AB86,
            Self::Curry => 0xBE9E6F,
            Self::Taffy => 0xC39B6A,
            Self::Tinsel => 0xC3964D,
            Self::Honey => 0xBA9238,
            Self::BrightGold => 0xCF9F52,
            Self::HarvestGold => 0xB68A3A,
            Self::Narcissus => 0xC39449,
            Self::NuggetGold => 0xC89720,
            Self::TawnyOlive => 0xC4962C,
            Self::Arrowwood => 0xBC8D1F,
            Self::Incense => 0xAF9A7E,
            Self::Sunflower => 0xD39237,
            Self::PureCashmere => 0xADA396,
            Self::Crockery => 0xA49887,
            Self::PlazaTaupe => 0xAEA393,
            Self::Tuffet => 0xA59788,
            Self::Aluminum => 0x9F9586,
            Self::Twill => 0xA79B82,
            Self::Greige => 0x928475,
            Self::OliveGray => 0xA6997A,
            Self::Sponge => 0xA49775,
            Self::Starfish => 0xB09A77,
            Self::Antelope => 0xB19664,
            Self::MustardGold => 0xB08E51,
            Self::AmberGold => 0xC19552,
            Self::Yam => 0xD0893F,
            Self::GoldenNugget => 0xDB9B59,
            Self::HoneyYellow => 0xCA9456,
            Self::OakBuff => 0xCF9C63,
            Self::Nugget => 0xCF8848,
            Self::DesertSun => 0xC87629,
            Self::Topaz => 0xD08344,
            Self::OrangePepper => 0xDF7500,
            Self::LightTaupe => 0xB19D8D,
            Self::Nomad => 0xB49F89,
            Self::Tuscany => 0xBE9785,
            Self::CafeCreme => 0xC79685,
            Self::Roebuck => 0xB09080,
            Self::Sandstorm => 0xBD8B69,
            Self::OrangeOchre => 0xDC793A,
            Self::RussetOrange => 0xE47127,
            Self::SunOrange => 0xF48037,
            Self::HarvestPumpkin => 0xD56231,
            Self::String => 0xAA9F96,
            Self::Natural => 0xAA907D,
            Self::Cornstalk => 0xA9947A,
            Self::Brush => 0xB99984,
            Self::WarmTaupe => 0xAF9483,
            Self::Nougat => 0xB69885,
            Self::Macaroon => 0xB38B71,
            Self::Lark => 0xB89B72,
            Self::Copper => 0xC47E5A,
            Self::PrairieSand => 0xB59A6A,
            Self::ToastedNut => 0xC08768,
            Self::Sandstone => 0xC48A69,
            Self::CoralHaze => 0xE38E84,
            Self::MutedClay => 0xD29380,
            Self::Toast => 0xCA9978,
            Self::Pheasant => 0xC68463,
            Self::Doe => 0xB98E68,
            Self::Tan => 0xB69574,
            Self::Biscuit => 0xB4835B,
            Self::CoralGold => 0xD27D56,
            Self::CopperTan => 0xDE8E65,
            Self::BrandiedMelon => 0xCE7B5B,
            Self::Butterum => 0xC68F65,
            Self::Buckskin => 0xD18E54,
            Self::AutumnSunset => 0xF38554,
            Self::DustyOrange => 0xE27A53,
            Self::SunBaked => 0xD27F63,
            Self::GoldenOchre => 0xC77943,
            Self::PeachCaramel => 0xC5733D,
            Self::TomatoCream => 0xC57644,
            Self::CoralRose => 0xF3774D,
            Self::Amberglow => 0xDC793E,
            Self::PersimmonOrange => 0xF47327,
            Self::BirdOfParadise => 0xFF8C55,
            Self::OrangeTiger => 0xF96714,
            Self::OrangePeel => 0xFA7A35,
            Self::Nectarine => 0xFF8656,
            Self::Carrot => 0xFD6F3B,
            Self::VermillionOrange => 0xF9633B,
            Self::PuffinsBill => 0xE95C20,
            Self::VibrantOrange => 0xFF7420,
            Self::Atmosphere => 0xA89C94,
            Self::Cobblestone => 0xA89A8E,
            Self::Stucco => 0xA58D7F,
            Self::Chanterelle => 0xA28776,
            Self::Almondine => 0xA78C8B,
            Self::Cork => 0xBA8671,
            Self::Sunburn => 0xB37256,
            Self::CanyonClay => 0xCE8477,
            Self::Almond => 0xA7754D,
            Self::CoralAlmond => 0xE29D94,
            Self::Carnelian => 0xCE785D,
            Self::Meerkat => 0xA46F44,
            Self::Caramel => 0xC37C54,
            Self::Langoustino => 0xCA6C56,
            Self::Arabesque => 0xD16F52,
            Self::Melon => 0xFE8863,
            Self::ApricotBuff => 0xCD7E4D,
            Self::BurntOrange => 0xC86733,
            Self::GoldFlame => 0xB45422,
            Self::Flamingo => 0xDF7253,
            Self::Nasturtium => 0xFE6347,
            Self::Firecracker => 0xF36944,
            Self::ExoticOrange => 0xF96531,
            Self::JaffaOrange => 0xD86D39,
            Self::MandarinOrange => 0xEC6A37,
            Self::DragonFire => 0xFC642D,
            Self::GoldenPoppy => 0xF56733,
            Self::Bark => 0xA99592,
            Self::AdobeRose => 0xBA9F99,
            Self::ShadowGray => 0xBBA5A0,
            Self::Fawn => 0xAE9490,
            Self::RoseTan => 0xD19C97,
            Self::CameoBrown => 0xC08A80,
            Self::Rosette => 0xCE8E8B,
            Self::LobsterBisque => 0xDD9289,
            Self::RoseDawn => 0xC2877B,
            Self::TerraCotta => 0xD38377,
            Self::BurntCoral => 0xE9897E,
            Self::Crabapple => 0xD77E70,
            Self::Coral => 0xED7464,
            Self::Camellia => 0xF6745F,
            Self::FreshSalmon => 0xFF7F6A,
            Self::FusionCoral => 0xFF8576,
            Self::Persimmon => 0xF67866,
            Self::CoralQuartz => 0xF77464,
            Self::LivingCoral => 0xFF6F61,
            Self::PeachEcho => 0xF7786B,
            Self::PurpleDove => 0x98878C,
            Self::BrandiedApricot => 0xCA848A,
            Self::Mauveglow => 0xD18489,
            Self::TeaRose => 0xDC7178,
            Self::Lantana => 0xDA7E7A,
            Self::PeachBlossom => 0xDE8286,
            Self::ShellPink => 0xF88180,
            Self::SugarCoral => 0xF56C73,
            Self::GeorgiaPeach => 0xF97272,
            Self::Sphinx => 0xAB9895,
            Self::DeauvilleMauve => 0xAF9294,
            Self::Lilas => 0xB88995,
            Self::Foxglove => 0xB98391,
            Self::Polignac => 0xC28799,
            Self::WildRose => 0xCE8498,
            Self::StrawberryIce => 0xE78B90,
            Self::Confetti => 0xE6798E,
            Self::StrawberryPink => 0xF57F8E,
            Self::PinkLemonade => 0xEE6D8A,
            Self::Woodrose => 0xAE8C8E,
            Self::OrchidHaze => 0xB0879B,
            Self::MauveOrchid => 0xB58299,
            Self::WildOrchid => 0xD979A2,
            Self::PinkCarnation => 0xED7A9E,
            Self::AzaleaPink => 0xE96A97,
            Self::CashmereRose => 0xCE879F,
            Self::MoonliteMauve => 0xD28FB0,
            Self::SmokyGrape => 0xB88AAC,
            Self::Crocus => 0xC67FAE,
            Self::OperaMauve => 0xCA80B1,
            Self::Cyclamen => 0xD687BA,
            Self::MauveShadows => 0xB598A3,
            Self::SeaFog => 0xA5929D,
            Self::LavenderMist => 0xAE90A7,
            Self::LavenderHerb => 0xB18EAA,
            Self::Violet => 0xC17FB5,
            Self::VioletTulle => 0xC193C0,
            Self::AfricanViolet => 0xB085B7,
            Self::Lupine => 0xBE9CC1,
            Self::RegalOrchid => 0xA98BAF,
            Self::SheerLilac => 0xB793C0,
            Self::SandVerbena => 0x9F90C1,
            Self::Satellite => 0x9F8D89,
            Self::OpalGray => 0xA49E9E,
            Self::Ash => 0xA09998,
            Self::GullGray => 0xA49CA0,
            Self::Wisteria => 0xA198AF,
            Self::HeirloomLilac => 0x9D96B2,
            Self::Viola => 0xA692BA,
            Self::Rhapsody => 0x9F86AA,
            Self::VioletTulip => 0x9E91C3,
            Self::SilverSconce => 0xA19FA5,
            Self::LilacGray => 0x9896A4,
            Self::DappleGray => 0x9C9BA7,
            Self::LavenderAura => 0x9F99AA,
            Self::Alloy => 0x98979A,
            Self::Sleet => 0x92949B,
            Self::Eventide => 0x959EB7,
            Self::LavenderLustre => 0x8C9CC1,
            Self::BlueHeron => 0x96A3C7,
            Self::BrunneraBlue => 0x9BA9CA,
            Self::EasterEgg => 0x919BC9,
            Self::Grapemist => 0x8398CA,
            Self::ThistleDown => 0x9499BB,
            Self::SweetLavender => 0x9A9BC1,
            Self::DustyBlue => 0x8C9DAD,
            Self::AshleyBlue => 0x8699AB,
            Self::ForeverBlue => 0x899BB8,
            Self::DellaRobbiaBlue => 0x7A9DCB,
            Self::Allure => 0x7291B4,
            Self::Hydrangea => 0x849BCC,
            Self::CornflowerBlue => 0x7391C8,
            Self::Provence => 0x658DC6,
            Self::GranadaSky => 0x5D81BB,
            Self::Arona => 0x879BA3,
            Self::StoneBlue => 0x829CA5,
            Self::DuskBlue => 0x7BA0C0,
            Self::HeritageBlue => 0x5D96BC,
            Self::LittleBoyBlue => 0x6EA2D5,
            Self::BonnieBlue => 0x539CCC,
            Self::MourningDove => 0x94908B,
            Self::Drizzle => 0xA09F9C,
            Self::Abyss => 0x8F9E9D,
            Self::Slate => 0x8C9FA1,
            Self::Tourmaline => 0x86A1A9,
            Self::CameoBlue => 0x769DA6,
            Self::BlueMist => 0x5BACC3,
            Self::HorizonBlue => 0x289DBE,
            Self::DelphiniumBlue => 0x6198AE,
            Self::MauiBlue => 0x52A2B4,
            Self::CyanBlue => 0x14A3C7,
            Self::Aquarius => 0x3CADD4,
            Self::BlueAtoll => 0x00B1D2,
            Self::Stillwater => 0x70A4B0,
            Self::ReefWaters => 0x6F9FA9,
            Self::Limestone => 0x989A98,
            Self::GhostGray => 0x9C9B98,
            Self::SilverBlue => 0x8A9A9A,
            Self::MineralBlue => 0x6D9192,
            Self::Porcelain => 0x5D9CA4,
            Self::ScubaBlue => 0x00ABC0,
            Self::PeacockBlue => 0x00A0B0,
            Self::Bluebird => 0x009DAE,
            Self::WetWeather => 0x929090,
            Self::BlueSurf => 0x90A8A4,
            Self::Wasabi => 0x73A89E,
            Self::Canton => 0x6DA29E,
            Self::DustyTurquoise => 0x649B9E,
            Self::SeaBlue => 0x549F98,
            Self::Meadowbrook => 0x60A0A3,
            Self::Baltic => 0x279D9F,
            Self::Ceramic => 0x00AAA9,
            Self::Jadeite => 0x95A69F,
            Self::AgateGreen => 0x599F99,
            Self::Lagoon => 0x4D9E9A,
            Self::SeaGreen => 0x149C88,
            Self::BrightAqua => 0x30A299,
            Self::PoolGreen => 0x00AF9D,
            Self::Billiard => 0x00AA92,
            Self::PeacockGreen => 0x00A78B,
            Self::BerylGreen => 0x619187,
            Self::Arcadia => 0x00A28A,
            Self::MarineGreen => 0x40A48E,
            Self::FlintGray => 0xA09C98,
            Self::SlateGray => 0x8A9691,
            Self::GreenMilieu => 0x8A9992,
            Self::LilyPad => 0x818F84,
            Self::IcebergGreen => 0x8C9C92,
            Self::GreenBay => 0x7E9285,
            Self::Feldspar => 0x729B8B,
            Self::GreenSpruce => 0x589F7E,
            Self::GumdropGreen => 0x2EA785,
            Self::WroughtIron => 0x999E98,
            Self::GraniteGreen => 0x86A293,
            Self::MalachiteGreen => 0x709A89,
            Self::CremeDeMenthe => 0x70A38D,
            Self::WinterGreen => 0x4F9E81,
            Self::MingGreen => 0x3AA278,
            Self::HollyGreen => 0x0F9D76,
            Self::Mint => 0x00A170,
            Self::Blarney => 0x00A776,
            Self::Seagrass => 0x959889,
            Self::Katydid => 0x66BC91,
            Self::ShaleGreen => 0x739072,
            Self::Greenbriar => 0x4B9B69,
            Self::KellyGreen => 0x339C5E,
            Self::Basil => 0x879F84,
            Self::IslandGreen => 0x2BAE66,
            Self::MineralGreen => 0x7A9B78,
            Self::Jadesheen => 0x77A276,
            Self::Peppermint => 0x699E6D,
            Self::VibrantGreen => 0x55A860,
            Self::ClassicGreen => 0x39A845,
            Self::PoisonGreen => 0x4DB560,
            Self::FrostGray => 0x848283,
            Self::OilGreen => 0x80856D,
            Self::TurfGreen => 0x6F8C69,
            Self::StoneGreen => 0x658E67,
            Self::FluoriteGreen => 0x699158,
            Self::OnlineLime => 0x44883C,
            Self::ElephantSkin => 0x8F8982,
            Self::RockRidge => 0x918C86,
            Self::LodenFrost => 0x788F74,
            Self::AspenGreen => 0x7E9B76,
            Self::Watercress => 0x748C69,
            Self::ForestGreen => 0x6B8D53,
            Self::PiquantGreen => 0x769358,
            Self::Epsom => 0x849161,
            Self::TurtleGreen => 0x81894E,
            Self::Peridot => 0x819548,
            Self::SilverSage => 0x938B78,
            Self::DuskyGreen => 0x746C57,
            Self::Mosstone => 0x858961,
            Self::Guacamole => 0x797B3A,
            Self::GreenOlive => 0x8D8B55,
            Self::LaurelOak => 0x918C7E,
            Self::Vetiver => 0x807D6F,
            Self::Mermaid => 0x817A65,
            Self::Aloe => 0x817A60,
            Self::Boa => 0x8E855F,
            Self::DriedHerb => 0x847A59,
            Self::TreeHouse => 0x988C75,
            Self::GreenMoss => 0x857946,
            Self::TaupeGray => 0x8E7C71,
            Self::EcruOlive => 0x927B3C,
            Self::GoldenPalm => 0xAA8805,
            Self::AmberGreen => 0x9A803A,
            Self::BronzeMist => 0x9C7E41,
            Self::Fossil => 0x806F63,
            Self::FennelSeed => 0x998456,
            Self::DullGold => 0x8A6F48,
            Self::MedalBronze => 0x977547,
            Self::ChaiTea => 0xB1832F,
            Self::Dune => 0x998978,
            Self::Elmwood => 0x8C7C61,
            Self::Kelp => 0x988467,
            Self::AntiqueBronze => 0x907954,
            Self::Bistre => 0x98754A,
            Self::TigersEye => 0x977C61,
            Self::SpruceYellow => 0xBE8A4A,
            Self::Chipmunk => 0x976F4C,
            Self::AppleCinnamon => 0xB0885A,
            Self::GoldenOak => 0xBE752D,
            Self::HoneyMustard => 0xB68F52,
            Self::IncaGold => 0xBB7A2C,
            Self::RoastedPecan => 0x93592B,
            Self::RoastedCashew => 0x918579,
            Self::SenecaRock => 0x9A927F,
            Self::WinterTwig => 0x948A7A,
            Self::Chinchilla => 0x9C8E7B,
            Self::Coriander => 0x938772,
            Self::PetrifiedOak => 0x8D7960,
            Self::LeadGray => 0x8A7963,
            Self::Dijon => 0x97754C,
            Self::BoneBrown => 0x9D7446,
            Self::WoodThrush => 0xA47D43,
            Self::BrownSugar => 0xA17249,
            Self::Cashew => 0xA47149,
            Self::Marmalade => 0xC16512,
            Self::ArganOil => 0x8B593E,
            Self::Hazel => 0xAE7250,
            Self::AutumnMaple => 0xC46215,
            Self::AmberBrown => 0xA66646,
            Self::MoonRock => 0x958B84,
            Self::Fungi => 0x8F8177,
            Self::Praline => 0xAD8B75,
            Self::Camel => 0xB0846A,
            Self::TawnyBirch => 0xAE856C,
            Self::TawnyBrown => 0xAB856F,
            Self::CafeAuLait => 0xAE8774,
            Self::MochaMousse => 0xA47864,
            Self::TimberWolf => 0x8D8070,
            Self::DesertTaupe => 0x8D7E71,
            Self::SilverMink => 0x9F8D7C,
            Self::SepiaTint => 0x897560,
            Self::Portabella => 0x937B6A,
            Self::Amphora => 0x9F8672,
            Self::Tannin => 0xA68A6D,
            Self::Woodsmoke => 0x947764,
            Self::Burro => 0x947764,
            Self::TobaccoBrown => 0x9A7352,
            Self::IndianTan => 0xAD8567,
            Self::Lion => 0xA0714F,
            Self::Bran => 0xA66E4A,
            Self::Adobe => 0xA3623B,
            Self::TawnyOrange => 0xD37F6F,
            Self::Autumnal => 0xA15325,
            Self::SpiceRoute => 0xB95B3F,
            Self::AutumnLeaf => 0xB56A4C,
            Self::Exuberance => 0xE86800,
            Self::OrangePopsicle => 0xFF7913,
            Self::ApricotOrange => 0xC86B3C,
            Self::CelosiaOrange => 0xE8703A,
            Self::ScarletIbis => 0xF45520,
            Self::PineBark => 0x827064,
            Self::BeaverFur => 0x997867,
            Self::GingerSnap => 0x977D70,
            Self::RawUmber => 0x92705F,
            Self::BrickDust => 0xB07069,
            Self::PecanBrown => 0xA36E51,
            Self::RawSienna => 0xB9714F,
            Self::Ginger => 0xC96551,
            Self::Mango => 0xB75E41,
            Self::PureedPumpkin => 0xC34121,
            Self::SummerFig => 0xBE4B3B,
            Self::Koi => 0xD15837,
            Self::Tigerlily => 0xE2583E,
            Self::Orangeade => 0xE2552C,
            Self::Flame => 0xF2552C,
            Self::TangerineTango => 0xDD4124,
            Self::RedOrange => 0xF05627,
            Self::SteepleGray => 0x827E7C,
            Self::WildDove => 0x8B8C89,
            Self::Cloudburst => 0x837F7F,
            Self::StormFront => 0x787376,
            Self::Quail => 0x98868C,
            Self::Cinder => 0x8A7E78,
            Self::Antler => 0x957A76,
            Self::WistfulMauve => 0x946C74,
            Self::NostalgiaRose => 0xA4777E,
            Self::AshRose => 0xB5817D,
            Self::Burlwood => 0x9B716B,
            Self::OldRose => 0xB47B77,
            Self::CanyonRose => 0xAF6C67,
            Self::Mauvewood => 0xA75D67,
            Self::DesertSand => 0xBD7B74,
            Self::CedarWood => 0xA1655B,
            Self::Aragon => 0xB06455,
            Self::MineralRed => 0xB35457,
            Self::ApricotBrandy => 0xC26A5A,
            Self::BurntSienna => 0xC65D52,
            Self::Cranberry => 0xBB4A4D,
            Self::Emberglow => 0xEA6759,
            Self::Paprika => 0xCE4D42,
            Self::Grenadine => 0xDF3F32,
            Self::MandarinRed => 0xE74A33,
            Self::CherryTomato => 0xEB3C27,
            Self::Fiesta => 0xDD4132,
            Self::Elderberry => 0x9D848E,
            Self::HeatherRose => 0xAD6D7F,
            Self::MesaRose => 0xA66E7A,
            Self::DuskyOrchid => 0x9A7182,
            Self::MellowMauve => 0x996378,
            Self::DecoRose => 0x985F68,
            Self::RoseWine => 0xA4596D,
            Self::HollyBerry => 0xB44E5D,
            Self::RoseOfSharon => 0xDC5B62,
            Self::Chrysanthemum => 0xBE454F,
            Self::PorcelainRose => 0xEA6B6A,
            Self::SpicedCoral => 0xD75C5D,
            Self::Dubarry => 0xF25F66,
            Self::Poinsettia => 0xCB3441,
            Self::HotCoral => 0xF35B53,
            Self::Bittersweet => 0xD93744,
            Self::PoppyRed => 0xDC343B,
            Self::Moonscape => 0x725F69,
            Self::Bordeaux => 0x96637B,
            Self::DustyRose => 0xBA797D,
            Self::Malaga => 0x9F5069,
            Self::SunkistCoral => 0xEA6676,
            Self::ClaretRed => 0xC84C61,
            Self::CalypsoCoral => 0xEE5C6C,
            Self::Geranium => 0xDA3D58,
            Self::ParadisePink => 0xE4445E,
            Self::RedViolet => 0xA35776,
            Self::Carmine => 0xBC4869,
            Self::Azalea => 0xD42E5B,
            Self::FruitDove => 0xCE5B78,
            Self::DesertRose => 0xCF6977,
            Self::Bubblegum => 0xEA738D,
            Self::RaptureRose => 0xD16277,
            Self::CamelliaRose => 0xEB6081,
            Self::HotPink => 0xE55982,
            Self::FuchsiaRose => 0xC74375,
            Self::FandangoPink => 0xE04F80,
            Self::PinkYarrow => 0xCE3175,
            Self::Magenta => 0xD23C77,
            Self::ChateauRose => 0xD2738F,
            Self::ShockingPink => 0xDE5B8C,
            Self::LilacRose => 0xBD4275,
            Self::CarmineRose => 0xE35B8F,
            Self::Toadstool => 0x988088,
            Self::IbisRose => 0xCA628F,
            Self::Zinc => 0x92898A,
            Self::DahliaMauve => 0xA64F82,
            Self::RoseViolet => 0xC0428A,
            Self::SuperPink => 0xCE6BA4,
            Self::PhloxPink => 0xCE5E9A,
            Self::Mulberry => 0xA76C97,
            Self::SpringCrocus => 0xBA69A1,
            Self::Rosebud => 0xB65F9A,
            Self::Bodacious => 0xB76BA3,
            Self::DustyLavender => 0xA1759C,
            Self::IrisOrchid => 0xA767A2,
            Self::Valerian => 0x9F7A93,
            Self::DiffusedOrchid => 0x9879A2,
            Self::OrchidMist => 0x917798,
            Self::ChalkViolet => 0x8F7DA5,
            Self::EnglishLavendar => 0x9D7BB0,
            Self::Hyacinth => 0x936CA7,
            Self::AmethystOrchid => 0x926AA6,
            Self::FairyWren => 0x9479AF,
            Self::Bougainvillea => 0x9884B9,
            Self::PaisleyPurple => 0x8B79B1,
            Self::SunlitAllium => 0x9787BB,
            Self::Gull => 0x918C8F,
            Self::Nirvana => 0xA2919B,
            Self::PurpleAsh => 0x8F8395,
            Self::Dusk => 0x897F98,
            Self::Daybreak => 0x8981A0,
            Self::AsterPurple => 0x7D74A8,
            Self::DahliaPurple => 0x7E6EAC,
            Self::MinimalGray => 0x948D99,
            Self::Quicksilver => 0x7E7D88,
            Self::LavenderGray => 0x9890A2,
            Self::SilverFiligree => 0x7F7C81,
            Self::Sharkskin => 0x838487,
            Self::Tempest => 0x79839B,
            Self::Stonewash => 0x74809A,
            Self::CountryBlue => 0x717F9B,
            Self::PurpleImpression => 0x858FB1,
            Self::EnglishManor => 0x7181A4,
            Self::BlueIce => 0x70789B,
            Self::ColonyBlue => 0x65769A,
            Self::LavenderViolet => 0x767BA5,
            Self::PersianViolet => 0x8C8EB2,
            Self::PaleIris => 0x8895C5,
            Self::Jacaranda => 0x848DC5,
            Self::DeepPeriwinkle => 0x7C83BC,
            Self::SilverBullet => 0x81839A,
            Self::PersianJewel => 0x6E81BE,
            Self::BlueBonnet => 0x6384B8,
            Self::Iolite => 0x707BB4,
            Self::Titanium => 0x807D7F,
            Self::Infinity => 0x6E7E99,
            Self::GrayFlannel => 0x848182,
            Self::BlueShadow => 0x66829A,
            Self::FadedDenim => 0x798EA4,
            Self::BlueHeaven => 0x5B7E98,
            Self::Riviera => 0x5879A2,
            Self::Riverside => 0x4C6A92,
            Self::QuietHarbor => 0x5A789A,
            Self::SilverLakeBlue => 0x618BB9,
            Self::LichenBlue => 0x5D89B3,
            Self::PacificCoast => 0x5480AC,
            Self::Ultramarine => 0x5B7EBD,
            Self::Marina => 0x4F84C4,
            Self::Citadel => 0x748995,
            Self::Niagara => 0x5487A4,
            Self::CendreBlue => 0x3E7FA5,
            Self::AzureBlue => 0x4D91C6,
            Self::IbizaBlue => 0x007CB7,
            Self::DivaBlue => 0x007BB2,
            Self::AdriaticBlue => 0x5C899B,
            Self::BlueMoon => 0x3686A0,
            Self::Blithe => 0x0084BD,
            Self::NeutralGray => 0x8E918F,
            Self::Monument => 0x84898C,
            Self::Lead => 0x7A898F,
            Self::SmokeBlue => 0x6D8994,
            Self::Larkspur => 0x3C7D90,
            Self::Bluejay => 0x157EA0,
            Self::NavagioBay => 0x3183A0,
            Self::VividBlue => 0x0088B0,
            Self::DresdenBlue => 0x0086BB,
            Self::MalibuBlue => 0x008CC1,
            Self::BlueDanube => 0x0087B6,
            Self::BarrierReef => 0x0084A1,
            Self::HawaiianOcean => 0x008DB9,
            Self::StormBlue => 0x47788A,
            Self::PagodaBlue => 0x1A7F8E,
            Self::AlgiersBlue => 0x00859C,
            Self::CaneelBay => 0x00849F,
            Self::CapriBreeze => 0x008799,
            Self::BristolBlue => 0x558F91,
            Self::Arctic => 0x648589,
            Self::Teal => 0x478589,
            Self::LakeBlue => 0x008C96,
            Self::TealBlue => 0x007F7C,
            Self::Navigate => 0x008583,
            Self::DeepPeacockBlue => 0x008381,
            Self::Lapis => 0x008684,
            Self::Griffin => 0x8D8F8F,
            Self::ChinoisGreen => 0x7C8C87,
            Self::Trellis => 0x6A8988,
            Self::OilBlue => 0x658C88,
            Self::GreenBlueSlate => 0x358082,
            Self::LatigoBay => 0x379190,
            Self::ViridianGreen => 0x009499,
            Self::Columbia => 0x009288,
            Self::DynastyGreen => 0x008E80,
            Self::SpectraGreen => 0x009B8C,
            Self::PorcelainGreen => 0x108780,
            Self::Alhambra => 0x008778,
            Self::DeepSea => 0x4F7C74,
            Self::Greenlake => 0x007D69,
            Self::DeepGreen => 0x009276,
            Self::VividGreen => 0x009E82,
            Self::Emerald => 0x009473,
            Self::BottleGreen => 0x427D6D,
            Self::Viridis => 0x00846B,
            Self::Parakeet => 0x008C69,
            Self::DarkIvy => 0x5B7763,
            Self::PineGreen => 0x3A795E,
            Self::SimplyGreen => 0x009B75,
            Self::DeepMint => 0x009E6D,
            Self::LaurelWreath => 0x616F65,
            Self::JellyBean => 0x008658,
            Self::FernGreen => 0x008C45,
            Self::Shadow => 0x888D82,
            Self::SeaSpray => 0x717E6F,
            Self::DeepGrassGreen => 0x558367,
            Self::MediumGreen => 0x3C824E,
            Self::Kashmir => 0x6F8D6A,
            Self::HedgeGreen => 0x768A75,
            Self::MintGreen => 0x487D49,
            Self::SmokedPearl => 0x656466,
            Self::Kale => 0x5A7247,
            Self::Dill => 0x6F7755,
            Self::EnglishIvy => 0x61845B,
            Self::VineyardGreen => 0x5F7355,
            Self::WillowBough => 0x59754D,
            Self::ElmGreen => 0x547053,
            Self::ArtichokeGreen => 0x4B6D41,
            Self::Cactus => 0x53713D,
            Self::Treetop => 0x476A30,
            Self::Castlerock => 0x5F5E62,
            Self::Mayfly => 0x65663F,
            Self::Pesto => 0x595F34,
            Self::Gunmetal => 0x5C5D5B,
            Self::DeepLichenGreen => 0x6E6E5C,
            Self::Olivine => 0x666B54,
            Self::BronzeGreen => 0x525F48,
            Self::Cypress => 0x545A3E,
            Self::CallisteGreen => 0x757A4E,
            Self::CedarGreen => 0x5E6737,
            Self::TwistOfLime => 0x4E632C,
            Self::Grasshopper => 0x77824A,
            Self::DarkGullGray => 0x625D5D,
            Self::FourLeafClover => 0x616652,
            Self::LodenGreen => 0x6E7153,
            Self::CapuletOlive => 0x656344,
            Self::Avocado => 0x676232,
            Self::CallaGreen => 0x6A6F34,
            Self::Gargoyle => 0x686767,
            Self::CastorGray => 0x646762,
            Self::BungeeCord => 0x696156,
            Self::DustyOlive => 0x646356,
            Self::BurntOlive => 0x646049,
            Self::WinterMoss => 0x5B5A41,
            Self::Iguana => 0x818455,
            Self::OliveBranch => 0x646A45,
            Self::Woodbine => 0x7B7F32,
            Self::CharcoalGray => 0x6C6868,
            Self::StoneGray => 0x685E4F,
            Self::CovertGreen => 0x80765F,
            Self::OliveDrab => 0x756D47,
            Self::MartiniOlive => 0x716A4D,
            Self::FirGreen => 0x67592A,
            Self::Lizard => 0x71643E,
            Self::GothicOlive => 0x7C6E4F,
            Self::Capers => 0x695E4B,
            Self::Nutria => 0x75663E,
            Self::Butternut => 0x7A643F,
            Self::Plantation => 0x7A6332,
            Self::DriedTobacco => 0x997B38,
            Self::Tapenade => 0x805D24,
            Self::Kangaroo => 0x725E43,
            Self::Sepia => 0x6B543E,
            Self::CoffeeLiqueur => 0x6A513B,
            Self::Rubber => 0x815B37,
            Self::BuckthornBrown => 0xA76F1F,
            Self::BronzeBrown => 0x825E2F,
            Self::Cumin => 0x927240,
            Self::GoldenBrown => 0x91672F,
            Self::CathaySpice => 0x99642C,
            Self::Shitake => 0x736253,
            Self::Cub => 0x6E5C4B,
            Self::Caribou => 0x816D5E,
            Self::Otter => 0x7F674F,
            Self::Ermine => 0x836B4F,
            Self::Bison => 0x6E4F3A,
            Self::Emperador => 0x684832,
            Self::ToastedCoconut => 0x8B6A4F,
            Self::Thrush => 0x936B4F,
            Self::Toffee => 0x755139,
            Self::Dachshund => 0x704F37,
            Self::MonksRobe => 0x704822,
            Self::ThaiCurry => 0xAB6819,
            Self::HoneyGinger => 0xA86217,
            Self::FallenRock => 0x807669,
            Self::Brindle => 0x82776B,
            Self::Walnut => 0x776A5F,
            Self::Partridge => 0x725440,
            Self::Aztec => 0x7A5747,
            Self::Rawhide => 0x865E49,
            Self::MochaBisque => 0x8C543A,
            Self::LeatherBrown => 0x97572B,
            Self::CaramelCafe => 0x864C24,
            Self::GlazedGinger => 0x91552B,
            Self::SugarAlmond => 0x935529,
            Self::SudanBrown => 0xAC6B29,
            Self::PumpkinSpice => 0xA05C17,
            Self::Driftwood => 0x847A75,
            Self::CocoaBrown => 0x6C5043,
            Self::CarobBrown => 0x855C4C,
            Self::CoconutShell => 0x874E3C,
            Self::Russet => 0x8F5F50,
            Self::RusticBrown => 0x855141,
            Self::Sierra => 0x985C41,
            Self::BrownPatina => 0x834F3D,
            Self::GingerBread => 0x8C4A2F,
            Self::Umber => 0x944A1F,
            Self::Rust => 0xB55A30,
            Self::HawaiianSunset => 0xBB5C14,
            Self::BombayBrown => 0x9F5130,
            Self::Falcon => 0x6D625B,
            Self::Iron => 0x736460,
            Self::DeepTaupe => 0x7B6660,
            Self::Acorn => 0x7E5E52,
            Self::Clove => 0x876155,
            Self::Brownie => 0x8F7265,
            Self::SpicedApple => 0x783937,
            Self::Nutmeg => 0x7E5C54,
            Self::CopperBrown => 0x9A6051,
            Self::PottersClay => 0x9E4624,
            Self::Auburn => 0xA15843,
            Self::CinnamonStick => 0x9B4722,
            Self::Bruschetta => 0xA75949,
            Self::BurntBrick => 0xA14D3A,
            Self::BurntOchre => 0xBB4F35,
            Self::RooibosTea => 0xA23C26,
            Self::Sparrow => 0x69595C,
            Self::Flint => 0x705861,
            Self::Peppercorn => 0x6C5656,
            Self::PlumWine => 0x674550,
            Self::Marron => 0x6E4C4B,
            Self::CrushedBerry => 0x804F5A,
            Self::WildGinger => 0x7C4C53,
            Self::Cognac => 0x8B645A,
            Self::Mahogany => 0x824D46,
            Self::AppleButter => 0x844B4D,
            Self::Chutney => 0x98594B,
            Self::EtruscanRed => 0xA2574B,
            Self::WitheredRose => 0xA26666,
            Self::LightMahogany => 0xAD6D68,
            Self::Marsala => 0x964F4C,
            Self::ChiliOil => 0x8E3C36,
            Self::BakedClay => 0x9C5642,
            Self::RedOchre => 0x913832,
            Self::Redwood => 0xA6594C,
            Self::TandoriSpice => 0x9F4440,
            Self::SpicyOrange => 0xD73C26,
            Self::OrangeRust => 0xC25A3C,
            Self::Chili => 0xBE5141,
            Self::Ketchup => 0x9A382D,
            Self::MeccaOrange => 0xBD5745,
            Self::AutumnGlaze => 0xB3573F,
            Self::RedClay => 0xC2452D,
            Self::PlumTruffle => 0x675657,
            Self::RoseBrown => 0x80565B,
            Self::BarnRed => 0x8F423B,
            Self::GingerSpice => 0xB65D48,
            Self::HotSauce => 0xAB4F41,
            Self::CopperCoin => 0xBA6B57,
            Self::Cinnabar => 0x9C453B,
            Self::BrandyBrown => 0x73362A,
            Self::BossaNova => 0x973A36,
            Self::ValiantPoppy => 0xBC322C,
            Self::AuroraRed => 0xB93A32,
            Self::AuraOrange => 0xB4262A,
            Self::MoltenLava => 0xB5332E,
            Self::OrangeCom => 0xDA321C,
            Self::Poinciana => 0xCA3422,
            Self::RoseTaupe => 0x806062,
            Self::RenaissanceRose => 0x865560,
            Self::Nocturne => 0x7A4B56,
            Self::RoanRouge => 0x885157,
            Self::Maroon => 0x834655,
            Self::FadedRose => 0xBF6464,
            Self::DustyCedar => 0xAD5D5D,
            Self::EarthRed => 0x95424E,
            Self::GarnetRose => 0xAC4B55,
            Self::BaroqueRose => 0xB35A66,
            Self::SlateRose => 0xB45865,
            Self::Cardinal => 0xAD3E48,
            Self::BakedApple => 0xB34646,
            Self::DeepSeaCoral => 0xD9615B,
            Self::Cayenne => 0xE04951,
            Self::RococcoRed => 0xBB363F,
            Self::Toreador => 0xB61032,
            Self::Lychee => 0xBA0B32,
            Self::MarsRed => 0xBC2731,
            Self::Salsa => 0xAA182B,
            Self::PompeianRed => 0xA4292E,
            Self::GojiBerry => 0xB91228,
            Self::Tomato => 0xCE2939,
            Self::TomatoPuree => 0xC53346,
            Self::FlameScarlet => 0xCD212A,
            Self::ChineseRed => 0xBE132D,
            Self::FieryRed => 0xD01C1F,
            Self::Shark => 0x6D636B,
            Self::ArcticDusk => 0x735B6A,
            Self::BlackPlum => 0x6C5765,
            Self::Ephemera => 0x6F5965,
            Self::Tulipwood => 0x805466,
            Self::GrapeNectar => 0x8D5C74,
            Self::Damson => 0x854C65,
            Self::HawthornRose => 0x884C5E,
            Self::VioletQuartz => 0x8B4963,
            Self::DryRose => 0x8C4759,
            Self::RaspberryWine => 0xB63753,
            Self::Raspberry => 0xD32E5E,
            Self::RougeRed => 0xE24666,
            Self::Teaberry => 0xDC3855,
            Self::JalapenoRed => 0xB2103C,
            Self::Barberry => 0xBF1945,
            Self::SkiPatrol => 0xBB1237,
            Self::Hibiscus => 0xDD3848,
            Self::HighRiskRed => 0xC71F2D,
            Self::Lollipop => 0xCC1C3B,
            Self::TwilightMauve => 0x8B6F70,
            Self::RoseRed => 0xC92351,
            Self::VirtualPink => 0xC6174E,
            Self::BrightRose => 0xC51959,
            Self::Jazzy => 0xB61C50,
            Self::LovePotion => 0xC01352,
            Self::Beaujolais => 0x80304C,
            Self::RaspberrySorbet => 0xD2386C,
            Self::PinkPeacock => 0xC62168,
            Self::GrapeShake => 0x886971,
            Self::Honeysuckle => 0xD94F70,
            Self::PinkFlambe => 0xD3507A,
            Self::Cabaret => 0xCB3373,
            Self::BeetrootPurple => 0xCF2D71,
            Self::Clover => 0x8A3371,
            Self::CactusFlower => 0xA83E6C,
            Self::FuchsiaRed => 0xAB3475,
            Self::RaspberryRose => 0xCC4385,
            Self::VeryBerry => 0xB73275,
            Self::FuchsiaPurple => 0xD33479,
            Self::MagentaHaze => 0x9D446E,
            Self::BatonRouge => 0x973C6C,
            Self::PurpleWine => 0x8C3573,
            Self::ArgylePurple => 0x895C79,
            Self::PurpleGumdrop => 0x7A596F,
            Self::BerryConserve => 0x765269,
            Self::GrapeKiss => 0x7B4368,
            Self::Amethyst => 0x864D75,
            Self::DeepOrchid => 0x903F75,
            Self::StrikingPurple => 0x944E87,
            Self::PurpleOrchid => 0xAD4D8C,
            Self::Willowherb => 0x8E4483,
            Self::Grapeade => 0x85677B,
            Self::ConcordGrape => 0x7C5379,
            Self::VeryGrape => 0x927288,
            Self::RadiantOrchid => 0xAD5E99,
            Self::MeadowMauve => 0xA9568C,
            Self::Dahlia => 0x843E83,
            Self::HyacinthViolet => 0x8D4687,
            Self::VividViola => 0x993C7C,
            Self::Charisma => 0x632A60,
            Self::VintageViolet => 0x634F62,
            Self::GrapeJam => 0x725671,
            Self::ChineseViolet => 0x835E81,
            Self::GrapeCompote => 0x6B5876,
            Self::PatricianPurple => 0x6C4E79,
            Self::PurpleHeart => 0x745587,
            Self::PlumJam => 0x624076,
            Self::CrushedGrape => 0x7A547F,
            Self::LavenderCrystal => 0x936A98,
            Self::RoyalLilac => 0x774D8E,
            Self::Dewberry => 0x8B5987,
            Self::PurpleSapphire => 0x6F4685,
            Self::ImperialPalace => 0x604E7A,
            Self::Mystical => 0x5F4E72,
            Self::Bellflower => 0x9469A2,
            Self::DeepLavender => 0x775496,
            Self::ChiveBlossom => 0x7D5D99,
            Self::PicassoLily => 0x634878,
            Self::GrayRidge => 0x847986,
            Self::PurpleSage => 0x75697E,
            Self::MulledGrape => 0x675A74,
            Self::MontanaGrape => 0x6C5971,
            Self::PurpleHaze => 0x807396,
            Self::PassionFlower => 0x6D5698,
            Self::Cadet => 0x6A6378,
            Self::Heron => 0x62617E,
            Self::TwilightPurple => 0x66648B,
            Self::CorsicanBlue => 0x646093,
            Self::DustedPeri => 0x696BA0,
            Self::Veronica => 0x6D6695,
            Self::UltraViolet => 0x5F4B8B,
            Self::PurpleCorallite => 0x5A4E8F,
            Self::PurpleOpulence => 0x60569A,
            Self::Excalibur => 0x676168,
            Self::Tornado => 0x5E5B60,
            Self::VolcanicGlass => 0x615C60,
            Self::FolkstoneGray => 0x626879,
            Self::Grisaille => 0x585E6F,
            Self::FlintStone => 0x677283,
            Self::GrayBlue => 0x4D587A,
            Self::ChinaBlue => 0x546477,
            Self::CoastalFjord => 0x505D7E,
            Self::BijouBlue => 0x4E5E7F,
            Self::CoronetBlue => 0x59728E,
            Self::VelvetMorning => 0x60688D,
            Self::DutchBlue => 0x4A638D,
            Self::BlueHorizon => 0x4E6482,
            Self::BleachedDenim => 0x646F9B,
            Self::Marlin => 0x515B87,
            Self::BlueGranite => 0x717388,
            Self::Wedgewood => 0x6479B3,
            Self::BlueYonder => 0x5A77A8,
            Self::BlueIris => 0x5A5B9F,
            Self::VioletStorm => 0x5C619D,
            Self::AmparoBlue => 0x4960A8,
            Self::BajaBlue => 0x5F6DB0,
            Self::DazzlingBlue => 0x3850A0,
            Self::IrisBloom => 0x5B609E,
            Self::SpectrumBlue => 0x3D3C7C,
            Self::SteelGray => 0x726F70,
            Self::QuietShade => 0x66676D,
            Self::GoblinBlue => 0x5F7278,
            Self::DecemberSky => 0x767275,
            Self::RealTeal => 0x405D73,
            Self::CaptainsBlue => 0x557088,
            Self::BlueAshes => 0x3B5F78,
            Self::CopenBlue => 0x516B84,
            Self::Stellar => 0x46647E,
            Self::MoonlightBlue => 0x506886,
            Self::BeringSea => 0x4B5B6E,
            Self::FederalBlue => 0x43628B,
            Self::DeepWater => 0x266691,
            Self::VallartaBlue => 0x30658E,
            Self::ParisianBlue => 0x4F7CA4,
            Self::Regatta => 0x487AB7,
            Self::StarSapphire => 0x386192,
            Self::PalaceBlue => 0x346CB0,
            Self::Daphne => 0x0F5F9A,
            Self::NebulasBlue => 0x2D62A3,
            Self::StrongBlue => 0x1F5DA0,
            Self::MoonMist => 0x80817D,
            Self::FrenchBlue => 0x0072B5,
            Self::Campanula => 0x3272AF,
            Self::VictoriaBlue => 0x08589D,
            Self::StormyWeather => 0x58646D,
            Self::BlueMirage => 0x5C6D7C,
            Self::Bluestone => 0x577284,
            Self::ProvincialBlue => 0x5C798E,
            Self::Bluesteel => 0x35637C,
            Self::SaxonyBlue => 0x1F6680,
            Self::BlueSapphire => 0x09577B,
            Self::Faience => 0x2A6A8B,
            Self::DirectoireBlue => 0x0061A3,
            Self::BrilliantBlue => 0x0075B3,
            Self::IndigoBunting => 0x006CA9,
            Self::BlueAster => 0x0077B3,
            Self::AegeanBlue => 0x4E6E81,
            Self::SwedishBlue => 0x007EB1,
            Self::MediterraneanBlue => 0x1478A7,
            Self::Tapestry => 0x436573,
            Self::FjordBlue => 0x007290,
            Self::TurkishTile => 0x00698B,
            Self::MykonosBlue => 0x005780,
            Self::Cloisonne => 0x0075AF,
            Self::Trooper => 0x697A7E,
            Self::ColonialBlue => 0x2D6471,
            Self::CaribbeanSea => 0x00819D,
            Self::MosaicBlue => 0x00758F,
            Self::Celestial => 0x006380,
            Self::BlueJewel => 0x007BAA,
            Self::MethylBlue => 0x0074A8,
            Self::HawaiianSurf => 0x0078A7,
            Self::NorthAtlantic => 0x536D70,
            Self::TahitianTide => 0x006B7E,
            Self::StormySea => 0x6E8082,
            Self::Hydro => 0x426972,
            Self::BiscayBay => 0x097988,
            Self::HarborBlue => 0x00656E,
            Self::EnamelBlue => 0x007A8E,
            Self::TileBlue => 0x008491,
            Self::DeepLake => 0x00656B,
            Self::TropicalGreen => 0x008786,
            Self::Fanfare => 0x006D70,
            Self::Parasailing => 0x00736C,
            Self::QuetzalGreen => 0x006865,
            Self::BrushedNickel => 0x73706F,
            Self::SedonaSage => 0x686D6C,
            Self::SeaPine => 0x4C6969,
            Self::NorthSea => 0x316C6B,
            Self::Bayou => 0x20706F,
            Self::BlueGrass => 0x007C7A,
            Self::Pewter => 0x666564,
            Self::GraniteGray => 0x615E5F,
            Self::EiffelTower => 0x5C5658,
            Self::BlueSpruce => 0x486B67,
            Self::Bayberry => 0x255958,
            Self::AlpineGreen => 0x005F56,
            Self::UltramarineGreen => 0x006B54,
            Self::SilverPine => 0x4E6866,
            Self::AntiqueGreen => 0x29675C,
            Self::CadmiumGreen => 0x00675B,
            Self::BalsamGreen => 0x576664,
            Self::BrittanyBlue => 0x4C7E86,
            Self::DarkForest => 0x556962,
            Self::SagebrushGreen => 0x567572,
            Self::PosyGreen => 0x325B51,
            Self::DeepJungle => 0x36716F,
            Self::Tidepool => 0x0A6F69,
            Self::Ivy => 0x226C63,
            Self::Fir => 0x3A725F,
            Self::FrostySpruce => 0x578270,
            Self::ShadyGlade => 0x006E5B,
            Self::Bosphorus => 0x007558,
            Self::GolfGreen => 0x008763,
            Self::SmokePine => 0x3E6257,
            Self::GalapagosGreen => 0x29685F,
            Self::AgaveGreen => 0x6B7169,
            Self::PepperGreen => 0x007D60,
            Self::LushMeadow => 0x006E51,
            Self::GardenTopiary => 0x3E524B,
            Self::DuckGreen => 0x53665C,
            Self::FoliageGreen => 0x3E6F58,
            Self::Leprechaun => 0x378661,
            Self::Amazon => 0x1F7349,
            Self::JollyGreen => 0x007844,
            Self::Myrtle => 0x4F6B58,
            Self::Comfrey => 0x5B7961,
            Self::Fairway => 0x477050,
            Self::Juniper => 0x3D7245,
            Self::Raven => 0x413E3D,
            Self::Asphalt => 0x434447,
            Self::GrayPinstripe => 0x49494D,
            Self::GardenGreen => 0x495E35,
            Self::JetBlack => 0x2D2C2F,
            Self::ClimbingIvy => 0x444940,
            Self::Thyme => 0x50574C,
            Self::Beetle => 0x55584C,
            Self::BlackForest => 0x414F3C,
            Self::Chive => 0x4A5335,
            Self::SeaTurtle => 0x5E5749,
            Self::Beluga => 0x4A4843,
            Self::DeepDepths => 0x46483C,
            Self::ForestNight => 0x434237,
            Self::DuffelBag => 0x394034,
            Self::KombuGreen => 0x3A4032,
            Self::RifleGreen => 0x414832,
            Self::BlackInk => 0x44413C,
            Self::Peat => 0x3B3A36,
            Self::Rosin => 0x36362D,
            Self::Kalamata => 0x5F5B4C,
            Self::GrapeLeaf => 0x545144,
            Self::IvyGreen => 0x585442,
            Self::OliveNight => 0x535040,
            Self::DarkOlive => 0x574D35,
            Self::BlackOlive => 0x48413B,
            Self::Wren => 0x4A4139,
            Self::Teak => 0x655341,
            Self::Beech => 0x5B4F3B,
            Self::MilitaryOlive => 0x63563B,
            Self::Demitasse => 0x40342B,
            Self::Morel => 0x685C53,
            Self::ChocolateChip => 0x685A4E,
            Self::MajorBrown => 0x5B5149,
            Self::TurkishCoffee => 0x483F39,
            Self::SlateBlack => 0x4B3D33,
            Self::DesertPalm => 0x5A4632,
            Self::Canteen => 0x5E5347,
            Self::Tarmac => 0x5A5348,
            Self::Crocodile => 0x5D5348,
            Self::Delicioso => 0x3F352F,
            Self::Mulch => 0x433937,
            Self::ChocolateBrown => 0x4E403B,
            Self::CoffeeBean => 0x40312F,
            Self::RainDrum => 0x5F4C40,
            Self::FrenchRoast => 0x58423F,
            Self::Bracken => 0x4F3F3B,
            Self::Java => 0x433331,
            Self::Ganache => 0x34292A,
            Self::DarkEarth => 0x5C4939,
            Self::Breen => 0x795D34,
            Self::AfterDark => 0x3C3535,
            Self::Licorice => 0x3A3536,
            Self::Espresso => 0x363031,
            Self::Mole => 0x392D2B,
            Self::ChocolateTorte => 0x382E2D,
            Self::ChocolatePlum => 0x3C2D2E,
            Self::BlackCoffee => 0x3B302F,
            Self::Carafe => 0x5D473A,
            Self::Chestnut => 0x584039,
            Self::Pinecone => 0x61473B,
            Self::ShoppingBag => 0x5A4743,
            Self::ChocolateLab => 0x5C3E35,
            Self::ShavedChocolate => 0x543B35,
            Self::Mustang => 0x684B40,
            Self::PottingSoil => 0x54392D,
            Self::Cappuccino => 0x633F33,
            Self::FondueFudge => 0x5D4236,
            Self::RootBeer => 0x714A41,
            Self::FriarBrown => 0x6E493A,
            Self::Tiramisu => 0x634235,
            Self::RockyRoad => 0x5A3E36,
            Self::Brunette => 0x664238,
            Self::TortoiseShell => 0x754734,
            Self::ArabianSpice => 0x884332,
            Self::Picante => 0x8D3F2D,
            Self::SealBrown => 0x493B39,
            Self::BitterChocolate => 0x503130,
            Self::Sable => 0x6E403C,
            Self::RumRaisin => 0x583432,
            Self::BrownStone => 0x593C39,
            Self::HotChocolate => 0x683B39,
            Self::Andorra => 0x603535,
            Self::MadderBrown => 0x6A3331,
            Self::Sequoia => 0x804839,
            Self::Henna => 0x7C423C,
            Self::FiredBrick => 0x6A2E2A,
            Self::RussetBrown => 0x743332,
            Self::ChicoryCoffee => 0x4A342E,
            Self::DeepMahogany => 0x553B39,
            Self::SmokedPaprika => 0x6E362C,
            Self::Mink => 0x734B42,
            Self::Fudgesickle => 0x63403A,
            Self::ChocolateFondant => 0x56352D,
            Self::CherryMahogany => 0x66352B,
            Self::Cinnamon => 0x6B4139,
            Self::Puce => 0x503938,
            Self::RedMahogany => 0x60373D,
            Self::Zinfandel => 0x5C2935,
            Self::OxbloodRed => 0x70393F,
            Self::Port => 0x663336,
            Self::ChocolateTruffle => 0x612E35,
            Self::WindsorWine => 0x582B36,
            Self::BurntRusset => 0x7E3940,
            Self::SunDriedTomato => 0x752329,
            Self::Rosewood => 0x813639,
            Self::Cowhide => 0x884344,
            Self::Merlot => 0x72262C,
            Self::Syrah => 0x6A282C,
            Self::RedPear => 0x7B3539,
            Self::BurntHenna => 0x7E392F,
            Self::BrickRed => 0x8C373E,
            Self::RedDahlia => 0x7D2027,
            Self::ChiliPepper => 0x9B1B30,
            Self::ScarletSage => 0x9D202F,
            Self::Raisin => 0x524144,
            Self::PrunePurple => 0x5C3A4D,
            Self::Burgundy => 0x64313E,
            Self::Fudge => 0x493338,
            Self::Huckleberry => 0x5B4349,
            Self::CatawbaGrape => 0x5D3C43,
            Self::VineyardWine => 0x58363D,
            Self::Sassafras => 0x54353B,
            Self::DecadentChocolate => 0x513235,
            Self::PortRoyale => 0x502B33,
            Self::RubyWine => 0x77333B,
            Self::BikingRed => 0x77212E,
            Self::Rhubarb => 0x77202F,
            Self::Garnet => 0x953640,
            Self::RioRed => 0x8A2232,
            Self::Samba => 0xA2242F,
            Self::RibbonRed => 0xB92636,
            Self::TrueRed => 0xBF1932,
            Self::MauveWine => 0x5B3644,
            Self::Fig => 0x532D3B,
            Self::Cabernet => 0x64242E,
            Self::TawnyPort => 0x5C2C35,
            Self::Cordovan => 0x702F3B,
            Self::BarbadosCherry => 0xAA0A27,
            Self::HauteRed => 0xA11729,
            Self::AmericanBeauty => 0xA73340,
            Self::Scarlet => 0xBC2B3D,
            Self::TangoRed => 0xAC0E2E,
            Self::Crimson => 0xAE0E36,
            Self::RacingRed => 0xBD162C,
            Self::LipstickRed => 0xB31A38,
            Self::DeepClaret => 0x973443,
            Self::RedBud => 0x962D49,
            Self::PersianRed => 0xA21441,
            Self::JesterRed => 0x9E1030,
            Self::Scooter => 0x941E32,
            Self::Pomegranate => 0x6C2831,
            Self::TibetanRed => 0x782A39,
            Self::RumbaRed => 0x7C2439,
            Self::Cerise => 0xA41247,
            Self::Hortensia => 0x553B50,
            Self::Prune => 0x603749,
            Self::Rhododendron => 0x722B3F,
            Self::RedPlum => 0x7C2946,
            Self::BeetRed => 0x7A1F3D,
            Self::Anemone => 0x842C48,
            Self::Granita => 0xA52350,
            Self::CherriesJubilee => 0xA22452,
            Self::Vivacious => 0xA32857,
            Self::Sangria => 0x982551,
            Self::Winetasting => 0x492A34,
            Self::Eggplant => 0x613F4C,
            Self::CrushedViolets => 0x643A4C,
            Self::GrapeWine => 0x5A2F43,
            Self::Amaranth => 0x6F3C56,
            Self::PickledBeet => 0x4D233D,
            Self::MagentaPurple => 0x6B264B,
            Self::PlumCaspia => 0x61224A,
            Self::PurplePotion => 0x692746,
            Self::Boysenberry => 0x85325C,
            Self::RaspberryRadiance => 0x802A50,
            Self::FestivalFuchsia => 0x9E2C6A,
            Self::ItalianPlum => 0x533146,
            Self::PotentPurple => 0x462639,
            Self::DarkPurple => 0x582147,
            Self::WinterBloom => 0x47243B,
            Self::WildAster => 0x92316F,
            Self::Wineberry => 0x5A395B,
            Self::BlackberryWine => 0x4D3246,
            Self::Phlox => 0x692D5D,
            Self::Hollyhock => 0x823270,
            Self::Gloxinia => 0x622E5A,
            Self::Byzantium => 0x853B7B,
            Self::Indigo => 0x4C3957,
            Self::ShadowPurple => 0x4E334E,
            Self::PlumPurple => 0x51304E,
            Self::Plum => 0x5A315D,
            Self::PurplePassion => 0x683D62,
            Self::GrapeJuice => 0x682961,
            Self::PlumPerfect => 0x473442,
            Self::DeepPurple => 0x50314C,
            Self::WoodViolet => 0x75406A,
            Self::SparklingGrape => 0x773376,
            Self::SunsetPurple => 0x6F456E,
            Self::SpicedPlum => 0x6D4773,
            Self::BrightViolet => 0x784384,
            Self::Majesty => 0x593761,
            Self::GrapeRoyale => 0x4F2D54,
            Self::PurplePennant => 0x432C47,
            Self::BlackberryCordial => 0x3F2A47,
            Self::MeadowViolet => 0x764F82,
            Self::ImperialPurple => 0x542C5D,
            Self::AmaranthPurple => 0x6A397B,
            Self::PurpleMagic => 0x663271,
            Self::Pansy => 0x653D7C,
            Self::Mysterioso => 0x46394B,
            Self::SweetGrape => 0x4B3B4F,
            Self::PurpleReign => 0x56456B,
            Self::Loganberry => 0x5A4769,
            Self::Acai => 0x46295A,
            Self::Petunia => 0x4F3466,
            Self::TillandsiaPurple => 0x563474,
            Self::CrownJewel => 0x482D54,
            Self::RoyalPurple => 0x603F83,
            Self::Nightshade => 0x433748,
            Self::DeepWell => 0x2C2A33,
            Self::NavyCosmos => 0x503B53,
            Self::PurplePlumeria => 0x473854,
            Self::GothicGrape => 0x473951,
            Self::MulberryPurple => 0x493C62,
            Self::PurpleVelvet => 0x41354D,
            Self::Grape => 0x433455,
            Self::GentianViolet => 0x544275,
            Self::ParachutePurple => 0x392852,
            Self::Heliotrope => 0x4F3872,
            Self::PrismViolet => 0x53357D,
            Self::VioletIndigo => 0x3E285C,
            Self::PlumKitten => 0x625B5C,
            Self::Eclipse => 0x343148,
            Self::EveningBlue => 0x2A293E,
            Self::AstralAura => 0x363151,
            Self::MaritimeBlue => 0x27293D,
            Self::NavyBlue => 0x403F6F,
            Self::BlueRibbon => 0x3A395F,
            Self::DeepWisteria => 0x443F6F,
            Self::DeepBlue => 0x44377D,
            Self::Liberty => 0x4D448A,
            Self::MazarineBlue => 0x273C76,
            Self::Pavement => 0x524D50,
            Self::Magnet => 0x4D4B4F,
            Self::Obsidian => 0x3A363B,
            Self::Shale => 0x4A3F41,
            Self::Rabbit => 0x5F575C,
            Self::DarkShadow => 0x4A4B4D,
            Self::ForgedIron => 0x48464A,
            Self::NineIron => 0x46434A,
            Self::BlackBean => 0x2E272A,
            Self::IronGate => 0x4E5055,
            Self::BlackBeauty => 0x26262A,
            Self::Graystone => 0x4D495B,
            Self::BlackenedPearl => 0x4D4B50,
            Self::Periscope => 0x46444C,
            Self::NightshadowBlue => 0x4E5368,
            Self::Peacoat => 0x2B2E43,
            Self::BlackIris => 0x2B3042,
            Self::SkyCaptain => 0x262934,
            Self::NavyBlazer => 0x282D3C,
            Self::NightSky => 0x2A2A35,
            Self::PatriotBlue => 0x363756,
            Self::CrownBlue => 0x464B65,
            Self::Graphite => 0x3B3B48,
            Self::BlueIndigo => 0x49516D,
            Self::VintageIndigo => 0x4A556B,
            Self::OdysseyGray => 0x434452,
            Self::MedievalBlue => 0x29304E,
            Self::DeepCobalt => 0x404466,
            Self::SkipperBlue => 0x484A72,
            Self::TwilightBlue => 0x313D64,
            Self::Blueprint => 0x2D3359,
            Self::BlueDepths => 0x263056,
            Self::OrientBlue => 0x47457A,
            Self::DeepUltramarine => 0x384883,
            Self::ClematisBlue => 0x363B7C,
            Self::SurfTheWeb => 0x203C7F,
            Self::SodaliteBlue => 0x253668,
            Self::RoyalBlue => 0x3D428B,
            Self::MonacoBlue => 0x274374,
            Self::BlackOnyx => 0x2B272B,
            Self::TapShoe => 0x2A2B2D,
            Self::StretchLimo => 0x2B2C30,
            Self::Caviar => 0x292A2D,
            Self::Anthracite => 0x28282D,
            Self::Meteorite => 0x2B2929,
            Self::OuterSpace => 0x2F3441,
            Self::TotalEclipse => 0x2C313D,
            Self::Salute => 0x282B34,
            Self::Carbon => 0x272F38,
            Self::DarkNavy => 0x232F36,
            Self::OmbreBlue => 0x434854,
            Self::BlueGraphite => 0x323137,
            Self::IndiaInk => 0x3C3F4A,
            Self::DarkSapphire => 0x262B37,
            Self::Blueberry => 0x2C333E,
            Self::ParisianNight => 0x323441,
            Self::BlueNights => 0x363B48,
            Self::DressBlues => 0x2A3244,
            Self::MoodIndigo => 0x353A4C,
            Self::EnsignBlue => 0x384C67,
            Self::EstateBlue => 0x233658,
            Self::InsigniaBlue => 0x2F3E55,
            Self::NavyPeony => 0x223A5E,
            Self::TrueNavy => 0x3F5277,
            Self::SargassoSea => 0x35435A,
            Self::Poseidon => 0x123955,
            Self::SailorBlue => 0x0E3A53,
            Self::DarkBlue => 0x305679,
            Self::BrightCobalt => 0x385D8D,
            Self::GibraltarSea => 0x123850,
            Self::Delft => 0x3D5E8C,
            Self::Limoges => 0x243F6C,
            Self::LapisBlue => 0x004B8D,
            Self::BaleineBlue => 0x155187,
            Self::SnorkelBlue => 0x034F84,
            Self::NauticalBlue => 0x1A5091,
            Self::ClassicBlue => 0x0F4C81,
            Self::TurkishSea => 0x195190,
            Self::GalaxyBlue => 0x2A4B7C,
            Self::OlympianBlue => 0x1A4C8B,
            Self::TrueBlue => 0x1E4477,
            Self::Ebony => 0x41424A,
            Self::MidnightNavy => 0x34414E,
            Self::DarkDenim => 0x35465E,
            Self::BlueOpal => 0x0F3B57,
            Self::BlueWingTeal => 0x2C4053,
            Self::MoonlitOcean => 0x293B4D,
            Self::MajolicaBlue => 0x274357,
            Self::DeepDive => 0x29495C,
            Self::Midnight => 0x325B74,
            Self::PrincessBlue => 0x00539C,
            Self::Skydiver => 0x00589B,
            Self::MoonlessNight => 0x2F2D30,
            Self::Phantom => 0x39373B,
            Self::Turbulence => 0x4E545B,
            Self::Vulcan => 0x2D3036,
            Self::DarkSlate => 0x46515A,
            Self::IndianTeal => 0x3C586B,
            Self::OrionBlue => 0x3E4F5C,
            Self::InkBlue => 0x0B5369,
            Self::MoroccanBlue => 0x0F4E67,
            Self::ImperialBlue => 0x005A92,
            Self::PirateBlack => 0x363838,
            Self::Stargazer => 0x39505C,
            Self::MallardBlue => 0x3A5C6E,
            Self::LegionBlue => 0x1F495B,
            Self::ReflectingPond => 0x203E4A,
            Self::Corsair => 0x18576C,
            Self::LyonsBlue => 0x005871,
            Self::Seaport => 0x005E7D,
            Self::Mediterranea => 0x32575D,
            Self::ShadedSpruce => 0x00585E,
            Self::BlueCoral => 0x1B5366,
            Self::OceanDepths => 0x006175,
            Self::CrystalTeal => 0x00637C,
            Self::DeepLagoon => 0x005265,
            Self::AtlanticDeep => 0x274E55,
            Self::MallardGreen => 0x405E5C,
            Self::Balsam => 0x33565E,
            Self::Dragonfly => 0x2A5C6A,
            Self::GreenGables => 0x324241,
            Self::DeepTeal => 0x18454B,
            Self::Pacific => 0x1F595C,
            Self::TealGreen => 0x006361,
            Self::UrbanChic => 0x464E4D,
            Self::SeaMoss => 0x254445,
            Self::DarkestSpruce => 0x303D3C,
            Self::Storm => 0x035453,
            Self::BotanicalGarden => 0x12403C,
            Self::Everglade => 0x005B5D,
            Self::ForestBiome => 0x184A45,
            Self::RainForest => 0x15463E,
            Self::PonderosaPine => 0x203B3D,
            Self::Scarab => 0x23312D,
            Self::PineGrove => 0x213631,
            Self::BistroGreen => 0x395551,
            Self::TrekkingGreen => 0x355048,
            Self::Jasper => 0x335959,
            Self::JuneBug => 0x264A48,
            Self::Evergreen => 0x11574A,
            Self::HunterGreen => 0x335749,
            Self::DarkGreen => 0x314F40,
            Self::Cilantro => 0x43544B,
            Self::JetSet => 0x262C2A,
            Self::JungleGreen => 0x3C4E47,
            Self::Sycamore => 0x35463D,
            Self::MountainView => 0x2E3D30,
            Self::Pineneedle => 0x334D41,
            Self::VerdantGreen => 0x12674A,
            Self::Eden => 0x264E36,
            Self::DeepForest => 0x37413A,
            Self::GreenerPastures => 0x37503D,
        }
    }

    /// Whether `s` is one of the accepted spellings of this color:
    /// the canonical name, a bare code, a code with any
    /// [`PantoneFhiSuffix`] or the bare color name.
    fn accepts(&self, s: &str) -> bool {
        if s.eq_ignore_ascii_case(self.as_str()) || s.eq_ignore_ascii_case(self.name()) || s.eq_ignore_ascii_case(self.code()) {
            return true;
        }
        let Some(suffix) = s.strip_prefix(self.code()).and_then(|rest| rest.strip_prefix(' ')) else {
            return false;
        };
        suffix.parse::<PantoneFhiSuffix>().is_ok()
    }
}

impl FromStr for PantoneFhiNamedColor {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .iter()
            .find(|color| color.accepts(s))
            .copied()
            .ok_or_else(|| ParseColorError::UnknownPantoneFhiColor { input: s.into() })
    }
}

impl Display for PantoneFhiNamedColor {
    /// Formats the canonical `Pantone <code> TCX <name>` string.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<PantoneFhiNamedColor> for Color<u8> {
    fn from(color: PantoneFhiNamedColor) -> Self {
        color.to_color()
    }
}

impl From<PantoneFhiNamedColor> for Color<f64> {
    fn from(color: PantoneFhiNamedColor) -> Self {
        color.to_color().into()
    }
}

impl From<PantoneFhiNamedColor> for Color<f32> {
    fn from(color: PantoneFhiNamedColor) -> Self {
        color.to_color().into()
    }
}

impl From<PantoneFhiNamedColor> for HexColor {
    fn from(color: PantoneFhiNamedColor) -> Self {
        Self::from(color.to_color())
    }
}

/// The wire format is the canonical `Pantone <code> TCX <name>` string.
impl Serialize for PantoneFhiNamedColor {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// Deserialization accepts all spellings of [`FromStr`].
impl<'de> Deserialize<'de> for PantoneFhiNamedColor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "zbus")]
impl zvariant::Type for PantoneFhiNamedColor {
    const SIGNATURE: &'static zvariant::Signature = <str as zvariant::Type>::SIGNATURE;
}

#[cfg(feature = "async-graphql")]
async_graphql::scalar!(PantoneFhiNamedColor);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_covers_the_2310_colors() {
        assert_eq!(PantoneFhiNamedColor::ALL.len(), 2310);
    }

    #[test]
    fn all_is_sorted_by_code() {
        let names: Vec<&str> = PantoneFhiNamedColor::ALL.iter().map(PantoneFhiNamedColor::as_str).collect();
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(names, sorted);
    }

    #[test]
    fn code_and_name_split_the_canonical_string() {
        assert_eq!(PantoneFhiNamedColor::TrueBlue.code(), "19-4057");
        assert_eq!(PantoneFhiNamedColor::TrueBlue.name(), "True Blue");
        assert_eq!(PantoneFhiNamedColor::BrightWhite.code(), "11-0601");
        assert_eq!(PantoneFhiNamedColor::JetBlack.name(), "Jet Black");
    }

    #[test]
    fn from_str_is_case_insensitive() {
        assert_eq!("Pantone 19-4057 TCX True Blue".parse::<PantoneFhiNamedColor>().unwrap(), PantoneFhiNamedColor::TrueBlue);
        assert_eq!("pantone 19-4057 tcx true blue".parse::<PantoneFhiNamedColor>().unwrap(), PantoneFhiNamedColor::TrueBlue);
    }

    #[test]
    fn from_str_accepts_bare_forms_and_other_suffixes() {
        assert_eq!("19-4057".parse::<PantoneFhiNamedColor>().unwrap(), PantoneFhiNamedColor::TrueBlue);
        assert_eq!("19-4057 TPX".parse::<PantoneFhiNamedColor>().unwrap(), PantoneFhiNamedColor::TrueBlue);
        assert_eq!("19-4057 tpg".parse::<PantoneFhiNamedColor>().unwrap(), PantoneFhiNamedColor::TrueBlue);
        assert_eq!("True Blue".parse::<PantoneFhiNamedColor>().unwrap(), PantoneFhiNamedColor::TrueBlue);
        assert_eq!("turquoise".parse::<PantoneFhiNamedColor>().unwrap(), PantoneFhiNamedColor::Turquoise);
    }

    #[test]
    fn display_and_as_str_produce_the_canonical_name() {
        assert_eq!(PantoneFhiNamedColor::TrueBlue.to_string(), "Pantone 19-4057 TCX True Blue");
        assert_eq!(PantoneFhiNamedColor::JetBlack.as_str(), "Pantone 19-0303 TCX Jet Black");
    }

    #[test]
    fn display_from_str_round_trip_covers_all_colors() {
        for color in PantoneFhiNamedColor::ALL {
            assert_eq!(color.to_string().parse::<PantoneFhiNamedColor>().unwrap(), *color);
        }
    }

    #[test]
    fn to_color_maps_names_to_srgb() {
        assert_eq!(PantoneFhiNamedColor::TrueBlue.to_color(), Color::<u8>::new(0x1E, 0x44, 0x77));
        assert_eq!(PantoneFhiNamedColor::Turquoise.to_color(), Color::<u8>::new(0x45, 0xB5, 0xAA));
    }

    #[test]
    fn nearest_finds_the_closest_named_color() {
        assert_eq!(PantoneFhiNamedColor::nearest(&Color::<u8>::new(0x1E, 0x44, 0x77)), PantoneFhiNamedColor::TrueBlue);
        assert_eq!(PantoneFhiNamedColor::nearest(&Color::<u8>::new(0xFF, 0xFF, 0xFF)), PantoneFhiNamedColor::BrightWhite);
    }

    #[test]
    fn suffix_is_the_canonical_cotton_guide() {
        assert_eq!(PantoneFhiNamedColor::TrueBlue.suffix(), PantoneFhiSuffix::Cotton);
        assert!(PantoneFhiNamedColor::ALL.iter().all(|color| color.suffix() == PantoneFhiSuffix::Cotton));
    }

    #[test]
    fn unknown_names_are_rejected() {
        assert!("Pantone 99-9999 TCX Nope".parse::<PantoneFhiNamedColor>().is_err());
        assert!("19-4057 XX".parse::<PantoneFhiNamedColor>().is_err());
        assert_eq!(
            "red".parse::<PantoneFhiNamedColor>().unwrap_err(),
            ParseColorError::UnknownPantoneFhiColor { input: "red".into() }
        );
    }

    #[test]
    fn serde_round_trip() {
        let json = serde_json::to_string(&PantoneFhiNamedColor::TrueBlue).unwrap();
        assert_eq!(json, "\"Pantone 19-4057 TCX True Blue\"");
        assert_eq!(serde_json::from_str::<PantoneFhiNamedColor>("\"19-4057\"").unwrap(), PantoneFhiNamedColor::TrueBlue);
    }

    #[cfg(feature = "zbus")]
    #[test]
    fn zvariant_round_trip() {
        assert_eq!(<PantoneFhiNamedColor as zvariant::Type>::SIGNATURE.to_string(), "s");
        let context = zvariant::serialized::Context::new_dbus(zvariant::LE, 0);
        let bytes = zvariant::to_bytes(context, &PantoneFhiNamedColor::TrueBlue).unwrap();
        let decoded: PantoneFhiNamedColor = bytes.deserialize().unwrap().0;
        assert_eq!(decoded, PantoneFhiNamedColor::TrueBlue);
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn json_schema_lists_the_canonical_names() {
        let schema = serde_json::to_value(schemars::schema_for!(PantoneFhiNamedColor)).unwrap();
        let schema = schema.to_string();
        assert!(schema.contains("\"Pantone 19-4057 TCX True Blue\""));
        assert!(schema.contains("\"Pantone 15-5519 TCX Turquoise\""));
    }

    #[cfg(feature = "async-graphql")]
    #[test]
    fn graphql_scalar_round_trip() {
        use async_graphql::ScalarType;
        let parsed = PantoneFhiNamedColor::parse(PantoneFhiNamedColor::TrueBlue.to_value()).unwrap();
        assert_eq!(parsed, PantoneFhiNamedColor::TrueBlue);
    }
}
