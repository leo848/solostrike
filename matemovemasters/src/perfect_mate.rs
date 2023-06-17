use std::ops::Not;

use shakmaty::{CastlingMode::Standard, Chess, EnPassantMode::Legal, FromSetup, Move, Position, Bitboard};

pub fn perfect_mate(chess: &Chess) -> bool {
    let mut game = chess.board().clone();
    let mut setup = chess.clone().into_setup(Legal);

    for occupant in chess.board().occupied() {
        let prev_game = game.clone();

        game.discard_piece_at(occupant);

        setup.board = game.clone();
        let chess: Chess = match Chess::from_setup(setup.clone(), Standard) {
            Ok(chess) => chess,
            Err(_) => {
                game = prev_game;
                continue;
            }
        };
        if chess.is_checkmate() {
            return false;
        }
        game = prev_game;
    }

    return true;
}

pub fn create_perfect_mate(chess: &Chess, move_: &Move) -> Chess {
    create_perfect_mate_rec(chess, move_, 10)
}

fn create_perfect_mate_rec(chess: &Chess, move_: &Move, depth: usize) -> Chess {
    if depth == 0 {
        return chess.clone()
    }

    let mut game = chess.board().clone();
    let mut setup = chess.clone().into_setup(Legal);

    setup.castling_rights = Bitboard::EMPTY;
    setup.ep_square = None;

    debug_assert!({
        let mut chess = chess.clone();
        chess.play_unchecked(move_);
        chess.is_checkmate()
    });

    for occupant in chess.board().occupied() {
        let prev_game = game.clone();

        game.discard_piece_at(occupant);

        setup.board = game.clone();
        let chess: Chess = match Chess::from_setup(setup.clone(), Standard) {
            Ok(chess) => chess,
            Err(_) => {
                game = prev_game;
                continue;
            }
        };

        let Ok(chess) = chess.play(move_) else {
            game = prev_game;
            continue;
        };

        if chess.is_checkmate().not() {
            game = prev_game;
            continue;
        }
    }

    setup.board = game;
    let mut chess = Chess::from_setup(setup, Standard).expect("Invalid position");

    if !perfect_mate(&{
        let mut chess = chess.clone();
        chess.play_unchecked(move_);
        chess
    }) {
        chess = create_perfect_mate_rec(&chess, move_, depth - 1)
    }

    chess

}
