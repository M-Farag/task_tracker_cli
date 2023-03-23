use ttc::helper;
use std::env;

fn main() {
    let arguments:Vec<String> = env::args().collect();
    let task_name =  arguments.get(1).unwrap();
    let task_duration = arguments.get(2).unwrap();
    // let duration_unit = arguments.get(3).unwrap();

    let task_duration = helper::convert_text_to_usize(task_duration).unwrap();

    println!("Task name: {} & for duration: {}",task_name,task_duration);
}