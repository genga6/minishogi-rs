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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hand {
    pub gold: u8,
    pub silver: u8,
    pub bishop: u8,
    pub rook: u8,
    pub pawn: u8,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            gold: 0,
            silver: 0,
            bishop: 0,
            rook: 0,
            pawn: 0,
        }
    }

    pub fn add(&mut self, piece_type: PieceType) {
        match piece_type {
            PieceType::Gold => self.gold += 1,
            PieceType::Silver => self.silver += 1,
            PieceType::Bishop => self.bishop += 1,
            PieceType::Rook => self.rook += 1,
            PieceType::Pawn => self.pawn += 1,
            PieceType::King => {}
        }
    }

    pub fn remove(&mut self, piece_type: PieceType) -> bool {
        // 可変借用
        let count = match piece_type {
            PieceType::Gold => &mut self.gold,
            PieceType::Silver => &mut self.silver,
            PieceType::Bishop => &mut self.bishop,
            PieceType::Rook => &mut self.rook,
            PieceType::Pawn => &mut self.pawn,
            PieceType::King => return false,
        };

        if *count > 0 {
            *count -= 1;
            true
        } else {
            false
        }
    }

    pub fn get(&self, piece_type: PieceType) -> u8 {
        match piece_type {
            PieceType::Gold => self.gold,
            PieceType::Silver => self.silver,
            PieceType::Bishop => self.bishop,
            PieceType::Rook => self.rook,
            PieceType::Pawn => self.pawn,
            PieceType::King => 0,
        }
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameState {
    pub board: Board,
    pub sente_hand: Hand,
    pub gote_hand: Hand,
}

impl GameState {
    pub fn get_hand(&self, player: Player) -> &Hand {
        match player {
            Player::Sente => &self.sente_hand,
            Player::Gote => &self.gote_hand,
        }
    }

    pub fn get_hand_mut(&mut self, player: Player) -> &mut Hand {
        match player {
            Player::Sente => &mut self.sente_hand,
            Player::Gote => &mut self.gote_hand,
        }
    }
}

pub fn init() -> GameState {
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

    GameState {
        board,
        sente_hand: Hand::new(),
        gote_hand: Hand::new(),
    }
}
