pub struct Person {
    name: String,
    _age: u8
}

impl Person {
    pub fn new(name: String, _age: u8) -> Person {
        Person {
            name,
            _age
        }
    }

    // No need to explicit lifetimes due to Rust's lifetimes rules
    pub fn return_name(&self) -> &str {
        &self.name
    }
}
