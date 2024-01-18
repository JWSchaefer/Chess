use crate::board_abstract::Board;
use crate::variations::classic::pieces::{Pieces, N_PIECES};
use crate::variations::classic::players::{Players, N_PLAYERS};
use crate::variations::classic::squares::N_SQUARES;


pub struct ClassicBoard {
    pub state : Option<[[u64;  N_PIECES]; N_PLAYERS]>
}


impl ClassicBoard {

    // State setter 
    pub fn set_state(&mut self, state : [[u64; N_PIECES]; N_PLAYERS]) {
        self.state = Some(state);
    }

    // Calculate Transpose
    fn _transpose(state : [[u64; N_PIECES]; N_PLAYERS]) -> [u64; N_SQUARES] {

        let mut transpose : [u64; N_SQUARES] = [0; N_SQUARES];

        for i in 0..64 {
            for j in 0..N_PIECES {
                transpose[i] |= ((state[0][j] >> i) & 1) << j;
                transpose[i] |= ((state[1][j] >> i) & 1) << j + N_PIECES;
            }
        }

        transpose
  
    }

    fn get_transpose(&self) -> Result<[u64; N_SQUARES], &'static str>{

        match self.state {
            None => Err("Board state not set. Unable to generate transpose."),
            Some(state) => Ok(Self::_transpose(state))
        }

    }

        // Checks a given transposed state for one or more piece is occupying a given square on the board
    fn _check_overlap(transpose :  [u64; N_SQUARES]) -> bool{

        let result = transpose
            .iter()
            .map(|x| x.count_ones() > 1);

        match result.into_iter().position(|x| x) {
            Some(_) => true,
            None    => false
        }

    }

    fn _collapse_state(transpose : [u64; N_SQUARES]) -> ([Pieces; N_SQUARES],[Players;N_SQUARES]) {

        println!("{:?}", transpose);

        ([Pieces::B; N_SQUARES],[Players::W; N_SQUARES])

    }

        // Collapses the bit boards into a single integer encoded array
    pub fn collapse_state(&self) -> Result<([Pieces; N_SQUARES],[Players; N_SQUARES]), &'static str> {


        let transpose =  match self.get_transpose() {
            Ok(transpose) => transpose,
            Err(err) => panic!("{}", err)
        };

        Ok(Self::_collapse_state(transpose))
    }

}


impl Board for ClassicBoard {

    // Constructor 
    fn new() -> Self {
        Self { state : None }
    }


    // Checks if one or more piece is occupying a given square on the board
    fn check_valid(&self) -> bool {

        match self.get_transpose(){
            Err(err) => panic!("{}", err),
            Ok(transpose) => Self::_check_overlap(transpose)
        }


    }


}

impl std::fmt::Display for ClassicBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.check_valid() {
            false => {write!(f, "This is a test")},
            true => {panic!("Overlaps found in board state")},
        }
    }
}
