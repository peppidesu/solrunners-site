//! # Routes
//! Contains all routes in the application.
//! 
//! ## Structure
//! Routes are organized into directories in a way that reflects the hierarchy of the routes in
//! the application. The `Router` struct is used to manage this hierarchy.
//! 
//! ## Example
//! A good example of how to define routes is the `res` module.

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
    pub use std::path::PathBuf;
    pub use crate::error::HandleTeraError;    
}

/// Error catchers for the application
pub mod error;

/// Routes in /res.
/// Serves static resources such as stylesheets, images, etc.
mod res;

/// Routes in /page.
/// Serves page content.
mod page;

use crate::prelude::*;
use crate::components;

use prelude::*;

/// Router for the root path
pub fn router() -> Router {
    Router::new("/", routes![        
        base_page,
        favicon
    ])
    .router(res::router())    
    .router(page::router())
}


/// ## Base page endpoint
/// This endpoint is used to render the base page template.
/// The base page will lazily load the content for the requested page, allowing for static
/// content to persist across pages.
#[get("/<path..>")]
fn base_page(path: PathBuf) -> Result<(ContentType, String), status::Custom<&'static str>> {    
    if path.starts_with("page") {
        return Ok(error::error_page(Status::NotFound));
    }
    let content = components::page_base::render(&path.to_string_lossy())
        .handle_tera_error()?;
    Ok((ContentType::HTML, content))
}

/// Endpoint for the favicon
#[get("/favicon.ico")]
fn favicon() -> Redirect {
    Redirect::to("/res/media/favicon.ico")
}