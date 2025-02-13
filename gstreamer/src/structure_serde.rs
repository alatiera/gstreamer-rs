// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::upper_case_acronyms)]

use glib::{Date, ToValue};

use serde::de;
use serde::de::{Deserialize, DeserializeSeed, Deserializer, SeqAccess, Visitor};
use serde::ser;
use serde::ser::{Serialize, SerializeSeq, SerializeTuple, Serializer};

use std::fmt;

use crate::Buffer;
use crate::DateTime;
use crate::Sample;

use crate::date_time_serde;
use crate::value::*;
use crate::value_serde::*;

use crate::Structure;
use crate::StructureRef;

struct FieldSe<'a>(&'static str, &'a glib::SendValue);
impl<'a> Serialize for FieldSe<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        ser_value!(self.1, |type_, value| {
            let mut tup = serializer.serialize_tuple(3)?;
            tup.serialize_element(self.0)?;
            tup.serialize_element(type_)?;
            tup.serialize_element(&value)?;
            tup.end()
        })
    }
}

struct StructureForIter<'a>(&'a StructureRef);
impl<'a> Serialize for StructureForIter<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let iter = self.0.iter();
        let size = iter.size_hint().0;
        if size > 0 {
            let mut seq = serializer.serialize_seq(Some(size))?;
            for field in iter {
                seq.serialize_element(&FieldSe(field.0, field.1))?;
            }
            seq.end()
        } else {
            let seq = serializer.serialize_seq(None)?;
            seq.end()
        }
    }
}

impl Serialize for StructureRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut tup = serializer.serialize_tuple(2)?;
        tup.serialize_element(self.name())?;
        tup.serialize_element(&StructureForIter(self))?;
        tup.end()
    }
}

impl Serialize for Structure {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        <StructureRef as Serialize>::serialize(self.as_ref(), serializer)
    }
}

struct FieldDe(String, SendValue);
impl From<FieldDe> for (String, glib::SendValue) {
    fn from(field_de: FieldDe) -> Self {
        skip_assert_initialized!();
        (field_de.0, field_de.1.into())
    }
}

struct FieldVisitor;
impl<'de> Visitor<'de> for FieldVisitor {
    type Value = FieldDe;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(
            "a tuple of 3 elements (name: `String`, type name: `String`, value: `Value`)",
        )
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let name = seq
            .next_element::<String>()?
            .ok_or_else(|| de::Error::custom("Expected a value for `Value` name"))?;
        let type_name = seq
            .next_element::<String>()?
            .ok_or_else(|| de::Error::custom("Expected a value for `Value` type"))?;
        let send_value = de_send_value!(type_name, seq)?
            .ok_or_else(|| de::Error::custom("Expected a value for `Value`"))?;
        Ok(FieldDe(name, send_value))
    }
}

impl<'de> Deserialize<'de> for FieldDe {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        deserializer.deserialize_tuple(3, FieldVisitor)
    }
}

struct FieldsDe<'a>(&'a mut StructureRef);

struct FieldsVisitor<'a>(&'a mut StructureRef);
impl<'de, 'a> Visitor<'de> for FieldsVisitor<'a> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence of `Structure` `Field`s")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
        while let Some(field) = seq.next_element::<FieldDe>()? {
            let (name, value): (String, glib::SendValue) = field.into();
            self.0.set_value(name.as_str(), value);
        }

        Ok(())
    }
}

impl<'de, 'a> DeserializeSeed<'de> for FieldsDe<'a> {
    type Value = ();

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        skip_assert_initialized!();
        deserializer.deserialize_seq(FieldsVisitor(self.0))
    }
}

struct StructureVisitor;
impl<'de> Visitor<'de> for StructureVisitor {
    type Value = Structure;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a `Structure` (name, sequence of `Field`s)")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let name = seq
            .next_element::<String>()?
            .ok_or_else(|| de::Error::custom("Expected a name for the `Structure`"))?;
        let mut structure = Structure::new_empty(&name);
        seq.next_element_seed(FieldsDe(structure.as_mut()))?
            .ok_or_else(|| de::Error::custom("Expected a sequence of `Field`s"))?;

