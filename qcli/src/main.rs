#![deny(elided_lifetimes_in_paths)]
mod quay_config_reader;
use clap::Parser;
use console_subscriber::spawn;
use futures::future::join_all;
use std::error::Error;
//use console_subscriber;

use crate::quay_config_reader::{
    organization_struct::organization_struct::Actions, quay_config_reader::QuayXmlConfig,
};

#[derive(Parser, Debug)]
#[command(author, version, about="Quay batch processing cli written in Rust", long_about = None)]
#[command(help_template(
    "\
{before-help}{name} {version} - {author-with-newline}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
"
))]
struct Args {
    #[arg(short, long)]
    dir: String,

    #[arg(short, long)]
    authfile: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //console_subscriber::init();
    let args = Args::parse();
    let mut config = QuayXmlConfig::new(args.dir);

    config.load_config().await?;

    let mut handles = Vec::new();
    let mut handles_all_robots = Vec::new();

    let orgs = config.get_organizations();

    for org in orgs {
        println!("Added org: {}", org.quay_organization);
        handles.push(org.create_organization());
        for robot in &org.robots {
            handles_all_robots.push(org.create_robot(robot));
        }
    }

    println!("------------");
    // Create organization
    let results = join_all(handles);


    for result in results.await {
        match result {
            Ok(r) => {
                println!("------------------------");
                println!("Creating organization {}",r.description);
                println!("Status code: {}", r.status_code);
                println!("Message: {}",r.response);
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("------------");
    // Create robots
    println!("Creating {} robots cuncurrently",handles_all_robots.len());
    let results = join_all(handles_all_robots);

    
    for result in results.await {
        match result {
            Ok(r) => {
                println!("------------------------");
                println!("Creating robot {}",r.description);
                println!("Status code: {}", r.status_code);
                println!("Message: {}",r.response);
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    Ok(())
}
