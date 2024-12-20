use regex::Regex;

/// Receives input and prints output
pub fn day03_part1(lines: &mut dyn Iterator<Item = String>) {
    let multiplications: u32 = day03_part1_handler(lines);
    println!("Sum {}", multiplications);
}

pub fn day03_part2(lines: &mut dyn Iterator<Item = String>) {
    let multiplications: u32 = day03_part2_handler(lines);
    println!("Sum {}", multiplications);
}

fn day03_part1_handler(lines: &mut dyn Iterator<Item = String>) -> u32 {
    lines.map(line_value).sum()
}

fn day03_part2_handler(lines: &mut dyn Iterator<Item = String>) -> u32 {
    let joined = lines
        .map(|x| x.trim().to_string())
        .reduce(|a, b| format!("{}{}", a, b));
    line_value2(joined.unwrap())
}

fn line_value(line: String) -> u32 {
    let mut commands = vec![];
    for command in MultiplyCommand::regex()
        .find_iter(&line)
        .map(|x| x.as_str())
    {
        commands.push(MultiplyCommand::parse_mul_command(command));
    }

    commands.iter().flatten().map(|cmd| cmd.value()).sum()
}

fn line_value2(line: String) -> u32 {
    let mut commands: Vec<CommandType> = vec![];

    MultiplyCommand::regex().captures_iter(&line).for_each(|c| {
        let rng = c.get(0).unwrap().range();
        let (_fullstr, [lhs, rhs]) = c.extract();
        let l = lhs.parse::<u32>();
        let r = rhs.parse::<u32>();
        if let (Ok(lhs), Ok(rhs)) = (l, r) {
            let cmd = MultiplyCommand { lhs, rhs };
            commands.push(CommandType::Multiply {
                idx: rng.start,
                cmd,
            });
        }
    });

    do_not_regex().captures_iter(&line).for_each(|c| {
        let rng = c.get(0).unwrap().range();
        commands.push(CommandType::DoNot { idx: rng.start })
    });
    do_regex().captures_iter(&line).for_each(|c| {
        let rng = c.get(0).unwrap().range();
        commands.push(CommandType::Do { idx: rng.start })
    });

    commands.sort_by(|a, b| CommandType::get_idx(a).cmp(CommandType::get_idx(b)));

    let mut enabled: bool = true;
    let mut total: u32 = 0;
    for cmd in commands {
        match cmd {
            CommandType::DoNot { idx: _ } => enabled = false,
            CommandType::Do { idx: _ } => enabled = true,
            CommandType::Multiply { idx: _, cmd } => {
                if enabled {
                    total += cmd.value();
                }
            }
        }
    }
    total
}

enum CommandType {
    DoNot { idx: usize },
    Do { idx: usize },
    Multiply { idx: usize, cmd: MultiplyCommand },
}

impl CommandType {
    fn get_idx(cmd: &CommandType) -> &usize {
        match cmd {
            CommandType::DoNot { idx } => idx,
            CommandType::Do { idx } => idx,
            CommandType::Multiply { idx, cmd: _ } => idx,
        }
    }
}

fn do_not_regex() -> Regex {
    Regex::new(r"don't\(\)").unwrap()
}

fn do_regex() -> Regex {
    Regex::new(r"do\(\)").unwrap()
}

#[derive(Eq, PartialEq, Debug)]
struct MultiplyCommand {
    lhs: u32,
    rhs: u32,
}

impl MultiplyCommand {
    fn value(&self) -> u32 {
        self.lhs * self.rhs
    }

    fn regex() -> Regex {
        Regex::new(r"mul\((\d+),(\d+)\)").unwrap()
    }

    fn parse_mul_command(input: &str) -> Option<MultiplyCommand> {
        let reg = MultiplyCommand::regex();
        let mut itr = reg.captures_iter(input);
        match itr.next() {
            Some(c) => {
                let (_, [lhs, rhs]) = c.extract();
                let l = lhs.parse::<u32>();
                let r = rhs.parse::<u32>();
                if let (Ok(lhs), Ok(rhs)) = (l, r) {
                    Some(MultiplyCommand { lhs, rhs })
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<String> {
        let lines: Vec<String> =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                .split('\n')
                .map(|x| x.to_string())
                .collect();
        lines
    }

    fn sample_data2() -> Vec<String> {
        let lines: Vec<String> =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
                .split('\n')
                .map(|x| x.to_string())
                .collect();
        lines
    }

    #[test]
    fn test_parse_multiply_command() {
        assert_eq!(None, MultiplyCommand::parse_mul_command("mul[a,b]"));
        assert_eq!(
            Some(MultiplyCommand { lhs: 5, rhs: 10 }),
            MultiplyCommand::parse_mul_command("mul(5,10)")
        );
    }

    #[test]
    fn test_day03_part1_handler() {
        let lines = sample_data();
        let calculated = day03_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(161, calculated);
    }

    #[test]
    fn test_day03_part2_handler() {
        let lines = sample_data2();
        let calculated = day03_part2_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(48, calculated);
    }
}
