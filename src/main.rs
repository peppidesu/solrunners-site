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
        // Mount the main router
        .mount_router("/", routes::router())
        // Mount the error catcher
        .register("/", catchers![
            routes::error::default_catcher
        ])
}