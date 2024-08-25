use crate::routes::prelude::*;
use crate::error::file_read_err_to_status;

use rocket::fs::NamedFile;
use rocket::http::Header;
use rocket_etag_if_none_match::{EtagIfNoneMatch, entity_tag::EntityTag};


/// A responder for a cached file.
/// 
#[derive(Responder)]
#[response()]
pub(super) struct CachedFileResponder {
    /// The file to cache
    inner: NamedFile,    
    /// Cache control header
    cache_control: Header<'static>,    
    /// Etag header
    etag: Header<'static>,
}

impl CachedFileResponder {
    /// Create a new `CachedFileResponder` from a `NamedFile`.
    pub async fn new(inner: NamedFile, etag_if_none_match: EtagIfNoneMatch<'_>) 
        -> Result<Self, status::Custom<&'static str>> {        

        let etag = EntityTag::from_file_meta(&inner.metadata().await    
            .map_err(file_read_err_to_status)?);
        
        if etag_if_none_match.weak_eq(&etag) {
            return Err(status::Custom(Status::NotModified, ""));
        }

        // Cache Control header
        let cache_control = Header::new("Cache-Control", "public, max-age=0, must-revalidate");
        // Create the etag header
        let etag_header = Header::new("ETag", etag.to_string());

        

        Ok(Self { inner, cache_control, etag: etag_header })
    }
}

