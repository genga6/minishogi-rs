use crate::board::{GameState, PieceType, Player};
use crate::rules::{self, Move};

const SEARCH_DEPTH: u32 = 4;
const INF: i32 = 100_000;

// ---- 評価関数 ----

fn piece_value(piece_type: PieceType, promoted: bool) -> i32 {
    match (piece_type, promoted) {
        (PieceType::Pawn, false) => 100,
        (PieceType::Pawn, true) => 500,
        (PieceType::Silver, false) => 400,
        (PieceType::Silver, true) => 500,
        (PieceType::Gold, _) => 500,
        (PieceType::Bishop, false) => 600,
        (PieceType::Bishop, true) => 800,
        (PieceType::Rook, false) => 700,
        (PieceType::Rook, true) => 900,
        (PieceType::King, _) => 0,
    }
}

fn evaluate(state: &GameState) -> i32 {
    // 終局判定: 王が取られていれば即座に極値を返す
    let sente_king = has_king(state, Player::Sente);
    let gote_king = has_king(state, Player::Gote);
    if !sente_king {
        return -INF;
    }
    if !gote_king {
        return INF;
    }

    let mut score = 0;

    // 盤上の駒
    for row in &state.board {
        for cell in row.iter().flatten() {
            let value = piece_value(cell.piece_type, cell.promoted);
            match cell.owner {
                Player::Sente => score += value,
                Player::Gote => score -= value,
            }
        }
    }

    // 持ち駒（未成り価値）
    let hand_types = [
        PieceType::Pawn,
        PieceType::Silver,
        PieceType::Gold,
        PieceType::Bishop,
        PieceType::Rook,
    ];

    for &pt in &hand_types {
        let value = piece_value(pt, false);
        score += value * state.sente_hand.get(pt) as i32;
        score -= value * state.gote_hand.get(pt) as i32;
    }

    score
}

fn has_king(state: &GameState, player: Player) -> bool {
    state.board.iter().any(|row| {
        row.iter()
            .any(|cell| cell.is_some_and(|p| p.piece_type == PieceType::King && p.owner == player))
    })
}

// ---- Alpha-Beta 探索 ----

pub fn best_move_alpha_beta(state: &GameState, player: Player) -> Option<Move> {
    let legal_moves = rules::generate_legal_moves(state, player);
    if legal_moves.is_empty() {
        return None;
    }

    let maximizing = player == Player::Sente;
    let mut best_mv = None;
    // 先手なら、最初は「無限の負（最低点）」をセットし、それより高い点数を探す。
    // 後手なら、最初は「無限の正（最高点）」をセットし、それより低い点数を探す。
    let mut best_score = if maximizing { -INF - 1 } else { INF + 1 };

    // 全候補手の探索ループ
    for mv in &legal_moves {
        let new_state = rules::make_move(state, *mv, player);
        let score = alpha_beta(&new_state, SEARCH_DEPTH - 1, -INF, INF, !maximizing);

        let is_better = if maximizing {
            score > best_score
        } else {
            score < best_score
        };

        if is_better {
            best_score = score;
            best_mv = Some(*mv);
        }
    }

    best_mv
}

fn alpha_beta(
    state: &GameState,
    depth: u32,
    mut alpha: i32,
    mut beta: i32,
    maximizing: bool,
) -> i32 {
    // 終局判定
    if !has_king(state, Player::Sente) {
        return -INF;
    }
    if !has_king(state, Player::Gote) {
        return INF;
    }

    // 葉ノード
    if depth == 0 {
        return evaluate(state);
    }

    let player = if maximizing {
        Player::Sente
    } else {
        Player::Gote
    };
    let legal_moves = rules::generate_legal_moves(state, player);

    if legal_moves.is_empty() {
        return if maximizing { -INF } else { INF };
    }

    if maximizing {
        let mut max_eval = -INF;
        for mv in &legal_moves {
            let new_state = rules::make_move(state, *mv, player);
            let eval = alpha_beta(&new_state, depth - 1, alpha, beta, false);
            max_eval = max_eval.max(eval);
            alpha = alpha.max(eval);
            if beta <= alpha {
                break;
            }
        }
        max_eval
    } else {
        let mut min_eval = INF;
        for mv in &legal_moves {
            let new_state = rules::make_move(state, *mv, player);
            let eval = alpha_beta(&new_state, depth - 1, alpha, beta, true);
            min_eval = min_eval.min(eval);
            beta = beta.min(eval);
            if beta <= alpha {
                break;
            }
        }
        min_eval
    }
}

// ---- MCTS （モンテカルロ木探索） ----

const MCTS_ITERATIONS: u32 = 20_000;
const MCTS_UCB1_C: f64 = 1.41;
/// ランダムプレイアウトの手数（短くして評価関数で補う）
const MCTS_ROLLOUT_DEPTH: u32 = 10;

/// xorshift64 による高速な擬似乱数生成器
struct Rng {
    state: u64,
}

