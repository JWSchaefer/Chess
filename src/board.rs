// use crate::board_utils::constants::pieces;
use crate::board_utils::constants::general::N_PIECES;



pub struct Board {
    pub state : Option<[[u64; N_PIECES]; 2]>
}

impl Board {

    // Constructor 
    pub fn new() -> Self {
        Self { state : None }
    }

    // State setter 
    pub fn set_state(&mut self, state : [[u64; N_PIECES]; 2]) {
        self.state = Some(state);
    }

    fn _transpose(state : [[u64; N_PIECES]; 2]) -> [u64; 64] {

        let mut transpose : [u64; 64] = [0; 64];

        for i in 0..64 {
            for j in 0..N_PIECES {
                transpose[i] |= ((state[0][j] >> i) & 1) << j;
                transpose[i] |= ((state[1][j] >> i) & 1) << j + N_PIECES;
            }
        }

        transpose
  
    }

    fn get_transpose(&self) -> Result<[u64; 64], &'static str>{

        match self.state {
            None => Err("Board state not set. Unable to generate transpose."),
            Some(state) => Ok(Self::_transpose(state))
        }

    }

    // Checks a given transposed state for one or more piece is occupying a given square on the board
    fn _check_overlap(transpose :  [u64; 64]) -> bool{

        let result = transpose
            .iter()
            .map(|x| x.count_ones() > 1);

        match result.into_iter().position(|x| x) {
            Some(_) => true,
            None    => false
        }

    }

    // Checks if one or more piece is occupying a given square on the board
    pub fn check_overlap(&self) -> Result<bool, &'static str> {

        match self.get_transpose() {
            Err(err) => Err(err),
            Ok(transpose) => Ok(Self::_check_overlap(transpose))
        }

    }

    fn _collapse_state(transpose : [u64; 64]) -> [usize; 64] {

        [0; 64]

    }

    // Collapses the bit boards into a single integer encoded array
    pub fn collapse_state(&self) -> Result<[usize; 64], &'static str> {

        let transpose =  match self.get_transpose() {
            Ok(transpose) => transpose,
            Err(err) => panic!("{}", err)
        };

        Ok(Self::_collapse_state(transpose))
    }

}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.check_overlap() {
            Ok(false) => {write!(f, "This is a test")},
            Ok(true) => {panic!("Overlaps found in board state")},
            Err(err) => {panic!("While attepting to display board, the following error was encountered:\n{}", err)}
        }
    }
}
