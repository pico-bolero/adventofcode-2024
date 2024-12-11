/// Approach Part1:
/// - Parse the input into a grid
/// - the goal is to find all the paths for the trail heads at position 0
///   - iterate through the grid to find all the 0 positions and add them to a HashMap
/// - brute force: walk each direction in the graph if a value of the next number is there. That will
///     walk each path. Is there a way to limit the search space? Start at the end '9'? Start at 5 and branch?
/// - When a path is complete, capture it, add 1 to trailhead hashmap

/// Receives input and prints output
pub fn day10_part1(lines: &mut dyn Iterator<Item = String>) {
    let result: u64 = day10_part1_handler(lines);
    println!("Sum {}", result);
}

fn day10_part1_handler(lines: &mut (dyn Iterator<Item = String>)) -> u64 {
    let input = lines.next().unwrap(); // There is only one line of input
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<String> {
        let lines: Vec<String> = "2333133121414131402"
            .split('\n')
            .map(|x| x.to_string())
            .collect();
        lines
    }

    #[test]
    fn test_day10_part1_handler() {
        assert!(true);
    }
}
