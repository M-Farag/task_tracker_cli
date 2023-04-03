use ttc::{helper,task::{self, Task}};
use std::env;

fn main() {
    let arguments:Vec<String> = env::args().collect();
    
    let t1:Task = Task::new(arguments).unwrap_or_else(|err|{
        println!("Err: {err}");
        std::process::exit(1);
    });

    println!("Task 1: {:#?}",t1);
}