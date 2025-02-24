#![allow(non_snake_case)]
use dioxus::html::HasFileData;
use dioxus::prelude::*;

#[component]
pub fn DropZone(
    oncomplete: Option<EventHandler<Vec<String>>>,
    onclick: Option<EventHandler<Event<MouseData>>>,
    onupload: EventHandler<Vec<u8>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    onchange: Option<EventHandler<bool>>,
    #[props(default = "image/*".to_string())] accept: String,
    #[props(default = false)] multiple: bool,
) -> Element {
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
                        if let Some(contents) = file_engine.read_file(filename).await {
                            onupload(contents);
                        }
                    }
                }
                if let Some(onchange) = onchange {
                    onchange(false);
                }
            },
            onclick: move |_| {
                use wasm_bindgen::JsCast;
                let input = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("drop-zone-file-selector")
                    .unwrap();
                input.dyn_ref::<web_sys::HtmlInputElement>().unwrap().click();
            },
            ..attributes,
            {children}
        }
        input {
            id: "drop-zone-file-selector",
            class: "hidden",
            accept,
            multiple,
            r#type: "file",

            onchange: move |ev| async move {
                if let Some(file_engine) = ev.files() {
                    tracing::debug!("got file_engine");
                    let filenames = file_engine.files();
                    for i in 0..filenames.len() {
                        let filename = &filenames[i];
                        if let Some(contents) = file_engine.read_file(filename).await {
                            onupload(contents);
                        }
                    }
                }
            },
        }
    }
}
