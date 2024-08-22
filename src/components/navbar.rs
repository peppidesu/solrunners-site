use crate::prelude::*;

/// Renders the navbar template
/// # Returns
/// A string containing the rendered template
/// # Errors
/// If the template fails to render, a
pub fn navbar() -> Result<String, tera::Error> {
    let ctx = TeraContext::new();     
    template.render("components/navbar.html", &ctx)
}