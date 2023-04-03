use reqwest::header::{ACCESS_CONTROL_ALLOW_HEADERS, CONTENT_TYPE};

use crate::{CreateUserPayload, Reply, UserReply};

pub struct Client {
  pub(crate) server: String,
}

pub enum ClientError {
  Reqwest(reqwest::Error),
  Msg(String),
}

impl From<reqwest::Error> for ClientError {
  fn from(value: reqwest::Error) -> Self {
    ClientError::Reqwest(value)
  }
}

impl<T> From<Reply<T>> for Result<T, ClientError> {
  fn from(value: Reply<T>) -> Self {
    match value {
      Reply::Data(reply) => Ok(reply),
      Reply::Err(msg) => Err(ClientError::Msg(msg)),
    }
  }
}

impl Client {
  pub fn new(server: String) -> Self {
    Self { server }
  }

  pub fn user(&self) -> UserClient {
    UserClient::new(self)
  }
}

pub struct UserClient {
  url: String,
}

impl UserClient {
  fn new(parent: &Client) -> Self {
    Self {
      url: format!("{}/user", parent.server),
    }
  }

  pub async fn create_one(&self, username: impl Into<String>) -> Result<UserReply, ClientError> {
    let body = serde_json::to_string(&CreateUserPayload {
      username: username.into(),
    })
    .unwrap();
    let client = reqwest::Client::new();
    let reqwest = client
      .post(self.url.clone())
      .header("Accept", "application/json")
      .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
      .header(CONTENT_TYPE, "application/json")
      .body(body);
    let resp = reqwest //
      .send()
      .await?
      .json::<Reply<UserReply>>()
      .await?;
    resp.into()
  }
}
