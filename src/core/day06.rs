/// Receives input and prints output
pub fn day06_part1(lines: &mut dyn Iterator<Item = String>) {
    let result: u32 = day06_part1_handler(lines);
    println!("Sum {}", result);
}

fn day06_part1_handler(lines: &mut (dyn Iterator<Item = String>)) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<String> {
        let lines: Vec<String> = "".split('\n').map(|x| x.to_string()).collect();
        lines
    }

    #[test]
    fn test_day06_part1_handler() {
        let lines = sample_data();
        // let calculated = day06_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        // assert_eq!(143, calculated);
    }
}
