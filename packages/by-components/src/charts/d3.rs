use js_sys::Function;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = d3)]
    pub fn select(selector: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = d3, js_name = select)]
    pub fn select_svg(selector: &str) -> D3Selection;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "Object")]
    pub type D3Selection;

    #[wasm_bindgen(method, js_name = append)]
    pub fn append(this: &D3Selection, element: &str) -> D3Selection;

    #[wasm_bindgen(method, js_name = attr)]
    pub fn attr(this: &D3Selection, name: &str, value: &JsValue) -> D3Selection;

    #[wasm_bindgen(method, js_name = attr)]
    pub fn attr_with_str(this: &D3Selection, name: &str, value: &str) -> D3Selection;

    // attr function with an integer value
    #[wasm_bindgen(method, js_name = attr)]
    pub fn attr_with_i32(this: &D3Selection, name: &str, value: i32) -> D3Selection;

    // attr function with a floating-point number
    #[wasm_bindgen(method, js_name = attr)]
    pub fn attr_with_f64(this: &D3Selection, name: &str, value: f64) -> D3Selection;

    // attr function with a JavaScript function (closure)
    #[wasm_bindgen(method, js_name = attr)]
    pub fn attr_with_closure(this: &D3Selection, name: &str, value: &Function) -> D3Selection;

    #[wasm_bindgen(method, js_name = style)]
    pub fn style(this: &D3Selection, name: &str, value: &str) -> D3Selection;

    #[wasm_bindgen(method, js_name = data)]
    pub fn data(this: &D3Selection, data: &JsValue) -> D3Selection;

    #[wasm_bindgen(method, js_name = enter)]
    pub fn enter(this: &D3Selection) -> D3Selection;

    #[wasm_bindgen(method, js_name = selectAll)]
    pub fn select_all(this: &D3Selection, selector: &str) -> D3Selection;

    #[wasm_bindgen(method, js_name = text)]
    pub fn text(this: &D3Selection, value: &Function) -> D3Selection;

    #[wasm_bindgen(method, js_name = node)]
    pub fn node(this: &D3Selection) -> JsValue;

    #[wasm_bindgen(method, js_name = transition)]
    pub fn transition(this: &D3Selection) -> D3Selection;

    #[wasm_bindgen(method, js_name = duration)]
    pub fn duration(this: &D3Selection, time: i32) -> D3Selection;

    #[wasm_bindgen(method, js_name = duration)]
    pub fn duration_with_closure(this: &D3Selection, time: &Function) -> D3Selection;

    #[wasm_bindgen(method, js_name = delay)]
    pub fn delay(this: &D3Selection, time: &Function) -> D3Selection;
}
