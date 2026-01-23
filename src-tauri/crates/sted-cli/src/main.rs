use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).expect("Please provide a path to the RCP file");
    let binary = fs::read(path).expect("Failed to read the RCP file");
    let rcp_file = sted_core::load(&binary);

    sted_core::play(&rcp_file);
}
