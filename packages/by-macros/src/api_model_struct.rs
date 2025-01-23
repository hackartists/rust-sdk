#![allow(dead_code)]

use convert_case::{Case, Casing};
use indexmap::IndexMap;
use proc_macro::TokenStream;
use quote::quote;
use std::convert::From;
use syn::{DataStruct, Field};

use crate::sql_model::{AutoOperation, SqlAttribute, SqlAttributeKey, SqlModel, SqlModelKey};

pub struct ApiModel {
    pub name: String,
    pub table_name: String,
    pub rename: Case,
    pub fields: IndexMap<String, ApiField>,
}

impl ApiModel {
    pub fn insert_function(&self) -> proc_macro2::TokenStream {
        let mut insert_fields = vec![];
        let mut insert_values = vec![];

        let mut i = 1;

        let mut returning = vec![];
        let mut return_bounds = vec![];
        let mut args = vec![];
        let mut binds = vec![];
        let mut type_bridges = vec![];

        for (field_name, field) in self.fields.iter() {
            let sql_field_name = syn::LitStr::new(&field_name, proc_macro2::Span::call_site());
            returning.push(field_name.clone());
            let n = field.field_name_token();
            let rust_type = &field.rust_type;

            if field.primary_key {
                type_bridges.push(quote! {
                    let #n: i64 = row.get(#sql_field_name);
                });
                return_bounds.push(quote! {
                    #n: format!("{}",  #n)
                });
            } else {
                if rust_type == "u64" || rust_type == "u32" {
                    let bridge_type = syn::Ident::new(
                        &rust_type.replace("u", "i"),
                        proc_macro2::Span::call_site(),
                    );
                    let real_type = syn::Ident::new(rust_type, proc_macro2::Span::call_site());
                    type_bridges.push(quote! {
                        let #n: #bridge_type = row.get(#sql_field_name);
                    });
                    return_bounds.push(quote! {
                        #n: #n as #real_type
                    });
                } else {
                    return_bounds.push(quote! {
                        #n: row.get(#sql_field_name)
                    });
                }
            }

            if field.omitted {
                continue;
            }

            args.push(field.arg_token());
            binds.push(quote! {
                .bind(#n)
            });

            insert_fields.push(field_name.clone());
            insert_values.push(format!("${}", i));

            i += 1;
        }

        let q = syn::LitStr::new(
            &format!(
                "INSERT INTO {} ({}) VALUES ({}) RETURNING {}",
                self.table_name,
                insert_fields.join(", "),
                insert_values.join(", "),
                returning.join(", "),
            ),
            proc_macro2::Span::call_site(),
        );
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());

        let output = quote! {
            pub async fn insert(&self, #(#args),*) -> Result<#name> {
                use sqlx::Row;

                let row = sqlx::query(#q)
                    #(#binds)*
                    .fetch_one(&self.pool)
                    .await?;

                #(#type_bridges)*
                Ok(#name {
                    #(#return_bounds),*
                })
            }
        };
        tracing::debug!("insert function output: {}", output);

        output.into()
    }
}

impl ApiModel {
    pub fn new(name: String, data: &DataStruct, attr: TokenStream) -> Self {
        let mut api_fields = IndexMap::new();

        let attrs = super::sql_model::parse_sql_model(attr);
        let table_name = match attrs.get(&SqlModelKey::Table) {
            Some(SqlModel::Table(name)) => name.to_string(),
            _ => name.to_case(Case::Snake),
        };

        let rename = match attrs.get(&SqlModelKey::Rename) {
            Some(SqlModel::Rename(rename)) => rename.clone(),
            _ => Case::Snake,
        };

        for f in data.fields.iter() {
            let field_name = f.clone().ident.unwrap().to_string().to_case(rename);
            let f = ApiField::from(f);

            api_fields.insert(field_name, f);
        }

        Self {
            fields: api_fields,
            name,
            table_name,
            rename,
        }
    }
}

pub enum Relation {
    ManyToMany {
        table_name: String,
        foreign_table_name: String,
        foreign_key: String,
        foreign_key_type: String,
    },
    ManyToOne {
        table_name: String,
        foreign_key: String,
        foreign_key_type: String,
    },
    OneToMany {
        #[allow(dead_code)]
        table_name: String,
        foreign_key: String,
    },
}

