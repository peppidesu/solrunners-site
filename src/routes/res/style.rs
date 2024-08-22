use crate::prelude::*;
use crate::routes::prelude::*;
use crate::error::file_read_err_to_status;

use std::path::PathBuf;
use rocket::fs::NamedFile;

/// Endpoint for loading stylesheets
#[get("/style/<path..>")]
pub async fn style(path: PathBuf) -> Result<NamedFile, status::Custom<&'static str>> {        
    // Check if the path is a CSS or a WOFF2 file
    if path.extension().map_or(false, |ext| ["css", "woff2"].iter().all(|e| ext != *e)) {        
        return Err(status::Custom(Status::BadRequest, "Must be a CSS file"));        
    }

    // Path is in <project_root>/res/style/<path>
    let path = PathBuf::new()
        .join(RES_PATH)        
        .join("style")
        .join(path);

    // Read the file to a string
    let content = NamedFile::open(path)
        .await
        .map_err(file_read_err_to_status)?;

    Ok(content)
}