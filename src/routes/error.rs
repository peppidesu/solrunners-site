use crate::{components, prelude::*};
use crate::routes::prelude::*;
use rocket::Request;

/// The default error catcher
#[catch(default)]
pub fn default_catcher(stat: Status, _: &Request) -> (ContentType, String) {
    let mut ctx = TeraContext::new();
    ctx.insert("code", &stat.code);
    
    ctx.insert("readable", match stat.code {
        401 => "You need to be logged in to view this page.",
        403 => "You don't have permission to view this page.",
        404 => "We couldn't find the page you were looking for.",        
        410 => "This page has been removed.",
        429 => "You're making too many requests. Please try again later.",
        400 => "Something was wrong with your request.",
        500..=599 => "Something went wrong on our end. Please try again later.",
        _ => "Unknown Error"
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

