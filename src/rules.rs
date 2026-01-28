use crate::board::{Board, PieceType, Player};

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
    To(Position, Position), // (from, to)
                            // Drop(Position, PieceType), // (to, type)
}

pub fn generate_legal_moves(board: &Board, player: Player) -> Vec<Move> {
    let mut moves = Vec::new();

    for y in 0..5 {
        for x in 0..5 {
            if let Some(piece) = board[y][x]
                && piece.owner == player
            {
                let from = Position::new(x, y);
                add_move_for_piece(&mut moves, board, player, piece.piece_type, from);
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
    from: Position,
) {
    match p_type {
        PieceType::Rook => {
            let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
            add_sliding_moves(moves, board, player, from, &dirs);
        }
        PieceType::Bishop => {
            let dirs = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
            add_sliding_moves(moves, board, player, from, &dirs);
        }
        _ => {
            let offsets = get_stepping_offsets(player, p_type);
            add_stepping_moves(moves, board, player, from, &offsets);
        }
    }
}

fn add_stepping_moves(
    moves: &mut Vec<Move>,
    board: &Board,
    player: Player,
    from: Position,
    offsets: &[(i8, i8)],
) {
    for &(dx, dy) in offsets {
        if let Some(to) = apply_offset(from, dx, dy)
            && !is_friendly_piece(board, to, player)
        {
            moves.push(Move::To(from, to));
        }
    }
}

fn add_sliding_moves(
    moves: &mut Vec<Move>,
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
                    moves.push(Move::To(from, next));
                }
                break;
            } else {
                moves.push(Move::To(from, next));
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

pub fn make_move(board: &Board, mv: Move) -> Board {
    // 元のboardを保持（&Board）し、複製（*board）した盤面を書き換えて（mut）返す
    let mut new_board = *board;

    match mv {
        Move::To(from, to) => {
            let piece = new_board[from.y][from.x].expect("move_from position should have a piece");

            new_board[to.y][to.x] = Some(piece);

            new_board[from.y][from.x] = None;
        }
    }

    new_board
}
