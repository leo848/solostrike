use shakmaty::{Chess, Position, FromSetup, EnPassantMode::Legal, CastlingMode::Standard};

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
