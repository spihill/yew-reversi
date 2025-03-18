use std::rc::Rc;

use crate::{
    board::Board,
    types::{Color, Coordinate},
};

use super::types::Node;

pub fn get_best_move(board: &Board, color: &Color) -> Coordinate {
    let monte_node = Rc::new(Node::new(*board, *color, color, None, 0));
    for total_visits in 1..1001 {
        let node = monte_node.select(total_visits);
        let winner = node.simulate();
        node.backpropagate(winner);
    }
    monte_node.best_move()
}
