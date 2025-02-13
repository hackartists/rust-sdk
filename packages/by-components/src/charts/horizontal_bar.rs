// #![allow(non_snake_case)]
// use dioxus::prelude::*;
// use std::{cell::RefCell, rc::Rc, sync::Arc};

// use dioxus::prelude::*;
// use serde_wasm_bindgen::{from_value, to_value};
// use wasm_bindgen::{prelude::*, JsCast, JsValue};

// use crate::{charts::d3, theme::ChartTheme};

// #[component]
// pub fn HorizontalBar(
//     id: String,
//     value: i64,
//     max_value: i64,
//     #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
//     children: Element,
// ) -> Element {
//     let chart_theme: ChartTheme = try_use_context().unwrap_or_default();
//     let color_pool = chart_theme.stack_bar_color_pool;

//     rsx! {
//         div {
//             id: "{id}",
//             onmounted: move |_el| {
//                 use dioxus::web::WebEventExt;
//                 let el = _el.as_web_event();
//                 let width = el.client_width();
//                 let height = el.client_height();
//                 let svg = d3::select_svg(&format!("#{}", id))
//                     .append("svg")
//                     .attr_with_i32("width", width)
//                     .attr_with_i32("height", height);
//                 el.append_child(&svg).unwrap();
//             },
//             ..attributes,
//             {children}
//         }
//     }
// }
