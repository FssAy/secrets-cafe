mod state;
mod tier;
mod token;
mod post;

pub use state::*;
pub use tier::*;
pub use token::*;
pub use post::*;

use std::fmt;
use serde::de::{self, Error, Visitor};

struct U8Visitor;

impl<'de> Visitor<'de> for U8Visitor {
    type Value = u8;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 0 and 255")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
        where
            E: de::Error,
    {
        Ok(value as u8)
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
        where
            E: de::Error,
    {
        if value >= i32::from(u8::MIN) && value <= i32::from(u8::MAX) {
            Ok(value as u8)
        } else {
            Err(E::custom(format!("u8 out of range: {}", value)))
        }
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
    {
        if value >= i64::from(u8::MIN) && value <= i64::from(u8::MAX) {
            Ok(value as u8)
        } else {
            Err(E::custom(format!("u8 out of range: {}", value)))
        }
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E> where E: Error {
        Ok(value)
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E> where E: Error {
        if value <= u32::from(u8::MAX) {
            Ok(value as u8)
        } else {
            Err(E::custom(format!("u8 out of range: {}", value)))
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> where E: Error {
        if value <= u64::from(u8::MAX) {
            Ok(value as u8)
        } else {
            Err(E::custom(format!("u8 out of range: {}", value)))
        }
    }
}
