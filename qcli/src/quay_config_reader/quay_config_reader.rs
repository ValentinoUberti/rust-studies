use crate::quay_config_reader::organization_struct::organization_struct::{Actions, QuayFnArguments};

use super::organization_struct::organization_struct::{OrganizationYaml, QuayResponse};

use array_tool::vec::Uniq;
use futures::future::join_all;
use governor::clock::{QuantaClock, QuantaInstant};
use governor::middleware::NoOpMiddleware;
use governor::state::{InMemoryState, NotKeyed};
use governor::{self, RateLimiter};
use log::{debug, error, info, warn};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, stdin, Write};
use std::num::NonZeroU32;
use std::path::Path;
use std::{fs::File, sync::Arc};
use tokio::fs::{self, read_dir, write};

#[derive(Debug)]
pub struct QuayXmlConfig {
    organization: Vec<OrganizationYaml>,
    directory: String,
    governor: Arc<RateLimiter<NotKeyed, InMemoryState, QuantaClock, NoOpMiddleware<QuantaInstant>>>,
    log_level: log::Level,
    log_verbosity: u8,
    quay_login_configs: QuayLoginConfigs,
}

impl QuayXmlConfig {
    pub fn new(
        directory: &String,
        req_per_seconds: u32,
        log_level: log::Level,
        log_verbosity: u8,
    ) -> Result<Self, Box<dyn Error>> {
        let governor = Arc::new(governor::RateLimiter::direct(governor::Quota::per_minute(
            NonZeroU32::new(req_per_seconds).unwrap(),
        )));

        let quay_configs_file = File::open(".qcli/login.yaml")?;

        match serde_yaml::from_reader(quay_configs_file) {
            Ok(quay_login_configs) => {
            return Ok(Self {
                organization: vec![],
                directory: directory.clone(),
                governor,
                log_level,
                log_verbosity,
                quay_login_configs
                
            });
        }, 
        Err(e) => return Err(Box::new(e)),
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

                    //Check if replicated
                    


                }
                Err(e) => {
                    error!("{:?}", e)
                }
            }
        }

       // let mut org_to_add: Vec<OrganizationYaml> = Vec::new();

        let tmp_organization = self.organization.clone();

        for org in &tmp_organization {
            match &org.replicate_to {
                Some(replicated_to) => {
                    for endpoint in replicated_to {
                       let mut new_org= org.clone();
                       new_org.change_endpoint(endpoint.to_string());
                       if (!self.organization.contains(&new_org)) {
                           self.organization.push(new_org);
                       } else {
                        let str_error=format!("Endpoint replication '{}' already present as a main Quay organization '{}' with endpoint '{}'. Ignoring....",endpoint,new_org.quay_organization,new_org.quay_endpoint);
                        warn!("{}", str_error);
                       }
                       
                    }
                }
                None => {}
            }
        }

        Ok(())
    }

    pub async fn check_config(&self) -> Result<(), std::io::Error> {
        let mut files = read_dir(self.directory.to_owned()).await?;
        while let Some(f) = files.next_entry().await? {
            let f2 = File::open(f.path())?;
           
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
        let mut quay_endpoints: Vec<String> = Vec::new();

        for org in self.organization {
            quay_endpoints.push(org.quay_endpoint.clone());
            
            match org.replicate_to {
                Some(replicated_to) => {
                    quay_endpoints.extend(replicated_to);
                }
                None => {}
            }
        }

        quay_endpoints = quay_endpoints.unique();

        info!("Found {} unique Quay endpoint(s)", quay_endpoints.len());

        // Checking if .qcli directory exists and creating it if does not.

        let login_directory = ".qcli";
        let login_file = "login.yaml";

        if !Path::new(login_directory).is_dir() {
            info!(".qcli directory does not exists. Creating...");
            fs::create_dir(login_directory).await?;
            info!(".qcli directory created.");
        } else {
            info!(".qcli directory exists.")
        }

        if !std::path::Path::new(login_file).exists() {
            warn!("{} does not exits. Creating...", login_file);

            let mut logins = QuayLoginConfigs::default();

            for q in quay_endpoints {
                print!("Please insert token for {}: ", q);
                io::stdout().flush()?;
                let mut token = String::new();
                io::stdin().read_line(&mut token)?;

                let endpoint = QuayEndopoint {
                    quay_endpoint: q,
                    quay_token: token.trim().to_string(),
                };

                logins.quay_endpoint_login.push(endpoint);
            }

            let f = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(format!("{}/{}", login_directory, login_file))?;

            serde_yaml::to_writer(f, &logins)?;
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

                info!("{:?}", r);

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

            let token: String;

            if let Some(t) = self.quay_login_configs.get_token_from_quay_endopoint(org.get_quay_endpoint()) {

                token = t
            } else {
                let err_str = format!("No token found for {} Quay endpoint. Please run 'qcli login. Ignoring this Quay organization.",org.get_quay_endpoint());
                error!("{}",err_str);
                continue;

            }


            let quay_fn_arguments = QuayFnArguments {
                token,
                governor: self.get_cloned_governor(),
                log_level: self.log_level,
                log_verbosity: self.log_verbosity,
            };

            handles_delete_organization
                .push(org.delete_organization(quay_fn_arguments));
        }
        let results = join_all(handles_delete_organization);

        for result in results.await {
            self.print_result("Organization ->".to_string(), result);
        }

        Ok(())
    }

   
    

    pub async fn create_all(&self) -> Result<(), Box<dyn Error>> {
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
            info!(
                "Processing config for organization: {}",
                org.quay_organization
            );


            let token: String;

            if let Some(t) = self.quay_login_configs.get_token_from_quay_endopoint(org.get_quay_endpoint()) {

                token = t
            } else {
                let err_str = format!("No token found for {} Quay endpoint. Please run 'qcli login. Ignoring this Quay organization.",org.get_quay_endpoint());
                error!("{}",err_str);
                continue;

            }


            let quay_fn_arguments = QuayFnArguments {
                token,
                governor: self.get_cloned_governor(),
                log_level: self.log_level,
                log_verbosity: self.log_verbosity,
            };

            handles_all_organizations
                .push(org.create_organization(quay_fn_arguments.clone()));

            for robot in &org.robots {
                handles_all_robots.push(org.create_robot(
                    robot,
                    quay_fn_arguments.clone()
                ));
            }
            for team in &org.teams {
                handles_all_teams.push(org.create_team(
                    team,
                    quay_fn_arguments.clone()
                ));

                for member in &team.members.users {
                    handles_all_team_members.push(org.add_user_to_team(
                        &team.name,
                        &member,
                        quay_fn_arguments.clone()
                    ))
                }

                for member in &team.members.robots {
                    handles_all_team_members.push(org.add_robot_to_team(
                        &team.name,
                        &member,
                        quay_fn_arguments.clone()
                    ))
                }
            }

            for repository in &org.repositories {
                handles_all_repositories.push(org.create_repository(
                    repository,
                    quay_fn_arguments.clone()
                ));
                handles_all_extra_user_permissions.push(org.get_user_permission_from_repository(
                    &repository,
                    quay_fn_arguments.clone()
                ));
                handles_all_extra_team_permissions.push(org.get_team_permission_from_repository(
                    &repository,
                    quay_fn_arguments.clone()
                ));

                handles_all_mirror_configurations.push(org.create_repository_mirror(
                    &repository,
                  
                    quay_fn_arguments.clone()
                ));

                if let Some(permissions) = &repository.permissions {
                    for robot in &permissions.robots {
                        handles_all_repositories_permissions.push(
                            org.grant_robot_permission_to_repository(
                                &repository.name,
                                &robot,
                                quay_fn_arguments.clone()
                            ),
                        )
                    }

                    for team in &permissions.teams {
                        for t in team {
                            handles_all_repositories_permissions.push(
                                org.grant_team_permission_to_repository(
                                    &repository.name,
                                    &t,
                                    quay_fn_arguments.clone()
                                ),
                            )
                        }
                    }
                    for user in &permissions.users {
                        handles_all_repositories_permissions.push(
                            org.grant_user_permission_to_repository(
                                &repository.name,
                                &user,
                                quay_fn_arguments.clone()
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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct QuayLoginConfigs {
    pub quay_endpoint_login: Vec<QuayEndopoint>,
}



impl QuayLoginConfigs {

    fn get_quay_login_configs(&self) -> Vec<QuayEndopoint> {
        self.quay_endpoint_login.clone()
    }

    pub fn get_token_from_quay_endopoint(&self,endpoint: String) -> Option<String> {

        for configured_endpoint in self.get_quay_login_configs() {

            if configured_endpoint.quay_endpoint == endpoint {
                return Some(configured_endpoint.quay_token)
            }

        }

        None
    }
}

#[derive(Debug, Serialize, Deserialize, Default,Clone)]
struct QuayEndopoint {
    pub quay_endpoint: String,
    pub quay_token: String,
}
