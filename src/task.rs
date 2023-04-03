#[derive(Debug)]
pub struct Task {
    name:String,
    duration:String,
    unit:String
}

impl Task{

   pub fn new(arguments:Vec<String>) -> Result<Task,&'static str>
    {
        if arguments.len() < 4 {
            return Err("Err: Not enough arguments");
        }
        let name = arguments.get(1).expect("Err: Getting task name").to_string();
        let duration = arguments.get(2).expect("Err: Getting task duration").to_string();   
        let unit = arguments.get(3).expect("Err: Getting duration unit").to_string();
        Ok(Task { name, duration, unit })
    }
    
}