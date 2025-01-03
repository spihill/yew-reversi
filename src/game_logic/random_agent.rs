use rand::seq::SliceRandom;

use super::{
    ai_agent::AiAgent,
    board::Board,
    types::{AgentType, Color, Coordinate},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RandomAgent {
    color: Color,
}

impl RandomAgent {
    pub fn new(color: Color) -> Self {
        RandomAgent { color }
    }
}

impl AiAgent for RandomAgent {
    fn color(&self) -> Color {
        self.color
    }

    fn next_move(&self, board: &Board) -> Coordinate {
        let mut rng = rand::thread_rng();
        *board.valid_moves(self.color).choose(&mut rng).unwrap()
    }

    fn agent_type(&self) -> super::types::AgentType {
        AgentType::Random
    }
}
