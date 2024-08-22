#[macro_use] extern crate rocket;

use rocket::{
    http::{ContentType, Status}, 
    response::status
};

use solrunners_site::{
    prelude::*, resource, templ_err_to_status, views
};

#[get("/")]
fn index() -> Result<(ContentType, String), status::Custom<&'static str>> {
    let mut ctx = TeraContext::new();

    ctx.insert("time", &chrono::Local::now().time().format("%H:%M").to_string());
    
    ctx.insert("navbar", &views::navbar().map_err(templ_err_to_status)?);
    
    let rendered = templ.render("index.html", &ctx)
        .map_err(templ_err_to_status)?;
    
    Ok((ContentType::HTML, rendered))
    
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            index,        
        ])
        .mount("/res", resource::routes())
}