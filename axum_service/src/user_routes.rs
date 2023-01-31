use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use domain::UserRepo;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::reply::Reply;

#[macro_export]
macro_rules! create_user_routes {
  ($user_repo:expr) => {{
    use axum::routing::get;
    use axum::Router;
    use axum_service::user_routes::{create_user, get_user_by_id, get_users};
    Router::new()
      .route("/", get(get_users).post(create_user))
      .route("/:id", get(get_user_by_id))
      .with_state($user_repo)
  }};
}

pub async fn create_user<UR: UserRepo>(
  State(state): State<Arc<Mutex<UR>>>,
  Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
  let mut user_repo = state.lock().await;
  let created_user = user_repo.create_user(payload.username).await;
  match created_user {
    Ok(u) => {
      let user_reply: UserReply = u.into();
      (StatusCode::OK, Json(Reply::data(user_reply)))
    }
    Err(e) => {
      let status_code = match e {
        domain::CreateUserError::NameBadFormatted => StatusCode::BAD_REQUEST,
        domain::CreateUserError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
      };
      (status_code, Json(Reply::err(e.to_string())))
    }
  }
}

pub async fn get_users<UR: UserRepo>(
  State(state): State<Arc<Mutex<UR>>>,
) -> (StatusCode, Json<Reply<Vec<UserReply>>>) {
  let state = state.lock().await;
  let users = match state.get_users().await {
    Ok(users) => users,
    Err(e) => {
      return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(Reply::err(e.to_string())),
      )
    }
  };
  let reply = users.into_iter().map(UserReply::from).collect::<Vec<_>>();
  (StatusCode::OK, Json(Reply::data(reply)))
}

pub async fn get_user_by_id<UR: UserRepo>(
  State(user_repo): State<Arc<Mutex<UR>>>,
  Path(id): Path<String>,
) -> impl IntoResponse {
  let user_repo = user_repo.lock().await;
  match user_repo.get_user_by_id(id).await {
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
  id: String,
  username: String,
}

impl<U: domain::User> From<U> for UserReply {
  fn from(value: U) -> Self {
    UserReply {
      id: value.id(),
      username: value.name(),
    }
  }
}
