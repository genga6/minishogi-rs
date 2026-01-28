use minishogi_rs::{board, rules, ui};
use std::io::{self, Write};

fn main() {
    println!("=== 5×5 Mini Shogi Start ===\n");

    let mut state = board::init();
    let mut current_player = board::Player::Sente;

    loop {
        ui::print_game_state(&state);

        if is_game_over(&state) {
            let winner = get_winner(&state);
            match winner {
                Some(board::Player::Sente) => println!("先手の勝ち！"),
                Some(board::Player::Gote) => println!("後手の勝ち！"),
                None => println!("引き分け"),
            }
            break;
        }

        let player_name = match current_player {
            board::Player::Sente => "先手",
            board::Player::Gote => "後手",
        };

        println!("{}の番です", player_name);

        let legal_moves = rules::generate_legal_moves(&state, current_player);
        if legal_moves.is_empty() {
            println!("合法手がありません。負けです。");
            break;
        }

        loop {
            println!("\n入力形式:");
            println!("  移動: <from> <to> (例: 1e 1d)");
            println!("  打つ: drop <駒> <to> (例: drop 金 3c)");
            println!("  終了: quit");
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input == "quit" {
                println!("ゲームを終了します");
                return;
            }

            match parse_input(input, &legal_moves) {
                Ok(mv) => {
                    state = rules::make_move(&state, mv, current_player);
                    break;
                }
                Err(e) => {
                    println!("エラー: {}", e);
                    continue;
                }
            }
        }

        current_player = match current_player {
            board::Player::Sente => board::Player::Gote,
            board::Player::Gote => board::Player::Sente,
        };
    }
}

fn parse_input(input: &str, legal_moves: &[rules::Move]) -> Result<rules::Move, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return Err("入力が空です".to_string());
    }

    let mv = if parts[0] == "drop" {
        if parts.len() != 3 {
            return Err("drop コマンドの形式: drop <駒> <位置>".to_string());
        }

        let piece_type = parse_piece_type(parts[1])?;
        let to = parse_position(parts[2])?;

        rules::Move::Drop(to, piece_type)
    } else {
        if parts.len() != 2 {
            return Err("移動の形式: <from> <to>".to_string());
        }

        let from = parse_position(parts[0])?;
        let to = parse_position(parts[1])?;

        rules::Move::To(from, to)
    };

    if !legal_moves.contains(&mv) {
        return Err("その手は不正です".to_string());
    }

    Ok(mv)
}

fn parse_position(s: &str) -> Result<rules::Position, String> {
    if s.len() != 2 {
        return Err(format!("位置の形式が不正です: {}", s));
    }

    let chars: Vec<char> = s.chars().collect();
    let x_char = chars[0];
    let y_char = chars[1];

    let x = match x_char {
        '1' => 4,
        '2' => 3,
        '3' => 2,
        '4' => 1,
        '5' => 0,
        _ => return Err(format!("x座標が不正です: {}", x_char)),
    };

    let y = match y_char {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        _ => return Err(format!("y座標が不正です: {}", x_char)),
    };

    Ok(rules::Position::new(x, y))
}

fn parse_piece_type(s: &str) -> Result<board::PieceType, String> {
    match s {
        "王" | "玉" => Ok(board::PieceType::King),
        "金" => Ok(board::PieceType::Gold),
        "銀" => Ok(board::PieceType::Silver),
        "角" => Ok(board::PieceType::Bishop),
        "飛" => Ok(board::PieceType::Rook),
        "歩" => Ok(board::PieceType::Pawn),
        _ => Err(format!("不明な駒: {}", s)),
    }
}

fn check_kings_exist(state: &board::GameState) -> (bool, bool) {
    let mut sente_king_exists = false;
    let mut gote_king_exists = false;

    for row in &state.board {
        for piece in row.iter().flatten() {
            if piece.piece_type == board::PieceType::King {
                match piece.owner {
                    board::Player::Sente => sente_king_exists = true,
                    board::Player::Gote => gote_king_exists = true,
                }
            }
        }
    }

    (sente_king_exists, gote_king_exists)
}

fn is_game_over(state: &board::GameState) -> bool {
    let (sente_king_exists, gote_king_exists) = check_kings_exist(state);
    !sente_king_exists || !gote_king_exists
}

fn get_winner(state: &board::GameState) -> Option<board::Player> {
    let (sente_king_exists, gote_king_exists) = check_kings_exist(state);

    if !sente_king_exists {
        Some(board::Player::Gote)
    } else if !gote_king_exists {
        Some(board::Player::Sente)
    } else {
        None
    }
}
