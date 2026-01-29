use crate::board::{Board, GameState, Piece, PieceType, Player};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    To(Position, Position, bool), // (from, to, promote)
    Drop(Position, PieceType),    // (to, type)
}

pub fn generate_legal_moves(state: &GameState, player: Player) -> Vec<Move> {
    let mut moves = generate_moves(state, player);

    // 打ち歩詰めチェック: 歩のdropで相手が詰みになる手を除外
    let opponent = opponent_of(player);
    moves.retain(|mv| {
        if let Move::Drop(_, PieceType::Pawn) = mv {
            let new_state = make_move(state, *mv, player);
            !is_checkmate(&new_state, opponent)
        } else {
            true
        }
    });

    moves
}

/// 二歩チェック付きの手生成（打ち歩詰めチェックなし）
fn generate_moves(state: &GameState, player: Player) -> Vec<Move> {
    let mut moves = Vec::new();

    for y in 0..5 {
        for x in 0..5 {
            if let Some(piece) = state.board[y][x]
                && piece.owner == player
            {
                let from = Position::new(x, y);
                add_move_for_piece(
                    &mut moves,
                    &state.board,
                    player,
                    piece.piece_type,
                    piece.promoted,
                    from,
                );
            }
        }
    }

    let hand = state.get_hand(player);
    let piece_types = [
        PieceType::Pawn,
        PieceType::Silver,
        PieceType::Gold,
        PieceType::Bishop,
        PieceType::Rook,
    ];

    // 二歩チェック用: 各列に自分の未成り歩があるか事前計算
    let mut pawn_columns = [false; 5];
    for row in &state.board {
        for (x, cell) in row.iter().enumerate() {
            if let Some(piece) = cell
                && piece.piece_type == PieceType::Pawn
                && piece.owner == player
                && !piece.promoted
            {
                pawn_columns[x] = true;
            }
        }
    }

    for &piece_type in &piece_types {
        if hand.get(piece_type) > 0 {
            for (y, row) in state.board.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if cell.is_none() {
                        if piece_type == PieceType::Pawn && pawn_columns[x] {
                            continue;
                        }
                        moves.push(Move::Drop(Position::new(x, y), piece_type));
                    }
                }
            }
        }
    }

    moves
}

fn add_move_for_piece(
    moves: &mut Vec<Move>,
    board: &Board,
    player: Player,
    p_type: PieceType,
    promoted: bool,
    from: Position,
) {
    let destinations = collect_piece_destinations(board, player, p_type, promoted, from);
    let can_promote = !promoted && can_piece_promote(p_type);

    for to in destinations {
        if can_promote && (is_in_promotion_zone(player, from.y) || is_in_promotion_zone(player, to.y))
        {
            moves.push(Move::To(from, to, true));
            // 歩が最奥段に到達した場合は強制成り（不成の選択肢なし）
            if !must_promote(player, p_type, to.y) {
                moves.push(Move::To(from, to, false));
            }
        } else {
            moves.push(Move::To(from, to, false));
        }
    }
}

fn can_piece_promote(p_type: PieceType) -> bool {
    matches!(
        p_type,
        PieceType::Pawn | PieceType::Silver | PieceType::Bishop | PieceType::Rook
    )
}

fn is_in_promotion_zone(player: Player, y: usize) -> bool {
    match player {
        Player::Sente => y == 0,
        Player::Gote => y == 4,
    }
}

fn must_promote(player: Player, p_type: PieceType, to_y: usize) -> bool {
    p_type == PieceType::Pawn && is_in_promotion_zone(player, to_y)
}

fn collect_stepping_moves(
    destinations: &mut Vec<Position>,
    board: &Board,
    player: Player,
    from: Position,
    offsets: &[(i8, i8)],
) {
    for &(dx, dy) in offsets {
        if let Some(to) = apply_offset(from, dx, dy)
            && !is_friendly_piece(board, to, player)
        {
            destinations.push(to);
        }
    }
}

fn collect_sliding_moves(
    destinations: &mut Vec<Position>,
    board: &Board,
    player: Player,
    from: Position,
    dirs: &[(i8, i8)],
) {
    for &(dx, dy) in dirs {
        let mut curr = from;
        while let Some(next) = apply_offset(curr, dx, dy) {
            if let Some(target_piece) = board[next.y][next.x] {
                if target_piece.owner != player {
                    destinations.push(next);
                }
                break;
            } else {
                destinations.push(next);
                curr = next;
            }
        }
    }
}

fn apply_offset(pos: Position, dx: i8, dy: i8) -> Option<Position> {
    let nx = pos.x as i8 + dx;
    let ny = pos.y as i8 + dy;

    if (0..5).contains(&nx) && (0..5).contains(&ny) {
        Some(Position::new(nx as usize, ny as usize))
    } else {
        None
    }
}

fn is_friendly_piece(board: &Board, pos: Position, player: Player) -> bool {
    if let Some(piece) = board[pos.y][pos.x] {
        piece.owner == player
    } else {
        false
    }
}

