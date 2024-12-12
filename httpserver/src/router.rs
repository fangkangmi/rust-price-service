use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            // if the method is GET, then call the appropriate handler
            // Only GET method is implemented
            httprequest::Method::GET => match &req.resource {
                // if the resource is a path, then call the appropriate handler
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split('/').collect();
                    match route[1] {
                        // after localhost:3000/, if it is api, then call WebServiceHandler
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
                //To learn: Why I need this???
                httprequest::Resource::Uninitialized => todo!(),
            },
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}