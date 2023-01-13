mod quay_config_reader;
use std::error::Error;
use clap::Parser;

use crate::quay_config_reader::quay_config_reader::QuayXmlConfig;

#[derive(Parser, Debug)]
#[command(author, version, about="Quay batch processing cli written in Rust", long_about = None)]
#[command(help_template(
    "\
{before-help}{name} {version} - {author-with-newline}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
"
))]
struct Args {
    #[arg(short, long)]
    dir: String,

    #[arg(short, long)]
    authfile: String,
}

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {
    let args = Args::parse();
    let mut config = QuayXmlConfig::new(args.dir);
    config.load_config().await?;

    for org in config.get_organizations() {
        println!("Org name: {}",org.quay_organization)
    }

    Ok(())
}
