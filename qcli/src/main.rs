#![deny(elided_lifetimes_in_paths)]
mod quay_config_reader;
use clap::{Args, Parser, Subcommand};
use env_logger::Env;
use std::error::Error;
//use console_subscriber;
use env_logger;

use log::info;

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
    /// Quay yaml directory [REQUIRED]
    dir: String,

    #[arg(short, long)]
    /// Accepted log level: info, debug
    log_level: Option<log::Level>,
}

#[derive(Subcommand)]
enum SubCommands {
    /// Create all Quay organizations
    Create(Create),
    /// Delete all Quay organizations
    Delete(Delete),
    /// Check all Quay organizations yaml files
    Check(Check),
    /// Login to detected Quay organizations
    Login(Login),
}

#[derive(Args)]
struct Login {}

#[derive(Args)]
struct Create {}

#[derive(Args)]
struct Delete {}

#[derive(Args)]
struct Check {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //console_subscriber::init();

    let cli = Cli::parse();
    let req_per_seconds = 300;

    let log_level: log::Level;

    match cli.log_level {
        Some(ll) => log_level = ll,
        None => log_level = log::Level::Info,
    }
    //env_logger::init_from_env(Env::default().default_filter_or(log_level.as_str()));
    env_logger::Builder::from_env(Env::default().default_filter_or(log_level.as_str()))
        .default_format()
        .init();
    let mut config = QuayXmlConfig::new(&cli.dir, req_per_seconds, log_level);

    match &cli.command {
        SubCommands::Create(_) => {
            info!(
                "Checking quay configurations file in {} directory...",
                &cli.dir
            );

            config.check_config().await?;

            info!(
                "Loading quay configurations file in {} directory...",
                &cli.dir
            );

            config.load_config().await?;

            info!("Creating quay configurations...");

            config.create_all().await?;
        }
        SubCommands::Delete(_) => {
            info!(
                "Checking quay configurations file in {} directory...",
                &cli.dir
            );

            config.check_config().await?;

            info!(
                "Loading quay configurations file in {} directory...",
                &cli.dir
            );

            config.load_config().await?;

            info!("Creating quay configurations...");

            config.delete_all().await?;
        }
        SubCommands::Check(_) => {
            info!(
                "Checking quay configurations file in {} directory...",
                &cli.dir
            );

            config.check_config().await?;
        }
        SubCommands::Login(_) => {
            todo!()
        }
    }
    Ok(())
}
