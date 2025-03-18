use std::rc::Rc;

use game_logic::{
    board::{self, Board},
    monte_carlo::types::Node,
    types::Color,
};
use rand::seq::IndexedRandom;

fn random_move(board: &board::Board, color: Color) -> Board {
    let mut rng = rand::rng();
    let choosed = *board.valid_moves(color).choose(&mut rng).unwrap();
    let mut board = *board;
    board.move_piece(choosed, color).unwrap();
    board
}
fn main() {
    let mut board = Board::new();

    loop {
        let mut moved = false;
        if !board.valid_moves(Color::Black).is_empty() {
            board = random_move(&board, Color::Black);
            moved = true;
        }
        if !board.valid_moves(Color::White).is_empty() {
            let monte_node = Rc::new(Node::new(board, Color::White, &Color::White, None, 0));
            for total_visits in 1..1001 {
                let node = monte_node.select(total_visits);
                let winner = node.simulate();
                node.backpropagate(winner);
            }
            let best_move = monte_node.best_move();
            board.move_piece(best_move, Color::White).unwrap();

            moved = true;
        }
        if !moved {
            break;
        }
    }
    println!("{:?}", board.count_pieces());
}
