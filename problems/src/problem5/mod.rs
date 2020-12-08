use failure::{format_err, Error};
use utils::{result, split_by_lines, ProblemResult, RetTypes};

const SEQ_LEN: usize = 10;
const ROW_IDX: usize = 7;

#[derive(Debug)]
enum Dir {
    Front,
    Back,
    Left,
    Right,
}

#[derive(Copy, Clone)]
struct Interval {
    lower: f64,
    upper: f64,
}

fn split_left(int: Interval) -> Interval {
    Interval {
        lower: int.lower,
        upper: (0.5 * (int.lower + int.upper)).floor(),
    }
}

fn split_right(int: Interval) -> Interval {
    Interval {
        lower: (0.5 * (int.lower + int.upper)).ceil(),
        upper: int.upper,
    }
}

fn find_all_seats(input: &[Vec<Dir>]) -> ProblemResult<Vec<usize>> {
    let mut seats = Vec::new();

    for steps in input {
        assert_eq!(steps.len(), SEQ_LEN);

        let mut row = Interval {
            lower: 0f64,
            upper: 127f64,
        };
        let mut col = Interval {
            lower: 0f64,
            upper: 7f64,
        };

        for (idx, step) in steps.iter().enumerate() {
            if idx < ROW_IDX {
                // find a row
                row = match step {
                    Dir::Front => split_left(row),
                    Dir::Back => split_right(row),
                    _ => return Err(format_err!("wrong direction {:?}", step)),
                }
            } else {
                // find a column
                col = match step {
                    Dir::Left => split_left(col),
                    Dir::Right => split_right(col),
                    _ => return Err(format_err!("wrong direction {:?}", step)),
                }
            }
        }

        seats.push(row.lower as usize * 8 + col.lower as usize);
    }

    Ok(seats)
}

fn first_star(input: &[usize]) -> usize {
    *input.iter().max().unwrap()
}

fn second_star(input: &[usize]) -> usize {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();

    let expected_sum: usize = (*min..=*max).sum();
    let cur_sum: usize = input.iter().sum();

    expected_sum - cur_sum
}

pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let input: Vec<Vec<Dir>> = split_by_lines(input_raw, &|line: &str| {
        line.chars()
            .map(|c| match c {
                'F' => Ok(Dir::Front),
                'B' => Ok(Dir::Back),
                'L' => Ok(Dir::Left),
                'R' => Ok(Dir::Right),
                c => Err(format_err!(
                    "Failed to parse input, unknown character '{}'",
                    c
                )),
            })
            .collect::<Result<Vec<Dir>, _>>()
    })?;

    let seats = find_all_seats(&input)?;

    Ok(RetTypes::Usize(result(
        Ok(first_star(&seats)),
        Ok(second_star(&seats)),
    )))
}

#[cfg(test)]
mod tests {

    use super::{find_all_seats, Dir};

    #[test]
    fn test_first() {
        // FBFBBFFRLR
        assert_eq!(
            find_all_seats(&vec![vec![
                Dir::Front,
                Dir::Back,
                Dir::Front,
                Dir::Back,
                Dir::Back,
                Dir::Front,
                Dir::Front,
                Dir::Right,
                Dir::Left,
                Dir::Right
            ],])
            .unwrap()[0],
            357
        );

        // FFFBBBFRRR
        assert_eq!(
            find_all_seats(&vec![vec![
                Dir::Front,
                Dir::Front,
                Dir::Front,
                Dir::Back,
                Dir::Back,
                Dir::Back,
                Dir::Front,
                Dir::Right,
                Dir::Right,
                Dir::Right
            ],])
            .unwrap()[0],
            119
        );
    }
}
