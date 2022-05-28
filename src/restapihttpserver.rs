use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use std::net::SocketAddr;
use super::random;

async fn getrandomwebservice(_req: Request<Body>) 
    -> Result<Response<Body>, hyper::Error> {
  Ok(Response::new(Body::from(random::get_random_number())))
}

// statusnotfoundwebservice wraps "404 not found" page as http response
//  statusnotfoundwebservice is async (runs as non-blocking task) and private
async fn statusnotfoundwebservice(_req: Request<Body>) 
    -> Result<Response<Body>, hyper::Error> {
  Ok(Response::builder()
    .status(StatusCode::NOT_FOUND)
    .body(Body::from(String::from("404 Not Found")))
    .unwrap())
}

// webservicerouter
async fn webservicerouter(_req: Request<Body>) 
    -> Result<Response<Body>, hyper::Error> {
  match (_req.method(), _req.uri().path()) {
    (&Method::GET, "/api/v1/random") => getrandomwebservice(_req).await,
    _ => statusnotfoundwebservice(_req).await
  }  
}

pub async fn httpserver(addr: SocketAddr) {
  let server_future = Server::bind(&addr)
    .serve(make_service_fn(|_| async {
      Ok::<_, hyper::Error>(service_fn(webservicerouter))
    }));
  
  println!("random webserver is running");
  let r = server_future.await;
  if r.is_err() {
    eprintln!("random webserver error: {}", r.err().unwrap());
  }
}