use std::fmt::{self};

use super::types::{BitBoard, Color, Coordinate};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Board {
    white: BitBoard,
    black: BitBoard,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board { white: 0, black: 0 };
        // 初期配置
        board.set_piece((3, 3).into(), Color::White);
        board.set_piece((3, 4).into(), Color::Black);
        board.set_piece((4, 3).into(), Color::Black);
        board.set_piece((4, 4).into(), Color::White);
        board
    }

    fn set_piece(&mut self, coordinate: Coordinate, color: Color) {
        match color {
            Color::Black => self.black |= coordinate.to_bit(),
            Color::White => self.white |= coordinate.to_bit(),
        }
    }

    pub fn as_array(&self) -> [[Option<Color>; 8]; 8] {
        let mut result = [[None; 8]; 8];
        for y in 0..8 {
            for x in 0..8 {
                let bit = 1 << (y * 8 + x);
                result[y as usize][x as usize] = if self.black & bit != 0 {
                    Some(Color::Black)
                } else if self.white & bit != 0 {
                    Some(Color::White)
                } else {
                    None
                };
            }
        }
        result
    }

    pub fn move_piece(&mut self, coordinate: Coordinate, turn: Color) -> Result<(), String> {
        let bit = coordinate.to_bit();

        let (own, opponent) = match turn {
            Color::Black => (&mut self.black, &mut self.white),
            Color::White => (&mut self.white, &mut self.black),
        };

        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut flipped = false;

        for &(dx, dy) in &directions {
            let mut cx = coordinate.x as i32;
            let mut cy = coordinate.y as i32;
            let mut to_flip = Vec::new();

            loop {
                cx += dx;
                cy += dy;

                // ボードの外に出たら終了
                if !(0..8).contains(&cx) || !(0..8).contains(&cy) {
                    break;
                }

                let next_bit = 1 << (cy * 8 + cx);

                if *opponent & next_bit != 0 {
                    // 相手の駒を記録
                    to_flip.push(next_bit);
                } else if *own & next_bit != 0 {
                    // 自分の駒が見つかれば反転可能
                    if !to_flip.is_empty() {
                        flipped = true;
                        for bit in to_flip {
                            *own |= bit;
                            *opponent &= !bit;
                        }
                    }
                    break;
                } else {
                    // 空白に到達したら終了
                    break;
                }
            }
        }

        if flipped {
            // 駒を配置
            *own |= bit;
            Ok(())
        } else {
            Err("Invalid move: No pieces to flip".into())
        }
    }

    pub fn valid_moves(&self, turn: Color) -> Vec<Coordinate> {
        let mut moves = Vec::new();
        let (own, opponent) = match turn {
            Color::Black => (self.black, self.white),
            Color::White => (self.white, self.black),
        };

        // 各方向への探索オフセット
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for y in 0..8 {
            for x in 0..8 {
                let coord: Coordinate = (x, y).into();
                let bit = coord.to_bit();

                if (self.white | self.black) & bit != 0 {
                    continue;
                }

                let mut valid = false;

                for &(dx, dy) in &directions {
                    let mut found_opponent = false;
                    let mut cx = x as i32;
                    let mut cy = y as i32;

                    loop {
                        cx += dx;
                        cy += dy;

                        // ボードの外に出たら終了
                        if !(0..8).contains(&cx) || !(0..8).contains(&cy) {
                            break;
                        }

                        let next_bit = Coordinate {
                            x: cx as u32,
                            y: cy as u32,
                        }
                        .to_bit();

                        if opponent & next_bit != 0 {
                            // 相手の駒を見つけた
                            found_opponent = true;
                        } else if own & next_bit != 0 {
                            // 自分の駒を見つけた
                            if found_opponent {
                                valid = true;
                            }
                            break;
                        } else {
                            // 空白マスに到達
                            break;
                        }
                    }
                    if valid {
                        moves.push(coord);
                        break;
                    }
                }
            }
        }
        moves
    }

    pub fn count_pieces(&self) -> (u32, u32) {
        (self.black.count_ones(), self.white.count_ones())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let array = self.as_array();
        for row in array.iter() {
            for cell in row.iter() {
                let symbol = match cell {
                    Some(Color::Black) => "B",
                    Some(Color::White) => "W",
                    None => ".",
                };
                write!(f, "{} ", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
