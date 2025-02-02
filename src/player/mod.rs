pub mod team;
pub use team::Team;

pub struct Player {
    team: Team
}
impl Player {
    pub fn new(team: Team) -> Self {
        Player { team: team }
    }

    pub fn is_team(&self, team: Team) -> bool {
        self.team == team
    }

    pub fn get_team(&self) -> Team {
        self.team
    }
}