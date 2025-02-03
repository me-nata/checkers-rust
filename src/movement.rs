use crate::{board::Board, common::Position, player::Team};

#[derive(Debug)]
pub enum MovementStatus {
    Not,
    Simple,
    Capture,
    NoPieceInThisPosition,
    NoPieceToCapture,
    PositionAlreadyOccupied,
    OutOfBoard,
    MovementNotAccept
}
impl MovementStatus {
    pub fn verify_physical_limits(board: &Board, from: Position, to: Position)
    -> Result<(), MovementStatus> {
        let min = Position(0, 0);
        let max = Position(8, 8);

        // is in the board
        if !from.is_entry(min, max) || !to.is_entry(min, max) {
            return Err(MovementStatus::OutOfBoard);
        }
        
        // position from exists
        if !board.has_piece(from) {
            return Err(MovementStatus::NoPieceInThisPosition);
        }
        
        // position to is not occupied
        if board.has_piece(to) {
            return Err(MovementStatus::PositionAlreadyOccupied);
        }

        Ok(())
    }

    fn verify_diagonal_queen_move(
        board: &Board, 
        from: Position, 
        to: Position, 
        from_team: Team, 
        delta_x: isize, 
        delta_y: isize
    ) -> Result<(MovementStatus, Option<Position>), MovementStatus>{
        let mut i = from.0;
        let mut j = from.1;
        let mut found_piece: Option<Position> = None;

        while i != to.0 || j != to.1{
            i = if i != to.0 {(i as isize + delta_x) as usize} else {i};
            j = if j != to.1 {(j as isize + delta_y) as usize} else {j};

            if board.has_piece(Position(i, j)) {
                let is_same_team = board.team_from(Position(i, j)).unwrap() == from_team;

                if found_piece.is_some() || is_same_team {
                    return Err(MovementStatus::MovementNotAccept);
                }

                found_piece = Some(Position(i, j));
            }
        }

        match found_piece.is_some() {
            true => Ok((MovementStatus::Capture, found_piece)),
            false => Ok((MovementStatus::Simple, None))
        }
    }

    fn verify_normal_move(
        board: &Board,
        from: Position,
        from_team: Team,
        delta: (isize, isize),
    ) -> Result<(MovementStatus, Option<Position>), MovementStatus> {
        let to = Position(
            (from.0 as isize + delta.0) as usize,
            (from.1 as isize + delta.1) as usize,
        );

        if !board.has_piece(to) {
            return Err(MovementStatus::NoPieceToCapture);
        }

        match board.team_from(to).unwrap() {
            team if team == from_team => Ok((MovementStatus::Simple, None)),
            _ => Ok((MovementStatus::Capture, Some(to))),
        }
    }

    fn verify_move_rules(board: &Board, from: Position, to: Position)
    -> Result<(MovementStatus, Option<Position>), MovementStatus> {

        let (dx, dy) = Position::diff(from, to);
        let (abs_dx, abs_dy) = (dx.abs(), dy.abs());

        // simple move
        if abs_dx == 1 && abs_dy == 0 {
            return Ok((MovementStatus::Simple, None));
        }

        // no horizontal move or back move (for not queen)
        let is_queen = board.is_queen_in(from).unwrap();
        if (abs_dx != abs_dy) || (is_queen && abs_dy > 0) {
            return Err(MovementStatus::MovementNotAccept);
        }

        let from_team = board.team_from(from).unwrap();
        
        let (delta_x, delta_y) = (
            if dx > 0 {1 as isize} else {-1 as isize},
            if dy > 0 {1 as isize} else {-1 as isize},
        );

        if is_queen {
            MovementStatus::verify_diagonal_queen_move(&board, from, to, from_team, delta_x, delta_y)
        } else {
            MovementStatus::verify_normal_move(&board, from, from_team, (delta_x, delta_y))
        }
    }

    pub fn verify_move_piece(board: &Board, from: Position, to: Position) 
    -> Result<(MovementStatus, Option<Position>), MovementStatus> {
        if from == to {
            return Ok((MovementStatus::Not, None));
        }

        match MovementStatus::verify_physical_limits(board, from, to) {
            Ok(_) => MovementStatus::verify_move_rules(board, from, to),
            Err(err) => Err(err)
        }
    }
}