use crate::board_utils::constants::general;
use crate::board_utils::constants::{ranks, squares::{*}};

pub const EMPTY : [[u64; general::N_PIECES]; 2] = [
    [ // White
        0, // king
        0, // queen
        0, // rook
        0, // biship
        0, // knight
        0  // pawns
    ],
    [ // Black
        0, // king
        0, // queen
        0, // rook
        0, // biship
        0, // knight
        0  // pawns
    ]
];

pub const DEFAULT : [[u64; general::N_PIECES]; 2] = [
    [ // White
        A5,      // king
        A4,      // queen
        A1 | A8, // rook
        A3 | A6, // biship
        A2 | A7, // knight
        ranks::B // pawns
    ],
    [ // Black
        H5,      // king
        H4,      // queen
        H1 | H8, // rook
        H3 | H6, // biship
        H2 | H7, // knight
        ranks::G // pawns
    ]
];

pub const TEST_OVERLAP : [[u64; general::N_PIECES]; 2] = [
    [ // White
        A5,      // king
        A4,      // queen
        A3, // rook
        A2 , // biship
        A1, // knight
        ranks::H // pawns
    ],
    [ // Black
        0,      // king
        0,      // queen
        0, // rook
        0 , // biship
        A1, // knight
        ranks::H // pawns
]
];



