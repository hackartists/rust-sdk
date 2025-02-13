#![allow(non_snake_case)]

use std::{cell::RefCell, rc::Rc, sync::Arc};

use dioxus::prelude::*;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::{prelude::*, JsCast, JsValue};

use crate::{charts::d3, theme::ChartTheme};

#[derive(Props, Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StackBarData {
    label: String,
    value: i32,
}

impl StackBarData {
    pub fn new(label: String, value: i32) -> Self {
        Self { label, value }
    }
}

#[component]
pub fn StackBarChart(
    id: String,
    height: String,
    data: Vec<StackBarData>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let chart_theme: ChartTheme = try_use_context().unwrap_or_default();
    let color_pool = chart_theme.stack_bar_color_pool;

    rsx! {
        div {
            class: "w-full flex flex-col gap-[10px] rounded-[8px] overflow-hidden",
            id: "{id}",
            height,
            onmounted: move |_el| {
                use dioxus::web::WebEventExt;
                let el = _el.as_web_event();
                let svg = inject_d3_chart(
                    &id,
                    el.client_width(),
                    el.client_height(),
                    &color_pool,
                    &data,
                );
                el.append_child(&svg).unwrap();
            },
            ..attributes,
            {children}
        }
    }
}

pub fn inject_d3_chart(
    id: &str,
    width: i32,
    height: i32,
    colors: &Vec<String>,
    data: &Vec<StackBarData>,
) -> web_sys::Node {
    tracing::debug!("injecting d3 chart: {} {} {}", id, width, height);
    let total = data.iter().map(|d| d.value).sum::<i32>();
    let svg = d3::select_svg(&format!("#{}", id))
        .append("svg")
        .attr_with_i32("width", width)
        .attr_with_i32("height", height);
    let data = to_value(&data).unwrap();

    let x_offset = Rc::new(RefCell::new(0));
    let text_offset = Rc::new(RefCell::new(0));
    let index = Rc::new(RefCell::new(0));
    let colors = Arc::new(colors.clone());
    const TEXT_LEFT_PADDING: i32 = 17;

    let handle_start_offset = Closure::wrap(Box::new(move |v: JsValue| -> i32 {
        let data: StackBarData = from_value(v).unwrap_throw();
        let mut x_offset = x_offset.borrow_mut();
        let prev_x = *x_offset;
        *x_offset += ((data.value as f32 / total as f32) * width as f32) as i32;
        prev_x
    }) as Box<dyn FnMut(JsValue) -> i32>);

    let handle_width = Closure::wrap(Box::new(move |v: JsValue| -> i32 {
        let data: StackBarData = from_value(v).unwrap_throw();
        ((data.value as f32 / total as f32) * width as f32) as i32
    }) as Box<dyn FnMut(JsValue) -> i32>);

    let handle_color = Closure::wrap(Box::new(move |_v: JsValue| -> String {
        let mut i = index.borrow_mut();
        let color = colors[*i % colors.len()].clone();
        *i = (*i + 1) % colors.len();
        color
    }) as Box<dyn FnMut(JsValue) -> String>);

    let handle_text_offset = Closure::wrap(Box::new(move |v: JsValue| -> i32 {
        let data: StackBarData = from_value(v).unwrap_throw();
        let mut text_offset = text_offset.borrow_mut();
        let prev_x = *text_offset;
        *text_offset += ((data.value as f32 / total as f32) * width as f32) as i32;
        prev_x + TEXT_LEFT_PADDING
    }) as Box<dyn FnMut(JsValue) -> i32>);

    let bars = svg
        .select_all("rect")
        .data(&data)
        .enter()
        .append("rect")
        .attr_with_closure("x", handle_start_offset.as_ref().unchecked_ref())
        .attr_with_i32("y", 0)
        .attr_with_closure("fill", handle_color.as_ref().unchecked_ref())
        .attr_with_str("stroke", "#fff")
        .attr_with_i32("height", height);

    bars.transition()
        .duration(1000) // Animation duration (1 second)
        .attr_with_closure("width", handle_width.as_ref().unchecked_ref());

    let handle_text = Closure::wrap(Box::new(|v: JsValue| -> String {
        from_value::<StackBarData>(v).unwrap_throw().label
    }) as Box<dyn FnMut(JsValue) -> String>);

    let texts = svg
        .select_all("text")
        .data(&data)
        .enter()
        .append("text")
        .attr_with_closure("x", handle_start_offset.as_ref().unchecked_ref())
        .attr_with_i32("y", height / 2)
        .attr_with_str("fill", "white")
        .attr_with_str("font-size", "14px")
        .attr_with_str("font-weight", "bold")
        .attr_with_str("text-anchor", "start")
        .attr_with_str("dominant-baseline", "middle")
        .text(handle_text.as_ref().unchecked_ref());

    texts
        .transition()
        .duration(1000)
        .attr_with_closure("x", handle_text_offset.as_ref().unchecked_ref());

    handle_start_offset.forget();
    handle_width.forget();
    handle_color.forget();
    handle_text.forget();

    svg.node().into()
}
