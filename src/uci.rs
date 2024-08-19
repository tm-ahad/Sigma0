use std::collections::HashMap;
use std::str::FromStr;
use std::io::{self, BufRead, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use chess::{Board, BoardStatus, ChessMove};
use crate::consts::OPENING_BOOK_MAX_PLIES;
use crate::move_database::MoveDatabase;
use crate::move_string_conversion::move_to_string;
use crate::search::engine;

#[derive(Debug, Clone)]
pub struct ExtendedOption {
    pub option_type: String,
    pub default: Option<String>,
    pub min: Option<i32>,
    pub max: Option<i32>,
    pub value: Option<String>,
}

impl ExtendedOption {
    pub fn new(option_type: &str, default: Option<&str>, min: Option<i32>, max: Option<i32>) -> Self {
        ExtendedOption {
            option_type: option_type.to_string(),
            default: default.map(|s| s.to_string()),
            min,
            max,
            value: None,
        }
    }

    pub fn set_value(&mut self, value: &str) {
        if let Some(min) = self.min {
            if let Some(max) = self.max {
                let value_int = value.parse::<i32>().unwrap_or(min - 1);
                if value_int < min || value_int > max {
                    panic!("Value {} is out of bounds ({:?} to {:?})", value_int, min, max);
                }
            }
        }
        self.value = Some(value.to_string());
    }
}

pub fn start_uci() {
    let cloned_db = Arc::new(Mutex::new(MoveDatabase::load()));

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut board = Board::default();
    let mut plies = 0;

    let mut options: HashMap<String, ExtendedOption> = HashMap::new();

    // Initialize options with min, max, and default values
    options.insert("Hash".to_string(), ExtendedOption::new("spin", Some("64"), Some(1), Some(2048)));
    options.insert("Threads".to_string(), ExtendedOption::new("spin", Some("1"), Some(1), Some(16)));
    options.insert("Move Overhead".to_string(), ExtendedOption::new("spin", Some("2000"), Some(0), Some(10000)));

    loop {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "quit" {
            break;
        } else if input.starts_with("uci") {
            writeln!(stdout, "id name Sigma0").expect("Failed to write response");
            writeln!(stdout, "id author T.M Ahad").expect("Failed to write response");

            for (k, opt) in &options {
                let bind = String::new();

                let default_value = opt.default.as_ref().unwrap_or(&bind);
                let min_value = opt.min.unwrap_or(i32::MIN);
                let max_value = opt.max.unwrap_or(i32::MAX);
                
                writeln!(
                    stdout,
                    "option name {} type {} default {} min {} max {}",
                    k, opt.option_type, default_value, min_value, max_value
                ).expect("Failed to write response");
            }

            writeln!(stdout, "uciok").expect("Failed to write response");
        } else if input == "isready" {
            writeln!(stdout, "readyok").expect("Failed to write response");
        } else if input.starts_with("ucinewgame") {
            board = Board::default();
        } else if input.starts_with("position") {
            board = Board::default();
            
            if input.contains("startpos") {
                if let Some(moves_index) = input.find("moves") {
                    let moves_len = "moves".len();
                    let moves: Vec<&str> = input[moves_index + moves_len..]
                        .split_whitespace()
                        .collect();

                    plies = moves.len() as i32;
                    for mv_str in moves {
                        let mv = ChessMove::from_str(mv_str)
                            .unwrap_or_else(|_| panic!("Invalid move"));

                        board = board.make_move_new(mv);
                    }
                }
            } else if let Some(fen_index) = input.find("fen") {
                let fen = &input[fen_index + 4..].trim();
                board = Board::from_str(fen).unwrap();
            }
        } else if input.starts_with("go") {
            if board.status() != BoardStatus::Ongoing {
                _ = writeln!(stdout, "bestmove 0000");
            }

            let best_move = engine(&board, plies, cloned_db.lock().unwrap());
            
            writeln!(stdout, "bestmove {}", move_to_string(best_move))
                .expect("Failed to write response");

            if plies <= OPENING_BOOK_MAX_PLIES {
                let cloned_db = cloned_db.clone();
                let handle = thread::spawn(move || {
                    let mut db = cloned_db.lock().unwrap();
                    db.add_move(&board);
                });

                handle.join().unwrap();
            }
        } else if input.starts_with("setoption name") {
            if let Some((name, value)) = input[14..].split_once(" value ") {
                let name = name.trim();
                if let Some(option) = options.get_mut(name) {
                    option.set_value(value.trim());
                }
            }
        } else if input.starts_with("getoption name") {
            let name = input[14..].trim();
            if let Some(option) = options.get(name) {
                writeln!(
                    stdout,
                    "option name {} type {} value {} default {} min {} max {}",
                    name,
                    option.option_type,
                    option.value.as_ref().unwrap_or(&"".to_string()),
                    option.default.as_ref().unwrap_or(&"".to_string()),
                    option.min.unwrap_or(i32::MIN),
                    option.max.unwrap_or(i32::MAX),
                ).expect("Failed to write response");
            } else {
                writeln!(stdout, "option name {} not found", name).expect("Failed to write response");
            }
        }
    }
}
