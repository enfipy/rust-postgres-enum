pub use postgres_enum_derive::*;

pub use bytes;
pub use num_enum;
pub use postgres_protocol;
pub use postgres_types;

#[cfg(test)]
mod tests {
    use super::*;
    use num_enum::TryFromPrimitive;

    #[derive(Clone, Copy, Debug, TryFromPrimitive, FromToSqlEnum)]
    #[repr(i16)]
    enum Number {
        One = 1,
        Two = 2,
    }

    #[test]
    fn test_derive() {}
}
