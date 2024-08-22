use rocket::{
    get, http::{ContentType, Status}, response::status
};
use std::path::PathBuf;

use crate::prelude::*;
use crate::error::file_read_err_to_status;

/// Endpoint for loading stylesheets
#[get("/style/<path..>")]
pub fn style(path: PathBuf) -> Result<(ContentType, String), status::Custom<&'static str>> {        
    // Check if the path is a CSS file
    if path.extension().map_or(false, |ext| ext != "css") {        
        return Err(status::Custom(Status::BadRequest, "Must be a CSS file"));        
    }

    // Path is in <project_root>/res/style/<path>
    let path = PathBuf::new()
        .join(RES_PATH)        
        .join("style")
        .join(path);

    // Read the file to a string
    let content = std::fs::read_to_string(path).map_err(file_read_err_to_status)?;

    Ok((ContentType::CSS, content))
}