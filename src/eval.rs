use std::collections::HashMap;

use chess::{Board, BoardStatus, ChessMove, Color, File, MoveGen, Piece, Rank, Square, ALL_SQUARES, EMPTY};
use chess::BoardStatus::{Checkmate, Stalemate};
use chess::Color::{White, Black};
use crate::consts::{CONTROLLING_SQUARE, CONTROLLING_SQUARE_OPENING, DEFENDING_PIECE, DEFENDING_PIECE_OPENING, ENDGAME_KING_DISTANCE, GOOD_KNIGHT, KING_MOVED_NOT_ENDGAME, MAX_PIECE_FOR_ENDGAME, MIDDLEGAME_FOR_QUEEN_SAFETY, OPENING_FOR_DIFF_EVAL, OPENING_FOR_KING_SAFETY, OPENING_FOR_PIECE_SAFETY, OPENING_QUEEN_SAFETY, PAWN_ON_SAFE_FILE_DISADVANTAGE, PAWN_SHIELD_SCORE, ROOK_ON_7TH_RANK_BONUS};
use crate::material::material;
use crate::piece_table::{king_square_value, pawn_square_value};

pub fn is_terminal(status: BoardStatus) -> bool 
{
    status != BoardStatus::Ongoing
}

pub fn white_score(advantage: f32, turn: Color) -> f32 
{
    if turn == White { advantage } else { -advantage }
}

pub fn count_all_pieces(board: &Board) -> u8 
{
    let mut res = 0;

    for sq in ALL_SQUARES 
    {
        if board.piece_on(sq).is_some() 
        {
            res += 1;
        }
    }

    res
}

fn invert_color(color: Color) -> Color
{
    match color 
    {
        Color::White => Color::Black,
        Color::Black => Color::White
    }
}

fn is_valid_file_rank(file: i8, rank: i8) -> bool
{
    (0..=7).contains(&file) && (0..=7).contains(&rank)
}

fn is_piece_defended(board: &Board, sq: Square, color: Color) -> bool
{
    let queen_directions: Vec<(i8, i8)> = vec!
    [
        (-1, 1),
        (1, 1),
        (1, -1),
        (-1, -1),
        (1, 0),
        (0, 1),
        (0, -1),
        (-1, 0)
    ];

    let knight_directions: Vec<(i8, i8)> = vec!
    [
        (-2, 1),
        (-2, -1),
        (1, -2),
        (-1, -2),
        (2, 1),
        (2, -1),
        (1, 2),
        (-1, 2)
    ];

    let mut file = sq.get_file().to_index() as i8;
    let mut rank = sq.get_rank().to_index() as i8;

    for directions in queen_directions 
    {
        while is_valid_file_rank(file, rank) 
        {
            let square = Square::make_square
            (
                Rank::from_index(file as usize), 
                File::from_index(rank as usize)
            );

            let piece = board.piece_on(square);
            let piece_color = board.color_on(square);

            if let Some(piece) = piece 
            {
                if piece_color == Some(color) 
                {
                    let multiple = (directions.0*directions.1).abs();
                    let distance = distance(sq, square);

                    if piece == Piece::Pawn && multiple == 1 && distance == 0 
                    {
                        if color == White && directions.1 == 1 
                        {
                            return true;
                        }
                        else if color == Black && directions.0 == -1 
                        {
                            return true;
                        }
                    } 
                    else if piece == Piece::King && distance == 0 
                    {
                        return true;
                    }
                    else if piece == Piece::Bishop && multiple == 1 
                    {
                        return true;
                    }
                    else if piece == Piece::Rook && multiple == 0 
                    {
                        return true;
                    }
                    else if piece == Piece::Queen 
                    {
                        return true;
                    }
                }
            }

            file += directions.0;
            rank += directions.1;
        }
    }

    file = sq.get_file().to_index() as i8;
    rank = sq.get_rank().to_index() as i8;

    for directions in knight_directions 
    {
        let file = file + directions.0;
        let rank = rank + directions.1;

        let square = Square::make_square
        (
            Rank::from_index(file as usize), 
            File::from_index(rank as usize)
        );

        let piece_color = board.color_on(square);
        let piece = board.piece_on(square);

        if piece_color == Some(color) && piece == Some(Piece::Knight)
        {
            return true;
        }
    }

    false
}

fn distance(sq: Square, sq2: Square) -> u8 
{
    let sqrf = square_index(sq).0.abs_diff(square_index(sq2).0);
    let sqfd = square_index(sq).0.abs_diff(square_index(sq2).0);

    sqrf.max(sqfd) - 1
}

