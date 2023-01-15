pub mod organization_struct {
    use std::{collections::HashMap, error::Error, time::Duration};

    use async_trait::async_trait;

    use reqwest::StatusCode;
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
        async fn create_robot(&self,robot: &RobotDetails) -> Result<QuayResponse, Box<dyn Error>>;
        async fn delete_organization(&self, token: String) -> bool;
        async fn send_post_request(
            &self,
            endpoint: String,
            body: HashMap<&str, &String>,
            token: &String,
            description: &String,
        ) -> Result<QuayResponse, Box<dyn Error>> {
            let api = reqwest::Client::new()
                .post(endpoint)
                .timeout(Duration::from_secs(5))
                .header("Content-Type", "application/json")
                .header("accept", "application/json")
                .header("Authorization", format!("Bearer {}", &token))
                .json(&body);

            let response_status = api.send().await?;
            let status_code = response_status.status();
            let response = response_status.json::<serde_json::Value>().await?;
            //println!("{}", status_code);
            let quay_response = QuayResponse {
                response,
                status_code,
                description: description.clone(),
            };
            Ok(quay_response)
        }
        async fn send_put_request(
            &self,
            endpoint: String,
            body: HashMap<&str, &String>,
            token: &String,
            description: &String,
        ) -> Result<QuayResponse, Box<dyn Error>> {
            let api = reqwest::Client::new()
                .put(endpoint)
                .timeout(Duration::from_secs(5))
                .header("Content-Type", "application/json")
                .header("accept", "application/json")
                .header("Authorization", format!("Bearer {}", &token))
                .json(&body);

            let response_status = api.send().await?;
            let status_code = response_status.status();
            let response = response_status.json::<serde_json::Value>().await?;
            //println!("{}", status_code);
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
        async fn create_organization(&self) -> Result<QuayResponse, Box<dyn Error>> {
            let endpoint = format!("https://{}/api/v1/organization/", &self.quay_endpoint);
            let mut body = HashMap::new();
            body.insert("name", &self.quay_organization);
            body.insert("email", &self.quay_organization_role_email);

            
            let response = &self
                .send_post_request(
                    endpoint,
                    body,
                    &self.quay_oauth_token,
                    &self.quay_organization,
                )
                .await?;

            //println!("{:?}", response);

            Ok(response.clone())
        }
        async fn create_robot(&self, robot: &RobotDetails) -> Result<QuayResponse, Box<dyn Error>> {
            let endpoint = format!("https://{}/api/v1/organization/{}/robots/{}", &self.quay_endpoint,&self.quay_organization,robot.name);
            let mut body = HashMap::new();
            
            let empty= &String::from("{}");
            body.insert("description", &robot.desc);
            //body.insert("unstructured_metadata", empty);

            let description =format!("Creating robot '{}' for organization '{}'",robot.name,&self.quay_organization);
            let response = &self
                .send_put_request(
                    endpoint,
                    body,
                    &self.quay_oauth_token,
                    &description,
                )
                .await?;

            //println!("{:?}", response);

            Ok(response.clone())
        }

        async fn delete_organization(&self, token: String) -> bool {
            println!("Delete");
            true
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
        repositories: Vec<Repository>,

        #[serde(rename = "robots")]
        pub robots: Vec<RobotDetails>,

        #[serde(rename = "teams")]
        teams: Vec<Team>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Repository {
        #[serde(rename = "name")]
        name: String,

        #[serde(rename = "description")]
        description: Option<String>,

        #[serde(rename = "mirror")]
        mirror: bool,

        #[serde(rename = "mirror_params")]
        mirror_params: Option<MirrorParams>,

        #[serde(rename = "permissions")]
        permissions: Option<Permissions>,
    }

    #[derive(Serialize, Deserialize, Debug)]
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

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Permissions {
        #[serde(rename = "robots")]
        robots: Vec<UserElement>,

        #[serde(rename = "users")]
        users: Vec<UserElement>,

        #[serde(rename = "teams")]
        teams: Option<Vec<UserElement>>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserElement {
        #[serde(rename = "name")]
        name: String,

        #[serde(rename = "role")]
        role: String,
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
        name: String,

        #[serde(rename = "description")]
        description: String,

        #[serde(rename = "members")]
        members: Members,

        #[serde(rename = "role")]
        role: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Members {
        #[serde(rename = "users")]
        users: Vec<String>,

        #[serde(rename = "robots")]
        robots: Vec<String>,
    }
}
