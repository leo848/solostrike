use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::convert::identity;
use std::fs::OpenOptions;
use std::iter;

use shakmaty::fen::Fen;
use shakmaty::{Chess, MoveList, Outcome, Position};

fn more_than_one_checkmate(chess: Chess, moves: &MoveList) -> bool {
    let mut game = chess.clone();
    moves
        .iter()
        .map(|move_| {
            game.play_unchecked(move_);
            let game_over = game.is_game_over();
            game = chess.clone();
            game_over
        })
        .filter(|&b| b)
        .nth(1)
        .is_some()
}

fn random_game() -> Option<Fen> {
    let mut game = Chess::new();
    let mut prev = game.clone();
    let mut legal_moves = MoveList::new();
    
    while !game.is_game_over() {
        if game.fullmoves().get() >= 100 {
            return None;
        }
        prev = game.clone();
        legal_moves = game.legal_moves();
        let random_idx = fastrand::usize(..legal_moves.len());
        game.play_unchecked(&legal_moves[random_idx]);
    }

    let Some(Outcome::Decisive { .. }) = game.outcome() else {
        return None;
    };

    if more_than_one_checkmate(prev.clone(), &legal_moves) {
        return None;
    }

    Some(Fen::from_setup(
        prev.into_setup(shakmaty::EnPassantMode::Legal),
    ))
}

const AMOUNT: usize = 100_000;
const MOD: usize = 128;

fn main() {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("fens.json")
        .expect("failed to open file");
    let bar = ProgressBar::new(AMOUNT as u64);

    bar.set_style(
        ProgressStyle::with_template("[{eta_precise}] {bar:50} {percent}% ({per_sec})")
            .expect("failed to create progress bar"),
    );

    bar.tick();

    let output = iter::repeat_with(random_game)
        .filter_map(identity)
        .take(AMOUNT)
        .map(|fen| fen.to_string())
        .enumerate()
        .inspect(|(i, _fen)| {
            if i % MOD == MOD - 1 {
                bar.inc(MOD as u64);
            }
        })
        .map(|(_, fen)| fen)
        .collect::<Vec<String>>();
    bar.finish();

    serde_json::to_writer_pretty(file, &output).expect("failed to serialize");
}
