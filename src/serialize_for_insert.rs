use std::error::Error;
use std::fmt;

use fallible_iterator::FallibleIterator;
use itertools::{process_results, Itertools};
use postgres_protocol::escape::escape_literal;
use tokio_postgres::types::{FromSql, Kind, Type};
use uuid::Uuid;

pub use postgres_protocol::types::ArrayDimensions;

use crate::array::Array;
use crate::raw::Raw;

pub trait SerializeForInsert {
    fn serialize(type_: &Type, value: &Self) -> Result<String, Box<dyn Error + Sync + Send>>;
}

impl<'a> SerializeForInsert for Raw<'a> {
    // TODO: This feels repetative; what can be done better?
    fn serialize(type_: &Type, value: &Raw<'a>) -> Result<String, Box<dyn Error + Sync + Send>> {
        match *type_.kind() {
            Kind::Array(ref member_type) => match *member_type {
                Type::VARCHAR | Type::TEXT | Type::BPCHAR | Type::NAME | Type::UNKNOWN => {
                    let s: Array<String> = value.try_into()?;
                    SerializeForInsert::serialize(type_, &s)
                }
                Type::UUID => {
                    let u: Array<Uuid> = value.try_into()?;
                    SerializeForInsert::serialize(type_, &u)
                }
                Type::BOOL => {
                    let u: Array<bool> = value.try_into()?;
                    SerializeForInsert::serialize(type_, &u)
                }
                Type::CHAR => {
                    let u: Array<i8> = value.try_into()?;
                    SerializeForInsert::serialize(type_, &u)
                }
                Type::INT2 => {
                    let u: Array<i16> = value.try_into()?;
                    SerializeForInsert::serialize(type_, &u)
                }
                Type::INT4 => {
                    let u: Array<i32> = value.try_into()?;
                    SerializeForInsert::serialize(type_, &u)
                }
                Type::INT8 => {
                    let u: Array<i64> = value.try_into()?;
                    SerializeForInsert::serialize(type_, &u)
                }
                Type::FLOAT4 => {
                    let u: Array<f32> = value.try_into()?;
                    SerializeForInsert::serialize(type_, &u)
                }
                Type::FLOAT8 => {
                    let u: Array<f64> = value.try_into()?;
                    SerializeForInsert::serialize(type_, &u)
                }
                Type::TIMESTAMPTZ => {
                    let dt: Array<time::OffsetDateTime> = value.try_into()?;
                    SerializeForInsert::serialize(type_, &dt)
                }
                Type::DATE => {
                    let dt: Array<time::Date> = value.try_into()?;
                    SerializeForInsert::serialize(type_, &dt)
                }
                Type::JSON | Type::JSONB => {
                    let dt: Array<serde_json::Value> = value.try_into()?;
                    SerializeForInsert::serialize(type_, &dt)
                }
                Type::OID => Err(Box::new(TypeError)),
                _ => {
                    panic!("Unsupported type {}", type_);
                }
            },
            Kind::Simple => match *type_ {
                Type::VARCHAR | Type::TEXT | Type::BPCHAR | Type::NAME | Type::UNKNOWN => {
                    let s: String = value.try_into()?;
                    SerializeForInsert::serialize(type_, &s)
                }
                Type::UUID => {
                    let u: Uuid = value.try_into()?;
                    SerializeForInsert::serialize(type_, &u)
                }
                Type::BOOL => {
                    let u: bool = value.try_into()?;
                    SerializeForInsert::serialize(type_, &u)
                }
                Type::CHAR => {
                    let u: i8 = value.try_into()?;
                    SerializeForInsert::serialize(type_, &u)
                }
                Type::INT2 => {
                    let u: i16 = value.try_into()?;
                    SerializeForInsert::serialize(type_, &u)
                }
                Type::INT4 => {
                    let u: i32 = value.try_into()?;
                    SerializeForInsert::serialize(type_, &u)
                }
                Type::INT8 => {
                    let u: i64 = value.try_into()?;
                    SerializeForInsert::serialize(type_, &u)
                }
                Type::FLOAT4 => {
                    let u: f32 = value.try_into()?;
                    SerializeForInsert::serialize(type_, &u)
                }
                Type::FLOAT8 => {
                    let u: f64 = value.try_into()?;
                    SerializeForInsert::serialize(type_, &u)
                }
                Type::TIMESTAMPTZ => {
                    let dt: time::OffsetDateTime = value.try_into()?;
                    SerializeForInsert::serialize(type_, &dt)
                }
                Type::DATE => {
                    let dt: time::Date = value.try_into()?;
                    SerializeForInsert::serialize(type_, &dt)
                }
                Type::JSON | Type::JSONB => {
                    let dt: serde_json::Value = value.try_into()?;
                    SerializeForInsert::serialize(type_, &dt)
                }
                Type::OID => Err(Box::new(TypeError)),
                _ => {
                    panic!("Unsupported type {}", type_);
                }
            },
            _ => {
                panic!("Unsupported kind {:?}", type_.kind());
            }
        }
    }
}

