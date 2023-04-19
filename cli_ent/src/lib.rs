use anyhow::Result;
use clap::{Parser, Subcommand};
use service_client::client::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
  /// Turn debugging information on
  #[arg(short, long, action = clap::ArgAction::Count)]
  debug: u8,

  #[command(subcommand)]
  command: Option<UserCommand>,
}

#[derive(Subcommand)]
enum UserCommand {
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
  Update {
    #[arg(short, long)]
    id: String,
    #[arg(short, long)]
    name: String,
  },
  Rm {
    #[arg(short, long)]
    id: String,
  },
}

pub async fn run_cli(user_client: UserClient) -> Result<()> {
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

  match cmd {
    UserCommand::List => get_users(user_client).await?,
    UserCommand::Get { id } => get_user_by_id(user_client, id).await?,
    UserCommand::Create { name, .. } => create_user(user_client, name).await?,
    UserCommand::Update { id, name } => update_user(user_client, id, name).await?,
    UserCommand::Rm { id } => delete_user(user_client, id).await?,
  }
  Ok(())
}

pub async fn create_user(user_client: UserClient, name: String) -> Result<()> {
  let reply = user_client
    .create_one(&service_client::CreateUserPayload { username: name })
    .await;
  match reply {
    Ok(user) => {
      println!("Created: {user:?}");
    }
    Err(e) => {
      println!("{e:?}");
    }
  }
  Ok(())
}

pub async fn get_users(user_client: UserClient) -> Result<()> {
  let reply = user_client.fetch_all().await;
  match reply {
    Ok(users) => {
      println!("Found: {users:?}");
    }
    Err(e) => {
      println!("{e:?}");
    }
  }
  Ok(())
}

pub async fn get_user_by_id(user_client: UserClient, id: String) -> Result<()> {
  let reply = user_client.fetch_one(&id).await;
  match reply {
    Ok(user) => {
      println!("Found: {user:?}");
    }
    Err(e) => {
      println!("{e:?}");
    }
  }
  Ok(())
}

pub async fn update_user(user_client: UserClient, id: String, name: String) -> Result<()> {
  let reply = user_client
    .update_one(&id, &service_client::UpdateUserPayload { username: name })
    .await;
  match reply {
    Ok(user) => {
      println!("Updated: {user:?}");
    }
    Err(e) => {
      println!("{e:?}");
    }
  }
  Ok(())
}

pub async fn delete_user(user_client: UserClient, id: String) -> Result<()> {
  let reply = user_client.delete_one(&id).await;
  match reply {
    Ok(user) => {
      println!("Deleted: {user:?}");
    }
    Err(e) => {
      println!("{e:?}");
    }
  }
  Ok(())
}
