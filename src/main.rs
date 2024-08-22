//! Entry point of the application

#[macro_use] extern crate rocket;

use solrunners_site::prelude::*;
use solrunners_site::routes;


#[launch]
fn rocket() -> _ {
    rocket::build()
        // PathBuf joins two "/" into "//". This would work fine but is a bit ugly.
        // Instead, we use the hacky solution of mouting the router to "".
        .mount_router("", routes::router())
}