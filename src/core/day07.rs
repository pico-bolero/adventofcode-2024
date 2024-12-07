/// Receives input and prints output
pub fn day07_part1(lines: &mut dyn Iterator<Item = String>) {
    let result: u64 = day07_part1_handler(lines);
    println!("Sum {}", result);
}

fn day07_part1_handler(lines: &mut (dyn Iterator<Item = String>)) -> u64 {
    let entries: Vec<(u64, Vec<u64>)> = lines.map(|line| parse_line(line.as_str())).collect();
    entries
        .iter()
        .filter(|(test_value, inputs)| can_be_calculated(test_value, inputs))
        .map(|(test_value, _)| test_value)
        .sum()
}

fn add(x: u64, y: u64) -> u64 {
    x + y
}

fn multiply(x: u64, y: u64) -> u64 {
    x * y
}

fn arithematic_combos(size: usize) -> Vec<Vec<fn(u64, u64) -> u64>> {
    let mut results: Vec<Vec<fn(u64, u64) -> u64>> = vec![];
    /* Dyanmic number of operators to generate made me think of a bit patterns
    +++ ++* +*+ *++ +** *+* **+ *** */

    for i in 0..(2usize.pow(size.try_into().unwrap())) {
        let mut operator_fns: Vec<fn(u64, u64) -> u64> = vec![];
        let mut bitmap = i;
        for _ in 0..size {
            match 1 & bitmap {
                0 => {
                    operator_fns.push(add);
                }
                1 => {
                    operator_fns.push(multiply);
                }
                _ => {
                    panic!("Should have been 0 or 1")
                }
            }
            bitmap = bitmap >> 1;
        }
        results.push(operator_fns);
    }
    results
}

fn can_be_calculated(test_value: &u64, inputs: &Vec<u64>) -> bool {
    // Generate the set of operations
    let combos = arithematic_combos(inputs.len() - 1);

    // get a combo, place the values of the inputs in position, see if it calculates to the test_value.
    combos
        .iter()
        .any(|combo| *test_value == evaluate(combo, inputs))
}

fn evaluate(combo: &[fn(u64, u64) -> u64], inputs: &[u64]) -> u64 {
    // iteration isn't great here, it should probably be a nested grouping of functions
    // let operations = inputs.iter().zip(inputs.iter().skip(1)).zip(combo);
    let mut prev = combo[0](inputs[0], inputs[1]);
    for i in 1..combo.len() {
        prev = combo[i](prev, inputs[i + 1]);
    }
    prev
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let splits: Vec<&str> = line.split(":").map(|x| x.trim()).collect();
    let answer = splits[0].parse::<u64>().expect("First item is the result");
    let inputs = splits[1]
        .split(" ")
        .map(|x| x.trim())
        .map(|x| x.parse::<u64>().expect("should be a parsable number"))
        .collect();
    (answer, inputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<String> {
        let lines: Vec<String> = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            .split('\n')
            .map(|x| x.to_string())
            .collect();
        lines
    }

    #[test]
    fn test_day07_part1_handler() {
        let lines = sample_data();
        let calculated = day07_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(3749, calculated);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!((123u64, vec![4u64, 5u64, 6u64]), parse_line("123: 4 5 6"));
    }

    #[test]
    fn test_arithmatic_combos() {
        let operators_needed = 3usize;
        let expected_len = 2usize.pow(operators_needed.try_into().unwrap());
        let combos = arithematic_combos(operators_needed);
        assert_eq!(3usize, combos[0].len());
        assert_eq!(expected_len, combos.len());
    }

    #[test]
    fn test_evaluate() {
        assert_eq!(2, evaluate(&[add], &vec![1, 1]));
        assert_eq!(4, evaluate(&[multiply], &vec![2, 2]));
        assert_eq!(12, evaluate(&[add, multiply], &vec![3, 3, 2]));
        assert_eq!(18, evaluate(&[multiply, add, multiply], &vec![2, 2, 2, 3]));
    }
}
