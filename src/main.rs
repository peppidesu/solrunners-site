//! Entry point of the application

#[macro_use] extern crate rocket;

use solrunners_site::prelude::*;
use solrunners_site::routes;


/// Builds the Rocket application
#[launch]
fn rocket() -> _ {
    rocket::build()
        // Mount the main router
        .mount_router("/", routes::router())
        // Mount the error catcher
        .register("/", catchers![
            routes::error::default_catcher
        ])
}