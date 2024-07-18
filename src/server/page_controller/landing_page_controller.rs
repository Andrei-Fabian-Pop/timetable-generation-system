use std::convert::Infallible;
use std::fs;
use std::io::Error;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{Request, Response, StatusCode};

use crate::server::page_controller::page_controller_traits::{PageControllerGet, PageControllerPost};
use crate::server::macros::{empty_box_body, full_box_body};

pub struct LandingPageController;

impl LandingPageController {
    const HTML_FILE_PATH: &'static str = "./src/server/html/index.html";
}

impl PageControllerGet for LandingPageController {
    fn get_request() -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error> {
        let html_content = fs::read_to_string(Self::HTML_FILE_PATH)
            .map_err(|e| {
                eprintln!("Error reading HTML file: {}", e);
                Error::from(e)
            }).unwrap();

        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(full_box_body!(html_content))
            .unwrap())
    }
}

impl PageControllerPost for LandingPageController {
    async fn post_request(_: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error> {
        // For POST requests to the landing page, we're not handling any logic here. Might remove this altogether :P
        println!("Received POST request on landing page");

        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(empty_box_body!())
            .unwrap())
    }
}
