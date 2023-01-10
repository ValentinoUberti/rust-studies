use clap::Parser;



#[derive(Parser, Debug)]
#[command(author, version, about="Quay batch processing cli written in Rust", long_about = None)]
#[command(help_template("\
{before-help}{name} {version} - {author-with-newline}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
"))]
struct Args {
    #[arg(short, long)]
    dir: String,

    #[arg(short, long)]
    authfile: String,
}

fn main() {
   
    let args = Args::parse();
    
    println!("{}",args.dir);

}