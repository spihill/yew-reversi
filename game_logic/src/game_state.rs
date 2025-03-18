use crate::{
    ai_agent::AiAgent,
    board::Board,
    types::{AgentType, Color, Coordinate, GameStatus},
};

#[derive(Debug)]
pub struct GameState {
    pub board: Board,
    pub turn_count: u32,
    pub turn: Color,
    pub status: GameStatus,
    pub ai_agent: Option<Box<dyn AiAgent>>,
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
            && self.turn_count == other.turn_count
            && self.turn == other.turn
            && self.status == other.status
            && self.agent_type() == other.agent_type()
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: Board::new(),
            turn_count: 0,
            turn: Color::Black,
            status: GameStatus::BeforeStart,
            ai_agent: None,
        }
    }

    pub fn make_move(&mut self, coord: Coordinate) -> Result<(), String> {
        let turn = self.turn;
        self.board.move_piece(coord, turn)?;
        self.finish_turn();
        Ok(())
    }

    pub fn valid_moves(&self) -> Vec<Coordinate> {
        self.board.valid_moves(self.turn)
    }

    pub fn make_agent_move(&mut self) -> Result<(), String> {
        if let Some(agent) = &self.ai_agent {
            let coord = agent.next_move(&self.board);
            self.make_move(coord)
        } else {
            Err("No AI agent set".into())
        }
    }

    pub fn agent_type(&self) -> Option<AgentType> {
        self.ai_agent.as_ref().map(|agent| agent.agent_type())
    }

    pub fn finish_turn(&mut self) {
        self.turn_count += 1;
        if self.board.valid_moves(self.turn).is_empty()
            && self.board.valid_moves(self.turn.opponent()).is_empty()
        {
            let (black_count, white_count) = self.board.count_pieces();
            self.status = match black_count.cmp(&white_count) {
                std::cmp::Ordering::Less => GameStatus::Winner(Color::White),
                std::cmp::Ordering::Equal => GameStatus::Draw,
                std::cmp::Ordering::Greater => GameStatus::Winner(Color::Black),
            }
        } else if !self.board.valid_moves(self.turn.opponent()).is_empty() {
            self.turn = self.turn.opponent();
        }

        let ai_agent_color = self.ai_agent.as_ref().map(|agent| agent.color()).unwrap();

        if ai_agent_color == self.turn && self.status == GameStatus::InProgress {
            self.make_agent_move().unwrap();
        }
    }
}
