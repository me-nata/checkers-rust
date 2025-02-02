use crate::player::Team;

#[derive(Clone, Copy)]
pub struct Piece {
    team: Team,
    is_queen: bool,
}
impl Piece {
    pub fn new(team: Team) -> Self {
        Piece {
            team: team,
            is_queen: false,
        }
    }

    pub fn get_team(&self) -> Team {
        self.team
    }

    pub fn is_queen(&self) -> bool {
        self.is_queen
    }

    pub fn set_queen(&mut self) {
        self.is_queen = true;
    }
}