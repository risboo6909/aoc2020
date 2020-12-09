use std::collections::HashSet;

use failure::{format_err, Error};
use utils::{result, split_by_lines, RetTypes};

const WINDOW_SIZE: usize = 25;

fn find_pair_sum(input: &[usize], expected: usize) -> Option<(usize, usize)> {
    let mut lookup: HashSet<usize> = HashSet::new();

    for n in input.iter() {
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

fn first_star(input: &[usize], window_size: usize) -> usize {
    for idx in 0..input.len() {
        match find_pair_sum(&input[idx..idx + window_size], input[idx + window_size]) {
            Some(_) => {}
            None => return input[idx + window_size],
        }
    }
    0
}

fn second_star(input: &[usize], n: usize) -> usize {
    for idx1 in 0..input.len() {
        let mut part_sum = 0;
        for idx2 in idx1..input.len() {
            part_sum += input[idx2];
            if part_sum == n {
                return input[idx1..idx2].iter().max().unwrap()
                    + input[idx1..idx2].iter().min().unwrap();
            }
        }
    }
    0
}

pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let input: Vec<usize> = split_by_lines(input_raw, &|e: &str| {
        e.parse::<usize>()
            .map_err(|_| format_err!("Failed to parse input"))
    })?;

    let fst = first_star(&input, WINDOW_SIZE);

    Ok(RetTypes::Usize(result(
        Ok(fst),
        Ok(second_star(&input, fst)),
    )))
}

#[cfg(test)]
mod tests {
    use super::{first_star, second_star};
    #[test]
    fn test_first() {
        let res = first_star(
            &[
                35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                309, 576,
            ],
            5,
        );
        assert_eq!(res, 127);
    }

    #[test]
    fn test_second() {
        let res = second_star(
            &[
                35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                309, 576,
            ],
            127,
        );
        assert_eq!(res, 62);
    }
}
