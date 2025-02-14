#![allow(non_snake_case)]

use std::{cell::RefCell, rc::Rc};

use dioxus::prelude::*;
use js_sys::Function;
use num_format::{Locale, ToFormattedString};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

use crate::{
    charts::{
        d3::{self, D3Arc},
        utils::closure,
    },
    theme::ChartTheme,
};

#[component]
pub fn PieChart(
    height: String,
    data: Vec<PieChartData>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let chart_theme: ChartTheme = try_use_context().unwrap_or_default();
    let colors = chart_theme.pie_chart_colors;

    rsx! {
        div {
            height,
            onmounted: move |_el| {
                use dioxus::web::WebEventExt;
                let el = _el.as_web_event();
                let width = el.client_width();
                let height = el.client_height();
                tracing::debug!("width: {}, height: {}", width, height);
                let svg = inject_svg(
                    width,
                    height,
                    data.clone(),
                    colors.iter().map(|e| e.to_string()).collect::<Vec<_>>(),
                );
                el.append_child(&svg).unwrap();
            },
            ..attributes,
            {children}
        }
    }
}

#[derive(Props, Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PieChartData {
    label: String,
    value: i32,
}

impl PieChartData {
    pub fn new(label: String, value: i32) -> Self {
        Self { label, value }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArcData {
    data: PieChartData,
    start_angle: f64,
    end_angle: f64,
    pad_angle: f64,
    value: i32,
}

fn inject_svg(
    width: i32,
    height: i32,
    data: Vec<PieChartData>,
    colors: Vec<String>,
) -> web_sys::Node {
    let width = width as f64;
    let height = height as f64;
    let no_of_items = data.len();
    let total = data.iter().map(|d| d.value).sum::<i32>();

    let radius: f64 = width.min(height) / 2.0;

    let arc = d3::arc().inner_radius(0.0).outer_radius(radius);

    let handle_value = closure(|d: PieChartData| -> i32 { d.value });
    let pie = d3::pie()
        .pad_angle(1.0 / radius)
        .sort(&JsValue::NULL)
        .value(handle_value.as_ref().unchecked_ref());

    let domain = data.iter().map(|d| d.label.to_string()).collect::<Vec<_>>();
    let domain = to_value(&domain).unwrap();
    let interpolate = closure(move |t: f64| -> JsValue {
        tracing::debug!("color interpolating: {}", t);
        d3::interpolate_rgb_basis(colors.clone())
            .call1(&JsValue::NULL, &t.into())
            .unwrap()
    });
    let quantize = d3::quantize(interpolate.as_ref().unchecked_ref(), no_of_items);
    let color = d3::scale_ordinal().domain(&domain).range(&quantize);

    let pie_data = pie
        .call1(&JsValue::NULL, &to_value(&data).unwrap_throw())
        .unwrap();

    let svg = d3::create("svg")
        .attr_with_f64("width", width)
        .attr_with_f64("height", height)
        .attr_with_str(
            "viewBox",
            &format!("{} {} {} {}", -width / 2.0, -height / 2.0, width, height),
        );

    let handle_color = closure(move |d: ArcData| -> JsValue {
        color
            .call1(
                &JsValue::NULL,
                &to_value(&d.data.label.to_string()).unwrap_throw(),
            )
            .unwrap()
    });

    let handle_text = closure(move |d: ArcData| -> String {
        format!(
            "{}: {}",
            d.data.label,
            d.data.value.to_formatted_string(&Locale::en)
        )
    });

    let arc_sync = Rc::new(RefCell::new(arc.clone()));

    let handle_transform = Closure::wrap(Box::new(move |d: JsValue| -> String {
        tracing::debug!("handle_transform");
        let arc: D3Arc = arc_sync.borrow().clone().dyn_into().unwrap();
        let v = arc.centroid(&d);
        format!(
            "translate({})",
            v.iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }) as Box<dyn Fn(JsValue) -> String>);

    let handle_texts = Closure::wrap(Box::new(move |d: JsValue| -> JsValue {
        let text: d3::D3Selection = d.dyn_into().unwrap();
        tracing::debug!("handle_texts ");

        text.append("tspan")
            .attr_with_str("y", "-0.4em")
            .attr_with_str("font-weight", "bold")
            .text(handle_text.as_ref().unchecked_ref())
            .into()
    }) as Box<dyn Fn(JsValue) -> JsValue>);

    let handle_text = closure(move |d: ArcData| -> String {
        format!(
            "{}",
            d.data.label,
            // d.data.value.to_formatted_string(&Locale::en)
        )
    });

    let p = svg
        .append("g")
        .select_all0()
        .data(&pie_data)
        .join("path")
        .attr_with_closure("fill", handle_color.as_ref().unchecked_ref());

    const DURATION: i32 = 100;
    let delay = Closure::wrap(Box::new(|_d: JsValue, i: i32| -> i32 { i * DURATION })
        as Box<dyn FnMut(JsValue, i32) -> i32>);

    let arc_sync = Rc::new(RefCell::new(arc.clone()));

    let handle_anim = closure(move |mut d: ArcData| -> JsValue {
        tracing::debug!("handle_anim");
        let i = d3::interpolate(d.start_angle + 0.1, d.end_angle);
        let arc: Function = arc_sync.borrow().clone().dyn_into().unwrap();

        let f = Closure::wrap(Box::new(move |t: JsValue| -> JsValue {
            d.end_angle = from_value(i.call1(&JsValue::NULL, &t).unwrap_throw()).unwrap();
            let v = to_value(&d).unwrap_throw();
            arc.call1(&JsValue::NULL, &v).unwrap()
        }) as Box<dyn FnMut(JsValue) -> JsValue>);

        tracing::debug!("handle_anim ended");
        let h: Function = f.as_ref().clone().dyn_into().unwrap();
        f.forget();

        h.into()
    });

    p.transition()
        .delay(delay.as_ref().unchecked_ref())
        .duration(DURATION)
        .attr_tween("d", handle_anim.as_ref().unchecked_ref());
    p.append("title").text(handle_text.as_ref().unchecked_ref());

    let f: Function = Function::new_with_args(
        "text",
        &format!(
            r#"
return text.filter(d => (d.endAngle - d.startAngle) > 0.25).append("tspan")
    .attr("x", 0)
    .attr("y", "0.7em")
    .attr("fill-opacity", 0.7)
    .text(d => (d.data.value * 100 / {total}).toLocaleString("en-US") + "%")
"#
        ),
    );

    svg.append("g")
        .attr_with_i32("font-size", 12)
        .attr_with_str("text-anchor", "middle")
        .select_all0()
        .data(&pie_data)
        .join("text")
        .attr("transform", handle_transform.as_ref().unchecked_ref())
        .call(handle_texts.as_ref().unchecked_ref())
        .call(&f);

    handle_anim.forget();
    delay.forget();
    handle_text.forget();
    handle_texts.forget();
    handle_transform.forget();
    interpolate.forget();
    handle_value.forget();

    svg.node().into()
}
