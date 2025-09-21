use std::fmt;

const PROTEIN_STRENGTH: f64 = 4.0;
const FAT_STRENGTH: f64 = 9.0;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub strength: f64,
    pub score: i32,
    pub money: i32,
    pub weapons: Vec<String>,
}

pub struct Fruit {
    pub weight_in_kg: f64,
}

pub struct Meat {
    pub weight_in_kg: f64,
    pub fat_content: f64,
}

impl Player {
    pub fn eat<T: Food>(&mut self, food: T) {
        self.strength += food.gives();
    }
}

pub trait Food {
    fn gives(&self) -> f64;
}

impl Food for Fruit {
    fn gives(&self) -> f64 {
        self.weight_in_kg * PROTEIN_STRENGTH
    }
}

impl Food for Meat {
    fn gives(&self) -> f64 {
        let protein_kg = self.weight_in_kg * (1.0 - self.fat_content);
        let fat_kg = self.weight_in_kg * self.fat_content;
        protein_kg * PROTEIN_STRENGTH + fat_kg * FAT_STRENGTH
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(
            f,
            "Strength: {}, Score: {}, Money: {}",
            self.strength, self.score, self.money
        )?;
        write!(f, "Weapons: {:?}", self.weapons)
    }
}
