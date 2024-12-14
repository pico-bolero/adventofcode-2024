/// Day 14 - Approach
///  I suspect that each robot will eventually repeat its path. The goal is to figure
///  out how many cycles are required before it repeats. Then it doesn't matter
///  how many total cycles are requested as you can get the position with a modulus operation
///  - there may be a way to figure that out just using delta and modifiers.
use crate::core::modulus;

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
    fn test_day14_part1_handler() {
        todo!()
    }

    #[test]
    fn test_boundary_wrap_experiment() {
        // grid boundary 11
        // step is 3
        // initial_position is 2
        // cycle 0  1  2  3   4   5   6   7   8   9   10
        // step: 2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32
        //                11              22,
        // pos   2, 5, 8, 0,  3,  6,  9,  1,  4,  7,  10
        // formula = (initial_position + cycles * step) mod (boundary)
        let initial_position = 2u64;
        let cycles = 9u64;
        let step = 3u64;
        let boundary = 11u64;
        assert_eq!(
            7,
            modulus::modulus(initial_position + cycles * step, boundary)
        );

        // grid boundary 11
        // step is -3
        // initial_position is 2
        // cycle 0   1    2   3    4    5    6    7    8    9
        // step: 2, -1,  -4, -7, -10, -13, -16, -19  -22, -25
        //                             11,            11
        // pos   2, 10,  7,   4,   1,   9,   6,  3,    0,   8,

        let initial_position = 2i64;
        let cycles = 9i64;
        let step = -3i64;
        let boundary = 11i64;
        for cycle in 0..cycles {
            let position = modulus::modulus(initial_position + cycle * step, boundary);
            println!("{}", position);
        }
        assert_eq!(
            8,
            modulus::modulus(initial_position + cycles * step, boundary)
        );
    }
}
