#![allow(dead_code, unused)]

use crate::api_model::*;
use convert_case::{Case, Casing};
use indexmap::IndexMap;
use proc_macro::TokenStream;
use quote::quote;
use std::{collections::HashSet, convert::From};
use syn::{DataStruct, Field};

#[cfg(feature = "server")]
use crate::sql_model::{AutoOperation, SqlAttribute, SqlAttributeKey, SqlModel, SqlModelKey};

pub struct ApiModel<'a> {
    #[cfg(feature = "server")]
    pub table_name: String,
    #[cfg(feature = "server")]
    pub rename: Case,
    #[cfg(feature = "server")]
    pub fields: IndexMap<String, ApiField>,

    pub name: String,
    pub name_id: &'a syn::Ident,

    pub iter_type: String,
    pub base: String,
    pub parent_ids: Vec<String>,
    pub summary_fields: Vec<Field>,
    pub queryable_fields: Vec<Field>,
    pub action_names: IndexMap<String, ActionField>,
    pub action_by_id_names: IndexMap<String, ActionField>,
    pub query_action_names: IndexMap<String, ActionField>,
    pub read_action_names: IndexMap<String, ActionField>,
}

impl std::fmt::Debug for ApiModel<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "server")]
        return f
            .debug_struct("ApiModel")
            .field("table_name", &self.table_name)
            .field("rename", &self.rename)
            .field("name", &self.name)
            .field("iter_type", &self.iter_type)
            .field("base", &self.base)
            .field("parent_ids", &self.parent_ids)
            .field("summary_fields", &self.summary_fields)
            .field("queryable_fields", &self.queryable_fields)
            .finish();

        #[cfg(not(feature = "server"))]
        f.debug_struct("ApiModel")
            .field("name", &self.name)
            .field("iter_type", &self.iter_type)
            .field("base", &self.base)
            .field("parent_ids", &self.parent_ids)
            .field("summary_fields", &self.summary_fields)
            .field("queryable_fields", &self.queryable_fields)
            .finish()
    }
}

impl ApiModel<'_> {
    pub fn query_fields(&self) -> Vec<syn::Ident> {
        let mut hashed_fields = HashSet::new();
        let mut fields = vec![];

        for (_, f) in &self.query_action_names {
            match f {
                ActionField::Fields(v) => {
                    for field in v {
                        let field_name = &field.ident;

                        if hashed_fields.contains(field_name) {
                            continue;
                        }
                        hashed_fields.insert(field_name.clone());
                        fields.push(field_name.clone().unwrap());
                    }
                }
                _ => {
                    panic!("Related field should not be in queryable fields");
                }
            }
        }

        for f in self.queryable_fields.iter() {
            let field_name = &f.ident;

            if hashed_fields.contains(field_name) {
                continue;
            }
            hashed_fields.insert(field_name.clone());
            fields.push(field_name.clone().unwrap());
        }

        tracing::debug!("Query fields: {:?}", fields);

        fields
    }

    pub fn read_action_fields(&self) -> Vec<syn::Ident> {
        let mut hashed_fields = HashSet::new();
        let mut fields = vec![];

        for (_, f) in &self.read_action_names {
            match f {
                ActionField::Fields(v) => {
                    for field in v {
                        let field_name = &field.ident;

                        if hashed_fields.contains(field_name) {
                            continue;
                        }
                        hashed_fields.insert(field_name.clone());
                        fields.push(field_name.clone().unwrap());
                    }
                }
                _ => {
                    panic!("Related field should not be in queryable fields");
                }
            }
        }

        tracing::debug!("Read action fields: {:?}", fields);

        fields
    }
    pub fn struct_name(&self) -> syn::Ident {
        syn::Ident::new(&self.name, proc_macro2::Span::call_site())
    }

    pub fn read_action_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}ReadAction", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn query_action_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}Query", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn action_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}Action", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn action_by_id_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}ActionById", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn summary_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}Summary", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn queryable_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}Query", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn repository_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}Repository", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn client_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}Client", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn param_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}Param", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn get_response_struct_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("{}GetResponse", self.name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn iter_type_name(&self) -> proc_macro2::TokenStream {
        if self.should_have_summary() {
            format!("Vec<{}Summary>", self.name)
        } else {
            format!("Vec<{}>", self.name)
        }
        .parse()
        .unwrap()
    }

    pub fn param_struct(&self) -> proc_macro2::TokenStream {
        let name = self.param_struct_name();
        let query = self.queryable_struct_name();
        let read = self.read_action_struct_name();
        let mut enums = vec![];

        enums.push(quote! {
            Query(#query),
        });

        if self.should_have_read_action() {
            enums.push(quote! {
                Read(#read),
            });
        }

        let output = quote! {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Eq, PartialEq, by_macros::QueryDisplay)]
            #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
            #[serde(tag = "param-type")]
            #[serde(rename_all = "kebab-case")]
            pub enum #name {
                #(#enums)*
            }
        };

        output.into()
    }

    pub fn should_have_queryable(&self) -> bool {
        !self.queryable_fields.is_empty() & !self.query_action_names.is_empty()
    }

    pub fn should_have_summary(&self) -> bool {
        !self.summary_fields.is_empty()
    }

    pub fn should_have_action(&self) -> bool {
        !self.action_names.is_empty()
    }

    pub fn should_have_action_by_id(&self) -> bool {
        !self.action_by_id_names.is_empty()
    }

    pub fn should_have_read_action(&self) -> bool {
        !self.read_action_names.is_empty()
    }

    pub fn get_response_struct(&self) -> proc_macro2::TokenStream {
        let name = self.name_id;
        let mut enums = vec![];
        let iter_type = self.iter_type_name();

        enums.push(quote! {
            Query(#iter_type),
        });

        if self.should_have_read_action() {
            enums.push(quote! {
                Read(#name),
            });
        }

        let response = self.get_response_struct_name();

        let output = quote! {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
            #[serde(tag = "param_type")]
            #[serde(rename_all = "snake_case")]
            #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
            pub enum #response {
                #(#enums)*
            }
        };

        output.into()
    }
}

