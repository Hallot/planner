use std::env;
use std::fs::File;

mod parser;
mod input_data;
mod task;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./{} json_file", args[0]);
        return
    }
    let json_path = &args[1];

    let mut file = File::open(json_path).expect(&format!("Unable to open file {}", json_path));
    let input_data = parser::parse_from_json(&mut file).expect("Unable to parse JSON file, check format");
}
