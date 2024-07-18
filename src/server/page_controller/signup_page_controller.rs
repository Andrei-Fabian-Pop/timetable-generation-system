use std::collections::HashMap;
use std::convert::Infallible;
use std::fs;

use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use http_body_util::combinators::BoxBody;
use hyper::{Method, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use url::form_urlencoded;
use crate::app_config::{AppConfig, DatabaseConfig};
use crate::db::src::database::Database;
use crate::server::macros::{empty_box_body, full_box_body, load_config};
use crate::server::page_controller::page_controller_traits::{PageControllerGet, PageControllerPost};

pub struct SignupPageController {}

static INDEX: &[u8] = b"<html><body><form action=\"post\" method=\"post\">Name: <input type=\"text\" name=\"name\"><br>Number: <input type=\"text\" name=\"number\"><br><input type=\"submit\"></body></html>";
static MISSING: &[u8] = b"Missing field";
static NOTNUMERIC: &[u8] = b"Number field is not numeric";

impl SignupPageController {
    const HTML_FILE_PATH: &'static str = "./src/server/html/signup.html";

    async fn param_example(&self, req: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error> {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/") | (&Method::GET, "/post") => Ok(Response::new(full_box_body!(INDEX))),
            (&Method::POST, "/post") => {
                let b: Bytes = req.collect().await?.to_bytes();
                let params: HashMap<String, String> = form_urlencoded::parse(b.as_ref())
                    .into_owned()
                    .collect::<HashMap<String, String>>();

                let name: &String = params.get("name").ok_or_else(|| {
                    Response::builder()
                        .status(StatusCode::UNPROCESSABLE_ENTITY)
                        .body::<BoxBody<Bytes, Infallible>>(full_box_body!(MISSING))
                        .unwrap()
                }).unwrap();
                let number: f64 = params.get("number").and_then(|n| n.parse::<f64>().ok()).ok_or_else(|| {
                    Response::builder()
                        .status(StatusCode::UNPROCESSABLE_ENTITY)
                        .body::<BoxBody<Bytes, Infallible>>(full_box_body!(NOTNUMERIC))
                        .unwrap()
                }).unwrap();

                let body = format!("Hello {}, your number is {}", name, number);
                Ok(Response::new(full_box_body!(body)))
            }
            (&Method::GET, "/get") => {
                let query: &str = req.uri().query().ok_or_else(|| {
                    Response::builder()
                        .status(StatusCode::UNPROCESSABLE_ENTITY)
                        .body::<BoxBody<Bytes, Infallible>>(full_box_body!(MISSING))
                        .unwrap()
                }).unwrap();
                let params: HashMap<String, String> = form_urlencoded::parse(query.as_bytes())
                    .into_owned()
                    .collect::<HashMap<String, String>>();
                let page: &String = params.get("page").ok_or_else(|| {
                    Response::builder()
                        .status(StatusCode::UNPROCESSABLE_ENTITY)
                        .body::<BoxBody<Bytes, Infallible>>(full_box_body!(MISSING))
                        .unwrap()
                }).unwrap();
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
        let html_content = fs::read_to_string(Self::HTML_FILE_PATH).expect("Error reading the file");
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(full_box_body!(html_content))
            .unwrap())
    }
}

impl PageControllerPost for SignupPageController {
    async fn post_request(request: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error> {
        let settings: AppConfig = load_config!(AppConfig);

        let app_config: AppConfig = settings.try_into().unwrap();
        let db_config: DatabaseConfig = app_config.database;

        let db: Database = Database::new(&format!(
            "host={} user={} dbname={} password={}",
            db_config.host, db_config.user, db_config.dbname, db_config.password
        )).await.expect("Failed to connect to the database");

        let b: Bytes = request.collect().await?.to_bytes();
        let params: HashMap<String, String> = form_urlencoded::parse(b.as_ref())
            .into_owned()
            .collect::<HashMap<String, String>>();

        #[derive(Serialize, Deserialize)]
        struct UserCredentials {
            username: String,
            password: String,
        }

        let uc = UserCredentials {
            username: params["username"].to_string(),
            password: params["password"].to_string(),
        };

        let valid: bool = db.check_credentials(&uc.username, &uc.password).await.map_err(|e| {
            println!("Failed to check credentials: {}", e);
        }).unwrap_or(false);

        if valid {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(full_box_body!(serde_json::to_string(&json!({
                    "status": "success",
                    "message": "Login successful"
                })).expect("Serialization error")))
                .unwrap())
        } else {
            Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(full_box_body!(serde_json::to_string(&json!({
                    "status": "error",
                    "message": "Invalid credentials"
                })).expect("Serialization error")))
                .unwrap())
        }
    }
}
