use super::{Display, Formatter, Result};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const WOOD_START_NUMBER: u32 = 0;
const GOLD_START_NUMBER: u32 = 0;

const WOOD_INC_NUMBER: u32 = 200;
const GOLD_INC_NUMBER: u32 = 120;

/// Represents resource types
#[derive(Debug, EnumIter)]
pub enum Resource {
    Wood,
    Gold,
}

/// Represents player's resources
pub struct Resources {
    pub wood: u32,
    pub gold: u32,
}

impl Display for Resources {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} wood, {} gold", self.wood, self.gold)
    }
}

impl Resources {
    /// Creates new resources
    pub fn new() -> Self {
        Self {
            wood: WOOD_START_NUMBER,
            gold: GOLD_START_NUMBER,
        }
    }

    /// Adds resources
    pub fn harvest(&mut self, resource: &Resource) {
        match resource {
            Resource::Wood => self.wood += WOOD_INC_NUMBER,
            Resource::Gold => self.gold += GOLD_INC_NUMBER,
        }
    }

    /// Spends given resources
    pub fn spend_resources(&mut self, that: Resources, times: u32) -> bool {
        let wood = that.wood * times;
        let gold = that.gold * times;
        println!("Using {} wood and {} gold.", wood, gold);

        if self.wood < wood || self.gold < gold {
            println!("You do not have enough resorces for that action!");
            return false;
        }

        self.wood -= wood;
        self.gold -= gold;
        true
    }
}

pub fn print_info() {
    Resource::iter().for_each(|resource| {
        let resources = match resource {
            Resource::Wood => WOOD_INC_NUMBER,
            Resource::Gold => GOLD_INC_NUMBER,
        };
        println!("For one harvest you get {:?} {:?}.", &resource, resources);
    });
}
