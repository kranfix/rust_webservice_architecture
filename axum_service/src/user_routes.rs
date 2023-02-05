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
    use axum_service::user_routes::{create_user, delete_user, get_user_by_id, get_users};
    Router::new()
      .route("/", get(get_users).post(create_user))
      .route("/:id", get(get_user_by_id).delete(delete_user))
      .with_state($user_repo)
  }};
}

pub async fn create_user<UR: UserRepo>(
  State(state): State<Arc<Mutex<UR>>>,
  Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
  let created_user = {
    let mut user_repo = state.lock().await;
    user_repo.create_user(payload.username).await
  };
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
  let users_result = {
    let state = state.lock().await;
    state.get_users().await
  };

  let users = match users_result {
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
  let result = {
    let user_repo = user_repo.lock().await;
    user_repo.get_user_by_id(id).await
  };
  let err = match result {
    Ok(user) => return (StatusCode::OK, Json(Reply::data(UserReply::from(user)))),
    Err(e) => e,
  };

  let status_code = match &err {
    domain::GetUsersByIdError::NotFound(_) => StatusCode::NOT_FOUND,
    domain::GetUsersByIdError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
  };
  (status_code, Json(Reply::err(err.to_string())))
}

pub async fn delete_user<UR: UserRepo>(
  State(user_repo): State<Arc<Mutex<UR>>>,
  Path(id): Path<String>,
) -> impl IntoResponse {
  let res = {
    let mut user_repo = user_repo.lock().await;
    user_repo.delete_user(id).await
  };
  let err = match res {
    Ok(user) => return (StatusCode::OK, Json(Reply::data(UserReply::from(user)))),
    Err(e) => e,
  };

  let status_code = match &err {
    domain::DeleteUserError::UserNotFound(_) => StatusCode::NOT_FOUND,
    domain::DeleteUserError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
  };
  (status_code, Json(Reply::err(err.to_string())))
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
