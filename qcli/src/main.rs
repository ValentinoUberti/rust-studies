#![deny(elided_lifetimes_in_paths)]
mod quay_config_reader;
use clap::Parser;
use console_subscriber::spawn;
use futures::future::join_all;
use std::error::Error;
//use console_subscriber;

use crate::quay_config_reader::{
    organization_struct::organization_struct::{Actions, UserElement},
    quay_config_reader::QuayXmlConfig,
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
    let mut handles_delete_organization = Vec::new();
    let mut handles_all_robots = Vec::new();
    let mut handles_all_teams = Vec::new();
    let mut handles_all_repositories = Vec::new();
    let mut handles_all_repositories_permissions = Vec::new();

    let orgs = config.get_organizations();

    for org in orgs {
        println!("Added org: {}", org.quay_organization);

        handles.push(org.create_organization());
        handles_delete_organization.push(org.delete_organization());

        for robot in &org.robots {
            handles_all_robots.push(org.create_robot(robot));
        }
        for team in &org.teams {
            handles_all_teams.push(org.create_team(team));
        }
        for repository in &org.repositories {
            handles_all_repositories.push(org.create_repository(repository));

            if let Some(permissions) = &repository.permissions {
                for robot in &permissions.robots {
                    handles_all_repositories_permissions
                        .push(org.grant_robot_permission_to_repository(&repository.name, &robot))
                }

                for team in &permissions.teams {
                    for t in team {
                        handles_all_repositories_permissions
                            .push(org.grant_team_permission_to_repository(&repository.name, &t))
                    }
                }
                for user in &permissions.users {
                    handles_all_repositories_permissions
                        .push(org.grant_user_permission_to_repository(&repository.name, &user))
                }
            }
        }
    }

    println!("------------");
    // Create organization
    let results = join_all(handles);

    for result in results.await {
        match result {
            Ok(r) => {
                println!("------------------------");
                println!("Creating organization {}", r.description);
                println!("Status code: {}", r.status_code);
                println!("Message: {}", r.response);
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("------------");
    // Create robots
    println!("Creating {} robots cuncurrently", handles_all_robots.len());
    let results = join_all(handles_all_robots);

    for result in results.await {
        match result {
            Ok(r) => {
                println!("------------------------");
                println!("{}", r.description);
                println!("Status code: {}", r.status_code);
                println!("Message: {}", r.response);
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("------------");
    // Create teams
    println!("Creating {} teams cuncurrently", handles_all_teams.len());
    let results = join_all(handles_all_teams);

    for result in results.await {
        match result {
            Ok(r) => {
                println!("------------------------");
                println!("{}", r.description);
                println!("Status code: {}", r.status_code);
                println!("Message: {}", r.response);
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("------------");
    // Create repositories
    println!(
        "Creating {} repositories cuncurrently",
        handles_all_repositories.len()
    );
    let results = join_all(handles_all_repositories);

    for result in results.await {
        match result {
            Ok(r) => {
                println!("------------------------");
                println!("{}", r.description);
                println!("Status code: {}", r.status_code);
                println!("Message: {}", r.response);
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("------------");
    // Create repositories
    println!(
        "Creating {} repositories permissions cuncurrently",
        handles_all_repositories_permissions.len()
    );
    let results = join_all(handles_all_repositories_permissions);

    for result in results.await {
        match result {
            Ok(r) => {
                println!("------------------------");
                println!("{}", r.description);
                println!("Status code: {}", r.status_code);
                println!("Message: {}", r.response);
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    /*
     let results = join_all(handles_delete_organization);

     for result in results.await {
         match result {
             Ok(r) => {
                 println!("------------------------");
                 println!("{}", r.description);
                 println!("Status code: {}", r.status_code);
                 println!("Message: {}", r.response);
             }
             Err(e) => println!("Error: {}", e),
         }
     }
    */
    Ok(())
}
