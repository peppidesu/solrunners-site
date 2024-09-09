//! Entry point of the application

#[macro_use] extern crate rocket;

use std::sync::Arc;

use dotenvy::dotenv;
use solrunners_site::prelude::*;
use solrunners_site::routes;
use solrunners_site::scss::compile_all_scss;

/// Builds the Rocket application
#[launch]
fn rocket() -> _ {    
    dotenv().expect("Failed to load .env file");
    compile_all_scss();

    let db_url = std::env::var("COUCHDB_URL").expect("COUCHDB_URL not set");
    let db_username = std::env::var("COUCHDB_USER").expect("COUCHDB_USER not set");
    let db_password = std::env::var("COUCHDB_PASSWORD").expect("COUCHDB_PASSWORD not set");

    let client = couch_rs::Client::new(&db_url, &db_username, &db_password)
        .expect("Failed to create CouchDB client");    

    let db = DbClient {
        client: Arc::new(client)
    };

    rocket::build()        
        .mount_router("/", routes::router())        
        .register("/page", catchers![
            routes::error::page_html_catcher
        ])
        .register("/", catchers![
            routes::error::default_html_catcher
        ])
        .manage(db)
}