use reqwest::header::{ACCESS_CONTROL_ALLOW_HEADERS, CONTENT_TYPE};

use crate::{CreateUserPayload, Reply, UserReply};

/// Client
///
/// ```
/// let client = Client::new("http://localhost:3000");
/// let user_client = client.user();
/// let users = user_client.fetch_all().await.unwrap();
/// ```
pub struct Client {
  pub(crate) server: String,
}

#[derive(Debug)]
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
  pub fn new(server: impl Into<String>) -> Self {
    Self {
      server: server.into(),
    }
  }

  /// Handler for `/users/*`
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
      url: format!("{}/users", parent.server),
    }
  }

  pub async fn fetch_all(&self) -> Result<Vec<UserReply>, ClientError> {
    let client = reqwest::Client::new();
    let reqwest = client
      .get(self.url.clone())
      .header("Accept", "application/json")
      .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
      .header(CONTENT_TYPE, "application/json");
    let resp = reqwest //
      .send()
      .await?
      .json::<Reply<Vec<UserReply>>>()
      .await?;
    resp.into()
  }

  pub async fn create_one(&self, payload: &CreateUserPayload) -> Result<UserReply, ClientError> {
    let body = serde_json::to_string(payload).unwrap();
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

  pub async fn delete_one(&self, id: &str) -> Result<UserReply, ClientError> {
    let client = reqwest::Client::new();
    let reqwest = client
      .delete(format!("{}/{}", self.url, id))
      .header("Accept", "application/json")
      .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
      .header(CONTENT_TYPE, "application/json");
    let resp = reqwest //
      .send()
      .await?
      .json::<Reply<UserReply>>()
      .await?;
    resp.into()
  }
}
