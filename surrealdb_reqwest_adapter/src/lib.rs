pub mod query_result;
use domain::{async_trait, CreateUserError};
use query_result::QueryResult;
use serde::{de::DeserializeOwned, Deserialize};

pub struct SurrealReqwest {
  ns: String,
  db: String,
  addr: String,
  auth: Auth,
}

impl SurrealReqwest {
  pub fn new(
    ns: impl Into<String>,
    db: impl Into<String>,
    addr: impl Into<String>,
    auth: Auth,
  ) -> Self {
    Self {
      ns: ns.into(),
      db: db.into(),
      addr: addr.into(),
      auth,
    }
  }

  pub async fn sql<T: DeserializeOwned>(
    &self,
    sql: impl Into<String>,
  ) -> Result<Vec<QueryResult<T>>, ()> {
    let client = reqwest::Client::new()
      .post(format!("{}/sql", self.addr))
      .basic_auth(self.auth.user.clone(), Some(self.auth.pass.clone()))
      .header("NS", self.ns.clone())
      .header("DB", self.db.clone())
      .header("Accept", "Application/json")
      .body(sql.into());
    let resp = client
      .send()
      .await
      .unwrap()
      .json::<Vec<QueryResult<T>>>()
      .await
      .unwrap();
    Ok(resp)
  }
}

pub struct Auth {
  user: String,
  pass: String,
}

impl Auth {
  pub fn new(user: impl Into<String>, pass: impl Into<String>) -> Auth {
    Auth {
      user: user.into(),
      pass: pass.into(),
    }
  }
}

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
