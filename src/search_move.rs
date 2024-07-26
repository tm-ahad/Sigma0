use chess::ChessMove;

#[derive(Clone)]
pub struct SearchMove
{
    mov: Option<ChessMove>,
    evaluation: f32
}

impl SearchMove
{
    pub fn mov(&self) -> Option<ChessMove>
    {
        self.mov
    }

    pub fn eval(&self) -> f32
    {
        self.evaluation
    }

    pub fn new(mov: Option<ChessMove>, evaluation: f32) -> SearchMove
    {
        SearchMove
        {
            mov,
            evaluation
        }
    }
}