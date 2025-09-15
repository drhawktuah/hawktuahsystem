use std::collections::VecDeque;
use std::thread::sleep;
use std::time::{Duration, Instant};

/// HAWKTUAHSYSTEM: A PRELUDE TO THE ALBUM (A SIMULATION OF A SCHEDULER)

pub trait Runnable {
    fn run(&mut self);
}

pub struct Task {
    pub name: String,
    pub interval: Duration,
    pub next_run: Instant,
    pub job: Box<dyn Runnable>
}

pub struct Scheduler {
    tasks: VecDeque<Task>,
}

impl Task {
    pub fn run(&mut self) {
        self.job.run();
        self.next_run = Instant::now() + self.interval;
    }
}

/// A wrapper for VecDeque to handle scheduled tasks
impl Scheduler {
    pub fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push_back(task);
    }

    pub fn pop_task(&mut self) -> Option<Task> {
        self.tasks.pop_front() // since this is shorthand for "return self.tasks.pop_front" in other languages, only return pop_front's popped value
    }

    pub fn push_task(&mut self, task: Task) {
        self.tasks.push_back(task);
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }
}

/*
impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            tasks: VecDeque::new(),
        }
    }

    pub fn add_task<T: Runnable + 'static>(&mut self, name: &str, interval: Duration, job: T) {
        let task = Task {
            name: name.to_string(),
            next_run: Instant::now(),
            interval,
            job: Box::new(job),
        };

        self.tasks.push_back(task);
    }

    pub fn run(&mut self) {
        loop {
            let now = Instant::now();

            for _ in 0..self.tasks.len() {
                let mut task = self.tasks
                    .pop_front()
                    .unwrap();

                if task.next_run <= now {
                    println!("[{}] Running task", task.name);

                    task.run();
                } else {
                    println!("[{}] Skipping task (next at {:?}", task.name, task.next_run);
                }

                self.tasks.push_back(task);
            }

            sleep(Duration::from_millis(100));
        }
    }
}
*/