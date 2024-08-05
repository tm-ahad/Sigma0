use std::{io::{self, Write}, str::FromStr, time::Duration};
use chess::{Board, ChessMove};
use serde_json::Value;
use ureq::{Agent, AgentBuilder};

pub struct EndGameTablebase {
    agent: Agent,
    failed: bool
}

impl EndGameTablebase {
    pub fn new() -> Self {
        EndGameTablebase {
            agent: AgentBuilder::new()
                .timeout(Duration::from_secs(2))
                .timeout_connect(Duration::from_secs(2))
                .build(),
            failed: false,
        }
    }

    pub fn get_move(&mut self, board: &Board) -> Option<ChessMove> 
    {

        if self.failed 
        {
            return None
        }

        let fen = board.to_string();
        let url = format!("http://tablebase.lichess.ovh/standard/mailine?fen={fen}");

        let response = self.agent.get(&url)
            .call()
            .map_err(|_| {})
            .ok()?
            .into_json::<Value>()
            .map_err(|_| {})
            .ok()?;

        if let Some(Value::Array(moves)) = response.get("mainline") {
            if let Some(Value::Object(first_move)) = moves.first() {
                if let Some(uci) = first_move.get("uci").and_then(Value::as_str) {
                    return ChessMove::from_str(uci).ok();
                }
            }
        }

        if self.failed == false 
        {
            let mut stdout = io::stdout();
            let _ = write!(stdout, "info syzygy endgame tablebases can't be loaded due to network error.");
        }

        self.failed = true;

        None
    }
}
