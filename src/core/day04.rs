use super::scan_radius::RadiusScan;

/// Receives input and prints output
pub fn day04_part1(lines: &mut dyn Iterator<Item = String>) {
    let result: u32 = day04_part1_handler(lines);
    println!("Sum {}", result);
}

fn day04_part1_handler(lines: &mut (dyn Iterator<Item = String>)) -> u32 {
    let mut total: u32 = 0;
    let radius: usize = 3;
    let mut prelude: Vec<Option<String>> = vec![];
    for _ in 0..radius {
        prelude.push(lines.next());
    }
    let mut scanner = RadiusScan::new(radius, prelude);

    while let Some(line) = lines.next() {
        scanner.shift(Some(line));
        while let Some(grid) = scanner.next() {
            total += xmas_count(grid);
        }
    }
    // epilogue, scanner hasn't finished processing all its rows yet.
    for _ in 0..radius {
        scanner.shift(None);
        while let Some(grid) = scanner.next() {
            total += xmas_count(grid);
        }
    }
    total
}

/// Receives input and prints output
pub fn day04_part2(lines: &mut dyn Iterator<Item = String>) {
    let result: u32 = day04_part2_handler(lines);
    println!("Sum {}", result);
}

fn day04_part2_handler(lines: &mut (dyn Iterator<Item = String>)) -> u32 {
    let mut total: u32 = 0;
    let radius: usize = 1;
    let mut prelude: Vec<Option<String>> = vec![];
    for _ in 0..radius {
        prelude.push(lines.next());
    }
    let mut scanner = RadiusScan::new(radius, prelude);

    while let Some(line) = lines.next() {
        scanner.shift(Some(line));
        while let Some(grid) = scanner.next() {
            total += cross_mas_count(grid);
        }
    }
    // epilogue, scanner hasn't finished processing all its rows yet.
    for _ in 0..radius {
        scanner.shift(None);
        while let Some(grid) = scanner.next() {
            total += cross_mas_count(grid);
        }
    }
    total
}

fn xmas_count(input: Vec<Vec<Option<char>>>) -> u32 {
    let mut g = [[' '; 7]; 7];
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if let Some(c) = input[row][col] {
                g[row][col] = c;
            }
        }
    }
    let mut hits = 0u32;

    // Check for XMAS
    //   0123456
    // 0 S..S..S
    // 1 .A.A.A.
    // 2 ..MMM..
    // 3 SAMXMAS
    // 4 ..MMM..
    // 5 .A.A.A.
    // 6 S..S..S

    //horizontal
    hits += is_xmas((g[3][3], g[3][4], g[3][5], g[3][6])); // forward
    hits += is_xmas((g[3][3], g[3][2], g[3][1], g[3][0])); // backward

    // vertical
    hits += is_xmas((g[3][3], g[4][3], g[5][3], g[6][3])); // going down
    hits += is_xmas((g[3][3], g[2][3], g[1][3], g[0][3])); // going up

    // diagonal
    hits += is_xmas((g[3][3], g[2][4], g[1][5], g[0][6])); // up right
    hits += is_xmas((g[3][3], g[4][4], g[5][5], g[6][6])); // down right

    hits += is_xmas((g[3][3], g[2][2], g[1][1], g[0][0])); // up left
    hits += is_xmas((g[3][3], g[4][2], g[5][1], g[6][0])); // down left

    hits
}

fn is_xmas(input: (char, char, char, char)) -> u32 {
    if ('X', 'M', 'A', 'S') == input {
        1
    } else {
        0
    }
}

fn is_cross_mas(a: (char, char, char), b: (char, char, char)) -> u32 {
    if ('M', 'A', 'S') == a && ('M', 'A', 'S') == b {
        1
    } else {
        0
    }
}

