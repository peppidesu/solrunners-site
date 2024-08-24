use std::borrow::Cow;

use rocket::{fs::NamedFile, http::{Header, Status}, response::status, tokio::io::AsyncReadExt as _};
use rocket_etag_if_none_match::{
    entity_tag::EntityTag, EtagIfNoneMatch
};

use crate::error::file_read_err_to_status;
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
    pub async fn new<'a>(mut inner: NamedFile, etag_if_none_match: EtagIfNoneMatch<'a>) 
        -> Result<Self, status::Custom<&'static str>> {        

        let etag = EntityTag::from_file_meta(&inner.metadata().await.unwrap());
        
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

