/// Solution:
// Parse the data into a sparse HashMap<Point,Char>
// Iterate over the values to create a new map of HashMap<Char,Vec<Point>> (maybe this first?)
// For each Char, Vec<Point>: permutation Vec(<char>,<Point>,<Point>), calculate the slope.
//   slope = delta x, delta y. Emit an antinode<Point> that extends the line on each end
// Aggregate the antinodes into a HashSet
// Sum up the antinodes in the grid area
pub fn day08_part1(lines: &mut dyn Iterator<Item = String>) {
    let result: u64 = day08_part1_handler(lines);
    println!("Sum {}", result);
}

fn day08_part1_handler(_lines: &mut (dyn Iterator<Item = String>)) -> u64 {
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
    fn test_day08_part1_handler() {
        let lines = sample_data();
        // let calculated = day08_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        //assert_eq!(14, calculated);
    }
}