impl<T: SerializeForInsert> SerializeForInsert for Option<T> {
    fn serialize(type_: &Type, value: &Option<T>) -> Result<String, Box<dyn Error + Sync + Send>> {
        match value {
            None => Ok("NULL".to_string()),
            Some(v) => T::serialize(type_, v),
        }
    }
}

impl<'a, T: FromSql<'a> + SerializeForInsert> SerializeForInsert for Array<'a, T> {
    fn serialize(
        type_: &Type,
        value: &Array<'a, T>,
    ) -> Result<String, Box<dyn Error + Sync + Send>> {
        let member_type = match *type_.kind() {
            Kind::Array(ref member) => member,
            _ => panic!("expected array type"),
        };

        if value.dimensions().count()? > 1 {
            return Err("array contains too many dimensions".into());
        }

        Ok(format!(
            "ARRAY[{}]::{}[]",
            process_results(
                value
                    .values()
                    .map(|ss| Option::<T>::serialize(member_type, &ss))
                    .iterator(),
                |mut iter| iter.join(", ")
            )?,
            member_type.name()
        ))
    }
}

impl SerializeForInsert for String {
    fn serialize(_type: &Type, value: &String) -> Result<String, Box<dyn Error + Sync + Send>> {
        Ok(escape_literal(value))
    }
}

impl SerializeForInsert for Uuid {
    fn serialize(_type: &Type, value: &Uuid) -> Result<String, Box<dyn Error + Sync + Send>> {
        Ok(format!("'{}'", value.to_hyphenated().to_string()))
    }
}

impl SerializeForInsert for bool {
    fn serialize(_type: &Type, value: &bool) -> Result<String, Box<dyn Error + Sync + Send>> {
        if *value {
            Ok("true".to_string())
        } else {
            Ok("false".to_string())
        }
    }
}

impl SerializeForInsert for i8 {
    fn serialize(_type: &Type, value: &i8) -> Result<String, Box<dyn Error + Sync + Send>> {
        Ok(value.to_string())
    }
}

impl SerializeForInsert for i16 {
    fn serialize(_type: &Type, value: &i16) -> Result<String, Box<dyn Error + Sync + Send>> {
        Ok(value.to_string())
    }
}

impl SerializeForInsert for i32 {
    fn serialize(_type: &Type, value: &i32) -> Result<String, Box<dyn Error + Sync + Send>> {
        Ok(value.to_string())
    }
}

impl SerializeForInsert for i64 {
    fn serialize(_type: &Type, value: &i64) -> Result<String, Box<dyn Error + Sync + Send>> {
        Ok(value.to_string())
    }
}

impl SerializeForInsert for f32 {
    fn serialize(_type: &Type, value: &f32) -> Result<String, Box<dyn Error + Sync + Send>> {
        Ok(value.to_string())
    }
}

impl SerializeForInsert for f64 {
    fn serialize(_type: &Type, value: &f64) -> Result<String, Box<dyn Error + Sync + Send>> {
        Ok(value.to_string())
    }
}

impl SerializeForInsert for time::Date {
    fn serialize(_type: &Type, value: &time::Date) -> Result<String, Box<dyn Error + Sync + Send>> {
        Ok(format!("'{}'", value.format("%Y-%m-%d")))
    }
}

impl SerializeForInsert for time::OffsetDateTime {
    fn serialize(
        _type: &Type,
        value: &time::OffsetDateTime,
    ) -> Result<String, Box<dyn Error + Sync + Send>> {
        Ok(format!("'{}'", value.format(time::Format::Rfc3339)))
    }
}

impl SerializeForInsert for serde_json::Value {
    fn serialize(
        type_: &Type,
        value: &serde_json::Value,
    ) -> Result<String, Box<dyn Error + Sync + Send>> {
        Ok(format!("'{}'::{}", value.to_string(), type_.name()))
    }
}

#[derive(Debug, Clone)]
struct TypeError;

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OID not supported")
    }
}

impl std::error::Error for TypeError {}
