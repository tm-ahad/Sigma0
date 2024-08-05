use std::{env, io::{self, Write}, str::FromStr};
use chess::{Board, ChessMove};
use redis::{Client, Commands, Connection};
use serde_json::Value;
use ureq::Agent;

pub struct MoveDatabase 
{
    conn: Option<Connection>,
    agent: Agent
}

impl MoveDatabase 
{
    pub fn load() -> MoveDatabase 
    {
        let connection = env::var("REDIS_CONNECTION")
            .unwrap_or_else(|_| panic!("environment variable REDIS_CONNECTION not found."));
        
        let client = Client::open(connection);

        if let Ok(client) = client 
        {
            MoveDatabase 
            {
                conn: Some
                (
                    client
                        .get_connection()
                        .unwrap()
                ),
                agent: Agent::new()
            }
        }
        else 
        {
            let mut stdout = io::stdout();
            let _ = write!(stdout, "info opening book can't be loaded due to network error.");

            MoveDatabase 
            {
                conn: None,
                agent: Agent::new()
            }
        }
    }

    pub fn get_move(&mut self, board: &Board) -> Option<ChessMove>
    {
        let conn;
        if self.conn.is_none()
        {
            return None;
        }
        else 
        {
            conn = self.conn.as_mut().unwrap();
        }

        let board_fen = board.to_string();
        let uci = conn.get(board_fen)
            .map_or(None, |uci: String| Some(uci));

        if let Some(uci) = uci 
        {
            Some(ChessMove::from_str(&uci).unwrap())
        }
        else 
        {
            None
        }
    }

    pub fn add_move(&mut self, board: &Board) 
    {
        let conn;
        if self.conn.is_none()
        {
            return ()
        }
        else 
        {
            conn = self.conn.as_mut().unwrap();
        }

        let board_fen = board.to_string();
        let uri = format!("https://stockfish.online/api/s/v2.php?fen={board_fen}&depth=15");
        let stockfish_move = self.agent.get(&uri)
            .call()
            .unwrap()
            .into_json::<Value>()
            .unwrap();

        match stockfish_move 
        {
            Value::Object(map) => 
            {
                let str = map["bestmove"]
                    .to_string();

                let spl = str
                    .split_whitespace()
                    .collect::<Vec<&str>>();

                let uci_move = spl[1].to_string();
                let _ = conn.set::<String, String, String>(board_fen, uci_move);
            },
            _ => {}
        }

    }
}

