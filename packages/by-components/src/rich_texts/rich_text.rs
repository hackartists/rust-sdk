use dioxus::prelude::*;
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::js_sys::eval;
use web_sys::{Event, HtmlElement, window};

#[component]
pub fn RichText(
    #[props(default = "rich-text".to_string())] id: String,
    content: String,
    onchange: EventHandler<String>,
) -> Element {
    let mut closure_ref = use_signal(|| None as Option<Closure<dyn FnMut(web_sys::Event)>>);

    use_effect({
        let id = id.clone();
        let onchange = onchange.clone();

        move || {
            let event_name = format!("content-updated-{}", id);

            let init_js = format!(
                r#"
                (function tryInit() {{
                    let editor = document.getElementById("{id}");
                    if (editor && window.Quill && !editor.__quill) {{
                        editor.__quill = new Quill(editor, {{ theme: "snow" }});

                        let toolbar = editor.previousElementSibling;
                        if (toolbar) {{
                            toolbar.style.width = "100%";
                            toolbar.style.maxWidth = "none";
                            toolbar.style.boxSizing = "border-box";
                        }}

                        editor.__quill.on('text-change', function() {{
                            document.dispatchEvent(new CustomEvent("{event_name}"));
                        }});
                    }} else {{
                        setTimeout(tryInit, 200);
                    }}
                }})();
                "#
            );
            let _ = eval(&init_js);

            let id_cloned = id.clone();
            let onchange_cloned = onchange.clone();
            let closure = Closure::wrap(Box::new(move |_event: Event| {
                if let Some(editor) = window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id(&id_cloned)
                {
                    if let Ok(Some(ql_editor)) = editor
                        .dyn_ref::<HtmlElement>()
                        .unwrap()
                        .query_selector(".ql-editor")
                    {
                        let html = ql_editor.inner_html();
                        onchange_cloned.call(html);
                    }
                }
            }) as Box<dyn FnMut(_)>);

            window()
                .unwrap()
                .document()
                .unwrap()
                .add_event_listener_with_callback(&event_name, closure.as_ref().unchecked_ref())
                .unwrap();

            closure_ref.set(Some(closure));

            let _ = move || {
                if let Some(cleanup) = closure_ref.take() {
                    let _ = window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .remove_event_listener_with_callback(
                            &event_name,
                            cleanup.as_ref().unchecked_ref(),
                        );

                    closure_ref.set(None);
                }
            };
        }
    });

    use_effect({
        let id = id.clone();
        let content = content.clone();
        move || {
            let sync_js = format!(
                r#"
                (function syncContent() {{
                    let editor = document.getElementById("{id}");
                    if (editor && editor.__quill) {{
                        let current = editor.__quill.root.innerHTML;
                        let next = `{content}`;
                        if (current !== next) {{
                            editor.__quill.clipboard.dangerouslyPasteHTML(next);
                        }}
                    }}
                }})();
                "#
            );
            let _ = eval(&sync_js);
        }
    });

    rsx! {
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/quill@2.0.0-dev.4/dist/quill.snow.css",
        }
        script { src: "https://cdn.jsdelivr.net/npm/quill@2.0.0-dev.4/dist/quill.min.js" }
        div {
            id: "{id}",
            class: "rich-text-editor w-full min-h-[100px] overflow-y-auto border border-gray-300 rounded-md",
        }
    }
}
