use std::collections::HashSet;

/// Receives input and prints output
pub fn day06_part1(lines: &mut dyn Iterator<Item = String>) {
    let result: usize = day06_part1_handler(lines);
    println!("Sum {}", result);
}

fn day06_part1_handler(lines: &mut (dyn Iterator<Item = String>)) -> usize {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let (mut actor, bounds, objects) = extract_actor_bounds_and_objects(lines);
    let mut d = Direction::NORTH;

    // Actor position and direction should be in a struct together

    while in_bounds(actor, bounds) {
        visited.insert(actor);
        if is_facing_object(&actor, &d, &objects) {
            d = d.turn_right();
        }
        actor = d.step(actor);
    }
    visited.len()
}

/// If the next step would touch an object, then the actor is facing an object
fn is_facing_object(
    actor: &(isize, isize),
    d: &Direction,
    objects: &HashSet<(isize, isize)>,
) -> bool {
    let next_step = d.step(*actor);
    objects.contains(&next_step)
}

/// checks the bounds which are integer values. Negative is out of bounds
fn in_bounds(actor: (isize, isize), bounds: (isize, isize)) -> bool {
    0 <= actor.0 && actor.0 <= bounds.0 && 0 <= actor.1 && actor.1 <= bounds.1
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Direction {
    fn step(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match *self {
            Direction::NORTH => (x, y - 1),
            Direction::SOUTH => (x, y + 1),
            Direction::EAST => (x + 1, y),
            Direction::WEST => (x - 1, y),
        }
    }

    fn turn_right(&self) -> Direction {
        match *self {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        }
    }
}

// parses the input
fn extract_actor_bounds_and_objects(
    lines: &mut (dyn Iterator<Item = String>),
) -> ((isize, isize), (isize, isize), HashSet<(isize, isize)>) {
    let mut objects: HashSet<(isize, isize)> = HashSet::new();
    let mut actor: (isize, isize) = (0, 0);
    let mut bounds: (isize, isize) = (0, 0);
    lines.enumerate().for_each(|(y, line)| {
        let y = isize::try_from(y).expect("y should fit");
        bounds.1 = bounds.1.max(y);
        line.chars().enumerate().for_each(|(x, c)| {
            let x = isize::try_from(x).expect("x should fit");
            bounds.0 = bounds.0.max(x);
            match c {
                '^' => {
                    actor = (x, y); // coordinate space will be a little wonky. Up has decreasing y values
                }
                '#' => {
                    objects.insert((x, y)); // coordinate space will be a little wonky. Up has decreasing y values
                }
                _ => {}
            }
        });
    });
    (actor, bounds, objects)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<String> {
        let lines: Vec<String> = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .split('\n')
            .map(|x| x.to_string())
            .collect();
        lines
    }

    #[test]
    fn test_day06_part1_handler() {
        let lines = sample_data();
        let calculated = day06_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(41, calculated);
    }

    #[test]
    fn test_extract_actor_bounds_objects() {
        let lines = sample_data();
        let (actor, bounds, objects) =
            extract_actor_bounds_and_objects(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(10, bounds.0 + 1); // number of rows
        assert_eq!(10, bounds.1 + 1); // number of cols
        assert_eq!((4isize, 6isize), actor);
        // spot check some objects
        assert!(objects.contains(&(4isize, 0isize)));
        assert!(objects.contains(&(6isize, 9isize)));
        assert!(!objects.contains(&(0isize, 0isize)));
        assert!(!objects.contains(&(9isize, 9isize)));
    }

    #[test]
    fn test_directions() {
        let mut point = (1isize, 1isize);
        let mut dir = Direction::SOUTH;

        dir = dir.turn_right();
        assert_eq!(dir, Direction::WEST);
        point = dir.step(point);
        assert_eq!((0, 1), point);

        dir = dir.turn_right();
        assert_eq!(dir, Direction::NORTH);
        point = dir.step(point);
        assert_eq!((0, 0), point);

        dir = dir.turn_right();
        assert_eq!(dir, Direction::EAST);
        point = dir.step(point);
        assert_eq!((1, 0), point);

        dir = dir.turn_right();
        assert_eq!(dir, Direction::SOUTH);
        point = dir.step(point);
        assert_eq!((1, 1), point);
    }

    #[test]
    fn test_in_bounds() {
        let actor = (0isize, 0isize);
        let bounds = (2isize, 2isize);
        assert!(in_bounds(actor, bounds));

        let actor = (2isize, 2isize);
        let bounds = (2isize, 2isize);
        assert!(in_bounds(actor, bounds));

        // Out of bounds
        let actor = (-1isize, 0isize);
        assert!(!in_bounds(actor, bounds));

        let actor = (0isize, -1isize);
        assert!(!in_bounds(actor, bounds));

        let actor = (3isize, 0isize);
        assert!(!in_bounds(actor, bounds));

        let actor = (0isize, 3isize);
        assert!(!in_bounds(actor, bounds));
    }

    #[test]
    fn test_is_facing_object() {
        let mut objects: HashSet<(isize, isize)> = HashSet::new();

        let actor = (1isize, 1isize);
        // Not facing anything
        assert!(!is_facing_object(&actor, &Direction::NORTH, &objects));
        assert!(!is_facing_object(&actor, &Direction::SOUTH, &objects));
        assert!(!is_facing_object(&actor, &Direction::EAST, &objects));
        assert!(!is_facing_object(&actor, &Direction::WEST, &objects));

        // facing everything!
        objects.insert((0, 1));
        objects.insert((1, 0));
        objects.insert((2, 1));
        objects.insert((1, 2));
        assert!(is_facing_object(&actor, &Direction::NORTH, &objects));
        assert!(is_facing_object(&actor, &Direction::SOUTH, &objects));
        assert!(is_facing_object(&actor, &Direction::EAST, &objects));
        assert!(is_facing_object(&actor, &Direction::WEST, &objects));
    }
}
