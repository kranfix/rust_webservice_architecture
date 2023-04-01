use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Reply<T> {
  Data(T),
  Err(String),
}

impl<T> From<Reply<T>> for Result<T, String> {
  fn from(value: Reply<T>) -> Self {
    match value {
      Reply::Data(d) => Ok(d),
      Reply::Err(e) => Err(e),
    }
  }
}

impl<T, E: Error> From<Result<T, E>> for Reply<T> {
  fn from(value: Result<T, E>) -> Self {
    match value {
      Ok(d) => Reply::Data(d),
      Err(e) => Reply::Err(e.to_string()),
    }
  }
}
