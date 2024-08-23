use crate::prelude::*;

/// Renders the `page_base` template
/// ## Arguments
/// * `title` - The title of the page. Used in the <title> tag and the navbar.
/// * `content` - The content to be inserted into the #content div.
/// ## Returns
/// The rendered template.
/// ## Errors
/// If the template fails to render, a `tera::Error` is returned
pub fn render(title: &str, content: &str) -> Result<String, tera::Error> {
    let mut ctx = TeraContext::new();
    
    ctx.insert("title", title);
    
    let navbar =super::navbar::render(title)?;
    ctx.insert("navbar", &navbar);
    
    ctx.insert("content", content);

    template.render("components/page_base.html", &ctx)
}