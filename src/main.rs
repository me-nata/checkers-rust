use common::Position;
use game::Game;
use movement::MovementAction;

mod common;
mod player;
mod game;
mod board;
mod movement;

fn main() {
    let mut game = Game::new();
    game.initialize();
    
    let (p1, p2) = (Position(2, 0), Position(3, 0));
    game.move_piece(MovementAction(p1, p2));
    
    let (p1, p2) = (Position(5, 1), Position(4, 1));
    game.move_piece(MovementAction(p1, p2));

    let (p1, p2) = (Position(3, 0), Position(5, 2));
    game.move_piece(MovementAction(p1, p2));
}