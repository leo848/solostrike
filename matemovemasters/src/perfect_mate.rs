use shakmaty::{CastlingMode::Standard, Chess, EnPassantMode::Legal, FromSetup, Move, Position};

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
    todo!()
}
