use shakmaty::{Chess, Position, Outcome};
use shakmaty::fen::Fen;

fn random_game() -> Option<Fen> {
    let mut game = Chess::new();
    let mut prev = game.clone();

    while game.outcome().is_none() {
        prev = game.clone();
        let legal_moves = game.legal_moves();
        let random_idx = fastrand::usize(..legal_moves.len());
        game.play_unchecked(
            &legal_moves[random_idx]
        );
    }

    let Some(Outcome::Decisive { winner }) = game.outcome() else {
        return None;
    };

    println!("{winner} won");
    println!();
    println!("{:?}", prev.board());
    println!();
    println!("{:?}", game.board());

    todo!()
}

fn main() {
    random_game();
}
