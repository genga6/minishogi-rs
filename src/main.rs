use minishogi_rs::{board, rules, ui};

fn main() {
    println!("=== 5×5 Shogi Start ===");

    let mut b = board::init();
    ui::print_board(&b);

    println!("\n--- Sente's Legal Moves ---");
    let moves = rules::generate_legal_moves(&b, board::Player::Sente);
    println!("Total: {} moves", moves.len());

    if let Some(first_move) = moves.first() {
        println!("\n--- Making first move ---");

        match first_move {
            rules::Move::To(from, to) => {
                println!("Move: ({}, {}) -> ({}, {})", from.x, from.y, to.x, to.y);
            }
        }

        b = rules::make_move(&b, *first_move);
        println!("\n--- After move ---");
        ui::print_board(&b);

        println!("\n--- Gote's Legal Moves ---");
        let gote_moves = rules::generate_legal_moves(&b, board::Player::Gote);
        println!("Total: {} moves", gote_moves.len());
    }
}
