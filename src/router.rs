use std::path::PathBuf;

use rocket::Rocket;

/// Router for recursively mounting routes.
/// ## Example
/// ```rust
/// use rocket::routes;
/// 
/// fn router() -> Router {
///     Router::new("/base", routes![
///         route1,
///         route2
///     ])
///     .router(Router::new("/sub", routes![
///         route3,
///         route4
///     ]));
/// }
/// ```
/// The above code will produce the following routes:
/// - `/base/route1`
/// - `/base/route2`
/// - `/base/sub/route3`
/// - `/base/sub/route4`
/// 
pub struct Router {    
    base: &'static str,
    routes: Vec<rocket::Route>,
    routers: Vec<Router>
}

impl Router {
    /// Create a new router
    /// ## Arguments
    /// - `base` - The base path to mount the routes to. This path will be appended to the parent path if it exists.
    /// - `routes` - A list of routes to mount to the base path.
    pub fn new(base: &'static str, routes: Vec<rocket::Route>) -> Self {
        Self {
            base,
            routes,
            routers: Vec::new()
        }
    }

    /// Add a router to the current router.
    pub fn router(mut self, router: Router) -> Self {
        self.routers.push(router);
        self
    }
}

/// Extension trait for mounting routers to a Rocket instance.
pub trait MountRouter {
    /// Mount a router to the Rocket instance.
    fn mount_router(self, base: &str, router: Router) -> Self;
}

impl MountRouter for Rocket<rocket::Build> {
    fn mount_router(mut self, base: &str, router: Router) -> Self {
        let binding = PathBuf::from(base).join(router.base);
        let path = binding.to_str().unwrap();
        for route in &router.routes {
            println!("Mounting route: {}", path.to_owned() + route.uri.as_str());
        }
        self = self.mount(path, router.routes);

        for r in router.routers {
            self = self.mount_router(path, r);
        }

        self
    }
}