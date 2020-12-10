use std::cmp::max;
use std::collections::{HashMap, HashSet};

use itertools::sorted;

use failure::{format_err, Error};
use utils::{result, split_by_lines, RetTypes};

fn first_star_rec(
    input: &mut HashSet<usize>,
    cur_joltage: usize,
    one_jolt_cnt: usize,
    three_jolt_cnt: usize,
) -> usize {
    if input.is_empty() {
        return one_jolt_cnt * (three_jolt_cnt + 1);
    }

    let mut res = 0;

    for i in 1..=3 {
        let next_adp_joltage = cur_joltage + i;

        if input.contains(&next_adp_joltage) {
            if res > 0 {
                break;
            }

            input.remove(&next_adp_joltage);
            if i == 1 {
                res = max(
                    res,
                    first_star_rec(input, next_adp_joltage, one_jolt_cnt + 1, three_jolt_cnt),
                );
            } else if i == 3 {
                res = max(
                    res,
                    first_star_rec(input, next_adp_joltage, one_jolt_cnt, three_jolt_cnt + 1),
                );
            } else {
                res = max(
                    res,
                    first_star_rec(input, next_adp_joltage, one_jolt_cnt, three_jolt_cnt),
                );
            }
            input.insert(next_adp_joltage);
        }
    }

    res
}

fn first_star(input: &[usize]) -> usize {
    let mut tmp: HashSet<usize> = input.iter().copied().collect();
    first_star_rec(&mut tmp, 0, 0, 0)
}

fn second_star_rec(
    input: &mut HashSet<usize>,
    dp: &mut HashMap<Vec<usize>, usize>,
    cur_joltage: usize,
    target_joltage: usize,
) -> usize {
    let as_vec = sorted(input.iter()).copied().collect::<Vec<usize>>();

    // see if we already have this computed before
    if dp.contains_key(&as_vec) {
        return *dp.get(&as_vec).unwrap();
    }

    if cur_joltage + 3 == target_joltage {
        return 1;
    }

    let mut res = 0;

    let mut taken = vec![];

    for i in 1..=3 {
        let next_adp_joltage = cur_joltage + i;

        if input.contains(&next_adp_joltage) {
            taken.push(next_adp_joltage);

            input.remove(&next_adp_joltage);

            res += second_star_rec(input, dp, next_adp_joltage, target_joltage);
        }
    }

    for item in taken {
        input.insert(item);
    }

    dp.entry(sorted(input.iter()).copied().collect::<Vec<usize>>())
        .or_insert(res);

    res
}

fn second_star(input: &[usize], target_joltage: usize) -> usize {
    let mut tmp: HashSet<usize> = input.iter().copied().collect();
    second_star_rec(&mut tmp, &mut HashMap::new(), 0, target_joltage)
}

pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let input: Vec<usize> = split_by_lines(input_raw, &|e: &str| {
        e.parse::<usize>()
            .map_err(|_| format_err!("Failed to parse input"))
    })?;

    let fst = first_star(&input);

    Ok(RetTypes::Usize(result(
        Ok(fst),
        Ok(second_star(&input, *input.iter().max().unwrap() + 3)),
    )))
}

#[cfg(test)]
mod tests {
    use super::{first_star, second_star};
    #[test]
    fn test_first() {
        let res = first_star(&[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]);
        assert_eq!(res, 35);
    }

    #[test]
    fn test_second() {
        let res = second_star(
            &[
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3,
            ],
            52,
        );
        assert_eq!(res, 19208);
    }
}
