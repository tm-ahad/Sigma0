use chess::Piece;

pub fn promo_piece_to_char(s: Piece) -> char
{
    match s
    {
        Piece::Bishop => 'b',
        Piece::Knight => 'n',
        Piece::Queen => 'q',
        Piece::Rook => 'r',
        _ => todo!()
    }
}

