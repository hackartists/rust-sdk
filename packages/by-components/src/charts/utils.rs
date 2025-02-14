use std::str::FromStr;

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
        // web_sys::console::log_1(&v);
        let data: I = from_value(v).unwrap_throw();
        let v = f(data).into();
        // web_sys::console::log_1(&v);
        v
    }) as Box<dyn FnMut(JsValue) -> JsValue>);

    h
}

pub fn closure_with_index<F, I, R>(mut f: F) -> Closure<dyn FnMut(JsValue, usize) -> JsValue>
where
    F: FnMut(I, usize) -> R + 'static,
    R: Into<JsValue>,
    I: serde::de::DeserializeOwned,
{
    let h = Closure::wrap(Box::new(move |v: JsValue, i: usize| -> JsValue {
        let data: I = from_value(v).unwrap_throw();
        f(data, i).into()
    }) as Box<dyn FnMut(JsValue, usize) -> JsValue>);

    h
}

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub opacity: f32,
}

impl Color {
    pub fn new(r: i32, g: i32, b: i32, opacity: f32) -> Self {
        Self { r, g, b, opacity }
    }

    pub fn to_css(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.opacity)
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('#');
        let r = i32::from_str_radix(&s[0..2], 16).map_err(|_| ())?;
        let g = i32::from_str_radix(&s[2..4], 16).map_err(|_| ())?;
        let b = i32::from_str_radix(&s[4..6], 16).map_err(|_| ())?;
        Ok(Self::new(r, g, b, 1.0))
    }
}
