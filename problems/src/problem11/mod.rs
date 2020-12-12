use failure::{format_err, Error};
use itertools::join;
use utils::{result, split_by_lines, RetTypes};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Cell {
    Empty,
    Occupied,
    Floor,
}

#[derive(Copy, Clone, Eq, Debug)]
enum Dir {
    N(Cell), // North, ...
    NE(Cell),
    E(Cell),
    SE(Cell),
    S(Cell),
    SW(Cell),
    W(Cell),
    NW(Cell),
}

impl Dir {
    fn unwrap(self) -> Cell {
        match self {
            Dir::N(c) => c,
            Dir::NE(c) => c,
            Dir::E(c) => c,
            Dir::SE(c) => c,
            Dir::S(c) => c,
            Dir::SW(c) => c,
            Dir::W(c) => c,
            Dir::NW(c) => c,
        }
    }

    fn is_floor(&self) -> bool {
        self.unwrap() == Cell::Floor
    }
}

impl PartialEq for Dir {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Dir::N(_) => matches!(other, Dir::N(_)),
            Dir::NE(_) => matches!(other, Dir::NE(_)),
            Dir::E(_) => matches!(other, Dir::E(_)),
            Dir::SE(_) => matches!(other, Dir::SE(_)),
            Dir::S(_) => matches!(other, Dir::S(_)),
            Dir::SW(_) => matches!(other, Dir::SW(_)),
            Dir::W(_) => matches!(other, Dir::W(_)),
            Dir::NW(_) => matches!(other, Dir::NW(_)),
        }
    }
}

type ScanFn = dyn Fn(&[Vec<Cell>], usize, usize, usize) -> Vec<Dir>;

fn _print_board(board: &[Vec<Cell>]) {
    for row in board {
        println!(
            "{}",
            join(
                row.iter()
                    .map(|c| match c {
                        Cell::Empty => 'L',
                        Cell::Occupied => '#',
                        Cell::Floor => '.',
                    })
                    .collect::<Vec<char>>(),
                ""
            )
        );
    }
    println!();
}

fn get_adjecent(board: &[Vec<Cell>], row: usize, col: usize, n: usize) -> Vec<Dir> {
    let mut res = vec![];

    if n >= board.len() || n >= board[0].len() {
        return res
    }

    // north
    if row > n - 1 {
        res.push(Dir::N(board[row - n][col]));
    }

    // west
    if col > n - 1 {
        res.push(Dir::W(board[row][col - n]));
    }

    // north-west
    if row > n - 1 && col > n - 1 {
        res.push(Dir::NW(board[row - n][col - n]));
    }

    // south
    if row < board.len() - n {
        res.push(Dir::S(board[row + n][col]));
    }

    // south-west
    if col > n - 1 && row < board.len() - n {
        res.push(Dir::SW(board[row + n][col - n]));
    }

    // south-east
    if col < board[0].len() - n && row < board.len() - n {
        res.push(Dir::SE(board[row + n][col + n]));
    }

    // east
    if col < board[0].len() - n {
        res.push(Dir::E(board[row][col + n]));
    }

    // north-east
    if col < board[0].len() - n && row > n - 1 {
        res.push(Dir::NE(board[row - n][col + n]));
    }

    res
}

fn scan_lines(board: &[Vec<Cell>], row: usize, col: usize, _n: usize) -> Vec<Dir> {
    let mut n = 1;
    let mut res = vec![];

    // not very effective in some cases but straightforward
    while res.len() < 8 {
        let adj = get_adjecent(board, row, col, n);

        if adj.is_empty() {
            break;
        }

        for dir in adj {
            if dir.is_floor() || res.contains(&dir){
                continue;
            }
            res.push(dir)
        }

        n += 1;
    }

    res
}

fn check_eq(board_1: &[Vec<Cell>], board_2: &[Vec<Cell>]) -> bool {
    board_1
        .iter()
        .flatten()
        .zip(board_2.iter().flatten())
        .try_for_each(
            |(c1, c2)| -> Result<(), ()> {
                if c1 != c2 {
                    Err(())
                } else {
                    Ok(())
                }
            },
        )
        .is_ok()
}

