#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue
}


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