use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use shakmaty::EnPassantMode;
use std::convert::identity;
use std::fs::OpenOptions;
use std::iter;
use itertools::Itertools;

use shakmaty::fen::Fen;
use shakmaty::{Chess, MoveList, Outcome, Position};

fn one_game_end(chess: Chess, moves: &MoveList, only_mate: bool) -> bool {
    let mut game = chess.clone();
    moves
        .iter()
        .map(|move_| {
            game.play_unchecked(move_);
            let game_over = if only_mate {
                game.is_checkmate()
            } else {
                game.is_game_over()
            };
            game = chess.clone();
            game_over
        })
        .filter(|&b| b)
        .take(2)
        .count() == 1
}

fn random_game(immediate_mode: bool) -> Option<Fen> {
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

        if immediate_mode {
            if one_game_end(game.clone(), &legal_moves, true) {
                return Some(Fen::from_setup(game.into_setup(EnPassantMode::Legal)));
            }
        }

        game.play_unchecked(&legal_moves[random_idx]);
    }

    if immediate_mode {
        return None;
    }

    let Some(Outcome::Decisive { .. }) = game.outcome() else {
        return None;
    };

    if !one_game_end(prev.clone(), &legal_moves, false) {
        return None;
    }

    Some(Fen::from_setup(prev.into_setup(EnPassantMode::Legal)))
}

const AMOUNT: usize = 100_000;
const MOD: usize = 128;

fn main() {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("fens2.json")
        .expect("failed to open file");
    let bar = ProgressBar::new(AMOUNT as u64);

    bar.set_style(
        ProgressStyle::with_template("[{eta_precise}] {bar:50} {percent}% ({per_sec})")
            .expect("failed to create progress bar"),
    );

    bar.tick();

    let output = iter::repeat_with(|| random_game(true))
        .filter_map(identity)
        .map(|fen| fen.to_string())
        .unique()
        .take(AMOUNT)
        .enumerate()
        .inspect(|(i, _fen)| {
            if i % MOD == MOD - 1 {
                bar.inc(MOD as u64);
            }
        })
        .map(|(_, fen)| fen)
        .sorted_by_key(|fen| fen.len())
        .collect::<Vec<String>>();
    bar.finish();

    serde_json::to_writer_pretty(file, &output).expect("failed to serialize");
}
