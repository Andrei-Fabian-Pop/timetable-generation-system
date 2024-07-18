use std::collections::HashMap;
use std::net::SocketAddr;
use hyper::{Method, Request, Response, StatusCode};
use std::convert::Infallible;
use http_body_util::combinators::BoxBody;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use regex::Regex;
use crate::app_config::{AppConfig, DatabaseConfig};
use crate::server::macros::{empty_box_body, load_config};
use crate::server::page_controller::landing_page_controller::LandingPageController;
use crate::server::page_controller::page_controller_traits::{PageControllerGet, PageControllerPost};
use crate::server::page_controller::signup_page_controller::SignupPageController;

pub struct HttpServer {
    routes: HashMap<String, Route>,
}

struct Route {
    pattern: Regex,
}

impl HttpServer {
    pub fn new() -> Self {
        HttpServer {
            routes: HashMap::new(),
        }
    }

    async fn route(request: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error> {
        println!("Request method: {}", request.method());
        println!("Request URI: {}", request.uri());

        // TODO: add the favicon.ico thing
        match request.method() {
            &Method::GET => {
                match request.uri().to_string().as_str() {
                    "/" => LandingPageController::get_request(),
                    "/signup.html" => SignupPageController::get_request(),
                    _ => Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(empty_box_body!())
                        .unwrap()),
                }
            }
            &Method::POST => {
                if request.uri().to_string().as_str() == "/signup.html" {
                    SignupPageController::post_request(request).await
                } else {
                    Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(empty_box_body!())
                        .unwrap())
                }
            }
            _ => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(empty_box_body!())
                .unwrap())
        }
    }

    pub async fn run(&mut self) -> std::io::Result<()> {
        let settings: AppConfig = load_config!(AppConfig);

        let app_config: AppConfig = settings.try_into().unwrap();

        let address: SocketAddr = (app_config.host_ip(), app_config.server.port).into();
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
