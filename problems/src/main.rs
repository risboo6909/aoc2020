#![feature(const_option)]

mod problem1;
mod problem10;
mod problem11;
mod problem12;
mod problem13;
mod problem14;
mod problem15;
mod problem16;
mod problem17;
mod problem18;
mod problem19;
mod problem2;
mod problem3;
mod problem4;
mod problem5;
mod problem6;
mod problem7;
mod problem8;
mod problem9;

use colored::*;
use crossbeam::{queue::SegQueue, thread};
use failure::Error;
use std::{marker::Sync, sync::Arc, time::SystemTime};

use utils::RetTypes;

// problems
use crate::problem1 as p1;
use crate::problem10 as p10;
use crate::problem11 as p11;
use crate::problem12 as p12;
use crate::problem13 as p13;
use crate::problem14 as p14;
use crate::problem15 as p15;
use crate::problem16 as p16;
use crate::problem17 as p17;
use crate::problem18 as p18;
use crate::problem19 as p19;
use crate::problem2 as p2;
use crate::problem3 as p3;
use crate::problem4 as p4;
use crate::problem5 as p5;
use crate::problem6 as p6;
use crate::problem7 as p7;
use crate::problem8 as p8;
use crate::problem9 as p9;

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

    #[allow(clippy::type_complexity)]
    let q: Arc<SegQueue<(&(dyn Fn() -> Result<RetTypes, Error> + Sync), usize)>> =
        Arc::new(SegQueue::new());

    q.push((&p1::solve, 1));
    q.push((&p2::solve, 2));
    q.push((&p3::solve, 3));
    q.push((&p4::solve, 4));
    q.push((&p5::solve, 5));
    q.push((&p6::solve, 6));
    q.push((&p7::solve, 7));
    q.push((&p8::solve, 8));
    q.push((&p9::solve, 9));
    q.push((&p10::solve, 10));
    q.push((&p11::solve, 11));
    q.push((&p12::solve, 12));
    q.push((&p13::solve, 13));
    q.push((&p14::solve, 14));
    q.push((&p15::solve, 15));
    q.push((&p16::solve, 16));
    q.push((&p17::solve, 17));
    q.push((&p18::solve, 18));
    q.push((&p19::solve, 19));

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
