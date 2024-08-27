use crate::{components, prelude::*};
use crate::routes::prelude::*;
use rocket::Request;

/// The default error catcher
#[catch(default)]
pub fn default_html_catcher(stat: Status, _: &Request) -> (ContentType, String) {
    // Embed simple error page in the base page template
    let content = components::page_base::render(&error_page(stat))
        .handle_tera_error()        
        .unwrap_or_else(|_| error_page(Status::InternalServerError));

    (ContentType::HTML, content)
}

/// Simple error catcher for `/page/*`
#[catch(default)]
pub fn page_html_catcher(stat: Status, _: &Request) -> (ContentType, String) {
    (ContentType::HTML, error_page(stat))
}

/// Generates an error page
pub fn error_page(stat: Status) -> String {
    let mut ctx = TeraContext::new();

    // Set the status code
    ctx.insert("code", &stat.code);
    
    // Exlamation text (header of the page)
    ctx.insert("exclamation", match stat.code {
        401 => "Please log in.",
        403 => "Whoa there!",
        404 => "So empty...",
        410 => "Sorry!",
        429 => "Hey! Slow down!",
        400..=499 => "Oops!",
        500..=599 => "Well, this is embarrassing.",
        727 => "WYSI",
        _ => "Oh no!"
    });

    // Readable error message (subheader of the page)
    ctx.insert("readable", match stat.code {
        401 => "You need to be logged in to view this page.",
        403 => "You don't have permission to view this page.",
        404 => "We couldn't find the page you were looking for.",        
        410 => "This page has been removed.",
        429 => "You're making too many requests. Please try again later.",
        400..=499 => "Something was wrong with your request.",
        500..=599 => "Something went wrong on our end. Please try again later.",       
        _ => "We're not sure what went wrong there."
    });

    // The actual error message
    ctx.insert("message", &stat.reason().unwrap_or("Unknown error"));
    
    template.render("pages/error.html", &ctx)
        .unwrap_or_else(|e| format!("Error rendering error page: {}", e))
}