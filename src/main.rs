use ttc::{helper,task::{self, Task}};
use std::env;

fn main() {
    let arguments:Vec<String> = env::args().collect();
    let mut t1:Task = Task::new(
         arguments.get(1).expect("Err: Getting task name"),
         helper::convert_text_to_usize(
            arguments.get(2).expect("Err: Getting task duration")   
         ).unwrap()
         ,
         arguments.get(3).expect("Err: Getting duration unit")
    );

    println!("Task 1: {:#?}",t1);
}