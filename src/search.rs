
use std::sync::MutexGuard;

use chess::{Board, ChessMove, Color, MoveGen, EMPTY};
use crate::consts::{ENDGAME_PIECE_FOR_GREATER_DEPTH, ENDGAME_SEARCH_DEPTH, MAX_PIECE_FOR_ENDGAME, OPENING_FOR_DIFF_EVAL, OPENING_SEARCH_DEPTH, SEARCH_DEPTH, USE_QUIESCENSE_SEARCH_AFTER_NPLIES};
use crate::endgame_tablebase::EndGameTablebase;
use crate::eval::{count_all_pieces, eval, is_bad_king_move, is_terminal};
use crate::move_database::MoveDatabase;
use crate::search_move::SearchMove;
use crate::transposition_table::TranspostionTable;

fn quiescence_search(
    board: &Board,
    mut alpha: f32,
    mut beta: f32,
    maximizing_player: bool,
    depth: u8,
    plies: i32,
    transposition_table: &mut TranspostionTable
) -> SearchMove {
    if depth == 0 || is_terminal(board.status()) 
    {
        return SearchMove::new(None, eval(board, MoveGen::new_legal(board).collect(), plies, false));
    }

    let _eval = eval(board, MoveGen::new_legal(board).collect(), plies, false);
    let mut best_move = SearchMove::new(None, _eval);

    let moves_ordered = order_moves_by_evaluation(board, MoveGen::new_legal(board).collect(), maximizing_player, plies);

    for mv in moves_ordered {
        if board.piece_on(mv.get_dest()).is_none() || board.checkers() == &EMPTY 
        {
            continue;
        }

        let next_board = board.make_move_new(mv);
        let eval_mv;

        if let Some(val) = transposition_table.get_position(&next_board) 
        {
            eval_mv = val.clone();
        } else {
            eval_mv = quiescence_search(
                &next_board,
                alpha,
                beta,
                !maximizing_player,
                depth - 1,
                plies + 1,
                transposition_table
            );

            transposition_table.add_position(&next_board, &eval_mv);
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
    let board_eval = eval(board, MoveGen::new_legal(board).collect(), plies, false);

    if depth == 0 || is_terminal(board.status()) 
    {
        let mov = if plies > USE_QUIESCENSE_SEARCH_AFTER_NPLIES 
        {
            quiescence_search(board, alpha, beta, maximizing_player, 4, plies, transposition_table)
        } 
        else 
        {
            SearchMove::new(None, eval(board, MoveGen::new_legal(board).collect(), plies, false))
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

    let moves_ordered = order_moves_by_evaluation(board, MoveGen::new_legal(board).collect(), maximizing_player, plies);
    let pieces = count_all_pieces(&board);

    for mv in moves_ordered 
    {
        if is_bad_king_move(&board, &mv, pieces)
        {
            continue
        }

        let next_board = board.make_move_new(mv);
        let mut eval_mv = None;

        if let Some(val) = transposition_table.get_position(&next_board) 
        {
            eval_mv = Some(val.clone());
        } 
        else 
        {
            let curr_eval = if let Some(mv) = eval_mv
            {
                mv.eval()
            }
            else 
            {
                eval(&next_board, MoveGen::new_legal(&next_board).collect(), plies, false)
            };

            let eval_diff = (board_eval - curr_eval).abs();
            if eval_diff > 1.5 && !extended
            {
                eval_mv = Some(alpha_beta(
                    &next_board,
                    depth,
                    alpha,
                    beta,
                    !maximizing_player,
                    plies + 1,
                    true,
                    de_extended,
                    transposition_table
                ));
            } 
            else if eval_diff < 1.0 && !de_extended && depth > 1
            {
                eval_mv = Some(alpha_beta(
                    &next_board,
                    depth - 2,
                    alpha,
                    beta,
                    !maximizing_player,
                    plies + 1,
                    extended,
                    true,
                    transposition_table
                ));
            } 
            else 
            {
                eval_mv = Some(alpha_beta(
                    &next_board,
                    depth - 1,
                    alpha,
                    beta,
                    !maximizing_player,
                    plies + 1,
                    extended,
                    de_extended,
                    transposition_table
                ));
            }

            let eval_mv = &eval_mv.clone().unwrap();

            transposition_table.add_position(&next_board, eval_mv);
        }
        
        let evaluation = eval_mv.clone().unwrap().eval();

        if best_move.mov().is_none() 
        {
            best_move = SearchMove::new(Some(mv), evaluation);
        }

        if maximizing_player 
        {
            if evaluation > best_move.eval() 
            {
                best_move = SearchMove::new(Some(mv), evaluation);
            }
            alpha = alpha.max(best_move.eval());
        } 
        else 
        {
            if evaluation < best_move.eval() 
            {
                best_move = SearchMove::new(Some(mv), evaluation);
            }
            beta = beta.min(best_move.eval());
        }

        if beta <= alpha 
        {
            break;
        }
    }

    best_move
}

fn order_moves_by_evaluation(board: &Board, movegen: Vec<ChessMove>, maximizing_player: bool, plies: i32) -> Vec<ChessMove> 
{
    let mut move_evaluations: Vec<(ChessMove, f32)> = movegen.into_iter().map(|mv: ChessMove| 
        {
        let next_board = board.make_move_new(mv);
        let evaluation = eval(&next_board, MoveGen::new_legal(&next_board).collect(), plies + 1, false);
        (mv, evaluation)
    }).collect();

    if maximizing_player 
    {
        move_evaluations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    } 
    else 
    {
        move_evaluations.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    }

    move_evaluations.into_iter().map(|(mv, _)| mv).collect()
}

pub fn engine(board: &Board, plies: i32, mut db: MutexGuard<MoveDatabase>) -> ChessMove 
{
    let pieces = count_all_pieces(board);

    let mut transposition_table = TranspostionTable::new();
    let mut endgame_tablebase = EndGameTablebase::new();

    let optimal_move = if pieces <= MAX_PIECE_FOR_ENDGAME
    {
        endgame_tablebase.get_move(board)
    }
    else if plies <= OPENING_FOR_DIFF_EVAL 
    {
        db.get_move(&board)
    }
    else 
    {
        None
    };

    if let Some(mov) = optimal_move 
    {
        mov
    } 
    else 
    {
        let search_move = alpha_beta
        (
            board,
            if plies <= OPENING_FOR_DIFF_EVAL 
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

        // let new_board = board.make_move_new(search_move.mov().unwrap());
        // println!("Eval: {}", eval(board, MoveGen::new_legal(&new_board).collect(), plies, true));

        search_move.mov().unwrap()
    }
}
 