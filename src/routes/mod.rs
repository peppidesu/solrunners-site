//! # Routes
//! Contains all routes in the application.
//! 
//! ## Structure
//! Routes are organized into directories in a way that reflects the hierarchy of the routes in
//! the application. The `Router` struct is used to manage this hierarchy.
//! 
//! ## Example
//! A good example of how to define routes is the `res` module.

/// Error catchers for the application
pub mod error;

/// Routes in /res.
/// Serves static resources such as stylesheets, images, etc.
mod res;

/// Prelude module for `routes`. Contains commonly used imports.
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

    pub use crate::error::HandleTeraError;    
}


use crate::prelude::*;
use crate::components;

use prelude::*;

/// Router for the root path
pub fn router() -> Router {
    Router::new("/", routes![
        index,
        now,
        about,
        favicon
    ])
    .router(res::router())    
}

/// Renders the contents of a page with the given context
fn render_page(page: &str, ctx: TeraContext) -> Result<(ContentType, String), status::Custom<&'static str>> {
    let content = template.render(page, &ctx)
        .handle_tera_error()?;
        
    let content = components::page_base::render(page, &content)
        .handle_tera_error()?;

    Ok((ContentType::HTML, content))
}

/// Index page
#[get("/")]
fn index() -> Result<(ContentType, String), status::Custom<&'static str>> {
    let mut ctx = TeraContext::new();
    // Used for current time example
    ctx.insert("time", &chrono::Local::now().format("%H:%M:%S").to_string());
    
    render_page("pages/home.html", ctx)
}

/// Now page
#[get("/now")]
fn now() -> Result<(ContentType, String), status::Custom<&'static str>> {
    let ctx = TeraContext::new();  
    render_page("pages/now.html", ctx)
}

/// About page
#[get("/about")]
fn about() -> Result<(ContentType, String), status::Custom<&'static str>> {
    let ctx = TeraContext::new();
    render_page("pages/about.html", ctx)
}

/// Endpoint for the favicon
#[get("/favicon.ico")]
fn favicon() -> Redirect {
    Redirect::to("/res/media/favicon.ico")
}