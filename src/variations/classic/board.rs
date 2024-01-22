use colored::Colorize;

use crate::board_abstract::Board;
use crate::variations::classic::pieces::{Piece, N_PIECES};
use crate::variations::classic::players::{Player, N_PLAYERS};
use crate::variations::classic::squares::N_SQUARES;

pub struct ClassicBoard {
    pub state : Option<[[u64;  N_PIECES]; N_PLAYERS]>
}


impl ClassicBoard {

    // State setter 
    pub fn set_state(&mut self, state : [[u64; N_PIECES]; N_PLAYERS]) {
        self.state = Some(state);
    }


    fn get_transpose(&self) -> [u64; N_SQUARES] {
        
        let state = match self.state {
            Some(state) => {state},
            None => panic!("Board state not set. Unable to generate transpose.")
        };

        let mut transpose : [u64; N_SQUARES] = [0; N_SQUARES];

        for i in 0..64 {
            for j in 0..N_PIECES {
                transpose[i] |= ((state[0][j] >> i) & 1) << j;
                transpose[i] |= ((state[1][j] >> i) & 1) << j + N_PIECES;
            }
        }

        transpose

    }


    // Checks a given transposed state for one or more piece is occupying a given square on the board
    fn check_overlap(&self) -> bool{

        match self
        .get_transpose()
        .iter()
        .map(|x| x.count_ones() > 1)
        .position(|x| x) {
            Some(_) => true,
            None    => false
        }

    }



    // Collapses the bit boards into a single integer encoded array
    pub fn collapse_state_detailed(&self) -> [(Piece, Player); N_SQUARES] {

        self.force_valid();
    
        let mut result = [(Piece::NullPiece, Player::NullPlayer); N_SQUARES];

        for (i, &col) in self.get_transpose().iter().enumerate() {
            result[i] = (
                Piece::get_piece_from_transpose_column(col),
                Player::get_player_from_transpose_column(col)
        )}

        result

    }

}


impl Board for ClassicBoard {

    // Constructor 
    fn new() -> Self {
        Self { state : None }
    }

    fn force_valid(&self) {
        match self.check_overlap() {
            false => {},
            true => panic!("Overlap found in board state")
        };
    }

    fn check_valid(&self) -> bool {
        return self.check_overlap();
    }


}

impl std::fmt::Display for ClassicBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) ->  std::fmt::Result {

        self .force_valid();

        let _state = self.collapse_state_detailed();

        let state : Vec<_> = _state.chunks(8).rev().collect(); 

        let square_colors = [(128,128,128), (79,79,79)];
        let piece_colours = [(255,255,255), (0, 0, 0)];

        for (i,row) in state.iter().enumerate() {
            for (j, (piece, player)) in row.iter().enumerate(){

                let (sr,sg,sb) = square_colors[(i  + j) % 2];

                let (pr,pg,pb) = match *player as usize {
                    0 => piece_colours[0],
                    1 => piece_colours[1],
                    _ => {print!("{}", format!(" {} ", piece.get_short_name()).on_truecolor(sr, sg, sb)); continue}
                };

                print!("{}", 
                format!(" {} ", piece.get_short_name())
                .on_truecolor(sr, sg, sb)
                .truecolor(pr, pg, pb)
            );
            }

        if i < 7 {
            print!("\n");
        }
        
        }



    
        


        write!(f,"")
    }
}
