use super::{Display, Formatter, Result};
use super::Resources;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const BASE_START_NUMBER: u32 = 1;
pub const BASE_INC_NUMBER: u32 = 1;
const BASE_CAPACITY: u32 = 200;

pub const BASE_RESOURCES: Resources = Resources {
    wood: 220,
    gold: 100,
};

#[derive(Debug, EnumIter)]
pub enum Building {
    Base,
}

/// Represents player's buildings
pub struct Buildings {
    bases: u32,
}

impl Display for Buildings {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} bases", self.bases)
    }
}

impl Buildings {
    /// Creates new buildings
    pub fn new() -> Self {
        Self {
            bases: BASE_START_NUMBER,
        }
    }

    /// Returns the buildings capacity
    pub fn get_capacity(&self) -> u32 {
        self.bases * BASE_CAPACITY
    }

    pub fn get_buildings(&self, building: Building) -> u32 {
        match building {
            Building::Base => self.bases,
        }
    }

    pub fn build(&mut self, building: &Building) {
        match building {
            Building::Base => self.bases += BASE_INC_NUMBER,
        }
    }
}

pub fn print_info() {
    Building::iter().for_each(|building| {
        let resources = match building {
            Building::Base => BASE_RESOURCES,
        };
        println!("{:?}: {}.", &building, resources);
    });
}
