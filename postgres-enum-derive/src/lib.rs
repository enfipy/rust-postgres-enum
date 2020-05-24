#![recursion_limit = "256"]
extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;

fn impl_postgres_enum_derive(input: syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    let name = &input.ident;
    let out = quote! {
        impl<'a> postgres_types::FromSql<'a> for #name {
            fn from_sql(
                _: &postgres_types::Type,
                raw: &[u8],
            ) -> std::result::Result<Self, Box<dyn std::error::Error + Sync + Send>> {
                use std::convert::TryFrom;
                let value = postgres_protocol::types::int2_from_sql(raw)?;
                Self::try_from(value).map_err(|_| Box::from("Failed to deserialize enum"))
            }

            postgres_types::accepts!(INT2);
        }

        impl postgres_types::ToSql for Number {
            fn to_sql(
                &self,
                _: &postgres_types::Type,
                out: &mut bytes::BytesMut,
            ) -> std::result::Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            where
                Self: Sized,
            {
                let res = *self as i16;
                postgres_protocol::types::int2_to_sql(res, out);
                Ok(postgres_types::IsNull::No)
            }

            postgres_types::accepts!(INT2);

            postgres_types::to_sql_checked!();
        }
    };
    Ok(out)
}

#[proc_macro_derive(FromToSqlEnum)]
pub fn derive_postgres_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse(input).unwrap();
    impl_postgres_enum_derive(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
