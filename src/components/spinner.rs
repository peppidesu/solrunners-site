use crate::prelude::*;

/// Renders the `spinner` template
/// ## Returns
/// The rendered template.
/// ## Errors
/// If the template fails to render, a `tera::Error` is returned
pub fn render() -> Result<String, tera::Error> {
    let ctx = TeraContext::new();
    template.render("components/spinner.html", &ctx)
}