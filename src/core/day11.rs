use crate::core::parser;

/// Receives input and prints output
pub fn day11_part1(lines: &mut dyn Iterator<Item = String>) {
    let result: usize = day11_part1_handler(lines);
    println!("Sum {}", result);
}

fn day11_part1_handler(lines: &mut (dyn Iterator<Item = String>)) -> usize {
    let input = lines.next().unwrap(); // There is only one line of input
    let mut line: Vec<u64> = parser::parse_delimited_str(&input, " ");
    for _ in 0..25 {
        line = generate_next_line(line);
    }
    line.len()
}

fn generate_next_line(line: Vec<u64>) -> Vec<u64> {
    line.iter().flat_map(|x| transform_u64(*x)).collect()
}

fn transform_u64(input: u64) -> Vec<u64> {
    match input {
        0 => zero_to_one(input),
        _ if has_even_length(input) => split_even_length(input),
        _ => multiply_by_2024(input),
    }
}

fn zero_to_one(input: u64) -> Vec<u64> {
    assert!(input == 0);
    vec![1u64]
}

fn has_even_length(input: u64) -> bool {
    let s = format!("{}", input);
    s.len() % 2 == 0
}

fn split_even_length(input: u64) -> Vec<u64> {
    assert!(has_even_length(input));
    let s = format!("{}", input);
    const RADIX: u32 = 10;
    let left = &s[0..s.len() / 2];
    let right = &s[s.len() / 2..];

    vec![
        u64::from_str_radix(left, RADIX),
        u64::from_str_radix(right, RADIX),
    ]
    .iter()
    .cloned()
    .flat_map(|x| x.ok())
    .collect()
}

fn multiply_by_2024(input: u64) -> Vec<u64> {
    vec![input * 2024u64]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<String> {
        let lines: Vec<String> = "125 17".split('\n').map(|x| x.to_string()).collect();
        lines
    }

    #[test]
    fn test_day11_part1_handler() {
        let lines = sample_data();
        let calculated = day11_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(55312, calculated);
    }
}
