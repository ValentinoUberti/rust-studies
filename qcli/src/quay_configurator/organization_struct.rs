
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use governor::clock::{QuantaClock, QuantaInstant};
use governor::middleware::NoOpMiddleware;
use governor::state::{InMemoryState, NotKeyed};
use governor::{self, RateLimiter};
use log::debug;
use std::sync::Arc;
use std::{collections::HashMap, error::Error, time::Duration};
use substring::Substring;

use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::quay_config_reader::MirrorLogin;

#[derive(Debug, Default, Clone)]
pub struct QuayResponse {
    pub response: Value,
    pub status_code: StatusCode,
    pub description: String,
}

#[async_trait]
pub trait Actions {
    async fn create_organization(
        &self,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>>;
    async fn grant_user_permission_to_repository(
        &self,
        repo: &String,
        user: &UserElement,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>>;
    async fn delete_user_permission_from_repository(
        &self,
        repo: &String,
        user: &UserElement,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>>;

    async fn delete_team_permission_from_repository(
        &self,
        repo: &String,
        user: &UserElement,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>>;
    async fn get_user_permission_from_repository(
        &self,
        repo: &Repository,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>>;
    async fn get_team_permission_from_repository(
        &self,
        repo: &Repository,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>>;
    async fn grant_robot_permission_to_repository(
        &self,
        repo: &String,
        user: &UserElement,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>>;
    async fn grant_team_permission_to_repository(
        &self,
        repo: &String,
        user: &UserElement,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>>;
    async fn delete_organization(
        &self,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>>;
    async fn create_robot(
        &self,
        robot: &RobotDetails,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>>;
    async fn create_team(
        &self,
        team: &Team,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>>;
    async fn add_user_to_team(
        &self,
        team: &String,
        user: &String,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>>;

    async fn add_robot_to_team(
        &self,
        team: &String,
        user: &String,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>>;
    async fn create_repository(
        &self,
        team: &Repository,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>>;
    async fn create_repository_mirror(
        &self,
        team: &Repository,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>>;
    async fn send_request<T>(
        &self,
        endpoint: String,
        body: &T,
        description: &String,
        method: reqwest::Method,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>>
    where
        T: Serialize + std::marker::Sync,
    {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(quay_fn_arguments.tls_verify)
            .build()?;

        let api = client
            .request(method, endpoint)
            .timeout(Duration::from_secs(quay_fn_arguments.timeout))
            .header("Content-Type", "application/json")
            .header("accept", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", &quay_fn_arguments.token),
            )
            .json(body);

        ////println!("{:?}", api);
        let retry_jitter = governor::Jitter::new(Duration::ZERO, Duration::from_millis(1));
        quay_fn_arguments
            .governor
            .until_ready_with_jitter(retry_jitter)
            .await;

        //println("{:?}",governor.)
        let response_status = api.send().await?;
        let status_code = response_status.status();
        let response = match response_status.json::<serde_json::Value>().await {
            Ok(r) => r,
            Err(_) => Value::Null,
        };
        ////println!("{:?}", response);
        let quay_response = QuayResponse {
            response,
            status_code,
            description: description.clone(),
        };

        if quay_fn_arguments.log_level == log::Level::Debug {
            debug!("{:?}", quay_response.clone());
        }
        Ok(quay_response)
    }
}

#[async_trait]
impl Actions for OrganizationYaml {
    async fn create_organization(
        &self,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>> {
        let endpoint = format!("https://{}/api/v1/organization/", &self.quay_endpoint);
        let mut body = HashMap::new();
        body.insert("name", &self.quay_organization);
        body.insert("email", &self.quay_organization_role_email);

        let response = &self
            .send_request(
                endpoint,
                &body,
                &format!(
                    "Org: {} Endpoint: {}",
                    &self.quay_organization, &self.quay_endpoint
                ),
                Method::POST,
                quay_fn_arguments,
            )
            .await?;

        Ok(response.clone())
    }

    async fn grant_user_permission_to_repository(
        &self,
        repo: &String,
        user: &UserElement,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>> {
        let endpoint = format!(
            "https://{}/api/v1/repository/{}/{}/permissions/user/{}",
            &self.quay_endpoint, &self.quay_organization, repo, user.name
        );
        let mut body = HashMap::new();
        body.insert("role", &user.role);

        let response = &self
            .send_request(
                endpoint,
                &body,
                &self.quay_organization,
                Method::PUT,
                quay_fn_arguments,
            )
            .await?;

        Ok(response.clone())
    }
    async fn delete_user_permission_from_repository(
        &self,
        repo: &String,
        user: &UserElement,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>> {
        let endpoint = format!(
            "https://{}/api/v1/repository/{}/{}/permissions/user/{}",
            &self.quay_endpoint, &self.quay_organization, repo, user.name
        );
        let mut body: HashMap<&str, &String> = HashMap::new();
        body.insert("role", &user.role);

        let response = &self
            .send_request(
                endpoint,
                &body,
                &self.quay_organization,
                Method::DELETE,
                quay_fn_arguments,
            )
            .await?;

        Ok(response.clone())
    }
    async fn delete_team_permission_from_repository(
        &self,
        repo: &String,
        user: &UserElement,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>> {
        let endpoint = format!(
            "https://{}/api/v1/repository/{}/{}/permissions/team/{}",
            &self.quay_endpoint, &self.quay_organization, repo, user.name
        );
        let mut body: HashMap<&str, &String> = HashMap::new();
        body.insert("role", &user.role);

        let response = &self
            .send_request(
                endpoint,
                &body,
                &self.quay_organization,
                Method::DELETE,
                quay_fn_arguments,
            )
            .await?;

        Ok(response.clone())
    }

    async fn get_user_permission_from_repository(
        &self,
        repo: &Repository,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>> {
        let endpoint = format!(
            "https://{}/api/v1/repository/{}/{}/permissions/user/",
            &self.quay_endpoint, &self.quay_organization, repo.name,
        );
        let body: HashMap<&str, &String> = HashMap::new();
        let response = &self
            .send_request(
                endpoint,
                &body,
                &self.quay_organization,
                Method::GET,
                quay_fn_arguments.clone(),
            )
            .await?;

        ////println!("{} - {}", &self.quay_organization, repo.name);
        match response.status_code {
            StatusCode::OK => {
                let mut actual_repo_permissions: Permissions = Permissions::new();
                //println!();
                //println!("####################");
                //println!(
                //   "Organization {} :: Repository: {} ",
                //    &self.quay_organization, repo.name
                //);
                //println!("####################");
                //println!();
                //For users and robots
                if let Some(objs) = response.response.as_object() {
                    if let Some(objs_permissions) = objs["permissions"].as_object() {
                        for (_, v) in objs_permissions.iter() {
                            if let Some(name) = v["name"].as_str() {
                                if let Some(role) = v["role"].as_str() {
                                    if let Some(is_robot) = v["is_robot"].as_bool() {
                                        if is_robot {
                                            let single_robot_permission = UserElement::new(
                                                name.to_string()
                                                    .substring(
                                                        self.quay_organization.len() + 1,
                                                        name.len(),
                                                    )
                                                    .to_string(),
                                                role.to_string(),
                                            );
                                            actual_repo_permissions
                                                .robots
                                                .push(single_robot_permission);
                                        } else {
                                            let single_user_permission = UserElement::new(
                                                name.to_string(),
                                                role.to_string(),
                                            );
                                            actual_repo_permissions
                                                .users
                                                .push(single_user_permission);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                ////println!("Actual permissions: {:?}", actual_repo_permissions);
                //println!("---------");
                let configured_repository = self.repositories.iter().find(|r| r == &repo);
                ////println!("{:?}",configured_repository);
                match configured_repository {
                    Some(configured_repo) => {
                        let mut diff_users: Vec<UserElement> = actual_repo_permissions.users;
                        //println!("Actual USERS permissions {:?}", diff_users);

                        if let Some(user) = configured_repo.permissions.as_ref() {
                            for el_permission in &user.users {
                                diff_users.retain(|x| x != el_permission);
                            }
                            //println!("Wanted USERS permissions {:?}", &user.users);
                        } else {
                            //println!("Wanted USERS permissions: NONE");
                            //If there is not a wanted user, Quay adds a single admin user so the difference must be zero.
                            if diff_users.len() > 0 {
                                diff_users.clear();
                                //println!("--> The admin user is not being counted.")
                            }
                        }

                        //println!("Difference USER permissions {:?}", diff_users);

                        for user in diff_users {
                            self.delete_user_permission_from_repository(
                                &repo.name,
                                &user,
                                quay_fn_arguments.clone(),
                            )
                            .await?;
                        }

                        //println!();

                        let mut diff_robots: Vec<UserElement> = actual_repo_permissions.robots;
                        debug!("Actual ROBOTS permissions {:?}", diff_robots);

                        if let Some(user) = configured_repo.permissions.as_ref() {
                            for el_permission in &user.robots {
                                diff_robots.retain(|x| x != el_permission);
                            }
                            //println!("Wanted ROBOT permissions {:?}", &user.robots);
                        } else {
                            //println!("Wanted ROBOT permissions: NONE");
                        }

                        //println!("Difference ROBOTS permissions {:?}", diff_robots);

                        //Fix the robot name
                        let diff_fixed_robots = diff_robots.iter().map(|robot| UserElement {
                            name: format!("{}+{}", &self.quay_organization, robot.name),
                            role: robot.role.to_owned(),
                        });
                        for robot in diff_fixed_robots {
                            self.delete_user_permission_from_repository(
                                &repo.name,
                                &robot,
                                quay_fn_arguments.clone(),
                            )
                            .await?;
                        }
                        //delete_user_permission_to_repository
                    }
                    None => {}
                }
            }
            _ => {}
        }

        Ok(response.clone())
    }

    async fn get_team_permission_from_repository(
        &self,
        repo: &Repository,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>> {
        let endpoint = format!(
            "https://{}/api/v1/repository/{}/{}/permissions/team/",
            &self.quay_endpoint, &self.quay_organization, repo.name,
        );
        let body: HashMap<&str, &String> = HashMap::new();
        let response = &self
            .send_request(
                endpoint,
                &body,
                &self.quay_organization,
                Method::GET,
                quay_fn_arguments.clone(),
            )
            .await?;

        ////println!("{} - {}", &self.quay_organization, repo.name);
        match response.status_code {
            StatusCode::OK => {
                let mut actual_repo_permissions = Vec::new();
                //println!();
                //println!("####################");
                //println!(
                //    "Organization {} :: Repository: {} ",
                //    &self.quay_organization, repo.name
                // );
                //println!("####################");
                //println!();
                //For team
                if let Some(objs) = response.response.as_object() {
                    if let Some(objs_permissions) = objs["permissions"].as_object() {
                        for (_, v) in objs_permissions.iter() {
                            if let Some(name) = v["name"].as_str() {
                                if let Some(role) = v["role"].as_str() {
                                    let single_team_permission =
                                        UserElement::new(name.to_string(), role.to_string());
                                    actual_repo_permissions.push(single_team_permission);
                                }
                            }
                        }
                    }
                }

                //println!("Actual permissions: {:?}", actual_repo_permissions);
                //println!("---------");
                let configured_repository = self.repositories.iter().find(|r| r == &repo);
                ////println!("{:?}",configured_repository);
                match configured_repository {
                    Some(configured_repo) => {
                        //println!("Actual TEAMS permissions {:?}", actual_repo_permissions);

                        let mut diff_teams = actual_repo_permissions;
                        if let Some(user) = configured_repo.permissions.as_ref() {
                            if let Some(teams) = &user.teams {
                                for el_permission in teams {
                                    diff_teams.retain(|x| x != el_permission);
                                }
                                //println!("Wanted TEAMS permissions {:?}", &user.teams);
                            }
                        }

                        //println!("Difference TEAMS permissions {:?}", diff_teams);

                        for team in diff_teams {
                            self.delete_team_permission_from_repository(
                                &repo.name,
                                &team,
                                quay_fn_arguments.clone(),
                            )
                            .await?;
                        }

                        //println!();
                    }
                    None => {
                        //println!("No teams present")
                    }
                }
            }
            _ => {}
        }

        Ok(response.clone())
    }

    async fn grant_robot_permission_to_repository(
        &self,
        repo: &String,
        user: &UserElement,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>> {
        let endpoint = format!(
            "https://{}/api/v1/repository/{}/{}/permissions/user/{}",
            &self.quay_endpoint,
            &self.quay_organization,
            repo,
            format!("{}+{}", &self.quay_organization, user.name)
        );
        let mut body = HashMap::new();
        body.insert("role", &user.role);

        let response = &self
            .send_request(
                endpoint,
                &body,
                &self.quay_organization,
                Method::PUT,
                quay_fn_arguments,
            )
            .await?;

        Ok(response.clone())
    }
    async fn grant_team_permission_to_repository(
        &self,
        repo: &String,
        user: &UserElement,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>> {
        let endpoint = format!(
            "https://{}/api/v1/repository/{}/{}/permissions/team/{}",
            &self.quay_endpoint, &self.quay_organization, repo, user.name
        );
        let mut body = HashMap::new();
        body.insert("role", &user.role);

        let response = &self
            .send_request(
                endpoint,
                &body,
                &self.quay_organization,
                Method::PUT,
                quay_fn_arguments,
            )
            .await?;

        Ok(response.clone())
    }
    async fn delete_organization(
        &self,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>> {
        let endpoint = format!(
            "https://{}/api/v1/organization/{}",
            &self.quay_endpoint, &self.quay_organization
        );
        let body: HashMap<&str, &String> = HashMap::new();

        let response = &self
            .send_request(
                endpoint,
                &body,
                &self.quay_organization,
                Method::DELETE,
                quay_fn_arguments,
            )
            .await?;

        Ok(response.clone())
    }
    async fn create_robot(
        &self,
        robot: &RobotDetails,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>> {
        let endpoint = format!(
            "https://{}/api/v1/organization/{}/robots/{}",
            &self.quay_endpoint, &self.quay_organization, robot.name
        );
        let mut body: HashMap<&str, &String> = HashMap::new();

        body.insert("description", &robot.desc);

        let description = format!(
            "Creating robot '{}' for organization '{}'",
            robot.name, &self.quay_organization
        );
        let response = &self
            .send_request(
                endpoint,
                &body,
                &description,
                Method::PUT,
                quay_fn_arguments,
            )
            .await?;

        Ok(response.clone())
    }

    async fn create_team(
        &self,
        team: &Team,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>> {
        let endpoint = format!(
            "https://{}/api/v1/organization/{}/team/{}",
            &self.quay_endpoint, &self.quay_organization, team.name
        );
        let mut body = HashMap::new();

        body.insert("description", &team.description);
        body.insert("role", &team.role);
        //body.insert("unstructured_metadata", empty);

        let description = format!(
            "Creating team '{}' for organization '{}'",
            team.name, &self.quay_organization
        );
        let response = &self
            .send_request(
                endpoint,
                &body,
                &description,
                Method::PUT,
                quay_fn_arguments,
            )
            .await?;

        Ok(response.clone())
    }

    async fn add_user_to_team(
        &self,
        team: &String,
        user: &String,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>> {
        let endpoint = format!(
            "https://{}/api/v1/organization/{}/team/{}/members/{}",
            &self.quay_endpoint, &self.quay_organization, team, user
        );
        let body: HashMap<&str, &String> = HashMap::new();

        let response = &self
            .send_request(
                endpoint,
                &body,
                &self.quay_organization,
                Method::PUT,
                quay_fn_arguments,
            )
            .await?;

        Ok(response.clone())
    }

    async fn add_robot_to_team(
        &self,
        team: &String,
        robot: &String,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>> {
        let endpoint = format!(
            "https://{}/api/v1/organization/{}/team/{}/members/{}",
            &self.quay_endpoint,
            &self.quay_organization,
            team,
            format!("{}+{}", &self.quay_organization, robot)
        );
        let body: HashMap<&str, &String> = HashMap::new();

        let response = &self
            .send_request(
                endpoint,
                &body,
                &self.quay_organization,
                Method::PUT,
                quay_fn_arguments,
            )
            .await?;

        Ok(response.clone())
    }
    async fn create_repository(
        &self,
        repo: &Repository,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>> {
        let endpoint = format!("https://{}/api/v1/repository", &self.quay_endpoint,);
        let mut body: HashMap<&str, &String> = HashMap::new();

        let repo_kind = String::from("image");
        let empty = String::from("");
        let desc = repo.description.as_ref().unwrap_or(&empty);
        let default_visibility = String::from("public");
        body.insert("description", desc);
        body.insert("repo_kind", &repo_kind);
        body.insert("namespace", &self.quay_organization);
        body.insert("repository", &repo.name);
        body.insert(
            "visibility",
            repo.visibility.as_ref().unwrap_or(&default_visibility),
        );

        //body.insert("unstructured_metadata", empty);

        let description = format!(
            "Creating repository '{}' for organization '{}'",
            repo.name, &self.quay_organization
        );
        let response = &self
            .send_request(
                endpoint,
                &body,
                &description,
                Method::POST,
                quay_fn_arguments,
            )
            .await?;

        Ok(response.clone())
    }

    async fn create_repository_mirror(
        &self,
        repo: &Repository,
        quay_fn_arguments: QuayFnArguments,
    ) -> Result<QuayResponse, Box<dyn Error>> {
        let endpoint = format!(
            "https://{}/api/v1/repository/{}/{}/mirror",
            &self.quay_endpoint, &self.quay_organization, repo.name
        );

        //println!("{}", endpoint);

        match &repo.mirror_params {
            Some(params) => {
                let proxy_configuration = QuayMirrorProxy {
                    http_proxy: params.http_proxy.clone(),
                    https_proxy: params.https_proxy.clone(),
                    no_proxy: params.no_proxy.clone(),
                };

                //println!("{:?}",params);

                let external_registry_config = ExternalRegistryConfig {
                    verify_tls: params.ext_registry_verify_tls,
                    unsigned_images: params.ext_registry_unsigned_image.unwrap_or_default(),
                    proxy: proxy_configuration,
                };

                //println!("{:?}",external_registry_config);

                let root_rule = RootRule {
                    rule_kind: "tag_glob_csv".to_string(),
                    rule_value: params.src_image_tags.clone(),
                };

                let now: DateTime<Utc> = Utc::now();

                //let date_format = format("{}-{}-{}T{}:{}:{}",now.);

                let formatted = format!("{}", now.format("%Y-%m-%dT%H:%M:%Sz"));

                let mut external_registry_password: Option<String> = None;
                // Get the appropriate repository mirror password

                match &quay_fn_arguments.mirror_login {
                    Some(mirrors) => {
                        for mirror in mirrors {
                            if Some(mirror.ext_registry_username.clone())
                                == params.ext_registry_username.clone()
                            {
                                if mirror.repository == repo.name {
                                    if mirror.organization == self.quay_organization {
                                        external_registry_password =
                                            Some(mirror.ext_registry_password.clone());
                                    }
                                }
                            }
                        }
                    }
                    None => {}
                }

                let body = MirrorConfig {
                    external_reference: format!(
                        "{}/{}",
                        params.src_registry,
                        params.src_image.clone()
                    ),
                    external_registry_password,
                    external_registry_username: params.ext_registry_username.clone(),
                    sync_interval: params.sync_interval,
                    sync_start_date: formatted,
                    //sync_start_date: "2023-01-22T06:28:00Z".to_string(),
                    robot_username: format!(
                        "{}+{}",
                        &self.quay_organization,
                        params.robot_username.clone()
                    ),
                    external_registry_config,
                    root_rule,
                };

                //println!("{}", serde_json::to_string(&body).unwrap());

                let description = format!(
                    "Configuring mirror for repository '{}' for organization '{}'",
                    repo.name, &self.quay_organization
                );

                //Change repository state to mirror

                let endpoint_state = format!(
                    "https://{}/api/v1/repository/{}/{}/changestate",
                    &self.quay_endpoint, &self.quay_organization, repo.name
                );

                let mut body_state: HashMap<&str, &str> = HashMap::new();

                body_state.insert("state", "MIRROR");

                let _response = &self
                    .send_request(
                        endpoint_state,
                        &body_state,
                        &description,
                        Method::PUT,
                        quay_fn_arguments.clone(),
                    )
                    .await?;

                let response = &self
                    .send_request(
                        endpoint.clone(),
                        &body,
                        &description,
                        Method::POST,
                        quay_fn_arguments.clone(),
                    )
                    .await?;

                if response.status_code == StatusCode::CONFLICT {
                    //println!("Mirror configuration already exists, updating...");

                    let response_put = &self
                        .send_request(
                            endpoint,
                            &body,
                            &description,
                            Method::PUT,
                            quay_fn_arguments,
                        )
                        .await?;

                    return Ok(response_put.clone());
                }

                return Ok(response.clone());
            }

            None => {
                let response = QuayResponse {
                    response: Value::Null,
                    description: String::from("Mirroring disabled"),
                    status_code: StatusCode::OK,
                };
                return Ok(response.clone());
            }
        }

        //body.insert("unstructured_metadata", empty);
    }
}

impl OrganizationYaml {
    pub fn change_endpoint(&mut self, endpoint: String) {
        self.quay_endpoint = endpoint;
    }

    pub fn get_quay_endpoint(&self) -> String {
        self.quay_endpoint.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OrganizationYaml {
    #[serde(rename = "quay_endpoint")]
    pub quay_endpoint: String,

    #[serde(rename = "replicate_to")]
    pub replicate_to: Option<Vec<String>>,

    #[serde(rename = "quay_validate_certs")]
    quay_validate_certs: String,

    #[serde(rename = "quay_organization")]
    pub quay_organization: String,

    #[serde(rename = "quay_organization_role_name")]
    quay_organization_role_name: String,

    #[serde(rename = "quay_organization_role_email")]
    quay_organization_role_email: String,

    #[serde(rename = "repositories")]
    pub repositories: Vec<Repository>,

    #[serde(rename = "robots")]
    pub robots: Vec<RobotDetails>,

    #[serde(rename = "teams")]
    pub teams: Vec<Team>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Repository {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    description: Option<String>,

    #[serde(rename = "visibility")]
    visibility: Option<String>,

    #[serde(rename = "mirror")]
    mirror: bool,

    #[serde(rename = "mirror_params")]
    pub mirror_params: Option<MirrorParams>,

    #[serde(rename = "permissions")]
    pub permissions: Option<Permissions>,
}
/*
{
    "external_reference": "quay.io/valeube/hello-openshift",
    "external_registry_username": "valeidm",
    "external_registry_password": "casafu11",
    "sync_interval": 86400,
    "sync_start_date": "2023-01-21T06:13:00Z",
    "robot_username": "44444444+robot",
    "external_registry_config": {
        "verify_tls": true,
        "unsigned_images": true,
        "proxy": {
            "http_proxy": null,
            "https_proxy": null,
            "no_proxy": null
        }
    },
    "root_rule": {
        "rule_kind": "tag_glob_csv",
        "rule_value": [
            "latest"
        ]
    }
}
*/

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MirrorConfig {
    #[serde(rename = "external_reference")]
    pub external_reference: String,
    #[serde(rename = "external_registry_username")]
    pub external_registry_username: Option<String>,
    #[serde(rename = "external_registry_password")]
    pub external_registry_password: Option<String>,
    #[serde(rename = "sync_interval")]
    pub sync_interval: i64,
    #[serde(rename = "sync_start_date")]
    pub sync_start_date: String,
    #[serde(rename = "robot_username")]
    pub robot_username: String,
    #[serde(rename = "external_registry_config")]
    pub external_registry_config: ExternalRegistryConfig,
    #[serde(rename = "root_rule")]
    pub root_rule: RootRule,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalRegistryConfig {
    #[serde(rename = "verify_tls")]
    pub verify_tls: bool,
    #[serde(rename = "unsigned_images")]
    pub unsigned_images: bool,
    pub proxy: QuayMirrorProxy,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuayMirrorProxy {
    #[serde(rename = "http_proxy")]
    pub http_proxy: Option<String>,
    #[serde(rename = "https_proxy")]
    pub https_proxy: Option<String>,
    #[serde(rename = "no_proxy")]
    pub no_proxy: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootRule {
    #[serde(rename = "rule_kind")]
    pub rule_kind: String,
    #[serde(rename = "rule_value")]
    pub rule_value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MirrorParams {
    #[serde(rename = "src_registry")]
    src_registry: String,

    #[serde(rename = "src_image")]
    src_image: String,

    #[serde(rename = "src_image_tags")]
    src_image_tags: Vec<String>,

    #[serde(rename = "ext_registry_verify_tls")]
    ext_registry_verify_tls: bool,

    #[serde(rename = "ext_registry_unsigned_image")]
    ext_registry_unsigned_image: Option<bool>,

    #[serde(rename = "robot_username")]
    robot_username: String,

    #[serde(rename = "sync_interval")]
    sync_interval: i64,

    #[serde(rename = "is_enabled")]
    is_enabled: bool,

    #[serde(rename = "https_proxy")]
    https_proxy: Option<String>,

    #[serde(rename = "http_proxy")]
    http_proxy: Option<String>,

    #[serde(rename = "no_proxy")]
    no_proxy: Option<String>,

    #[serde(rename = "ext_registry_username")]
    pub ext_registry_username: Option<String>,

    #[serde(rename = "ext_registry_password")]
    ext_registry_password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Permissions {
    #[serde(rename = "robots")]
    pub robots: Vec<UserElement>,

    #[serde(rename = "users")]
    pub users: Vec<UserElement>,

    #[serde(rename = "teams")]
    pub teams: Option<Vec<UserElement>>,
}

impl Permissions {
    pub fn new() -> Permissions {
        Permissions {
            robots: Vec::new(),
            users: Vec::new(),
            teams: Some(Vec::new()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UserElement {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "role")]
    pub role: String,
}

impl UserElement {
    pub fn new(name: String, role: String) -> UserElement {
        UserElement { name, role }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RobotDetails {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "desc")]
    pub desc: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Team {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "members")]
    pub members: Members,

    #[serde(rename = "role")]
    role: String,
}

/// Repository's member structs.
/// In the Quay context, a 'member' could be a user or a robot.
///
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Members {
    #[serde(rename = "users")]
    pub users: Vec<String>,

    #[serde(rename = "robots")]
    pub robots: Vec<String>,
}

/// Helper struct to pass arguments to functions.
/// Useful for easily extend accepted function arguments.
#[derive(Debug, Clone)]
pub struct QuayFnArguments {
    /// Quay token
    pub(crate) token: String,
    /// Rate limiter
    pub governor:
        Arc<RateLimiter<NotKeyed, InMemoryState, QuantaClock, NoOpMiddleware<QuantaInstant>>>,
    /// Log level
    pub log_level: log::Level,
    /// Log verbosity
    pub log_verbosity: u8,
    /// Connection timeout in seconds
    pub timeout: u64,
    /// Verify Quay tls
    pub tls_verify: bool,

    pub mirror_login: Option<Vec<MirrorLogin>>,
}

#[derive(Debug, Clone)]
pub struct QuayFnArgumentsMirrorLogin {
    pub repo: String,
    pub user: String,
    pub password: String,
}
