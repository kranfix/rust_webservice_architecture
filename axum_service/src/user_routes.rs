use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use domain::UserRepo;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[macro_export]
macro_rules! create_user_routes {
  ($UR:ident, $user_repo:expr) => {{
    use axum::routing::get;
    use axum::Router;
    use axum_service::user_routes::{create_user, get_user_by_id, get_users};
    Router::new()
      .route("/users", get(get_users::<$UR>).post(create_user::<$UR>))
      .route("/users/:id", get(get_user_by_id::<$UR>))
      .with_state($user_repo)
  }};
}

pub async fn create_user<UR: UserRepo>(
  State(state): State<Arc<Mutex<UR>>>,
  Json(payload): Json<CreateUserPayload>,
) -> Json<UserReply> {
  let mut user_repo = state.lock().await;
  let created_user = user_repo.create_user(payload.username).await;
  Json(created_user.into())
}

pub async fn get_users<UR: UserRepo>(State(state): State<Arc<Mutex<UR>>>) -> Json<Vec<UserReply>> {
  let state = state.lock().await;
  let users = state.get_users().await;
  Json(
    users
      .iter()
      .map(|u| UserReply::from(u.clone()))
      .collect::<Vec<_>>(),
  )
}

pub async fn get_user_by_id<UR: UserRepo>(
  State(user_repo): State<Arc<Mutex<UR>>>,
  Path(id): Path<u32>,
) -> impl IntoResponse {
  let user_repo = user_repo.lock().await;
  match user_repo.get_user_by_id(id.into()).await {
    Some(user) => (StatusCode::OK, Json(Some(UserReply::from(user)))),
    None => (StatusCode::NOT_FOUND, Json(None)),
  }
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUserPayload {
  username: String,
}

// the output to our `create_user` handler
#[derive(Serialize, Clone)]
pub struct UserReply {
  id: u32,
  username: String,
}

impl<U: domain::User> From<U> for UserReply {
  fn from(value: U) -> Self {
    UserReply {
      id: value.id().into(),
      username: value.name(),
    }
  }
}
