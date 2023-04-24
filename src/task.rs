//! Tasks module contains all the necessary functions to create a task and log it to a file

use std::{fs, error::Error, io::Write};
use chrono::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name="Task tracker cli  app",about="Track your tasks and log it to file")]
struct Config {
    #[structopt(long="name",short="n",help="Name of the task")]
    name:String,

    #[structopt(long="duration",short="d",help="Duration of the task")]
    duration:u64,

    #[structopt(long="unit",short="u",default_value="s",help="Unit of the duration can be h for hours, m for minutes and s for seconds, default is seconds")]
    unit:String,   
}

impl Config{
    pub fn get_arguments() -> Result<Config,Box<dyn Error>>
    {
        if let Err(e) = Config::from_args_safe() {
            return Err(format!("{}",e).into())
        }
        Ok(Config::from_args())
    }
}

#[derive(Debug)]
pub struct Task {
    name:String,
    duration:u64,
    unit:String
}

impl Task{
        const TASKS_FILE_NAME:&'static str = "tasks.txt";

        /// Creates a new task
        /// # Arguments
        /// * `name` - Name of the task
        /// * `duration` - Duration of the task
        /// * `unit` - Unit of the duration
        /// 
        /// # Panics
        /// This function will panic if any of the arguments are not valid
        /// 
        /// # Errors
        /// This function will return an error if the duration is not a number
        /// 
        pub fn new() -> Result<Task,Box<dyn Error>>
        {
            let arguments = Config::get_arguments()?;
            Ok(Task { name:arguments.name, duration:arguments.duration, unit:arguments.unit })
        }

        pub fn start_task(&self) -> ()
        {
            println!("Started task: {},Duration: {}{}", &self.name, &self.duration, &self.unit);
            let sleep_time = match &self.unit[..] {
                "s"  => self.duration * 1000,
                "m" => self.duration * 60 * 1000,
                "h"  => self.duration * 60 * 60 * 1000,
                _ => 0
            };

            std::thread::sleep(std::time::Duration::from_millis(sleep_time));
            println!("Finished task: {}", &self.name);
        }

        pub fn log_task_to_file(&self)->  Result<bool,Box<dyn Error>>
        {
            let mut file_handler= fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(Task::TASKS_FILE_NAME)?;

            let local_time = Local::now();
            writeln!(file_handler,"LogDateTime: {}, Task: {},Duration: {}{}",local_time.format("%Y-%m-%d %I:%M:%S %p"),&self.name,&self.duration,&self.unit)?;
            Ok(true)
        }
    
}

#[cfg(test)]
mod test_task_module {
    use super::*;

}