use std::{collections::HashMap, str::FromStr, time::Duration};
use chess::{Board, ChessMove};
use serde_json::Value;
use ureq::{Agent, AgentBuilder};

pub struct EndGameTablebase 
{
    agent: Agent,
    map: HashMap<String, String>
}

impl EndGameTablebase 
{
    pub fn new() -> Self 
    {
        EndGameTablebase 
        {
            agent: AgentBuilder::new()
                .timeout(Duration::from_secs(2))
                .timeout_connect(Duration::from_secs(2))
                .build(),
            map: HashMap::new()
        }
    }

    pub fn get_move(&mut self, board: &Board) -> Option<ChessMove> 
    {
        let fen = board.to_string();
        let cache = self.map.get(&fen);

        if cache.is_some()
        {
            return Some(ChessMove::from_str(cache.unwrap()).unwrap())
        }

        let url = format!("https://tablebase.lichess.ovh/standard?fen={fen}");

        let response = self.agent.get(&url)
            .call()
            .map_err(|_| {})
            .ok()?
            .into_json::<Value>()
            .map_err(|_| {})
            .ok()?;

        let cloned_board = board.clone();
            
        return if let Some(Value::Array(moves)) = response.get("moves") 
        {
            let moves = moves
                .iter()
                .map(|a| 
                    {
                        match a 
                        {
                            Value::Object(map) => ChessMove::from_str(map["uci"].as_str().unwrap()).unwrap(),
                            _ => todo!()
                        }
                    })
                .collect::<Vec<ChessMove>>();

            for mov in &moves 
            {
                self.map.insert(cloned_board.to_string(), mov.to_string());
                cloned_board.make_move_new(mov.clone());
            }

            Some(moves[0])
        }
        else 
        {
            None
        }
    }
}
