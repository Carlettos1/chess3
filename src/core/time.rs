use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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

impl Div<u32> for ChessTime {
    type Output = ChessTime;

    fn div(self, rhs: u32) -> Self::Output {
        Self {
            round: self.round / rhs,
            turn: self.turn / rhs,
            movement: self.movement / rhs,
        }
    }
}

impl Div<u32> for &ChessTime {
    type Output = ChessTime;

    fn div(self, rhs: u32) -> Self::Output {
        *self / rhs
    }
}

impl DivAssign<u32> for ChessTime {
    fn div_assign(&mut self, rhs: u32) {
        self.round /= rhs;
        self.turn /= rhs;
        self.movement /= rhs;
    }
}

impl Mul<u32> for ChessTime {
    type Output = ChessTime;

    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            round: self.round * rhs,
            turn: self.turn * rhs,
            movement: self.movement * rhs,
        }
    }
}

impl Mul<u32> for &ChessTime {
    type Output = ChessTime;

    fn mul(self, rhs: u32) -> Self::Output {
        *self * rhs
    }
}

impl MulAssign<u32> for ChessTime {
    fn mul_assign(&mut self, rhs: u32) {
        self.round *= rhs;
        self.turn *= rhs;
        self.movement *= rhs;
    }
}

impl Add for ChessTime {
    type Output = ChessTime;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            round: self.round + rhs.round,
            turn: self.turn + rhs.turn,
            movement: self.movement + rhs.movement,
        }
    }
}

impl AddAssign for ChessTime {
    fn add_assign(&mut self, rhs: Self) {
        self.round += rhs.round;
        self.turn += rhs.turn;
        self.movement += rhs.movement;
    }
}

impl Sub for ChessTime {
    type Output = ChessTime;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            round: self.round - rhs.round,
            turn: self.turn - rhs.turn,
            movement: self.movement - rhs.movement,
        }
    }
}

impl SubAssign for ChessTime {
    fn sub_assign(&mut self, rhs: Self) {
        self.round -= rhs.round;
        self.turn -= rhs.turn;
        self.movement -= rhs.movement;
    }
}
