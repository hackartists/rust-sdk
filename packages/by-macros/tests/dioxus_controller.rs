use by_macros::DioxusController;

type Signal<T> = fn() -> T;

#[derive(DioxusController)]
pub struct Controller {
    pub s1_type: Signal<String>,
    pub s2_type: Signal<String>,
}

#[test]
fn test_dioxus_controller() {
    let ctrl = Controller {
        s1_type: || "Hello".to_string(),
        s2_type: || "Hello".to_string(),
    };

    assert_eq!(ctrl.s1_type(), "Hello");
}
