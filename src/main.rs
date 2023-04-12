use ttc::task::Task;

fn main() {
    
    let t1:Task = Task::new().unwrap_or_else(|err|{
        println!("Err: {err}");
        std::process::exit(1);
    });

    t1.start_task();

    if let Err(err) =  t1.log_task_to_file(){
        println!("Err: {:#?}",err);
        std::process::exit(1);
    }
    
}