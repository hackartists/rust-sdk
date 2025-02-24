use std::str::FromStr;
use std::sync::Arc;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use rsx::{execute_dioxus_formatter, generate_dioxus_component};
mod figma;
mod rsx;

use tokio::fs::{self, File};
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();

    let dir_path = match option_env!("OUTPUT_DIR") {
        Some(dir) => dir.to_string(),
        None => {
            let current_dir = std::env::current_dir().unwrap();
            format!("{}", current_dir.join("icons").to_str().unwrap())
        }
    };

    let dir = std::path::Path::new(&dir_path);

    if !dir.exists() {
        if let Err(e) = fs::create_dir_all(dir).await {
            tracing::error!("Failed to create directory: {}", e);
        }
    }
    tracing::debug!("Output Directory: {:?}", dir_path);
    let client = Arc::new(reqwest::Client::new());
    let icon_nodes = figma::parse_figma_files(
        client.clone(),
        env!("FIGMA_FILE_KEY"),
        env!("FIGMA_TOKEN"),
        env!("FIGMA_ROOT_NODE_ID"),
    )
    .await?;

    let mut categories: Vec<String> = vec![];
    let mut components: Vec<String> = vec![];

    for node in icon_nodes {
        let (category, icons) = node;
        let file_name = format!("{}.rs", category.to_case(Case::Snake));
        categories.push(category.to_case(Case::Snake));
        let file_path = format!("{}/{}", dir_path, file_name);
        let file = File::create(&file_path).await?;
        let writer = Arc::new(Mutex::new(BufWriter::new(file)));
        let code = quote! {
            use dioxus::prelude::*;
        };
        writer
            .lock()
            .await
            .write_all(code.to_string().as_bytes())
            .await?;
        let tasks: Vec<_> = icons
            .into_iter()
            .map(|(icon_name, url)| {
                components.push(icon_name.clone());
                let writer = Arc::clone(&writer);
                let client = Arc::clone(&client);
                let category = category.clone();
                let icon_name = icon_name.clone();
                tokio::spawn(async move {
                    tracing::debug!("Convert {}/{}", category, icon_name);
                    let (body, default_value) = figma::fetch_and_parse_svg(client.clone(), &url)
                        .await
                        .expect("Failed to fetch svg");
                    let icon_code =
                        generate_dioxus_component(&icon_name, &body, default_value).to_string();
                    let mut writer = writer.lock().await;
                    writer
                        .write_all(icon_code.as_bytes())
                        .await
                        .expect("Failed to write");
                    writer.flush().await.expect("Failed to flush");
                })
            })
            .collect();

        for task in tasks {
            task.await?;
        }
        execute_dioxus_formatter(&file_path).await?;
    }

    let mod_file_path = format!("{}/mod.rs", dir_path);
    let mods = categories.iter().map(|category| {
        let v = TokenStream::from_str(&category).unwrap();
        quote! {
            pub mod #v;
        }
    });
    let prelude = categories.iter().map(|category| {
        let v = TokenStream::from_str(&category).unwrap();
        quote! {
            pub use super::#v::*;
        }
    });
    let code = quote! {
        #(#mods)*
        pub mod prelude {
            #(#prelude)*
        }
    };
    let mut file = File::create(&mod_file_path).await?;
    file.write_all(code.to_string().as_bytes()).await?;
    execute_dioxus_formatter(&mod_file_path).await?;
    Ok(())
}
