### Task tracker CLI
This is a CLI app that helps you track your tasks and time spent on them.

### Problems it solves
- Setup a count down timer per task to help you track time spent on a task
- Receive notifications when a task starts or ends - notifications
- Log tasks to a CSV file - log_tasks
- Log tasks to a google sheet - log_tasks


### How to use it locally
- `ttc "task name" 1 hour` - start a task
   - `ttc` is the CLI app name - app_name
   - ` "task name" ` is the you want to track name - task_name
   - ` 1 ` is the unsigned integer representing the time amount - duration_amount
   - ` hour` is the time unit, can be seconds, minutes, hours - duration_unit

### Why I built it
I wanted to build a CLI app that helps me track my tasks and time spent on them. I also wanted to build a CLI app that I can use to learn more about building CLI apps with Rust.