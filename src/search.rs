use chess::{Board, ChessMove, Color, MoveGen, EMPTY};
use crate::consts::{ENDGAME_PIECE_FOR_GREATER_DEPTH, ENDGAME_SEARCH_DEPTH, FIRST_N_MOVES_TO_EXTEND, MAX_PIECE_FOR_ENDGAME, OPENING_FOR_USING_OPENING_BOOK, OPENING_SEARCH_DEPTH, SEARCH_DEPTH};
use crate::endgame_tablebase::EndGameTablebase;
use crate::eval::{count_all_pieces, eval, is_bad_king_move, is_terminal};
use crate::opening_book::OpeningBook;
use crate::search_move::SearchMove;
use crate::transposition_table::TranspostionTable;

fn quiescence_search(
    board: &Board,
    alpha: f32,
    beta: f32,
    maximizing_player: bool,
    depth: u8,
    transposition_table: &mut TranspostionTable
) -> SearchMove {
    if depth == 0 {
        return SearchMove::new(None, eval(board, MoveGen::new_legal(board).collect(), 0, false));
    }

    let mut alpha = alpha;
    let stand_pat = eval(board, MoveGen::new_legal(board).collect(), 0, false);

    if stand_pat >= beta {
        return SearchMove::new(None, beta);
    }

    if alpha < stand_pat {
        alpha = stand_pat;
    }

    let mut best_move = SearchMove::new(None, stand_pat);
    let moves_ordered = order_moves_by_evaluation(board, MoveGen::new_legal(&board).collect(), maximizing_player, 0);

    for mv in moves_ordered 
    {
        if is_bad_king_move(board, &mv, 0) || board.piece_on(mv.get_dest()).is_none() || board.checkers() == &EMPTY 
        {
            continue;
        }

        let next_board = board.make_move_new(mv);
        let eval_mv = if let Some(val) = transposition_table.get_position(&next_board) {
            val.clone()
        } 
        else 
        {
            quiescence_search(&next_board, -beta, -alpha, !maximizing_player, depth - 1, transposition_table)
        };

        if eval_mv.eval() > alpha 
        {
            alpha = eval_mv.eval();
            best_move = SearchMove::new(Some(mv), eval_mv.eval());
        }

        if alpha >= beta {
            break;
        }
    }

    best_move
}


pub fn alpha_beta(
    board: &Board,
    depth: u8,
    mut alpha: f32,
    mut beta: f32,
    maximizing_player: bool,
    plies: i32,
    extended: bool,
    de_extended: bool,
    transposition_table: &mut TranspostionTable
) -> SearchMove 
{
    let movegen: Vec<ChessMove> = MoveGen::new_legal(board).collect();

    if depth == 0 || is_terminal(board.status()) 
    {
        let mov = if plies > 7  
        {
            quiescence_search(board, alpha, beta, maximizing_player, 2, transposition_table)
        }
        else 
        {
            SearchMove::new(None, eval(board, MoveGen::new_legal(&board).collect(), plies, false))
        };

        return mov;
    }

    let mut best_move = if maximizing_player 
    {
        SearchMove::new(None, f32::NEG_INFINITY)
    } 
    else 
    {
        SearchMove::new(None, f32::INFINITY)
    };

    let moves_ordered = order_moves_by_evaluation(board, movegen, maximizing_player, plies);
    let mut i = 0;

    for mv in moves_ordered 
    {
        if is_bad_king_move(board, &mv, plies) 
        {
            continue;
        }

        let next_board = board.make_move_new(mv);
        let eval_mv;

        if let Some(val) = transposition_table.get_position(&next_board) 
        {
            eval_mv = val.clone()
        }
        else 
        {
            if i < FIRST_N_MOVES_TO_EXTEND && !extended
            {
                eval_mv = alpha_beta(
                    &next_board,
                    depth,
                    alpha,
                    beta,
                    !maximizing_player,
                    plies + 1,
                    true,
                    de_extended,
                    transposition_table
                );
            }
            else if !de_extended
            {
                eval_mv = alpha_beta
                (
                    &next_board,
                    depth - 2,
                    alpha,
                    beta,
                    !maximizing_player,
                    plies + 1,
                    extended,
                    true,
                    transposition_table
                ); 
            }
            else 
            {
                eval_mv = alpha_beta
                (
                    &next_board,
                    depth - 1,
                    alpha,
                    beta,
                    !maximizing_player,
                    plies + 1,
                    extended,
                    de_extended,
                    transposition_table
                ); 
            }
        }

        transposition_table.add_position(&next_board, &eval_mv);

        if best_move.mov().is_none() {
            best_move = SearchMove::new(Some(mv), eval_mv.eval());
        }

        if maximizing_player {
            if eval_mv.eval() > best_move.eval() {
                best_move = SearchMove::new(Some(mv), eval_mv.eval());
            }
            alpha = alpha.max(best_move.eval());
        } else {
            if eval_mv.eval() < best_move.eval() {
                best_move = SearchMove::new(Some(mv), eval_mv.eval());
            }
            beta = beta.min(best_move.eval());
        }


        if beta <= alpha {
            break;
        }
        i += 1;
    }

    best_move
}

fn order_moves_by_evaluation(board: &Board, movegen: Vec<ChessMove>, maximizing_player: bool, plies: i32) -> Vec<ChessMove> {
    let mut move_evaluations: Vec<(ChessMove, f32)> = movegen.into_iter().map(|mv: ChessMove| {
        let next_board = board.make_move_new(mv);
        let evaluation = eval(&next_board, MoveGen::new_legal(&next_board).collect(), plies + 1, false);
        (mv, evaluation)
    }).collect();

    if maximizing_player {
        move_evaluations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    } else {
        move_evaluations.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    }

    move_evaluations.into_iter().map(|(mv, _)| mv).collect()
}

pub fn engine(board: &Board, plies: i32) -> ChessMove {
    let mut transposition_table = TranspostionTable::new();
    let mut optimal_move = None;

    let mut opening_book = OpeningBook::new();
    let mut table_base = EndGameTablebase::new();

    let pieces = count_all_pieces(board);

    if pieces <= MAX_PIECE_FOR_ENDGAME
    {
        optimal_move = table_base.get_move(board);
    }

    if plies <= OPENING_FOR_USING_OPENING_BOOK 
    {
        optimal_move = opening_book.get_move(board);
    }

    if let Some(mov) = optimal_move 
    {
        mov
    }
    else 
    {
        let search_move = alpha_beta(
            board,
            if plies <= OPENING_FOR_USING_OPENING_BOOK 
            {
                OPENING_SEARCH_DEPTH
            }
            else if pieces <= ENDGAME_PIECE_FOR_GREATER_DEPTH
            {   
                ENDGAME_SEARCH_DEPTH 
            }
            else 
            {
                SEARCH_DEPTH
            },
            f32::NEG_INFINITY,
            f32::INFINITY,
            board.side_to_move() == Color::White,
            plies,
            false,
            false,
            &mut transposition_table
        );
        
        search_move.mov().unwrap()
    }

}
