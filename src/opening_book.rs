use std::time::Duration;

use chess::{Board, ChessMove};
use ureq::{Agent, AgentBuilder};

pub struct OpeningBook 
{
    _agent: Agent
}

impl OpeningBook 
{
    pub fn new() -> Self
    {
        OpeningBook 
        {
            _agent: AgentBuilder::new()
                .timeout(Duration::from_secs(2))
                .build()
        }
    }

    pub fn get_move(&self, _board: &Board) -> Option<ChessMove> 
    {
        // let fen = board.to_string();
        // let color = if board.side_to_move() == Color::White {"white"} else {"black"};

        // let url = format!("https://explorer.lichess.ovh/master?player=foo&color={color}&fen={fen}");

        // let response = self.agent.get(&url)
        //     .call()
        //     .map_err(|_| {})
        //     .ok()?
        //     .into_json::<Value>()
        //     .map_err(|_| {})
        //     .ok()?;

        // if let Some(Value::Array(moves)) = response.get("moves") 
        // {
        //     if let Some(Value::Object(first_move)) = moves.get(0) {
        //         if let Some(uci) = first_move.get("uci").and_then(Value::as_str) {
        //             return ChessMove::from_str(uci).ok();
        //         }
        //     }
        // }

        None
    }
}