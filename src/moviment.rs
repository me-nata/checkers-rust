use crate::{board::Board, common::Position, player::Team};

#[derive(Debug)]
pub enum MovimentStatus {
    Not,
    Simple,
    Capture,
    NoPieceInThisPosition,
    NoPieceToCapture,
    PositionAlreadyOccupied,
    OutOfBoard,
    MovimentNotAccept
}
impl MovimentStatus {
    pub fn verify_phisical_limits(board: &Board, from: Position, to: Position)
    -> Result<(), MovimentStatus> {
        let min = Position(0, 0);
        let max = Position(8, 8);

        // is in the board
        if !from.is_entry(min, max) || !to.is_entry(min, max) {
            return Err(MovimentStatus::OutOfBoard);
        }
        
        // position from exists
        if !board.has_piece(from) {
            return Err(MovimentStatus::NoPieceInThisPosition);
        }
        
        // position to is not occupied
        if board.has_piece(to) {
            return Err(MovimentStatus::PositionAlreadyOccupied);
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
    ) -> Result<(MovimentStatus, Option<Position>), MovimentStatus>{
        let mut i = from.0;
        let mut j = from.1;
        let mut found_piece: Option<Position> = None;

        while i != to.0 || j != to.1{
            i = if i != to.0 {(i as isize + delta_x) as usize} else {i};
            j = if j != to.1 {(j as isize + delta_y) as usize} else {j};

            if board.has_piece(Position(i, j)) {
                let is_same_team = board.team_from(Position(i, j)).unwrap() == from_team;

                if found_piece.is_some() || is_same_team {
                    return Err(MovimentStatus::MovimentNotAccept);
                }

                found_piece = Some(Position(i, j));
            }
        }

        match found_piece.is_some() {
            true => Ok((MovimentStatus::Capture, found_piece)),
            false => Ok((MovimentStatus::Simple, None))
        }
    }

    fn verify_normal_move(
        board: &Board, 
        from: Position, 
        from_team: Team, 
        delta_x: isize, 
        delta_y: isize
    ) -> Result<(MovimentStatus, Option<Position>), MovimentStatus>{
        let (i, j) = (
            (from.0 + delta_x as usize),
            (from.1 + delta_y as usize)
        );

        let outher_pos = Position(i, j);
        if !board.has_piece(outher_pos) {
            return Err(MovimentStatus::NoPieceToCapture);
        }

        match board.team_from(outher_pos).unwrap() {
            from_team => Ok((MovimentStatus::Simple, None)),
            _ => Ok((MovimentStatus::Capture, Some(outher_pos)))
        }        
    }

    fn verify_move_rules(board: &Board, from: Position, to: Position)
    -> Result<(MovimentStatus, Option<Position>), MovimentStatus> {

        let (dx, dy) = Position::diff(from, to);
        let (abs_dx, abs_dy) = (dx.abs(), dy.abs());

        // simple move
        if abs_dx == 1 && abs_dy == 0 {
            return Ok((MovimentStatus::Simple, None));
        }

        // no horizontal move or back move (for not queen)
        let is_queen = board.is_queen_in(from).unwrap();
        if (abs_dx != abs_dy) || (is_queen && abs_dy > 0) {
            return Err(MovimentStatus::MovimentNotAccept);
        }

        let from_team = board.team_from(from).unwrap();
        
        let (delta_x, delta_y) = (
            if dx > 0 {1 as isize} else {-1 as isize},
            if dy > 0 {1 as isize} else {-1 as isize},
        );

        if is_queen {
            MovimentStatus::verify_diagonal_queen_move(&board, from, to, from_team, delta_x, delta_y)
        } else {
            MovimentStatus::verify_normal_move(&board, from, from_team, delta_x, delta_y)
        }
    }

    pub fn verify_move_piece(board: &Board, from: Position, to: Position) 
    -> Result<(MovimentStatus, Option<Position>), MovimentStatus> {
        if from == to {
            return Ok((MovimentStatus::Not, None));
        }

        match MovimentStatus::verify_phisical_limits(board, from, to) {
            Ok(_) => MovimentStatus::verify_move_rules(board, from, to),
            Err(err) => Err(err)
        }
    }
}