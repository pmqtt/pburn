mod definitions;
use clap::{Parser};
use definitions::program_configuration::{*};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args : Args = Args::parse();
    let f = std::fs::File::open(&args.filename).expect("Could not open file.");
    let scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
    println!("{:?}",args.filename);
    let filename:String = "config.md".to_string();
    scrape_config.generate_mark_down(&filename);


}
