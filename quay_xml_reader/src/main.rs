use std::fs::File;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Serialize, Deserialize)]
pub struct OrganizationYaml {
    #[serde(rename = "quay_endpoint")]
    quay_endpoint: String,

    #[serde(rename = "quay_oauth_token")]
    quay_oauth_token: String,

    #[serde(rename = "quay_validate_certs")]
    quay_validate_certs: String,

    #[serde(rename = "quay_organization")]
    quay_organization: String,

    #[serde(rename = "quay_organization_role_name")]
    quay_organization_role_name: String,

    #[serde(rename = "quay_organization_role_email")]
    quay_organization_role_email: String,

    #[serde(rename = "repositories")]
    repositories: Vec<Repository>,

    #[serde(rename = "robots")]
    robots: Vec<RobotDetails>,

    #[serde(rename = "teams")]
    teams: Vec<Team>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Permissions {
    #[serde(rename = "robots")]
    robots: Vec<UserElement>,

    #[serde(rename = "users")]
    users: Vec<UserElement>,

    #[serde(rename = "teams")]
    teams: Option<Vec<UserElement>>,
}

#[derive(Serialize, Deserialize)]
pub struct UserElement {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "role")]
    role: String,
}

#[derive(Serialize, Deserialize)]
pub struct RobotDetails {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "desc")]
    desc: String,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Members {
    #[serde(rename = "users")]
    users: Vec<String>,

    #[serde(rename = "robots")]
    robots: Vec<String>,
}

fn main() -> std::io::Result<()> {
    // Open files
    let f = File::open("xml-files/example-organization.yaml")?;
   
    let scrape_config: OrganizationYaml = serde_yaml::from_reader(f).expect("Could not read values.");

    println!("{}",scrape_config.quay_endpoint);
    for repo in scrape_config.repositories {
        println!("{:#?}",repo.name);
    }
    
    
    
   /*
    match yamldoc {
        Ok(doc) => { 
            println!("Parsing succeded: ");
            extracted = &doc[0];
            
        }
            ,
        Err(e) => {
            println!("Error parsing: {}", e.to_string());
            
        }
    };

    
    println!("{}",extracted.);

    //println!("{}",contents);
  */
    Ok(())
}
