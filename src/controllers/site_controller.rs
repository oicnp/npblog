use sapper::{SapperModule, SapperRouter, Response, Request, Result as SapperResult};
use sapper_std::{render};
//use super::super::models::{Article};
use super::super::utils::{Permissions, WebContext};

pub struct SiteController;


impl SiteController {
    fn index(req: &mut Request) -> SapperResult<Response> {
        let web = req.ext().get::<WebContext>().unwrap().clone();

        res_html!("site/index.html", web)
    }

    fn login(req: &mut Request) -> SapperResult<Response> {
        let permission = req.ext().get::<Permissions>().unwrap().to_owned();
        let web = req.ext().get::<WebContext>().unwrap().clone();
        match permission {
            Some(_) => {
                res_redirect!("/home")
            },
            None => {
                res_html!("site/login.html", web)
            }
        }
    }
}

impl SapperModule for SiteController {

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        
        router.get("/", SiteController::index);
        router.get("/login", SiteController::login);
        Ok(())
    }
}
