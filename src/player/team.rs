#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Team {
    Black,
    White
}
impl Team {
    pub fn get_opponent(team: Team) -> Team {
        match team {
            Team::Black => Team::White,
            Team::White => Team::Black,
        }
    }
}