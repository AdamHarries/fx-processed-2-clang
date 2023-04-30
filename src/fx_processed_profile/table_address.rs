use serde::de;
use serde::de::{Deserializer, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;

/// A table address is designed to replicate the Flow.js pattern of:
///     SomeNumberType | -1
/// In tables (such as frame tables), this represents either the "empty" cell, which we also call the "base" cell, or an offset from this "base" cell. We also handle the case of a malformed (negative) address.
/// In Rust, we want to make sure that addresses are 64 bit, so that we can correctly symbolicate on 64-bit systems, which means that we can't store MAX_INT(64) + (-1), as that would require 65+ bits. Instead, we have a union of valid addresses, a "base" marker (which is -1), and an error marker, for other negative values that might find their way into a table.
/// Unfortunately, it's not easy to get serde to automatically infer a parse for this type, as it would prefer to assign -1 to "Error" - if we could even convince it to write a parser for this type.
/// This means that we have to define our own serialization and deserialization machinery, explicitly. This requires a visitor for different integer types, which then calls the correct creation method for the TableAddress.

pub type Address = u64;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TableAddress {
    Address(Address),
    Base,
    Error(i64),
}

impl TableAddress {
    fn from_positive<V>(value: V) -> TableAddress
    where
        u64: From<V>,
    {
        TableAddress::Address(u64::from(value))
    }

    fn from_negative<V>(value: V) -> TableAddress
    where
        i64: From<V>,
    {
        let isf = i64::from(value);
        if isf == -1 {
            TableAddress::Base
        } else {
            TableAddress::Error(isf)
        }
    }
}

impl Serialize for TableAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            &TableAddress::Address(u) => serializer.serialize_u64(u),
            &TableAddress::Base => serializer.serialize_i8(-1),
            &TableAddress::Error(i) => serializer.serialize_i64(i),
        }
    }
}

struct AddressVisitor;

impl<'de> Visitor<'de> for AddressVisitor {
    type Value = TableAddress;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A valid 64-bit integer, or -1")
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TableAddress::from_positive(value))
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TableAddress::from_positive(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TableAddress::from_positive(value))
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TableAddress::from_negative(value))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TableAddress::from_negative(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TableAddress::from_negative(value))
    }
}

impl<'de> Deserialize<'de> for TableAddress {
    fn deserialize<D>(deserializer: D) -> Result<TableAddress, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(AddressVisitor)
    }
}
