use std::fmt::Debug;

use super::{
    board::Board,
    random_agent::RandomAgent,
    types::{AgentType, Color, Coordinate},
};

pub trait AiAgent: Debug {
    fn color(&self) -> Color;
    fn agent_type(&self) -> AgentType;
    fn next_move(&self, board: &Board) -> Coordinate;
}

pub fn initialize_agent(agent_type: AgentType, color: Color) -> Box<dyn AiAgent> {
    match agent_type {
        AgentType::Random => Box::new(RandomAgent::new(color)),
    }
}

pub fn renew_agent(agent: &Option<Box<dyn AiAgent>>) -> Option<Box<dyn AiAgent>> {
    if let Some(agent) = agent {
        match agent.agent_type() {
            AgentType::Random => Some(Box::new(RandomAgent::new(agent.color()))),
        }
    } else {
        None
    }
}
