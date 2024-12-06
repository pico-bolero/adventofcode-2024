use std::collections::{HashMap, HashSet};

/// Receives input and prints output
pub fn day05_part1(lines: &mut dyn Iterator<Item = String>) {
    let result: u32 = day05_part1_handler(lines);
    println!("Sum {}", result);
}

/// Receives input and prints output
pub fn day05_part2(lines: &mut dyn Iterator<Item = String>) {
    let result: u32 = day05_part2_handler(lines);
    println!("Sum {}", result);
}

fn day05_part1_handler(lines: &mut (dyn Iterator<Item = String>)) -> u32 {
    /*
    The data intially comes into two chunks. Please on a double return.
        1. process chunk 1 into rules
        2. process chunk 2 into the data to validate
    */
    let all_lines: Vec<String> = lines.collect();
    let rules = extract_rules(&mut all_lines.iter().map(|x| x.to_string()));
    let orders = extract_orders(&mut all_lines.iter().map(|x| x.to_string()));
    let result: u32 = orders
        .iter()
        .filter(|order| is_valid_order(order, &rules))
        .filter_map(|x| extract_middle::<u32>(x))
        .sum();
    result
}

fn day05_part2_handler(lines: &mut (dyn Iterator<Item = String>)) -> u32 {
    /*
    The data intially comes into two chunks. Please on a double return.
        1. process chunk 1 into rules
        2. process chunk 2 into the data to validate
    */
    let all_lines: Vec<String> = lines.collect();
    let rules = extract_rules(&mut all_lines.iter().map(|x| x.to_string()));
    let orders = extract_orders(&mut all_lines.iter().map(|x| x.to_string()));
    let result: u32 = orders
        .iter()
        .filter(|order| !is_valid_order(order, &rules))
        .map(|x| sort_for_rules(x, &rules))
        .filter_map(|x| extract_middle::<u32>(&x))
        .sum();
    result
}

fn sort_for_rules(pages: &[u32], all_rules: &HashMap<u32, PageRules>) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    pages.iter().for_each(|x| {
        for i in 0..pages.len() {
            result.insert(i, *x);
            if is_valid_order(&result, all_rules) {
                break;
            } else {
                // undo the last insert and try again
                // I am embrassed by my brute force.
                result.remove(i);
            }
        }
    });
    result
}

fn extract_middle<T: Copy>(v: &[T]) -> Option<T> {
    if v.len() % 2 == 0 {
        None // No even numers
    } else {
        let half = (v.len() - 1) / 2;
        Some(v[half])
    }
}

/// The order starts with a page number, then additional page numbers
///   Verify that the rule holds for the page number
fn is_valid_order(pages: &[u32], all_rules: &HashMap<u32, PageRules>) -> bool {
    let mut seen: HashSet<u32> = HashSet::new();
    pages.iter().all(|x| {
        seen.insert(*x);

        // check the rules to see if there are any violators in the map
        if let Some(page_rules) = all_rules.get(x) {
            let rules = &page_rules.rules;
            let passed_rules = rules.iter().all(|rule| match rule {
                Rule::ExistsBefore { page } => !seen.contains(page),
            });
            passed_rules
        } else {
            true
        }
    })
}

fn extract_orders(lines: &mut (dyn Iterator<Item = String>)) -> Vec<Vec<u32>> {
    let _orders: Vec<Vec<u32>> = lines
        .skip_while(|x| !x.is_empty())
        .filter_map(|x| {
            if x.is_empty() {
                None
            } else {
                Some(parse_delimited_str::<u32>(x.as_str(), ","))
            }
        })
        .collect();
    _orders
}

fn extract_rules(lines: &mut (dyn Iterator<Item = String>)) -> HashMap<u32, PageRules> {
    let mut rules: HashMap<u32, PageRules> = HashMap::new();
    lines
        .take_while(|x| x.as_str() != "\n")
        .map(|x| parse_delimited_str::<u32>(x.as_str(), "|"))
        .flat_map(|v| {
            if v.len() == 2 {
                return Ok(v);
            }
            Err(())
        })
        .for_each(|v| {
            let key = v[0];
            let value = v[1];
            match rules.get_mut(&key) {
                // Update rule
                Some(page_rule) => page_rule.rules.push(Rule::ExistsBefore { page: value }),
                // Create rule
                None => {
                    let opt = rules.insert(
                        key,
                        PageRules {
                            rules: vec![Rule::ExistsBefore { page: value }],
                        },
                    );
                    match opt {
                        None => { /* this is expected */ }
                        Some(_) => {
                            panic!("unexpected existing rule")
                        }
                    }
                }
            }
        });
    rules
}
enum Rule {
    ExistsBefore { page: u32 },
}

struct PageRules {
    rules: Vec<Rule>,
}

/// Splits and parses a string into the types
fn parse_delimited_str<T: std::str::FromStr>(input: &str, delimiter: &str) -> Vec<T> {
    input
        .split(delimiter)
        .flat_map(|x| x.parse::<T>())
        .collect()
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
    fn test_day05_part2_handler() {
        let lines = sample_data();
        let calculated = day05_part2_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(123, calculated);
    }

    #[test]
    fn test_extract_middle() {
        assert_eq!(Some(7), extract_middle(&vec![1, 3, 5, 7, 9, 11, 13]))
    }

    #[test]
    fn test_parse_delimited_str() {
        assert_eq!(vec![0, 1, 2], parse_delimited_str("0,1,2", ","));
        assert_eq!(vec![0, 1, 2], parse_delimited_str("0|1|2", "|"));
        assert_eq!(vec![0u64, 1u64, 2u64], parse_delimited_str("0|1|2", "|"));
        assert_eq!(
            vec![0i32, -1i32, -2i32],
            parse_delimited_str("0\t-1\t-2", "\t")
        );
    }
}
