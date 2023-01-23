use super::organization_struct::organization_struct::OrganizationYaml;
use governor::clock::{QuantaClock, QuantaInstant};
use governor::middleware::NoOpMiddleware;
use governor::state::{InMemoryState, NotKeyed};
use governor::{self, Quota, RateLimiter};
use std::num::NonZeroU32;
use std::{fs::File, sync::Arc};
use tokio::fs::read_dir;

#[derive(Debug)]
pub struct QuayXmlConfig {
    organization: Vec<OrganizationYaml>,
    directory: String,
    governor: Arc<RateLimiter<NotKeyed, InMemoryState, QuantaClock, NoOpMiddleware<QuantaInstant>>>,
}

impl QuayXmlConfig {
    pub fn new(directory: String, req_per_seconds: u32) -> Self {
        let governor = Arc::new(governor::RateLimiter::direct(governor::Quota::per_minute(
            NonZeroU32::new(req_per_seconds).unwrap(),
        )));
        Self {
            organization: vec![],
            directory,
            governor,
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

    pub fn get_cloned_governor(&self) -> Arc<RateLimiter<NotKeyed, InMemoryState, QuantaClock, NoOpMiddleware<QuantaInstant>>> {
        self.governor.clone()
    }

}
