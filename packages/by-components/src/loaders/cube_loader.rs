use dioxus::prelude::*;

use crate::theme::ColorTheme;

#[component]
pub fn CubeLoader() -> Element {
    let color: ColorTheme = try_use_context().unwrap_or_default();
    let cube_color = &color.loader.primary;
    let cube_border_color = &color.loader.secondary;

    let size = "80px";
    let css = include_str!("cube_loader.css");
    let face_classes = ["shadow", "bottom", "top", "left", "right", "back", "front"];

    rsx! {
        style { "{css}" }
        div {
            class: "scene",
            height: "calc({size} * 2.75)",
            width: "calc({size} * 2.75)",
            div { class: "cube-wrapper",
                div { class: "cube",
                    div { class: "cube-faces", width: size, height: size,
                        for face_class in face_classes.iter() {
                            div {
                                class: "cube-face {face_class}",
                                background: "{cube_color}",
                                border: "solid 1px {cube_border_color}",
                            }
                        }
                    }
                }
            }
        }
    }
}
