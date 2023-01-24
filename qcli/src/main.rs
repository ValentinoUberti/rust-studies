#![deny(elided_lifetimes_in_paths)]
mod quay_config_reader;
use clap::{Args, Parser, Subcommand};
use env_logger::Env;
use std::error::Error;
//use console_subscriber;
use env_logger;

use log::error;
use log::info;
use log::warn;

use crate::quay_config_reader::quay_config_reader::QuayXmlConfig;

#[derive(Parser)]
#[command(author, version, about="Quay batch processing cli written in Rust", long_about = None)]
#[command(help_template(
    "\
{before-help}{name} {version} - {author-with-newline}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
"
))]
struct Cli {
    #[command(subcommand)]
    command: SubCommands,

    #[arg(short, long)]
    dir: String,

    #[arg(short, long)]
    /// Accepted log level: info, debug
    log_level: Option<log::Level>,
}

#[derive(Subcommand)]
enum SubCommands {
    /// Adds files to myapp
    Create(Create),
    Delete(Delete),
    Check(Check),
}

#[derive(Args)]
struct Create {}

#[derive(Args)]
struct Delete {}

#[derive(Args)]
struct Check {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //console_subscriber::init();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let cli = Cli::parse();
    let req_per_seconds = 300;

    let log_level: log::Level;
    
    match cli.log_level {
        Some(ll) => log_level=ll,
        None => log_level=log::Level::Info,
    }

 

    let mut config = QuayXmlConfig::new(&cli.dir, req_per_seconds, log_level);
    
    match &cli.command {
        SubCommands::Create(_) => {
            //println!("-----");
            info!("Checking quay configurations file in {} directory...",&cli.dir);
            //println!("-----");
            config.check_config().await?;
            //println!("-----");
            info!("Loading quay configurations file in {} directory...",&cli.dir);
            //println!("-----");
            config.load_config().await?;
            //println!("-----");
            info!("Creating quay configurations...");
            //println!("-----");
            config.create_all(1).await?;
        }
        SubCommands::Delete(_) => {
            println!("-----");
            println!("Checking quay configurations file in {} directory...",&cli.dir);
            println!("-----");
            config.check_config().await?;
            println!("-----");
            println!("Loading quay configurations file in {} directory...",&cli.dir);
            println!("-----");
            config.load_config().await?;
            println!("-----");
            println!("Creating quay configurations...");
            println!("-----");
            config.delete_all(1).await?;
        }
        SubCommands::Check(_) => {
            println!("-----");
            println!("Checking quay configurations file in {} directory...",&cli.dir);
            println!("-----");
            config.check_config().await?;
        }
    }
  Ok(())
}
