use common::Position;
use game::Game;
use moviment::MovimentStatus;

mod common;
mod player;
mod game;
mod board;
mod moviment;


fn display(game: &Game, status: Option<MovimentStatus>) {
    println!("Status: {:?}", status);
    game.display();
}

fn main() {
    let mut game = Game::new();
    game.initialize();

    let mut status: Option<MovimentStatus> = None;
    display(&game, status);
    
    let (p1, p2) = (Position(2, 0), Position(3, 0));
    status = Some(game.move_piece(p1, p2));
    display(&game, status);
    
    let (p1, p2) = (Position(5, 1), Position(4, 1));
    status = Some(game.move_piece(p1, p2));
    display(&game, status);

    let (p1, p2) = (Position(3, 0), Position(5, 2));
    status = Some(game.move_piece(p1, p2));
    display(&game, status);
}
