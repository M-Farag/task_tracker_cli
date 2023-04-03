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
        
        Ok(Task { name: arguments[1].clone(), duration: arguments[2].clone(), unit: arguments[3].clone() })
    }
    
}