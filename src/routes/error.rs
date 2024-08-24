use crate::{components, prelude::*};
use crate::routes::prelude::*;
use rocket::Request;

/// The default error catcher
#[catch(default)]
pub fn default_catcher(stat: Status, _: &Request) -> (ContentType, String) {
    let mut ctx = TeraContext::new();
    ctx.insert("code", &stat.code);
    
    ctx.insert("exclamation", match stat.code {
        401 => "Please log in.",
        403 => "Whoa there, buddy!",
        404 => "Welcome to white space.",
        410 => "Sorry!",
        429 => "Hey! Slow down!",
        400..=499 => "Oops!",
        500..=599 => "Well, this is embarrassing.",
        _ => "Whoops!"
    });

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

    ctx.insert("message", &stat.reason().unwrap_or("Unknown error"));

    let content = template.render("pages/error.html", &ctx)
        .unwrap_or_else(|e| format!("Error rendering error page: {}", e));
    
    (
        ContentType::HTML, 
        components::page_base::render("error", &content)
            .unwrap_or_else(|e| format!("Error rendering error page: {}", e))
    )
}