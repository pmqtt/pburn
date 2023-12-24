mod definitions;
mod docker;
mod driver;
mod visitors;
mod action;
mod network;
mod runtime;

use std::env;
use std::rc::Rc;
use crate::visitors::generate_mark_down::GenerateMarkDownVisitor;
use crate::visitors::generate_setup_environment::GenerateSetupEnvironmentVisitor;
use clap::Parser;
use definitions::program_configuration::*;
use crate::visitors::generate_communication_protocol::GenerateCommunicationProtocol;
use crate::visitors::generate_test::GenerateTest;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    filename: String,
}

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    let args: Args = Args::parse();

    let f = std::fs::File::open(&args.filename).expect("Could not open file.");
    //let value: serde_yaml::Value  = serde_yaml::from_reader(f).expect("Hallo Welt");
   // println!("Value:{:?}",value);
    let mut scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
    let mut markdown_visitor = GenerateMarkDownVisitor::from_filename("config2.md".to_string());
    let mut setup_generation_visitor = GenerateSetupEnvironmentVisitor::new();
    let mut idd_visitor = GenerateCommunicationProtocol::new();
    scrape_config.accept(&mut markdown_visitor);
    scrape_config.accept(&mut setup_generation_visitor);
    scrape_config.accept(&mut idd_visitor);
    let mut playbook: GenerateTest = GenerateTest::new(Rc::new(setup_generation_visitor),Rc::new(idd_visitor));
    scrape_config.accept(&mut playbook);
    for section in &mut playbook.playbook{
        section.execute();
        match section.verify(){
            Ok(success) => {
                if success{
                    println!("Test success");
                }
            }
            Err(_) => {}
        }
    }

}
