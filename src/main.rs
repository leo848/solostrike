use std::convert::identity;
use std::fs::OpenOptions;
use std::iter;

use shakmaty::{Chess, Position, Outcome, MoveList};
use shakmaty::fen::Fen;

fn more_than_one_checkmate(chess: Chess, moves: &MoveList) -> bool {
    let mut game = chess.clone();
    moves.iter().map(|move_| {
        game.play_unchecked(move_);
        let game_over = game.is_game_over();
        game = chess.clone();
        game_over
    }).filter(|&b| b).nth(1).is_some()
}

fn random_game() -> Option<Fen> {
    let mut game = Chess::new();
    let mut prev = game.clone();
    let mut legal_moves = MoveList::new();

    while game.outcome().is_none() {
        prev = game.clone();
        legal_moves = game.legal_moves();
        let random_idx = fastrand::usize(..legal_moves.len());
        game.play_unchecked(
            &legal_moves[random_idx]
        );
    }

    let Some(Outcome::Decisive { .. }) = game.outcome() else {
        return None;
    };

    if more_than_one_checkmate(prev.clone(), &legal_moves) {
        return None;
    }

    Some(Fen::from_setup(prev.into_setup(shakmaty::EnPassantMode::Legal)))
}

fn main() {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("fens.json")
        .expect("failed to open file");
    let output = iter::repeat_with(random_game)
        .filter_map(identity)
        .take(1_000)
        .map(|fen| fen.to_string())
        .collect::<Vec<String>>();
    serde_json::to_writer_pretty(file, &output).expect("failed to serialize");
}
