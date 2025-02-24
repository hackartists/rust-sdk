#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn CloseCircle(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {

        svg {
            fill: "none",
            height: "16",
            view_box: "0 0 16 16",
            width: "16",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M8 1C4.1 1 1 4.1 1 8C1 11.9 4.1 15 8 15C11.9 15 15 11.9 15 8C15 4.1 11.9 1 8 1ZM10.7 11.5L8 8.8L5.3 11.5L4.5 10.7L7.2 8L4.5 5.3L5.3 4.5L8 7.2L10.7 4.5L11.5 5.3L8.8 8L11.5 10.7L10.7 11.5Z",
                fill: "black",
            }
        }
    }
}