pub fn is_bad_king_move(board: &Board, mov: &ChessMove, plies: i32) -> bool
{
    let is_opening_for_king_safety = plies < OPENING_FOR_KING_SAFETY;
    let source = mov.get_source();
    let dest = mov.get_dest();

    let is_castling = dest.to_int().abs_diff(source.to_int()) == 2;
    let is_capturing = board.piece_on(dest).is_some();
    let is_check = board.checkers() != &EMPTY;

    is_opening_for_king_safety && !is_castling && !is_capturing && !is_check &&
    (
        mov.get_source() == board.king_square(Color::White) ||
        mov.get_source() == board.king_square(Color::Black)
    )
}

pub fn eval(
    board: &Board, 
    legal_moves: Vec<ChessMove>, 
    plies: i32,
    _log: bool
) -> f32 
{
    let pieces = count_all_pieces(board);
    let mut pawn_on_files: u8 = 0;

    if board.status() == Checkmate 
    {
        return match board.side_to_move() 
        {
            White => f32::NEG_INFINITY,
            Black => f32::INFINITY,
        };
    } 
    else if board.status() == Stalemate || pieces == 2
    {
        return 0.0;
    }

    let mut score_for_white = 0.0;

    let is_endgame = pieces <= MAX_PIECE_FOR_ENDGAME;
    let is_opening_for_piece_safety = plies < OPENING_FOR_PIECE_SAFETY;
    let is_opening_for_king_safety = plies < OPENING_FOR_KING_SAFETY;

    let mut captured = HashMap::new();
    let mut max_captured = 0.0;

    let mut pawn_shield = None;

    if is_opening_for_king_safety 
    {
        let colors = [Color::White, Color::Black];
        for _color in colors 
        {
            let white_king = square_index(board.king_square(Color::White));
            let rank = white_king.0 as i8;
            let file = white_king.1 as i8;

            let rank_change: i8 = if _color == Color::White {1} else {-1};
                
            let sq1 = Square::make_square(Rank::from_index((rank+rank_change) as usize), File::from_index((file-1) as usize));
            let sq2 = Square::make_square(Rank::from_index((rank+rank_change) as usize), File::from_index(file as usize));
            let sq3 = Square::make_square(Rank::from_index((rank+rank_change) as usize), File::from_index((file+1) as usize));

            if board.piece_on(sq1).is_some()
                && board.piece_on(sq2).is_some()
                && board.piece_on(sq3).is_some()
            {
                score_for_white += white_score(PAWN_SHIELD_SCORE, _color);
                pawn_shield = Some(_color)
            }
        }
    }

    if _log{println!("Eval step 1: {}", score_for_white)};

    for square in ALL_SQUARES 
    {
        if let Some(piece) = board.piece_on(square) 
        {
            let color = board.color_on(square).unwrap();
            let (rank, file) = square_index(square);

            if piece == Piece::Rook && color == White && rank == 6 
            {
                score_for_white += ROOK_ON_7TH_RANK_BONUS
            }

            if is_endgame && piece == Piece::Pawn 
            {
                let file_index= square.get_file().to_index();
                if ((pawn_on_files << file_index) & 1_u8) != 0
                {
                    score_for_white -= white_score(PAWN_ON_SAFE_FILE_DISADVANTAGE, color)
                }

                pawn_on_files |= 1 << square.get_file().to_index();

                let king = board.king_square(color);
                let enemy_king = board.king_square(invert_color(color));

                score_for_white += distance(king, square) as f32 / 1.6;
                score_for_white -= distance(enemy_king, square) as f32 / 1.6;
            }

            if piece == Piece::Rook && color == Black && rank == 1 
            {
                score_for_white -= ROOK_ON_7TH_RANK_BONUS
            }

            if piece == Piece::Knight && is_opening_for_piece_safety
            {
                if color == White && (square == Square::F3 || square == Square::C3) 
                {
                    score_for_white += GOOD_KNIGHT
                }

                if color == Black && (square == Square::C6 || square == Square::F6) 
                {
                    score_for_white -= GOOD_KNIGHT
                }
            }

            if piece == Piece::King && is_opening_for_king_safety
            {
                if color == Black && rank != 7
                {
                    score_for_white += KING_MOVED_NOT_ENDGAME/plies as f32;
                }

                if color == White && rank != 0
                {
                    score_for_white -= KING_MOVED_NOT_ENDGAME/plies as f32
                }
            }

            if piece == Piece::Queen && plies <= MIDDLEGAME_FOR_QUEEN_SAFETY
            {
                let white_range = if plies <= OPENING_FOR_PIECE_SAFETY 
                {
                    2..=7
                }
                else if plies <= MIDDLEGAME_FOR_QUEEN_SAFETY 
                {
                    4..=7
                }
                else 
                {
                    todo!()
                };

                let black_range = if plies <= OPENING_FOR_PIECE_SAFETY 
                {
                    0..=5
                }
                else if plies <= MIDDLEGAME_FOR_QUEEN_SAFETY 
                {
                    0..=3
                }
                else 
                {
                    todo!()
                };

                if _log{println!("{} {} {} {:?} {}", piece == Piece::Queen, color == Color::White, white_range.contains(&rank), white_range, rank)};

                score_for_white += match (piece, color) 
                {
                    (Piece::Queen, White) if white_range.contains(&rank) => -OPENING_QUEEN_SAFETY,
                    (Piece::Queen, Black) if black_range.contains(&rank) => OPENING_QUEEN_SAFETY,
                    _ => 0.0
                }
            }
            
            let opposite_king = board.king_square(invert_color(color));

            if piece == Piece::Queen && !is_opening_for_king_safety && is_piece_defended(board, square, color) && pawn_shield != Some(invert_color(color))
            {
                let distance = distance(opposite_king, square);

                score_for_white += white_score(match distance 
                {
                    1 => 2.6,
                    2 => 0.8,
                    _ => 0.0
                }, color);
            }

            score_for_white += match piece 
            {
                Piece::Pawn => white_score(pawn_square_value(rank, file, color, is_endgame, board), color),
                Piece::King => white_score(king_square_value(rank, file, color, is_endgame, board), color),
                _ => white_score(material(Some(piece)), color),
            };
        }
    }

    if _log{println!("Eval step 2: {}", score_for_white)};

    for mov in &legal_moves
    {
        if is_bad_king_move(board, mov, plies) 
        {
            continue;
        }

        let source_piece = board.piece_on(mov.get_source());
        let source_color = board.color_on(mov.get_source());

        if let Some(dest_piece) = board.piece_on(mov.get_dest()) 
        {
            let dest_color = board.color_on(mov.get_dest()).unwrap();
            let dest_sq = mov.get_dest();
                    
            if let Some(val) = captured.get(&mov.get_source())
            {
                score_for_white += white_score(*val, source_color.unwrap());
            }
            else if !is_piece_defended(board, dest_sq, dest_color) 
            {
                let source_material = material(Some(dest_piece));

                captured.insert(mov.get_source(), DEFENDING_PIECE);
                score_for_white += white_score(0.0_f32.max(source_material-max_captured), source_color.unwrap());

                max_captured = max_captured.max(source_material);
            }
            else 
            {  
                let source_material = material(Some(dest_piece))-material(source_piece);

                captured.insert(mov.get_source(), DEFENDING_PIECE);
                score_for_white += white_score(0.0_f32.max(source_material-max_captured), source_color.unwrap());

                max_captured = max_captured.max(source_material);
            }
        }
        else 
        {
            score_for_white += white_score(CONTROLLING_SQUARE, board.side_to_move())
        }   
    }
    if _log{println!("Eval step 3: {}", score_for_white)};

    let flipped_board = board.null_move();

    if let Some(flipped_board) = flipped_board 
    {
        let legal_moves_other_side = MoveGen::new_legal(&flipped_board);

        for mov in legal_moves_other_side 
        {
            if board.piece_on(mov.get_dest()).is_none() 
            {
                score_for_white += white_score(
                    if plies <= OPENING_FOR_DIFF_EVAL 
                    {
                        DEFENDING_PIECE_OPENING
                    }
                    else 
                    {
                        DEFENDING_PIECE
                    }, 
                    flipped_board.side_to_move()
                )
            }
            else 
            {
                score_for_white += white_score(
                    if plies <= OPENING_FOR_DIFF_EVAL 
                    {
                        CONTROLLING_SQUARE_OPENING
                    }
                    else 
                    {
                        CONTROLLING_SQUARE
                    }, 
                    flipped_board.side_to_move()
                )
            }
        }
    }

    if _log{println!("Eval step 4: {}", score_for_white)};

    if is_endgame 
    {
        let wk = board.king_square(White);
        let bk = board.king_square(Black);

        let diff = distance(wk, bk) as f32;

        score_for_white += diff*score_for_white*ENDGAME_KING_DISTANCE
    }

    if _log{println!("Eval step 5: {}", score_for_white)};

    score_for_white
}

fn square_index(square: Square) -> (u8, u8) 
{
    let idx = square.to_index();
    ((idx / 8) as u8, (idx % 8) as u8)
}
