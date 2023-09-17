use std::{
    fmt::Debug,
    num::{NonZeroU16, NonZeroU8},
};

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "card_type")]
enum Card {
    #[serde(rename = "POKEMON")]
    Pokemon(Pokemon),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Pokemon {
    name: String,
    lang: Lang,
    foil: Option<Foil>,
    size: CardSize,
    artists: Artists,
    regulation_mark: RegulationMark,
    set_icon: String,
    collector_number: CollectorNumber,

    // NOTE: These fields may be combined in the future
    rarity: String,
    rarity_icon: String,

    copyright: Copyright,
    tags: Option<Vec<CardTag>>,
    stage: Stage,
    stage_text: Option<String>,
    hp: NonZeroU16,
    weakness: Weakness,
    resistance: Option<Resistance>,
    retreat: Option<NonZeroU8>,
    flavor_text: Option<String>,
    text: Vec<Text>,
    _tcgl: Tcgl,
    images: Images,
    sort_number: NonZeroU16,
    types: Vec<EnergyType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Lang {
    #[serde(rename = "en-US")]
    EnUs,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Foil {
    #[serde(rename = "type")]
    ty: FoilType,
    mask: FoilMask,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum FoilType {
    FlatSilver,
    SunPillar,
    SvHolo,
    SvUltra,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum FoilMask {
    Reverse,
    Holo,
    Etched,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CardSize {
    #[serde(rename = "STANDARD")]
    Standard,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Artists {
    text: String,
    list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum RegulationMark {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CollectorNumber {
    denominator: String,
    full: String,
    numerator: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Copyright {
    text: String,
    // REVIEW: Enum?
    year: u16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CardTag {
    #[serde(rename = "EX_LOWER")]
    ExLower,
    #[serde(rename = "TERA")]
    Tera,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Stage {
    Basic,
    #[serde(rename = "Stage 1")]
    Stage1,
    #[serde(rename = "Stage 2")]
    Stage2,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Weakness {
    amount: u8,
    operator: WeaknessOperator,
    types: Vec<EnergyType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum WeaknessOperator {
    #[serde(rename = "+")]
    Add,
    #[serde(rename = "×")]
    Multiply,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Resistance {
    amount: u8,
    operator: ResistanceOperator,
    types: Vec<EnergyType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ResistanceOperator {
    #[serde(rename = "-")]
    Subtract,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Tcgl {
    #[serde(rename = "archetypeID", with = "crate::u32_hex")]
    archetype_id: u32,
    #[serde(rename = "cardID")]
    card_id: String,
    key: String,
    #[serde(rename = "longFormID")]
    long_form_id: String,
    oa_reldate: String,
    reldate: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Images {
    tcgl: TcglImages,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TcglImages {
    jpg: ImageJpg,
    png: ImagePng,
    tex: ImageTex,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ImageJpg {
    front: Url,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ImagePng {
    front: Url,
    foil: Option<Url>,
    etch: Option<Url>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ImageTex {
    front: Url,
    foil: Option<Url>,
    etch: Option<Url>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Damage {
    // REVIEW: Multiple of 10, nonzero?
    amount: u16,
    suffix: Option<DamageSuffix>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum DamageSuffix {
    #[serde(rename = "+")]
    Add,
    #[serde(rename = "-")]
    Subtract,
    #[serde(rename = "×")]
    Multiply,
}

#[derive(Debug, Serialize, Deserialize)]
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
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Attack {
    name: String,
    text: String,
    cost: Vec<EnergyType>,
    damage: Option<Damage>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Ability {
    name: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RuleBox {
    name: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Effect {
    name: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
        <&str>::deserialize(deserializer).and_then(|s| {
            s.strip_prefix(HEX_ENCODING_PREFIX)
                .ok_or(de::Error::invalid_value(
                    de::Unexpected::Str(s),
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

    let json = std::fs::read_to_string("sv1.en-US.json").unwrap();

    let cards = serde_json::from_str::<Vec<Value>>(&json).unwrap();

    for value in cards {
        dbg!(&value["name"]);

        if value["card_type"] != "POKEMON" {
            continue;
        }

        let card: Card = serde_json::from_str(&serde_json::to_string(&value).unwrap()).unwrap();

        dbg!(card);
    }
}
