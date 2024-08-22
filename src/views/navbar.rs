use crate::prelude::*;

pub fn navbar() -> TeraResult<String> {
    let ctx = TeraContext::new();     
    templ.render("navbar.html", &ctx)
}