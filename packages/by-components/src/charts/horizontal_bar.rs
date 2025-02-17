#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{charts::d3, theme::ChartTheme};

#[component]
pub fn HorizontalBar(
    id: String,
    value: i64,
    height: String,
    max_value: i64,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let chart_theme: ChartTheme = try_use_context().unwrap_or_default();
    let colors = chart_theme.horizontal_bar_gradient_colors;

    use_effect(use_reactive((&value, &max_value), {
        let id = id.clone();
        move |(value, max_value)| {
            let el = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id(&id)
                .unwrap();

            while let Some(child) = el.first_child() {
                el.remove_child(&child).unwrap();
            }

            let width = el.client_width();
            let height = el.client_height();

            let svg = inject_svg(
                value as f64 / max_value as f64 * width as f64,
                height,
                &colors,
            );
            el.append_child(&svg).unwrap();
        }
    }));

    rsx! {
        div { id, height, ..attributes, {children} }
    }
}

fn inject_svg(width: f64, height: i32, colors: &Vec<&'static str>) -> web_sys::Node {
    let svg = d3::create("svg")
        .attr_with_f64("width", width)
        .attr_with_i32("height", height);

    let defs = svg.append("defs");

    let clippath_id = "rounded-right";
    let clippath = defs.append("clipPath").attr_with_str("id", &clippath_id);

    let cliprect = clippath
        .append("rect")
        .attr_with_i32("height", height)
        .attr_with_i32("rx", 10)
        .attr_with_i32("ry", 10);

    let gradient = defs
        .append("linearGradient")
        .attr_with_str("id", "barGradientColor")
        .attr_with_str("x1", "0%")
        .attr_with_str("y1", "0%")
        .attr_with_str("x2", "100%")
        .attr_with_str("y2", "0%");

    for (i, color) in colors.into_iter().enumerate() {
        let pos = format!("{}%", (100 * i) / (colors.len().saturating_sub(1)).max(1));
        tracing::debug!("Gradient color stop {} at {}", color, pos);

        gradient
            .append("stop")
            .attr_with_str("offset", &pos)
            .attr_with_str("stop-color", color);
    }

    let bars = svg
        .append("rect")
        .attr_with_i32("x", 0)
        .attr_with_i32("y", 0)
        .attr_with_i32("height", height)
        .attr_with_str("fill", "url(#barGradientColor)")
        .attr_with_str("clip-path", format!("url(#{clippath_id})").as_str());

    bars.transition()
        .duration(1000)
        .attr_with_f64("width", width);

    cliprect
        .transition()
        .duration(1000)
        .attr_with_f64("width", width);

    svg.node().into()
}
