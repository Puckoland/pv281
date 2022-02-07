use crate::lib::read_number;
use super::player::{self, Player};
use std::io::Result;
use wartycoon::read_line;

const RULES: &str = "Welcome to Wartycoon!
Each player can do one action every round.
At the end of each round there is fight between troops in field. Winner of fight gets one point.
Player with most points at end of the game wins.
May the best player win!";

/// Represents Wartycoon game
pub struct Game {
    ended: bool,
    round: u32,
    max_rounds: u32,
    players: Vec<Player>,
}

impl Game {
    /// Creates new instance of Wartycoon game
    pub fn new() -> Result<Self> {
        print!("Enter maximum number of rounds: ");
        let max_rounds = read_number(); 
        let mut game = Self {
            ended: false,
            round: 0,
            max_rounds,
            players: Vec::new(),
        };
        game.add_player()?;
        game.add_player()?;

        Ok(game)
    }

    /// Adds new player to game
    fn add_player(&mut self) -> Result<()> {
        let new_player_number = self.players.len() + 1;
        print!("Enter your name player {}: ", new_player_number);

        match read_line() {
            Ok(name) => self.players.push(Player::new(name)),
            Err(_) => {
                println!("An error occured reading your name. Try again!");
                self.add_player()?;
            }
        }

        Ok(())
    }

    /// Simulates fight of players' units
    /// Adds score to winnig player
    fn fight(&mut self) -> Option<()> {
        let mut iterator = self.players.iter_mut();
        let player_1 = iterator.next()?;
        let player_2 = iterator.next()?;

        let strength_1 = player_1.get_strength();
        let strength_2 = player_2.get_strength();

        println!(
            "*** Fight is happening! ***\n{}'s troops strength: {}\n{}'s troops strength: {}",
            player_1.name, strength_1, player_2.name, strength_2
        );

        let winner = if strength_1 > strength_2 {
            player_1
        } else if strength_2 > strength_1 {
            player_2
        } else {
            println!("The fight was a draw! No one gets a point!");
            return Some(());
        };
        println!("{} wins the fight and gets a point!", winner.name.clone());
        winner.add_score();
        Some(())
    }

    /// Prints players' score and announces winner
    fn get_results(&self) -> Option<()> {
        println!("\n\nGame ends!");
        let scores: Vec<(String, u32)> = self
            .players
            .iter()
            .map(|player| (player.name.clone(), player.score))
            .collect();
        println!("Players' score:");
        scores.iter().for_each(|score| print!("Player {}: {}.\n", score.0, score.1));
        let player_1 = self.players.get(0)?;
        let player_2 = self.players.get(1)?;
        match player_1.score {
            scores if scores > player_2.score => {
                println!("Winner is {}! Congratulations!", player_1.name.clone())
            }
            scores if scores < player_2.score => {
                println!("Winner is {}! Congratulations!", player_2.name.clone())
            }
            _ => println!("There is no winner. It is a draw!"),
        };
        Some(())
    }

    /// Plays round of game
    fn play_round(&mut self) {
        self.round += 1;
        println!("\n\n----- ROUND {} -----", self.round);
        self.players.iter().for_each(|p| println!("{}", p));

        self.players.iter_mut().for_each(|player| {
            let result = player.do_action();
            if !result {
                self.ended = true;
            }
        });
        self.fight();
    }

    /// Starts the game
    pub fn start(&mut self) {
        println!("{}", RULES);
        print_info();
        while !self.ended && self.round < self.max_rounds {
            self.play_round();
        }
        self.get_results();
    }
}

fn print_info() {
    player::print_info();
}
