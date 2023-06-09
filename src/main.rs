use shakmaty::{Chess, Position};
use shakmaty::fen::Fen;

fn random_game() -> Option<Fen> {
    let mut game = Chess::new();

    while game.outcome().is_none() {
        let legal_moves = game.legal_moves();
        let random_idx = fastrand::usize(..legal_moves.len());
        game.play_unchecked(
            &legal_moves[random_idx]
        );
    }

    todo!()
}

fn main() {
    println!("Hello, world!");
}
