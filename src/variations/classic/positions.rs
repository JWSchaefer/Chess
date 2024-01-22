
use crate::variations::classic::{ranks, squares::*, pieces};

pub const EMPTY : [[u64; pieces::N_PIECES]; 2] = [
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

pub const DEFAULT : [[u64; pieces::N_PIECES]; 2] = [
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

pub const TEST_OVERLAP : [[u64; pieces::N_PIECES]; 2] = [
    [ // White
        D4, // king
        0, // queen
        0, // rook
        0, // biship
        0, // knight
        0 // pawns
    ],
    [ // Black
        0,  // king
        0,  // queen
        0,  // rook
        0,  // biship
        D4, // knight
        0   // pawns
]
];



