use serde::Serialize;

use crate::prelude::*;

/// Struct representing a page button in the navbar
#[derive(Serialize)]
struct Page<'a> {
    /// The name of the page as it appears in the navbar
    name: &'a str,
    /// The URI to redirect to when the page is clicked
    uri: &'a str
}

/// List of pages in the navbar
static PAGES: [Page; 3] = [
    Page { name: "home",        uri: "/" },
    Page { name: "now",         uri: "/now" },
    Page { name: "zine",        uri: "/zine" },    
];

/// Renders the navbar template
/// ## Arguments
/// * `current_page` - The name of the current page. Used to highlight the corresponding button in 
///   the navbar.
/// ## Returns
/// A string containing the rendered template.
/// ## Errors
/// If the template fails to render, a `tera::Error` is returned.
pub fn render() -> Result<String, tera::Error> {
    let mut ctx = TeraContext::new();         
    ctx.insert("pages", &PAGES);    
    template.render("components/navbar.html", &ctx)
}