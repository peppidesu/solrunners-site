use crate::prelude::*;
use std::path::PathBuf;

use rocket::{
    fs::NamedFile, get, http::{ContentType, Status}, response::status
};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        style, 
        media
    ]
}

/// Endpoint for loading stylesheets
#[get("/style/<path..>")]
fn style(path: PathBuf) -> Result<(ContentType, String), status::Custom<&'static str>> {        
    // Check if the path is a CSS file
    if path.extension().map_or(false, |ext| ext != "css") {        
        return Err(status::Custom(Status::BadRequest, "Must be a CSS file"));        
    }

    // Path is in <project_root>/res/style/<path>
    let path = PathBuf::new()
        .join(RESOURCE_PATH)        
        .join("style")
        .join(path);

    // Read the file to a string
    let content = std::fs::read_to_string(path).map_err(map_file_read_err_to_status)?;

    Ok((ContentType::CSS, content))
}

#[get("/media/<path..>")]
async fn media(path: PathBuf) -> Result<NamedFile, status::Custom<&'static str>> {        
    
    // Check for supported file types
    // If the file type is not supported, return a BadRequest status
    if path.extension().map_or(false, |ext| {
        ["png", "jpg", "jpeg", "gif", "svg", "ico"].iter().all(|e| ext != *e)
    }) {
        return Err(status::Custom(Status::BadRequest, "Unsupported file type"));
    }

    // Path is in <project_root>/res/media/<path>
    let path = PathBuf::new()
        .join(RESOURCE_PATH)
        .join("media")
        .join(path);

    // Open the file asynchronously using `NamedFile`.
    // This will automatically set the correct content type
    let content = NamedFile::open(path)
        .await
        .map_err(map_file_read_err_to_status)?;

    // If the content needs sanitization, do it here

    Ok(content)
}

/// Helper function to map io::Error to status::Custom
fn map_file_read_err_to_status(e: std::io::Error) -> status::Custom<&'static str> {
    match e.kind() {
        std::io::ErrorKind::NotFound 
            => status::Custom(Status::NotFound, "File not found"),
        std::io::ErrorKind::PermissionDenied 
            => status::Custom(Status::Forbidden, "Permission denied"),
        _ => {
            // Log the error to the console
            eprintln!("Error: {:?}", e);
            // Return a generic error message
            status::Custom(Status::InternalServerError, "Unknown file error")
        }
    }
}