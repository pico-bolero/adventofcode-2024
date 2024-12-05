/// Receives input and prints output
pub fn day05_part1(lines: &mut dyn Iterator<Item = String>) {
    let result: u32 = day05_part1_handler(lines);
    println!("Sum {}", result);
}

fn day05_part1_handler(lines: &mut (dyn Iterator<Item = String>)) -> u32 {
    /*
    The data intially comes into two chunks. Please on a double return.
        1. process chunk 1 into rules
        2. process chunk 2 into the data to validate
    */
    todo!()
}

enum Rule {
    ExistsBefore { page: u32 },
}

struct PageRules {
    page: u32,
    rules: Vec<Rule>,
}

/// Accepts a string like 23|32 and returns a tuple
fn parse_rule_str(input: &str) -> Result<(u32, u32), ()> {
    let splits: Vec<&str> = input.split("|").collect();
    if splits.len() != 2 {
        return Err(());
    }
    let left = splits[0].parse::<u32>();
    let right = splits[1].parse::<u32>();
    match (left, right) {
        (Ok(x), Ok(y)) => Ok((x, y)),
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<String> {
        let lines: Vec<String> = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            .split('\n')
            .map(|x| x.to_string())
            .collect();
        lines
    }

    #[test]
    fn test_day05_part1_handler() {
        let lines = sample_data();
        let calculated = day05_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(143, calculated);
    }

    #[test]
    fn test_parse_rule_str() {
        assert_eq!(Ok((64u32, 46u32)), parse_rule_str("64|46"));
        assert_eq!(Err(()), parse_rule_str("6446"));
        assert_eq!(Err(()), parse_rule_str("ab|bc"));
    }
}
