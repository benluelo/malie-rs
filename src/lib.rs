use std::{
    fmt::Debug,
    num::{NonZeroU16, NonZeroU8},
    str::FromStr,
};

use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "card_type")]
enum Card {
    #[serde(rename = "POKEMON")]
    Pokemon(Pokemon),
    #[serde(rename = "TRAINER")]
    Trainer(Trainer),
    #[serde(rename = "ENERGY")]
    Energy(Energy),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Pokemon {
    name: String,
    lang: Lang,
    #[serde(skip_serializing_if = "Option::is_none")]
    foil: Option<Foil>,
    size: CardSize,
    back: CardBack,
    #[serde(skip_serializing_if = "Option::is_none")]
    artists: Option<Artists>,
    regulation_mark: RegulationMark,
    set_icon: String,
    collector_number: CollectorNumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    rarity: Option<Rarity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copyright: Option<Copyright>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CardTag>>,
    stage: Stage,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage_text: Option<String>,
    hp: NonZeroU16,
    #[serde(skip_serializing_if = "Option::is_none")]
    weakness: Option<Weakness>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resistance: Option<Resistance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retreat: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flavor_text: Option<String>,
    text: Vec<Text>,
    ext: Ext,
    images: Images,
    types: Vec<EnergyType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "subtype")]
