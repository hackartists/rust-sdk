#![allow(non_snake_case)]
use by_macros::DioxusController;
use dioxus::prelude::*;

/// Responsive component provides a way to render different components based on the screen size.
/// Set mobile_first to true if you want to render mobile component first.
/// If mobile first is set, tablet will be rendered when the screen size is larger than or equals `tablet`.
/// If mobile first is not set, tablet will be rendered when the screen size is smaller than or equals `tablet`.
///
/// Popular parameters
/// - desktop-first: 480.0(mobile), 1024.0(tablet), 1440.0(desktop)-don't use
/// - mobile-first: 320.0(mobile)-don't use, 768.0(tablet), 1024.0(desktop)
///
/// # Example
/// ```no_run
/// use by_components::responsive::Responsive;
/// use dioxus::prelude::*;
///
/// fn app() -> Element {
///     rsx! {
///         Responsive { Router::<Route> {} }
///     }
/// }
/// ```
#[component]
pub fn Responsive(
    #[props(default = 480.0)] mobile: f64,
    #[props(default = 1024.0)] tablet: f64,
    #[props(default = 1024.0)] desktop: f64,
    #[props(default = false)] mobile_first: bool,
    children: Element,
) -> Element {
    let mut srv = ResponsiveService::new(mobile, tablet, desktop, mobile_first);

    rsx! {
        div {
            class: "w-full h-full",
            onresize: move |e| {
                let width = e.get_border_box_size().unwrap_or_default().width;
                tracing::debug!("ResponsiveComponent: onresize width: {width}");
                srv.width.set(width);
            },
            {children}
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    by_macros::EnumProp,
    Default,
)]
#[serde(rename_all = "kebab-case")]
pub enum PlatformSize {
    Mobile,
    Tablet,
    #[default]
    Desktop,
}

#[derive(Clone, Copy, DioxusController)]
pub struct ResponsiveService {
    mobile: Signal<f64>,
    tablet: Signal<f64>,
    desktop: Signal<f64>,
    width: Signal<f64>,
    platform_size: Memo<PlatformSize>,
}

impl ResponsiveService {
    pub fn new(mobile: f64, tablet: f64, desktop: f64, mobile_first: bool) -> Self {
        let size = use_signal(|| if mobile_first { mobile } else { desktop });
        let mobile = use_signal(|| mobile);
        let tablet = use_signal(|| tablet);
        let desktop = use_signal(|| desktop);
        let srv = Self {
            mobile,
            tablet,
            desktop,
            width: size,
            platform_size: use_memo(move || {
                let size = size();
                let mobile = mobile();
                let tablet = tablet();
                let desktop = desktop();

                if !mobile_first {
                    if size <= mobile {
                        PlatformSize::Mobile
                    } else if size <= tablet {
                        PlatformSize::Tablet
                    } else {
                        PlatformSize::Desktop
                    }
                } else {
                    if size >= tablet {
                        PlatformSize::Tablet
                    } else if size >= desktop {
                        PlatformSize::Desktop
                    } else {
                        PlatformSize::Mobile
                    }
                }
            }),
        };
        use_context_provider(move || srv);

        srv
    }
}
