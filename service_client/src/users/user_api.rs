use serde::{Deserialize, Serialize};

// the output to our `create_user` handler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserReply {
  pub id: String,
  pub username: String,
}

impl<U: domain::User> From<U> for UserReply {
  fn from(value: U) -> Self {
    UserReply {
      id: value.id(),
      username: value.name(),
    }
  }
}

// the input to our `create_user` handler
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserPayload {
  pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserPayload {
  pub username: String,
}
