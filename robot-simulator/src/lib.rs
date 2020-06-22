// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use std::borrow::Borrow;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }
    pub fn get_direction(current_direction: Direction, offset: i8) -> Direction {
        let all_direction = Direction::all();
        let pos: i8 = (all_direction
            .iter()
            .position(|d| *d == current_direction)
            .unwrap_or(0) as i8 + 4 + offset) % 4;
        all_direction[pos as usize]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            position: (x, y),
            direction: d,
        }
    }

    pub fn turn_right(&mut self) -> Self {
        self.direction = Direction::get_direction(self.direction, 1);
        *self.borrow()
    }

    pub fn turn_left(&mut self) -> Self {
        self.direction = Direction::get_direction(self.direction, -1);
        *self.borrow()
    }

    pub fn advance(&mut self) -> Self {
        let (ref mut x, ref mut y) = self.position;
        match self.direction {
            Direction::North => {
                *y += 1;
            }
            Direction::East => {
                *x += 1;
            }
            Direction::South => {
                *y -= 1;
            }
            Direction::West => {
                *x -= 1;
            }
        }
        *self.borrow()
    }

    pub fn instructions(&mut self, instructions: &str) -> Self {
        instructions.chars().for_each(|i| match i {
            'L' => {
                self.turn_left();
            }
            'R' => {
                self.turn_right();
            }
            'A' => {
                self.advance();
            }
            _ => unreachable!(),
        });
        *self.borrow()
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
