### Task tracker CLI v0.0.3
This is a CLI app that helps you track your tasks and time spent on them.

### Problems it solves
- Setup a count down timer per task to help you track time spent on a task
- Receive notifications when a task starts or ends - notifications
- Log tasks to a flat file - log_tasks
- Log tasks to a google sheet - log_tasks


### How to use it locally
- Make sure you have the ~/.cargo/bin directory in your $PATH
- On your terminal, run `cargo install ttc`

####  With time tracking
Default behavior is to track time spent on a task, the thread will be sleeping for the duration of the task. The thread will be woken up when the task is completed & automatically log the task to a flat file.

- `ttc -n "task name" -d 1 -u h` - start a task
   - `ttc` is the CLI app name
   - ` -n "task name" ` The flag -n is used to specify the task name
   - ` -d 1 ` the flag -d is used to specify the duration of the task
   - ` -u h` the flag -u is used to specify the unit of time for the duration of the task. The unit can be h for hours, m for minutes or s for seconds

#### Without time tracking
Using the flag `-l` will log the task to a flat file without time tracking

- `ttc -n "task name" -d 1 -u h -l` - start a task
   - `ttc` is the CLI app name
   - ` -n "task name" ` The flag -n is used to specify the task name
   - ` -d 1 ` the flag -d is used to specify the duration of the task
   - ` -u h` the flag -u is used to specify the unit of time for the duration 
   of the task. The unit can be h for hours, m for minutes or s for seconds
   - ` -l` the flag -l is used to specify that you want to log the task to a flat file without time tracking


### Why I built it
I wanted to build a CLI app that helps me track my tasks and time spent on them. I also wanted to build a CLI app that I can use to learn more about building CLI apps with Rust.

### How to use it in your rust crates/projects
- Add `ttc = "*"` to your Cargo.toml dependencies
```rust
      use ttc::Task;
      let t1:Task = Task::new().unwrap_or_else(|err|{
            println!("Err: {err}");
            std::process::exit(1);
         });

         t1.start_task();

         if let Err(err) =  t1.log_task_to_file(){
            println!("Err: {:#?}",err);
            std::process::exit(1);
         }
```