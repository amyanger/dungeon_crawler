mod dungeon;
mod player;
mod enemy;
mod game;

use game::Game;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let mut game = Game::new();
    let result = game.run();

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;

    result
}