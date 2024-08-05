use chess::Piece;

pub fn material(piece: Option<Piece>) -> f32
{
    match piece
    {
        Some(Piece::Pawn) => 1.0,
        Some(Piece::Knight) => 3.00,
        Some(Piece::Bishop) => 3.35,
        Some(Piece::Rook) => 5.73,
        Some(Piece::Queen) => 9.5,
        Some(Piece::King) => 2.26,
        None => 0.0,
    }
}
