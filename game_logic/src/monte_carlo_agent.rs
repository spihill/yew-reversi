use crate::monte_carlo::runner::get_best_move;

use super::{
    ai_agent::AiAgent,
    board::Board,
    types::{AgentType, Color, Coordinate},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MonteCarloAgent {
    color: Color,
}

impl MonteCarloAgent {
    pub fn new(color: Color) -> Self {
        MonteCarloAgent { color }
    }
}

impl AiAgent for MonteCarloAgent {
    fn color(&self) -> Color {
        self.color
    }

    fn next_move(&self, board: &Board) -> Coordinate {
        get_best_move(board, &self.color)
    }

    fn agent_type(&self) -> super::types::AgentType {
        AgentType::MonteCarlo
    }
}
