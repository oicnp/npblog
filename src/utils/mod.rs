
use sapper::{Key, Request};
use sapper_std::{Context};

pub struct Permissions;

impl Key for Permissions {
    type Value = Option<i16>;
}

pub struct WebContext;

impl Key for WebContext {
    type Value = Context;
}


/// Get visitor status and web context
pub fn get_identity_and_web_context(req: &Request) -> (Option<i16>, Context) {
    let web = Context::new();
    (None, web)
}