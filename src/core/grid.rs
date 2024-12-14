/// Grid is a utility for ..uh... handling grid data?
/// Why not an array of arrays? This module is meant to
///  add some features for parsing into a grid, walking
///  the grid and checking neighbors. All that stuff that
///  AoC wants you to do. The use case is for a 'full' grid and not a sparse grid.

/// Using x, y coordinates
pub struct Point {
    _x: usize,
    _y: usize,
}

/*
/// Using row and column indices
pub struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, PartialEq)]
pub struct GridItem<T> {
    item: T,
}

pub struct Grid<T> {
    grid: Vec<Vec<GridItem<T>>>,
}
impl<T> Grid<T> {
    pub fn item_at_point(&self, point: Point) {}

    pub fn item_at_position(&self, position: Position) {}

    pub fn row(&self, index: usize) -> Option<Vec<&GridItem<T>>> {
        match self.grid.get(index) {
            None => None,
            Some(row) => Some(row.iter().collect::<Vec<&GridItem<T>>>()),
        }
    }
    pub fn col(&self, index: usize) -> Option<Vec<&GridItem<T>>> {
        let len = self.grid.g
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<Vec<GridItem<String>>> {
        vec![
            vec![GridItem {
                item: "ABC".to_string(),
            }],
            vec![GridItem {
                item: "DEF".to_string(),
            }],
            vec![GridItem {
                item: "GHI".to_string(),
            }],
        ]
    }

    #[test]
    fn test_row() {
        let grid: Grid<String> = Grid {
            grid: sample_data(),
        };
        // Out of bounds returns None
        assert_eq!(None, grid.row(5000));
        let item = GridItem {
            item: "ABC".to_string(),
        };
        // In bounds returns Some
        let expected = Some(vec![&item]);
        assert_eq!(expected, grid.row(0));
    }
    #[test]
    fn test_col() {
        let grid: Grid<String> = Grid {
            grid: sample_data(),
        };
        // Out of bounds returns None
        assert_eq!(None, grid.col(3));
        let item = GridItem {
            item: "ABC".to_string(),
        };
        // In bounds returns Some
        let expected = Some(vec![&item]);
        assert_eq!(expected, grid.row(0));
    }
}
*/
