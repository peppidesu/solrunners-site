//! Entry point of the application

#[macro_use] extern crate rocket;

use solrunners_site::prelude::*;
use solrunners_site::routes;
use solrunners_site::scss::compile_all_scss;


/// Builds the Rocket application
#[launch]
fn rocket() -> _ {    
    compile_all_scss();
    rocket::build()        
        .mount_router("/", routes::router())        
        .register("/page", catchers![
            routes::error::page_html_catcher
        ])
        .register("/", catchers![
            routes::error::default_html_catcher
        ])
}