use minishogi_rs::{board, rules, ui};

fn main() {
    println!("=== 5×5 Shogi Start ===");
    let b = board::init();
    ui::print_board(&b);

    println!("--- Sente's Legal Moves ---");
    let moves = rules::generate_legal_moves(&b, board::Player::Sente);

    for mv in moves {
        match mv {
            rules::Move::To(from, to) => {
                println!("Move: ({}, {}) -> ({}, {})", from.x, from.y, to.x, to.y);
            }
        }
    }
}
