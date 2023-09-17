use std::num::{NonZeroU16, NonZeroU8};

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
    #[serde(rename = "archetypeID")]
    archetype_id: String,
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
