use crate::kernel::scheduler::Runnable;

pub struct HelloTask {
    count: u32,
}

impl HelloTask {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Runnable for HelloTask {
    fn run(&mut self) {
        self.count += 1;

        println!("Hello from hello task! {}", self.count);
    }
}