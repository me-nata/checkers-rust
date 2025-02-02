#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position(pub usize, pub usize);
impl Position {
    pub fn diff(p1: Position, p2: Position) -> (isize, isize) {
        (
            p2.0 as isize - p1.0 as isize, 
            p2.1 as isize  - p1.1 as isize
        )
    }

    pub fn is_entry(&self, p1: Position, p2: Position) -> bool {
        (p1.0 <= self.0 && p1.0 <= self.1) && (p2.1 > self.0 && p2.1 > self.1)
    }
}
