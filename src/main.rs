#[macro_use] extern crate rocket;

use rocket::{http::{ContentType, Status}, response::{status, stream::{Event, EventStream}}};
use tera::{Context, Tera};
use lazy_static::lazy_static;

use std::{path::PathBuf, vec};

static TEMPLATES_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/templates");

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new(
            &format!("{}/**/*", TEMPLATES_PATH)
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


#[get("/")]
fn index() -> Result<(ContentType, String), status::Custom<&'static str>> {
    let mut ctx = Context::new();
    ctx.insert("time", &chrono::Local::now().time().format("%H:%M").to_string());

    let rendered = TEMPLATES.render("index.html", &ctx)
        .map_err(|_| status::Custom(Status::InternalServerError, "Template error"))?;

    Ok((ContentType::HTML, rendered))
}

/// Allowed extensions for the resource endpoint
static RES_ALLOWED_EXTENSIONS: [&str; 6] = ["css", "png", "jpg", "jpeg", "svg", "webm"];

#[get("/res/<path..>")]
fn resource(path: PathBuf) -> Result<(ContentType, String), status::Custom<&'static str>> {        
    // Check if the path is a CSS file
    if path.extension().map_or(false, |ext| !RES_ALLOWED_EXTENSIONS.contains(&ext.to_str().unwrap())) {        
        return Err(status::Custom(Status::BadRequest, "Must be a CSS file"));        
    }

    let path = PathBuf::new()
        .join(TEMPLATES_PATH)
        .join("res")
        .join(path);

    let content = std::fs::read_to_string(path).map_err(|e| {
        match e.kind() {
            std::io::ErrorKind::NotFound 
                => status::Custom(Status::NotFound, "File not found"),
            std::io::ErrorKind::PermissionDenied 
                => status::Custom(Status::Forbidden, "Permission denied"),

            _ => {
                println!("Error: {:?}", e);
                status::Custom(Status::InternalServerError, "Unknown file error")
            }
        }
    })?;

    Ok((ContentType::CSS, content))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index,
        resource        
    ])
}