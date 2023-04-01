use serde::{Deserialize, Serialize};

// the output to our `create_user` handler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserReply {
  pub id: String,
  pub username: String,
}

// the input to our `create_user` handler
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserPayload {
  pub username: String,
}
