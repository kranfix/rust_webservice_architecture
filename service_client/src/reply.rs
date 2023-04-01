use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub enum Reply<T> {
  #[allow(non_camel_case_types)]
  data(T),
  #[allow(non_camel_case_types)]
  err(String),
}

impl<T> From<Reply<T>> for Result<T, String> {
  fn from(value: Reply<T>) -> Self {
    match value {
      Reply::data(d) => Ok(d),
      Reply::err(e) => Err(e),
    }
  }
}

impl<T, E: Error> From<Result<T, E>> for Reply<T> {
  fn from(value: Result<T, E>) -> Self {
    match value {
      Ok(d) => Reply::data(d),
      Err(e) => Reply::err(e.to_string()),
    }
  }
}
