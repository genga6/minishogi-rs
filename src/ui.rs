use crate::board::{Board, GameState, Hand, PieceType, Player};

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

pub fn print_hand(hand: &Hand, player: Player) {
    let player_name = match player {
        Player::Sente => "先手",
        Player::Gote => "後手",
    };

    print!("{}の持ち駒: ", player_name);

    let mut pieces = Vec::new();
    if hand.gold > 0 {
        pieces.push(format!("金{}", hand.gold));
    }
    if hand.silver > 0 {
        pieces.push(format!("銀{}", hand.silver));
    }
    if hand.bishop > 0 {
        pieces.push(format!("角{}", hand.bishop));
    }
    if hand.rook > 0 {
        pieces.push(format!("飛{}", hand.rook));
    }
    if hand.pawn > 0 {
        pieces.push(format!("歩{}", hand.pawn));
    }

    if pieces.is_empty() {
        println!("なし");
    } else {
        println!("{}", pieces.join(" "));
    }
}

pub fn print_game_state(state: &GameState) {
    println!();
    print_hand(&state.gote_hand, Player::Gote);
    println!();
    print_board(&state.board);
    println!();
    print_hand(&state.sente_hand, Player::Sente);
    println!();
}
