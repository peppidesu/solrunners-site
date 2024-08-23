/// This module contains the routes for the application
pub mod error;
mod res;

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


#[get("/")]
fn index() -> Result<(ContentType, String), status::Custom<&'static str>> {
    let mut ctx = TeraContext::new();
    ctx.insert("time", &chrono::Local::now().format("%H:%M:%S").to_string());
    let content = template.render("pages/home.html", &ctx)
        .handle_tera_error()?;
        
    let content = components::page_base::render("home", &content)
        .handle_tera_error()?;

    Ok((ContentType::HTML, content))
}

#[get("/now")]
fn now() -> Result<(ContentType, String), status::Custom<&'static str>> {
    let ctx = TeraContext::new();
    
    let content = template.render("pages/now.html", &ctx)
        .handle_tera_error()?;
        
    let content = components::page_base::render("now", &content)
        .handle_tera_error()?;

    Ok((ContentType::HTML, content))
}

#[get("/about")]
fn about() -> Result<(ContentType, String), status::Custom<&'static str>> {
    let ctx = TeraContext::new();
    
    let content = template.render("pages/about.html", &ctx)
        .handle_tera_error()?;
        
    let content = components::page_base::render("about", &content)
        .handle_tera_error()?;

    Ok((ContentType::HTML, content))
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
        now,
        about,
        favicon
    ])
    .router(res::router())    
}