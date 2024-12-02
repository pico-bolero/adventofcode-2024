/// Receives input and prints output
pub fn day02_part1(lines: &mut dyn Iterator<Item = String>) {
    let safe_count: usize = day02_part1_handler(lines);
    println!("Sum {}", safe_count);
}

pub fn day02_part1_handler(lines: &mut dyn Iterator<Item = String>) -> usize {
    let safe_count: usize = lines.filter(report_is_safe).count();
    safe_count
}

fn report_is_safe(input: &String) -> bool {
    let nums: Vec<u32> = parse_str_with_separator(input.as_str(), " ");
    is_increasing(&nums) || is_decreasing(&nums)
}

pub fn is_increasing(input: &Vec<u32>) -> bool {
    let result = input
        .iter()
        .zip(input.iter().skip(1))
        .map(|(a, b)| (*a, *b))
        .filter(|(a, b)| a < b && b - a > 0 && b - a <= 3)
        .collect::<Vec<_>>();
    input.len() - 1 == result.len()
}

pub fn is_decreasing(input: &Vec<u32>) -> bool {
    let result = input
        .iter()
        .zip(input.iter().skip(1))
        .map(|(a, b)| (*a, *b))
        .filter(|(a, b)| a > b && a - b > 0 && a - b <= 3)
        .collect::<Vec<_>>();
    input.len() - 1 == result.len()
}

pub fn parse_str_with_separator(input: &str, delimiter: &str) -> Vec<u32> {
    let parsed: Vec<u32> = input
        .trim()
        .split(delimiter)
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().expect("Should parse into u32"))
        .collect();
    parsed
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<String> {
        let lines: Vec<String> = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .split('\n')
            .map(|x| x.to_string())
            .collect();
        lines
    }

    #[test]
    fn test_day02_par1_handler() {
        let lines = sample_data();
        let calculated = day02_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(2, calculated);
    }

    #[test]
    fn test_is_increasing() {
        let input = vec![0, 1, 2, 3, 5];
        assert!(is_increasing(&input));
        let input = vec![0, 1, 2, 3, 3];
        assert!(!is_increasing(&input));
        let input = vec![0, 1, 2, 3, 2];
        assert!(!is_increasing(&input));
    }

    #[test]
    fn test_is_descreasing() {
        let input = vec![5, 3, 2, 1, 0];
        assert!(is_decreasing(&input));
        let input = vec![6, 6, 3, 4, 2, 1, 0];
        assert!(!is_decreasing(&input));
        let input = vec![6, 7, 3, 4, 2, 1, 0];
        assert!(!is_decreasing(&input));
    }
}
