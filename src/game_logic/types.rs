#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn opponent(self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AgentType {
    Random,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameStatus {
    BeforeStart,
    InProgress,
    Winner(Color),
    Draw,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
}
impl Coordinate {
    pub fn to_bit(self) -> BitBoard {
        1 << (self.y * 8 + self.x)
    }
}

impl From<(u32, u32)> for Coordinate {
    fn from((x, y): (u32, u32)) -> Self {
        Coordinate { x, y }
    }
}

pub type BitBoard = u64;
