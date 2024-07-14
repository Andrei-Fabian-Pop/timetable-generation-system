use std::convert::Infallible;
use std::fs;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{Error, Response, StatusCode};
use crate::server::macros::full_box_body;
use crate::server::ui::page_controller::page_controller_traits::PageControllerGet;

pub struct DashboardPageController {}

impl DashboardPageController {
    const HTML_FILE_PATH: &'static str = "./src/server/html/dashboard.html";

}

impl PageControllerGet for DashboardPageController {
    fn get_request() -> Result<Response<BoxBody<Bytes, Infallible>>, Error> {
        println!("Searched: {}", Self::HTML_FILE_PATH);
        let html_content = fs::read_to_string(Self::HTML_FILE_PATH).expect("Could not read the html file!");
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(full_box_body!(html_content))
            .unwrap())
    }
}