#[cfg(feature = "server")]
impl ApiModel<'_> {
    pub fn find_function(&self) -> proc_macro2::TokenStream {
        let name = self.iter_type_name();
        let query_struct = self.query_action_struct_name();
        let fields = self.query_fields();

        let q = syn::LitStr::new(
            &format!("SELECT * FROM {}", self.table_name),
            proc_macro2::Span::call_site(),
        );
        let qc = syn::LitStr::new(
            &format!("SELECT COUNT(*) FROM {}", self.table_name),
            proc_macro2::Span::call_site(),
        );

        let tail_q = syn::LitStr::new(
            &format!("LIMIT ${} OFFSET ${}", 1, 2),
            proc_macro2::Span::call_site(),
        );

        let mut binds = vec![];
        let mut where_clause = vec![];
        let fmt_str = syn::LitStr::new(
            &format!("{}Repository::find_one", self.name),
            proc_macro2::Span::call_site(),
        );

        for f in fields.iter() {
            let fname = syn::LitStr::new(&f.to_string(), proc_macro2::Span::call_site());

            binds.push(quote! {
                if let Some(#f) = &param.#f {
                    tracing::debug!("{} binding {} = {}", #fmt_str, #fname, #f);
                    q = q.bind(#f);
                }
            });

            where_clause.push(quote! {
                if let Some(#f) = &param.#f {
                    i += 1;
                    where_clause.push(format!("{} = ${}", #fname, i));
                }
            });
        }

        let call_map = self.call_map_iter_type();
        let declare_where_clause = if fields.len() > 0 {
            quote! {
                let mut i = 2;
                let mut where_clause = vec![];
            }
        } else {
            quote! {}
        };
        let compose_query = if fields.len() > 0 {
            quote! {
                let where_clause_str = where_clause.join(" AND ");
                let query = if where_clause.len() > 0 {
                    format!("{} WHERE {} {}", #q, where_clause_str, #tail_q)
                } else {
                    format!("{} {}", #q, #tail_q)
                };

                let count_query = if where_clause.len() > 0 {
                    format!("{} WHERE {}", #qc, where_clause_str)
                } else {
                    format!("{}", #qc)
                };
                let query = format!("WITH data AS ({}) SELECT ({}) AS total_count, data.* FROM data;", query, count_query);
                tracing::debug!("{} query {}", #fmt_str, query);
                let offset: i32 = (param.size as i32) * (param.page() - 1);
                let mut q = sqlx::query(&query).bind(param.size as i32).bind(offset);
            }
        } else {
            quote! {
                let query = format!("WITH data AS ({} {}) SELECT ({}) AS total_count, data.* FROM data;", #q, #tail_q, #qc);
                tracing::debug!("{} query {}", #fmt_str, query);
                let offset: i32 = (param.size as i32) * (param.page() - 1);
                let q = sqlx::query(&query).bind(param.size as i32).bind(offset);
            }
        };

        let output = quote! {
            pub async fn find(&self, param: &#query_struct) -> Result<(#name, i64)> {
                use sqlx::Row;

                #declare_where_clause
                #(#where_clause)*

                #compose_query

                #(#binds)*
                let mut total = 0;
                let rows = q
                    #call_map
                .fetch_all(&self.pool).await?;

                Ok((rows, total))
            }
        };

        output.into()
    }

    pub fn find_one_function(&self) -> proc_macro2::TokenStream {
        let name = self.name_id;
        let read_action = self.read_action_struct_name();
        let fields = self.read_action_fields();

        let q = syn::LitStr::new(
            &format!("SELECT * FROM {}", self.table_name),
            proc_macro2::Span::call_site(),
        );

        let mut binds = vec![];
        let mut where_clause = vec![];
        let fmt_str = syn::LitStr::new(
            &format!("{}Repository::find_one", self.name),
            proc_macro2::Span::call_site(),
        );

        for f in fields.iter() {
            let fname = syn::LitStr::new(&f.to_string(), proc_macro2::Span::call_site());

            binds.push(quote! {
                if let Some(#f) = &param.#f {
                    tracing::debug!("{} binding {} = {}", #fmt_str, #fname, #f);
                    q = q.bind(#f);
                }
            });

            where_clause.push(quote! {
                if let Some(#f) = &param.#f {
                    i += 1;
                    where_clause.push(format!("{} = ${}", #fname, i));
                }
            });
        }

        let call_map = self.call_map();

        let output = quote! {
            pub async fn find_one(&self, param: &#read_action) -> Result<#name> {
                use sqlx::Row;
                let mut i = 0;
                let mut where_clause = vec![];
                #(#where_clause)*

                let where_clause_str = where_clause.join(" AND ");
                let query = if where_clause.len() > 0 {
                    format!("{} WHERE {}", #q, where_clause_str)
                } else {
                    format!("{}", #q)
                };
                tracing::debug!("{} query {}", #fmt_str, query);

                let mut q = sqlx::query(&query);

                #(#binds)*
                let row = q
                    #call_map
                .fetch_one(&self.pool).await?;

                Ok(row)
            }
        };

        output.into()
    }

    pub fn call_map_iter_type(&self) -> proc_macro2::TokenStream {
        if self.should_have_summary() {
            self.call_map_summary()
        } else {
            self.call_map_iter()
        }
    }

    pub fn call_map_summary(&self) -> proc_macro2::TokenStream {
        let name = self.summary_struct_name();
        let mut type_bridges = vec![];
        let mut return_bounds = vec![];

        for field in self.summary_fields.iter() {
            let field = ApiField::from(field);
            let field_name = field.name.clone();

            let sql_field_name = syn::LitStr::new(&field_name, proc_macro2::Span::call_site());
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
        }

        let output = quote! {
            .map(|row: sqlx::postgres::PgRow| {
                total = row.get("total_count");
                #(#type_bridges)*
                #name {
                    #(#return_bounds),*
                }
            })
        };

        output.into()
    }

    pub fn call_map_iter(&self) -> proc_macro2::TokenStream {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let mut type_bridges = vec![];
        let mut return_bounds = vec![];

        for (field_name, field) in self.fields.iter() {
            let sql_field_name = syn::LitStr::new(&field_name, proc_macro2::Span::call_site());
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
        }

        let output = quote! {
            .map(|row: sqlx::postgres::PgRow| {
                total = row.get("total_count");

                #(#type_bridges)*
                #name {
                    #(#return_bounds),*
                }
            })
        };

        output.into()
    }

    pub fn call_map(&self) -> proc_macro2::TokenStream {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let mut type_bridges = vec![];
        let mut return_bounds = vec![];

        for (field_name, field) in self.fields.iter() {
            let sql_field_name = syn::LitStr::new(&field_name, proc_macro2::Span::call_site());
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
        }

        let output = quote! {
            .map(|row: sqlx::postgres::PgRow| {
                #(#type_bridges)*
                #name {
                    #(#return_bounds),*
                }
            })
        };

        output.into()
    }

    pub fn insert_function(&self) -> proc_macro2::TokenStream {
        let mut insert_fields = vec![];
        let mut insert_values = vec![];

        let mut i = 1;

        let mut returning = vec![];
        let mut args = vec![];
        let mut binds = vec![];

        for (field_name, field) in self.fields.iter() {
            returning.push(field_name.clone());
            let n = field.field_name_token();

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
        let call_map = self.call_map();

        let output = quote! {
            pub async fn insert(&self, #(#args),*) -> Result<#name> {
                use sqlx::Row;

                let row = sqlx::query(#q)
                    #(#binds)*
                #call_map
                .fetch_one(&self.pool)
                    .await?;

                Ok(row)
            }
        };
        tracing::debug!("insert function output: {}", output);

        output.into()
    }

    pub fn select_functions(&self) -> proc_macro2::TokenStream {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let read_action = self.read_action_struct_name();

        let q = syn::LitStr::new(
            &format!("SELECT * FROM {} LIMIT 1", self.table_name),
            proc_macro2::Span::call_site(),
        );

        let output = quote! {
        //         pub async fn select_by_id(&self, id: i64) -> Result<#name> {
        //             let row = sqlx::query_as!(#name, "SELECT * FROM {} WHERE id = $1", id)
        //                 .fetch_one(&self.pool)
        //                 .await?;

        //             Ok(row)
        //         }

                pub async fn find_one(&self, param: #read_action) -> Result<#name> {
                    let rows = sqlx::query("SELECT * FROM $1 LIMIT 1 OFFSET $3", size, (page - 1) * size)
                        .fetch_all(&self.pool)
                        .await?;
                    let rows = sqlx::query_as!(#name, "SELECT * FROM {}", self.table_name)
                        .fetch_all(&self.pool)
                        .await?;

                    Ok(rows)
                }
            };

        output.into()
    }
}

impl<'a> ApiModel<'a> {
    pub fn new(name_id: &'a syn::Ident, data: &DataStruct, attr: TokenStream) -> Self {
        #[cfg(feature = "server")]
        let mut api_fields = IndexMap::new();
        let name = name_id.to_string();

        let mut base = String::new();
        let mut parent_ids = Vec::new();
        let mut iter_type = "Vec".to_string();
        let mut read_action_names = IndexMap::<String, ActionField>::new();

        for arg in attr.to_string().split(',') {
            let parts: Vec<&str> = arg.split('=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim().trim_matches('"');
                match key {
                    "base" => {
                        base = value
                            .split('/')
                            .map(|v| {
                                if v.starts_with(':') {
                                    parent_ids.push(
                                        v.trim_start_matches(':').to_string().to_case(Case::Snake),
                                    );
                                    "{}"
                                } else {
                                    v
                                }
                            })
                            .collect::<Vec<&str>>()
                            .join("/");
                    }
                    "iter_type" => iter_type = value.to_string(),
                    "read_action" => {
                        let value = value
                            .trim_matches('[')
                            .trim_matches(']')
                            .split(",")
                            .collect::<Vec<&str>>();
                        for v in value {
                            tracing::debug!("Read action: {}", v);
                            let v = v.trim();
                            read_action_names.insert(v.to_string(), ActionField::Fields(vec![]));
                        }
                    }
                    _ => {}
                }
            }
        }

        #[cfg(feature = "server")]
        let attrs = super::sql_model::parse_sql_model(attr);
        #[cfg(feature = "server")]
        let table_name = match attrs.get(&SqlModelKey::Table) {
            Some(SqlModel::Table(name)) => name.to_string(),
            _ => name.to_case(Case::Snake),
        };

        #[cfg(feature = "server")]
        let rename = match attrs.get(&SqlModelKey::Rename) {
            Some(SqlModel::Rename(rename)) => rename.clone(),
            _ => Case::Snake,
        };

        let mut summary_fields = Vec::new();
        let mut queryable_fields = Vec::new();
        let mut action_names = IndexMap::<String, ActionField>::new();
        let mut action_by_id_names = IndexMap::<String, ActionField>::new();
        let mut query_action_names = IndexMap::<String, ActionField>::new();

        #[cfg(feature = "server")]
        for f in data.fields.iter() {
            let field_name = f.clone().ident.unwrap().to_string().to_case(rename);
            let f = ApiField::from(f);

            api_fields.insert(field_name, f);
        }

        if let syn::Fields::Named(named_fields) = &data.fields {
            for field in &named_fields.named {
                for attr in &field.attrs {
                    let mut actions = vec![];
                    let mut related = None::<String>;

                    for t in parse_action_attr(attr) {
                        match t {
                            ActionType::Summary => {
                                summary_fields.push(field.clone());
                            }
                            ActionType::Queryable => {
                                queryable_fields.push(field.clone());
                            }
                            ActionType::Action(action_names) => {
                                actions.push(ActionType::Action(action_names));
                            }
                            ActionType::ActionById(action_names) => {
                                actions.push(ActionType::ActionById(action_names));
                            }
                            ActionType::Related(st) => {
                                related = Some(st);
                            }
                            ActionType::QueryActions(action_names) => {
                                actions.push(ActionType::QueryActions(action_names));
                            }
                            ActionType::ReadActions(action_names) => {
                                actions.push(ActionType::ReadActions(action_names));
                            }
                        }
                    }

                    for action in actions {
                        match (related.clone(), action) {
                            (Some(st), ActionType::Action(actions)) => {
                                for action_name in actions {
                                    action_names
                                        .entry(action_name)
                                        .or_insert_with(|| ActionField::Related(st.clone()));
                                }
                            }
                            (Some(st), ActionType::ActionById(actions)) => {
                                for action_name in actions {
                                    action_by_id_names
                                        .entry(action_name)
                                        .or_insert_with(|| ActionField::Related(st.clone()));
                                }
                            }
                            (None, ActionType::Action(actions)) => {
                                for action_name in actions {
                                    match action_names
                                        .entry(action_name)
                                        .or_insert_with(|| ActionField::Fields(vec![]))
                                    {
                                        ActionField::Fields(v) => {
                                            v.push(field.clone());
                                        }

                                        _ => {
                                            panic!("Action should have fields")
                                        }
                                    };
                                }
                            }
                            (None, ActionType::ActionById(actions)) => {
                                for action_name in actions {
                                    match action_by_id_names
                                        .entry(action_name)
                                        .or_insert_with(|| ActionField::Fields(vec![]))
                                    {
                                        ActionField::Fields(v) => {
                                            v.push(field.clone());
                                        }
                                        _ => {
                                            panic!("ActionById should have fields")
                                        }
                                    };
                                }
                            }
                            (_, ActionType::QueryActions(actions)) => {
                                for action_name in actions {
                                    match query_action_names
                                        .entry(action_name)
                                        .or_insert_with(|| ActionField::Fields(vec![]))
                                    {
                                        ActionField::Fields(v) => {
                                            v.push(field.clone());
                                        }
                                        _ => {
                                            panic!("ActionById should have fields")
                                        }
                                    };
                                }
                            }
                            (_, ActionType::ReadActions(actions)) => {
                                for action_name in actions {
                                    match read_action_names
                                        .entry(action_name)
                                        .or_insert_with(|| ActionField::Fields(vec![]))
                                    {
                                        ActionField::Fields(v) => {
                                            v.push(field.clone());
                                        }
                                        _ => {
                                            panic!("ActionById should have fields")
                                        }
                                    };
                                }
                            }

                            _ => {}
                        }
                    }
                }
            }
        }

        Self {
            #[cfg(feature = "server")]
            fields: api_fields,
            #[cfg(feature = "server")]
            table_name,
            #[cfg(feature = "server")]
            rename,

            name,
            name_id,
            iter_type,
            base,
            read_action_names,
            parent_ids,

            summary_fields,
            queryable_fields,
            action_names,
            action_by_id_names,
            query_action_names,
        }
    }
}

#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
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

    pub summary: bool,
    pub queryable: bool,
    pub action_names: Vec<String>,
    pub action_by_id_names: Vec<String>,
    pub query_action_names: Vec<String>,
    pub read_action_names: Vec<String>,
    pub related: Option<String>,
}

#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
impl From<&Field> for ApiField {
    fn from(field: &Field) -> Self {
        let name = field.clone().ident.unwrap().to_string();
        let rust_type = match &field.ty {
            syn::Type::Path(ref type_path) => {
                type_path.path.segments.last().unwrap().ident.to_string()
            }
            _ => "".to_string(),
        };

        let mut summary = false;
        let mut queryable = false;
        let mut action_names = Vec::new();
        let mut action_by_id_names = Vec::new();
        let mut query_action_names = Vec::new();
        let mut read_action_names = Vec::new();
        let mut related = None;

        for attr in &field.attrs {
            for t in crate::api_model::parse_action_attr(attr) {
                match t {
                    ActionType::Summary => {
                        summary = true;
                    }
                    ActionType::Queryable => {
                        queryable = true;
                    }
                    ActionType::Action(an) => {
                        action_names = an;
                    }
                    ActionType::ActionById(action_names) => {
                        action_by_id_names = action_names;
                    }
                    ActionType::Related(st) => {
                        related = Some(st);
                    }
                    ActionType::QueryActions(action_names) => {
                        query_action_names = action_names;
                    }
                    ActionType::ReadActions(action_names) => {
                        read_action_names = action_names;
                    }
                }
            }
        }

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
            summary,
            queryable,
            action_names,
            action_by_id_names,
            query_action_names,
            read_action_names,
            related,
        }
    }
}

#[cfg(feature = "server")]
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
