#[derive(Copy, Clone)]
pub enum Pieces {
    K,
    Q,
    R,
    B,
    N,
    P,
    _NPieces
}

pub const N_PIECES: usize = Pieces::_NPieces as usize;

impl Pieces {

    pub fn get_encoding(&self) -> usize {
        match self {
            Pieces::_NPieces => panic!("_NPieces is not a valid piece"),
            other => *other as usize,
        }
    }

    pub fn get_short_name(&self) -> char {
        match self {
            Pieces::K => 'K',
            Pieces::Q => 'Q',
            Pieces::R => 'R',
            Pieces::B => 'B',
            Pieces::N => 'N',
            Pieces::P => 'P',
            _ => panic!("_NPieces is not a valid piece")
        }
    }

    pub fn get_long_name(&self) -> &'static str {
        match self {
            Pieces::K => "King",
            Pieces::Q => "Queen",
            Pieces::R => "Rook",
            Pieces::B => "Bishop",
            Pieces::N => "Night",
            Pieces::P => "Pawn",
            _ => panic!("_NPieces is not a valid piece")
        }
    }

    pub fn get_piece_from_encoding(encode: usize) -> Pieces {
        match encode {
            0 => Pieces::K,
            1 => Pieces::Q,
            2 => Pieces::R,
            3 => Pieces::B,
            4 => Pieces::N,
            5 => Pieces::P,
            _ => panic!("Invalid encoding {} used.", encode)
        }
    }

    pub fn as_array() -> [Pieces; N_PIECES] {[
            Pieces::K,
            Pieces::Q,
            Pieces::R,
            Pieces::B,
            Pieces::N,
            Pieces::P
        ]
    }

}






