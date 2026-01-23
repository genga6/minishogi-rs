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

    // King だけ配置してみる
    board[0][2] = Some(Piece {
        piece_type: PieceType::King,
        owner: Player::Gote,
    });

    board[4][2] = Some(Piece {
        piece_type: PieceType::King,
        owner: Player::Sente,
    });

    board
}
