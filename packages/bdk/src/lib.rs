pub mod prelude {
    pub use by_macros;
    pub use by_macros::*;
    pub use by_types;
    pub use dioxus_translate;
    pub use dioxus_translate::*;
    pub use reqwest;
    pub use rest_api;
    pub use tracing;
    pub use validator;

    #[cfg(feature = "fe")]
    pub use btracing;
    #[cfg(feature = "fe")]
    pub use by_components;
    #[cfg(feature = "fe")]
    pub use dioxus;
    #[cfg(feature = "fe")]
    pub use dioxus::prelude::*;
    #[cfg(feature = "fe")]
    pub use dioxus_aws;
    #[cfg(feature = "fe")]
    pub use dioxus_logger;
    #[cfg(feature = "fe")]
    pub use dioxus_popup;

    #[cfg(feature = "server")]
    pub use dioxus::fullstack;
    #[cfg(feature = "server")]
    pub use dioxus::fullstack::prelude::*;

    #[cfg(feature = "be")]
    pub use aide;
    #[cfg(feature = "be")]
    pub use bigdecimal;
    #[cfg(feature = "be")]
    pub use schemars;
    #[cfg(feature = "be")]
    pub use schemars::JsonSchema;

    #[cfg(feature = "be")]
    pub use by_axum;
    #[cfg(feature = "be")]
    pub use schemars;
    #[cfg(feature = "be")]
    pub use sqlx;
}
