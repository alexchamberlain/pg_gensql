use std::error::Error;

use tokio_postgres::types::{FromSql, Type, WrongType};

pub struct Raw<'a> {
    raw: &'a [u8],
    type_: Type,
}

impl<'a> Raw<'a> {
    // TODO: Can we actually implement TryFrom/TryInto?
    pub fn try_into<T: FromSql<'a>>(self: &Raw<'a>) -> Result<T, Box<dyn Error + Sync + Send>> {
        let ty = &self.type_;
        if !T::accepts(ty) {
            return Err(Box::new(WrongType::new::<T>(ty.clone())));
        }

        T::from_sql(ty, self.raw)
    }
}

impl<'a> FromSql<'a> for Raw<'a> {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Raw<'a>, Box<dyn Error + Sync + Send>> {
        Ok(Raw {
            raw,
            type_: ty.clone(),
        })
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }
}
