use std::error::Error;
use std::marker::PhantomData;

use fallible_iterator::FallibleIterator;

use postgres_protocol::types;
pub use postgres_protocol::types::ArrayDimensions;
use tokio_postgres::types::FromSql;
use tokio_postgres::types::{Kind, Type};

pub struct Array<'a, T> {
    array: types::Array<'a>,
    member_type: Type,
    type_marker: PhantomData<T>,
}

pub struct ArrayValues<'a, T>
where
    T: FromSql<'a>,
{
    array_values: types::ArrayValues<'a>,
    member_type: Type,
    type_marker: PhantomData<T>,
}

impl<'a, T: FromSql<'a>> Array<'a, T> {
    /// Returns true if there are `NULL` elements.
    #[inline]
    pub fn has_nulls(&self) -> bool {
        self.array.has_nulls()
    }

    /// Returns the Type of the elements of the array.
    #[inline]
    pub fn element_type(&self) -> &Type {
        &self.member_type
    }

    #[inline]
    pub fn dimensions(&self) -> ArrayDimensions<'a> {
        self.array.dimensions()
    }

    /// Returns an iterator over the values of the array.
    #[inline]
    pub fn values(&self) -> ArrayValues<'a, T> {
        ArrayValues {
            member_type: self.member_type.clone(),
            array_values: self.array.values(),
            type_marker: PhantomData,
        }
    }
}

impl<'a, T: FromSql<'a>> FromSql<'a> for Array<'a, T> {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Array<'a, T>, Box<dyn Error + Sync + Send>> {
        let member_type = match *ty.kind() {
            Kind::Array(ref member) => member,
            _ => panic!("expected array type"),
        };

        let array = types::array_from_sql(raw)?;

        Ok(Array {
            member_type: member_type.clone(),
            array,
            type_marker: PhantomData,
        })
    }

    fn accepts(ty: &Type) -> bool {
        match *ty.kind() {
            Kind::Array(ref inner) => T::accepts(inner),
            _ => false,
        }
    }
}

impl<'a, T: FromSql<'a>> FallibleIterator for ArrayValues<'a, T> {
    type Item = Option<T>;
    type Error = Box<dyn Error + Sync + Send>;

    #[inline]
    fn next(&mut self) -> Result<Option<Option<T>>, Box<dyn Error + Sync + Send>> {
        let n = self.array_values.next();

        match n {
            Err(e) => Err(e),
            Ok(n) => match n {
                None => Ok(None),
                Some(n) => {
                    let value: Result<Option<T>, Box<dyn Error + Sync + Send>> =
                        FromSql::from_sql_nullable(&self.member_type, n);
                    value.map(|v| Some(v))
                }
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.array_values.size_hint()
    }
}
