use crate::board_utils::lookup::Lookup;

pub const K : usize = 0;
pub const Q : usize = 1;
pub const R : usize = 2;
pub const B : usize = 3;
pub const N : usize = 4;
pub const P : usize = 5; 

pub fn get_piece_short_name_lookup() -> Lookup<usize, char> {
    let keys = vec![0, 1, 2, 3, 4, 5];
    let values = vec!['K', 'Q', 'R', 'B', 'N', 'P'];

    Lookup { keys, values }
}

pub fn get_piece_long_name_lookup() -> Lookup<usize, &'static str> {
    let keys = vec![0, 1, 2, 3, 4, 5];
    let values = vec!["King", "Queen", "Rook", "Bishop", "Knight", "Pawn"];

    Lookup { keys, values }
}


