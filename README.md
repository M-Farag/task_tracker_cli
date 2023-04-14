### Task tracker CLI
This is a CLI app that helps you track your tasks and time spent on them.

### Problems it solves
- Setup a count down timer per task to help you track time spent on a task
- Receive notifications when a task starts or ends - notifications
- Log tasks to a flat file - log_tasks
- Log tasks to a google sheet - log_tasks


### How to use it locally
- Make sure you have the ~/.cargo/bin directory in your $PATH
- On your terminal, run `cargo install ttc`
- `ttc "task name" 1 h` - start a task
   - `ttc` is the CLI app name - app_name
   - ` "task name" ` is the you want to track name - task_name
   - ` 1 ` is the unsigned integer representing the time amount - duration_amount
   - ` h` is the time unit, can be `s for seconds`,`m for minutes`, `h for hours`

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