pub mod client;
pub mod query_result;

pub use client::SurrealReqwest;
pub use client::*;
use domain::{async_trait, CreateUserError};
use query_result::QueryResult;
use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Person {
  pub id: String,
  pub name: String,
}

impl domain::User for Person {
  fn id(&self) -> String {
    self.id.clone()
  }
  fn name(&self) -> String {
    self.name.clone()
  }
}

#[async_trait]
impl domain::UserRepo for SurrealReqwest {
  type User = Person;

  async fn create_user(&mut self, name: String) -> Result<Self::User, CreateUserError> {
    if name.trim().is_empty() {
      return Err(CreateUserError::NameBadFormatted);
    }
    let query_results = self
      .sql::<Person>(format!("CREATE person SET name={name}"))
      .await
      .map_err(|_| CreateUserError::Internal)?;
    let create_result = query_results.into_iter().next().unwrap();
    match create_result {
      QueryResult::OK { result, .. } => Ok(result[0].clone()),
      QueryResult::ERR { .. } => Err(CreateUserError::Internal),
    }
  }

  async fn get_users(&self) -> Vec<Self::User> {
    let query_results = self.sql::<Person>("SELECT * FROM person").await.unwrap();
    let select_result = query_results.into_iter().next().unwrap();
    match select_result {
      QueryResult::OK { result, .. } => result,
      QueryResult::ERR { .. } => Vec::new(),
    }
  }

  async fn get_user_by_id(&self, id: String) -> Option<Self::User> {
    let query_results = self
      .sql::<Person>(format!(r#"SELECT * FROM person:"{id}""#))
      .await
      .unwrap();
    let select_result = query_results.into_iter().next().unwrap();
    match select_result {
      QueryResult::OK { result, .. } => result.into_iter().next(),
      QueryResult::ERR { .. } => None,
    }
  }
}
