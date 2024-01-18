use std::panic;

use colored::Colorize;

use chess::board_abstract::Board;
use chess::variations::classic::board::ClassicBoard;
use chess::variations::classic::positions;


fn main() {
    println!("\n");
    let mut my_board = ClassicBoard::new();

    let h_bar = String::from_iter(vec!['—'; 80]).red();

    // println!("{}", "K".white().on_truecolor(128,128,128));
    // println!("{}", "P".black().on_truecolor(128,128,128));
    // println!("{}", "K".white().on_truecolor(47,79,79));
    // println!("{}", "P".black().on_truecolor(47,79,79));
    
    // ————————————————————————————————————————————————————————————————————————————————
    println!("{}", "NULL POSITION".bright_green());
    println!("{}", h_bar);
    
    println!("Board:");
    let _ = panic::catch_unwind(|| {
        println!("{}", my_board);
    });
    println!("Overlap: {}", my_board.check_valid());
    println!("\n");

    // ————————————————————————————————————————————————————————————————————————————————

    println!("{}", "EMPTY POSITION".bright_green());
    println!("{}", h_bar);
    my_board.set_state(positions::EMPTY);

    let _ = panic::catch_unwind(|| {
        println!("Board: {}", my_board);
    });
    println!("Overlap: {}", my_board.check_valid());
    print!("\n");

    // ————————————————————————————————————————————————————————————————————————————————

    println!("{}", "DEFAULT POSITION".bright_green());
    println!("{}", h_bar);
    my_board.set_state(positions::DEFAULT);

    println!("Board: {}", my_board);
    println!("Overlap: {}", my_board.check_valid());
    println!("Collapsed State:");
    
    let (_,_) = my_board.collapse_state().unwrap();
    
    println!("\n");

    // ————————————————————————————————————————————————————————————————————————————————

    println!("{}", "OVERLAPPED POSITION".bright_green());
    println!("{}", h_bar);
    my_board.set_state(positions::TEST_OVERLAP);

    let _ = panic::catch_unwind(|| {
        println!("Board: {}", my_board);
    });
    println!("Overlap: {}", my_board.check_valid());
    println!("\n");

    // ————————————————————————————————————————————————————————————————————————————————







}

