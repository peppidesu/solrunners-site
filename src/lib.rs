#[macro_use] extern crate rocket;

use std::error::Error;

use lazy_static::lazy_static;
use tera::Tera;

pub mod resource;
pub mod views;

pub static RESOURCE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/res");

lazy_static! {
    pub static ref templ: Tera = {
        let mut tera = match Tera::new(
            &format!("{}/templates/**/*.html", RESOURCE_PATH)
        ) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html"]);                
        tera
    };
}

pub fn templ_err_to_status(e: tera::Error) -> rocket::response::status::Custom<&'static str> {
    println!("Template error: {e}\n  {}", 
        e.source()
            .map(|e| e.to_string())
            .unwrap_or_else(|| "Unknown source".to_string())
    );
    rocket::response::status::Custom(rocket::http::Status::InternalServerError, "Template error")
}

pub mod prelude {
    pub use crate::{
        templ,
        RESOURCE_PATH
    };
    pub use tera::{
        Tera,
        Context as TeraContext,
        Result as TeraResult
    };        
}

