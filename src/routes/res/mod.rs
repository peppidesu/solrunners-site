use crate::prelude::*;

mod media;
mod style;


pub fn router() -> Router {
    Router::new("/res", routes![
        media::media,
        style::style
    ])
}