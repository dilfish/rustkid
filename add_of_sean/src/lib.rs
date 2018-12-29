#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(adder_do(2, 2), 4);
    }
}


/// Adds one to the other number.
///
/// # Examples
///
/// ```
/// assert_eq!(6, adder_of_sean::adder_do(3,3));
/// ```
pub fn adder_do(i: u32, j:u32) -> u32 {
	i + j
}
