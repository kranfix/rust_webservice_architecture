use axum::{routing::get, Router};
use axum_service::{root, run_server};
use std::sync::Arc;
use surrealdb_reqwest_adapter::{Auth, SurrealReqwest};

#[tokio::main]
async fn main() {
  env_logger::init();

  let auth = Auth::new("my_user", "my_pass");
  let surreal_client = SurrealReqwest::new("my_ns", "my_db", "http://localhost:8000", auth);
  let user_repo = Arc::new(surreal_client);
  let user_routes = axum_service::create_user_routes!(user_repo);

  // build our application with a route
  let app = Router::new() //
    .route("/", get(root))
    .nest("/users", user_routes);

  run_server(app).await
}
