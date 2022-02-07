mod game;
mod lib;
mod player;

use game::Game;
use std::io::Result;

fn main() -> Result<()> {
    let mut game = Game::new()?;
    game.start();

    Ok(())
}
