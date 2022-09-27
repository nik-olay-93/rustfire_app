use std::fmt::Error;

use rustfire_core::{
    board::{Board, MinionSlot},
    minions::generate_test_minion,
};

fn main() -> Result<(), Error> {
    let mut board = Board::new();
    let minion = generate_test_minion();
    board.summon_minion(&minion, 1)?;

    if let Some(minionslot) = board.player1.minionslots.get_mut(0) {
        match minionslot {
            MinionSlot::Minion(minion) => {
                println!("Minion: {}", minion.name);
            }
            MinionSlot::None => {}
        }
    }
    Ok(())
}
