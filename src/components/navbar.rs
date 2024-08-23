use serde::Serialize;

use crate::prelude::*;

#[derive(Serialize)]
struct Page<'a> {
    name: &'a str,
    uri: &'a str
}

static PAGES: [Page; 4] = [
    Page { name: "home",        uri: "/" },
    Page { name: "now",         uri: "/now" },
    Page { name: "zine",        uri: "/zine" },
    Page { name: "about",       uri: "/about" },
];

/// Renders the navbar template
/// ## Arguments
/// * `current_page` - The name of the current page. Used to highlight the corresponding button in 
///   the navbar.
/// ## Returns
/// A string containing the rendered template.
/// ## Errors
/// If the template fails to render, a `tera::Error` is returned.
pub fn render(current_page: &str) -> Result<String, tera::Error> {
    let mut ctx = TeraContext::new();         
    ctx.insert("pages", &PAGES);
    ctx.insert("current_page", current_page);
    template.render("components/navbar.html", &ctx)
}