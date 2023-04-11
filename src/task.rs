use std::{fs, error::Error, io::Write};

#[derive(Debug)]
pub struct Task {
    name:String,
    duration:u64,
    unit:String
}

impl Task{
    const TASKS_FILE_NAME:&'static str = "tasks.txt";
   pub fn new(arguments:Vec<String>) -> Result<Task,&'static str>
    {
        if arguments.len() < 4 {
            return Err("Err: Not enough arguments");
        }
        let duration = Task::parse_duration(&arguments[2])?;
        Ok(Task { name: arguments[1].clone(), duration: duration, unit: arguments[3].clone() })
    }

    pub fn start_task(&self) -> ()
    {
        println!("Started task: {},Duration: {}{}", &self.name, &self.duration, &self.unit);
        let sleep_time = match &self.unit[..] {
            "s"|"seconds"  => self.duration * 1000,
            "m"| "minutes" => self.duration * 60 * 1000,
            "h"| "hours"  => self.duration * 60 * 60 * 1000,
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

        writeln!(file_handler,"Task: {},Duration: {}{}",&self.name,&self.duration,&self.unit)?;
        Ok(true)
    }

    fn parse_duration(duration:&String) -> Result<u64,&'static str>
    {
        let duration = duration.parse::<u64>();
        match duration {
            Ok(duration) => Ok(duration),
            Err(_) => Err("Err: Duration is not a number")
        }
    }
    
}

#[cfg(test)]
mod test_task_module {
    use super::*;

    #[test]
    fn test_parse_duration() {
        let duration = Task::parse_duration(&String::from("10"));
        assert_eq!(duration, Ok(10));
    }

    #[test]
    fn test_parse_duration_error() {
        let duration = Task::parse_duration(&String::from("ten"));
        assert_eq!(duration, Err("Err: Duration is not a number"));
    }
}