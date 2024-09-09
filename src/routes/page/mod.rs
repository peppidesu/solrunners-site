use rocket::futures::TryFutureExt;
use rocket::State;

use crate::prelude::*;
use crate::routes::prelude::*;

/// Router for the /page path
pub fn router() -> Router {
    Router::new("/page", routes![
        home,
        now        
    ])
}

/// Home page endpoint
#[get("/")]
async fn home(db: &State<DbClient>) -> Result<String, status::Custom<&'static str>> {
    let mut ctx = TeraContext::new();
    ctx.insert("time", &chrono::Local::now().format("%H:%M:%S").to_string());

    let test_data = db.client.list_dbs().await
        .map_err(|_| status::Custom(Status::InternalServerError, "Failed to list databases"))?;
    ctx.insert("test_data", &format!("{:?}", test_data));
    
    let content = template.render("pages/home.html", &ctx)
        .handle_tera_error()?;
    Ok(content)
}

/// Now page endpoint
#[get("/now")]
fn now() -> Result<String, status::Custom<&'static str>> {
    let ctx = TeraContext::new();
    let content = template.render("pages/now.html", &ctx)
        .handle_tera_error()?;
    Ok(content)
}

