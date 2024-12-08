#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Point {
    row: isize,
    col: isize,
}

/// Represents data around a center point
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RadiusScan {
    /// The number of rows to hold in memory
    radius: usize,

    /// Centerpoint of grid
    position: Point,

    /// The current data in the Scan
    lines: Vec<Option<String>>,
}

impl RadiusScan {
    /// Creates an empty RadiusScan
    pub fn new(radius: usize, prelude_rows: Vec<Option<String>>) -> RadiusScan {
        assert!(prelude_rows.len() == radius);
        let mut lines: Vec<Option<String>> = vec![];
        for _ in 0..radius + 1 {
            lines.push(None);
        }
        prelude_rows.iter().cloned().for_each(|line| {
            if line.is_some() {
                lines.push(line)
            }
        });
        // The next row 'shifted' into the row scan will push
        // the initial row into the middle
        RadiusScan {
            radius,
            position: Point { row: -1, col: -1 },
            lines,
        }
    }

    /// Shifts all the elements to the beginning of the next line.
    pub fn shift(&mut self, line: Option<String>) {
        self.lines.remove(0);
        self.lines.push(line);
        self.position = Point {
            row: self.position.row + 1,
            col: 0,
        };
    }

    /// Move the position one column to the right.
    pub fn next(&mut self) -> Option<Vec<Vec<Option<char>>>> {
        if let Some(middle) = &self.lines[self.radius] {
            let row_width: isize = middle.len().try_into().unwrap();
            if self.position.col < row_width {
                let grid = Some(self.grid());
                self.position.col += 1;
                return grid;
            } else {
                return None;
            }
        }
        None
    }

    /// Creates a grid from the current position
    fn grid(&self) -> Vec<Vec<Option<char>>> {
        let size = 2 * self.radius + 1;

        // initialize None into every grid cell
        let mut grid: Vec<Vec<Option<char>>> = vec![vec![None; size]; size];

        self.lines.iter().enumerate().for_each(|(row, line)| {
            let radius: isize = self.radius.try_into().unwrap();
            if let Some(line) = line {
                let chars: Vec<char> = line.chars().collect();
                for col in 0..size {
                    let y_left: isize = self.position.col - radius;
                    let y_offset: isize = col.try_into().unwrap();
                    let col_idx: isize = y_left + y_offset;
                    if col_idx >= 0 && col_idx < line.len().try_into().unwrap() {
                        let i: usize = col_idx.try_into().unwrap();
                        grid[row][col] = Some(chars[i]);
                    }
                }
            }
        });
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<String> {
        let lines: Vec<String> = "ABC
DEF
GHI"
        .split('\n')
        .map(|x| x.to_string())
        .collect();
        lines
    }

    #[test]
    fn test_shift() {
        let lines = sample_data();
        let mut iter = lines.iter();
        let init = vec![iter.next()];
        let init = init.iter().flatten().map(|x| Some((*x).clone())).collect();
        let mut scanner = RadiusScan::new(1, init);

        scanner.shift(iter.next().cloned());
        let grid = scanner.next();
        let expected = vec![
            vec![None, None, None],
            vec![None, Some('A'), Some('B')],
            vec![None, Some('D'), Some('E')],
        ];
        assert_eq!(expected, grid.unwrap());

        let grid = scanner.next();
        let expected = vec![
            vec![None, None, None],
            vec![Some('A'), Some('B'), Some('C')],
            vec![Some('D'), Some('E'), Some('F')],
        ];
        assert_eq!(expected, grid.unwrap());

        let grid = scanner.next();
        let expected = vec![
            vec![None, None, None],
            vec![Some('B'), Some('C'), None],
            vec![Some('E'), Some('F'), None],
        ];
        assert_eq!(expected, grid.unwrap());

        let grid = scanner.next();
        assert!(grid.is_none()); // center point is at the edge

        scanner.shift(iter.next().cloned());
        let grid = scanner.next();
        let expected = vec![
            vec![None, Some('A'), Some('B')],
            vec![None, Some('D'), Some('E')],
            vec![None, Some('G'), Some('H')],
        ];
        assert_eq!(expected, grid.unwrap());

        scanner.shift(iter.next().cloned());
        let grid = scanner.next();
        let expected = vec![
            vec![None, Some('D'), Some('E')],
            vec![None, Some('G'), Some('H')],
            vec![None, None, None],
        ];
        assert_eq!(expected, grid.unwrap());

        scanner.shift(iter.next().cloned());
        let grid = scanner.next();
        assert!(grid.is_none());
    }
}
