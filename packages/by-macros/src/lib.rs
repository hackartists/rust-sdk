extern crate proc_macro;

mod action;
mod api_model;
mod api_model_struct;
mod enum_prop;
#[cfg(feature = "server")]
mod query_builder_functions;
mod query_display;
#[cfg(feature = "server")]
mod sql_model;

use api_model::api_model_impl;
use enum_prop::enum_prop_impl;
use proc_macro::TokenStream;
use query_display::query_display_impl;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput};

#[proc_macro_derive(QueryDisplay)]
pub fn query_display_derive(input: TokenStream) -> TokenStream {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();
    query_display_impl(input)
}

#[proc_macro_attribute]
pub fn api_model(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();
    api_model_impl(attr.into(), item.into()).into()
}

#[proc_macro_derive(EnumProp)]
pub fn enum_prop_derive(input: TokenStream) -> TokenStream {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();
    enum_prop_impl(input)
}

#[proc_macro_derive(ApiModel)]
pub fn derive_api_model(input: TokenStream) -> TokenStream {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();

    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let Data::Enum(DataEnum { variants, .. }) = &input.data else {
        return syn::Error::new_spanned(input.ident, "ApiModel can only be derived for enums")
            .to_compile_error()
            .into();
    };

    let try_from_arms = variants.iter().map(|v| {
        let ident = &v.ident;
        let discriminant = match &v.discriminant {
            Some((_, expr)) => quote! { #expr },
            None => quote! { compile_error!("Enum variants must have explicit discriminants"); },
        };
        tracing::trace!("discriminant: {}", discriminant.to_string());
        quote! { #discriminant => Ok(#name::#ident), }
    });

    let expanded = quote! {
        impl std::convert::TryFrom<i32> for #name {
            type Error = String;

            fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
                match value {
                    #(#try_from_arms)*
                    _ => Err(format!("Invalid {}: {}", stringify!(#name), value)),
                }
            }
        }

        impl std::convert::Into<i32> for #name {
            fn into(self) -> i32 {
                self as i32
            }
        }

        #[cfg(feature = "server")]
        impl sqlx::Type<sqlx::Postgres> for #name {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                <i32 as sqlx::Type<sqlx::Postgres>>::type_info()
            }
        }

        #[cfg(feature = "server")]
        impl sqlx::Encode<'_, sqlx::Postgres> for #name {
            fn encode_by_ref(
                &self,
                buf: &mut sqlx::postgres::PgArgumentBuffer,
            ) -> std::result::Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
                let value: i32 = (*self).clone().into();
                <i32 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&value, buf)
            }
        }

        #[cfg(feature = "server")]
        impl<'r> sqlx::Decode<'r, sqlx::Postgres> for #name {
            fn decode(
                value: sqlx::postgres::PgValueRef<'r>,
            ) -> std::result::Result<Self, sqlx::error::BoxDynError> {
                let int_value: i32 = <i32 as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
                #name::try_from(int_value)
                    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)).into())
            }
        }

        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_i32(self.clone() as i32)
            }
        }

        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = i32::deserialize(deserializer)?;
                Self::try_from(value)
                    .map_err(|v| serde::de::Error::custom(format!("Failed to parse ApiModel: {}", v)))
            }
        }
    };

    tracing::trace!("ApiModel expanded: {}", expanded.to_string());

    TokenStream::from(expanded)
}
