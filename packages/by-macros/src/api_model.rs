use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Field, Fields, Meta};

#[derive(Debug)]
enum ActionType {
    Summary,
    Queryable,
    Action(String),
    ActionById(String),
    Related(String),
}

/// Parse the attribute string and return the action type
/// The attribute should be in the form of #[api_model(action::action_name, action_by_id::action_name)]
///

fn parse_action_attr(attr: &Attribute) -> Vec<ActionType> {
    let mut types = Vec::new();

    if let Meta::List(meta_list) = attr.meta.clone() {
        if meta_list.path.is_ident("api_model") {
            let mut selected_action = ActionType::Summary;
            let mut selected_at = 0;
            let mut i = 0;

            for nested in meta_list.tokens.clone() {
                if let proc_macro2::TokenTree::Ident(iden) = nested {
                    let id = iden.to_string();
                    match id.as_str() {
                        "summary" => {
                            types.push(ActionType::Summary);
                        }
                        "queryable" => {
                            types.push(ActionType::Queryable);
                        }
                        "action" => {
                            selected_at = i;
                            selected_action = ActionType::Action("".to_string());
                        }
                        "action_by_id" => {
                            selected_at = i;
                            selected_action = ActionType::ActionById("".to_string());
                        }
                        "related" => {
                            selected_at = i;
                            selected_action = ActionType::Related("".to_string());
                        }
                        _ => {
                            if selected_at == (i - 2) {
                                match &selected_action {
                                    ActionType::Action(_) => {
                                        types.push(ActionType::Action(id.to_string()));
                                    }
                                    ActionType::ActionById(_) => {
                                        types.push(ActionType::ActionById(id.to_string()));
                                    }
                                    ActionType::Related(_) => {
                                        types.push(ActionType::Related(id.to_string()));
                                    }
                                    _ => {}
                                }
                            } else {
                                panic!("Unexpected attribute key: {}", id);
                            }
                        }
                    }
                }

                i += 1;
            }
        }
    }

    types
}

#[derive(Debug)]
enum ActionField {
    Fields(Vec<Field>),
    Related(String),
}

pub fn api_model_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_cloned = item.clone();
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;
    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => panic!("api_mode can only be applied to structs"),
    };

    let attr_args = attr.to_string();
    let mut iter_type = "CommonQueryResponse".to_string();
    let mut base_endpoint = String::new();

    for arg in attr_args.split(',') {
        let parts: Vec<&str> = arg.split('=').collect();
        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim().trim_matches('"');
            match key {
                "base" => base_endpoint = value.to_string(),
                "iter_type" => iter_type = value.to_string(),
                _ => panic!("Unexpected attribute key: {}", key),
            }
        }
    }

    let mut summary_fields = Vec::new();
    let mut queryable_fields = Vec::new();
    let mut action_names = HashMap::<String, ActionField>::new();
    let mut action_by_id_names = HashMap::<String, ActionField>::new();

    if let Fields::Named(named_fields) = fields {
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
                        ActionType::Action(action_name) => {
                            actions.push(ActionType::Action(action_name));
                        }
                        ActionType::ActionById(action_name) => {
                            actions.push(ActionType::ActionById(action_name));
                        }
                        ActionType::Related(st) => {
                            related = Some(st);
                        }
                    }
                }

                for action in actions {
                    match (related.clone(), action) {
                        (Some(st), ActionType::Action(action_name)) => {
                            action_names
                                .entry(action_name)
                                .or_insert_with(|| ActionField::Related(st));
                        }
                        (Some(st), ActionType::ActionById(action_name)) => {
                            action_by_id_names
                                .entry(action_name)
                                .or_insert_with(|| ActionField::Related(st));
                        }
                        (None, ActionType::Action(action_name)) => {
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
                        (None, ActionType::ActionById(action_name)) => {
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
                        _ => {}
                    }
                }
            }
        }
    }

    let summary_struct = generate_summary_struct(&struct_name, &summary_fields);
    let query_struct = generate_query_struct(&struct_name, &queryable_fields);
    let action_struct = generate_action_struct(&struct_name, &action_names, "Action");
    let action_by_id_struct =
        generate_action_struct(&struct_name, &action_by_id_names, "ActionById");

    let client_impl = generate_client_impl(
        struct_name,
        &base_endpoint,
        &iter_type,
        action_names.len() > 0,
        action_by_id_names.len() > 0,
    );
    let input = parse_macro_input!(input_cloned as syn::ItemStruct);
    let stripped_input = strip_struct_attributes(&input);

    let output = quote! {
        #stripped_input
        #action_struct
        #action_by_id_struct
        #summary_struct
        #query_struct
        #client_impl
    };

    output.into()
}

