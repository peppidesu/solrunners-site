use std::error::Error;

use rocket::{http::Status, response::status};

/// A trait for handling `tera::Error` and mapping it to a `status::Custom`
pub trait HandleTeraError<T> {
    /// Deals with a `tera::Error`, mapping it to a status 500 in production
    /// 
    /// ## Returns
    /// In production builds, returns:
    /// * `Ok` if the result is `Ok`
    /// * `Err` if the result is `Err`
    /// 
    /// In debug builds, returns:
    /// * `Ok` if the result is `Ok`
    /// * `Ok` with the error message if the result is `Err`
    /// 
    /// In both cases the error message is printed to the console.
    /// 
    /// ## Usage
    /// This trait is used to make templates that fail to render show their error in the browser
    /// when debugging. In production, the error is hidden and a status 500 is returned instead.
    /// 
    /// ## Errors
    /// Only errors in debug mode, when the result is `Err`
    fn handle_tera_error(self) -> Result<T, status::Custom<&'static str>>;
}

impl HandleTeraError<String> for Result<String, tera::Error> {
    fn handle_tera_error(self) -> Result<String, status::Custom<&'static str>> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                let msg = tera_err_display_verbose(e);
                eprintln!("{}", &msg);

                #[cfg(debug_assertions)] {
                    Ok(msg)
                }
                #[cfg(not(debug_assertions))] {
                    Err(status::Custom(Status::InternalServerError, "Template error"))
                }
            }
        }
    }
}

/// Display a `tera::Error` in a more verbose manner
fn tera_err_display_verbose(e: tera::Error) -> String {
    format!("Template error: {e}\n  {}", 
        e.source()
            .map(|e| e.to_string())
            .unwrap_or_else(|| "Unknown source".to_string())
    )
}

/// Maps `io::Error` to `status::Custom` depending on the error kind
/// ## Returns
/// * status 404 if the file is not found
/// * status 403 if we don't have permission to read the file
/// * status 500 for all other errors
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