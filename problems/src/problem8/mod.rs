use std::collections::HashSet;

use failure::{Error, format_err};
use utils::{result, split_by_lines, RetTypes};

#[derive(Debug)]
enum Op {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

#[derive(Default)]
struct Computer {
    acc: isize,
}

type IsLoop = bool;

impl Computer {

    fn run(&mut self, program: &[Op]) -> IsLoop {

        let mut line_numbers = HashSet::new();
        let mut ip = 0isize;

        self.acc = 0;

        while ip < program.len() as isize {

            if line_numbers.contains(&ip) {
                return true
            }

            let op = &program[ip as usize];
            line_numbers.insert(ip);

            match op {
                Op::Acc(arg) => {
                    self.acc += arg;
                },
                Op::Jmp(arg) => {
                    ip += arg;
                    continue;
                },
                Op::Nop(_) => {},
            }
            ip += 1;
        }

        false

    }
}

fn first_star(input: &[Op]) -> isize {
    let mut c = Computer::default();
    c.run(&input);

    c.acc
}

fn switch_op_at_idx(input: &mut [Op], idx: usize) -> bool {
    match input[idx] {
        Op::Jmp(arg) => {input[idx] = Op::Nop(arg); true}
        Op::Nop(arg) => {input[idx] = Op::Jmp(arg); true}
        _ => false,
    }
} 

fn second_star(input: &mut [Op]) -> isize {
    let mut c = Computer::default();

    for idx in 0..input.len() {

        if c.run(&input) {

            if !switch_op_at_idx(input, idx) {
                // we can't switch 'acc'
                continue
            }

            if !c.run(&input) {
                // no loop, we've fixed the program
                return c.acc
            }

            // restore previous state for the given index
            switch_op_at_idx(input, idx);
        }
    }

    0
}

fn parse(input_raw: &str) -> Result<Vec<Op>, Error> {
    let res: Result<Vec<Op>, _> = split_by_lines(input_raw, &|line: &str| {
        let mut splitter = line.splitn(2, ' ');
        // op
        let op = splitter.next().unwrap();
        // argument
        let arg = splitter.next().unwrap().parse::<isize>()?;
        match op {
            "acc" => Ok(Op::Acc(arg)),
            "jmp" => Ok(Op::Jmp(arg)),
            "nop" => Ok(Op::Nop(arg)),
            _ => Err(format_err!("unknown op '{}'", op)),
        }

    });

    res
}

pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let mut input = parse(input_raw)?;

    let fst = first_star(&input);
    // just to be sure I won't brake anything doing part two
    assert_eq!(fst, 1337);

    Ok(RetTypes::Isize(result(
        Ok(fst),
        Ok(second_star(&mut input)),
    )))
}
