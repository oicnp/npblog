
extern crate sapper;
extern crate sapper_std;
extern crate npblog;

use sapper::{SapperApp, SapperAppShell, Request, Response, Result as SapperResult};
use self::npblog::controllers::{SiteController};
use self::npblog::utils::{ WebContext, get_identity_and_web_context};


struct BlogApp;

impl SapperAppShell for BlogApp {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        sapper_std::init(req, Some("npblog_session"))?;
        let (_identity, web) = get_identity_and_web_context(req);
        // req.ext_mut().insert::<Permissions>(identity);
        req.ext_mut().insert::<WebContext>(web);
        Ok(())
    }

    fn after(&self, req: &Request, res: &mut Response) -> SapperResult<()> {
        sapper_std::finish(req, res)?;
        Ok(())
    }
}

fn main() {
    let mut app = SapperApp::new();
    let port = 8081;

    app.address("0.0.0.0")
        .port(port)
        .init_global(Box::new(move |_req: &mut Request| {
            // req.ext_mut().insert::<Redis>(redis_pool.clone());
            // req.ext_mut().insert::<Postgresql>(pg_pool.clone());
            Ok(())
        }))
        .with_shell(Box::new(BlogApp))
        .add_module(Box::new(SiteController));

    app.static_service(true)
        .not_found_page(sapper_std::render("site/404.html", sapper_std::Context::new() ));

    println!("Start listen on http://{}:{}", "0.0.0.0", port);
    app.run_http();
}
