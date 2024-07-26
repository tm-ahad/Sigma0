use std::str::FromStr;
use std::io::{self, BufRead, Write};
use chess::{Board, BoardStatus, ChessMove};
use crate::move_string_conversion::move_to_string;
use crate::search::engine;

pub fn start_uci()
{
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut board = Board::default();
    let mut plies = 0;

    loop
    {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "quit"
        {
            break;
        }
        else if input.starts_with("uci")
        {
            writeln!(stdout, "id name oogway").expect("Failed to write response");
            writeln!(stdout, "id author T.M Ahad").expect("Failed to write response");
            writeln!(stdout, "uciok").expect("Failed to write response");
        }
        else if input == "isready"
        {
            writeln!(stdout, "readyok").expect("Failed to write response");
        }
        else if input.starts_with("ucinewgame")
        {
            board = Board::default();
        }
        else if input.starts_with("position")
        {
            board = Board::default();
            
            if input.contains("startpos")
            {
                if let Some(moves_index) = input.find("moves")
                {
                    let moves_len = "moves".len();
                    let moves: Vec<&str> = input[moves_index + moves_len..]
                        .split_whitespace()
                        .collect();

                    plies = moves.len() as i32;
                    for mv_str in moves
                    {
                        let mv = ChessMove::from_str(mv_str)
                            .unwrap_or_else(|_| panic!("Invalid move"));

                        board = board.make_move_new(mv);
                    }
                }
            } 
            else if let Some(fen_index) = input.find("fen")
            {
                let fen = &input[fen_index + 4..].trim();
                board = Board::from_str(fen).unwrap();
            }
        }
        else if input.starts_with("go")
        {
            if board.status() != BoardStatus::Ongoing
            {
                _ = writeln!(stdout, "bestmove 0000");
            }

            let best_move = engine(&board, plies);
            writeln!(stdout, "bestmove {}", move_to_string(best_move))
                .expect("Failed to write response");
        }
    }
}
