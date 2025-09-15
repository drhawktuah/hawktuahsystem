use std::time::{Duration, Instant};
use std::thread::sleep;
use std::collections::{VecDeque};

/// HAWK TUAH SYSTEM (THIS IS A SIMULATION, A PRELUDE TO THE ALBUM)

struct Task {
    name: String,
    next_run: Instant,
    interval: Duration,
    job: fn(),
}

impl Task {
    fn run(&mut self) {
        ((self.job)());

        self.next_run = Instant::now() + self.interval;
    }
}

struct Scheduler {
    tasks: VecDeque<Task>
}

impl Scheduler {
    fn new() -> Self {
        Scheduler {
            tasks: VecDeque::new(),
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push_back(task);
    }

    fn run(&mut self) {
        loop {
            let now = Instant::now();

            for _i in 0..self.tasks.len() {
                let mut task = self.tasks.pop_front().unwrap();

                if task.next_run <= now {
                    println!("[{}] Running task", task.name);

                    task.run();
                } else {
                    println!("[{}] Skipping task, will run at {:?}", task.name, task.next_run);
                }

                self.tasks.push_back(task);
            }

            sleep(Duration::from_millis(100));
        }
    }
}

fn say_hello() {
    println!("Hello from task 1!");
}

fn say_world() {
    println!("Hello from task 2");
}

fn main() {
    let mut scheduler = Scheduler::new();

    scheduler.add_task(Task {
        name: "Task1".into(),
        next_run: Instant::now(),
        interval: Duration::from_secs(1),
        job: say_hello,
    });

    scheduler.add_task(Task {
        name: "Task2".into(),
        next_run: Instant::now(),
        interval: Duration::from_secs(2),
        job: say_world,
    });

    scheduler.run();
}