use crate::{
    board::Board, movement::MovementAction, player::*
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

    pub fn move_piece(&mut self, action: MovementAction) {
        action.move_piece(&mut self.board);
    }

    pub fn display(&self) {
        self.board.display();
    }
}