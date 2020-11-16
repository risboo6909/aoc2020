use failure::{format_err, Error};
use utils::{result, split_by_lines, ProblemResult, RetTypes};

fn fuel_req(mass: usize) -> usize {
    let a = mass / 3;
    if a >= 2 {
        a - 2
    } else {
        0
    }
}

fn first_star(input: &[usize]) -> ProblemResult<usize> {
    let mut fuel = 0;
    for mass in input {
        fuel += fuel_req(*mass);
    }
    Ok(fuel)
}

fn second_star(input: &[usize]) -> ProblemResult<usize> {
    let mut fuel = 0;

    for mass in input {
        let mut mass_remain = *mass;
        while mass_remain > 0 {
            mass_remain = fuel_req(mass_remain);
            fuel += mass_remain;
        }
    }

    Ok(fuel)
}

pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let input: Vec<usize> = split_by_lines(input_raw, &|e: &str| {
        e.parse::<usize>()
            .or_else(|_| Err(format_err!("Failed to parse input")))
    })?;

    Ok(
        RetTypes::Usize(
            result(first_star(&input), second_star(&input))
        )
    )

}
