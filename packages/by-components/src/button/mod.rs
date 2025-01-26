#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Button(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        button { ..attributes,{children} }
    }
}
