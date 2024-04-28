use std::collections::HashMap;
use std::convert::Infallible;
use std::fs;

use bytes::Bytes;
use http_body_util::{BodyExt, Empty, Full};
use http_body_util::combinators::BoxBody;
use hyper::{Method, Request, Response, StatusCode};
use hyper::body::Body;
use serde::{Deserialize, Serialize};
use url::form_urlencoded;

use crate::server::macros::{empty_box_body, full_box_body};
use crate::server::ui::page_controller::page_controller_traits::{PageControllerGet, PageControllerPost};

pub struct SignupPageController {}

// TODO: change
static INDEX: &[u8] = b"<html><body><form action=\"post\" method=\"post\">Name: <input type=\"text\" name=\"name\"><br>Number: <input type=\"text\" name=\"number\"><br><input type=\"submit\"></body></html>";
static MISSING: &[u8] = b"Missing field";
static NOTNUMERIC: &[u8] = b"Number field is not numeric";

impl SignupPageController {
    const HTML_FILE_PATH: &'static str = "./src/server/html/signup.html";

    async fn param_example(&self, req: Request<hyper::body::Incoming>,
    ) -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error> {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/") | (&Method::GET, "/post") => Ok(Response::new(full_box_body!(INDEX))),
            (&Method::POST, "/post") => {
                // Concatenate the body...
                let b = req.collect().await?.to_bytes();
                // Parse the request body. form_urlencoded::parse
                // always succeeds, but in general parsing may
                // fail (for example, an invalid post of json), so
                // returning early with BadRequest may be
                // necessary.
                //
                // Warning: this is a simplified use case. In
                // principle names can appear multiple times in a
                // form, and the values should be rolled up into a
                // HashMap<String, Vec<String>>. However, in this
                // example the simpler approach is sufficient.
                let params = form_urlencoded::parse(b.as_ref())
                    .into_owned()
                    .collect::<HashMap<String, String>>();

                // Validate the request parameters, returning
                // early if an invalid input is detected.
                let name = if let Some(n) = params.get("name") {
                    n
                } else {
                    return Ok(Response::builder()
                        .status(StatusCode::UNPROCESSABLE_ENTITY)
                        .body(full_box_body!(MISSING))
                        .unwrap());
                };
                let number = if let Some(n) = params.get("number") {
                    if let Ok(v) = n.parse::<f64>() {
                        v
                    } else {
                        return Ok(Response::builder()
                            .status(StatusCode::UNPROCESSABLE_ENTITY)
                            .body(full_box_body!(NOTNUMERIC))
                            .unwrap());
                    }
                } else {
                    return Ok(Response::builder()
                        .status(StatusCode::UNPROCESSABLE_ENTITY)
                        .body(full_box_body!(MISSING))
                        .unwrap());
                };

                // Render the response. This will often involve
                // calls to a database or web service, which will
                // require creating a new stream for the response
                // body. Since those may fail, other error
                // responses such as InternalServiceError may be
                // needed here, too.
                let body = format!("Hello {}, your number is {}", name, number);
                Ok(Response::new(full_box_body!(body)))
            }
            (&Method::GET, "/get") => {
                let query = if let Some(q) = req.uri().query() {
                    q
                } else {
                    return Ok(Response::builder()
                        .status(StatusCode::UNPROCESSABLE_ENTITY)
                        .body(full_box_body!(MISSING))
                        .unwrap());
                };
                let params = form_urlencoded::parse(query.as_bytes())
                    .into_owned()
                    .collect::<HashMap<String, String>>();
                let page = if let Some(p) = params.get("page") {
                    p
                } else {
                    return Ok(Response::builder()
                        .status(StatusCode::UNPROCESSABLE_ENTITY)
                        .body(full_box_body!(MISSING))
                        .unwrap());
                };
                let body = format!("You requested {}", page);
                Ok(Response::new(full_box_body!(body)))
            }
            _ => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(empty_box_body!())
                .unwrap()),
        }
    }
}

impl PageControllerGet for SignupPageController {
    fn get_request() -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error> {
        // TODO: proper err handling
        println!("Searched: {}", Self::HTML_FILE_PATH);
        let html_content = fs::read_to_string(Self::HTML_FILE_PATH).expect("====================== err reading the file");
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(full_box_body!(html_content))
            .unwrap())
    }
}

impl PageControllerPost for SignupPageController {
    async fn post_request(request: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error>  {
        println!("=> POST REQUEST");

        let b = request.collect().await?.to_bytes();
        let params = form_urlencoded::parse(b.as_ref())
            .into_owned()
            .collect::<HashMap<String, String>>();

        #[derive(Serialize, Deserialize)]
        struct UserCredentials {
            username: String,
            password: String,
        }

        let up = UserCredentials {
            username: params["username"].to_string() + "back",
            password: params["password"].to_string() + "back",
        };

        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(full_box_body!(serde_json::to_string(&up).expect("Ups")))
            .unwrap())
    }
}
