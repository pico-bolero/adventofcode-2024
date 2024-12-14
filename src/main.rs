pub mod core;
pub mod io;
use crate::core::day01;
use crate::core::day02;
use crate::core::day03;
use crate::core::day04;
use crate::core::day05;
use crate::core::day06;
use crate::core::day07;
use crate::core::day08;
use crate::core::day09;
use crate::core::day10;
use crate::core::day11;
use crate::core::day14;

use crate::io::data_loader;
use std::env;
use std::path::Path;

struct ScenarioConfig {
    file_path: String,
    process_fn: fn(&mut dyn Iterator<Item = String>) -> (),
}

// Examines the command line arguments and passes back
// the input file to load.
fn select_scenario() -> ScenarioConfig {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Expected 1 argument like 'day01_part1'");
    }
    match args[1].as_str() {
        "day01_part1" => ScenarioConfig {
            file_path: "./data/day01.txt".to_string(),
            process_fn: day01::day01_part1,
        },
        "day01_part2" => ScenarioConfig {
            file_path: "./data/day01.txt".to_string(),
            process_fn: day01::day01_part2,
        },
        "day02_part1" => ScenarioConfig {
            file_path: "./data/day02.txt".to_string(),
            process_fn: day02::day02_part1,
        },
        "day02_part2" => ScenarioConfig {
            file_path: "./data/day02.txt".to_string(),
            process_fn: day02::day02_part2,
        },
        "day03_part1" => ScenarioConfig {
            file_path: "./data/day03.txt".to_string(),
            process_fn: day03::day03_part1,
        },
        "day03_part2" => ScenarioConfig {
            file_path: "./data/day03.txt".to_string(),
            process_fn: day03::day03_part2,
        },
        "day04_part1" => ScenarioConfig {
            file_path: "./data/day04.txt".to_string(),
            process_fn: day04::day04_part1,
        },
        "day04_part2" => ScenarioConfig {
            file_path: "./data/day04.txt".to_string(),
            process_fn: day04::day04_part2,
        },
        "day05_part1" => ScenarioConfig {
            file_path: "./data/day05.txt".to_string(),
            process_fn: day05::day05_part1,
        },
        "day05_part2" => ScenarioConfig {
            file_path: "./data/day05.txt".to_string(),
            process_fn: day05::day05_part2,
        },
        "day06_part1" => ScenarioConfig {
            file_path: "./data/day06.txt".to_string(),
            process_fn: day06::day06_part1,
        },
        "day06_part2" => ScenarioConfig {
            file_path: "./data/day06.txt".to_string(),
            process_fn: day06::day06_part2,
        },
        "day07_part1" => ScenarioConfig {
            file_path: "./data/day07.txt".to_string(),
            process_fn: day07::day07_part1,
        },
        "day07_part2" => ScenarioConfig {
            file_path: "./data/day07.txt".to_string(),
            process_fn: day07::day07_part2,
        },
        "day08_part1" => ScenarioConfig {
            file_path: "./data/day08.txt".to_string(),
            process_fn: day08::day08_part1,
        },
        "day09_part1" => ScenarioConfig {
            file_path: "./data/day09.txt".to_string(),
            process_fn: day09::day09_part1,
        },
        "day09_part2" => ScenarioConfig {
            file_path: "./data/day09.txt".to_string(),
            process_fn: day09::day09_part2,
        },
        "day10_part1" => ScenarioConfig {
            file_path: "./data/day10.txt".to_string(),
            process_fn: day10::day10_part1,
        },
        "day11_part1" => ScenarioConfig {
            file_path: "./data/day11.txt".to_string(),
            process_fn: day11::day11_part1,
        },
        "day11_part2" => ScenarioConfig {
            file_path: "./data/day11.txt".to_string(),
            process_fn: day11::day11_part2,
        },
        "day14_part1" => ScenarioConfig {
            file_path: "./data/day14.txt".to_string(),
            process_fn: day14::day14_part1,
        },

        _ => {
            panic!("Expected argument like 'day01_part1' and not {}", &args[1]);
        }
    }
}

fn main() {
    // Parse the input arguments
    let scenario = select_scenario();

    // Get the lines iterator
    let lines = data_loader::read_lines(Path::new(scenario.file_path.as_str()))
        .unwrap_or_else(|_| panic!("Expected {} to have readable lines.", scenario.file_path));

    // Only process the 'Ok()' items
    let mut itr = lines.map_while(Result::ok);
    (scenario.process_fn)(&mut itr);
}
