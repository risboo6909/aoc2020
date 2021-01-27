use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use failure::Error;
use utils::{result, RetTypes};

mod point_trait;
use point_trait::Point;

mod point3d;
use point3d::Point3D;

mod point4d;
use point4d::Point4D;

fn count_active<T>(coords: T, univere: &HashSet<T>) -> usize
where
    T: Point + Eq + Hash + Copy,
{
    coords.get_vicinity().iter().fold(0, |acc, point| {
        if univere.contains(point) {
            acc + 1
        } else {
            acc
        }
    })
}

fn engine<T>(initial_cubes: &[T]) -> HashSet<T>
where
    T: Point + Eq + Hash + Copy + Debug,
{
    let mut cur_universe: HashSet<T> = HashSet::new();

    for cube in initial_cubes {
        cur_universe.insert(cube.clone());
    }

    for _ in 0..6 {
        let mut next_universe: HashSet<T> = HashSet::new();

        let mut visited: HashSet<T> = HashSet::new();
        let mut to_visit: Vec<T> = vec![];

        to_visit.extend(cur_universe.iter());

        while !to_visit.is_empty() {
            // unwrap with ease, because it has been checked above in while-loop body
            let coords = to_visit.pop().unwrap();

            // get vicinity coords for only those volumes which aren't empty
            if cur_universe.contains(&coords) {
                let vicinity = coords.get_vicinity();

                // extend, ignoring already visited points
                for point in vicinity.iter() {
                    if !visited.contains(&point) {
                        to_visit.push(*point);
                    }
                }

                let num_active = count_active(coords, &cur_universe);

                // cube remains active
                if num_active == 2 || num_active == 3 {
                    next_universe.insert(coords);
                }
            } else {
                // empty volume case
                let num_active = count_active(coords, &cur_universe);

                // new cube appears
                if num_active == 3 {
                    next_universe.insert(coords);
                }
            }

            visited.insert(coords);
        }

        // swap universe
        cur_universe = next_universe;
    }

    cur_universe
}

fn first_star(initial_cubes: &[Point3D]) -> usize {
    let universe = engine(&initial_cubes);
    universe.len()
}

fn second_star(initial_cubes: &[Point3D]) -> usize {
    let initial_cubes_4d: Vec<Point4D> = initial_cubes.iter().cloned().map(Point4D::from).collect();
    let universe = engine(&initial_cubes_4d);
    universe.len()
}

fn parse(input_raw: &str) -> Vec<Point3D> {
    let mut res = Vec::new();
    for (row_idx, line) in input_raw.lines().enumerate() {
        for (col_idx, c) in line.chars().enumerate() {
            if c == '#' {
                res.push(Point3D {
                    x: col_idx as isize,
                    y: row_idx as isize,
                    z: 0,
                })
            }
        }
    }
    res
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let cubes = parse(input_raw);

    Ok(RetTypes::Usize(result(
        Ok(first_star(&cubes)),
        Ok(second_star(&cubes)),
    )))
}
