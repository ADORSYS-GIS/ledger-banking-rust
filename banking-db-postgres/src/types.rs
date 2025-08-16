use heapless::String as HeaplessString;
use sqlx::postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef};
use sqlx::{Decode, Encode, Postgres, Type};
use std::error::Error;
use std::fmt;

/// A wrapper around `heapless::String<N>` for use with `sqlx`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaplessStringWrapper<const N: usize>(pub HeaplessString<N>);

#[derive(Debug)]
pub enum DecodeError {
    StringTooLong { actual: usize, max: usize },
    Sqlx(Box<dyn Error + 'static + Send + Sync>),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::StringTooLong { actual, max } => {
                write!(f, "String length {} exceeds maximum capacity {}", actual, max)
            }
            DecodeError::Sqlx(err) => write!(f, "SQLx error: {}", err),
        }
    }
}

impl Error for DecodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DecodeError::StringTooLong { .. } => None,
            DecodeError::Sqlx(err) => Some(err.as_ref()),
        }
    }
}

impl<const N: usize> Type<Postgres> for HeaplessStringWrapper<N> {
    fn type_info() -> PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        *ty == PgTypeInfo::with_name("TEXT") || *ty == PgTypeInfo::with_name("VARCHAR")
    }
}

impl<'q, const N: usize> Encode<'q, Postgres> for HeaplessStringWrapper<N> {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn Error + Send + Sync>> {
        <&str as Encode<Postgres>>::encode_by_ref(&self.0.as_str(), buf)
    }
}

impl<'r, const N: usize> Decode<'r, Postgres> for HeaplessStringWrapper<N> {
    fn decode(
        value: PgValueRef<'r>,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let s = <&str as Decode<Postgres>>::decode(value)?;
        let heapless_str = HeaplessString::try_from(s).map_err(|_| {
            Box::new(DecodeError::StringTooLong {
                actual: s.len(),
                max: N,
            }) as Box<dyn Error + Send + Sync>
        })?;
        Ok(HeaplessStringWrapper(heapless_str))
    }
}