fn get_stepping_offsets(player: Player, p_type: PieceType) -> Vec<(i8, i8)> {
    let forward = (0, -1);
    let back = (0, 1);
    let side_left = (-1, 0);
    let side_right = (1, 0);
    let diag_f_left = (-1, -1);
    let diag_f_right = (1, -1);
    let diag_b_left = (-1, 1);
    let diag_b_right = (1, 1);

    let mut offsets = match p_type {
        PieceType::King => vec![
            forward,
            back,
            side_left,
            side_right,
            diag_f_left,
            diag_f_right,
            diag_b_left,
            diag_b_right,
        ],
        PieceType::Gold => vec![
            forward,
            back,
            side_left,
            side_right,
            diag_f_left,
            diag_f_right,
        ],
        PieceType::Silver => vec![
            forward,
            diag_f_left,
            diag_f_right,
            diag_b_left,
            diag_b_right,
        ],
        PieceType::Pawn => vec![forward],
        _ => vec![],
    };

    if player == Player::Gote {
        for p in &mut offsets {
            p.0 = -p.0;
            p.1 = -p.1;
        }
    }

    offsets
}

pub fn opponent_of(player: Player) -> Player {
    match player {
        Player::Sente => Player::Gote,
        Player::Gote => Player::Sente,
    }
}

fn find_king(state: &GameState, player: Player) -> Option<Position> {
    for y in 0..5 {
        for x in 0..5 {
            if let Some(piece) = state.board[y][x]
                && piece.piece_type == PieceType::King
                && piece.owner == player
            {
                return Some(Position::new(x, y));
            }
        }
    }
    None
}

fn collect_piece_destinations(
    board: &Board,
    player: Player,
    p_type: PieceType,
    promoted: bool,
    from: Position,
) -> Vec<Position> {
    let mut destinations = Vec::new();

    if promoted {
        match p_type {
            PieceType::Rook => {
                let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
                collect_sliding_moves(&mut destinations, board, player, from, &dirs);
                let diag = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
                collect_stepping_moves(&mut destinations, board, player, from, &diag);
            }
            PieceType::Bishop => {
                let dirs = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
                collect_sliding_moves(&mut destinations, board, player, from, &dirs);
                let ortho = [(0, -1), (0, 1), (-1, 0), (1, 0)];
                collect_stepping_moves(&mut destinations, board, player, from, &ortho);
            }
            PieceType::Pawn | PieceType::Silver => {
                let offsets = get_stepping_offsets(player, PieceType::Gold);
                collect_stepping_moves(&mut destinations, board, player, from, &offsets);
            }
            _ => {}
        }
    } else {
        match p_type {
            PieceType::Rook => {
                let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
                collect_sliding_moves(&mut destinations, board, player, from, &dirs);
            }
            PieceType::Bishop => {
                let dirs = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
                collect_sliding_moves(&mut destinations, board, player, from, &dirs);
            }
            _ => {
                let offsets = get_stepping_offsets(player, p_type);
                collect_stepping_moves(&mut destinations, board, player, from, &offsets);
            }
        }
    }

    destinations
}

fn is_in_check(state: &GameState, player: Player) -> bool {
    let king_pos = match find_king(state, player) {
        Some(pos) => pos,
        None => return false,
    };

    let opponent = opponent_of(player);

    for y in 0..5 {
        for x in 0..5 {
            if let Some(piece) = state.board[y][x]
                && piece.owner == opponent
            {
                let from = Position::new(x, y);
                let dests = collect_piece_destinations(
                    &state.board,
                    opponent,
                    piece.piece_type,
                    piece.promoted,
                    from,
                );
                if dests.iter().any(|d| d.x == king_pos.x && d.y == king_pos.y) {
                    return true;
                }
            }
        }
    }

    false
}

fn is_checkmate(state: &GameState, player: Player) -> bool {
    if !is_in_check(state, player) {
        return false;
    }

    let moves = generate_moves(state, player);
    for mv in moves {
        let new_state = make_move(state, mv, player);
        if !is_in_check(&new_state, player) {
            return false;
        }
    }

    true
}

pub fn make_move(state: &GameState, mv: Move, player: Player) -> GameState {
    let mut new_state = *state;

    match mv {
        Move::To(from, to, promote) => {
            let mut piece =
                new_state.board[from.y][from.x].expect("move_from position should have a piece");

            if let Some(captured) = new_state.board[to.y][to.x] {
                new_state.get_hand_mut(player).add(captured.piece_type);
            }

            if promote {
                piece.promoted = true;
            }

            new_state.board[to.y][to.x] = Some(piece);
            new_state.board[from.y][from.x] = None;
        }
        Move::Drop(to, piece_type) => {
            if new_state.get_hand_mut(player).remove(piece_type) {
                let piece = Piece {
                    piece_type,
                    owner: player,
                    promoted: false,
                };
                new_state.board[to.y][to.x] = Some(piece);
            }
        }
    }

    new_state
}
