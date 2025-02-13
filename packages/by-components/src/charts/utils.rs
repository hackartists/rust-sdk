use serde_wasm_bindgen::from_value;
use wasm_bindgen::{prelude::Closure, JsValue, UnwrapThrowExt};

pub fn to_script(js_code: &str) -> Result<web_sys::Element, dioxus::prelude::RenderError> {
    use dioxus::{prelude::RenderError, CapturedError};
    let window = web_sys::window().ok_or(RenderError::Aborted(CapturedError::from_display(
        "failed to get window",
    )))?;
    let document = window
        .document()
        .ok_or(RenderError::Aborted(CapturedError::from_display(
            "failed to get document",
        )))?;
    let script = document.create_element("script").map_err(|e| {
        tracing::debug!("failed to create script {:?}", e);
        RenderError::Aborted(CapturedError::from_display("failed to create a script tag"))
    })?;
    script.set_text_content(Some(js_code));

    Ok(script)
}

pub fn closure<F, I, R>(mut f: F) -> Closure<dyn FnMut(JsValue) -> JsValue>
where
    F: FnMut(I) -> R + 'static,
    R: Into<JsValue>,
    I: serde::de::DeserializeOwned,
{
    let h = Closure::wrap(Box::new(move |v: JsValue| -> JsValue {
        let data: I = from_value(v).unwrap_throw();
        f(data).into()
    }) as Box<dyn FnMut(JsValue) -> JsValue>);

    h
}
