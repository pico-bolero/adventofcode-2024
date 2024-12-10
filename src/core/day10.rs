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
