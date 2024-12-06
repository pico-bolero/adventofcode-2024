#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RowScan {
    /// The number of rows to hold in memory
    pub size: usize,
    /// The current data in the Scan
    pub lines: Vec<Option<String>>,
}

impl RowScan {
    /// Creates an empty RowScan
    pub fn new(size: usize) -> RowScan {
        let mut lines: Vec<Option<String>> = vec![];
        for _ in 0..size {
            lines.push(None);
        }
        RowScan { size, lines }
    }

    /// Returns a new ScanArea by cloning the elements of the current
    /// ScanArea and shifting all the elements to the next position.
    pub fn shift(&mut self, line: Option<String>) {
        self.lines.remove(0);
        self.lines.push(line);
    }

    /// Returns a new ScanArea by cloning the elements of the current
    /// ScanArea and shifting all the elements to the next position.
    pub fn shift_new(&self, line: Option<String>) -> RowScan {
        let mut lines: Vec<Option<String>> = self.lines.iter().skip(1).cloned().collect();
        lines.push(line);
        RowScan {
            size: self.size,
            lines,
        }
    }

    /// Every row in the RowScan has data
    pub fn is_full(&self) -> bool {
        self.lines.iter().all(|x| x.is_some())
    }

    /// Returns the length of the longest string in the lines field
    pub fn max_data_width(&self) -> usize {
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
        let mut rows = RowScan::new(3);
        let lines = sample_data();
        let mut iter = lines.iter();

        assert!(!rows.is_full()); // RowScan is empty
                                  //
        rows.shift(iter.next().cloned());
        assert_eq!(vec![None, None, Some("ABC".to_string())], rows.lines);
        rows.shift(iter.next().cloned());
        assert_eq!(
            vec![None, Some("ABC".to_string()), Some("DEF".to_string())],
            rows.lines
        );
        rows.shift(iter.next().cloned());
        assert_eq!(
            vec![
                Some("ABC".to_string()),
                Some("DEF".to_string()),
                Some("GHI".to_string())
            ],
            rows.lines
        );
        assert!(rows.is_full());
        assert_eq!(3, rows.max_data_width());

        rows.shift(iter.next().cloned());
        assert_eq!(
            vec![Some("DEF".to_string()), Some("GHI".to_string()), None],
            rows.lines
        );
    }
}
