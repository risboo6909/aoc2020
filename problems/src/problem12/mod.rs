use failure::{format_err, Error};
use utils::{result, split_by_lines, RetTypes};

#[derive(Copy, Clone)]
enum Op {
    N(isize),
    S(isize),
    E(isize),
    W(isize),
    L(isize),
    R(isize),
    F(isize),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

const DIRS: &[Dir] = &[Dir::E, Dir::S, Dir::W, Dir::N];
#[derive(Debug)]
struct Coords {
    x: isize,
    y: isize,
}

impl Coords {
    fn new(x: isize, y: isize) -> Self {
        Coords { x, y }
    }

    fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }
}

struct MovingShip {
    coords: Coords,
    waypoint: Coords,
    dir_idx: usize,
}

impl MovingShip {
    fn new() -> Self {
        MovingShip {
            dir_idx: 0, // pointer in DIRS list
            waypoint: Coords::new(10, 1),
            coords: Coords::new(0, 0),
        }
    }

    fn shift(&self, idx: usize, deg: isize) -> usize {
        let n = deg as usize / 90;
        (idx + n) % DIRS.len()
    }

    fn rev_idx(&self, idx: usize) -> usize {
        DIRS.len() - 1 - idx
    }

    fn rot(&mut self, op: Op) {
        if let Op::R(deg) = op {
            self.dir_idx = self.shift(self.dir_idx, deg);
        } else if let Op::L(deg) = op {
            let mut rev_idx = self.rev_idx(self.dir_idx);
            rev_idx = self.shift(rev_idx, deg);
            self.dir_idx = self.rev_idx(rev_idx);
        }
    }

    fn update_ship(&mut self, op: Op) {
        match op {
            Op::N(units) => {
                self.coords.y += units;
            }
            Op::S(units) => {
                self.coords.y -= units;
            }
            Op::E(units) => {
                self.coords.x += units;
            }
            Op::W(units) => {
                self.coords.x -= units;
            }
            Op::L(_) | Op::R(_) => self.rot(op),
            _ => {}
        }
    }

    fn move_ship(&mut self, op: Op) {
        match op {
            Op::F(units) => match &DIRS[self.dir_idx] {
                Dir::N => self.update_ship(Op::N(units)),
                Dir::S => self.update_ship(Op::S(units)),
                Dir::E => self.update_ship(Op::E(units)),
                Dir::W => self.update_ship(Op::W(units)),
            },
            _ => self.update_ship(op),
        }
    }

    fn rot_wp(&mut self, op: Op) {
        let mut local = Coords::new(
            self.waypoint.x - self.coords.x,
            self.waypoint.y - self.coords.y,
        );

        if let Op::L(deg) = op {
            match deg % 360 {
                90 => {
                    local.y *= -1;
                    local.swap();
                }
                180 => {
                    local.x *= -1;
                    local.y *= -1;
                }
                270 => {
                    local.x *= -1;
                    local.swap();
                }
                _ => {}
            }
        } else if let Op::R(deg) = op {
            match deg % 360 {
                90 => {
                    local.x *= -1;
                    local.swap();
                }
                180 => {
                    local.x *= -1;
                    local.y *= -1;
                }
                270 => {
                    local.y *= -1;
                    local.swap();
                }
                _ => {}
            }
        }

        self.waypoint = Coords::new(local.x + self.coords.x, local.y + self.coords.y);
    }

    fn update_wp(&mut self, op: Op) {
        match op {
            Op::N(units) => {
                self.waypoint.y += units;
            }
            Op::S(units) => {
                self.waypoint.y -= units;
            }
            Op::E(units) => {
                self.waypoint.x += units;
            }
            Op::W(units) => {
                self.waypoint.x -= units;
            }
            Op::L(_) | Op::R(_) => self.rot_wp(op),
            _ => {}
        }
    }

    fn move_waypoint(&mut self, op: Op) {
        match op {
            Op::F(units) => {
                // moves the ship to the waypoint
                let v = (
                    self.waypoint.x - self.coords.x,
                    self.waypoint.y - self.coords.y,
                );
                for _ in 0..units {
                    self.coords.x += v.0;
                    self.waypoint.x += v.0;

                    self.coords.y += v.1;
                    self.waypoint.y += v.1;
                }
            }
            _ => self.update_wp(op),
        }
    }
}

fn first_star(input: &[Op]) -> usize {
    let mut pt = MovingShip::new();
    for op in input {
        pt.move_ship(*op);
    }
    (pt.coords.x.abs() + pt.coords.y.abs()) as usize
}

fn second_star(input: &[Op]) -> usize {
    let mut pt = MovingShip::new();
    for op in input {
        pt.move_waypoint(*op);
    }
    (pt.coords.x.abs() + pt.coords.y.abs()) as usize
}

fn parse(input_raw: &str) -> Result<Vec<Op>, Error> {
    split_by_lines(input_raw, &|line: &str| {
        let (op, n_str) = (&line[..1], &line[1..]);
        let n = n_str.parse::<isize>()?;

        match op {
            "N" => Ok(Op::N(n)),
            "S" => Ok(Op::S(n)),
            "E" => Ok(Op::E(n)),
            "W" => Ok(Op::W(n)),
            "L" => Ok(Op::L(n)),
            "R" => Ok(Op::R(n)),
            "F" => Ok(Op::F(n)),
            _ => Err(format_err!("unknown op '{}'", op)),
        }
    })
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
    use super::{first_star, parse, second_star, Dir, MovingShip, Op, DIRS};

    const RAW_INPUT: &str = concat!("F10\n", "N3\n", "F7\n", "R90\n", "F11",);

    #[test]
    fn test_rot() {
        let mut pt = MovingShip::new();
        pt.move_ship(Op::R(450));
        assert_eq!(DIRS[pt.dir_idx], Dir::S);
        pt.move_ship(Op::L(270));
        assert_eq!(DIRS[pt.dir_idx], Dir::W);
    }

    #[test]
    fn test_first() {
        let input = parse(RAW_INPUT).unwrap();
        assert_eq!(first_star(&input), 25);
    }

    #[test]
    fn test_second() {
        let input = parse(RAW_INPUT).unwrap();
        assert_eq!(second_star(&input), 286);
    }
}
