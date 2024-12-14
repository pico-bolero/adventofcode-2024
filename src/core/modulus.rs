use std::ops::Add;
use std::ops::Rem;

pub fn modulus<T>(a: T, b: T) -> T
where
    T: Add<Output = T> + Rem<Output = T> + Copy,
{
    ((a % b) + b) % b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_modulus() {
        assert_eq!(0, modulus(0, 3));
        assert_eq!(1, modulus(1, 3));
        assert_eq!(2, modulus(2, 3));
        assert_eq!(0, modulus(3, 3));
        assert_eq!(1, modulus(4, 3));
        assert_eq!(2, modulus(5, 3));
        assert_eq!(0, modulus(6, 3));
        assert_eq!(1, modulus(7, 3));
        assert_eq!(2, modulus(8, 3));
        assert_eq!(0, modulus(9, 3));
        assert_eq!(1, modulus(10, 3));
        assert_eq!(2, modulus(11, 3));
    }

    #[test]
    fn test_modulus_negative() {
        assert_eq!(0, modulus(0, 3));
        assert_eq!(2, modulus(-1, 3));
        assert_eq!(1, modulus(-2, 3));
        assert_eq!(0, modulus(-3, 3));
        assert_eq!(2, modulus(-4, 3));
        assert_eq!(1, modulus(-5, 3));
    }
}
