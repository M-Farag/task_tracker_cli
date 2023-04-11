use ttc::task::Task;
use std::env;

fn main() {
    let arguments:Vec<String> = env::args().collect();
    
    let t1:Task = Task::new(arguments).unwrap_or_else(|err|{
        println!("Err: {err}");
        std::process::exit(1);
    });

    if let Err(err) =  t1.write_message_to_file(){
        println!("Err: {:#?}",err);
        std::process::exit(1);
    }
    println!("Task 1: {:#?}",t1);
}