fn generate_action_struct(
    struct_name: &syn::Ident,
    actions: &HashMap<String, ActionField>,
    action_type: &str,
) -> proc_macro2::TokenStream {
    let action_name = syn::Ident::new(
        &format!("{}{}Request", struct_name, action_type),
        struct_name.span(),
    );
    let mut action_fields = vec![];
    let mut action_requests = vec![];

    for (k, v) in actions.iter() {
        let act = syn::Ident::new(&k.to_case(Case::Pascal), struct_name.span());
        let request_struct_name = match v {
            ActionField::Related(st) => syn::Ident::new(&st, struct_name.span()),
            _ => syn::Ident::new(
                &format!("{}{}Request", struct_name, act),
                struct_name.span(),
            ),
        };
        action_fields.push(quote! {
            #act(#request_struct_name),
        });

        if let ActionField::Fields(v) = v {
            let fields = v.iter().map(|field| {
                let field_name = &field.ident;
                let field_type = &field.ty;
                quote! { pub #field_name: #field_type, }
            });

            action_requests.push(quote! {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialEq)]
            #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
            pub struct #request_struct_name {
                #(#fields)*
            }
        });
        }
    }

    quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
        #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
        pub enum #action_name {
            #(#action_fields)*
        }

        #(#action_requests)*
    }
}

fn generate_summary_struct(
    struct_name: &syn::Ident,
    summary_fields: &[syn::Field],
) -> proc_macro2::TokenStream {
    let summary_name = syn::Ident::new(&format!("{}Summary", struct_name), struct_name.span());
    let fields = summary_fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! { pub #field_name: #field_type, }
    });

    quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialEq)]
        #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
        pub struct #summary_name {
            #(#fields)*
        }
    }
}

fn generate_query_struct(
    struct_name: &syn::Ident,

    queryable_fields: &[syn::Field],
) -> proc_macro2::TokenStream {
    let query_fields = queryable_fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! { pub #field_name: Option<#field_type>, }
    });
    let query_name = syn::Ident::new(&format!("{}Query", struct_name), struct_name.span());

    quote! {
        #[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, by_macros::QueryDisplay)]
        #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
        pub struct #query_name {
            pub size: usize,
            pub bookmark: Option<String>,
            #(#query_fields)*
        }
    }
}

fn generate_client_impl(
    struct_name: &syn::Ident,
    base_endpoint: &str,
    iter_type: &str,
    enable_action: bool,
    enable_action_by_id: bool,
) -> proc_macro2::TokenStream {
    let client_name = syn::Ident::new(&format!("{}Client", struct_name), struct_name.span());
    let query_name = syn::Ident::new(&format!("{}Query", struct_name), struct_name.span());
    let summary_name = syn::Ident::new(&format!("{}Summary", struct_name), struct_name.span());

    // Convert endpoints to string literals
    let base_endpoint_lit = syn::LitStr::new(base_endpoint, struct_name.span());

    // Dynamically generate iter_type with Summary
    let iter_type_with_summary = format!("{}<{}>", iter_type, summary_name);
    let iter_type_tokens: proc_macro2::TokenStream = iter_type_with_summary.parse().unwrap();

    let action = if enable_action {
        let action_name =
            syn::Ident::new(&format!("{}ActionRequest", struct_name), struct_name.span());
        quote! {
            pub async fn act(&self, params: #action_name) -> crate::Result<#iter_type_tokens> {
                let endpoint = format!("{}{}/action", self.endpoint, #base_endpoint_lit);
                rest_api::post(&endpoint, params).await
            }
        }
    } else {
        quote! {}
    };

    let action_by_id = if enable_action_by_id {
        let action_name = syn::Ident::new(
            &format!("{}ActionByIdRequest", struct_name),
            struct_name.span(),
        );
        quote! {
            pub async fn act_by_id(&self, id: &str, params: #action_name) -> crate::Result<#iter_type_tokens> {
                let endpoint = format!("{}{}/{}/action", self.endpoint, #base_endpoint_lit, id);
                rest_api::post(&endpoint, params).await
            }
        }
    } else {
        quote! {}
    };

    quote! {
        impl #struct_name {
            pub fn get_client(endpoint: &str) -> #client_name {
                #client_name { endpoint: endpoint.to_string() }
            }
        }

        #[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
        pub struct #client_name {
            pub endpoint: String,
        }

        impl #client_name {
            pub async fn query(
                &self,
                params: #query_name,
            ) -> crate::Result<#iter_type_tokens> {
                let endpoint = format!("{}{}", self.endpoint, #base_endpoint_lit);
                let query = format!("{}?{}", endpoint, params);
                rest_api::get(&query).await
            }

            pub async fn get(&self, id: &str) -> crate::Result<#struct_name> {
                let endpoint = format!("{}{}/{}", self.endpoint, #base_endpoint_lit, id);
                rest_api::get(&endpoint).await
            }

            #action
            #action_by_id
        }
    }
}

fn strip_struct_attributes(input: &syn::ItemStruct) -> syn::ItemStruct {
    let mut stripped_struct = input.clone();
    stripped_struct.fields = strip_api_model_attributes(&input.fields);
    stripped_struct
}

fn strip_api_model_attributes(fields: &syn::Fields) -> syn::Fields {
    match fields {
        syn::Fields::Named(named_fields) => {
            let stripped_fields = named_fields
                .named
                .iter()
                .map(|field| {
                    let mut field = field.clone();
                    field.attrs = field
                        .attrs
                        .iter()
                        .filter(|attr| !attr.path().is_ident("api_model"))
                        .cloned()
                        .collect();
                    field
                })
                .collect();
            syn::Fields::Named(syn::FieldsNamed {
                brace_token: named_fields.brace_token,
                named: stripped_fields,
            })
        }
        syn::Fields::Unnamed(_) | syn::Fields::Unit => fields.clone(),
    }
}
