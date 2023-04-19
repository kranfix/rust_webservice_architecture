use anyhow::Result;
use cli_ent::run_cli;
use service_client::client::*;

#[tokio::main]
async fn main() -> Result<()> {
  let client = Client::new("http://localhost:3000");
  let user_client = client.user();
  run_cli(user_client).await
}
