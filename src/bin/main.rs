use std::net::SocketAddr;
use std::env;

#[tokio::main]
async fn main() {

  let port_key = "PORT";
  let default_port = 8080;

  let port = match env::var(port_key) {
    Ok(val) => match val.parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                default_port
            }
        },
        Err(_) => {
            default_port
        }
    };


  let addr = SocketAddr::from(([0, 0, 0, 0], port));
  random::restapihttpserver::httpserver(addr).await;
}