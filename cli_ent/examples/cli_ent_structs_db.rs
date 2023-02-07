use anyhow::{Context, Ok, Result};
use clap::{Parser, Subcommand};
use domain::UserRepo;
use std::sync::Arc;
use surrealdb_reqwest_adapter::{Auth, SurrealReqwest};
use tokio::sync::Mutex;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
  /// Turn debugging information on
  #[arg(short, long, action = clap::ArgAction::Count)]
  debug: u8,

  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
  /// does testing things
  List,
  Get {
    #[arg(short, long)]
    id: String,
  },
  Create {
    #[arg(short, long)]
    name: String,
    #[arg(short, long)]
    email: String,
  },
  Rm {
    #[arg(short, long)]
    id: String,
  },
}

#[tokio::main]
async fn main() -> Result<()> {
  let cli = Cli::parse();

  // You can see how many times a particular flag or argument occurred
  // Note, only flags can have multiple occurrences
  match cli.debug {
    0 => println!("Debug mode is off"),
    1 => println!("Debug mode is kind of on"),
    2 => println!("Debug mode is on"),
    _ => println!("Don't be crazy"),
  }

  // You can check for the existence of subcommands, and if found use their
  // matches just as you would the top level cmd
  let cmd = match cli.command {
    Some(cmd) => cmd,
    None => {
      return Ok(());
    }
  };

  let auth = Auth::new("my_user", "my_pass");
  let surreal_client = SurrealReqwest::new("my_ns", "my_db", "http://localhost:8000", auth);
  let user_repo = Arc::new(Mutex::new(surreal_client));
  match cmd {
    Commands::List => get_users(user_repo).await?,
    Commands::Get { id } => get_user_by_id(user_repo, id).await?,
    Commands::Create { name, .. } => create_user(user_repo, name).await?,
    Commands::Rm { id } => delete_user(user_repo, id).await?,
  }
  Ok(())
}

pub async fn create_user<UR: UserRepo>(state: Arc<Mutex<UR>>, name: String) -> Result<()> {
  let created_user = {
    let mut user_repo = state.lock().await;
    user_repo.create_user(name).await
  };
  let user: UserReply = created_user?.into();
  println!("{user:?}");
  Ok(())
}

pub async fn get_users<UR: UserRepo>(state: Arc<Mutex<UR>>) -> Result<()> {
  let users_result = {
    let state = state.lock().await;
    state.get_users().await
  };

  let users: Vec<UserReply> = users_result
    .context("context")?
    .into_iter()
    .map(|u| u.into())
    .collect();
  println!("Found: {users:?}");
  Ok(())
}

pub async fn get_user_by_id<UR: UserRepo>(user_repo: Arc<Mutex<UR>>, id: String) -> Result<()> {
  let result = {
    let user_repo = user_repo.lock().await;
    user_repo.get_user_by_id(id).await
  };
  let user: UserReply = result?.into();
  println!("Found: {user:?}");
  Ok(())
}

pub async fn delete_user<UR: UserRepo>(user_repo: Arc<Mutex<UR>>, id: String) -> Result<()> {
  let res = {
    let mut user_repo = user_repo.lock().await;
    user_repo.delete_user(id).await
  };
  let user: UserReply = res?.into();
  println!("Deleted: {user:?}");
  Ok(())
}

// the output to our `create_user` handler
#[derive(Clone, Debug)]
pub struct UserReply {
  pub id: String,
  pub username: String,
}

impl<U: domain::User> From<U> for UserReply {
  fn from(value: U) -> Self {
    UserReply {
      id: value.id(),
      username: value.name(),
    }
  }
}