pub enum Trainer {
    #[serde(rename = "ITEM")]
    Item(Item),
    #[serde(rename = "SUPPORTER")]
    Supporter(Supporter),
    #[serde(rename = "TOOL")]
    Tool(Tool),
    #[serde(rename = "STADIUM")]
    Stadium(Stadium),
}

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub enum TrainerSubtype {
//     #[serde(rename = "ITEM")]
//     Item,
//     #[serde(rename = "SUPPORTER")]
//     Supporter,
//     #[serde(rename = "TOOL")]
//     Tool,
//     #[serde(rename = "STADIUM")]
//     Stadium,
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Item {
    // pub struct Trainer {
    // subtype: TrainerSubtype,
    name: String,
    lang: Lang,
    #[serde(skip_serializing_if = "Option::is_none")]
    foil: Option<Foil>,
    size: CardSize,
    back: CardBack,
    #[serde(skip_serializing_if = "Option::is_none")]
    artists: Option<Artists>,
    regulation_mark: RegulationMark,
    set_icon: String,
    collector_number: CollectorNumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    rarity: Option<Rarity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copyright: Option<Copyright>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CardTag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hp: Option<NonZeroU16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flavor_text: Option<String>,
    text: Vec<Text>,
    ext: Ext,
    images: Images,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Supporter {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtitle: Option<String>,
    lang: Lang,
    #[serde(skip_serializing_if = "Option::is_none")]
    foil: Option<Foil>,
    size: CardSize,
    back: CardBack,
    #[serde(skip_serializing_if = "Option::is_none")]
    artists: Option<Artists>,
    regulation_mark: RegulationMark,
    set_icon: String,
    collector_number: CollectorNumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    rarity: Option<Rarity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copyright: Option<Copyright>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CardTag>>,
    text: Vec<Text>,
    ext: Ext,
    images: Images,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tool {
    name: String,
    lang: Lang,
    #[serde(skip_serializing_if = "Option::is_none")]
    foil: Option<Foil>,
    size: CardSize,
    back: CardBack,
    #[serde(skip_serializing_if = "Option::is_none")]
    artists: Option<Artists>,
    regulation_mark: RegulationMark,
    set_icon: String,
    collector_number: CollectorNumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    rarity: Option<Rarity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copyright: Option<Copyright>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CardTag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flavor_text: Option<String>,
    text: Vec<Text>,
    ext: Ext,
    images: Images,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stadium {
    name: String,
    lang: Lang,
    #[serde(skip_serializing_if = "Option::is_none")]
    foil: Option<Foil>,
    size: CardSize,
    back: CardBack,
    #[serde(skip_serializing_if = "Option::is_none")]
    artists: Option<Artists>,
    regulation_mark: RegulationMark,
    set_icon: String,
    collector_number: CollectorNumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    rarity: Option<Rarity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copyright: Option<Copyright>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CardTag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flavor_text: Option<String>,
    text: Vec<Text>,
    ext: Ext,
    images: Images,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "subtype")]
pub enum Energy {
    #[serde(rename = "BASIC")]
    BasicEnergy(BasicEnergy),
    #[serde(rename = "SPECIAL")]
    SpecialEnergy(SpecialEnergy),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BasicEnergy {
    name: String,
    lang: Lang,
    #[serde(skip_serializing_if = "Option::is_none")]
    foil: Option<Foil>,
    size: CardSize,
    back: CardBack,
    // REVIEW: Do basic energies every have artist(s)?
    // artists: Artists,
    // REVIEW: Do basic energies every have a regulation mark?
    // regulation_mark: RegulationMark,
    set_icon: String,
    collector_number: CollectorNumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    rarity: Option<Rarity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copyright: Option<Copyright>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CardTag>>,
    ext: Ext,
    images: Images,
    types: Vec<EnergyType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpecialEnergy {
    name: String,
    lang: Lang,
    #[serde(skip_serializing_if = "Option::is_none")]
    foil: Option<Foil>,
    size: CardSize,
    back: CardBack,
    // REVIEW: Do basic energies every have artist(s)?
    // artists: Artists,
    regulation_mark: RegulationMark,
    set_icon: String,
    collector_number: CollectorNumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    rarity: Option<Rarity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copyright: Option<Copyright>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CardTag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flavor_text: Option<String>,
    text: Vec<Text>,
    ext: Ext,
    images: Images,
    // types: Vec<EnergyType>,
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FoilMask {
    Stamped,
    Reverse,
    Holo,
    Etched,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CardSize {
    #[serde(rename = "STANDARD")]
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
pub struct Artists {
    text: String,
    list: Vec<String>,
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CollectorNumber {
    full: String,
    numerator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    denominator: Option<String>,
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Copyright {
    text: String,
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Stage {
    #[serde(rename = "BASIC")]
    Basic,
    #[serde(rename = "STAGE1")]
    Stage1,
    #[serde(rename = "STAGE2")]
    Stage2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Weakness {
    amount: u8,
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
pub struct Ext {
    tcgl: Tcgl,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Tcgl {
    #[serde(rename = "archetypeID", with = "crate::u32_hex")]
    archetype_id: u32,
    #[serde(rename = "cardID")]
    card_id: String,
    key: String,
    #[serde(rename = "longFormID")]
    long_form_id: String,
    reldate: String,
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
#[serde(deny_unknown_fields)]
#[serde(tag = "kind")]
enum Text {
    #[serde(rename = "ATTACK")]
    Attack(Attack),
    #[serde(rename = "ABILITY")]
    Ability(Ability),
    #[serde(rename = "RULE_BOX")]
    RuleBox(RuleBox),
    #[serde(rename = "EFFECT")]
    Effect(Effect),
    #[serde(rename = "TEXT_BOX")]
    TextBox(TextBox),
    #[serde(rename = "REMINDER")]
    Reminder(Reminder),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Attack {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    cost: Vec<AttackCost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    damage: Option<Damage>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Ability {
    name: String,
    text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RuleBox {
    name: String,
    text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Effect {
    name: String,
    text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TextBox {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Reminder {
    text: String,
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

    // TODO: Custom serde implementation to flatten EnergyType into this enum
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

pub mod u32_hex {
    use serde::{de, Deserialize};

    const HEX_ENCODING_PREFIX: &str = "0x";

    pub fn serialize<S>(data: &u32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&format_args!(
            "{HEX_ENCODING_PREFIX}{encoding}",
            encoding = hex::encode(data.to_be_bytes())
        ))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
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
    use serde_json::Value;

    for dirent in std::fs::read_dir("sources").unwrap() {
        let dirent = dirent.unwrap();

        println!(
            "reading {}",
            dirent.path().file_name().unwrap().to_string_lossy()
        );

        let json = std::fs::read_to_string(dirent.path()).unwrap();

        let value = serde_json::from_str::<Value>(&json).unwrap();

        let cards = serde_json::from_value::<Vec<Card>>(value.clone()).unwrap();

        let value_roundtrip = serde_json::to_value(&cards).unwrap();

        // println!("{serialized}");

        std::fs::write("out.before", format!("{value:#?}")).unwrap();
        std::fs::write("out.after", format!("{value_roundtrip:#?}")).unwrap();

        assert_eq!(value, value_roundtrip);
    }

    // dbg!(&cards);

    // for value in cards {
    //     dbg!(&value["name"]);

    //     // if value["card_type"] != "POKEMON" {
    //     //     continue;
    //     // }

    //     let card: Card =
    // serde_json::from_str(&serde_json::to_string(&value).unwrap()).unwrap();

    //     dbg!(card);
    // }
}
