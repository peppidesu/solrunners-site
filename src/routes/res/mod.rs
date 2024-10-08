use crate::prelude::*;
/// /res/media endpoint
mod media;
/// /res/style endpoint
mod style;

/// The router for the /res endpoint.
pub fn router() -> Router {
    Router::new("/res", routes![
        media::media,
        style::style
    ])
}