pub struct ApiField {
    pub name: String, // this is a native field name in rust
    pub primary_key: bool,
    pub relation: Option<Relation>,
    pub r#type: String,
    pub unique: bool,
    pub auto: Vec<AutoOperation>,
    pub nullable: bool,
    pub omitted: bool,
    pub rust_type: String,
}

impl ApiField {
    pub fn arg_token(&self) -> proc_macro2::TokenStream {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let rust_type = syn::Ident::new(&self.rust_type, proc_macro2::Span::call_site());

        let output = quote! {
            #name: #rust_type
        };

        output.into()
    }

    pub fn field_name_token(&self) -> proc_macro2::TokenStream {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());

        let output = quote! {
            #name
        };

        output.into()
    }
}

impl From<&Field> for ApiField {
    fn from(field: &Field) -> Self {
        let name = field.clone().ident.unwrap().to_string();
        let rust_type = match &field.ty {
            syn::Type::Path(ref type_path) => {
                type_path.path.segments.last().unwrap().ident.to_string()
            }
            _ => "".to_string(),
        };

        let f = super::sql_model::parse_field_attr(field);
        let primary_key = f.attrs.contains_key(&SqlAttributeKey::PrimaryKey);

        let relation = match f.attrs.get(&SqlAttributeKey::ManyToMany) {
            Some(SqlAttribute::ManyToMany {
                table_name,
                foreign_table_name,
                foreign_key,
                foreign_key_type,
            }) => Some(Relation::ManyToMany {
                table_name: table_name.to_string(),
                foreign_table_name: foreign_table_name.to_string(),
                foreign_key: foreign_key.to_string(),
                foreign_key_type: foreign_key_type.to_string(),
            }),

            Some(SqlAttribute::ManyToOne {
                table_name,
                foreign_key,
                foreign_key_type,
            }) => Some(Relation::ManyToOne {
                table_name: table_name.to_string(),
                foreign_key: foreign_key.to_string(),
                foreign_key_type: foreign_key_type.to_string(),
            }),

            Some(SqlAttribute::OneToMany {
                table_name,
                foreign_key,
            }) => Some(Relation::OneToMany {
                table_name: table_name.to_string(),
                foreign_key: foreign_key.to_string(),
            }),

            _ => None,
        };

        let ((r#type, nullable), failed_type_inference) =
            match f.attrs.get(&SqlAttributeKey::SqlType) {
                Some(SqlAttribute::SqlType(t)) => ((t.to_string(), true), false),
                _ => match to_type(&field.ty) {
                    Some(t) => (t, false),
                    None => {
                        tracing::debug!("field type: {:?}", field.ty);
                        (("TEXT".to_string(), true), true)
                    }
                },
            };

        let auto: Vec<AutoOperation> = match f.attrs.get(&SqlAttributeKey::Auto) {
            Some(SqlAttribute::Auto(ops)) => ops.to_vec(),
            _ => vec![],
        };

        let omitted = failed_type_inference
            || match relation {
                Some(Relation::OneToMany { .. }) => true,
                _ => false,
            }
            || primary_key
            || !auto.is_empty();

        let unique = f.attrs.contains_key(&SqlAttributeKey::Unique);

        Self {
            name,
            primary_key,
            relation,
            r#type,
            unique,
            auto,
            nullable,
            omitted,
            rust_type,
        }
    }
}

fn to_type(var_type: &syn::Type) -> Option<(String, bool)> {
    let mut nullable = false;

    let name = match var_type {
        syn::Type::Path(ref type_path) => {
            let type_ident = type_path.path.segments.last().unwrap().ident.to_string();
            match type_ident.as_str() {
                "u64" | "i64" => "BIGINT".to_string(),
                "String" => "TEXT".to_string(),
                "bool" => "BOOLEAN".to_string(),
                "i32" => "INTEGER".to_string(),
                "f64" => "DOUBLE PRECISION".to_string(),

                "Option<u64>" | "Option<i64>" => {
                    nullable = true;
                    "BIGINT".to_string()
                }
                "Option<String>" => {
                    nullable = true;
                    "TEXT".to_string()
                }
                "Option<bool>" => {
                    nullable = true;
                    "BOOLEAN".to_string()
                }
                "Option<i32>" => {
                    nullable = true;
                    "INTEGER".to_string()
                }
                "Option<f64>" => {
                    nullable = true;
                    "DOUBLE PRECISION".to_string()
                }

                _ => return None,
            }
        }
        _ => {
            tracing::debug!("field type: {:?}", var_type);
            return None;
        }
    };

    Some((name, nullable))
}
