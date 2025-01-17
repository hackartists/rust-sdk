use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Meta};

pub fn api_model_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_cloned = item.clone();
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;
    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => panic!("api_mode can only be applied to structs"),
    };

    let attr_args = attr.to_string();
    let mut get_endpoint = String::new();
    let mut list_endpoint = String::new();
    let mut iter_type = "CommonQueryResponse".to_string();

    for arg in attr_args.split(',') {
        let parts: Vec<&str> = arg.split('=').collect();
        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim().trim_matches('"');
            match key {
                "get" => get_endpoint = value.to_string(),
                "list" => list_endpoint = value.to_string(),
                "iter_type" => iter_type = value.to_string(),
                _ => panic!("Unexpected attribute key: {}", key),
            }
        }
    }

    let mut summary_fields = Vec::new();
    let mut queryable_fields = Vec::new();

    if let Fields::Named(named_fields) = fields {
        for field in &named_fields.named {
            for attr in &field.attrs {
                if is_api_model_attr(attr, "summary") {
                    summary_fields.push(field.clone());
                }
                if is_api_model_attr(attr, "queryable") {
                    queryable_fields.push(field.clone());
                }
            }
        }
    }

    let summary_struct = generate_summary_struct(&struct_name, &summary_fields);

    let query_struct = generate_query_struct(&struct_name, &queryable_fields);

    let client_impl = generate_client_impl(struct_name, &get_endpoint, &list_endpoint, &iter_type);
    let input = parse_macro_input!(input_cloned as syn::ItemStruct);
    let stripped_input = strip_struct_attributes(&input);

    let output = quote! {
        #stripped_input
        #summary_struct
        #query_struct
        #client_impl
    };

    output.into()
}

fn is_api_model_attr(attr: &Attribute, key: &str) -> bool {
    if let Meta::List(meta_list) = attr.meta.clone() {
        if meta_list.path.is_ident("api_model") {
            for nested in meta_list.tokens.clone() {
                if let proc_macro2::TokenTree::Ident(iden) = nested {
                    if iden.to_string().as_str() == key {
                        return true;
                    }
                }
            }
        }
    }
    false
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
        #[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
        pub struct #query_name {
            pub size: usize,
            pub bookmark: Option<String>,
            #(#query_fields)*
        }
    }
}

fn generate_client_impl(
    struct_name: &syn::Ident,
    get_endpoint: &str,
    list_endpoint: &str,
    iter_type: &str,
) -> proc_macro2::TokenStream {
    let client_name = syn::Ident::new(&format!("{}Client", struct_name), struct_name.span());
    let query_name = syn::Ident::new(&format!("{}Query", struct_name), struct_name.span());
    let summary_name = syn::Ident::new(&format!("{}Summary", struct_name), struct_name.span());

    // Convert endpoints to string literals
    let get_endpoint_lit = syn::LitStr::new(get_endpoint, struct_name.span());
    let list_endpoint_lit = syn::LitStr::new(list_endpoint, struct_name.span());

    // Dynamically generate iter_type with Summary
    let iter_type_with_summary = format!("{}<{}>", iter_type, summary_name);
    let iter_type_tokens: proc_macro2::TokenStream = iter_type_with_summary.parse().unwrap();

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
                let endpoint = format!("{}{}", self.endpoint, #list_endpoint_lit);
                let query = format!("{}?{}", endpoint, params);
                rest_api::get(&query).await
            }

            pub async fn get(&self, id: &str) -> crate::Result<#struct_name> {
                let endpoint = format!("{}{}{}", self.endpoint, #get_endpoint_lit, id);
                rest_api::get(&endpoint).await
            }
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
