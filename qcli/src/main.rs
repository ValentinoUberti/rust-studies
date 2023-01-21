#![deny(elided_lifetimes_in_paths)]
mod quay_config_reader;
use clap::{Args, Parser, Subcommand};
//use console_subscriber::spawn;
use futures::future::join_all;

use quay_config_reader::organization_struct::organization_struct::QuayResponse;
use std::error::Error;
//use console_subscriber;

use crate::quay_config_reader::{
    organization_struct::organization_struct::Actions, quay_config_reader::QuayXmlConfig,
};

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
    #[arg(short, long)]
    dir: String,

    #[arg(short, long)]
    authfile: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Create(Create),
    Delete(Delete),
}

#[derive(Args)]
struct Create {}

#[derive(Args)]
struct Delete {}

fn print_result(description: String, result: Result<QuayResponse, Box<dyn Error>>) {
    match result {
        Ok(r) => {
            println!("------------------------");
            println!("{} {}", description, r.description);
            println!("Status code: {}", r.status_code);
            println!("Message: {}", r.response);
        }
        Err(e) => println!("Error: {}", e),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //console_subscriber::init();
    let cli = Cli::parse();
    let mut config = QuayXmlConfig::new(cli.dir);

    match &cli.command {
        Commands::Create(_) => {
            println!("Create")
        }
        Commands::Delete(_) => {
            println!("Delete")
        }
    }

    config.load_config().await?;

    let mut handles_all_organizations = Vec::new();
    let mut handles_delete_organization = Vec::new();
    let mut handles_all_robots = Vec::new();
    let mut handles_all_teams = Vec::new();
    let mut handles_all_repositories = Vec::new();
    let mut handles_all_repositories_permissions = Vec::new();
    let mut handles_all_team_members = Vec::new();
    let mut handles_all_extra_user_permissions = Vec::new();
    let mut handles_all_extra_team_permissions = Vec::new();
    let mut handles_all_mirror_configurations= Vec::new();
    let orgs = config.get_organizations();

    for org in orgs {
        println!("Added config for organization: {}", org.quay_organization);

        handles_all_organizations.push(org.create_organization());
        handles_delete_organization.push(org.delete_organization());

        for robot in &org.robots {
            handles_all_robots.push(org.create_robot(robot));
        }
        for team in &org.teams {
            handles_all_teams.push(org.create_team(team));

            for member in &team.members.users {
                handles_all_team_members.push(org.add_user_to_team(&team.name, &member))
            }

            for member in &team.members.robots {
                handles_all_team_members.push(org.add_robot_to_team(&team.name, &member))
            }
        }

        for repository in &org.repositories {
            handles_all_repositories.push(org.create_repository(repository));
            handles_all_extra_user_permissions
                .push(org.get_user_permission_from_repository(&repository));
            handles_all_extra_team_permissions
                .push(org.get_team_permission_from_repository(&repository));

            handles_all_mirror_configurations.push(org.create_repository_mirror(&repository));

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
/*
    println!("------------");
    // Create organization
    println!(
        "Creating {} organization cuncurrently",
        handles_all_organizations.len()
    );

    let results = join_all(handles_all_organizations);

    for result in results.await {
        print_result("Organization ->".to_string(), result);
        //pb.inc(1);
    }
    //pb.finish();

    println!("------------");
    // Create robots
    println!("Creating {} robots cuncurrently", handles_all_robots.len());
    let results = join_all(handles_all_robots);

    for result in results.await {
        print_result("Robots ->".to_string(), result);
    }

    println!("------------");
    // Create teams
    println!("Creating {} teams cuncurrently", handles_all_teams.len());
    let results = join_all(handles_all_teams);

    for result in results.await {
        print_result("Teams ->".to_string(), result);
    }

    println!("------------");
    // Adding team members
    println!(
        "Adding {} team members cuncurrently",
        handles_all_team_members.len()
    );
    let results = join_all(handles_all_team_members);

    for result in results.await {
        print_result("Team members ->".to_string(), result);
    }

    println!("------------");
    // Create repositories
    println!(
        "Creating {} repositories cuncurrently",
        handles_all_repositories.len()
    );

    let results = join_all(handles_all_repositories);

    for result in results.await {
        print_result("Repository ->".to_string(), result);
    }

    println!("------------");
    //  Get user currently repositories permission (IF ANY)
    println!(
        "Delete extra user and robot permission from {} repository cuncurrently",
        handles_all_extra_user_permissions.len()
    );
    let results = join_all(handles_all_extra_user_permissions);

    for result in results.await {
        //print_result("Repository USER permissions ->".to_string(), result);
    }

    println!("------------");
    // Get currently team repositories permission (IF ANY)
    println!(
        "Delete extra team permission from {} repository cuncurrently",
        handles_all_extra_team_permissions.len()
    );
    let results = join_all(handles_all_extra_team_permissions);

    for result in results.await {
        print_result("Repository TEAM permissions ->".to_string(), result);
    }
    println!("------------");
    // Create repositories permission
    println!(
        "Creating {} repositories permissions cuncurrently",
        handles_all_repositories_permissions.len()
    );
    let results = join_all(handles_all_repositories_permissions);

    for result in results.await {
        print_result("Repository permissions ->".to_string(), result);
    }
*/

    println!("------------");
    // Configure repository mirror
    println!(
        "Configuring {} repositories mirror concurrently",
        handles_all_mirror_configurations.len()
    );
    let results = join_all(handles_all_mirror_configurations);

    for result in results.await {
        print_result("Repository permissions ->".to_string(), result);
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
