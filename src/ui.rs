use crate::board::{GameState, Hand, PieceType, Player};
use crate::rules::Position;

fn piece_name(piece_type: PieceType, promoted: bool) -> &'static str {
    if promoted {
        match piece_type {
            PieceType::Pawn => "と",
            PieceType::Silver => "全",
            PieceType::Bishop => "馬",
            PieceType::Rook => "龍",
            _ => unreachable!(),
        }
    } else {
        match piece_type {
            PieceType::King => "王",
            PieceType::Gold => "金",
            PieceType::Silver => "銀",
            PieceType::Bishop => "角",
            PieceType::Rook => "飛",
            PieceType::Pawn => "歩",
        }
    }
}

pub fn print_board(state: &GameState, perspective: Player, last_move_to: Option<Position>) {
    let row_labels = ['a', 'b', 'c', 'd', 'e'];

    // 視点に応じて列ヘッダと行の走査順を変える
    match perspective {
        Player::Sente => {
            println!("     5   4   3   2   1");
        }
        Player::Gote => {
            println!("     1   2   3   4   5");
        }
    }
    println!("   +---+---+---+---+---+");

    for i in 0..5 {
        let y = match perspective {
            Player::Sente => i,
            Player::Gote => 4 - i,
        };

        print!(" {}|", row_labels[y]);

        for j in 0..5 {
            let x = match perspective {
                Player::Sente => j,
                Player::Gote => 4 - j,
            };

            let is_last_move = last_move_to.is_some_and(|p| p.x == x && p.y == y);

            match state.board[y][x] {
                Some(p) => {
                    let mark = if is_last_move {
                        "*"
                    } else if p.owner == Player::Sente {
                        " "
                    } else {
                        "^"
                    };
                    print!("{}{}|", mark, piece_name(p.piece_type, p.promoted));
                }
                None => {
                    if is_last_move {
                        print!("*. |");
                    } else {
                        print!(" . |");
                    }
                }
            }
        }
        println!();
        println!("   +---+---+---+---+---+");
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

pub fn print_game_state(
    state: &GameState,
    perspective: Player,
    last_move_to: Option<Position>,
) {
    // 相手の持ち駒を上、自分の持ち駒を下に表示
    let (top_player, bottom_player) = match perspective {
        Player::Sente => (Player::Gote, Player::Sente),
        Player::Gote => (Player::Sente, Player::Gote),
    };

    println!();
    print_hand(state.get_hand(top_player), top_player);
    println!();
    print_board(state, perspective, last_move_to);
    println!();
    print_hand(state.get_hand(bottom_player), bottom_player);
    println!();
}
