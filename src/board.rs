#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Sente,
    Gote,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    King,
    Gold,
    Silver,
    Bishop,
    Rook,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub owner: Player,
}

// [行(y)][列(x)]
// x=0  x=1  x=2  x=3  x=4
// y=0 [ .. , .. , .. , .. , .. ]
// y=1 [ .. , .. , .. , .. , .. ]
// y=2 [ .. , .. , .. , .. , .. ]
// y=3 [ .. , .. , .. , .. , .. ]
// y=4 [ .. , .. , .. , .. , .. ]
pub type Board = [[Option<Piece>; 5]; 5];

pub fn init() -> Board {
    let mut board: Board = [[None; 5]; 5];

    // 後手
    let gote_pieces = [
        PieceType::King,
        PieceType::Gold,
        PieceType::Silver,
        PieceType::Bishop,
        PieceType::Rook,
    ];

    for (x, &pt) in gote_pieces.iter().enumerate() {
        board[0][x] = Some(Piece {
            piece_type: pt,
            owner: Player::Gote,
        });
    }

    board[1][0] = Some(Piece {
        piece_type: PieceType::Pawn,
        owner: Player::Gote,
    });

    // 先手
    let sente_pieces = [
        PieceType::Rook,
        PieceType::Bishop,
        PieceType::Silver,
        PieceType::Gold,
        PieceType::King,
    ];

    for (x, &pt) in sente_pieces.iter().enumerate() {
        board[4][x] = Some(Piece {
            piece_type: pt,
            owner: Player::Sente,
        });
    }

    board[3][4] = Some(Piece {
        piece_type: PieceType::Pawn,
        owner: Player::Sente,
    });

    board
}
