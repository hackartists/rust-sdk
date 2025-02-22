use std::process::Stdio;
use std::{collections::HashMap, str::FromStr};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;

use tokio::process::Command;

pub fn generate_dioxus_component(
    name: &str,
    body: &str,
    value: HashMap<String, String>,
) -> TokenStream {
    let func_name = format!("{}", name).to_case(Case::Pascal);
    let func_name = quote::format_ident!("{}", func_name);

    let params = value.into_iter().map(|(name, value)| {
        let param_name = quote::format_ident!("{}", name);
        let v = format!("#[props(default = \"{}\".to_string())]", value);
        let default_value = TokenStream::from_str(&v).expect("msg");
        quote! {
            #default_value
            #param_name: String
        }
    });
    let body_token = TokenStream::from_str(body).expect("FAILED");

    let code = quote! {
        #[component]
        pub fn #func_name(
            #(#params),*
        ) -> Element {
            rsx! {
                #body_token
            }
        }
    };
    code
}

pub async fn execute_dioxus_formatter(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _ = Command::new("dx")
        .arg("fmt")
        .arg("--all-code")
        .arg("-f")
        .arg(file_name)
        .stdout(Stdio::null())
        .spawn()?;
    Ok(())
}
