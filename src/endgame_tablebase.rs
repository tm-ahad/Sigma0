use std::str::FromStr;

use chess::{Board, ChessMove};
use serde_json::Value;

pub struct EndgameTablebase;

impl EndgameTablebase 
{
    pub fn get_move(board: &Board) -> Option<ChessMove> 
    {
        let fen = board.to_string();
        let url = format!("http://tablebase.lichess.ovh/standard?fen={}", fen);

        let response = ureq::get(&url)
            .call()
            .map_err(|_| {})
            .ok()?
            .into_json::<Value>()
            .map_err(|_| {})
            .ok()?;

        if let Some(Value::Array(moves)) = response.get("moves") 
        {
            if let Some(Value::Object(first_move)) = moves.get(0) {
                if let Some(uci) = first_move.get("uci").and_then(Value::as_str) {
                    return ChessMove::from_str(uci).ok();
                }
            }
        }

        None
    }
}