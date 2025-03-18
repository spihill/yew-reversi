use rand::seq::IndexedRandom;
use rand::seq::SliceRandom;
use std::cell::RefMut;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::board::Board;
use crate::types::Color;
use crate::types::Coordinate;

pub type LightCoord = u8;

pub fn coord_to_light(coord: Coordinate) -> LightCoord {
    (coord.y * 8 + coord.x) as LightCoord
}

pub fn light_to_coord(light: LightCoord) -> Coordinate {
    Coordinate {
        x: (light % 8) as u32,
        y: (light / 8) as u32,
    }
}

const EXPANSION_THRESHOLD: u32 = 10;

pub struct Node<'a> {
    pub wins: RefCell<u32>,
    pub visits: RefCell<u32>,
    pub draws: RefCell<u32>,
    pub board: RefCell<Board>,
    pub turn: Color,
    pub self_color: &'a Color,
    pub children: RefCell<HashMap<LightCoord, Rc<Node<'a>>>>,
    pub parent: RefCell<Option<Weak<Node<'a>>>>,
    pub valid_moves: RefCell<Vec<LightCoord>>,
    pub depth: u32,
}

impl<'a> Node<'a> {
    pub fn new(
        board: Board,
        turn: Color,
        self_color: &'a Color,
        parent: Option<Rc<Node<'a>>>,
        depth: u32,
    ) -> Self {
        let mut rng = rand::rng();
        let mut valid_moves = board.valid_moves(turn);
        valid_moves.shuffle(&mut rng);
        let visits = if parent.is_none() {
            EXPANSION_THRESHOLD + 1
        } else {
            0
        };

        Node {
            wins: RefCell::new(0),
            visits: RefCell::new(visits),
            draws: RefCell::new(0),
            board: RefCell::new(board),
            turn,
            self_color,
            children: RefCell::new(HashMap::new()),
            parent: RefCell::new(parent.map(|p| Rc::downgrade(&p))),
            valid_moves: RefCell::new(
                valid_moves
                    .iter()
                    .map(|&coord| coord_to_light(coord))
                    .collect(),
            ),
            depth,
        }
    }

    pub fn select(self: &Rc<Self>, total_visits: u32) -> Rc<Node<'a>> {
        let mut valid_moves = self.valid_moves.borrow_mut();
        let visits = *self.visits.borrow();
        if visits < EXPANSION_THRESHOLD {
            return Rc::clone(self);
        }
        if !valid_moves.is_empty() {
            let c = self.expand(&mut valid_moves);
            return self.children.borrow().get(&c).unwrap().clone();
        }

        let mut best_ucb1 = f64::NEG_INFINITY;
        let mut best_node = None;

        let children = self.children.borrow();
        if children.is_empty() {
            return Rc::clone(self);
        }
        for child in children.values() {
            let ucb1 = child.ucb1(total_visits);
            if ucb1 > best_ucb1 {
                best_ucb1 = ucb1;
                best_node = Some(child);
            }
        }

        best_node.unwrap().select(total_visits)
    }

    pub fn best_move(self: &Rc<Self>) -> Coordinate {
        let children = self.children.borrow();
        let mut best_visits: i32 = -1;
        let mut best_coord = 0;
        for (coord, child) in children.iter() {
            let visits = *child.visits.borrow() as i32;
            if best_visits < visits {
                best_visits = visits;
                best_coord = *coord;
            }
        }
        light_to_coord(best_coord)
    }

    pub fn expand(self: &Rc<Self>, valid_moves: &mut RefMut<'_, Vec<u8>>) -> LightCoord {
        let valid_move = *valid_moves.last().unwrap();
        valid_moves.pop();

        let mut new_board = *self.board.borrow();
        new_board
            .move_piece(light_to_coord(valid_move), self.turn)
            .unwrap();

        let turn = if new_board.valid_moves(self.turn.opponent()).is_empty() {
            self.turn
        } else {
            self.turn.opponent()
        };

        let new_node = Node::new(
            new_board,
            turn,
            self.self_color,
            Some(Rc::clone(self)),
            self.depth + 1,
        );

        self.children
            .borrow_mut()
            .insert(valid_move, Rc::new(new_node));

        valid_move
    }

    pub fn backpropagate(self: &Rc<Self>, winner: Option<Color>) {
        *self.visits.borrow_mut() += 1;
        if let Some(winner) = winner {
            if winner != self.turn {
                *self.wins.borrow_mut() += 1;
            }
        } else {
            *self.draws.borrow_mut() += 1;
        }
        let parent_opt = self.parent.borrow().as_ref().and_then(|p| p.upgrade());
        if let Some(parent) = parent_opt {
            parent.backpropagate(winner);
        }
    }

    pub fn simulate(self: &Rc<Self>) -> Option<Color> {
        let mut board = *self.board.borrow();
        let mut turn = self.turn;
        let mut pass = false;
        let mut rng = rand::rng();
        loop {
            let valid_moves = board.valid_moves(turn);
            if valid_moves.is_empty() {
                if pass {
                    break;
                }
                pass = true;
                turn = turn.opponent();
                continue;
            }
            pass = false;
            let coord = *valid_moves.choose(&mut rng).unwrap();
            board.move_piece(coord, turn).unwrap();
            turn = turn.opponent();
        }
        let (black_count, white_count) = board.count_pieces();
        match black_count.cmp(&white_count) {
            std::cmp::Ordering::Less => Some(Color::White),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some(Color::Black),
        }
    }

    pub fn ucb1(self: &Rc<Self>, total_visits: u32) -> f64 {
        let visits = *self.visits.borrow();
        if *self.visits.borrow() == 0 {
            return f64::INFINITY;
        }
        let draws = *self.draws.borrow();
        let wins = *self.wins.borrow();
        let exploitation = (wins * 2 + draws) as f64 / (visits * 2) as f64;
        let exploration = 2.0 * (total_visits as f64).ln() / visits as f64;
        exploitation + exploration.sqrt()
    }
}
