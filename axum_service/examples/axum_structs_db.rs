use axum::{routing::get, Router};
use axum_service::{root, run_server};
use std::sync::Arc;
use structs_db::UserDB;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
  // initialize tracing
  let user_repo = Arc::new(Mutex::new(UserDB::default()));
  let user_routes = axum_service::create_user_routes!(user_repo);

  // build our application with a route
  let app = Router::new() //
    .route("/", get(root))
    .nest("/users", user_routes);

  run_server(app).await
}
