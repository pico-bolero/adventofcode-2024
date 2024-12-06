use std::collections::HashSet;

/// Receives input and prints output
pub fn day06_part1(lines: &mut dyn Iterator<Item = String>) {
    let result: u32 = day06_part1_handler(lines);
    println!("Sum {}", result);
}

fn day06_part1_handler(lines: &mut (dyn Iterator<Item = String>)) -> u32 {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let (actor, bounds, objects) = extract_actor_bounds_and_objects(lines);
    0u32
}

// parses the input
fn extract_actor_bounds_and_objects(
    lines: &mut (dyn Iterator<Item = String>),
) -> ((usize, usize), (usize, usize), HashSet<(usize, usize)>) {
    let mut objects: HashSet<(usize, usize)> = HashSet::new();
    let mut actor: (usize, usize) = (0, 0);
    let mut bounds: (usize, usize) = (0, 0);
    lines.enumerate().for_each(|(y, line)| {
        bounds.1 = bounds.1.max(y);
        line.chars().enumerate().for_each(|(x, c)| {
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
        // let calculated = day06_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        // assert_eq!(143, calculated);
    }

    #[test]
    fn test_extract_actor_bounds_objects() {
        let lines = sample_data();
        let (actor, bounds, objects) =
            extract_actor_bounds_and_objects(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(10, bounds.0 + 1); // number of rows
        assert_eq!(10, bounds.1 + 1); // number of cols
        assert_eq!((4usize, 6usize), actor);
        // spot check some objects
        assert!(objects.contains(&(4usize, 0usize)));
        assert!(objects.contains(&(6usize, 9usize)));
        assert!(!objects.contains(&(0usize, 0usize)));
        assert!(!objects.contains(&(9usize, 9usize)));
    }
}
