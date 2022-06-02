mod cards;
mod inputs;
mod logic;

use logic::*;

fn main() {
    let mut table = Table::new();
    table.play_game();
}