fn clone_board(board: &[Vec<Cell>]) -> Vec<Vec<Cell>> {
    let mut res = vec![];
    for row in board {
        res.push(row.clone());
    }
    res
}

fn update_board(
    scan_fn: &ScanFn,
    prev_board: &[Vec<Cell>],
    cur_board: &mut [Vec<Cell>],
    occupied_around: usize,
) {

    let f = |acc: usize, c: &Dir| {
        if c.unwrap() == Cell::Occupied {
            acc + 1
        } else {
            acc
        }
    };

    for row_idx in 0..prev_board.len() {
        for col_idx in 0..prev_board[row_idx].len() {
            match prev_board[row_idx][col_idx] {
                Cell::Empty => {
                    // no adjecent cells occupied
                    if scan_fn(&prev_board, row_idx, col_idx, 1)
                        .iter()
                        .all(|c| c.unwrap() == Cell::Empty || c.unwrap() == Cell::Floor)
                    {
                        cur_board[row_idx][col_idx] = Cell::Occupied;
                    }
                }
                Cell::Occupied => {
                    // occupied_around or more seats adjacent to it are also occupied, the seat becomes empty
                    if scan_fn(&prev_board, row_idx, col_idx, 1)
                        .iter()
                        .fold(0, f)
                        >= occupied_around
                    {
                        cur_board[row_idx][col_idx] = Cell::Empty;
                    }
                }
                _ => {}
            }
        }
    }
}

fn count_occupied(board: &[Vec<Cell>]) -> usize {
    board
        .iter()
        .flatten()
        .fold(0, |acc, c| if *c == Cell::Occupied { acc + 1 } else { acc })
}

fn first_star(board: &[Vec<Cell>]) -> usize {
    let mut prev_board = clone_board(board);
    let mut cur_board = clone_board(board);

    loop {
        update_board(&get_adjecent, &prev_board, &mut cur_board, 4);

        if check_eq(&prev_board, &cur_board) {
            break;
        }

        prev_board = clone_board(&cur_board);
    }

    count_occupied(&cur_board)
}

fn second_star(board: &[Vec<Cell>]) -> usize {
    let mut prev_board = clone_board(board);
    let mut cur_board = clone_board(board);

    loop {
        //_print_board(&cur_board);
        update_board(&scan_lines, &prev_board, &mut cur_board, 5);

        if check_eq(&prev_board, &cur_board) {
            break;
        }

        prev_board = clone_board(&cur_board);
    }

    count_occupied(&cur_board)
}

fn parse(input_raw: &str) -> Result<Vec<Vec<Cell>>, Error> {
    let res: Vec<Vec<Cell>> = split_by_lines(input_raw, &|line: &str| {
        let mut row = vec![];

        for c in line.chars() {
            row.push(match c {
                'L' => Cell::Empty,
                '.' => Cell::Floor,
                c => return Err(format_err!("unknown character '{}'", c)),
            });
        }

        Ok(row)
    })?;

    Ok(res)
}

pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let input = parse(input_raw)?;

    Ok(RetTypes::Usize(result(
        Ok(first_star(&input)),
        Ok(second_star(&input)),
    )))
}

#[cfg(test)]
mod tests {
    use super::{first_star, parse, second_star};

    const RAW_INPUT: &str = concat!(
        "L.LL.LL.LL\n",
        "LLLLLLL.LL\n",
        "L.L.L..L..\n",
        "LLLL.LL.LL\n",
        "L.LL.LL.LL\n",
        "L.LLLLL.LL\n",
        "..L.L.....\n",
        "LLLLLLLLLL\n",
        "L.LLLLLL.L\n",
        "L.LLLLL.LL",
    );

    #[test]
    fn test_first() {
        let board = parse(RAW_INPUT).unwrap();
        assert_eq!(first_star(&board), 37);
    }

    #[test]
    fn test_second() {
        let board = parse(RAW_INPUT).unwrap();
        assert_eq!(second_star(&board), 26);
    }
}
