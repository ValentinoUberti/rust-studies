use std::{error::Error, fs::File};
use super::organization_struct::{organization_struct::OrganizationYaml};


#[derive(Debug, Default)]
pub struct QuayXmlConfig {
    organization: Vec<String>,
    directory: String,
}

impl QuayXmlConfig {
    pub fn new(directory: String) -> Result<Self, Box<dyn Error>> {
       
        let f = File::open("xml-files/example-organization.yaml")?;
        let scrape_config: OrganizationYaml = serde_yaml::from_reader(f).expect("Could not read values.");


        Ok(Self {
            organization: vec![],
            directory,
        })
    }

    pub fn get_dir(&self) -> String {
        self.directory.clone()
    }
}
