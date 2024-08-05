use std::collections::HashMap;
use chess::Board;
use crate::search_move::SearchMove;

pub struct TranspostionTable 
{
    table: HashMap<u64, SearchMove>
}

impl TranspostionTable 
{
    pub fn new() -> Self
    {
        TranspostionTable 
        {
            table: HashMap::new()
        }
    }

    pub fn get_position(&self, board: &Board) -> Option<&SearchMove> 
    {
        self.table.get(&board.get_hash())
    }

    pub fn add_position(&mut self, board: &Board, mov: &SearchMove) 
    {
        self.table.insert(board.get_hash(), mov.clone());
    }
}
