use minishogi_rs::{board, rules, search, ui};
use std::io::{self, Write};

#[derive(Clone, Copy)]
enum AiAlgorithm {
    AlphaBeta,
    Mcts,
}

fn main() {
    println!("=== 5×5 Mini Shogi Start ===\n");

    let human_player = select_player();
    let ai_algorithm = select_algorithm();
    let ai_player = match human_player {
        board::Player::Sente => board::Player::Gote,
        board::Player::Gote => board::Player::Sente,
    };

    println!();

    let mut state = board::init();
    let mut current_player = board::Player::Sente;
    let mut last_move_to: Option<rules::Position> = None;

    loop {
        ui::print_game_state(&state, human_player, last_move_to);

        if is_game_over(&state) {
            let winner = get_winner(&state);
            match winner {
                Some(p) if p == human_player => println!("あなたの勝ち！"),
                Some(_) => println!("AIの勝ち！"),
                None => println!("引き分け"),
            }
            break;
        }

        let player_name = match current_player {
            board::Player::Sente => "先手",
            board::Player::Gote => "後手",
        };

        if current_player == ai_player {
            println!("{}（AI）の番です。思考中...", player_name);
            let ai_move = match ai_algorithm {
                AiAlgorithm::AlphaBeta => search::best_move_alpha_beta(&state, ai_player),
                AiAlgorithm::Mcts => search::best_move_mcts(&state, ai_player),
            };

            match ai_move {
                Some(mv) => {
                    println!("AIの手: {}", format_move(mv));
                    last_move_to = Some(move_destination(mv));
                    state = rules::make_move(&state, mv, current_player);
                }
                None => {
                    println!("AIに合法手がありません。あなたの勝ち！");
                    break;
                }
            }
        } else {
            println!("{}（あなた）の番です", player_name);

            let legal_moves = rules::generate_legal_moves(&state, current_player);
            if legal_moves.is_empty() {
                println!("合法手がありません。負けです。");
                break;
            }

            loop {
                println!("\n入力形式:");
                println!("  移動: <from> <to> (例: 1e 1d)");
                println!("  成り: <from> <to>+ (例: 1e 1d+)");
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

                match parse_input(input, &legal_moves, &state, current_player) {
                    Ok(mv) => {
                        last_move_to = Some(move_destination(mv));
                        state = rules::make_move(&state, mv, current_player);
                        break;
                    }
                    Err(e) => {
                        println!("エラー: {}", e);
                        continue;
                    }
                }
            }
        }

        current_player = match current_player {
            board::Player::Sente => board::Player::Gote,
            board::Player::Gote => board::Player::Sente,
        };
    }
}

fn move_destination(mv: rules::Move) -> rules::Position {
    match mv {
        rules::Move::To(_, to, _) => to,
        rules::Move::Drop(to, _) => to,
    }
}

fn select_player() -> board::Player {
    loop {
        println!("あなたの手番を選んでください:");
        println!("  s: 先手（先攻）");
        println!("  g: 後手（後攻）");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "s" => return board::Player::Sente,
            "g" => return board::Player::Gote,
            _ => println!("s または g を入力してください"),
        }
    }
}

fn select_algorithm() -> AiAlgorithm {
    loop {
        println!("\nAIアルゴリズムを選んでください:");
        println!("  1: Alpha-Beta探索（評価関数ベース）");
        println!("  2: MCTS（モンテカルロ木探索）");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => return AiAlgorithm::AlphaBeta,
            "2" => return AiAlgorithm::Mcts,
            _ => println!("1 または 2 を入力してください"),
        }
    }
}

fn format_move(mv: rules::Move) -> String {
    match mv {
        rules::Move::To(from, to, promote) => {
            let from_str = format_position(from);
            let to_str = format_position(to);
            if promote {
                format!("{} {}+", from_str, to_str)
            } else {
                format!("{} {}", from_str, to_str)
            }
        }
        rules::Move::Drop(to, piece_type) => {
            let to_str = format_position(to);
            let piece_name = match piece_type {
                board::PieceType::King => "王",
                board::PieceType::Gold => "金",
                board::PieceType::Silver => "銀",
                board::PieceType::Bishop => "角",
                board::PieceType::Rook => "飛",
                board::PieceType::Pawn => "歩",
            };
            format!("drop {} {}", piece_name, to_str)
        }
    }
}

fn format_position(pos: rules::Position) -> String {
    let x_char = match pos.x {
        0 => '5',
        1 => '4',
        2 => '3',
        3 => '2',
        4 => '1',
        _ => '?',
    };
    let y_char = match pos.y {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        _ => '?',
    };
    format!("{}{}", x_char, y_char)
}

fn parse_input(
    input: &str,
    legal_moves: &[rules::Move],
    state: &board::GameState,
    player: board::Player,
) -> Result<rules::Move, String> {
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

        // 持ち駒にあるかチェック
        if state.get_hand(player).get(piece_type) == 0 {
            let name = match piece_type {
                board::PieceType::King => "王",
                board::PieceType::Gold => "金",
                board::PieceType::Silver => "銀",
                board::PieceType::Bishop => "角",
                board::PieceType::Rook => "飛",
                board::PieceType::Pawn => "歩",
            };
            return Err(format!("{}は持ち駒にありません", name));
        }

        rules::Move::Drop(to, piece_type)
    } else {
        if parts.len() != 2 {
            return Err("移動の形式: <from> <to> または <from> <to>+".to_string());
        }

        let from = parse_position(parts[0])?;
        let promote = parts[1].ends_with('+');
        let to_str = if promote {
            &parts[1][..parts[1].len() - 1]
        } else {
            parts[1]
        };
        let to = parse_position(to_str)?;

        rules::Move::To(from, to, promote)
    };

    if !legal_moves.contains(&mv) {
        return Err("その手は合法手ではありません".to_string());
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
        _ => return Err(format!("y座標が不正です: {}", y_char)),
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
