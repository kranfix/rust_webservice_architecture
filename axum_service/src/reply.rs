use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Reply<T> {
  #[allow(non_camel_case_types)]
  data(T),
  #[allow(non_camel_case_types)]
  err(String),
}
