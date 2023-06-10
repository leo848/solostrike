use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use itertools::Itertools;
use shakmaty::Board;
use shakmaty::Color;
use shakmaty::EnPassantMode;
use shakmaty::Role;
use std::convert::identity;
use std::fs::OpenOptions;
use std::iter;

use shakmaty::fen::Fen;
use shakmaty::{Chess, MoveList, Outcome, Position};

fn one_game_end(chess: &Chess, moves: &MoveList, only_mate: bool) -> bool {
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
        .count()
        == 1
}

fn material_winning(chess: &Chess) -> bool {
    let our_material = chess.our(Role::Pawn).count()
        + (chess.our(Role::Knight) & chess.our(Role::Bishop)).count() * 3
        + chess.our(Role::Rook).count() * 5
        + chess.our(Role::Queen).count() * 9;
    let their_material = chess.their(Role::Pawn).count()
        + (chess.their(Role::Knight) & chess.their(Role::Bishop)).count() * 3
        + chess.their(Role::Rook).count() * 5
        + chess.their(Role::Queen).count() * 9;
    our_material >= their_material
}

struct GameConfig {
    immediate_mode: bool,
    only_material_losing: bool,
}

fn random_game(config: GameConfig) -> Option<Fen> {
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

        if config.immediate_mode {
            if one_game_end(&game, &legal_moves, true) {
                if !(config.only_material_losing && material_winning(&game)) {
                    return Some(Fen::from_setup(game.into_setup(EnPassantMode::Legal)));
                }
            }
        }

        game.play_unchecked(&legal_moves[random_idx]);
    }

    if config.immediate_mode {
        return None;
    }

    let Some(Outcome::Decisive { .. }) = game.outcome() else {
        return None;
    };

    if !one_game_end(&prev, &legal_moves, false) {
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
        .open("fens.json")
        .expect("failed to open file");
    let bar = ProgressBar::new(AMOUNT as u64);

    bar.set_style(
        ProgressStyle::with_template("[{eta_precise}] {bar:50} {percent}% ({per_sec})")
            .expect("failed to create progress bar"),
    );

    bar.tick();

    let output = iter::repeat_with(|| {
        random_game(GameConfig {
            immediate_mode: true,
            only_material_losing: false,
        })
    })
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
