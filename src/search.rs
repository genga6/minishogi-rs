use crate::board::[GameState, PieceType, Player];
use crate::rules::[self, Move];

const SEARCH_DEPTH: u32 = 4;
const INF: i32 = 100_000;

fn piece_value(piece_type: PieceType, promoted: bool) -> i32 {
    match (piece_type, promoted) {
        (PieceType::Pawn, false) => 100,
        (PieceType::Pawn, true) => 500,
        (PieceType::Silver, false) => 400,
        (PieceType::Silver, true) => 500,
        (PieceType::Gold, _) => 500,
        (PieceType::Bishop, false) => 600,
        (PieceType::Bishop, true) => 800,
        (PieceType::Rook, false) => 700,
        (PieceType::Rook, true) => 900,
        (PieceType::King, _) => 0,
    }
}

fn evaluate(state: &GameState) -> i32 {
    let mut score = 0;

    // 盤上の駒
    for row in &state.board {
        for cell in row.iter().flatten() {
            let value = piece_value(cell.piece_type, cell.promoted);
            match cell.owner {
                Player::Sente => score += value,
                Player::Gote => score -= value,
            }
        }
    }
    
    // 持ち駒（未成り価値）
    let hand_types = [
        PieceType::Pawn,
        PieceType::Silver,
        PieceType::Gold,
        PieceType::Bishop,
        PieceType::Rook,
    ];

    for &pt in &hand_types {
        let value = piece_value(pt, false);
        score += value * state.sente_hand.get(pt) as i32;
        score -= value * state.gote_hand.get(pt) as i32;
    }

    score
}

fn has_king(state: &GameState, player: Player) -> bool {
    state.board.iter().any(|row| {
        row.iter().any(|cell| {
            cell.is_some_and(|p| p.piece_type == PieceType::King && p.owner == player)
        })
    })
}

