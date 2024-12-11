use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;
    fn load_file(file_name: &str) -> Option<Vec<u8>> {
        // env! and env::var are used to get the value of the environment variable
        // the difference is that env! will panic if the environment variable is not set
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);
        let contents = fs::read_to_string(full_path).ok()?;
        Some(contents.into_bytes())
    }
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;

#[derive(Serialize, Deserialize)]
pub struct Myclaims {
    item: String,
    vat_incl: bool,
    quantity: i32,
}

impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource else {
            panic!("Resource not found");
        };
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            // after localhost:3000/, if it is health, then call health.html
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),

            // after localhost:3000/, if it is api, then call WebServiceHandler
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                    // return the contents of the file
                    HttpResponse::new("200", Some(map), Some(contents))
                }
                None => HttpResponse::new("404", None, Self::load_file("404.html")),
            },
        }
    }
}

impl WebServiceHandler {
    fn load_json() -> Vec<Myclaims> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "claims.json");
        println!("Loading JSON from: {}", full_path);
        let json_contents = fs::read_to_string(full_path);
        let claims: Vec<Myclaims> =
            serde_json::from_str(json_contents.unwrap().as_str()).unwrap();
        claims
    }
}

impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource else {
            panic!("Resource not found");
        };
        let route: Vec<&str> = s.split("/").collect();

        //localhost:3000/api/price
        match route[2] {
            "price" if route.len() >2 => {
                let body = serde_json::to_string(&Self::load_json()).unwrap().into_bytes();
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), Some(body))
            }
            _ => HttpResponse::new("404", None, Self::load_file("404.html")),
        }
    }
}