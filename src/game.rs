use crate::{
    board::Board, movement::MovementAction, player::*
};

pub struct Game {
    p1: Player,
    p2: Player,
    p1_turn: bool,
    board: Board,
}
impl Game {
    pub fn new() -> Self {
        Game {
            p1: Player::new(Team::Black),
            p2: Player::new(Team::White),
            p1_turn: true,
            board: Board::new(),
        }
    }

    fn switch_turn(&mut self) {
        self.p1_turn = !self.p1_turn;
    }

    pub fn initialize(&mut self) {
        self.board.initialize();
    }

    pub fn move_piece(&mut self, player: &Player, action: MovementAction) {
        if Team::is_the_same(player, &self.p1) || !self.p1_turn{
            action.move_piece(&mut self.board);
        }

        self.switch_turn();
    }

    pub fn display(&self) {
        self.board.display();
    }
}