use std::{collections::HashMap, str::FromStr, time::Duration};

use chess::{Board, ChessMove, Color};
use serde_json::Value;
use ureq::{Agent, AgentBuilder};

pub struct OpeningBook 
{
    agent: Agent
}

impl OpeningBook 
{

    pub fn new() -> Self
    {
        OpeningBook 
        {
            agent: AgentBuilder::new()
                .timeout(Duration::from_secs(2))
                .build()
        }
    }
    pub fn get_move(&self, board: &Board) -> Option<ChessMove>
    {
        let fen = board.to_string();
        let color = match board.side_to_move() 
        {
            Color::White => "white",
            Color::Black => "black"
        };

        let resp = self.agent.get(&format!("https://explorer.lichess.ovh/player?player=Sigma0&color={color}&fen={fen}"))
            .call();

        if let Ok(resp) = resp 
        {
            let str_response = resp.into_json::<HashMap<String, Value>>();

            if str_response.is_err() 
            {
                return None
            }
            
            let str_response = str_response.unwrap();
            let moves = str_response.get("moves")
                .unwrap();

            match moves 
            {
                Value::Array(vec) => 
                {
                    if vec.is_empty() 
                    {
                        return None
                    }

                    match &vec[0]
                    {
                        Value::Object(map) => 
                        {
                            Some(ChessMove::from_str(map["uci"].as_str().unwrap())
                                .unwrap_or_else(|_| panic!("Invalid move")))
                        },
                        _ => todo!()
                    }
                },
                _ => todo!()
            }
        }
        else 
        {
            None
        }   
    }
}

