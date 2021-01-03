use std::borrow::Cow;

use failure::{format_err, Error};
use utils::{result, RetTypes};

const BRUTE_FORCE_N: usize = 4;

#[derive(Debug)]
struct Sched<'a> {
    n: usize, // this is required for the first star only

    intervals: Cow<'a, [usize]>,
    deltas: Cow<'a, [usize]>,
}

impl<'a> Sched<'a> {
    fn new(intervals: Cow<'a, [usize]>, mut deltas: Cow<'a, [usize]>, n: usize) -> Self {
        deltas.to_mut().insert(0, 0);
        Sched {
            intervals,
            deltas,
            n,
        }
    }

    fn take_first_n(&self, n: usize) -> Self {
        Sched::new(
            Cow::from(self.intervals[..n].to_vec()),
            Cow::from(self.deltas[1..n].to_vec()),
            self.n,
        )
    }

    fn is_valid(&self, start_offset: usize) -> bool {
        let mut offset = 0;

        for (offset_idx, int) in self.intervals.iter().enumerate() {
            offset += self.deltas[offset_idx];
            if (start_offset + offset) % int != 0 {
                return false;
            }
        }

        true
    }
}

fn first_star(sched: &Sched) -> usize {
    let res = sched
        .intervals
        .iter()
        .map(|item| (item, item * (sched.n / item + 1) - sched.n))
        .min_by_key(|e| e.1)
        .unwrap();
    res.0 * res.1
}

fn brute_force(sched: &Sched, start_offset: usize) -> usize {
    let mut ts = start_offset;

    'outer: loop {
        ts += 1;
        if !sched.is_valid(ts) {
            continue 'outer;
        }

        return ts - start_offset;
    }
}

fn find_period(sched: &Sched) -> (usize, usize) {
    let offset = brute_force(sched, 0);
    let period = brute_force(sched, offset);

    (offset, period)
}

fn second_star(sched: &Sched, brute_force_items: usize) -> usize {
    let trunc_sched = sched.take_first_n(brute_force_items);

    // brute-force repeat period for truncated schedule
    let (mut cur_offset, period) = find_period(&trunc_sched);

    // brute-force the rest
    loop {
        if sched.is_valid(cur_offset) {
            return cur_offset;
        }

        cur_offset += period;
    }
}

fn parse(input_raw: &str) -> Result<Sched, Error> {
    let mut splitter = input_raw.split('\n');

    let n = splitter
        .next()
        .ok_or_else(|| format_err!("can't parse timestamp"))?
        .parse::<usize>()?;
    let sched_raw = splitter.next().ok_or_else(|| format_err!("can't parse timestamp"))?;

    let mut intervals: Vec<usize> = vec![];
    let mut deltas: Vec<usize> = vec![];

    let mut delta = 0;

    for item in sched_raw.split(',') {
        if item != "x" {
            deltas.push(delta);
            intervals.push(item.parse()?);

            delta = 1;
        } else {
            delta += 1;
        }
    }

    Ok(Sched {
        n,
        intervals: Cow::from(intervals),
        deltas: Cow::from(deltas),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let sched = parse(input_raw)?;

    Ok(RetTypes::Usize(result(
        Ok(first_star(&sched)),
        Ok(second_star(&sched, BRUTE_FORCE_N)),
    )))
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::{brute_force, find_period, second_star, Sched};

    #[test]
    fn test_bf() {
        assert_eq!(
            brute_force(&Sched::new(Cow::from(vec![5, 7]), Cow::from(vec![1]), 0), 0),
            20
        );
        assert_eq!(
            brute_force(
                &Sched::new(Cow::from(vec![67, 7, 59, 61]), Cow::from(vec![1, 1, 1]), 0),
                0
            ),
            754018
        );
        assert_eq!(
            brute_force(
                &Sched::new(Cow::from(vec![67, 7, 59, 61]), Cow::from(vec![2, 1, 1]), 0),
                0
            ),
            779210
        );
        assert_eq!(
            brute_force(
                &Sched::new(
                    Cow::from(vec![1789, 37, 47, 1889]),
                    Cow::from(vec![1, 1, 1]),
                    0
                ),
                0
            ),
            1202161486
        );
    }

    #[test]
    fn text_find_period() {
        assert_eq!(
            find_period(&Sched::new(Cow::from(vec![3, 5]), Cow::from(vec![1]), 0)),
            (9, 15)
        );
    }

    #[test]
    fn test_second_star() {
        assert_eq!(
            second_star(
                &Sched::new(
                    Cow::from(vec![1789, 37, 47, 1889]),
                    Cow::from(vec![1, 1, 1]),
                    3,
                ),
                2,
            ),
            1202161486
        );
    }
}
