#![allow(non_snake_case)]

use std::{cell::RefCell, rc::Rc, sync::Arc};

use dioxus::prelude::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsCast;

use crate::{
    charts::{d3, utils::closure},
    theme::ColorTheme,
};

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
    height: String,
    data: Vec<StackBarData>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let color: ColorTheme = try_use_context().unwrap_or_default();
    let color_pool = color.chart.stack_bar_color_pool;

    rsx! {
        div {
            height,
            onmounted: move |_el| {
                use dioxus::web::WebEventExt;
                let el = _el.as_web_event();
                let svg = inject_d3_chart(
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

fn inject_d3_chart(
    width: i32,
    height: i32,
    colors: &Vec<&'static str>,
    data: &Vec<StackBarData>,
) -> web_sys::Node {
    tracing::debug!("injecting d3 chart:  {} {}", width, height);
    let total = data.iter().map(|d| d.value).sum::<i32>();
    let svg = d3::create("svg")
        .attr_with_i32("width", width)
        .attr_with_i32("height", height);
    let data = to_value(&data).unwrap();

    let x_offset = Rc::new(RefCell::new(0));
    let text_offset = Rc::new(RefCell::new(0));
    let index = Rc::new(RefCell::new(0));
    let colors = Arc::new(colors.clone());
    const TEXT_LEFT_PADDING: i32 = 17;
    let duration = 1000;

    let handle_start_offset = closure(move |data: StackBarData| -> i32 {
        let mut x_offset = x_offset.borrow_mut();
        let prev_x = *x_offset;
        *x_offset += ((data.value as f32 / total as f32) * width as f32) as i32;
        prev_x
    });

    let acc = Rc::new(RefCell::new(0));
    let remained_width = Rc::new(RefCell::new(width));

    let handle_width = closure(move |data: StackBarData| -> i32 {
        let mut acc = acc.borrow_mut();
        *acc += data.value;

        let w = ((data.value as f32 / total as f32) * width as f32) as i32;
        let mut full_width = remained_width.borrow_mut();

        if *acc == total {
            width - *full_width
        } else {
            *full_width -= w;
            w
        }
    });

    let handle_color = closure(move |_v: StackBarData| -> String {
        let mut i = index.borrow_mut();
        let color = colors[*i % colors.len()];
        *i = (*i + 1) % colors.len();
        color.to_string()
    });

    let handle_text_offset = closure(move |data: StackBarData| -> i32 {
        let mut text_offset = text_offset.borrow_mut();
        let prev_x = *text_offset;
        *text_offset += ((data.value as f32 / total as f32) * width as f32) as i32;
        prev_x + TEXT_LEFT_PADDING
    });

    let sum = Rc::new(RefCell::new(0));
    let handle_delay = closure(move |data: StackBarData| -> i32 {
        let mut sum = sum.borrow_mut();
        let delay_percent = *sum as f32 / total as f32;

        *sum += data.value;

        (duration as f32 * delay_percent) as i32
    });

    let handle_duration = closure(move |data: StackBarData| -> i32 {
        let percent = data.value as f32 / total as f32;

        (duration as f32 * percent) as i32
    });

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
        .delay(handle_delay.as_ref().unchecked_ref())
        .duration_with_closure(handle_duration.as_ref().unchecked_ref())
        .attr_with_closure("width", handle_width.as_ref().unchecked_ref());

    let handle_text = closure(|data: StackBarData| -> String { data.label });

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
        .duration(duration)
        .attr_with_closure("x", handle_text_offset.as_ref().unchecked_ref());

    handle_start_offset.forget();
    handle_width.forget();
    handle_color.forget();
    handle_text.forget();
    handle_duration.forget();
    handle_delay.forget();

    svg.node().into()
}
