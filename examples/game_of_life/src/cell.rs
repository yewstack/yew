#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Alive,
    Dead,
}

#[derive(Clone, Copy)]
pub struct Cellule {
    pub state: State,
}

impl Cellule {
    pub fn new_dead() -> Self {
        Self { state: State::Dead }
    }

    pub fn set_alive(&mut self) {
        self.state = State::Alive;
    }

    pub fn set_dead(&mut self) {
        self.state = State::Dead;
    }

    pub fn is_alive(self) -> bool {
        self.state == State::Alive
    }

    pub fn toggle(&mut self) {
        if self.is_alive() {
            self.set_dead()
        } else {
            self.set_alive()
        }
    }

    pub fn count_alive_neighbors(neighbors: &[Self]) -> usize {
        neighbors.iter().filter(|n| n.is_alive()).count()
    }

    pub fn alone(neighbors: &[Self]) -> bool {
        Self::count_alive_neighbors(neighbors) < 2
    }

    pub fn overpopulated(neighbors: &[Self]) -> bool {
        Self::count_alive_neighbors(neighbors) > 3
    }

    pub fn can_be_revived(neighbors: &[Self]) -> bool {
        Self::count_alive_neighbors(neighbors) == 3
    }
}
