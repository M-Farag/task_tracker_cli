#[derive(Debug)]
pub struct Task {
    name:String,
    duration:u64,
    unit:String
}

impl Task{

   pub fn new(arguments:Vec<String>) -> Result<Task,&'static str>
    {
        if arguments.len() < 4 {
            return Err("Err: Not enough arguments");
        }
        let duration = Task::parse_duration(&arguments[2])?;
        Ok(Task { name: arguments[1].clone(), duration: duration, unit: arguments[3].clone() })
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