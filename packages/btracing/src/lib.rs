#![allow(non_snake_case)]

// pub use btracing_macros::*;
use dioxus::prelude::*;
use dioxus_toast::*;

static TOAST: GlobalSignal<ToastManager> = Global::new(|| ToastManager::default());

#[component]
pub fn ToastTracing() -> Element {
    // FIXME: DioxusToast does not consider contents size when fixed.
    rsx! {
        div { class: "fixed max-w-full w-[300px] top-0 right-0 z-[100]",
            dioxus_toast::ToastFrame { manager: TOAST.signal() }
        }
    }
}

pub fn info(msg: impl Into<String>) {
    let msg = msg.into();
    tracing::info!("btracing::info {}", msg);

    let p = ToastInfo {
        heading: None,
        context: msg,
        allow_toast_close: true,
        position: Position::TopRight,
        icon: None,
        hide_after: Some(6),
    };
    TOAST.signal().write().popup(p);
}

pub fn error(msg: impl Into<String>) {
    let msg = msg.into();
    tracing::error!("btracing::error {}", msg);

    let p = ToastInfo {
        heading: None,
        context: msg,
        allow_toast_close: true,
        position: Position::TopRight,
        icon: None,
        hide_after: Some(6),
    };
    TOAST.signal().write().popup(p);
}

pub fn warn(msg: impl Into<String>) {
    let msg = msg.into();
    tracing::error!("btracing::warn {}", msg);

    let p = ToastInfo {
        heading: None,
        context: msg,
        allow_toast_close: true,
        position: Position::TopRight,
        icon: None,
        hide_after: Some(6),
    };
    TOAST.signal().write().popup(p);
}

pub fn debug(msg: impl Into<String>) {
    let msg = msg.into();
    tracing::debug!("btracing::error {}", msg);
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        btracing::info(format!($($arg)*))
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        btracing::error(format!($($arg)*))
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        btracing::warn(format!($($arg)*))
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        btracing::debug(format!($($arg)*))
    }
}
