#[derive(Copy, Clone)]
pub enum Players {
    W,
    B,
    _NPlayers
}

pub const N_PLAYERS: usize = Players::_NPlayers as usize;

impl Players {

    pub fn get_encoding(&self) -> usize {
        match self {
            Players::_NPlayers =>  panic!("_NPlayers is not a valid player"),
            other => *other as usize
        }
       
    }

    pub fn get_short_name(&self) -> char {
        match self {
            Players::W => 'W',
            Players::B => 'B',
            _ => panic!("_NPlayers is not a valid player")
        }
    }

    pub fn get_long_name(&self) -> &'static str {
        match self {
            Players::W => "White",
            Players::B => "Black",
            _ => panic!("_NPlayers is not a valid player")
        }
    }

    pub fn get_piece_from_encoding(encode: usize) -> Players {
        match encode {
            0 => Players::W,
            1 => Players::B,
            _ => panic!("Invalid encoding {} used.", encode)
        }
    }

    pub fn as_array() -> [Players; N_PLAYERS] {[
            Players::W,
            Players::B
        ]
    }

}






