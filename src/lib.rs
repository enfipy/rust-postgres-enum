#![feature(proc_macro_diagnostic, proc_macro_span)]
#![feature(core_intrinsics, decl_macro)]
#![recursion_limit = "256"]

#[macro_use]
extern crate quote;
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;

fn impl_serde_enum_derive(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl postgres::types::FromSql for #name {
            fn from_sql(_: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
                let value = postgres_protocol::types::int8_from_sql(raw)?;
                Self::from_i64(value).ok_or(Box::from("Failed to deserialize enum"))
            }

            fn accepts(ty: &Type) -> bool {
                <i64 as ToSql>::accepts(ty)
            }
        }

        impl postgres::types::ToSql for #name {
            fn to_sql(&self, _: &Type, out: &mut Vec<u8>) -> Result<IsNull, Box<dyn Error + Sync + Send>>
            where
                Self: Sized,
            {
                let res = self.to_i64().ok_or::<Box<dyn Error + Sync + Send>>(Box::from("Failed to serialize enum"))?;
                postgres_protocol::types::int8_to_sql(res, out);
                Ok(IsNull::No)
            }

            fn accepts(ty: &Type) -> bool {
                <i64 as ToSql>::accepts(ty)
            }

            to_sql_checked!();
        }
    }
}

fn impl_serde_enum_struct_derive(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl postgres::types::FromSql for #name {
            fn from_sql(
                _: &postgres::types::Type,
                raw: &[u8],
            ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
                serde_json::from_slice(&raw).map_err(|_| Box::from("FATAL: Failed to deserialize data"))
            }

            fn accepts(ty: &postgres::types::Type) -> bool {
                <serde_json::Value as postgres::types::FromSql>::accepts(ty)
            }
        }

        impl postgres::types::ToSql for #name {
            fn to_sql(
                &self,
                _: &postgres::types::Type,
                out: &mut Vec<u8>,
            ) -> Result<postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            where
                Self: Sized,
            {
                *out = serde_json::to_vec(self)?;
                Ok(postgres::types::IsNull::No)
            }

            fn accepts(ty: &postgres::types::Type) -> bool {
                serde_json::Value::accepts(ty)
            }

            to_sql_checked!();
        }
    }
}

#[proc_macro_derive(EnumSql)]
pub fn serde_enum_derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_serde_enum_derive(&ast);
    gen.parse().unwrap()
}

#[proc_macro_derive(JsonEnumSql)]
pub fn serde_json_enum_derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_serde_enum_struct_derive(&ast);
    gen.parse().unwrap()
}
