use super::Player;

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

    pub fn is_the_same(p1: &Player, p2: &Player) -> bool {
        let team1 = p1.get_team(); 
        let team2 = p2.get_team(); 
        team1 == team2 
    }
}