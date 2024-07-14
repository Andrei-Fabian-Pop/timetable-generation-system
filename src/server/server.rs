use std::collections::HashMap;
use std::net::SocketAddr;
use hyper::{Method, Request, Response, StatusCode};

use std::convert::Infallible;
use std::fs;
use http_body_util::combinators::BoxBody;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use url::form_urlencoded::Parse;
use url::Url;
use regex::Regex;
use crate::server::macros::empty_box_body;
use crate::server::ui::page_controller::dashboard_controller::DashboardPageController;
use crate::server::ui::page_controller::landing_page_controller::LandingPageController;
use crate::server::ui::page_controller::page_controller_traits::{PageControllerGet, PageControllerPost};
use crate::server::ui::page_controller::signup_page_controller::{SignupPageController};

// TODO: do we still need ext?
// ???
// use crate::ext::support::{TokioExecutor, TokioTimer};

pub struct HttpServer {
    routes: HashMap<String, Route>,
}

struct Route {
    pattern: Regex,
}

impl HttpServer {
    pub fn new() -> Self {
        HttpServer {
            routes: HashMap::new()
        }
    }

    async fn route(request: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error> {
        println!("=== = = = {}", request.method());
        println!("{}", request.uri());

        // TODO: find a better way
        let uri_prefix: &str = "http://127.0.0.1:3000";

        // TODO: add proper validation
        let url_err = Url::parse(format!("{}{}", uri_prefix, request.uri()).as_str());
        let url: Url;
        match url_err {
            Ok(x) => { url = x; }
            // TODO: make macro with return not found :))
            Err(_) => {
                return Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(empty_box_body!())
                    .unwrap());
                // return Ok(Response::new(Full::new(Bytes::from(x.to_string()))));
            }
        };

        // let path_prefix: &str = "src/server/html/";

        // TODO: add that favicon.png thing
        // TODO: split in multiple functions
        // TODO: use traits to share behavior
        //
        // if request.uri() == "/favicon.ico" {
        //     // return Ok(Response::new(Full::new(Bytes::from("Not ok"))));
        //     Ok(Response::builder()
        //         .status(StatusCode::NOT_FOUND)
        //         .body(empty())
        //         .unwrap())
        //     // return Ok(Response::new(Full::new(Bytes::from(fs::read_to_string("./src/server/html/favicon.ico").unwrap()))));
        // }

        if request.method() == Method::GET {
            match request.uri().to_string().as_str() {
                "/" => { return LandingPageController::get_request() }
                "/signup.html" => {
                    // Some(Box::new(SignupPageController::new()))
                    return SignupPageController::get_request();
                }
                "/dashboard.html" => {
                    return DashboardPageController::get_request();
                }
                _ => {
                    // TODO: return empty page with macro
                }
            };

            // let controller = controller.unwrap();
            // let request = controller.get_request().unwrap();

            // TODO: proper err handling
            // Ok(Response::new(Full::new(request)))
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(empty_box_body!())
                .unwrap())
        } else if request.method() == Method::POST {
            // let controller: Option<Box<dyn PageControllerMethods>> = match request.uri().to_string().as_str() {
            //     "/" => { Some(Box::new(LandingPageController::new(request))) }
                // "/signup.html" => { Some(Box::new(SignupPageController::new())) }
                // _ => { None }
            // };

            // let controller = controller.unwrap();

            // let query_params: Parse = url.query_pairs();
            // let mut params: HashMap<String, String> = HashMap::new();
            // for (key, value) in query_params {
            //     println!("{} => {}", key, value);
            //     params.insert(key.to_string(), value.to_string());
            // }

            // let request = controller.post_request(request).unwrap();
            println!("{}", request.uri().path());

            if request.uri().path() == "/login" {
                return SignupPageController::post_request(request).await;
            } else {
                Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(empty_box_body!())
                    .unwrap())
            }

            // Ok(Response::new(Full::new(Bytes::from(res))))
        } else {
            // Ok(Response::new(Full::new(Bytes::from("Err"))))
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(empty_box_body!())
                .unwrap())
        }
    }

    pub async fn run(&mut self) -> std::io::Result<()> {
        let address: SocketAddr = ([127, 0, 0, 1], 3000).into();
        let listener: TcpListener = TcpListener::bind(address).await?;

        println!("Listening on http://{}", address);

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);

            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(HttpServer::route))
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }
}

