pub mod query_result;
use query_result::QueryResult;
use serde::de::DeserializeOwned;

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

#[derive(Clone)]
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
