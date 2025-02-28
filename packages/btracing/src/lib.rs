#![allow(non_snake_case)]

// pub use btracing_macros::*;
use dioxus::prelude::*;
pub use dioxus_toast::*;

pub static TOAST: GlobalSignal<ToastManager> = Global::new(|| ToastManager::default());

#[component]
pub fn ToastTracing() -> Element {
    // FIXME: DioxusToast does not consider contents size when fixed.
    rsx! {
        div { class: "fixed max-w-full w-[300px] top-0 right-0 z-[100]",
            dioxus_toast::ToastFrame { manager: TOAST.signal() }
        }
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        if tracing::event_enabled!(tracing::Level::INFO) {
            tracing::info!($($arg)*);
            let msg = format!($($arg)*);

            let p = $crate::ToastInfo {
                heading: None,
                context: msg,
                allow_toast_close: true,
                position: $crate::Position::TopRight,
                icon: None,
                hide_after: Some(6),
            };
            $crate::TOAST.signal().write().popup(p);
        }
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        if tracing::event_enabled!(tracing::Level::ERROR) {
            tracing::error!($($arg)*);
            let msg = format!($($arg)*);

            let p = $crate::ToastInfo {
                heading: None,
                context: msg,
                allow_toast_close: true,
                position: $crate::Position::TopRight,
                icon: None,
                hide_after: Some(6),
            };
            $crate::TOAST.signal().write().popup(p);
        }
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        if tracing::event_enabled!(tracing::Level::WARN) {
            tracing::warn!($($arg)*);
            let msg = format!($($arg)*);

            let p = $crate::ToastInfo {
                heading: None,
                context: msg,
                allow_toast_close: true,
                position: $crate::Position::TopRight,
                icon: None,
                hide_after: Some(6),
            };
            $crate::TOAST.signal().write().popup(p);
        }
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        tracing::debug!($($arg)*)
    }
}
