mod res;
mod zine;

mod prelude {
    pub use rocket::{
        get, 
        http::{
            ContentType, 
            Status
        },
        response::{
            status,
            Redirect
        }
    };
}

use crate::error::{tera_err_display_verbose, tera_err_to_status_500};
use crate::prelude::*;
use crate::components;

use prelude::*;

/// Endpoint for the index page
/// ## Errors
/// - If a template fails to render, a status code 500 is returned
#[get("/")]
fn index() -> Result<(ContentType, String), status::Custom<&'static str>> {
    let mut ctx = TeraContext::new();

    ctx.insert("time", &chrono::Local::now().time().format("%H:%M").to_string());
    
    // Insert the navbar template
    ctx.insert("navbar", &components::navbar()
        .unwrap_or_else(tera_err_display_verbose));
    
    let rendered = template.render("index.html", &ctx)
        .map_err(tera_err_to_status_500)?;
    
    Ok((ContentType::HTML, rendered))   
}

fn about() -> Result<(ContentType, String), status::Custom<&'static str>> {
    let mut ctx = TeraContext::new();

    // Insert the navbar template
    ctx.insert("navbar", &components::navbar()
        .unwrap_or_else(tera_err_display_verbose));
    
    let rendered = template.render("about.html", &ctx)
        .map_err(tera_err_to_status_500)?;
    
    Ok((ContentType::HTML, rendered))   
}

fn contact() -> Result<(ContentType, String), status::Custom<&'static str>> {
    let mut ctx = TeraContext::new();

    // Insert the navbar template
    ctx.insert("navbar", &components::navbar()
        .unwrap_or_else(tera_err_display_verbose));
    
    let rendered = template.render("contact.html", &ctx)
        .map_err(tera_err_to_status_500)?;
    
    Ok((ContentType::HTML, rendered))   
}


/// Endpoint for the favicon
#[get("/favicon.ico")]
fn favicon() -> Redirect {
    Redirect::to("/res/media/favicon.ico")
}

/// Router for the root path
pub fn router() -> Router {
    Router::new("/", routes![
        index,
        favicon
    ])
    .router(res::router())
    .router(zine::router())
}