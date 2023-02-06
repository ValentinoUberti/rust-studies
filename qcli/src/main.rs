#![deny(elided_lifetimes_in_paths)]
mod quay_configurator;
use chrono::Utc;
use clap::{Args, Parser, Subcommand};
use core::panic;
use env_logger::{fmt::Color, Env, Target};
use std::error::Error;
use tokio::time::Instant;
//use console_subscriber;
use crate::quay_configurator::quay_config_reader::QuayXmlConfig;
use env_logger;
use log::{error, info, Level};
use std::io::Write;

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
    /// Log level. Accepted log level: info, debug. Default to info.
    log_level: Option<log::Level>,

    #[arg(short, long)]
    /// Log verbosity. Accepted values: 0,5,10. Default to 0
    verbosity: Option<u8>,

    #[arg(long)]
    /// Connection timeout in seconds. Default to 5
    timeout: Option<u64>,

    #[arg(long)]
    /// Verify Quay tls certificate. Default to true
    tls_verify: Option<bool>,
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

    let time = Utc::now();

    let now = Instant::now();

    let cli = Cli::parse();
    let req_per_seconds = 300;

    let log_level: log::Level;
    match cli.log_level {
        Some(ll) => log_level = ll,
        None => log_level = log::Level::Info,
    }

    let log_verbosity: u8;
    match cli.verbosity {
        Some(verbosity) => log_verbosity = verbosity,
        None => log_verbosity = 0,
    }

    let timeout: u64;
    match cli.timeout {
        Some(to) => timeout = to,
        None => timeout = 5,
    }

    let tls_verify: bool;
    match cli.tls_verify {
        Some(verify) => tls_verify = verify,
        None => tls_verify = true,
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

    info!("UTC start time: {:?}", time.to_rfc3339());

    let mut config: QuayXmlConfig;

    match QuayXmlConfig::new(
        &cli.dir,
        req_per_seconds,
        log_level,
        log_verbosity,
        timeout,
        false,
        tls_verify,
    ) {
        Ok(c) => {
            config = c;
            info!("Basic config successfully loaded")
        }
        Err(_e) => {
            error!("Login config file not found or corrupted. Run qcli login.");
            match QuayXmlConfig::new(
                &cli.dir,
                req_per_seconds,
                log_level,
                log_verbosity,
                timeout,
                true,
                tls_verify,
            ) {
                Ok(c) => {
                    config = c;
                    info!("Dummy login config successfully loaded")
                }
                Err(e) => {
                    error!("Login config file not found or corrupted. Stopping execution.");
                    panic!("{}", e.to_string());
                }
            }
        }
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

    info!("Execution terminated.");
    info!("Total execution time in seconds: {}",now.elapsed().as_secs_f32() );

    Ok(())
}
