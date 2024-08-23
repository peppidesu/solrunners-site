//! Entry point of the application

#[macro_use] extern crate rocket;

use solrunners_site::prelude::*;
use solrunners_site::routes;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount_router("/", routes::router())
        .register("/", catchers![
            routes::error::default_catcher
        ])
}