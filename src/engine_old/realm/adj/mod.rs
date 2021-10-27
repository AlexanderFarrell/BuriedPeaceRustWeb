use crate::engine::realm::{Describe, Is};

#[derive(Debug)]
pub struct Aged {years: u32, months: u32}

impl Aged {
    pub fn new(years: u32, months: u32) -> Self {
        Aged { years, months }
    }
}

impl Describe for Aged {
    fn describe(&self) -> String {
        if self.years > 2 {
            format!("{} years old", self.years)
        } else {
            format!("{} months old", self.months + self.years * 12)
        }
    }
}

impl Is for Aged {
    fn noun_word(&self) -> &'static str { "Age" }
    fn adj_word(&self) -> &'static str { "Aged" }
}
