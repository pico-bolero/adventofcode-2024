/// Receives input and prints output
pub fn day01_part1(lines: &mut dyn Iterator<Item = String>) {
    println!("Sum {}", day01_part1_handler(lines));
}

/// Receives input and prints output
pub fn day01_part2(lines: &mut dyn Iterator<Item = String>) {
    println!("Sum {}", day01_part2_handler(lines));
}

/// Processes the data according to the rules for Day 1 Part 1
/// Read the input and split into tuples and unzip into vecs
/// Sort the vecs and calculate the deltas between the items
fn day01_part1_handler(lines: &mut dyn Iterator<Item = String>) -> u32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = lines.map(parse_to_tuple).unzip();
    left.sort();
    right.sort();
    let deltas: Vec<u32> = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| if a > b { a - b } else { b - a })
        .collect();
    deltas.iter().sum()
}

/// Processes the data according to the rules for Day 1 Part 2
/// Read the input and split into tuples and unzip into vecs
/// Iterate over the the first vec, using that value count all the instances in the other vec.
fn day01_part2_handler(lines: &mut dyn Iterator<Item = String>) -> u32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = lines.map(parse_to_tuple).unzip();
    left.sort();
    right.sort();

    let similarities: Vec<(u32, u32)> = left
        .iter()
        // The left isn't really needed, but used for debugging
        .map(|x| (*x, right.iter().filter(|y| x == *y).sum()))
        .collect();
    similarities.iter().map(|(_a, b)| *b).sum()
}

fn parse_to_tuple(line: String) -> (u32, u32) {
    let result: Vec<u32> = line
        .split(' ')
        .map(|x| x.trim())
        .flat_map(|x| x.parse::<u32>())
        .collect();
    assert!(result.len() == 2);
    (result[0], result[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<String> {
        let lines: Vec<String> = "3   4
                                  4   3
                                  2   5
                                  1   3
                                  3   9
                                  3   3"
            .split('\n')
            .map(|x| x.to_string())
            .collect();
        lines
    }

    #[test]
    fn test_day01_part1_handler() {
        let lines = sample_data();
        let calculated = day01_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(11, calculated);
    }

    #[test]
    fn test_day01_part2_handler() {
        let lines = sample_data();
        let calculated = day01_part2_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(31, calculated);
    }
}
