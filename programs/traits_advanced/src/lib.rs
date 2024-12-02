use std::ops::{Add, Sub};

pub struct Point {
    pub x: f32,
    pub y: f32
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}


pub struct Millimeters(u32);
impl Millimeters {
    pub fn new(x: u32) -> Millimeters {
        Millimeters(x)
    }

    pub fn value(&self) -> &u32 {
        &self.0
    }
}

pub struct Meters(u32);
impl Meters {
    pub fn new(x: u32) -> Meters {
        Meters(x)
    }
}


impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    
    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}