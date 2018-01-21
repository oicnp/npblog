
use sapper::{Key};
use sapper_std::{Context};

pub struct Permissions;

impl Key for Permissions {
    type Value = Option<i16>;
}

pub struct WebContext;

impl Key for WebContext {
    type Value = Context;
}