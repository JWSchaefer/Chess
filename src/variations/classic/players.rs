use super::pieces::N_PIECES;

#[derive(Copy, Clone)]
pub enum Player {
    W,
    B,
    NullPlayer
}

pub const N_PLAYERS: usize = Player::NullPlayer as usize;

pub const AS_ARRAY : [Player; N_PLAYERS] = [Player::W, Player::B];

impl Player {

    pub fn get_encoding(&self) -> usize {
        match self {
            Player::NullPlayer =>  panic!("NullPlayer is not a valid player"),
            other => *other as usize
        }
       
    }

    pub fn get_short_name(&self) -> char {
        match self {
            Player::W => 'W',
            Player::B => 'B',
            Player::NullPlayer => panic!("NullPlayer is not a valid player")
        }
    }

    pub fn get_long_name(&self) -> &'static str {
        match self {
            Player::W => "White",
            Player::B => "Black",
            Player::NullPlayer => panic!("NullPlayer is not a valid player")
        }
    }

    pub fn get_piece_from_encoding(encode: usize) -> Player {
        match encode {
            0 => Player::W,
            1 => Player::B,
            _ => panic!("Invalid encoding {} used.", encode)
        }
    }

    pub fn get_player_from_transpose_column(column : u64) -> Player {

        match column {
            0 => Player::NullPlayer,
            _ => AS_ARRAY[column.trailing_zeros() as usize / N_PIECES]
        }


    }



}






