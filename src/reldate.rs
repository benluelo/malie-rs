use core::fmt;

use serde::{Deserializer, Serializer, de};
use time::{
    PrimitiveDateTime,
    error::Format,
    format_description::{
        BorrowedFormatItem, Component,
        modifier::{
            Day, Hour, Minute, Month, MonthRepr, Padding, Second, Year, YearRange, YearRepr,
        },
    },
};

const ITEMS: &[BorrowedFormatItem<'_>] = &[
    BorrowedFormatItem::Component(Component::Year({
        let mut value = Year::default();
        value.padding = Padding::Zero;
        value.repr = YearRepr::Full;
        value.range = YearRange::Extended;
        value.iso_week_based = false;
        value.sign_is_mandatory = false;
        value
    })),
    BorrowedFormatItem::Literal(b"-"),
    BorrowedFormatItem::Component(Component::Month({
        let mut value = Month::default();
        value.padding = Padding::Zero;
        value.repr = MonthRepr::Numerical;
        value.case_sensitive = true;
        value
    })),
    BorrowedFormatItem::Literal(b"-"),
    BorrowedFormatItem::Component(Component::Day({
        let mut value = Day::default();
        value.padding = Padding::Zero;
        value
    })),
    BorrowedFormatItem::Literal(b" "),
    BorrowedFormatItem::Component(Component::Hour({
        let mut value = Hour::default();
        value.padding = Padding::Zero;
        value.is_12_hour_clock = false;
        value
    })),
    BorrowedFormatItem::Literal(b":"),
    BorrowedFormatItem::Component(Component::Minute({
        let mut value = Minute::default();
        value.padding = Padding::Zero;
        value
    })),
    BorrowedFormatItem::Literal(b":"),
    BorrowedFormatItem::Component(Component::Second({
        let mut value = Second::default();
        value.padding = Padding::Zero;
        value
    })),
    BorrowedFormatItem::Literal(b"+00:00"),
];

struct Visitor;

impl de::Visitor<'_> for Visitor {
    type Value = PrimitiveDateTime;
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            concat!("a(n) `", "PrimitiveDateTime", "` in the format \"{}\"",),
            "[year]-[month]-[day] [hour]:[minute]:[second]+00:00"
        )
    }
    fn visit_str<E: de::Error>(self, value: &str) -> Result<PrimitiveDateTime, E> {
        PrimitiveDateTime::parse(value, ITEMS).map_err(E::custom)
    }
}

pub fn serialize<S: Serializer>(
    datetime: &PrimitiveDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    use ::serde::Serialize;
    datetime
        .format(ITEMS)
        .map_err(Format::into_invalid_serde_value::<S>)?
        .serialize(serializer)
}

pub fn deserialize<'a, D: Deserializer<'a>>(
    deserializer: D,
) -> Result<PrimitiveDateTime, D::Error> {
    deserializer.deserialize_str(Visitor)
}