impl Rng {
    fn new() -> Self {
        use std::time::SystemTime;
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64
            | 1; // 0 を避ける
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    fn range(&mut self, max: usize) -> usize {
        (self.next() as usize) % max
    }
}

struct MctsNode {
    mv: Option<Move>,
    player: Player, // この局面での手番
    state: GameState,
    visits: u32,
    wins: f64,
    children: Vec<MctsNode>,
    untried_moves: Vec<Move>,
}

impl MctsNode {
    fn new(state: GameState, player: Player, mv: Option<Move>) -> Self {
        let untried_moves = rules::generate_legal_moves(&state, player);
        Self {
            mv,
            player,
            state,
            visits: 0,
            wins: 0.0,
            children: Vec::new(),
            untried_moves,
        }
    }

    fn is_fully_expanded(&self) -> bool {
        self.untried_moves.is_empty()
    }

    fn is_terminal(&self) -> bool {
        !has_king(&self.state, Player::Sente)
            || !has_king(&self.state, Player::Gote)
            || (self.untried_moves.is_empty() && self.children.is_empty())
    }

    fn ucb1(&self, parent_visits: u32) -> f64 {
        if self.visits == 0 {
            return f64::INFINITY;
        }
        let exploitation = self.wins / self.visits as f64;
        let exploration = MCTS_UCB1_C * ((parent_visits as f64).ln() / self.visits as f64).sqrt();
        exploitation + exploration
    }

    fn best_child_index(&self) -> Option<usize> {
        if self.children.is_empty() {
            return None;
        }
        let parent_visits = self.visits;
        self.children
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                a.ucb1(parent_visits)
                    .partial_cmp(&b.ucb1(parent_visits))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
    }

    fn expand(&mut self) -> usize {
        let mv = self.untried_moves.pop().unwrap();
        let new_state = rules::make_move(&self.state, mv, self.player);
        let next_player = rules::opponent_of(self.player);
        let child = MctsNode::new(new_state, next_player, Some(mv));
        self.children.push(child);
        self.children.len() - 1
    }
}

/// 短いランダムプレイアウト + 評価関数で勝率を推定
/// 評価値を sigmoid で [0, 1] の勝率に変換して返す
/// 返り値は「先手(Sente)から見た勝率」
fn simulate(state: &GameState, starting_player: Player, rng: &mut Rng) -> f64 {
    let mut current_state = *state;
    let mut current_player = starting_player;

    for _ in 0..MCTS_ROLLOUT_DEPTH {
        if !has_king(&current_state, Player::Sente) {
            return 0.0; // 先手の王なし → 先手負け
        }
        if !has_king(&current_state, Player::Gote) {
            return 1.0; // 後手の王なし → 先手勝ち
        }

        // 高速版（打ち歩詰めチェック省略）でプレイアウト
        let moves = rules::generate_moves_fast(&current_state, current_player);
        if moves.is_empty() {
            return if current_player == Player::Sente {
                0.0
            } else {
                1.0
            };
        }

        let idx = rng.range(moves.len());
        current_state = rules::make_move(&current_state, moves[idx], current_player);
        current_player = rules::opponent_of(current_player);
    }

    // プレイアウト終了後、評価関数でスコアリング
    let score = evaluate(&current_state) as f64;
    // sigmoid: score を勝率 [0, 1] に変換（400 はスケーリング定数）
    1.0 / (1.0 + (-score / 400.0).exp())
}

pub fn best_move_mcts(state: &GameState, player: Player) -> Option<Move> {
    let mut root = MctsNode::new(*state, player, None);

    if root.untried_moves.is_empty() {
        return None;
    }

    let mut rng = Rng::new();

    for _ in 0..MCTS_ITERATIONS {
        // 1. 選択 (Selection)
        let mut path: Vec<usize> = Vec::new();
        let mut node = &root;

        while node.is_fully_expanded() && !node.children.is_empty() {
            if node.is_terminal() {
                break;
            }
            if let Some(idx) = node.best_child_index() {
                path.push(idx);
                node = &node.children[idx];
            } else {
                break;
            }
        }

        // 2. 展開 (Expansion) + シミュレーション位置の決定
        let sim_state;
        let sim_player;

        {
            let mut current = &mut root;
            for &idx in &path {
                current = &mut current.children[idx];
            }

            if !current.is_fully_expanded() && !current.is_terminal() {
                let child_idx = current.expand();
                path.push(child_idx);
                sim_state = current.children[child_idx].state;
                sim_player = current.children[child_idx].player;
            } else {
                sim_state = current.state;
                sim_player = current.player;
            }
        }

        // 3. シミュレーション (Simulation)
        let sente_win_rate = simulate(&sim_state, sim_player, &mut rng);

        // 4. 逆伝播 (Backpropagation)
        // 各ノードに「そのノードの手番にとっての勝率」を加算
        let mut current = &mut root;
        current.visits += 1;
        current.wins += if current.player == Player::Sente {
            sente_win_rate
        } else {
            1.0 - sente_win_rate
        };

        for &idx in &path {
            current = &mut current.children[idx];
            current.visits += 1;
            current.wins += if current.player == Player::Sente {
                sente_win_rate
            } else {
                1.0 - sente_win_rate
            };
        }
    }

    // 最も訪問回数の多い子を選択
    root.children
        .iter()
        .max_by_key(|c| c.visits)
        .and_then(|c| c.mv)
}
