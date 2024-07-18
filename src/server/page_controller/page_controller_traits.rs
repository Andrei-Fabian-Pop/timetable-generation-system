use std::collections::HashMap;
use std::convert::Infallible;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{Request, Response};

pub trait PageControllerGet {
    fn get_request() -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error>;
}

pub trait PageControllerPost {
    async fn post_request(req: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error>;
}
