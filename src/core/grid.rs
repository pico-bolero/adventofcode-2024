//! Grid is a utility for ..uh... handling grid data?
//!  The use case is for a 'full' grid and not a sparse grid.
//!  Goals:
//!  * add some features for parsing into a grid, walking
//!  * the grid and checking neighbors.
//!  * All that stuff that AoC wants you to do.
//!  * Using x, y coordinates or rows & cols.

use std::cmp::min;

#[derive(Eq, PartialEq, Debug)]
pub struct Point<T>
where
    T: Copy,
{
    pub x: T,
    pub y: T,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GridItem<T> {
    item: T,
}
/// Raised when attempting an operation that is not inside the grid boundary
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IndexOutofBoundsError;

/// Fully populate the grid with capacity
pub struct Array2D<T> {
    grid: Vec<Vec<GridItem<T>>>,
    boundary: Point<usize>,
}

impl<T> Array2D<T>
where
    T: Clone,
{
    pub fn new(rows: usize, columns: usize, default: GridItem<T>) -> Array2D<T> {
        Array2D {
            grid: vec![vec![default; columns]; rows],
            boundary: Point {
                x: columns,
                y: rows,
            },
        }
    }

    pub fn get_point(&self, x: usize, y: usize) -> Option<&GridItem<T>> {
        if x >= self.boundary.x || y >= self.boundary.y {
            None
        } else {
            Some(&self.grid[y][x])
        }
    }

    pub fn set_point(
        &mut self,
        x: usize,
        y: usize,
        item: GridItem<T>,
    ) -> Result<&GridItem<T>, IndexOutofBoundsError> {
        if x >= self.boundary.x || y >= self.boundary.y {
            Err(IndexOutofBoundsError)
        } else {
            self.grid[y][x] = item;
            Ok(&self.grid[y][x])
        }
    }

    pub fn get_position(&self, row: usize, col: usize) -> Option<&GridItem<T>> {
        if col >= self.boundary.x || row >= self.boundary.y {
            None
        } else {
            Some(&self.grid[row][col])
        }
    }

    pub fn set_position(
        &mut self,
        row: usize,
        col: usize,
        item: GridItem<T>,
    ) -> Result<&GridItem<T>, IndexOutofBoundsError> {
        if col >= self.boundary.x || row >= self.boundary.y {
            Err(IndexOutofBoundsError)
        } else {
            self.grid[row][col] = item;
            Ok(&self.grid[row][col])
        }
    }

    pub fn get_row(&self, index: usize) -> Option<Vec<&GridItem<T>>> {
        if index >= self.boundary.y {
            None
        } else {
            Some(self.grid[index].iter().collect())
        }
    }

    pub fn get_col(&self, index: usize) -> Option<Vec<&GridItem<T>>> {
        if index >= self.boundary.x {
            None
        } else {
            Some(self.grid.iter().flat_map(|row| row.get(index)).collect())
        }
    }

    pub fn in_bounds(&self, point: Point<usize>) -> bool {
        point.x < self.boundary.x && point.y < self.boundary.y
    }

    pub fn window_for_point_radius(
        &self,
        point: Point<usize>,
        radius: usize,
    ) -> Vec<Vec<Option<&GridItem<T>>>> {
        let size = radius * 2 + 1;
        let mut window: Vec<Vec<Option<&GridItem<T>>>> = vec![vec![None; size]; size];
        let anchor = Point {
            x: point.x.wrapping_sub(radius),
            y: point.x.wrapping_sub(radius),
        };

        for row in 0..size {
            for col in 0..size {
                let y = anchor.y.wrapping_add(row);
                let x = anchor.x.wrapping_add(col);
                window[row][col] = self.get_point(x, y);
            }
        }
        window
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
    /// Exercises the creation and get and set for a 2d array
    fn test_array2d_new_get_and_set() {
        let mut array_2d = Array2D::new(3, 3, GridItem { item: '_' });
        assert_eq!(None, array_2d.get_row(3));
        assert_eq!(None, array_2d.get_col(3));

        assert_eq!(Some(vec![&GridItem { item: '_' }; 3]), array_2d.get_row(1));
        assert_eq!(Some(vec![&GridItem { item: '_' }; 3]), array_2d.get_col(1));

        assert_eq!(
            Err(IndexOutofBoundsError),
            array_2d.set_point(3, 3, GridItem { item: 'X' })
        );

        let _ = array_2d.set_point(1, 2, GridItem { item: 'A' });
        assert_eq!(Some(&GridItem { item: 'A' }), array_2d.get_point(1, 2));

        let _ = array_2d.set_position(2, 2, GridItem { item: 'B' });
        assert_eq!(Some(&GridItem { item: 'B' }), array_2d.get_position(2, 2));

        assert_eq!(
            Some(vec![
                &GridItem { item: '_' },
                &GridItem { item: 'A' },
                &GridItem { item: 'B' }
            ]),
            array_2d.get_row(2)
        );
    }

    #[test]
    /// Exercises getting a window around a point in a 2d array
    fn test_array2d_relative_area_for_point_and_radius() {
        let array_2d = Array2D::new(3, 3, GridItem { item: '_' });
        let calculated = array_2d.window_for_point_radius(Point { x: 0, y: 0 }, 1);

        // There are points that are not in the 2d array and those should be None
        let size = 3;
        let mut expected: Vec<Vec<Option<&GridItem<char>>>> = vec![vec![None; size]; size];
        expected[1][1] = Some(&GridItem { item: '_' });
        expected[1][2] = Some(&GridItem { item: '_' });
        expected[2][1] = Some(&GridItem { item: '_' });
        expected[2][2] = Some(&GridItem { item: '_' });
        assert_eq!(expected, calculated);

        let array_2d = Array2D::new(3, 3, GridItem { item: '_' });
        let calculated = array_2d.window_for_point_radius(Point { x: 3, y: 3 }, 1);
        let size = 3;
        let mut expected: Vec<Vec<Option<&GridItem<char>>>> = vec![vec![None; size]; size];
        expected[0][0] = Some(&GridItem { item: '_' });
        assert_eq!(expected, calculated);
    }
}
