use minishogi_rs::board;
use minishogi_rs::ui;

fn main() {
    println!("=== 5×5 Shogi Start ===");

    let b = board::init();

    ui::print_board(&b);
}
