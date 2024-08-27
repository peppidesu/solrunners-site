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
        home,
        now,
        favicon
    ])
    .router(res::router())    
    .router(page::router())
}


/// Index page endpoint
#[get("/")]
fn home() -> Result<(ContentType, String), status::Custom<&'static str>> {
    base_page(PathBuf::from(""))
}

/// Now page endpoint.
#[get("/now")]
fn now() -> Result<(ContentType, String), status::Custom<&'static str>> {
    base_page(PathBuf::from("now"))
}

/// Returns a base page for the given path.
/// 
/// The page has JS that will lazily fetch content from `/page/{path}`
fn base_page(path: PathBuf) -> Result<(ContentType, String), status::Custom<&'static str>> {    
    let content = components::page_base::render(&path.to_string_lossy())
        .handle_tera_error()?;
    Ok((ContentType::HTML, content))
}


/// Endpoint for the favicon
#[get("/favicon.ico")]
fn favicon() -> Redirect {
    Redirect::to("/res/media/favicon.ico")
}