use super::{Display, Formatter, Result};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::Resources;

const WARRIOR_START_NUMBER: u32 = 0;
const ARCHER_START_NUMBER: u32 = 0;

const WARRIOR_STRENGTH: f32 = 1.2;
const ARCHER_STRENGTH: f32 = 1.9;

pub const WARRIOR_RESOURCES: Resources = Resources { wood: 10, gold: 5 };
pub const ARCHER_RESOURCES: Resources = Resources { wood: 0, gold: 10 };

/// Represents unit types
#[derive(Debug, EnumIter)]
pub enum Unit {
    Warrior,
    Archer,
}

/// Represents player's units
#[derive(Debug)]
pub struct Units {
    warriors: u32,
    archers: u32,
}

impl Display for Units {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} warriors, {} archers", self.warriors, self.archers)
    }
}

impl Units {
    /// Creates new units
    ///
    /// ```
    /// let units = Unit::new();
    /// assert units.warriors == WARRIOR_START_NUMBER;
    /// assert units.archers == ARCHER_START_NUMBER;
    /// ```
    pub fn new() -> Self {
        Self {
            warriors: WARRIOR_START_NUMBER,
            archers: ARCHER_START_NUMBER,
        }
    }

    /// Returns the number of units
    pub fn get_number(&self) -> u32 {
        self.warriors + self.archers
    }

    /// Gets number of units of specified unit type
    pub fn get_units(&mut self, unit: &Unit) -> u32 {
        match unit {
            Unit::Warrior => self.warriors,
            Unit::Archer => self.archers,
        }
    }

    /// Adds number of units to specified unit type
    pub fn add_units(&mut self, unit: &Unit, number: u32) {
        match unit {
            Unit::Warrior => self.warriors += number,
            Unit::Archer => self.archers += number,
        }
    }

    /// Removes number of units from specified unit type
    pub fn remove_units(&mut self, unit: &Unit, number: u32) {
        match unit {
            Unit::Warrior => self.warriors -= number,
            Unit::Archer => self.archers -= number,
        }
    }

    /// Returns strength of units
    pub fn get_strength(&self) -> f32 {
        self.warriors as f32 * WARRIOR_STRENGTH + self.archers as f32 * ARCHER_STRENGTH
    }
}

pub fn print_info() {
    Unit::iter().for_each(|unit| {
        let resources = match unit {
            Unit::Warrior => WARRIOR_RESOURCES,
            Unit::Archer => ARCHER_RESOURCES,
        };
        println!("{:?}: {}.", &unit, resources);
    });
}
