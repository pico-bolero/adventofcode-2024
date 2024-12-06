/// Splits and parses a string into the types
pub fn parse_delimited_str<T: std::str::FromStr>(input: &str, delimiter: &str) -> Vec<T> {
    input
        .split(delimiter)
        .flat_map(|x| x.parse::<T>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
