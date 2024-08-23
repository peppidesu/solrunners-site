use crate::prelude::*;

use rocket::{fs::NamedFile, http::Header};

mod media;
mod style;

#[derive(Responder)]
#[response()]
struct CachedFileResponder {
    inner: NamedFile,    
    last_modified: Header<'static>
}

impl CachedFileResponder {
    pub async fn new(inner: NamedFile) -> Result<Self, std::io::Error> {        
        let last_modified = inner.metadata().await?.modified().ok()
            .map(|t| format!("{}", chrono::DateTime::<chrono::Utc>::from(t).format("%a, %d %b %Y %H:%M:%S GMT")))
            .unwrap_or_else(|| "Thu, 01 Jan 1970 00:00:00 GMT".to_string());

        Ok(Self {
            inner, 
            last_modified: Header::new("Last-Modified", last_modified)
        })
    }
}

pub fn router() -> Router {
    Router::new("/res", routes![
        media::media,
        style::style
    ])
}