fn cross_mas_count(input: Vec<Vec<Option<char>>>) -> u32 {
    let mut g = [[' '; 3]; 3];
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if let Some(c) = input[row][col] {
                g[row][col] = c;
            }
        }
    }
    let mut hits = 0u32;
    // M.M
    // .A.
    // S.S
    hits += is_cross_mas((g[0][0], g[1][1], g[2][2]), (g[0][2], g[1][1], g[2][0]));

    // S.S
    // .A.
    // M.M
    hits += is_cross_mas((g[2][0], g[1][1], g[0][2]), (g[2][2], g[1][1], g[0][0]));

    // M.S
    // .A.
    // M.S
    hits += is_cross_mas((g[0][0], g[1][1], g[2][2]), (g[2][0], g[1][1], g[0][2]));

    // S.M
    // .A.
    // S.M
    hits += is_cross_mas((g[0][2], g[1][1], g[2][0]), (g[2][2], g[1][1], g[0][0]));

    // .M.
    // MAS
    // .S.
    // hits += is_cross_mas((g[0][1], g[1][1], g[2][1]), (g[1][0], g[1][1], g[1][2]));

    // .M.
    // SAM
    // .S.
    // hits += is_cross_mas((g[0][1], g[1][1], g[2][1]), (g[1][2], g[1][1], g[1][0]));

    // .S.
    // SAM
    // .M.
    // hits += is_cross_mas((g[1][2], g[1][1], g[1][0]), (g[2][1], g[1][1], g[0][1]));

    // .S.
    // MAS
    // .M.
    // hits += is_cross_mas((g[1][0], g[1][1], g[1][2]), (g[2][1], g[1][1], g[0][1]));

    hits
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<String> {
        let lines: Vec<String> = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .split('\n')
            .map(|x| x.to_string())
            .collect();
        lines
    }

    fn _sample_data2() -> Vec<String> {
        let lines: Vec<String> = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX"
            .split('\n')
            .map(|x| x.to_string())
            .collect();
        lines
    }

    fn _sample_data3() -> Vec<String> {
        let lines: Vec<String> = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."
            .split('\n')
            .map(|x| x.to_string())
            .collect();
        lines
    }

    #[test]
    fn test_day04_part1_handler() {
        let lines = sample_data();
        // let lines = _sample_data2();
        let calculated = day04_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(18, calculated);
    }

    #[test]
    fn test_day04_part2_handler() {
        let lines = sample_data();
        // let lines = _sample_data3();
        let calculated = day04_part2_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(9, calculated);
    }

    #[test]
    fn test_cross_mas() {
        let input = vec![
            vec![Some('M'), None, Some('M')],
            vec![None, Some('A'), None],
            vec![Some('S'), None, Some('S')],
        ];
        assert_eq!(1, cross_mas_count(input));

        let input = vec![
            vec![Some('S'), None, Some('S')],
            vec![None, Some('A'), None],
            vec![Some('M'), None, Some('M')],
        ];
        assert_eq!(1, cross_mas_count(input));

        let input = vec![
            vec![Some('M'), None, Some('S')],
            vec![None, Some('A'), None],
            vec![Some('M'), None, Some('S')],
        ];
        assert_eq!(1, cross_mas_count(input));

        let input = vec![
            vec![Some('S'), None, Some('M')],
            vec![None, Some('A'), None],
            vec![Some('S'), None, Some('M')],
        ];
        assert_eq!(1, cross_mas_count(input));
        /*
                let input = vec![
                    vec![None, Some('M'), None],
                    vec![Some('M'), Some('A'), Some('S')],
                    vec![None, Some('S'), None],
                ];
                assert_eq!(1, cross_mas_count(input));

                let input = vec![
                    vec![None, Some('M'), None],
                    vec![Some('S'), Some('A'), Some('M')],
                    vec![None, Some('S'), None],
                ];
                assert_eq!(1, cross_mas_count(input));

                let input = vec![
                    vec![None, Some('S'), None],
                    vec![Some('S'), Some('A'), Some('M')],
                    vec![None, Some('M'), None],
                ];
                assert_eq!(1, cross_mas_count(input));

                let input = vec![
                    vec![None, Some('S'), None],
                    vec![Some('M'), Some('A'), Some('S')],
                    vec![None, Some('M'), None],
                ];
                assert_eq!(1, cross_mas_count(input));
        */
    }

    #[test]
    fn test_xmas_count() {
        let input = vec![
            vec![Some('S'), None, None, Some('S'), None, None, Some('S')],
            vec![None, Some('A'), None, Some('A'), None, Some('A'), None],
            vec![None, None, Some('M'), Some('M'), Some('M'), None, None],
            vec![
                Some('S'),
                Some('A'),
                Some('M'),
                Some('X'),
                Some('M'),
                Some('A'),
                Some('S'),
            ],
            vec![None, None, Some('M'), Some('M'), Some('M'), None, None],
            vec![None, Some('A'), None, Some('A'), None, Some('A'), None],
            vec![Some('S'), None, None, Some('S'), None, None, Some('S')],
        ];
        assert_eq!(8, xmas_count(input));
    }
}
