
use sapper::{
    App as SapperApp,
    Smock as SapperSmock,
    Result as SapperResult,
    Request,
    Response,
    Key
};
use sapper_std::WebContext;

use serde;
use serde_json;

#[macro_use] mod helper_macros;
mod db;
mod model;
mod dataservice;
mod util;


// include page modules
mod page;
//mod index_page;
//mod user_page;
//mod section_page;
//mod article_page;
//mod comment_page;


pub struct AppWebContext;
impl Key for AppWebContext { 
    type Value = WebContext;
}   



// define global smock
struct WebPage;

impl SapperSmock for WebPage {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        // define cookie prefix
        sapper_std::init(req, Some("forustm_session"))?;
        // init web instance state
        let mut web = WebContext::new();
        req.ext_mut().insert::<AppWebContext>(web);

        Ok(())
    }

    fn after(&self, req: &Request, res: &mut Response) -> SapperResult<()> {
        sapper_std::finish(req, res)?;
        Ok(())
    }
}

fn main () {
    let addr = "0.0.0.0";
    let port = 8081;
    let mut app = SapperApp::new();
    app.address(addr)
        .port(port)
        .with_smock(Box::new(WebPage))
        .add_module(Box::new(page::index_page::IndexPage))
        .add_module(Box::new(page::user_page::UserPage))
        .add_module(Box::new(page::section_page::SectionPage))
        .add_module(Box::new(page::article_page::ArticlePage))
        .add_module(Box::new(page::comment_page::CommentPage))
        .static_file_service(true);

    println!("Start listen on http://{}:{}", addr, port);
    app.run_http();

}
