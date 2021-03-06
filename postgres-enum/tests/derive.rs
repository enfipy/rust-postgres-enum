use num_enum::TryFromPrimitive;
use postgres_enum::FromToSqlEnum;

#[derive(Clone, Copy, Debug, TryFromPrimitive, FromToSqlEnum)]
#[repr(i16)]
enum Number {
    One = 1,
    Two = 2,
}

#[test]
fn test_derive() {}
