use crate::quay_config_reader::organization_struct::organization_struct::Actions;

use super::organization_struct::organization_struct::{OrganizationYaml, QuayResponse};
use futures::future::join_all;
use governor::clock::{QuantaClock, QuantaInstant};
use governor::middleware::NoOpMiddleware;
use governor::state::{InMemoryState, NotKeyed};
use governor::{self, RateLimiter};
use reqwest::StatusCode;
use std::error::Error;
use std::num::NonZeroU32;
use std::{fs::File, sync::Arc};
use tokio::fs::read_dir;

#[derive(Debug)]
pub struct QuayXmlConfig {
    organization: Vec<OrganizationYaml>,
    directory: String,
    governor: Arc<RateLimiter<NotKeyed, InMemoryState, QuantaClock, NoOpMiddleware<QuantaInstant>>>,
    log_level: log::Level,
}

impl QuayXmlConfig {
    pub fn new(directory: &String, req_per_seconds: u32, log_level: log::Level) -> Self {
        let governor = Arc::new(governor::RateLimiter::direct(governor::Quota::per_minute(
            NonZeroU32::new(req_per_seconds).unwrap(),
        )));
        Self {
            organization: vec![],
            directory: directory.clone(),
            governor,
            log_level,
        }
    }

    pub async fn load_config(&mut self) -> Result<(), std::io::Error> {
        let mut files = read_dir(self.directory.to_owned()).await?;

        while let Some(f) = files.next_entry().await? {
            println!("Loading config from  {:?} ", f.file_name());
            let f = File::open(f.path())?;
            let scrape_config: OrganizationYaml =
                serde_yaml::from_reader(f).expect("Could not read values.");
            self.organization.push(scrape_config);
        }

        Ok(())
    }

    pub async fn check_config(&self) -> Result<(), std::io::Error> {
        let mut files = read_dir(self.directory.to_owned()).await?;

        while let Some(f) = files.next_entry().await? {
            print!("Checking {:?} ", f.file_name());
            let f = File::open(f.path())?;
            let _scrape_config: OrganizationYaml =
                serde_yaml::from_reader(f).expect("Could not read values.");
            println!("\tOK");
        }

        Ok(())
    }

    pub fn get_organizations(&self) -> &Vec<OrganizationYaml> {
        &self.organization
    }

    pub fn get_cloned_governor(
        &self,
    ) -> Arc<RateLimiter<NotKeyed, InMemoryState, QuantaClock, NoOpMiddleware<QuantaInstant>>> {
        self.governor.clone()
    }

