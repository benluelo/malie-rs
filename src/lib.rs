use std::num::NonZeroU8;

use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    tags: Option<Vec<String>>,
    stage: String,
    stage_text: Option<String>,
    hp: u16,
    weakness: Weakness,
    resistance: Option<Resistance>,
    retreat: Option<NonZeroU8>,
    flavor_text: Option<String>,
    text: Vec<Text>,
    _tcgl: Tcgl,
    images: Images,
    sort_number: i64,
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
struct Tcgl {
    archetypeID: String,
    cardID: String,
    key: String,
    longFormID: String,
    oa_reldate: String,
    reldate: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Struct6 {
    front: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ImagePng {
    front: String,
    foil: Option<String>,
    etch: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ImageTex {
    front: String,
    foil: Option<String>,
    etch: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ImagesInner {
    jpg: Struct6,
    png: ImagePng,
    tex: ImageTex,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Images {
    tcgl: ImagesInner,
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
struct Weakness {
    amount: i64,
    operator: String,
    types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Resistance {
    amount: i64,
    operator: String,
    types: Vec<String>,
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

#[test]
fn serde() {
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
