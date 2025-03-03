use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufWriter, Write};

use rand::Rng;

#[derive(Debug, Clone)]
struct Task {
    id: usize,
    clock: usize,
    done: bool,
    start_time: usize,
    finish_time: usize,
}

impl Task {
    fn new(id: usize, clock: usize) -> Self {
        Task {
            id,
            clock,
            done: false,
            start_time: 0,
            finish_time: 0,
        }
    }

    fn task_exec(&mut self, current_time: &mut usize, quantum: Option<usize>) {
        self.start_time = *current_time;
        let cut_time = quantum.unwrap_or(self.clock);
        let actual_time = self.clock.min(cut_time);
        self.finish_time = self.start_time + actual_time;
        *current_time += actual_time;
        self.clock -= actual_time;

        if self.clock == 0 {
            self.done = true;
        }
    }
}

#[derive(Debug, Clone)]
struct PeriodicalTask {
    id: usize,
    clock: usize,
    period: usize,
    execution_time: usize,
    done: bool,
}

impl PeriodicalTask {
    fn new(id: usize, clock: usize, period: usize, execution_time: usize) -> Self {
        PeriodicalTask {
            id,
            clock,
            period,
            execution_time,
            done: false,
        }
    }

    fn task_exec(&mut self) {
        if self.done {
            return;
        }

        let actual_time = self.clock.min(self.execution_time);

        self.clock -= actual_time;

        if self.clock <= 0 {
            self.done = true;
        }
    }
}

// task of fifo
fn generate_task_deque(tasks: &mut VecDeque<Task>, count: usize) {
    let mut rng = rand::thread_rng();
    for id in 0..count {
        let clock = rng.gen_range(1, 10000000);
        tasks.push_back(Task::new(id, clock));
    }
}

// task of sjf and round robin
fn generate_tasks(tasks: &mut Vec<Task>, count: usize) {
    let mut rng = rand::thread_rng();
    for id in 0..count {
        let clock = rng.gen_range(1, 10000000);
        tasks.push(Task::new(id, clock));
    }
}

// task of rms
fn generate_periodical_tasks(tasks: &mut Vec<PeriodicalTask>, count: usize) {
    let mut rng = rand::thread_rng();
    for id in 0..count {
        let clock = rng.gen_range(1, 1000000);
        let period = rng.gen_range(100, 10000);
        let execution_time = (period as f64 * rng.gen_range(0.1, 0.4)) as usize;
        tasks.push(PeriodicalTask::new(id, clock, period, execution_time));
    }
}

// first in first out
pub fn fifo_schedulor(count: usize, file_name: &str) {
    let mut tasks: VecDeque<Task> = VecDeque::new();
    generate_task_deque(&mut tasks, count);

    let mut current_time = 0;
    let mut file = BufWriter::new(File::create(file_name).expect("Unable to create file"));

    writeln!(file, "id,start_time,finish_time").unwrap();
    for mut task in tasks {
        task.task_exec(&mut current_time, None);
        println!("task:{} finished!!", task.id);
        writeln!(file, "{},{},{}", task.id, task.start_time, task.finish_time).unwrap();
    }
}

// shortest job first
pub fn shortest_job_first(count: usize, file_name: &str) {
    let mut tasks: Vec<Task> = Vec::new();
    generate_tasks(&mut tasks, count);

    let mut current_time = 0;
    tasks.sort_by_key(|task| task.clock);
    let mut file = BufWriter::new(File::create(file_name).expect("Unable to create file"));

    writeln!(file, "id,start_time,finish_time").unwrap();
    for mut task in tasks {
        task.task_exec(&mut current_time, None);
        println!("task:{} finished!!", task.id);
        writeln!(file, "{},{},{}", task.id, task.start_time, task.finish_time).unwrap();
    }
}

// round robin
pub fn round_robin(count: usize, cut_clock: usize, file_name: &str) {
    let mut tasks: Vec<Task> = Vec::new();
    generate_tasks(&mut tasks, count);

    let mut current_time = 0;
    let mut finish_task = 0;
    let mut file = BufWriter::new(File::create(file_name).expect("Unable to create file"));

    writeln!(file, "id,start_time,finish_time").unwrap();
    while finish_task != tasks.len() {
        for task in tasks.iter_mut() {
            if !task.done {
                task.task_exec(&mut current_time, Some(cut_clock));
                writeln!(file, "{},{},{}", task.id, task.start_time, task.finish_time).unwrap();
                if task.done {
                    finish_task += 1;
                    println!("task:{} finished!!", task.id);
                }
            }
        }
    }
}

// ratemonotonic
pub fn rate_monotonic(count: usize) {
    let mut tasks: Vec<PeriodicalTask> = Vec::new();
    generate_periodical_tasks(&mut tasks, count);
    tasks.sort_by_key(|task| task.period);

    let mut finish_task = 0;

    while finish_task != tasks.len() {
        for task in tasks.iter_mut() {
            if !task.done {
                task.task_exec();

                if task.done {
                    finish_task += 1;
                    println!("task:{} finished!!", task.id);
                }
            }
        }
    }
}
