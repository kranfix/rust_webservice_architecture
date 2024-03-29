use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use domain::UserRepo;
use service_client::{CreateUserPayload, Reply, UpdateUserPayload, UserReply};
use std::sync::Arc;

#[macro_export]
macro_rules! create_user_routes {
  ($user_repo:expr) => {{
    use axum::routing::get;
    use axum::Router;
    use axum_service::user_routes::{
      create_user, delete_user, get_user_by_id, get_users, update_user,
    };
    Router::new()
      .route("/", get(get_users).post(create_user))
      .route(
        "/:id",
        get(get_user_by_id).delete(delete_user).put(update_user),
      )
      .with_state($user_repo)
  }};
}

pub async fn create_user<UR: UserRepo>(
  State(user_repo): State<Arc<UR>>,
  Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
  let created_user = user_repo.create_user(payload.username).await;
  match created_user {
    Ok(u) => {
      let user_reply: UserReply = u.into();
      (StatusCode::OK, Json(Reply::Data(user_reply)))
    }
    Err(e) => {
      log::warn!("create_user: {}", e);
      let status_code = match e {
        domain::CreateUserError::NameBadFormatted => StatusCode::BAD_REQUEST,
        domain::CreateUserError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
      };
      (status_code, Json(Reply::Err(e.to_string())))
    }
  }
}

pub async fn get_users<UR: UserRepo>(
  State(user_repo): State<Arc<UR>>,
) -> (StatusCode, Json<Reply<Vec<UserReply>>>) {
  let users_result = user_repo.get_users().await;

  let users = match users_result {
    Ok(users) => users,
    Err(e) => {
      log::warn!("get_users: {}", e);
      return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(Reply::Err(e.to_string())),
      );
    }
  };
  let reply = users.into_iter().map(UserReply::from).collect::<Vec<_>>();
  (StatusCode::OK, Json(Reply::Data(reply)))
}

pub async fn get_user_by_id<UR: UserRepo>(
  State(user_repo): State<Arc<UR>>,
  Path(id): Path<String>,
) -> impl IntoResponse {
  let result = user_repo.get_user_by_id(id).await;
  let err = match result {
    Ok(user) => return (StatusCode::OK, Json(Reply::Data(UserReply::from(user)))),
    Err(e) => {
      log::warn!("get_user_by_id: {}", e);
      e
    }
  };

  let status_code = match &err {
    domain::GetUsersByIdError::NotFound(_) => StatusCode::NOT_FOUND,
    domain::GetUsersByIdError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
  };
  (status_code, Json(Reply::Err(err.to_string())))
}

pub async fn delete_user<UR: UserRepo>(
  State(user_repo): State<Arc<UR>>,
  Path(id): Path<String>,
) -> impl IntoResponse {
  let res = user_repo.delete_user(id).await;
  let err = match res {
    Ok(user) => return (StatusCode::OK, Json(Reply::Data(UserReply::from(user)))),
    Err(e) => {
      log::warn!("delete_user: {}", e);
      e
    }
  };

  let status_code = match &err {
    domain::DeleteUserError::UserNotFound(_) => StatusCode::NOT_FOUND,
    domain::DeleteUserError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
  };
  (status_code, Json(Reply::Err(err.to_string())))
}

pub async fn update_user<UR: UserRepo>(
  State(user_repo): State<Arc<UR>>,
  Path(id): Path<String>,
  Json(payload): Json<UpdateUserPayload>,
) -> impl IntoResponse {
  let res = user_repo.update_user(id, payload.username).await;
  let err = match res {
    Ok(user) => return (StatusCode::OK, Json(Reply::Data(UserReply::from(user)))),
    Err(e) => {
      log::warn!("update_user: {}", e);
      e
    }
  };

  let status_code = match &err {
    domain::UpdateUserError::UserNotFound(_) => StatusCode::NOT_FOUND,
    domain::UpdateUserError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
  };
  (status_code, Json(Reply::Err(err.to_string())))
}
