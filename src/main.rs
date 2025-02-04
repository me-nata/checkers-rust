use common::Position;
use game::Game;

mod common;
mod player;
mod game;
mod board;
mod movement;

use movement::MovementStatus;

/// The entry point of the program, initializes a new game and performs a series
/// of moves, displaying the game state and movement status after each move.

fn main() {
    let mut game = Game::new();
    game.initialize();
    
    let (p1, p2) = (Position(2, 0), Position(3, 0));
    game.move_piece(p1, p2);
    
    let (p1, p2) = (Position(5, 1), Position(4, 1));
    game.move_piece(p1, p2);

    let (p1, p2) = (Position(3, 0), Position(5, 2));
    game.move_piece(p1, p2);
}