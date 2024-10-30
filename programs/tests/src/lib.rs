pub fn add(first: usize, second: usize) -> usize {
    first + second
}

pub fn greeting(name: &str) -> String {
    String::from("Helloooo")
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


    #[test]
    fn name_containts_carol() {
        let name = greeting("Carol");
        // Extra parameters given to assert! macro are printed in case of a failure
        assert!(name.contains("Carol"), "Greeting didn't contain name, value was {name}");
    }
}