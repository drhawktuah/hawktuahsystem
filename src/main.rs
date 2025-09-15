use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::kernel::tasks::HelloTask;
use crate::kernel::scheduler::{Scheduler, Task};

mod kernel;

// declare scheduler as mutable referemce (borrowing the original scheduler instance mutability)
fn run_scheduler_loop(scheduler: &mut Scheduler) {
    loop {
        let now = std::time::Instant::now();

        let task_count = scheduler.task_count();

        for _ in 0..task_count {
            if let Some(mut task) = scheduler.pop_task() {
                if task.next_run <= now {
                    println!("{} Running task", task.name);
                    task.run();
                } else {
                    println!("{} not ready yet", task.name);
                }

                scheduler.push_task(task);
            }
        }

        sleep(Duration::from_millis(100));
    }
}

fn main() {
    let mut scheduler = Scheduler::new();

    let hello_task = HelloTask::new(); 

    scheduler.add_task(Task {
        name: "hello".to_string(),
        interval: Duration::from_secs(1),
        next_run: Instant::now(),
        job: Box::new(hello_task),
    });

    run_scheduler_loop(&mut scheduler);
}