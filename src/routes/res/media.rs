use crate::prelude::*;
use crate::routes::prelude::*;
use crate::error::file_read_err_to_status;

use rocket::fs::NamedFile;
use rocket_etag_if_none_match::EtagIfNoneMatch;

use super::caching::CachedFileResponder;

/// Endpoint for media files.
/// # Parameters
/// - `path`: The path to the media file, relative to the /res/media directory.
/// # Errors
/// - If the file type is not supported, a BadRequest status is returned.
/// - If the file cannot be read, an error status is returned. see `file_read_err_to_status`.
#[get("/media/<path..>")]
pub async fn media(path: PathBuf, etag_if_none_match: EtagIfNoneMatch<'_>) 
    -> Result<CachedFileResponder, status::Custom<&'static str>> {        
    // Check for supported file types
    // If the file type is not supported, return a BadRequest status
    if path.extension().map_or(false, |ext| {
        ["png", "jpg", "jpeg", "gif", "svg", "ico"].iter().all(|e| ext != *e)
    }) {
        return Err(status::Custom(Status::BadRequest, "Unsupported file type"));
    }

    // Path is in <project_root>/res/media/<path>
    let path = PathBuf::new()
        .join(RES_PATH)
        .join("media")
        .join(path);

    // Open the file asynchronously using `NamedFile`.
    // This will automatically set the correct content type
    let content = NamedFile::open(path).await
        .map_err(file_read_err_to_status)?;

    // If the content needs sanitization, do it here

    // Return the file in a `CachedFileResponder` to enable caching
    CachedFileResponder::new(content, etag_if_none_match).await        
}
