mod res;

use crate::error::{tera_err_display_verbose, tera_err_to_status_500};
use crate::prelude::*;
use crate::views;

use rocket::http::ContentType;
use rocket::response::status;

/// Endpoint for the index page
/// # Returns
/// 
/// # Errors
/// - If a template fails to render, a status code 500 is returned
#[get("/")]
fn index() -> Result<(ContentType, String), status::Custom<&'static str>> {
    let mut ctx = TeraContext::new();

    ctx.insert("time", &chrono::Local::now().time().format("%H:%M").to_string());
    
    // Insert the navbar template
    ctx.insert("navbar", &views::navbar()
        .unwrap_or_else(tera_err_display_verbose));
    
    let rendered = template.render("index.html", &ctx)
        .map_err(tera_err_to_status_500)?;
    
    Ok((ContentType::HTML, rendered))   
}

/// Router for the root path
pub fn router() -> Router {
    Router::new("/", routes![
        index
    ])
    .router(res::router())
}