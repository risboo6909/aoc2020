use failure::Error;
use utils::{result, RetTypes};

const SLOPES: &[Slope] = &[
    Slope { right: 1, down: 1 },
    Slope { right: 3, down: 1 },
    Slope { right: 5, down: 1 },
    Slope { right: 7, down: 1 },
    Slope { right: 1, down: 2 },
];

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    Obstacle,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Obstacle,
            _ => Cell::Empty,
        }
    }
}

struct Slope {
    right: usize,
    down: usize,
}

// let's have some fun with const generics
struct Board {
    board: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Board {
    fn new(board: Vec<Cell>, width: usize, height: usize) -> Self {
        Board {
            board,
            width,
            height,
        }
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.width + (col % self.width)
    }

    #[inline(always)]
    fn get_cell(&self, row: usize, col: usize) -> Cell {
        let idx = self.get_index(row, col);
        self.board[idx]
    }
}

fn first_star(board: &Board, slope: &Slope) -> usize {
    let mut cur_col = 0;
    let mut cur_row = 0;

    let mut trees_count = 0;

    while cur_row < board.height {
        if board.get_cell(cur_row, cur_col) == Cell::Obstacle {
            trees_count += 1;
        }

        cur_col += slope.right;
        cur_row += slope.down;
    }

    trees_count
}

fn second_star(board: &Board, slopes: &[Slope]) -> usize {
    Iterator::product(
        slopes.iter().map(|slope| first_star(board, slope)),
    )
}

fn parse(input_raw: &[u8]) -> Board {
    let mut field = Vec::new();

    let mut rows = 0;
    let mut cols = 0;

    for c in input_raw {
        let c = *c as char;

        if c == '\n' {
            rows += 1;
            cols = 0;
        } else {
            field.push(Cell::from(c));
            cols += 1;
        }
    }

    // rows + 1 because there is must be no '\n' at the end of file
    Board::new(field, cols, rows + 1)
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_bytes!("./input");
    let board = parse(input_raw);

    Ok(RetTypes::Usize(result(
        Ok(first_star(&board, &Slope { right: 3, down: 1 })),
        Ok(second_star(&board, SLOPES)),
    )))
}

#[cfg(test)]
mod tests {
    use super::{first_star, parse, second_star, Slope, SLOPES};

    const RAW_INPUT: &[u8] = concat!(
        "..##.......\n",
        "#...#...#..\n",
        ".#....#..#.\n",
        "..#.#...#.#\n",
        ".#...##..#.\n",
        "..#.##.....\n",
        ".#.#.#....#\n",
        ".#........#\n",
        "#.##...#...\n",
        "#...##....#\n",
        ".#..#...#.#"
    )
    .as_bytes();

    #[test]
    fn test_parse() {
        let board = parse(RAW_INPUT);

        assert_eq!(board.width, 11);
        assert_eq!(board.height, 11);
    }

    #[test]
    fn test_first() {
        let board = parse(RAW_INPUT);
        assert_eq!(first_star(&board, &Slope { right: 3, down: 1 }), 7);
    }

    #[test]
    fn test_second() {
        let board = parse(RAW_INPUT);
        assert_eq!(second_star(&board, SLOPES), 336);
    }
}
