use anyhow::Result;
use cli_ent::run_cli;
use surrealdb_reqwest_adapter::{Auth, SurrealReqwest};

#[tokio::main]
async fn main() -> Result<()> {
  let auth = Auth::new("my_user", "my_pass");
  let surreal_client = SurrealReqwest::new("my_ns", "my_db", "http://localhost:8000", auth);
  run_cli(surreal_client).await
}
