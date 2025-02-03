mod piece;
pub use piece::Piece;
use crate::{common::Position, player::Team};
use crate::movement::MovementStatus;


pub struct Board([[Option<Piece>; 8]; 8]);
impl Board {
    pub fn new() -> Self {
        Board([[const { None }; 8]; 8])
    }

    pub fn initialize(&mut self)  {
        // white
        for row in 0..3 {
            let k = if row == 1 {1} else {0};
        
            for col in (k..8).step_by(2) {
                self.0[row as usize][col as usize] = Some(
                    Piece::new(Team::White)
                )
            }
        }

        // black
        for row in 5..8 {
            let k = if row == 6 {0} else {1};
        
            for col in (k..8).step_by(2) {
                self.0[row as usize][col as usize] = Some(
                    Piece::new(Team::Black)
                )
            }
        }
    }

    pub fn has_piece(&self, pos: Position) -> bool {
        let piece = self.0[pos.0][pos.1];
        piece.is_some()
    }

    pub fn team_from(&self, pos: Position) -> Option<Team> {
        self.0[pos.0][pos.1]
            .as_ref() 
            .map(|piece| piece.get_team())
    }

    pub fn is_queen_in(&self, pos: Position) -> Option<bool> {
        self.0[pos.0][pos.1]
            .as_ref() 
            .map(|piece| piece.is_queen())
    }

    pub fn remove_piece(&mut self, pos: Position) -> Option<Piece> {
        let response = self.0[pos.0][pos.1];
        self.0[pos.0][pos.1] = None;
        response
    }

    pub fn move_piece(&mut self, from: Position, to: Position) {
        self.0[to.0][to.1] = self.remove_piece(from);
    }

    pub fn set_queen(&mut self, pos: Position) {
        let verify = match self.team_from(pos).unwrap() {
            Team::White => {pos.0 == 7},
            Team::Black => {pos.0 == 0}
        };

        if verify {
            self.0[pos.0][pos.1].unwrap().set_queen();
        }
    }
}