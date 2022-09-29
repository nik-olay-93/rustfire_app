use rustfire_core::{board::Board, minion::Minion};

fn main() -> Result<(), String> {
    let mut board = Board::new();

    let mut minion = Minion::test();

    minion.on_summon = Some(|board, side, slot| {
        if let Err(error) = board.summon_minion(Minion::test(), 2, 0) {
            println!("{}", error);
        };
        println!("Summoned minion on slot {} of side {}", slot, side);
    });

    board.summon_minion(minion, 1, 0)?;

    println!("{:?}", board);

    Ok(())
}
