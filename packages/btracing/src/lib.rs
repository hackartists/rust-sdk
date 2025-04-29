#![allow(non_snake_case)]

use dioxus::prelude::*;

pub static TOAST: GlobalSignal<Option<ToastMessage>> = Global::new(|| None);

#[derive(Debug, Clone, Copy)]
pub enum ToastType {
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone)]
pub struct ToastMessage {
    pub toast_type: ToastType,
    pub message: String,
}

/// children should be symbol logo
#[component]
pub fn ToastTracing(
    children: Element,
    #[props(default = "#181B20".to_string())] background: String,
    #[props(default = "#F75C14".to_string())] warn: String,
    #[props(default = "#428EFF".to_string())] info: String,
    #[props(default = "#FF4242".to_string())] error: String,
    #[props(default = 5000)] interval: u32,
) -> Element {
    let css = include_str!("./btracing.css");
    let mut toast = TOAST.signal();

    rsx! {
        style { "{css}" }
        div {
            class: "btracing-toast",
            id: "btracing-toast-template",
            width: "100px",
            onclick: move |_| {
                *toast.write() = None;
            },
            div {
                class: "btracing-toast-inner",
                right: if toast().is_some() { "0px" } else { "-300px" },
                div { class: "btracing-toast-level-bar-container",
                    if let Some(ToastMessage { ref toast_type, .. }) = toast() {
                        div {
                            class: "btracing-toast-level-bar",
                            background_color: match toast_type {
                                ToastType::Info => info.clone(),
                                ToastType::Warn => warn.clone(),
                                ToastType::Error => error.clone(),
                            },
                        }
                    }
                }
                div { class: "btracing-toast-content",
                    div { class: "btracing-toast-header",
                        {children}
                        h3 { class: "btracing-toast-header-text", "" }
                    }
                    if let Some(ToastMessage { ref message, .. }) = toast() {
                        p { class: "btracing-toast-msg", "{message}" }
                    }
                }
            }
        }
        if toast().is_some() {
            div {
                onmounted: move |_| async move {
                    gloo_timers::future::TimeoutFuture::new(interval).await;
                    *TOAST.signal().write() = None;
                },
            }
        }
    }
}

#[derive(serde::Serialize)]
pub struct SlackMessage<'a> {
    pub text: &'a str,
}

#[macro_export]
macro_rules! notify {
    ($hook:expr, $msg:expr) => {
        let client = reqwest::Client::new();
        let payload = btracing::SlackMessage { text: $msg };

        for i in 0..3 {
            if let Ok(_) = client.post($hook).json(&payload).send().await {
                break;
            } else {
                if i == 3 {
                    tracing::error!("Failed to send Slack message");
                    break;
                }
                tracing::warn!("Failed to send Slack message, attempt {}/3", i + 1);
            }
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        if tracing::event_enabled!(tracing::Level::INFO) {
            tracing::info!($($arg)*);
            let message = format!($($arg)*);

            let p = $crate::ToastMessage {
                toast_type: $crate::ToastType::Info,
                message,
            };
            *$crate::TOAST.signal().write() = Some(p);
        }
    }
}

#[macro_export]
macro_rules! i {
    ($msg:expr) => {
        if tracing::event_enabled!(tracing::Level::INFO) {
            let message = format!("{}", $msg.translate(&Language::En));
            tracing::error!("{}", message);

            let p = $crate::ToastMessage {
                toast_type: $crate::ToastType::Info,
                message,
            };
            *$crate::TOAST.signal().write() = Some(p);
        }
    };

    ($lang:expr, $msg:expr) => {
        if tracing::event_enabled!(tracing::Level::INFO) {
            let message = format!("{}", $msg.translate(&$lang));
            tracing::error!("{}", message);

            let p = $crate::ToastMessage {
                toast_type: $crate::ToastType::Info,
                message,
            };
            *$crate::TOAST.signal().write() = Some(p);
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        if tracing::event_enabled!(tracing::Level::ERROR) {
            tracing::error!($($arg)*);
            let message = format!($($arg)*);

            let p = $crate::ToastMessage {
                toast_type: $crate::ToastType::Error,
                message,
            };
            *$crate::TOAST.signal().write() = Some(p);
        }
    }
}

#[macro_export]
macro_rules! e {
    ($err:expr) => {
        if tracing::event_enabled!(tracing::Level::ERROR) {
            let message = format!("{}", $err.translate(&Language::En));
            tracing::error!("{}", message);

            let p = $crate::ToastMessage {
                toast_type: $crate::ToastType::Error,
                message,
            };
            *$crate::TOAST.signal().write() = Some(p);
        }
    };

    ($lang:expr, $err:expr) => {
        if tracing::event_enabled!(tracing::Level::ERROR) {
            let message = format!("{}", $err.translate(&$lang));
            tracing::error!("{}", message);

            let p = $crate::ToastMessage {
                toast_type: $crate::ToastType::Error,
                message,
            };
            *$crate::TOAST.signal().write() = Some(p);
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        if tracing::event_enabled!(tracing::Level::WARN) {
            tracing::warn!($($arg)*);
            let message = format!($($arg)*);

            let p = $crate::ToastMessage {
                toast_type: $crate::ToastType::Warn,
                message,
            };
            *$crate::TOAST.signal().write() = Some(p);
        }
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        tracing::debug!($($arg)*)
    }
}
