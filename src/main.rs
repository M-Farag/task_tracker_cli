use ttc::task::Task;
use std::env;

fn main() {
    let arguments:Vec<String> = env::args().collect();
    
    let t1:Task = Task::new(arguments).unwrap_or_else(|err|{
        println!("Err: {err}");
        std::process::exit(1);
    });

    t1.start_task();

    if let Err(err) =  t1.log_task_to_file(){
        println!("Err: {:#?}",err);
        std::process::exit(1);
    }
    
}