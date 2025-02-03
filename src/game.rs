use crate::{
    movement::MovementStatus,
    common::Position,
    player::*,
    board::Board
};

pub struct Game {
    p1: Player,
    p2: Player,
    board: Board,
}
impl Game {
    pub fn new() -> Self {
        Game {
            p1: Player::new(Team::Black),
            p2: Player::new(Team::White),
            board: Board::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.board.initialize();
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> MovementStatus {
        let (movement_status, position) = match  MovementStatus::verify_move_piece(&self.board, from, to) {
            Ok((movement, position)) => (movement, position),
            Err(movement) => return movement
        };

        match movement_status {
            MovementStatus::Simple => self.board.move_piece(from, to),
            MovementStatus::Capture => {
                self.board.remove_piece(position.unwrap());
                self.board.move_piece(from, to);
            }
            _ => {}
        }

        self.become_queen(to);
        movement_status
    }

    pub fn display(&self) {
        println!();
        for row in (0..8).rev() {
            print!("{} ", row);
            for col in 0..8 {
                let pos = Position(row, col);
                match self.board.team_from(pos) {
                    Some(Team::White) => print!(" ⚪ "),
                    Some(Team::Black) => print!(" ⚫ "),
                    _ => print!(" .  ")
                }
            }
            println!();
        }
        println!("   0   1   2   3   4   5   6   7");
        println!();
    }

    fn become_queen(&mut self, pos: Position) {
        self.board.set_queen(pos);
    }
}