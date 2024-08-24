use rocket::{fs::NamedFile, http::Header};

/// A responder that enables caching of files through the `Last-Modified` header.
/// At some point this should use `ETag` instead, but for now this is good enough.
#[derive(Responder)]
#[response()]
pub(super) struct CachedFileResponder {
    /// The file to cache
    inner: NamedFile,    
    /// The `Last-Modified` header
    last_modified: Header<'static>
}

impl CachedFileResponder {
    /// Create a new `CachedFileResponder` from a `NamedFile`.
    pub async fn new(inner: NamedFile) -> Result<Self, std::io::Error> {        
        // Get the last modified time of the file
        let modified = inner.metadata().await?
            .modified().ok()
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

        // Format to RFC 2822
        let timestamp = chrono::DateTime::<chrono::Utc>::from(modified)
            .format("%a, %d %b %Y %T GMT")
            .to_string();

        // Create the Last-Modified header
        let last_modified = Header::new("Last-Modified", timestamp);
        
        Ok(Self { inner, last_modified })
    }
}