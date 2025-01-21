use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Field, Fields, LitStr, Meta};
use tracing::field;

#[derive(Debug)]
enum ActionType {
    Summary,
    Queryable,
    Action(Vec<String>),
    ActionById(Vec<String>),
    Related(String),
    QueryActions(Vec<String>),
    ReadActions(Vec<String>),
}

#[derive(Debug)]
enum ActionField {
    Fields(Vec<Field>),
    Related(String),
}

pub fn sql_model_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let repo_name = syn::Ident::new(&format!("{name}Repository"), name.span());
    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("api_mode can only be applied to structs"),
    };

    let attrs = parse_sql_model(attr);
    let table_name = if let Some(SqlModel::Table(ref table_name)) = attrs.get(&SqlModelKey::Table) {
        table_name.to_string()
    } else {
        format!("{}s", name.to_string().to_case(Case::Lower))
    };

    let rename = if let SqlModel::Rename(rename) = attrs
        .get(&SqlModelKey::Rename)
        .unwrap_or(&SqlModel::Rename(Case::Upper))
    {
        rename
    } else {
        todo!()
    };

    let create_table_function = create_table_tokens(&table_name, rename.clone(), &fields);
    let drop_table_query = syn::LitStr::new(
        &format!("DROP TABLE IF EXISTS {};", table_name),
        proc_macro2::Span::call_site(),
    );

    let output = quote! {
        impl #name {
            pub fn get_repository<'a>(pool: &'a sqlx::Pool<sqlx::Postgres>) -> #repo_name<'a> {
                #repo_name::new(pool)
            }
        }

        pub struct #repo_name<'a> {
            pool: &'a sqlx::Pool<sqlx::Postgres>,
        }

        impl<'a> #repo_name<'a> {
            pub fn new(pool: &'a sqlx::Pool<sqlx::Postgres>) -> Self {
                Self { pool }
            }

            #create_table_function
            pub async fn drop_table(&self) -> Result<(), sqlx::Error> {
                sqlx::query(#drop_table_query)
                    .execute(self.pool)
                    .await?;

                Ok(())
            }
        }
    };

    tracing::debug!("Generated code: {}", output.to_string());

    output.into()
}

#[derive(Eq, PartialEq, Hash)]
enum SqlAttributeKey {
    PrimaryKey,
    SqlType,
}

enum SqlAttribute {
    PrimaryKey,
    SqlType(String),
}

enum OpenedOffset {
    None,
    Type,
}

fn parse_field_attr(field: &Field) -> HashMap<SqlAttributeKey, SqlAttribute> {
    let mut field_attrs = HashMap::new();

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
                            "type" => {
                                opened = OpenedOffset::Type;
                            }
                            _ => match opened {
                                OpenedOffset::Type => {
                                    field_attrs.insert(
                                        SqlAttributeKey::SqlType,
                                        SqlAttribute::SqlType(id),
                                    );
                                }
                                OpenedOffset::None => {}
                            },
                        }
                    } else if let proc_macro2::TokenTree::Group(_group) = nested {
                    } else if let proc_macro2::TokenTree::Punct(punct) = nested {
                        if punct.to_string().as_str() == "," {
                            opened = OpenedOffset::None;
                        }
                    }
                }
            }
        }
    }

    field_attrs
}

fn create_table_tokens(table_name: &str, case: Case, fields: &Fields) -> proc_macro2::TokenStream {
    let mut output = vec![];
    let mut create_query_fields = vec![];

    let fields = if let Fields::Named(named_fields) = fields {
        named_fields.named.clone()
    } else {
        return quote! {};
    };

    for field in fields {
        let field = field.clone();
        let attrs = parse_field_attr(&field);

        let field_name = field.ident.unwrap();
        let field_type = field.ty;

        match field_type {
            syn::Type::Path(ref type_path) => {
                let type_ident = type_path.path.segments.last().unwrap().ident.to_string();
                let type_str = if let Some(SqlAttribute::SqlType(type_str)) =
                    attrs.get(&SqlAttributeKey::SqlType)
                {
                    type_str.to_string()
                } else {
                    match type_ident.as_str() {
                        "u64" | "i64" => "BIGINT NOT NULL".to_string(),
                        "String" => "TEXT NOT NULL".to_string(),
                        "bool" => "BOOLEAN NOT NULL".to_string(),
                        "i32" => "INTEGER NOT NULL".to_string(),
                        "f64" => "DOUBLE PRECISION NOT NULL".to_string(),
                        "Option<u64>" | "Option<i64>" => "BIGINT".to_string(),
                        "Option<String>" => "TEXT".to_string(),
                        "Option<bool>" => "BOOLEAN".to_string(),
                        "Option<i32>" => "INTEGER".to_string(),
                        "Option<f64>" => "DOUBLE PRECISION".to_string(),

                        _ => "".to_string(),
                    }
                };

                if !type_str.is_empty() {
                    create_query_fields.push(format!(
                        "{} {}",
                        field_name.to_string().to_case(case),
                        type_str
                    ))
                }
            }
            _ => {
                tracing::debug!("field type: {:?}", field_type);
            }
        }

        output.push(quote! {
            #field_name: #field_type
        });
    }

    let create_query_ouput = syn::LitStr::new(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} ({});",
            table_name,
            create_query_fields.join(",")
        ),
        proc_macro2::Span::call_site(),
    );

    quote! {
        pub async fn create_table(&self) -> Result<(), sqlx::Error> {
            sqlx::query(#create_query_ouput)
            .execute(self.pool)
            .await?;

            Ok(())
        }

    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum SqlModelKey {
    Table,
    Rename,
}

enum SqlModel {
    Table(String),
    Rename(Case),
}

fn parse_sql_model(attr: TokenStream) -> HashMap<SqlModelKey, SqlModel> {
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

enum SqlField {
    Type(LitStr),
}

fn parse_field_attributes(fields: Fields) -> Vec<SqlField> {
    let f = vec![];

    f
}
