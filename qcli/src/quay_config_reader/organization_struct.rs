pub mod organization_struct {
    use std::{collections::HashMap, error::Error, time::Duration};

    use async_trait::async_trait;

    use futures::SinkExt;
    use reqwest::{Method, StatusCode};
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    #[derive(Debug, Default, Clone)]
    pub struct QuayResponse {
        pub response: Value,
        pub status_code: StatusCode,
        pub description: String,
    }

    #[async_trait]
    pub trait Actions {
        async fn create_organization(&self) -> Result<QuayResponse, Box<dyn Error>>;
        async fn grant_user_permission_to_repository(
            &self,
            repo: &String,
            user: &UserElement,
        ) -> Result<QuayResponse, Box<dyn Error>>;
        async fn delete_extra_user_permission_from_repository(
            &self,
            repo: &Repository,
        ) -> Result<QuayResponse, Box<dyn Error>>;
        async fn grant_robot_permission_to_repository(
            &self,
            repo: &String,
            user: &UserElement,
        ) -> Result<QuayResponse, Box<dyn Error>>;
        async fn grant_team_permission_to_repository(
            &self,
            repo: &String,
            user: &UserElement,
        ) -> Result<QuayResponse, Box<dyn Error>>;
        async fn delete_organization(&self) -> Result<QuayResponse, Box<dyn Error>>;
        async fn create_robot(&self, robot: &RobotDetails) -> Result<QuayResponse, Box<dyn Error>>;
        async fn create_team(&self, team: &Team) -> Result<QuayResponse, Box<dyn Error>>;
        async fn add_user_to_team(
            &self,
            team: &String,
            user: &String,
        ) -> Result<QuayResponse, Box<dyn Error>>;
        async fn add_robot_to_team(
            &self,
            team: &String,
            user: &String,
        ) -> Result<QuayResponse, Box<dyn Error>>;
        async fn create_repository(
            &self,
            team: &Repository,
        ) -> Result<QuayResponse, Box<dyn Error>>;

        async fn send_request(
            &self,
            endpoint: String,
            body: HashMap<&str, &String>,
            token: &String,
            description: &String,
            method: reqwest::Method,
        ) -> Result<QuayResponse, Box<dyn Error>> {
            let api = reqwest::Client::new()
                .request(method, endpoint)
                .timeout(Duration::from_secs(5))
                .header("Content-Type", "application/json")
                .header("accept", "application/json")
                .header("Authorization", format!("Bearer {}", &token))
                .json(&body);

            //println!("{:?}", api);
            let response_status = api.send().await?;
            let status_code = response_status.status();
            let response = match response_status.json::<serde_json::Value>().await {
                Ok(r) => r,
                Err(_) => Value::Null,
            };
            //println!("{:?}", response);
            let quay_response = QuayResponse {
                response,
                status_code,
                description: description.clone(),
            };
            Ok(quay_response)
        }
    }

    #[async_trait]
    impl Actions for OrganizationYaml {
        async fn delete_extra_user_permission_from_repository(
            &self,
            repo: &Repository,
        ) -> Result<QuayResponse, Box<dyn Error>> {
            let endpoint = format!(
                "https://{}/api/v1/repository/{}/{}/permissions/user/",
                &self.quay_endpoint, &self.quay_organization, repo.name,
            );
            let body = HashMap::new();
            let response = &self
                .send_request(
                    endpoint,
                    body,
                    &self.quay_oauth_token,
                    &self.quay_organization,
                    Method::GET,
                )
                .await?;

            println!("{} - {}", &self.quay_organization, repo.name);
            match response.status_code {
                StatusCode::OK => {
                    let mut actual_repo_permissions: Permissions = Permissions::new();

                    //For users and robots
                    if let Some(objs) = response.response.as_object() {
                        if let Some(objs_permissions) = objs["permissions"].as_object() {
                            for (_, v) in objs_permissions.iter() {
                                if let Some(name) = v["name"].as_str() {
                                    if let Some(role) = v["role"].as_str() {
                                        if let Some(is_robot) = v["is_robot"].as_bool() {
                                            if is_robot {
                                                let single_robot_permission = UserElement::new(
                                                    name.to_string(),
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

                    println!("{:?}", actual_repo_permissions);
                    println!("---------");
                    let configured_repository = self.repositories.iter().find(|r| r == &repo);
                    match configured_repository {
                        Some(configured_repo) => {
                            println!("Here {}",&self.quay_organization);
                            
                            let mut diff: Vec<UserElement> = actual_repo_permissions.users;
                            println!("{:?}",diff);
                            
                            match configured_repo.permissions.as_ref() {
                                Some(user) => {
                                    for el_permission in &user.users {
                                        diff.retain(|x| x != el_permission);
                                    }
                                    println!("{:?}",diff);
                                }
                                None => {}
                            }
                        }
                        None => {}
                    }
                }
                _ => {}
            }

            Ok(response.clone())
        }

        async fn add_user_to_team(
            &self,
            team: &String,
            user: &String,
        ) -> Result<QuayResponse, Box<dyn Error>> {
            let endpoint = format!(
                "https://{}/api/v1/organization/{}/team/{}/members/{}",
                &self.quay_endpoint, &self.quay_organization, team, user
            );
            let body = HashMap::new();

            let response = &self
                .send_request(
                    endpoint,
                    body,
                    &self.quay_oauth_token,
                    &self.quay_organization,
                    Method::PUT,
                )
                .await?;

            Ok(response.clone())
        }
        async fn add_robot_to_team(
            &self,
            team: &String,
            robot: &String,
        ) -> Result<QuayResponse, Box<dyn Error>> {
            let endpoint = format!(
                "https://{}/api/v1/organization/{}/team/{}/members/{}",
                &self.quay_endpoint,
                &self.quay_organization,
                team,
                format!("{}+{}", &self.quay_organization, robot)
            );
            let body = HashMap::new();

            let response = &self
                .send_request(
                    endpoint,
                    body,
                    &self.quay_oauth_token,
                    &self.quay_organization,
                    Method::PUT,
                )
                .await?;

            Ok(response.clone())
        }

        async fn grant_user_permission_to_repository(
            &self,
            repo: &String,
            user: &UserElement,
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
                    body,
                    &self.quay_oauth_token,
                    &self.quay_organization,
                    Method::PUT,
                )
                .await?;

            Ok(response.clone())
        }

        async fn grant_team_permission_to_repository(
            &self,
            repo: &String,
            user: &UserElement,
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
                    body,
                    &self.quay_oauth_token,
                    &self.quay_organization,
                    Method::PUT,
                )
                .await?;

            Ok(response.clone())
        }
        async fn grant_robot_permission_to_repository(
            &self,
            repo: &String,
            user: &UserElement,
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
                    body,
                    &self.quay_oauth_token,
                    &self.quay_organization,
                    Method::PUT,
                )
                .await?;

            Ok(response.clone())
        }
        async fn create_organization(&self) -> Result<QuayResponse, Box<dyn Error>> {
            let endpoint = format!("https://{}/api/v1/organization/", &self.quay_endpoint);
            let mut body = HashMap::new();
            body.insert("name", &self.quay_organization);
            body.insert("email", &self.quay_organization_role_email);

            let response = &self
                .send_request(
                    endpoint,
                    body,
                    &self.quay_oauth_token,
                    &self.quay_organization,
                    Method::POST,
                )
                .await?;

            Ok(response.clone())
        }

        async fn delete_organization(&self) -> Result<QuayResponse, Box<dyn Error>> {
            let endpoint = format!(
                "https://{}/api/v1/organization/{}",
                &self.quay_endpoint, &self.quay_organization
            );
            let body = HashMap::new();

            let response = &self
                .send_request(
                    endpoint,
                    body,
                    &self.quay_oauth_token,
                    &self.quay_organization,
                    Method::DELETE,
                )
                .await?;

            Ok(response.clone())
        }

        async fn create_robot(&self, robot: &RobotDetails) -> Result<QuayResponse, Box<dyn Error>> {
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
                    body,
                    &self.quay_oauth_token,
                    &description,
                    Method::PUT,
                )
                .await?;

            Ok(response.clone())
        }

        async fn create_team(&self, team: &Team) -> Result<QuayResponse, Box<dyn Error>> {
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
                    body,
                    &self.quay_oauth_token,
                    &description,
                    Method::PUT,
                )
                .await?;

            Ok(response.clone())
        }
        async fn create_repository(
            &self,
            repo: &Repository,
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
                    body,
                    &self.quay_oauth_token,
                    &description,
                    Method::POST,
                )
                .await?;

            Ok(response.clone())
        }
    }

    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct OrganizationYaml {
        #[serde(rename = "quay_endpoint")]
        quay_endpoint: String,

        #[serde(rename = "quay_oauth_token")]
        quay_oauth_token: String,

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

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
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
        mirror_params: Option<MirrorParams>,

        #[serde(rename = "permissions")]
        pub permissions: Option<Permissions>,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub struct MirrorParams {
        #[serde(rename = "src_registry")]
        src_registry: String,

        #[serde(rename = "src_image")]
        src_image: String,

        #[serde(rename = "src_image_tags")]
        src_image_tags: String,

        #[serde(rename = "ext_registry_verify_tls")]
        ext_registry_verify_tls: bool,

        #[serde(rename = "robot_username")]
        robot_username: String,

        #[serde(rename = "sync_interval")]
        sync_interval: i64,

        #[serde(rename = "is_enabled")]
        is_enabled: bool,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
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

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
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

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RobotDetails {
        #[serde(rename = "name")]
        pub name: String,

        #[serde(rename = "desc")]
        pub desc: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
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

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Members {
        #[serde(rename = "users")]
        pub users: Vec<String>,

        #[serde(rename = "robots")]
        pub robots: Vec<String>,
    }
}
