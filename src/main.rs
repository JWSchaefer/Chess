use std::panic;

use chess::board::Board;
use chess::board_utils::constants::pieces;
use chess::board_utils::constants::positions;


use colored::Colorize;


fn main() {
    println!("\n");
    let mut my_board = Board::new();

    let h_bar = String::from_iter(vec!['—'; 80]).red();
    
    // ————————————————————————————————————————————————————————————————————————————————
    println!("{}", "NULL POSITION".bright_green());
    println!("{}", h_bar);
    
    println!("Board:");
    let _ = panic::catch_unwind(|| {
        println!("{}", my_board);
    });
    println!("Overlap: {}", my_board.check_overlap().unwrap_err());
    println!("\n");

    // ————————————————————————————————————————————————————————————————————————————————

    println!("{}", "EMPTY POSITION".bright_green());
    println!("{}", h_bar);
    my_board.set_state(positions::EMPTY);

    let _ = panic::catch_unwind(|| {
        println!("Board: {}", my_board);
    });
    println!("Overlap: {}", my_board.check_overlap().unwrap());
    print!("\n");

    // ————————————————————————————————————————————————————————————————————————————————

    println!("{}", "DEFAULT POSITION".bright_green());
    println!("{}", h_bar);
    my_board.set_state(positions::DEFAULT);

    println!("Board: {}", my_board);
    println!("Overlap: {}", my_board.check_overlap().unwrap());
    println!("\n");

    // ————————————————————————————————————————————————————————————————————————————————

    println!("{}", "OVERLAPPED POSITION".bright_green());
    println!("{}", h_bar);
    my_board.set_state(positions::TEST_OVERLAP);

    let _ = panic::catch_unwind(|| {
        println!("Board: {}", my_board);
    });
    println!("Overlap: {}", my_board.check_overlap().unwrap());
    println!("\n");

    // ————————————————————————————————————————————————————————————————————————————————

    println!("{}", "PIECES & PIECE LOOKUPS".bright_green());
    println!("{}", h_bar);

    let pieces_short_name_lookup = pieces::get_piece_short_name_lookup();
    let pieces_long_name_lookup  = pieces::get_piece_long_name_lookup();

    println!("The chess pieces are: ");
    print!("{}"  ,pieces_long_name_lookup.get_value(&pieces::K).unwrap());
    print!(", {}",pieces_long_name_lookup.get_value(&pieces::Q).unwrap());
    print!(", {}",pieces_long_name_lookup.get_value(&pieces::R).unwrap());
    print!(", {}",pieces_long_name_lookup.get_value(&pieces::B).unwrap());
    print!(", {}",pieces_long_name_lookup.get_value(&pieces::N).unwrap());
    print!(", {}",pieces_long_name_lookup.get_value(&pieces::P).unwrap());
    println!("\n");

    println!("Their encodings are: "); 
    print!("{}"  ,pieces_short_name_lookup.get_key(&'K').unwrap());
    print!(", {}",pieces_short_name_lookup.get_key(&'Q').unwrap());
    print!(", {}",pieces_short_name_lookup.get_key(&'R').unwrap());
    print!(", {}",pieces_short_name_lookup.get_key(&'B').unwrap());
    print!(", {}",pieces_short_name_lookup.get_key(&'N').unwrap());
    print!(", {}",pieces_short_name_lookup.get_key(&'P').unwrap());
    print!("\n");
    println!("{}", h_bar);







}

