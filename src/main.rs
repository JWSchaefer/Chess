use std::panic;

use colored::Colorize;

use chess::board_abstract::Board;
use chess::variations::classic::board::ClassicBoard;
use chess::variations::classic::positions;


fn main() {

    print!("\n");

    let mut my_board = ClassicBoard::new();

    let h_bar = String::from_iter(vec!['—'; 80]).red();
    
    // ————————————————————————————————————————————————————————————————————————————————
    println!("{}", "NULL POSITION".bright_green());
    println!("{}", h_bar);
    
    let _ = panic::catch_unwind(|| {
        println!("{}", my_board);  
    });

    let _ = panic::catch_unwind(|| {
        println!("Overlap: {}", my_board.check_valid());
    });
    
    println!("\n");

    // ————————————————————————————————————————————————————————————————————————————————

    println!("{}", "EMPTY POSITION".bright_green());
    println!("{}", h_bar);
    my_board.set_state(positions::EMPTY);

    let _ = panic::catch_unwind(|| {
        println!("{}", my_board);
    });
 
    println!("Overlap: {}", my_board.check_valid());
    print!("\n");

    // ————————————————————————————————————————————————————————————————————————————————

    println!("{}", "DEFAULT POSITION".bright_green());
    println!("{}", h_bar);
    my_board.set_state(positions::DEFAULT);

    println!("{}", my_board);
    println!("Overlap: {}", my_board.check_valid());
    
    let _ = my_board.collapse_state_detailed();
    
    println!("\n");

    // ————————————————————————————————————————————————————————————————————————————————

    println!("{}", "OVERLAPPED POSITION".bright_green());
    println!("{}", h_bar);
    my_board.set_state(positions::TEST_OVERLAP);

    
    println!("{}", my_board);
    
    println!("Overlap: {}", my_board.check_valid());
    println!("\n");

    // ————————————————————————————————————————————————————————————————————————————————







}

