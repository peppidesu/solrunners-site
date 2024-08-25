use crate::prelude::*;

/// Renders the `page_base` template
/// ## Arguments
/// * `title` - The title of the page. Used in the <title> tag and the navbar.
/// * `content` - The content to be inserted into the #content div.
/// ## Returns
/// The rendered template.
/// ## Errors
/// If the template fails to render, a `tera::Error` is returned
pub fn render(content_uri: &str) -> Result<String, tera::Error> {
    let mut ctx = TeraContext::new();

    let navbar = super::navbar::render()?;
    let spinner = super::spinner::render()?;
    ctx.insert("navbar", &navbar);
    ctx.insert("placeholder", &spinner);
    ctx.insert("content_uri", content_uri);

    template.render("components/page_base.html", &ctx)
}