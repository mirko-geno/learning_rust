pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}


pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn name(&self) -> &str {
        "Circle"
    }
}



pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}


impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn name(&self) -> &str {
        "Rectangle"
    }
}
