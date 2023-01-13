use super::organization_struct::organization_struct::OrganizationYaml;
use std::fs::File;
use tokio::fs::read_dir;


#[derive(Debug, Default)]
pub struct QuayXmlConfig {
    organization: Vec<OrganizationYaml>,
    directory: String,
}

impl QuayXmlConfig {
    pub fn new(directory: String) -> Self {
       
        Self {
            organization: vec![],
            directory,
        }
    }

    pub async fn load_config(&mut self) -> Result<(), std::io::Error> {
        
        let mut files = read_dir(self.directory.to_owned()).await?;

        while let Some(f) = files.next_entry().await? {
            //println!("{:?}: {}", f.file_name(), f.ino());
            let f = File::open(f.path())?;
            let scrape_config: OrganizationYaml =
                serde_yaml::from_reader(f).expect("Could not read values.");
            self.organization.push(scrape_config);
        }

        Ok(())
    }

    pub fn get_organizations(&self) -> &Vec<OrganizationYaml> {
        &self.organization
    }
    
}
