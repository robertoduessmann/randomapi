use std::net::SocketAddr;

#[tokio::main]
async fn main() {
  let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
  random::restapihttpserver::httpserver(addr).await;
}