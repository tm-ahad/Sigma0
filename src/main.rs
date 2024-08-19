use std::env::args;
use crate::info::print_info;
use crate::uci::start_uci;

mod eval;
mod search;
mod uci;
mod info;
mod material;
mod consts;
mod search_move;
mod promotion_piece;
mod move_string_conversion;
mod piece_table;
mod transposition_table;
mod endgame_tablebase;
mod move_database;

fn main()
{
    let arguments = args().collect::<Vec<String>>();
    let _ = dotenv::from_path("D:/rust-projects/Sigma1/.env");

    if arguments.len() == 1
    {
        start_uci()
    }
    else
    {
        print_info()
    }
}
