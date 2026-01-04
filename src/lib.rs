#![cfg_attr(not(test), no_std)]
#![warn(clippy::pedantic)]
#![allow(clippy::enum_variant_names)]

extern crate alloc;

use alloc::{borrow::Cow, vec::Vec};
use core::num::{NonZeroU8, NonZeroU16};

use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use url::Url;

/// Inlined version of <https://docs.rs/time/latest/time/serde/macro.format_description.html> to allow for this crate to be `#![no_std]`.
mod reldate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    deny_unknown_fields,
    tag = "card_type",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
enum Card<'a> {
    Pokemon(#[serde(borrow)] Pokemon<'a>),
    Trainer(#[serde(borrow)] Trainer<'a>),
    Energy(#[serde(borrow)] Energy<'a>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Pokemon<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    subtitle: Option<Cow<'a, str>>,
    lang: Lang,
    #[serde(skip_serializing_if = "Option::is_none")]
    foil: Option<Foil>,
    size: CardSize,
    back: CardBack,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    artists: Option<Artists<'a>>,
    regulation_mark: RegulationMark,
    #[serde(borrow)]
    set_icon: Cow<'a, str>,
    #[serde(borrow)]
    collector_number: CollectorNumber<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rarity: Option<Rarity>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    copyright: Option<Copyright<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CardTag>>,
    stage: Stage,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    stage_text: Option<Cow<'a, str>>,
    hp: NonZeroU16,
    #[serde(skip_serializing_if = "Option::is_none")]
    weakness: Option<Weakness>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resistance: Option<Resistance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retreat: Option<u8>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    flavor_text: Option<Cow<'a, str>>,
    #[serde(borrow)]
    text: Vec<Text<'a>>,
    #[serde(borrow)]
    ext: Ext<'a>,
    images: Images,
    types: Vec<EnergyType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "subtype")]
pub enum Trainer<'a> {
    Item(#[serde(borrow)] Item<'a>),
    Supporter(#[serde(borrow)] Supporter<'a>),
    Tool(#[serde(borrow)] Tool<'a>),
    Stadium(#[serde(borrow)] Stadium<'a>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Item<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    lang: Lang,
    #[serde(skip_serializing_if = "Option::is_none")]
    foil: Option<Foil>,
    size: CardSize,
    back: CardBack,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    artists: Option<Artists<'a>>,
    regulation_mark: RegulationMark,
    #[serde(borrow)]
    set_icon: Cow<'a, str>,
    #[serde(borrow)]
    collector_number: CollectorNumber<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rarity: Option<Rarity>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    copyright: Option<Copyright<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CardTag>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    stage_text: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hp: Option<NonZeroU16>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    flavor_text: Option<Cow<'a, str>>,
    #[serde(borrow)]
    text: Vec<Text<'a>>,
    #[serde(borrow)]
    ext: Ext<'a>,
    images: Images,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Supporter<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    subtitle: Option<Cow<'a, str>>,
    lang: Lang,
    #[serde(skip_serializing_if = "Option::is_none")]
    foil: Option<Foil>,
    size: CardSize,
    back: CardBack,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    artists: Option<Artists<'a>>,
    regulation_mark: RegulationMark,
    #[serde(borrow)]
    set_icon: Cow<'a, str>,
    #[serde(borrow)]
    collector_number: CollectorNumber<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rarity: Option<Rarity>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    copyright: Option<Copyright<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CardTag>>,
    #[serde(borrow)]
    text: Vec<Text<'a>>,
    #[serde(borrow)]
    ext: Ext<'a>,
    images: Images,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tool<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    lang: Lang,
    #[serde(skip_serializing_if = "Option::is_none")]
    foil: Option<Foil>,
    size: CardSize,
    back: CardBack,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    artists: Option<Artists<'a>>,
    regulation_mark: RegulationMark,
    #[serde(borrow)]
    set_icon: Cow<'a, str>,
    #[serde(borrow)]
    collector_number: CollectorNumber<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rarity: Option<Rarity>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    copyright: Option<Copyright<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CardTag>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    flavor_text: Option<Cow<'a, str>>,
    #[serde(borrow)]
    text: Vec<Text<'a>>,
    #[serde(borrow)]
    ext: Ext<'a>,
    images: Images,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stadium<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    lang: Lang,
    #[serde(skip_serializing_if = "Option::is_none")]
    foil: Option<Foil>,
    size: CardSize,
    back: CardBack,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    artists: Option<Artists<'a>>,
    regulation_mark: RegulationMark,
    #[serde(borrow)]
    set_icon: Cow<'a, str>,
    #[serde(borrow)]
    collector_number: CollectorNumber<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rarity: Option<Rarity>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    copyright: Option<Copyright<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CardTag>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    flavor_text: Option<Cow<'a, str>>,
    #[serde(borrow)]
    text: Vec<Text<'a>>,
    #[serde(borrow)]
    ext: Ext<'a>,
    images: Images,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, tag = "subtype")]
pub enum Energy<'a> {
    #[serde(rename = "BASIC")]
    Basic(#[serde(borrow)] BasicEnergy<'a>),
    #[serde(rename = "SPECIAL")]
    Special(#[serde(borrow)] SpecialEnergy<'a>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BasicEnergy<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    lang: Lang,
    #[serde(skip_serializing_if = "Option::is_none")]
    foil: Option<Foil>,
    size: CardSize,
    back: CardBack,
    #[serde(borrow)]
    set_icon: Cow<'a, str>,
    #[serde(borrow)]
    collector_number: CollectorNumber<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rarity: Option<Rarity>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    copyright: Option<Copyright<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CardTag>>,
    #[serde(borrow)]
    ext: Ext<'a>,
    images: Images,
    types: Vec<EnergyType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpecialEnergy<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    lang: Lang,
    #[serde(skip_serializing_if = "Option::is_none")]
    foil: Option<Foil>,
    size: CardSize,
    back: CardBack,
    regulation_mark: RegulationMark,
    #[serde(borrow)]
    set_icon: Cow<'a, str>,
    #[serde(borrow)]
    collector_number: CollectorNumber<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rarity: Option<Rarity>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    copyright: Option<Copyright<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CardTag>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    flavor_text: Option<Cow<'a, str>>,
    #[serde(borrow)]
    text: Vec<Text<'a>>,
    #[serde(borrow)]
    ext: Ext<'a>,
    images: Images,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Lang {
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "fr-FR")]
    FrFr,
    #[serde(rename = "it-IT")]
    ItIt,
    #[serde(rename = "de-DE")]
    DeDe,
    #[serde(rename = "es-ES")]
    EsEs,
    #[serde(rename = "es-419")]
    Es419,
    #[serde(rename = "pt-BR")]
    PtBr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Foil {
    #[serde(rename = "type")]
    ty: FoilType,
    mask: FoilMask,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FoilType {
    Stamped,
    Cosmos,
    FlatSilver,
    SunPillar,
    SvHolo,
    SvUltra,
    SvUltraScodix,
    AceFoil,
    Rainbow,
    CrackedIce,
    UltraGoldRainbow,
    Tinsel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FoilMask {
    Stamped,
    Reverse,
    Holo,
    Etched,
    ColdFoilEtched,
    CastAndCure,
    ReverseLaminate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardSize {
    Standard,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CardBack {
    #[serde(rename = "POKEMON_1999")]
    Pokemon1999,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Artists<'a> {
    #[serde(borrow)]
    text: Cow<'a, str>,
    #[serde(borrow)]
    list: Vec<Cow<'a, str>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum RegulationMark {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CollectorNumber<'a> {
    #[serde(borrow)]
    full: Cow<'a, str>,
    #[serde(borrow)]
    numerator: Cow<'a, str>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    denominator: Option<Cow<'a, str>>,
    numeric: NonZeroU16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Rarity {
    designation: RarityDesignation,
    icon: RarityIcon,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RarityDesignation {
    Common,
    Uncommon,
    Rare,
    DoubleRare,
    IllustrationRare,
    SpecialIllustrationRare,
    UltraRare,
    HyperRare,
    ShinyRare,
    ShinyUltraRare,
    Promo,
    AceSpecRare,
    MegaHyperRare,
    BlackWhiteRare,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RarityIcon {
    SolidCircle,
    SolidDiamond,
    SolidStar,
    TwoBlackStars,
    GoldStar,
    TwoGoldStars,
    ThreeGoldStars,
    TwoSilverStars,
    ShinyStar,
    TwoShinyStars,
    BlackStarPromo,
    PinkStar,
    FourPointStar,
    BlackWhiteStars,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Copyright<'a> {
    #[serde(borrow)]
    text: Cow<'a, str>,
    // REVIEW: Enum?
    year: u16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardTag {
    ExLower,
    Tera,
    Item,
    Tool,
    PlayableTrainer,
    Future,
    Ancient,
    Shiny,
    AceSpec,
    TrainersPokemon,
    MegaEvolution,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Stage {
    Basic,
    Stage1,
    Stage2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Weakness {
    amount: NonZeroU8,
    operator: WeaknessOperator,
    types: Vec<EnergyType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum WeaknessOperator {
    #[serde(rename = "+")]
    Add,
    #[serde(rename = "×")]
    Multiply,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Resistance {
    amount: u8,
    operator: ResistanceOperator,
    types: Vec<EnergyType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ResistanceOperator {
    #[serde(rename = "-")]
    Subtract,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ext<'a> {
    #[serde(borrow)]
    tcgl: Tcgl<'a>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Tcgl<'a> {
    #[serde(rename = "archetypeID", with = "crate::u32_hex")]
    archetype_id: u32,
    #[serde(rename = "cardID")]
    #[serde(borrow)]
    card_id: Cow<'a, str>,
    #[serde(borrow)]
    key: Cow<'a, str>,
    #[serde(rename = "longFormID")]
    #[serde(borrow)]
    long_form_id: Cow<'a, str>,
    #[serde(with = "reldate")]
    reldate: PrimitiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Images {
    tcgl: TcglImages,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TcglImages {
    jpg: ImageJpg,
    png: ImagePng,
    tex: ImageTex,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ImageJpg {
    front: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ImagePng {
    front: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    foil: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etch: Option<Url>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ImageTex {
    front: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    foil: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etch: Option<Url>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Damage {
    // REVIEW: Multiple of 10, nonzero?
    amount: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<DamageSuffix>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum DamageSuffix {
    #[serde(rename = "+")]
    Add,
    #[serde(rename = "-")]
    Subtract,
    #[serde(rename = "×")]
    Multiply,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
enum Text<'a> {
    Attack(#[serde(borrow)] Attack<'a>),
    Ability(#[serde(borrow)] Ability<'a>),
    RuleBox(#[serde(borrow)] RuleBox<'a>),
    Effect(#[serde(borrow)] Effect<'a>),
    TextBox(#[serde(borrow)] TextBox<'a>),
    Reminder(#[serde(borrow)] Reminder<'a>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Attack<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    text: Option<Cow<'a, str>>,
    cost: Vec<AttackCost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    damage: Option<Damage>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Ability<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    #[serde(borrow)]
    text: Cow<'a, str>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RuleBox<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    #[serde(borrow)]
    text: Cow<'a, str>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Effect<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    #[serde(borrow)]
    text: Cow<'a, str>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TextBox<'a> {
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
    #[serde(borrow)]
    text: Cow<'a, str>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Reminder<'a> {
    #[serde(borrow)]
    text: Cow<'a, str>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
enum EnergyType {
    Grass,
    Fire,
    Water,
    Lightning,
    Psychic,
    Fighting,
    Darkness,
    Metal,
    Dragon,
    Colorless,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
enum AttackCost {
    Free,
    #[serde(untagged)]
    Energy(EnergyType),
}

pub(crate) mod u32_hex {
    use alloc::string::String;

    use serde::{Deserialize, de};

    const HEX_ENCODING_PREFIX: &str = "0x";

    #[expect(clippy::trivially_copy_pass_by_ref, reason = "required by serde api")]
    pub(crate) fn serialize<S>(data: &u32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&format_args!(
            "{HEX_ENCODING_PREFIX}{encoding}",
            encoding = hex::encode(data.to_be_bytes())
        ))
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <String>::deserialize(deserializer).and_then(|s| {
            s.strip_prefix(HEX_ENCODING_PREFIX)
                .ok_or(de::Error::invalid_value(
                    de::Unexpected::Str(&s),
                    &"hex encoded u32 bytes, 0x-prefixed",
                ))
                .and_then(|maybe_hex| {
                    hex::decode(maybe_hex)
                        .map_err(|_| {
                            de::Error::invalid_value(
                                de::Unexpected::Str(maybe_hex),
                                &"hex encoded u32 bytes, 0x-prefixed",
                            )
                        })
                        .and_then(|hex| {
                            hex.try_into()
                                .map_err(|_| {
                                    de::Error::invalid_value(
                                        de::Unexpected::Str(maybe_hex),
                                        &"hex encoded u32 bytes, 0x-prefixed",
                                    )
                                })
                                .map(u32::from_be_bytes)
                        })
                })
        })
    }
}

#[test]
fn serde() {
    for dirent in std::fs::read_dir(std::env::var("SOURCES_DIR").unwrap()).unwrap() {
        let dirent = dirent.unwrap();

        println!(
            "reading {}",
            dirent.path().file_name().unwrap().to_string_lossy()
        );

        let path = dirent.path();

        let json = std::fs::read_to_string(path).unwrap();

        let cards = serde_json::from_str::<Vec<Card>>(&json).unwrap();

        println!("{} cards", cards.len());

        let roundtrip = serde_json::to_string_pretty(&cards).unwrap();

        // std::fs::write("out.before", format!("{value:#?}")).unwrap();
        // std::fs::write("out.after", format!("{value_roundtrip:#?}")).unwrap();

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&json).unwrap(),
            serde_json::from_str::<serde_json::Value>(&roundtrip).unwrap()
        );

        // dbg!(&cards);
    }
}
