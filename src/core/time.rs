#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChessTime {
    pub round: u32,
    pub turn: u32,
    pub movement: u32,
}

impl Default for ChessTime {
    fn default() -> Self {
        Self::new()
    }
}

impl ChessTime {
    pub fn new() -> Self {
        Self {
            round: 0,
            turn: 0,
            movement: 0,
        }
    }

    pub fn on_movement(&mut self) {
        self.movement += 1;
    }

    pub fn on_turn(&mut self) {
        self.turn += 1;
        self.movement = 0;
    }

    pub fn on_round(&mut self) {
        self.round += 1;
        self.turn = 0;
        self.movement = 0;
    }
}
