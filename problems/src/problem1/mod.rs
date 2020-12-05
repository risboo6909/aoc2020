use std::collections::HashSet;

use failure::{format_err, Error};
use utils::{result, split_by_lines, ProblemResult, RetTypes};

const YEAR: usize = 2020;

fn find_pair_sum(input: &[usize], expected: usize, start_from: usize) -> Option<(usize, usize)> {
    let mut lookup: HashSet<usize> = HashSet::new();

    for n in input.iter().skip(start_from) {
        if *n > expected {
            continue;
        }

        let to_add = expected - n;
        if lookup.contains(&to_add) {
            // found!
            return Some((to_add, *n));
        }
        lookup.insert(*n);
    }

    None
}

fn first_star(input: &[usize]) -> ProblemResult<usize> {
    match find_pair_sum(input, YEAR, 0) {
        Some((a, b)) => Ok(a * b),
        None => Err(format_err!("solution not found")),
    }
}

fn second_star(input: &[usize]) -> ProblemResult<usize> {
    for (idx, c) in input.iter().enumerate() {
        if let Some((a, b)) = find_pair_sum(input, YEAR - c, idx + 1) {
            return Ok(a * b * c);
        }
    }

    Err(format_err!("solution not found"))
}

pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let input: Vec<usize> = split_by_lines(input_raw, &|e: &str| {
        e.parse::<usize>()
            .map_err(|_| format_err!("Failed to parse input"))
    })?;

    Ok(RetTypes::Usize(result(
        first_star(&input),
        second_star(&input),
    )))
}
