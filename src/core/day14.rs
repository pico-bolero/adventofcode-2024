use crate::core::grid::Point;
use crate::core::modulus;
use regex::Regex;
use std::fmt::Error;
use std::str::FromStr;
/// Day 14 - Approach
///  I suspect that each robot will eventually repeat its path. The goal is to figure
///  out how many cycles are required before it repeats. Then it doesn't matter
///  how many total cycles are requested as you can get the position with a modulus operation
///  - there may be a way to figure that out just using delta and modifiers.

pub fn day14_part1(lines: &mut dyn Iterator<Item = String>) {
    let result = day14_part1_handler(lines, 101, 103, 100);
    println!("Sum {}", result);
}

fn day14_part1_handler(
    lines: &mut dyn Iterator<Item = String>,
    boundary_x: i32,
    boundary_y: i32,
    cycles: i32,
) -> u64 {
    let records: Vec<Record> = lines.flat_map(|x| Record::from_str(x.as_str())).collect();
    let points: Vec<Point<i32>> = cycle_records(&records, boundary_x, boundary_y, cycles);
    let safety_factor = calculate_safety_factor(&points, boundary_x, boundary_y);
    safety_factor
}

fn calculate_safety_factor(points: &[Point<i32>], boundary_x: i32, boundary_y: i32) -> u64 {
    let half_x = (boundary_x - 1) / 2;
    let half_y = (boundary_y - 1) / 2;
    let upper_left = (
        Point { x: 0, y: 0 },
        Point {
            x: half_x,
            y: half_y,
        },
    );
    let upper_right = (
        Point {
            x: half_x + 1,
            y: 0,
        },
        Point {
            x: boundary_x,
            y: half_y,
        },
    );
    let lower_left = (
        Point {
            x: 0,
            y: half_y + 1,
        },
        Point {
            x: half_x,
            y: boundary_y,
        },
    );
    let lower_right = (
        Point {
            x: half_x + 1,
            y: half_y + 1,
        },
        Point {
            x: boundary_x,
            y: boundary_y,
        },
    );
    let ul = points.iter().filter(|x| in_bounds(x, &upper_left)).count();
    let ur = points.iter().filter(|x| in_bounds(x, &upper_right)).count();
    let ll = points.iter().filter(|x| in_bounds(x, &lower_left)).count();
    let lr = points.iter().filter(|x| in_bounds(x, &lower_right)).count();
    let result = u64::try_from(ul * ur * ll * lr).unwrap();
    result
}

fn in_bounds(p: &Point<i32>, bounds: &(Point<i32>, Point<i32>)) -> bool {
    p.x >= bounds.0.x && p.x < bounds.1.x && p.y >= bounds.0.y && p.y < bounds.1.y
}

fn cycle_records(
    records: &Vec<Record>,
    boundary_x: i32,
    boundary_y: i32,
    cycles: i32,
) -> Vec<Point<i32>> {
    records
        .iter()
        .map(|r| cycle_record(r, boundary_x, boundary_y, cycles))
        .collect()
}

// Calculate the end point after a certain numbers of cycles a.k.a steps
fn cycle_record(r: &Record, boundary_x: i32, boundary_y: i32, cycles: i32) -> Point<i32> {
    Point {
        x: modulus::modulus(r.start.x + cycles * r.vector.x, boundary_x),
        y: modulus::modulus(r.start.y + cycles * r.vector.y, boundary_y),
    }
}

struct Record {
    start: Point<i32>,
    vector: Point<i32>,
}

impl FromStr for Record {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reg = Regex::new(r"p=(-?\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let mut itr = reg.captures_iter(s);
        let captures = itr.next().unwrap();
        let (_fullstr, [p_x, p_y, v_x, v_y]) = captures.extract();
        let radix = 10u32;
        let point: Point<i32> = Point {
            x: i32::from_str_radix(p_x, radix).unwrap(),
            y: i32::from_str_radix(p_y, radix).unwrap(),
        };
        let vector: Point<i32> = Point {
            x: i32::from_str_radix(v_x, radix).unwrap(),
            y: i32::from_str_radix(v_y, radix).unwrap(),
        };
        Ok(Record {
            start: point,
            vector,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<String> {
        let lines: Vec<String> = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
            .split('\n')
            .map(|x| x.to_string())
            .collect();
        lines
    }

    #[test]
    fn test_day14_part1_handler() {
        let lines = sample_data();
        let calculated = day14_part1_handler(&mut lines.iter().map(|x| x.to_string()), 11, 7, 100);
        assert_eq!(12, calculated);
    }

    #[test]
    fn test_cycle_records() {
        let lines = sample_data();
        let records: Vec<Record> = lines
            .iter()
            .flat_map(|x| Record::from_str(x.as_str()))
            .collect();
        let points: Vec<Point<i32>> = cycle_records(&records, 11, 7, 100);
        // spot checks
        /* - all possible locations and counts
        ......2..1.
        ...........
        1..........
        .11........
        .....1.....
        ...12......
        .1....1....
        */
        assert!(points.iter().any(|x| *x == Point { x: 1, y: 6 }));
        assert!(points.iter().any(|x| *x == Point { x: 9, y: 0 }));
        assert_eq!(
            2,
            points
                .iter()
                .filter(|x: &&Point<i32>| **x == Point { x: 6, y: 0 })
                .count()
        );
        assert_eq!(
            2,
            points
                .iter()
                .filter(|x: &&Point<i32>| **x == Point { x: 4, y: 5 })
                .count()
        );
    }

    #[test]
    fn test_cycle_record() {
        let r = Record::from_str("p=2,4 v=2,-3").unwrap();
        let p = cycle_record(&r, 11, 7, 5);
        assert_eq!(1, p.x);
        assert_eq!(3, p.y);
    }

    #[test]
    fn test_record_from_str() {
        let s = "p=91,23 v=-98,-65";
        let r = Record::from_str(s).unwrap();
        assert_eq!(r.start.x, 91);
        assert_eq!(r.start.y, 23);
        assert_eq!(r.vector.x, -98);
        assert_eq!(r.vector.y, -65);
    }

    #[test]
    fn test_boundary_wrap_experiment() {
        // grid boundary 11
        // step is 3
        // initial_position is 2
        // cycle 0  1  2  3   4   5   6   7   8   9   10
        // step: 2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32
        //                11              22,
        // pos   2, 5, 8, 0,  3,  6,  9,  1,  4,  7,  10
        // formula = (initial_position + cycles * step) mod (boundary)
        let initial_position = 2u64;
        let cycles = 9u64;
        let step = 3u64;
        let boundary = 11u64;
        assert_eq!(
            7,
            modulus::modulus(initial_position + cycles * step, boundary)
        );

        // grid boundary 11
        // step is -3
        // initial_position is 2
        // cycle 0   1    2   3    4    5    6    7    8    9
        // step: 2, -1,  -4, -7, -10, -13, -16, -19  -22, -25
        //                             11,            11
        // pos   2, 10,  7,   4,   1,   9,   6,  3,    0,   8,

        let initial_position = 2i64;
        let cycles = 9i64;
        let step = -3i64;
        let boundary = 11i64;
        for cycle in 0..cycles {
            let position = modulus::modulus(initial_position + cycle * step, boundary);
            println!("{}", position);
        }
        assert_eq!(
            8,
            modulus::modulus(initial_position + cycles * step, boundary)
        );
    }
}
