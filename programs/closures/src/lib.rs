#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue
}
/*
The unwrap_or_else method takes two parameters:
'self' (the 'Option' type variable that needs to be unwrapped) and
'f' which is a closure to a function that will execute in case 'Option' is None.

impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where F: FnOnce() -> T {
        match self {
                Some(x) => x,
                None => f()
        }
    }
}
*/

pub struct Inventory {
    pub shirts: Vec<ShirtColor>
}

impl Inventory {
    /* Not using closures
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        if let Some(pref) = user_preference {
            return pref
        } else {
            return self.most_stocked()
        }
    }
    */

    // Using closures
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
            } else {
            ShirtColor::Blue
        }
    }
}