    fn print_result(&self, description: String, result: Result<QuayResponse, Box<dyn Error>>) {
        match result {
            Ok(r) => {
                let mut corrected_description = String::new();

                if r.status_code == StatusCode::NO_CONTENT {
                    // 204 from success delete organization
                    corrected_description.insert_str(0, "No Content success");
                } else {
                    corrected_description.insert_str(0, r.description.as_str());
                }
                println!("------------------------");
                println!("{} {}", description, corrected_description);
                println!("Status code: {}", r.status_code);
                println!("Message: {}", r.response);
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    pub async fn delete_all(&self, verbosity: u8) -> Result<(), Box<dyn Error>> {
        let mut handles_delete_organization = Vec::new();
        let orgs = self.get_organizations();

        for org in orgs {
            println!("Processing organization: {}", org.quay_organization);

            handles_delete_organization
                .push(org.delete_organization(self.get_cloned_governor(), self.log_level));
        }
        let results = join_all(handles_delete_organization);

        for result in results.await {
            self.print_result("Organization ->".to_string(), result);
        }

        Ok(())
    }

    pub async fn create_all(&self, verbosity: u8) -> Result<(), Box<dyn Error>> {
        let mut handles_all_organizations = Vec::new();
        // let mut handles_delete_organization = Vec::new();
        let mut handles_all_robots = Vec::new();
        let mut handles_all_teams = Vec::new();
        let mut handles_all_repositories = Vec::new();
        let mut handles_all_repositories_permissions = Vec::new();
        let mut handles_all_team_members = Vec::new();
        let mut handles_all_extra_user_permissions = Vec::new();
        let mut handles_all_extra_team_permissions = Vec::new();
        let mut handles_all_mirror_configurations = Vec::new();
        let orgs = self.get_organizations();

        for org in orgs {
            println!(
                "Processing config for organization: {}",
                org.quay_organization
            );

            handles_all_organizations.push(org.create_organization(self.get_cloned_governor()));
            //handles_delete_organization.push(org.delete_organization(self.get_cloned_governor(),self));

            for robot in &org.robots {
                handles_all_robots.push(org.create_robot(robot, self.get_cloned_governor()));
            }
            for team in &org.teams {
                handles_all_teams.push(org.create_team(team, self.get_cloned_governor()));

                for member in &team.members.users {
                    handles_all_team_members.push(org.add_user_to_team(
                        &team.name,
                        &member,
                        self.get_cloned_governor(),
                    ))
                }

                for member in &team.members.robots {
                    handles_all_team_members.push(org.add_robot_to_team(
                        &team.name,
                        &member,
                        self.get_cloned_governor(),
                    ))
                }
            }

            for repository in &org.repositories {
                handles_all_repositories
                    .push(org.create_repository(repository, self.get_cloned_governor()));
                handles_all_extra_user_permissions.push(
                    org.get_user_permission_from_repository(
                        &repository,
                        self.get_cloned_governor(),
                    ),
                );
                handles_all_extra_team_permissions.push(
                    org.get_team_permission_from_repository(
                        &repository,
                        self.get_cloned_governor(),
                    ),
                );

                handles_all_mirror_configurations
                    .push(org.create_repository_mirror(&repository, self.get_cloned_governor()));

                if let Some(permissions) = &repository.permissions {
                    for robot in &permissions.robots {
                        handles_all_repositories_permissions.push(
                            org.grant_robot_permission_to_repository(
                                &repository.name,
                                &robot,
                                self.get_cloned_governor(),
                            ),
                        )
                    }

                    for team in &permissions.teams {
                        for t in team {
                            handles_all_repositories_permissions.push(
                                org.grant_team_permission_to_repository(
                                    &repository.name,
                                    &t,
                                    self.get_cloned_governor(),
                                ),
                            )
                        }
                    }
                    for user in &permissions.users {
                        handles_all_repositories_permissions.push(
                            org.grant_user_permission_to_repository(
                                &repository.name,
                                &user,
                                self.get_cloned_governor(),
                            ),
                        )
                    }
                }
            }
        }

        let total_requestes = handles_all_organizations.len()
            + handles_all_robots.len()
            + handles_all_teams.len()
            + handles_all_repositories.len()
            + (handles_all_repositories_permissions.len() * 2)
            + handles_all_team_members.len()
            + handles_all_extra_user_permissions.len()
            + handles_all_extra_team_permissions.len()
            + (handles_all_mirror_configurations.len() * 3);

        println!("------------");
        println!("TOTAL REQUESTS : {}", total_requestes);

        println!("------------");
        // Create organization
        println!(
            "Creating {} organization cuncurrently",
            handles_all_organizations.len()
        );

        let results = join_all(handles_all_organizations);

        for result in results.await {
            //print_result("Organization ->".to_string(), result);
        }

        println!("------------");
        // Create robots
        println!("Creating {} robots cuncurrently", handles_all_robots.len());

        let results = join_all(handles_all_robots);

        for result in results.await {
            //print_result("Robots ->".to_string(), result);
        }

        println!("------------");
        // Create teams
        println!("Creating {} teams cuncurrently", handles_all_teams.len());
        let results = join_all(handles_all_teams);

        for result in results.await {
            //print_result("Teams ->".to_string(), result);
        }

        println!("------------");
        // Adding team members
        println!(
            "Adding {} team members cuncurrently",
            handles_all_team_members.len()
        );
        let results = join_all(handles_all_team_members);

        for result in results.await {
            //print_result("Team members ->".to_string(), result);
        }

        println!("------------");
        // Create repositories
        println!(
            "Creating {} repositories cuncurrently",
            handles_all_repositories.len()
        );

        let results = join_all(handles_all_repositories);

        for result in results.await {
            //   print_result("Repository ->".to_string(), result);
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
            // print_result("Repository TEAM permissions ->".to_string(), result);
        }
        println!("------------");
        // Create repositories permission
        println!(
            "Creating {} repositories permissions cuncurrently",
            handles_all_repositories_permissions.len()
        );
        let results = join_all(handles_all_repositories_permissions);

        for result in results.await {
            //print_result("Repository permissions ->".to_string(), result);
        }

        println!("------------");
        // Configure repository mirror
        println!(
            "Configuring {} repositories mirror concurrently",
            handles_all_mirror_configurations.len()
        );
        let results = join_all(handles_all_mirror_configurations);

        for result in results.await {
            //print_result("Repository permissions ->".to_string(), result);
        }

        Ok(())
        /*

        */
    }
}
