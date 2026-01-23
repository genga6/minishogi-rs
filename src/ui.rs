use crate::board::{Board, PieceType, Player};

pub fn print_board(board: &Board) {
    println!("   5   4   3   2   1");
    println!(" +---+---+---+---+---+");

    for (y, row) in board.iter().enumerate() {
        print!("{}|", y + 1);

        for cell in row.iter() {
            match cell {
                Some(p) => {
                    let owner_mark = if p.owner == Player::Sente { " " } else { "^" };
                    let name = match p.piece_type {
                        PieceType::King => "王",
                        PieceType::Gold => "金",
                        PieceType::Silver => "銀",
                        PieceType::Bishop => "角",
                        PieceType::Rook => "飛",
                        PieceType::Pawn => "歩",
                    };
                    print!("{}{}|", owner_mark, name);
                }
                None => print!(" . |"),
            }
        }
        println!("\n +---+---+---+---+---+");
    }
}
