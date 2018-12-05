// Based on:
// - https://keens.github.io/blog/2017/12/16/dieselshounetashuu/ (old)
// - https://github.com/diesel-rs/diesel/pull/1453
/// Define enum that can be used as a column.
macro_rules! enum_column {
    (
        $(#[$meta:meta])*
        pub enum $name:ident { $($variant:ident = $val:expr,)* }
    ) => {
        use diesel::sql_types::SmallInt;
        use diesel::serialize::ToSql;
        use diesel::deserialize::FromSql;

        $(#[$meta])*
        #[derive(FromSqlRow, AsExpression)]
        #[sql_type = "SmallInt"]
        pub enum $name {
            $($variant = $val,)*
        }

        impl<DB: diesel::backend::Backend> ToSql<SmallInt, DB> for $name {
            fn to_sql<W: std::io::Write>(
                &self,
                out: &mut diesel::serialize::Output<W, DB>,
            ) -> Result<diesel::serialize::IsNull, Box<std::error::Error + Send + Sync>> {
                ToSql::<SmallInt, DB>::to_sql(&(*self as i16), out)
            }
        }

        impl<DB: diesel::backend::Backend> FromSql<SmallInt, DB> for $name
        where
            i16: FromSql<SmallInt, DB>,
        {
            fn from_sql(
                bytes: Option<&DB::RawValue>,
            ) -> Result<Self, Box<std::error::Error + Send + Sync>> {
                use self::$name::*;

                match <i16 as FromSql<SmallInt, DB>>::from_sql(bytes)? {
                    $($val => Ok($variant),)*
                    s => Err(format!("invalid {} value: {}", stringify!($name), s).into()),
                }
            }
        }
    }
}
