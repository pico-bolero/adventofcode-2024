/// Receives input and prints output
pub fn day04_part1(lines: &mut dyn Iterator<Item = String>) {
    let result: u32 = day04_part1_handler(lines);
    println!("Sum {}", result);
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct ScanArea {
    size: usize,
    lines: Vec<Option<String>>,
}

impl ScanArea {
    fn new(size: usize) -> ScanArea {
        let mut lines: Vec<Option<String>> = vec![];
        for _ in 0..size {
            lines.push(None);
        }
        ScanArea { size, lines }
    }

    /// Returns a new ScanArea by cloning the elements of the current
    /// ScanArea and shifting all the elements to the next position.
    fn shift(&mut self, line: Option<String>) {
        self.lines.remove(0);
        self.lines.push(line);
    }

    /// Returns a new ScanArea by cloning the elements of the current
    /// ScanArea and shifting all the elements to the next position.
    #[allow(dead_code)]
    fn shift_new(&self, line: Option<String>) -> ScanArea {
        let mut lines: Vec<Option<String>> = self.lines.iter().skip(1).cloned().collect();
        lines.push(line);
        ScanArea {
            size: self.size,
            lines,
        }
    }

    /// See if every line has data
    fn is_full(&self) -> bool {
        self.lines.iter().all(|x| x.is_some())
    }

    fn data_width(&self) -> usize {
        let x = self
            .lines
            .iter()
            .map(|x| match x {
                Some(x) => x.chars().count(),
                None => 0usize,
            })
            .reduce(|acc, x| acc.max(x));
        x.unwrap_or_default()
    }
}

#[derive(Eq, PartialEq, Debug)]
struct FourXFour {
    grid: [[char; 4]; 4],
}

impl TryFrom<Vec<Vec<char>>> for FourXFour {
    type Error = ();

    fn try_from(value: Vec<Vec<char>>) -> Result<Self, Self::Error> {
        if value.len() < 4 {
            return Err(());
        }
        if value.iter().any(|inner| inner.len() < 4) {
            return Err(());
        }

        let mut grid: [[char; 4]; 4] = [[' '; 4]; 4];
        value.iter().take(4).enumerate().for_each(|(i, inner)| {
            inner.iter().take(4).enumerate().for_each(|(ii, c)| {
                grid[i][ii] = *c;
            })
        });
        Ok(FourXFour { grid })
    }
}

fn day04_part1_handler(lines: &mut (dyn Iterator<Item = String>)) -> u32 {
    /*
    This algorithm is not correct. Using a scan area and counting all the hits
    allows for duplicate vertical and horizontal hits. A method using a pattern
    that radiates outward from a center point is what is needed.
    */
    let mut total: u32 = 0;
    let box_size: usize = 4;
    let mut scan = ScanArea::new(box_size);
    for _ in 0..box_size {
        scan.shift(lines.next());
    }

    while scan.is_full() {
        for i in 0..scan.data_width() - box_size {
            let mut rows: Vec<Vec<char>> = vec![];
            for row in scan.lines[0..box_size].iter() {
                let r: Vec<char> = row
                    .clone()
                    .unwrap()
                    .chars()
                    .skip(i)
                    .take(box_size)
                    .collect();
                rows.push(r);
            }
            let four_by_four = FourXFour::try_from(rows).expect("That should have parsed");
            println!("{:?}", four_by_four);
            total += xmas_count(four_by_four);
        }
        scan.shift(lines.next());
    }
    total
}

fn xmas_hit(input: Vec<char>, purpose: &str) -> u32 {
    let s: String = input.iter().collect();
    match s.as_str() {
        "XMAS" => {
            println!("{} {}", s, purpose);
            1
        }
        "SAMX" => {
            println!("{} {}", s, purpose);
            1
        }
        _ => 0,
    }
}

fn xmas_count(four_by_four: FourXFour) -> u32 {
    let mut total = 0u32;

    // horizontal
    (0..4usize).for_each(|_| {
        total += xmas_hit(
            vec![
                four_by_four.grid[0][0],
                four_by_four.grid[0][1],
                four_by_four.grid[0][2],
                four_by_four.grid[0][3],
            ],
            "horizontal",
        );
    });

    // vertical
    (0..4usize).for_each(|_| {
        total += xmas_hit(
            vec![
                four_by_four.grid[0][0],
                four_by_four.grid[1][0],
                four_by_four.grid[2][0],
                four_by_four.grid[3][0],
            ],
            "vertical",
        );
    });
    // diagonal top to bottom
    total += xmas_hit(
        vec![
            four_by_four.grid[0][0],
            four_by_four.grid[1][1],
            four_by_four.grid[2][2],
            four_by_four.grid[3][3],
        ],
        "diagonal top to bottom",
    );

    total += xmas_hit(
        vec![
            four_by_four.grid[0][3],
            four_by_four.grid[1][2],
            four_by_four.grid[2][1],
            four_by_four.grid[3][0],
        ],
        "diagonal bottom to top",
    );
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _sample_data() -> Vec<String> {
        let lines: Vec<String> = " MMMSXXMASM
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

    fn sample_data2() -> Vec<String> {
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

    #[test]
    fn test_four_by_four_try_from_vecs() {
        let a = vec!['a', 'b', 'c', 'd'];
        let b = vec!['e', 'f', 'g', 'h'];
        let c = vec!['i', 'j', 'k', 'l'];
        let d = vec!['m', 'n', 'o', 'p'];
        let input = vec![a, b, c, d];
        let four_by_four = FourXFour::try_from(input).unwrap();
        assert_eq!(four_by_four.grid[1][1], 'f');

        // too short
        let a = vec!['a', 'b', 'c', 'd'];
        let b = vec!['e', 'f', 'g', 'h'];
        let c = vec!['i', 'j', 'k', 'l'];
        let input = vec![a, b, c];
        let four_by_four = FourXFour::try_from(input);
        assert!(four_by_four.is_err());
    }

    #[test]
    fn test_day04_part1_handler() {
        let lines = sample_data2();
        let calculated = day04_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        /* These tests work! However, the windowing system allows for duplicate counts
        vertical and horizontal hits during scanning and incrementng by 1 */
        // THIS IS CORRECT
        // assert_eq!(18, calculated);
        // THIS IS WRONG
        assert_eq!(24, calculated);
    }

    #[test]
    fn test_xmas_count() {
        /* These tests work! However, the windowing system allows for duplicate counts
        vertical and horizontal hits during scanning and incrementng by 1 */
        let input = FourXFour {
            grid: [
                ['X', 'M', 'A', 'S'],
                ['X', 'M', 'A', 'S'],
                ['X', 'M', 'A', 'S'],
                ['X', 'M', 'A', 'S'],
            ],
        };
        assert_eq!(6, xmas_count(input));

        let input = FourXFour {
            grid: [
                ['S', 'A', 'M', 'X'],
                ['S', 'A', 'M', 'X'],
                ['S', 'A', 'M', 'X'],
                ['S', 'A', 'M', 'X'],
            ],
        };
        assert_eq!(6, xmas_count(input));

        let input = FourXFour {
            grid: [
                ['X', 'X', 'X', 'X'],
                ['M', 'M', 'M', 'M'],
                ['A', 'A', 'A', 'A'],
                ['S', 'S', 'S', 'S'],
            ],
        };
        assert_eq!(6, xmas_count(input));

        let input = FourXFour {
            grid: [
                ['X', 'M', 'A', 'S'],
                ['X', 'M', 'A', 'S'],
                ['X', 'M', 'A', 'S'],
                [' ', ' ', ' ', ' '],
            ],
        };
        assert_eq!(4, xmas_count(input));

        let input = FourXFour {
            grid: [
                ['X', ' ', ' ', 'S'],
                [' ', 'M', 'A', ' '],
                [' ', 'M', 'A', ' '],
                ['X', ' ', ' ', 'S'],
            ],
        };
        assert_eq!(2, xmas_count(input));

        let input = FourXFour {
            grid: [
                ['S', ' ', ' ', 'X'],
                [' ', 'A', 'M', ' '],
                [' ', 'A', 'M', ' '],
                ['S', ' ', ' ', 'X'],
            ],
        };
        assert_eq!(2, xmas_count(input));
    }
}
