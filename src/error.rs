use std::error::Error;

use rocket::{http::Status, response::status};

/// Maps `tera::Error` to a `status::Custom` with a 500 status code
pub fn tera_err_to_status_500(e: tera::Error) -> rocket::response::status::Custom<&'static str> {
    println!("{}", tera_err_display_verbose(e));
    rocket::response::status::Custom(rocket::http::Status::InternalServerError, "Template error")
}

/// Display a `tera::Error` in a more verbose manner
pub fn tera_err_display_verbose(e: tera::Error) -> String {
    format!("Template error: {e}\n  {}", 
        e.source()
            .map(|e| e.to_string())
            .unwrap_or_else(|| "Unknown source".to_string())
    )
}

/// Maps `io::Error` to `status::Custom`
pub fn file_read_err_to_status(err: std::io::Error) -> status::Custom<&'static str> {
    match err.kind() {
        std::io::ErrorKind::NotFound 
            => status::Custom(Status::NotFound, "File not found"),
        std::io::ErrorKind::PermissionDenied 
            => status::Custom(Status::Forbidden, "Permission denied"),
        _ => {
            // Log the error to the console
            eprintln!("Error: {:?}", err);
            // Return a generic error message
            status::Custom(Status::InternalServerError, "Unknown file error")
        }
    }
}