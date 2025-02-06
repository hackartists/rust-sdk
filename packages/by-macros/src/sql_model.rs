use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{parse_macro_input, DeriveInput, Field, Meta};

use crate::api_model_struct::ApiModel;

pub fn sql_model_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let repo_name = syn::Ident::new(&format!("{name}Repository"), name.span());

    let model = ApiModel::new(&input, attr.clone());
    if model.database.is_none() {
        return quote! {}.into();
    }

    let create_table_function = model.queries();
    let drop_table_function = model.drop_function();
    let insert = model.insert_function();
    let delete = model.delete_function();
    let update = model.update_function();
    let find_one = model.find_one_function();
    let find = model.find_function();
    let from_trait = model.from_pg_row_trait();
    let summary_trait = model.from_pg_row_summary_trait();
    let impl_functions = model.impl_functions();
    let impl_summary_functions = model.impl_summary_functions();
    let update_req_st = model.repo_update_request();

    let output = quote! {
        impl #name {
            pub fn get_repository(pool: sqlx::Pool<sqlx::Postgres>) -> #repo_name {
                #repo_name::new(pool)
            }
        }

        #impl_functions
        #impl_summary_functions

        #[derive(Debug, Clone)]
        pub struct #repo_name {
            pool: sqlx::Pool<sqlx::Postgres>,
        }

        #update_req_st

        impl #repo_name {
            pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
                Self { pool }
            }

            #create_table_function
            #drop_table_function
            #insert
            #update
            #delete
            #find_one
            #find
        }

        #from_trait
        #summary_trait
    };

    tracing::trace!("Generated code: {}", output.to_string());

    output.into()
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum SqlAttributeKey {
    PrimaryKey,
    SqlType,
    Relation,
    Unique,
    Auto,
    Version,
    Nullable,
    Aggregator,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AutoOperation {
    Insert,
    Update,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Aggregator {
    Count,
    Exist,
    Sum(String),
    Avg(String),
    Max(String),
    Min(String),
}

#[derive(Debug)]
pub enum SqlAttribute {
    PrimaryKey,
    SqlType(String),
    ManyToMany {
        // Table name of the join table
        table_name: String,
        // Foreign table name
        foreign_table_name: String,
        // Primary key in the foreign table (default: id)
        foreign_key: String,
        // Type of the primary key in the foreign table (default: BIGINT)
        foreign_key_type: String,
        // Reference key of foreign table in the join table
        foreign_primary_key: String,
        // Reference key of the current table in the join table
        foreign_reference_key: String,
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
    Unique,
    Auto(Vec<AutoOperation>),
    Version(String),
    Nullable,
    Aggregator(Aggregator),
}

#[derive(Debug)]
enum OpenedOffset {
    None,
    Type,
    ManyToMany,
    ManyToOne,
    OneToMany,
    ForeignTableName,
    ForeignKey,
    ForeignKeyType,
    ForeignPrimaryKey,
    ForeignReferenceKey,
    Auto,
    Version,
    Aggregator,
}

#[derive(Debug)]
pub struct SqlAttributes {
    pub attrs: HashMap<SqlAttributeKey, SqlAttribute>,
}

pub fn parse_field_attr(field: &Field) -> SqlAttributes {
    let mut field_attrs = HashMap::new();
    let name = field
        .ident
        .clone()
        .unwrap()
        .to_string()
        .to_case(Case::Snake);

    for attr in &field.attrs {
        if let Meta::List(meta_list) = attr.meta.clone() {
            if meta_list.path.is_ident("api_model") {
                let mut opened = OpenedOffset::None;

                for nested in meta_list.tokens.clone() {
                    if let proc_macro2::TokenTree::Ident(iden) = nested {
                        let id = iden.to_string();
                        match id.as_str() {
                            "primary_key" => {
                                field_attrs
                                    .insert(SqlAttributeKey::PrimaryKey, SqlAttribute::PrimaryKey);
                            }
                            "nullable" => {
                                field_attrs
                                    .insert(SqlAttributeKey::Nullable, SqlAttribute::Nullable);
                            }
                            "unique" => {
                                field_attrs.insert(SqlAttributeKey::Unique, SqlAttribute::Unique);
                            }
                            "type" => {
                                opened = OpenedOffset::Type;
                            }
                            "many_to_many" => {
                                opened = OpenedOffset::ManyToMany;
                            }
                            "many_to_one" => {
                                opened = OpenedOffset::ManyToOne;
                            }
                            "one_to_many" => {
                                opened = OpenedOffset::OneToMany;
                            }
                            "foreign_key" => {
                                opened = OpenedOffset::ForeignKey;
                            }
                            "foreign_table_name" => {
                                opened = OpenedOffset::ForeignTableName;
                            }
                            "foreign_key_type" => {
                                opened = OpenedOffset::ForeignKeyType;
                            }
                            "foreign_primary_key" => {
                                opened = OpenedOffset::ForeignPrimaryKey;
                            }
                            "foreign_reference_key" => {
                                opened = OpenedOffset::ForeignReferenceKey;
                            }
                            "auto" => {
                                opened = OpenedOffset::Auto;
                            }
                            "version" => {
                                opened = OpenedOffset::Version;
                            }
                            "aggregator" => {
                                opened = OpenedOffset::Aggregator;
                            }
                            _ => match opened {
                                OpenedOffset::Aggregator => match id.as_str() {
                                    "count" => {
                                        field_attrs.insert(
                                            SqlAttributeKey::Aggregator,
                                            SqlAttribute::Aggregator(Aggregator::Count),
                                        );
                                    }
                                    "sum" => {
                                        field_attrs.insert(
                                            SqlAttributeKey::Aggregator,
                                            SqlAttribute::Aggregator(Aggregator::Sum(
                                                "".to_string(),
                                            )),
                                        );
                                    }
                                    "avg" => {
                                        field_attrs.insert(
                                            SqlAttributeKey::Aggregator,
                                            SqlAttribute::Aggregator(Aggregator::Avg(
                                                "".to_string(),
                                            )),
                                        );
                                    }
                                    "max" => {
                                        field_attrs.insert(
                                            SqlAttributeKey::Aggregator,
                                            SqlAttribute::Aggregator(Aggregator::Max(
                                                "".to_string(),
                                            )),
                                        );
                                    }
                                    "min" => {
                                        field_attrs.insert(
                                            SqlAttributeKey::Aggregator,
                                            SqlAttribute::Aggregator(Aggregator::Min(
                                                "".to_string(),
                                            )),
                                        );
                                    }
                                    "exist" => {
                                        field_attrs.insert(
                                            SqlAttributeKey::Aggregator,
                                            SqlAttribute::Aggregator(Aggregator::Exist),
                                        );
                                    }
                                    _ => {
                                        tracing::error!("invalid aggregator: {id}");
                                    }
                                },
                                OpenedOffset::Version => {
                                    field_attrs.insert(
                                        SqlAttributeKey::Version,
                                        SqlAttribute::Version(id),
                                    );
                                }
                                OpenedOffset::Type => {
                                    field_attrs.insert(
                                        SqlAttributeKey::SqlType,
                                        SqlAttribute::SqlType(id),
                                    );
                                }
                                OpenedOffset::ManyToMany => {
                                    field_attrs.insert(
                                        SqlAttributeKey::Relation,
                                        SqlAttribute::ManyToMany {
                                            table_name: id,
                                            foreign_table_name: "".to_string(),
                                            foreign_key: "id".to_string(),
                                            foreign_key_type: "BIGINT".to_string(),
                                            foreign_primary_key: "".to_string(),
                                            foreign_reference_key: "".to_string(),
                                        },
                                    );
                                    tracing::trace!("many_to_many: {name}");
                                }
                                OpenedOffset::ManyToOne => {
                                    field_attrs.insert(
                                        SqlAttributeKey::Relation,
                                        SqlAttribute::ManyToOne {
                                            table_name: id,
                                            foreign_key: "id".to_string(),
                                            foreign_key_type: "BIGINT".to_string(),
                                        },
                                    );
                                    tracing::trace!("many_to_one: {name}");
                                }
                                OpenedOffset::OneToMany => {
                                    field_attrs.insert(
                                        SqlAttributeKey::Relation,
                                        SqlAttribute::OneToMany {
                                            table_name: id,
                                            foreign_key: "id".to_string(),
                                        },
                                    );
                                    tracing::trace!("one_to_many: {name}");
                                }
                                OpenedOffset::ForeignKey => {
                                    field_attrs.get_mut(&SqlAttributeKey::Relation).map(|attr| {
                                        if let SqlAttribute::ManyToOne {
                                            ref mut foreign_key,
                                            ..
                                        } = attr
                                        {
                                            *foreign_key = id
                                        } else if let SqlAttribute::ManyToMany {
                                            ref mut foreign_key,
                                            ..
                                        } = attr
                                        {
                                            *foreign_key = id
                                        } else if let SqlAttribute::OneToMany {
                                            ref mut foreign_key,
                                            ..
                                        } = attr
                                        {
                                            *foreign_key = id
                                        }
                                    });
                                }
                                OpenedOffset::ForeignTableName => {
                                    field_attrs.get_mut(&SqlAttributeKey::Relation).map(|attr| {
                                        if let SqlAttribute::ManyToMany {
                                            ref mut foreign_table_name,
                                            ..
                                        } = attr
                                        {
                                            *foreign_table_name = id
                                        }
                                    });
                                }
                                OpenedOffset::ForeignKeyType => {
                                    field_attrs.get_mut(&SqlAttributeKey::Relation).map(|attr| {
                                        if let SqlAttribute::ManyToOne {
                                            ref mut foreign_key_type,
                                            ..
                                        } = attr
                                        {
                                            *foreign_key_type = id
                                        } else if let SqlAttribute::ManyToMany {
                                            ref mut foreign_key_type,
                                            ..
                                        } = attr
                                        {
                                            *foreign_key_type = id
                                        }
                                    });
                                }
                                OpenedOffset::ForeignPrimaryKey => {
                                    field_attrs.get_mut(&SqlAttributeKey::Relation).map(|attr| {
                                        if let SqlAttribute::ManyToMany {
                                            ref mut foreign_primary_key,
                                            ..
                                        } = attr
                                        {
                                            *foreign_primary_key = id
                                        }
                                    });
                                }
                                OpenedOffset::ForeignReferenceKey => {
                                    field_attrs.get_mut(&SqlAttributeKey::Relation).map(|attr| {
                                        if let SqlAttribute::ManyToMany {
                                            ref mut foreign_reference_key,
                                            ..
                                        } = attr
                                        {
                                            *foreign_reference_key = id
                                        }
                                    });
                                }
                                OpenedOffset::Auto => {
                                    let auto = match id.as_str() {
                                        "insert" => AutoOperation::Insert,
                                        "update" => AutoOperation::Update,
                                        _ => {
                                            tracing::error!("invalid auto operation: {id}");
                                            continue;
                                        }
                                    };

                                    field_attrs
                                        .entry(SqlAttributeKey::Auto)
                                        .or_insert_with(|| SqlAttribute::Auto(vec![]));

                                    if let Some(SqlAttribute::Auto(ref mut operations)) =
                                        field_attrs.get_mut(&SqlAttributeKey::Auto)
                                    {
                                        operations.push(auto);
                                    }
                                }
                                OpenedOffset::None => {}
                            },
                        }
                    } else if let proc_macro2::TokenTree::Group(group) = nested {
                        match opened {
                            OpenedOffset::Auto => {
                                for nested in group.stream() {
                                    if let proc_macro2::TokenTree::Ident(iden) = nested {
                                        let id = iden.to_string();

                                        field_attrs
                                            .entry(SqlAttributeKey::Auto)
                                            .or_insert_with(|| SqlAttribute::Auto(vec![]));

                                        if let Some(SqlAttribute::Auto(ref mut operations)) =
                                            field_attrs.get_mut(&SqlAttributeKey::Auto)
                                        {
                                            operations.push(match id.as_str() {
                                                "insert" => AutoOperation::Insert,
                                                "update" => AutoOperation::Update,
                                                _ => {
                                                    tracing::error!("invalid auto operation: {id}");
                                                    continue;
                                                }
                                            });
                                        }
                                    }
                                }

                                opened = OpenedOffset::None;
                            }
                            OpenedOffset::Aggregator => {
                                for nested in group.stream() {
                                    if let proc_macro2::TokenTree::Ident(iden) = nested {
                                        let id = iden.to_string();

                                        field_attrs.get_mut(&SqlAttributeKey::Aggregator).map(
                                            |attr| {
                                                if let SqlAttribute::Aggregator(aggregator) = attr {
                                                    match aggregator {
                                                        Aggregator::Sum(ref mut field) => {
                                                            *field = id;
                                                        }
                                                        Aggregator::Avg(ref mut field) => {
                                                            *field = id;
                                                        }
                                                        Aggregator::Max(ref mut field) => {
                                                            *field = id;
                                                        }
                                                        Aggregator::Min(ref mut field) => {
                                                            *field = id;
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                            },
                                        );
                                    }
                                }

                                opened = OpenedOffset::None;
                            }
                            _ => {}
                        }
                    } else if let proc_macro2::TokenTree::Punct(punct) = nested {
                        if punct.to_string().as_str() == "," {
                            opened = OpenedOffset::None;
                        }
                    }
                }
            }
        }
    }

    SqlAttributes { attrs: field_attrs }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum SqlModelKey {
    Table,
    Rename,
}

pub enum SqlModel {
    Table(String),
    Rename(Case),
}

pub fn parse_sql_model(attr: TokenStream) -> HashMap<SqlModelKey, SqlModel> {
    let attr_args = attr.to_string();
    let mut models = HashMap::new();

    for arg in attr_args.split(',') {
        let parts: Vec<&str> = arg.split('=').collect();

        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim().trim_matches('"');

            match key {
                "table" => {
                    models.insert(SqlModelKey::Table, SqlModel::Table(value.to_string()));
                }
                "rename" => {
                    models.insert(
                        SqlModelKey::Rename,
                        match value {
                            "upcase" => SqlModel::Rename(Case::UpperSnake),
                            "camel" => SqlModel::Rename(Case::Camel),
                            "pascal" | "uppercamel" => SqlModel::Rename(Case::Pascal),
                            "snake" | "underscore" => SqlModel::Rename(Case::Snake),
                            "kebab" => SqlModel::Rename(Case::Kebab),
                            _ => {
                                panic!("invalid rename value {}", value);
                            }
                        },
                    );
                }
                _ => {}
            }
        }
    }

    models
}
