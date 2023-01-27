#![deny(elided_lifetimes_in_paths)]
mod quay_configurator;
use clap::{Args, Parser, Subcommand};
use core::panic;
use env_logger::{fmt::Color, Env, Target};
use std::error::Error;
//use console_subscriber;
use env_logger;
use std::io::Write;
use log::{info, Level};
use crate::quay_configurator::quay_config_reader::QuayXmlConfig;


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



/// qr async main
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
        .target(Target::Stdout)
        .format(|buf, record| {
            let mut level_style = buf.style();

            match record.level() {
                Level::Info => level_style.set_color(Color::Green).set_bold(true),
                Level::Debug => level_style.set_color(Color::Blue).set_bold(true),
                Level::Warn => level_style.set_color(Color::Yellow).set_bold(true),
                Level::Error => level_style.set_color(Color::Red).set_bold(true),
                Level::Trace => level_style.set_color(Color::Black).set_bold(true),
            };

            writeln!(
                buf,
                "[{} {}]: {}",
                buf.timestamp(),
                level_style.value(record.level()),
                record.args()
            )
        })
        .init();

    let mut config: QuayXmlConfig;
    match QuayXmlConfig::new(&cli.dir, req_per_seconds, log_level,1) {
        Ok(c) => {
            config = c;
            info!("Basic config successfully loaded")
        }
        Err(e) => panic!("{}", e.to_string()),
    }

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

            info!(
                "Loading quay configurations file in {} directory...",
                &cli.dir
            );
            config.load_config().await?;

            
        }
        SubCommands::Login(_) => {
            info!("Creating Quay login info from  {} directory...", &cli.dir);
            config.check_config().await?;
            config.load_config().await?;
            config.create_login().await?;
        }
    }
    Ok(())
}
