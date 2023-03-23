use task_tracker_cli::helper;
use std::env;

fn main() {
    let arguments:Vec<String> = env::args().collect();
    let task_name =  arguments.get(1).unwrap();

    println!("Task name: {}",task_name);
}