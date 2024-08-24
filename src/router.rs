//! Definitions for the Router struct.
use std::path::PathBuf;
use rocket::Rocket;

/// Router for recursively mounting routes.
/// See the `routes` module for more information on how to create routes.
/// ## Example
/// ```rust
/// use rocket::{get, routes};
/// use solrunners_site::prelude::*;
/// 
/// #[get("/route1")]
/// fn route1() -> &'static str {
///    "route1"
/// }
/// #[get("/route2")]
/// fn route2() -> &'static str {
///   "route2"
/// }
/// 
/// fn router() -> Router {
///     Router::new("/base", routes![
///         route1,
///         route2
///     ])
///     .router(Router::new("/sub", routes![
///         route1,
///         route2
///     ]))
/// }
/// ```
/// The above code will produce the following routes:
/// - `/base/route1` --> "route1"
/// - `/base/route2` --> "route2"
/// - `/base/sub/route1` --> "route1"
/// - `/base/sub/route2` --> "route2"
pub struct Router {   
    /// The base path to mount the routes to. 
    /// This path will be appended to the parent path if it exists. 
    base: &'static str,
    /// A list of routes to mount to the base path.
    routes: Vec<rocket::Route>,
    /// A list of routers to mount to the base path.
    routers: Vec<Router>
}

impl Router {
    /// Create a new router
    /// ## Arguments
    /// - `base` - The base path to mount the routes to. 
    ///    This path will be appended to the parent path if it exists.
    /// - `routes` - A list of routes to mount to the base path.
    pub fn new(base: &'static str, routes: Vec<rocket::Route>) -> Self {
        Self {
            base,
            routes,
            routers: Vec::new()
        }
    }

    /// Add a router to the current router.
    /// ## Arguments
    /// - `router` - The router to add.    
    pub fn router(mut self, router: Router) -> Self {
        self.routers.push(router);
        self
    }
}

/// Extension trait for mounting routers to a Rocket instance.
pub trait MountRouter {
    /// Mount `router` to the Rocket instance at `base`.
    fn mount_router(self, base: &str, router: Router) -> Self;
}

impl MountRouter for Rocket<rocket::Build> {
    fn mount_router(mut self, base: &str, router: Router) -> Self {
        let binding = PathBuf::from(base).join(router.base);
        let path = binding.to_str().unwrap();
    
        self = self.mount(path, router.routes);

        for r in router.routers {
            self = self.mount_router(path, r);
        }

        self
    }
}