pub fn to_script(js_code: &str) -> Result<web_sys::Element, dioxus::prelude::RenderError> {
    use dioxus::{prelude::RenderError, CapturedError};
    let window = web_sys::window().ok_or(RenderError::Aborted(CapturedError::from_display(
        "failed to get window",
    )))?;
    let document = window
        .document()
        .ok_or(RenderError::Aborted(CapturedError::from_display(
            "failed to get document",
        )))?;
    let script = document.create_element("script").map_err(|e| {
        tracing::debug!("failed to create script {:?}", e);
        RenderError::Aborted(CapturedError::from_display("failed to create a script tag"))
    })?;
    script.set_text_content(Some(js_code));

    Ok(script)
}
