use postgres_enum::FromToSqlEnum;
use postgres_enum::num_enum::TryFromPrimitive;

#[derive(Clone, Copy, Debug, TryFromPrimitive, FromToSqlEnum)]
#[repr(i16)]
enum Number {
    One = 1,
    Two = 2,
}

#[test]
fn test_derive() {}
