pub mod user_routes;

use std::net::SocketAddr;

use axum::Router;

pub async fn run_server(app: Router) {
  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  tracing::debug!("listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

// basic handler that responds with a static string
pub async fn root() -> &'static str {
  "Hello, World!"
}
