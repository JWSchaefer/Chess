#[derive(Copy, Clone)]
pub enum Piece {
    K,
    Q,
    R,
    B,
    N,
    P,
    NullPiece
}


pub const N_PIECES: usize = Piece::NullPiece as usize;

pub const AS_ARRAY : [Piece; N_PIECES] = [Piece::K, Piece::Q, Piece::R, Piece::B, Piece::N, Piece::P];

impl Piece {

    pub fn get_encoding(&self) -> usize {
        match self {
            Piece::NullPiece => panic!("NullPiece is not a valid piece"),
            other => *other as usize,
        }
    }


    pub fn get_short_name(&self) -> char {
        match self {
            Piece::K => 'K',
            Piece::Q => 'Q',
            Piece::R => 'R',
            Piece::B => 'B',
            Piece::N => 'N',
            Piece::P => 'P',
            Piece::NullPiece => ' '
        }
    }

    pub fn get_long_name(&self) -> &'static str {
        match self {
            Piece::K => "King",
            Piece::Q => "Queen",
            Piece::R => "Rook",
            Piece::B => "Bishop",
            Piece::N => "Night",
            Piece::P => "Pawn",
            Piece::NullPiece => "NONE"
        }
    }

    pub fn get_piece_from_encoding(encode: usize) -> Piece {
        match encode {
            0 => Piece::K,
            1 => Piece::Q,
            2 => Piece::R,
            3 => Piece::B,
            4 => Piece::N,
            5 => Piece::P,
            _ => panic!("Invalid encoding {} used.", encode)
        }
    }

    pub fn get_piece_from_transpose_column(column : u64) -> Piece {

        match column {
            0 => Piece::NullPiece,
            _ => AS_ARRAY[column.trailing_zeros() as usize % N_PIECES]
        }


    }


}
    

