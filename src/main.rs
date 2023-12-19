mod definitions;
mod docker;
mod visitors;
mod driver;

use clap::{Parser};
use definitions::program_configuration::{*};
use crate::docker::{*};
use crate::visitors::generate_mark_down::GenerateMarkDownVisitor;
use crate::visitors::generate_setup_environment::GenerateSetupEnvironmentVisitor;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    filename: String,
}


fn main()  {
    let args : Args = Args::parse();
    let f = std::fs::File::open(&args.filename).expect("Could not open file.");
    let mut scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
    let mut markdown_visitor = GenerateMarkDownVisitor::from_filename("config2.md".to_string());
    let mut setup_generation_visitor = GenerateSetupEnvironmentVisitor::new();
    scrape_config.accept(&mut markdown_visitor);
    scrape_config.accept(&mut setup_generation_visitor );


    //println!("{:?}",get_container_ip("3a27f596c74716c00de02961178a64f2da0ceef4bffb116918bf675f7e575f3b"));
    //let res = create_mongo_db_container("test_mongo","mongo","fzuimg57","8082:27017");
    //println!("{:?}",res);

}
