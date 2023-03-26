use ttc::{helper,task::{self, Task}};
use std::env;

fn main() {
    let arguments:Vec<String> = env::args().collect();

    helper::validate_arguments_length(&arguments, 4).unwrap();

    let t_name = arguments.get(1).expect("Err: Getting task name");
    let t_duration = helper::convert_text_to_usize(
        arguments.get(2).expect("Err: Getting task duration")   
     ).unwrap();
         
    let t_unit = arguments.get(3).expect("Err: Getting duration unit");
    
    let t1:Task = Task::new(t_name,t_duration,t_unit);

    println!("Task 1: {:#?}",t1);
}