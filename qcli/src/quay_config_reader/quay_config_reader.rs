use crate::quay_config_reader::organization_struct::organization_struct::Actions;

use super::organization_struct::organization_struct::{OrganizationYaml, QuayResponse};

use futures::future::join_all;
use governor::clock::{QuantaClock, QuantaInstant};
use governor::middleware::NoOpMiddleware;
use governor::state::{InMemoryState, NotKeyed};
use governor::{self, RateLimiter};
use log::{debug, error, info};
use reqwest::StatusCode;
use std::error::Error;
use std::num::NonZeroU32;
use std::{fs::File, sync::Arc};
use tokio::fs::read_dir;
use array_tool::vec::Uniq;


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
            info!("Loading config from  {:?} ", f.file_name());
            let f = File::open(f.path())?;
            //let scrape_config: OrganizationYaml =
            //    serde_yaml::from_reader(f)

            match serde_yaml::from_reader(f) {
                Ok(scrape_config) => {
                    self.organization.push(scrape_config);
                }
                Err(e) => {
                    error!("{:?}", e)
                }
            }
        }

        Ok(())
    }

    pub async fn check_config(&self) -> Result<(), std::io::Error> {
        let mut files = read_dir(self.directory.to_owned()).await?;
        while let Some(f) = files.next_entry().await? {
            let f2 = File::open(f.path())?;
            //let scrape_config: OrganizationYaml =
            //    serde_yaml::from_reader(f)

            let result: Result<OrganizationYaml, serde_yaml::Error> = serde_yaml::from_reader(f2);
            match result {
                Ok(_) => {
                    info!("Config verified from  {:?} ", f.file_name());
                }
                Err(e) => {
                    error!("{:?}", e)
                }
            }
        }
        Ok(())
    }

    
    pub async fn create_login(self) -> Result<(), Box<dyn Error>> {

       let mut quay_endopoints :Vec<String>= Vec::new();
       
        for org in self.organization {
            quay_endopoints.push(org.quay_endpoint.clone());
       }

       quay_endopoints = quay_endopoints.unique();

       info!("Found {} unique Quay endpoint(s)",quay_endopoints.len());

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

    fn print_result(&self, _description: String, result: Result<QuayResponse, Box<dyn Error>>) {
        match result {
            Ok(r) => {
                let mut corrected_description = String::new();

                if r.status_code == StatusCode::NO_CONTENT {
                    // 204 from success delete organization
                    corrected_description.insert_str(0, "No Content success");
                } else {
                    corrected_description.insert_str(0, r.description.as_str());
                }

                
                info!("{:?}",r);

                //println!("------------------------");
                //println!("{} {}", description, corrected_description);
                //println!("Status code: {}", r.status_code);
                //println!("Message: {}", r.response);
            }
            Err(e) => error!("Error: {}", e),
        }
    }

    pub async fn delete_all(&self) -> Result<(), Box<dyn Error>> {
        let mut handles_delete_organization = Vec::new();
        let orgs = self.get_organizations();

        for org in orgs {
            info!("Processing organization: {}", org.quay_organization);

            handles_delete_organization
                .push(org.delete_organization(self.get_cloned_governor(), self.log_level));
        }
        let results = join_all(handles_delete_organization);

        for result in results.await {
            self.print_result("Organization ->".to_string(), result);
        }

        Ok(())
    }

    pub async fn create_all(&self,) -> Result<(), Box<dyn Error>> {
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

        debug!("this is a debug {}", "message");

        for org in orgs {
            info!(
                "Processing config for organization: {}",
                org.quay_organization
            );

            handles_all_organizations
                .push(org.create_organization(self.get_cloned_governor(), self.log_level));
            

            for robot in &org.robots {
                handles_all_robots.push(org.create_robot(
                    robot,
                    self.get_cloned_governor(),
                    self.log_level,
                ));
            }
            for team in &org.teams {
                handles_all_teams.push(org.create_team(
                    team,
                    self.get_cloned_governor(),
                    self.log_level,
                ));

                for member in &team.members.users {
                    handles_all_team_members.push(org.add_user_to_team(
                        &team.name,
                        &member,
                        self.get_cloned_governor(),
                        self.log_level,
                    ))
                }

                for member in &team.members.robots {
                    handles_all_team_members.push(org.add_robot_to_team(
                        &team.name,
                        &member,
                        self.get_cloned_governor(),
                        self.log_level,
                    ))
                }
            }

            for repository in &org.repositories {
                handles_all_repositories.push(org.create_repository(
                    repository,
                    self.get_cloned_governor(),
                    self.log_level,
                ));
                handles_all_extra_user_permissions.push(org.get_user_permission_from_repository(
                    &repository,
                    self.get_cloned_governor(),
                    self.log_level,
                ));
                handles_all_extra_team_permissions.push(org.get_team_permission_from_repository(
                    &repository,
                    self.get_cloned_governor(),
                    self.log_level,
                ));

                handles_all_mirror_configurations.push(org.create_repository_mirror(
                    &repository,
                    self.get_cloned_governor(),
                    self.log_level,
                ));

                if let Some(permissions) = &repository.permissions {
                    for robot in &permissions.robots {
                        handles_all_repositories_permissions.push(
                            org.grant_robot_permission_to_repository(
                                &repository.name,
                                &robot,
                                self.get_cloned_governor(),
                                self.log_level,
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
                                    self.log_level,
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
                                self.log_level,
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

        
        info!("TOTAL REQUESTS : {}", total_requestes);

       
        // Create organization
        info!(
            "Creating {} organization cuncurrently",
            handles_all_organizations.len()
        );

        let results = join_all(handles_all_organizations);

        for result in results.await {
            self.print_result("Organization ->".to_string(), result);
        }

   
        // Create robots
        info!("Creating {} robots cuncurrently", handles_all_robots.len());

        let results = join_all(handles_all_robots);

        for result in results.await {
            self.print_result("Robots ->".to_string(), result);
        }

      
        // Create teams
        info!("Creating {} teams cuncurrently", handles_all_teams.len());
        let results = join_all(handles_all_teams);

        for result in results.await {
            self.print_result("Teams ->".to_string(), result);
        }

    
        // Adding team members
        info!(
            "Adding {} team members cuncurrently",
            handles_all_team_members.len()
        );
        let results = join_all(handles_all_team_members);

        for result in results.await {
            self.print_result("Team members ->".to_string(), result);
        }

    
        // Create repositories
        info!(
            "Creating {} repositories cuncurrently",
            handles_all_repositories.len()
        );

        let results = join_all(handles_all_repositories);

        for result in results.await {
            self.print_result("Repository ->".to_string(), result);
        }

       
        //  Get user currently repositories permission (IF ANY)
        info!(
            "Delete extra user and robot permission from {} repository cuncurrently",
            handles_all_extra_user_permissions.len()
        );
        let results = join_all(handles_all_extra_user_permissions);

        for result in results.await {
            self.print_result("Repository USER permissions ->".to_string(), result);
        }

  
        // Get currently team repositories permission (IF ANY)
        info!(
            "Delete extra team permission from {} repository cuncurrently",
            handles_all_extra_team_permissions.len()
        );
        let results = join_all(handles_all_extra_team_permissions);

        for result in results.await {
             self.print_result("Repository TEAM permissions ->".to_string(), result);
        }
  ;
        // Create repositories permission
        info!(
            "Creating {} repositories permissions cuncurrently",
            handles_all_repositories_permissions.len()
        );
        let results = join_all(handles_all_repositories_permissions);

        for result in results.await {
            self.print_result("Repository permissions ->".to_string(), result);
        }

       
        // Configure repository mirror
        info!(
            "Configuring {} repositories mirror concurrently",
            handles_all_mirror_configurations.len()
        );
        let results = join_all(handles_all_mirror_configurations);

        for result in results.await {
            self.print_result("Repository permissions ->".to_string(), result);
        }

        Ok(())
        /*

        */
    }
}
