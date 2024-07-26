use chess::{ChessMove, Square};
use crate::promotion_piece::promo_piece_to_char;

pub fn square_to_string(sq: Square) -> String
{
    let sqint = sq.to_int();

    let file = sqint % 8;
    let rank = (sqint/8)+1;

    let rank_ch = (rank + b'0') as char;
    let file_ch = (file + b'a') as char;

    format!("{file_ch}{rank_ch}")
}

pub fn move_to_string(mov: ChessMove) -> String
{
    format!
    (
        "{}{}{}",
        square_to_string(mov.get_source()),
        square_to_string(mov.get_dest()),
        if let Some(piece) = mov.get_promotion()
        {
            String::from(promo_piece_to_char(piece))
        }
        else
        {
            String::new()
        }
    )
}
