use minishogi_rs::{board, rules, ui};

fn main() {
    println!("=== 5×5 Shogi Start ===");

    let mut state = board::init();
    ui::print_game_state(&state);

    println!("\n--- Sente's Legal Moves ---");
    let moves = rules::generate_legal_moves(&state, board::Player::Sente);
    println!("Total: {} moves", moves.len());

    if let Some(first_move) = moves.first() {
        println!("\n--- Making first move ---");

        match first_move {
            rules::Move::To(from, to) => {
                println!("Move: ({}, {}) -> ({}, {})", from.x, from.y, to.x, to.y);
            }
            rules::Move::Drop(to, piece_type) => {
                println!("Drop: {:?} at ({}, {})", piece_type, to.x, to.y);
            }
        }

        state = rules::make_move(&state, *first_move, board::Player::Sente);
        println!("\n--- After move ---");
        ui::print_game_state(&state);

        println!("\n--- Gote's Legal Moves ---");
        let gote_moves = rules::generate_legal_moves(&state, board::Player::Gote);
        println!("Total: {} moves", gote_moves.len());
    }
}
