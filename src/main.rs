use rustfire_core::{board::Board, minion::Minion};

fn main() -> Result<(), String> {
    let mut board = Board::new();

    let mut custom_minion = Minion::test();

    custom_minion.health = 2;

    board.summon_minion(Minion::test(), 1, 0)?;
    board.summon_minion(custom_minion, 2, 0)?;

    println!("{:?}", board);

    board.minion_attack((1, 0), (2, 0))?;

    println!("{:?}", board);

    Ok(())
}