        Ok(structure)
    }
}

impl<'de> Deserialize<'de> for Structure {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        deserializer.deserialize_tuple(2, StructureVisitor)
    }
}

#[cfg(test)]
mod tests {
    use glib::{Date, DateMonth};

    use crate::Array;
    use crate::DateTime;
    use crate::Fraction;
    use crate::Structure;

    #[test]
    fn test_serialize() {
        crate::init().unwrap();

        let s = Structure::builder("test")
            .field("f1", "abc")
            .field("f2", &String::from("bcd"))
            .field("f3", 123i32)
            .field("fraction", Fraction::new(1, 2))
            .field("date", Date::from_dmy(19, DateMonth::August, 2019).unwrap())
            .field(
                "date_time",
                DateTime::new(2f32, 2019, 8, 19, 13, 34, 42f64).unwrap(),
            )
            .field("array", Array::new([1, 2]))
            .build();

        let pretty_config = ron::ser::PrettyConfig::new().new_line("".to_string());

        let res = ron::ser::to_string_pretty(&s, pretty_config);
        assert_eq!(
            Ok(concat!(
                r#"("test", ["#,
                r#"    ("f1", "String", Some("abc")),"#,
                r#"    ("f2", "String", Some("bcd")),"#,
                r#"    ("f3", "i32", 123),"#,
                r#"    ("fraction", "Fraction", (1, 2)),"#,
                r#"    ("date", "Date", Some(YMD(2019, 8, 19))),"#,
                r#"    ("date_time", "DateTime", Some(YMDhmsTz(2019, 8, 19, 13, 34, 42.0, 2.0))),"#,
                r#"    ("array", "Array", ["#,
                r#"        ("i32", 1),"#,
                r#"        ("i32", 2),"#,
                r#"    ]),"#,
                r#"])"#,
            )
            .to_owned()),
            res,
        );
    }

    #[test]
    fn test_deserialize() {
        crate::init().unwrap();

        let s_ron = r#"
            ("test", [
                ("f1", "String", Some("abc")),
                ("f2", "String", Some("bcd")),
                ("f3", "i32", 123),
                ("fraction", "Fraction", (1, 2)),
                ("date", "Date", Some(YMD(2019, 8, 19))),
                ("date_time", "DateTime", Some(YMDhmsTz(2019, 8, 19, 13, 34, 42, 2))),
                ("array", "Array", [
                    ("i32", 1),
                    ("i32", 2),
                ]),
            ])"#;
        let s: Structure = ron::de::from_str(s_ron).unwrap();
        assert_eq!(
            s.as_ref(),
            Structure::builder("test")
                .field("f1", "abc")
                .field("f2", "bcd")
                .field("f3", 123)
                .field("date", Date::from_dmy(19, DateMonth::August, 2019).unwrap())
                .field(
                    "date_time",
                    DateTime::new(2f32, 2019, 8, 19, 13, 34, 42f64).unwrap()
                )
                .field("fraction", Fraction::new(1, 2))
                .field("array", Array::new([1, 2]))
                .build()
                .as_ref()
        );
    }

    #[test]
    fn test_serde_roundtrip() {
        crate::init().unwrap();

        let s = Structure::builder("test")
            .field("f1", "abc")
            .field("f2", "bcd".to_owned())
            .field("f3", 123i32)
            .field("fraction", Fraction::new(1, 2))
            .field("date", Date::from_dmy(19, DateMonth::August, 2019).unwrap())
            .field(
                "date_time",
                DateTime::new(2f32, 2019, 8, 19, 13, 34, 42f64).unwrap(),
            )
            .field("array", Array::new([1, 2]))
            .build();
        let s_ser = ron::ser::to_string(&s).unwrap();
        let s_de: Structure = ron::de::from_str(s_ser.as_str()).unwrap();
        assert_eq!(s_de.as_ref(), s.as_ref());
    }
}
