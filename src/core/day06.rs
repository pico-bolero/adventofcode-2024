use std::collections::HashSet;

/// Receives input and prints output
pub fn day06_part1(lines: &mut dyn Iterator<Item = String>) {
    let result: usize = day06_part1_handler(lines);
    println!("Sum {}", result);
}

fn day06_part1_handler(lines: &mut (dyn Iterator<Item = String>)) -> usize {
    let (path, _exit, _bounds, _objects) = walking_path_scenario(lines);
    let path = path.expect("Part 1 is doesn't fail");
    let visited: HashSet<(isize, isize)> = path.iter().map(|w| w.point).collect();
    visited.len()
}

pub fn day06_part2(lines: &mut dyn Iterator<Item = String>) {
    let result: usize = day06_part2_handler(lines);
    println!("Sum {}", result);
}

fn day06_part2_handler(lines: &mut (dyn Iterator<Item = String>)) -> usize {
    // Need to cycle through these many times, so pull it into memory
    let all_lines: Vec<String> = lines.collect();

    // Get the initial successful path
    let (path, _exit, bounds, mut objects) =
        walking_path_scenario(&mut all_lines.iter().map(|x| x.to_string()));
    let path = path.expect("Part 1 is doesn't fail");
    let actor = path[0].point;

    // walk backwards each step through the waypoints
    // the path already has every waypoint
    // let steps: Vec<WayPoint> = backwards_steps(path.clone(), exit.clone());

    let mut steps = path.clone();
    steps.reverse();

    let mut cycle_causing_objects: HashSet<(isize, isize)> = HashSet::new();

    steps.iter().for_each(|step| {
        // replace the current point with an obstruction and walk the path
        objects.insert(step.point);
        let path_info = calculate_path(actor, bounds, &objects);
        objects.remove(&step.point);

        // When there is no path info, there is a cycle. Add the step as a result
        if path_info.is_none() {
            cycle_causing_objects.insert(step.point);
        }
    });
    cycle_causing_objects.len()
}

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
struct WayPoint {
    point: (isize, isize),
    direction: Direction,
}

fn walking_path_scenario(
    lines: &mut (dyn Iterator<Item = String>),
) -> (
    Option<Vec<WayPoint>>,
    Option<WayPoint>,
    (isize, isize),
    HashSet<(isize, isize)>,
) {
    let (actor, bounds, objects) = extract_actor_bounds_and_objects(lines);
    if let Some((path, exit)) = calculate_path(actor, bounds, &objects) {
        return (Some(path), Some(exit), bounds, objects);
    }
    (None, None, bounds, objects)
}

fn calculate_path(
    mut actor: (isize, isize),
    bounds: (isize, isize),
    objects: &HashSet<(isize, isize)>,
) -> Option<(Vec<WayPoint>, WayPoint)> {
    let mut path = vec![];
    let mut visited: HashSet<WayPoint> = HashSet::new();
    let mut d = Direction::North;

    while in_bounds(actor, bounds) {
        let way_point = WayPoint {
            point: actor,
            direction: d,
        };

        // Whoah! We already visited this waypoint from this direction.
        if !visited.insert(way_point) {
            return None;
        }

        path.push(way_point);
        if is_facing_object(&actor, &d, objects) {
            d = d.turn_right();
        }
        actor = d.step(actor);
    }

    // Exit Point
    let exit = WayPoint {
        direction: d,
        point: d.step_backward(actor),
    };

    Some((path, exit))
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

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn step(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match *self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
        }
    }

    fn step_backward(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match *self {
            Direction::North => (x, y + 1),
            Direction::South => (x, y - 1),
            Direction::East => (x - 1, y),
            Direction::West => (x + 1, y),
        }
    }

    fn turn_right(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
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
    fn test_day06_part2_handler() {
        let lines = sample_data();
        let calculated = day06_part2_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(6, calculated);
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
        let mut dir = Direction::South;

        dir = dir.turn_right();
        assert_eq!(dir, Direction::West);
        point = dir.step(point);
        assert_eq!((0, 1), point);

        dir = dir.turn_right();
        assert_eq!(dir, Direction::North);
        point = dir.step(point);
        assert_eq!((0, 0), point);

        dir = dir.turn_right();
        assert_eq!(dir, Direction::East);
        point = dir.step(point);
        assert_eq!((1, 0), point);

        dir = dir.turn_right();
        assert_eq!(dir, Direction::South);
        point = dir.step(point);
        assert_eq!((1, 1), point);

        point = dir.step_backward(point);
        assert_eq!((1, 0), point);

        dir = dir.turn_left();
        assert_eq!(dir, Direction::East);
        point = dir.step_backward(point);
        assert_eq!((0, 0), point);

        dir = dir.turn_left();
        assert_eq!(dir, Direction::North);
        point = dir.step_backward(point);
        assert_eq!((0, 1), point);

        dir = dir.turn_left();
        assert_eq!(dir, Direction::West);
        point = dir.step_backward(point);
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
        assert!(!is_facing_object(&actor, &Direction::North, &objects));
        assert!(!is_facing_object(&actor, &Direction::South, &objects));
        assert!(!is_facing_object(&actor, &Direction::East, &objects));
        assert!(!is_facing_object(&actor, &Direction::West, &objects));

        // facing everything!
        objects.insert((0, 1));
        objects.insert((1, 0));
        objects.insert((2, 1));
        objects.insert((1, 2));
        assert!(is_facing_object(&actor, &Direction::North, &objects));
        assert!(is_facing_object(&actor, &Direction::South, &objects));
        assert!(is_facing_object(&actor, &Direction::East, &objects));
        assert!(is_facing_object(&actor, &Direction::West, &objects));
    }
}
