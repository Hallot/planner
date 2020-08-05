use std::env;
use std::fs::File;

mod parser;
mod input_data;
mod task;
mod planning;
mod solver;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./{} json_file", args[0]);
        return
    }
    let json_path = &args[1];

    let mut file = File::open(json_path).expect(&format!("Unable to open file {}", json_path));
    let input_data = parser::parse_from_json(&mut file).expect("Unable to parse JSON file, check format");
    let planning = solver::simulated_annealing(input_data);
    let planning_json = serde_json::to_string(&planning).expect("Unable to create output JSON");
    println!("{}", planning_json);
}
