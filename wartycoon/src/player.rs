pub mod actions;
pub mod buildings;
pub mod resources;
pub mod units;

use std::fmt::{Display, Formatter, Result};

use actions::Action;
use buildings::{Building, Buildings};
use resources::{Resource, Resources};
use units::{Unit, Units};
use wartycoon::{read_enum, read_number};

use self::buildings::{BASE_INC_NUMBER, BASE_RESOURCES};
use self::units::{ARCHER_RESOURCES, WARRIOR_RESOURCES};

/// Represents player in game
pub struct Player {
    pub name: String,
    buildings: Buildings,
    units_in_base: Units,
    units_in_terrain: Units,
    resources: Resources,
    pub score: u32,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Player: {}\nBuildings: {}\nUnits in base: {}\nUnits in terrain: {}\nAvailible resources: {}\nScore: {}\n",
            self.name, self.buildings, self.units_in_base, self.units_in_terrain, self.resources, self.score)
    }
}

impl Player {
    /// Creates a new player
    ///
    /// ```
    /// let name = String::from("Frodo");
    /// let player = Player::new(name);
    /// assert player.name == "Frodo";
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            buildings: Buildings::new(),
            units_in_base: Units::new(),
            units_in_terrain: Units::new(),
            resources: Resources::new(),
            score: 0,
        }
    }

    /// Adds point to player's score
    pub fn add_score(&mut self) {
        self.score += 1;
    }

    /// Returns strength of units in terrain
    pub fn get_strength(&self) -> f32 {
        self.units_in_terrain.get_strength()
    }

    /// Adds new units to the player
    ///
    /// ```
    /// let name = String::from("Frodo");
    /// let player = Player::new(name);
    /// let num_of_units = 3;
    /// let unit_type = Unit::Warrior;
    /// player.train(unit_type, num_of_units);
    /// assert player.units_in_base.warriors == num_of_units;
    /// ```
    pub fn train(&mut self) -> bool {
        let unit = read_enum::<Unit, _>();
        print!("Enter number of units to train: ");
        let number = read_number();
        if self.units_in_base.get_number() + number > self.buildings.get_capacity() {
            println!(
                "There is no room for {} units in {} bases!",
                number,
                self.buildings.get_buildings(Building::Base)
            );
            return self.do_action();
        }
        let result = match unit {
            Unit::Warrior => self.resources.spend_resources(WARRIOR_RESOURCES, number),
            Unit::Archer => self.resources.spend_resources(ARCHER_RESOURCES, number),
        };
        if !result {
            return self.do_action();
        }

        self.units_in_base.add_units(&unit, number);

        true
    }

    /// Adds new building to player
    pub fn build(&mut self) -> bool {
        let building = &Building::Base;
        let result = match building {
            Building::Base => self
                .resources
                .spend_resources(BASE_RESOURCES, BASE_INC_NUMBER),
        };
        if !result {
            return self.do_action();
        }

        self.buildings.build(building);

        true
    }

    /// Adds new resources to player
    pub fn harvest(&mut self) -> bool {
        let resource = read_enum::<Resource, _>();
        self.resources.harvest(&resource);

        true
    }

    /// Sends units from base to terrain
    pub fn conquer(&mut self) -> bool {
        let unit = read_enum::<Unit, _>();
        print!("Enter number of units to use: ");
        let number = read_number();
        let in_base = self.units_in_base.get_units(&unit);
        if in_base < number {
            println!(
                "You cannot conquer with {} {:?}s, since you have only {} of them in base!",
                number, unit, in_base
            );
            return self.do_action();
        }

        self.units_in_base.remove_units(&unit, number);
        self.units_in_terrain.add_units(&unit, number);

        true
    }

    /// Reads action from user and executes given action
    /// Returns false if player wants to end game
    /// Returns true otherwise
    pub fn do_action(&mut self) -> bool {
        println!("{} it is your turn now. Do something!", self.name);
        match read_enum::<Action, _>() {
            Action::Build => self.build(),
            Action::Harvest => self.harvest(),
            Action::Train => self.train(),
            Action::Conquer => self.conquer(),
            Action::Quit => return false,
        }
    }
}

pub fn print_info() {
    println!("Costs:");
    units::print_info();
    buildings::print_info();
    resources::print_info();
}
