use chess::{Board, Color, Piece};
use crate::material::material;

pub const KING_SQUARE_TABLE: [f32; 64] = 
[
    -0.65, 0.23, 0.16, -0.15, -0.56, -0.34, 0.02, 0.13,
    0.29, -0.01, -0.20, -0.07, -0.08, -0.04, -0.38, -0.29,
    -0.09, 0.24, 0.02, -0.16, -0.20, 0.06, 0.22, -0.22,
    -0.17, -0.20, -0.12, -0.27, -0.30, -0.25, -0.14, -0.36,
    -0.49, -0.01, -0.27, -0.39, -0.46, -0.44, -0.33, -0.51,
    -0.14, -0.14, -0.22, -0.46, -0.44, -0.30, -0.15, -0.27,
    0.01, 0.07, -0.08, -0.64, -0.43, -0.16, 0.09, 0.08,
    -0.15, 0.36, 0.12, -0.54, 0.08, -0.28, 0.24, 0.14
];

pub const KING_SQUARE_TABLE_ENDGAME: [f32; 64] = 
[
    -0.74, -0.35, -0.18, -0.18, -0.11,  0.15,  0.04, -0.17,
    -0.12,  0.17,  0.14,  0.17,  0.17,  0.38,  0.23,  0.11,
     0.10,  0.17,  0.23,  0.15,  0.20,  0.45,  0.44,  0.13,
    -0.08,  0.22,  0.24,  0.27,  0.26,  0.33,  0.26,  0.03,
    -0.18, -0.04,  0.21,  0.24,  0.27,  0.23,  0.09, -0.11,
    -0.19, -0.03,  0.11,  0.21,  0.23,  0.16,  0.07, -0.09,
    -0.27, -0.11,  0.04,  0.13,  0.14,  0.04, -0.05, -0.17,
    -0.53, -0.34, -0.21, -0.11, -0.28, -0.14, -0.24, -0.43
];

pub const PAWN_SQUARE_TABLE: [f32; 64] = 
[
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.98, 1.34, 0.61, 0.95, 0.68, 1.26, 0.34, -0.11,
    -0.06, 0.07, 0.26, 0.31, 0.65, 0.56, 0.25, -0.20,
    -0.14, 0.13, 0.06, 0.21, 0.23, 0.12, 0.17, -0.23,
    -0.27, -0.02, -0.05, 0.12, 0.17, 0.06, 0.10, -0.25,
    -0.26, -0.04, -0.04, -0.10, 0.03, 0.03, 0.33, -0.12,
    -0.35, -0.01, -0.20, -0.23, -0.15, 0.24, 0.38, -0.22,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
];

pub const PAWN_SQUARE_TABLE_ENDGAME: [f32; 64] = 
[
    0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00,
    1.78, 1.73, 1.58, 1.34, 1.47, 1.32, 1.65, 1.87,
    0.94, 1.00, 0.85, 0.67, 0.56, 0.53, 0.82, 0.84,
    0.32, 0.24, 0.13, 0.05, -0.02, 0.04, 0.17, 0.17,
    0.13, 0.09, -0.03, -0.07, -0.07, -0.08, 0.03, -0.01,
    0.04, 0.07, -0.06, 0.01, 0.00, -0.05, -0.01, -0.08,
    0.13, 0.08, 0.08, 0.10, 0.13, 0.00, 0.02, -0.07,
    0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00
];

fn flip_index(index: usize) -> usize 
{
    63 - index
}

pub fn no_castle_rights(board: &Board, color: Color) -> bool
{
    let rights = board.castle_rights(color);
    !(rights.has_kingside() || rights.has_queenside())
}

pub fn pawn_square_value(rank: u8, file: u8, color: Color, is_endgame: bool, board: &Board) -> f32 
{
    let idx = (rank * 8 + file) as usize;
    let sq_table = if is_endgame || no_castle_rights(board, color)
    {
        PAWN_SQUARE_TABLE_ENDGAME
    }
    else 
    {
        PAWN_SQUARE_TABLE
    };

    match color 
    {
        Color::White => sq_table[flip_index(idx)] + material(Some(Piece::Pawn)),
        Color::Black => sq_table[idx] + material(Some(Piece::Pawn)),
    }
}

pub fn king_square_value(rank: u8, file: u8, color: Color, is_endgame: bool, board: &Board) -> f32 
{
    let idx = (rank * 8 + file) as usize;
    let sq_table = if is_endgame || no_castle_rights(board, board.side_to_move())
    {
        KING_SQUARE_TABLE_ENDGAME
    }
    else 
    {
        KING_SQUARE_TABLE
    };

    match color 
    {
        Color::White => sq_table[flip_index(idx)] + material(Some(Piece::King)),
        Color::Black => sq_table[idx] + material(Some(Piece::King)),
    }
}
