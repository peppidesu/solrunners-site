use crate::prelude::*;
use crate::routes::prelude::*;
use crate::error::file_read_err_to_status;

use std::path::PathBuf;
use rocket::fs::NamedFile;
use rocket_etag_if_none_match::EtagIfNoneMatch;

use super::caching::CachedFileResponder;

/// Endpoint for stylesheets and fonts.
/// # Parameters
/// - `path`: The path to the file, relative to the /res/style directory.
/// # Errors
/// - If the file type is not supported, a BadRequest status is returned.
/// - If the file cannot be read, an error status is returned. see `file_read_err_to_status`.
#[get("/style/<path..>")]
pub async fn style<'a>(path: PathBuf, etag_if_none_match: EtagIfNoneMatch<'a>) 
-> Result<CachedFileResponder, status::Custom<&'static str>> {
    // Check for supported file types
    // If the file type is not supported, return a BadRequest status
    if path.extension().map_or(false, |ext| ["css", "woff2"].iter().all(|e| ext != *e)) {        
        return Err(status::Custom(Status::BadRequest, "Unsupported file type"));        
    }

    // Path is in <project_root>/res/style/<path>
    let path = PathBuf::new()
        .join(RES_PATH)        
        .join("style")
        .join(path);

    // Open the file asynchronously using `NamedFile`.
    // This will automatically set the correct content type
    let content = NamedFile::open(path)
        .await
        .map_err(file_read_err_to_status)?;

    // Return the file as a `CachedFileResponder` to enable caching
    CachedFileResponder::new(content, etag_if_none_match).await        
}