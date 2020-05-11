use anyhow::Result;
use lapin::{options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties, ExchangeKind};
use log::{debug, info};
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "RabbitMQ Connection Tester")]
struct Opt {
  #[structopt(short, long, requires_all(&["username", "password", "port", "vhost", "encrypted"]))]
  host: Option<String>,
  #[structopt(short, long, requires_all(&["host", "password", "port", "vhost", "encrypted"]))]
  username: Option<String>,
  #[structopt(short, long, requires_all(&["host", "username", "port", "vhost", "encrypted"]))]
  password: Option<String>,
  #[structopt(short = "P", long, requires_all(&["host", "username", "password", "vhost", "encrypted"]))]
  port: Option<String>,
  #[structopt(short, long, requires_all(&["host", "username", "password", "port", "encrypted"]))]
  vhost: Option<String>,
  #[structopt(short, long, help = "If enabled, uses `amqps` rather than `amqp` protocol", requires_all(&["host", "username", "password", "port", "vhost"]))]
  encrypted: bool,
  #[structopt(
    long,
    help = "Specify the full AMQP connection string, overrides other options",
    required_unless_all(&["host", "username", "password", "port", "vhost", "encrypted"])
  )]
  uri: Option<String>,
  // #[structopt(short, long, help = "Test if an exchange exists (using passive exchange.declare)")]
  // exchange: Option<String>,
}

fn main() -> Result<()> {
  pretty_env_logger::init_timed();

  let opt = Opt::from_args();

  smol::run(async {
    let uri = if let Some(u) = opt.uri {
      u
    } else {
      let protocol = if opt.encrypted { "amqps" } else { "amqp" };
      format!(
        "{}://{}:{}@{}:{}/{}",
        protocol,
        &opt.username.unwrap(),
        &opt.password.unwrap(),
        &opt.host.unwrap(),
        &opt.port.unwrap(),
        &opt.vhost.unwrap(),
      )
    };

    info!("connecting");
    let conn = Connection::connect(&uri, ConnectionProperties::default())
      .await
      .expect("can't connect");
    info!("connected");

    info!("creating channel");
    let _chan = conn.create_channel().await.expect("can't open channel");
    info!("channel created");
  });

  Ok(())
}
