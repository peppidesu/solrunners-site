use crate::prelude::*;
use crate::routes::prelude::*;

pub fn router() -> Router {
    Router::new("/page", routes![
        home,
        now,
        about
    ])
}

#[get("/")]
fn home() -> Result<String, status::Custom<&'static str>> {
    let mut ctx = TeraContext::new();
    ctx.insert("time", &chrono::Local::now().format("%H:%M:%S").to_string());
    
    let content = template.render("pages/home.html", &ctx)
        .handle_tera_error()?;
    Ok(content)
}

#[get("/now")]
fn now() -> Result<String, status::Custom<&'static str>> {
    let ctx = TeraContext::new();
    let content = template.render("pages/now.html", &ctx)
        .handle_tera_error()?;
    Ok(content)
}

#[get("/about")]
fn about() -> Result<String, status::Custom<&'static str>> {
    let ctx = TeraContext::new();
    let content = template.render("pages/about.html", &ctx)
        .handle_tera_error()?;
    Ok(content)
}


