use crate::{board::Board, common::Position};
use super::MovementStatus;

pub struct MovementAction(pub Position, pub Position);
impl MovementAction {
    pub fn move_piece(&self, board: &mut Board) -> MovementStatus {
        let (movement_status, position) = 
            match  MovementStatus::verify_move_piece(board, self.0, self.1) {
                Ok((movement, position)) => (movement, position),
                Err(movement) => return movement
            };

        match movement_status {
            MovementStatus::Simple => board.move_piece(self.0, self.1),
            MovementStatus::Capture => {
                board.remove_piece(position.unwrap());
                board.move_piece(self.0, self.1);
            }
            _ => {}
        }

        board.set_queen(self.1);
        movement_status
    }
}