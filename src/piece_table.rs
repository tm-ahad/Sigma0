use chess::{Board, Color, Piece};
use crate::material::material;

pub const KING_SQUARE_TABLE: [f32; 64] = 
[
    -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
    -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
    -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5,
    -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5,
    -0.5, -0.5, -0.5, -1.0, -1.0, -0.5, -0.5, -0.5,
    -0.5, -0.5, -0.5, -1.0, -1.0, -0.5, -0.5, -0.5,
    -0.3, -0.3, -0.3, -0.3, -0.3, -0.3, -0.3, -0.3,
    0.0, 0.5, -0.1, 0.0, -0.1, 0.4, 0.0, 0.0,
];

pub const KING_SQUARE_TABLE_ENDGAME: [f32; 64] = 
[
    -0.9, -0.9, -0.9, -0.9, -0.9, -0.9, -0.9, -0.9,
    -0.9,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -0.9,
    -0.9,  0.9,  0.9,  0.9,  0.9,  0.9,  0.0, -0.9,
    -0.9,  0.9,  0.9,  1.0,  1.0,  0.9,  0.0, -0.9,
    -0.9,  0.9,  0.9,  1.0,  1.0,  0.9,  0.0, -0.9,
    -0.9,  0.9,  0.9,  0.9,  0.9,  0.9,  0.0, -0.9,
    -0.9,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, 0.0,
    -0.9, -1.0, -1.0, -1.0, -1.0, -1.0, -0.9, -0.9,
];

pub const PAWN_SQUARE_TABLE: [f32; 64] = 
[
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    0.1, 0.1, 0.1, 0.3, 0.3, 0.1, 0.1, 0.1,
    -0.1, -0.1, 0.1, 0.2, 0.2, 0.1, -0.1, -0.1,
    -0.1, -0.1, 0.1, 0.2, 0.2, 0.1, -0.1, -0.1,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
];

pub const PAWN_SQUARE_TABLE_ENDGAME: [f32; 64] = 
[
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4,
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3,
    0.2, 0.2, 0.2, 0.2, 0.2, 0.1, 0.2, 0.2,
    0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
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

pub fn king_square_value(rank: u8, file: u8, color: Color, is_endgame: bool) -> f32 
{
    let idx = (rank * 8 + file) as usize;
    let sq_table = if is_endgame
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
