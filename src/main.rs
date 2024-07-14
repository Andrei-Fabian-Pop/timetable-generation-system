mod server;
mod ext;
mod db;

use crate::server::server::HttpServer;

#[tokio::main]
async fn main() {
    let mut server: HttpServer = HttpServer::new();
    server.run().await.expect("Server error");
}
