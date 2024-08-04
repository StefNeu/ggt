use gcd_::gcd;

///Calculates the GCD, regardless of the order in which the numbers are given.
///
/// # Example
///
/// ggt(24, 4)
/// ggt(6, 18)
///
/// the function return a Result
pub fn ggt(first: i128, second: i128) -> Result<i128, &'static str> {
    gcd(first, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result: i128 = ggt(6, 18).unwrap();
        assert_eq!(result, 6);

        result = ggt(24, 4).unwrap();
        assert_eq!(result, 4);

        let result = ggt(-10, 10);
        assert!(result.is_err())
    }
}
