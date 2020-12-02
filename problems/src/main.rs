mod problem1;
mod problem2;

use colored::*;
use crossbeam::{queue::SegQueue, thread};
use failure::Error;
use std::{marker::Sync, sync::Arc, time::SystemTime};

use utils::RetTypes;

// problems
use crate::problem1 as p1;
use crate::problem2 as p2;

fn exec(f: &(dyn Fn() -> Result<RetTypes, Error>), problem_no: usize) {
    let now = SystemTime::now();
    let result = f();
    let elapsed = now.elapsed().unwrap().as_millis();

    match result {
        Err(err) => println!(
            "{} {}:\n{}: {}",
            "problem".bold(),
            problem_no.to_string().bold(),
            "error".bold().red(),
            err
        ),

        Ok(answer) => println!(
            "{} {}:\n{}",
            "problem".bold(),
            problem_no.to_string().bold(),
            answer
        ),
    }

    println!("time elapsed for problem: {} millis\n", elapsed);
}

fn main() {
    println!("\n{}\n\n", "Advent of code 2020".bold());

    let q: Arc<SegQueue<(&(dyn Fn() -> Result<RetTypes, Error> + Sync), usize)>> =
        Arc::new(SegQueue::new());

    q.push((&p1::solve, 1));
    q.push((&p2::solve, 2));

    println!("{} cores detected\n", num_cpus::get_physical());

    let now = SystemTime::now();

    thread::scope(|s| {
        for idx in 0..num_cpus::get_physical() {
            let q = Arc::clone(&q);
            s.spawn(move |_| {
                while let Ok((task, task_id)) = q.pop() {
                    println!("Worker {} executing problem {}\n", idx, task_id);
                    exec(task, task_id);
                }
            });
        }
    })
    .unwrap();

    println!(
        "{} {} {}",
        "Total time taken:".bold(),
        now.elapsed()
            .unwrap()
            .as_millis()
            .to_string()
            .bold()
            .green(),
        "millis".bold()
    );
}
