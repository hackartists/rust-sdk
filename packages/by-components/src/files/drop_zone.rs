#![allow(non_snake_case)]
use dioxus::html::HasFileData;
use dioxus::prelude::*;

pub type FileExtension = String;

#[component]
pub fn DropZone(
    oncomplete: Option<EventHandler<Vec<String>>>,
    onclick: Option<EventHandler<Event<MouseData>>>,
    onupload: EventHandler<(Vec<u8>, FileExtension)>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    onchange: Option<EventHandler<bool>>,
    #[props(default = "image/*".to_string())] accept: String,
    #[props(default = false)] multiple: bool,
) -> Element {
    let mut file_input: Signal<Option<web_sys::Element>> = use_signal(|| None);
    rsx! {
        button {
            ondragover: move |e| {
                e.prevent_default();
                e.stop_propagation();
                tracing::debug!("files in drop zone");
                if let Some(onchange) = onchange {
                    onchange(true);
                }
            },
            ondragleave: move |e| {
                e.prevent_default();
                e.stop_propagation();
                tracing::debug!("leave drop zone");
                if let Some(onchange) = onchange {
                    onchange(false);
                }
            },
            ondrop: move |ev| async move {
                tracing::debug!("drop files");
                ev.prevent_default();
                ev.stop_propagation();
                if let Some(file_engine) = ev.files() {
                    tracing::debug!("got file_engine");
                    let filenames = file_engine.files();
                    for i in 0..filenames.len() {
                        let filename = &filenames[i];
                        let ext = filename.split('.').last().unwrap_or_default();
                        if let Some(contents) = file_engine.read_file(filename).await {
                            onupload((contents, ext.to_string()));
                        }
                    }
                }
                if let Some(onchange) = onchange {
                    onchange(false);
                }
            },
            onclick: move |_| {
                use wasm_bindgen::JsCast;
                if let Some(file_input) = file_input() {
                    file_input.dyn_ref::<web_sys::HtmlInputElement>().unwrap().click();
                }
            },
            ..attributes,
            {children}
        }
        input {
            class: "hidden",
            onmounted: move |el| {
                use dioxus::web::WebEventExt;
                let w = el.as_web_event();
                file_input.set(Some(w));
            },
            accept,
            multiple,
            r#type: "file",

            onchange: move |ev| async move {
                if let Some(file_engine) = ev.files() {
                    tracing::debug!("got file_engine");
                    let filenames = file_engine.files();
                    for i in 0..filenames.len() {
                        let filename = &filenames[i];
                        let ext = filename.split('.').last().unwrap_or_default();
                        if let Some(contents) = file_engine.read_file(filename).await {
                            onupload((contents, ext.to_string()));
                        }
                    }
                }
            },
        }
    }
}
