pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 5);
        assert_eq!(result, 7);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn another_one() {
        let result = add(2, 3);
        assert_eq!(result, 7);
    }
}