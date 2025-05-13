use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    pub fn other(&self) -> Axis {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X,
        }
    }

    /// Returns the direction of the axis with the given sign
    /// if 0, returns the positive direction
    pub fn to_direction(&self, sign: i32) -> Direction {
        match (self, sign.is_negative()) {
            (Axis::X, true) => Direction::Left,
            (Axis::X, false) => Direction::Right,
            (Axis::Y, true) => Direction::Down,
            (Axis::Y, false) => Direction::Up,
        }
    }

    pub fn to_position(&self) -> Position {
        match self {
            Axis::X => Position::new(1, 0),
            Axis::Y => Position::new(0, 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,    // (x: 0, y: +1) (Axis: Y)
    Right, // (x: +1, y: 0) (Axis: X)
    Down,  // (x: 0, y: -1) (Axis: Y)
    Left,  // (x: -1, y: 0) (Axis: X)
}

impl Direction {
    pub fn axis(&self) -> Axis {
        match self {
            Direction::Up | Direction::Down => Axis::Y,
            Direction::Right | Direction::Left => Axis::X,
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    pub fn to_position(&self) -> Position {
        match self {
            Direction::Up => Position::new(0, 1),
            Direction::Right => Position::new(1, 0),
            Direction::Down => Position::new(0, -1),
            Direction::Left => Position::new(-1, 0),
        }
    }

    pub fn to_subdirection(&self) -> SubDirection {
        match self {
            Direction::Up => SubDirection::Up,
            Direction::Right => SubDirection::Right,
            Direction::Down => SubDirection::Down,
            Direction::Left => SubDirection::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubDirection {
    Up,        // Up
    UpRight,   // Up + Right
    Right,     // Right
    DownRight, // Down + Right
    Down,      // Down
    DownLeft,  // Down + Left
    Left,      // Left
    UpLeft,    // Up + Left
}

impl SubDirection {
    pub fn opposite(&self) -> SubDirection {
        match self {
            SubDirection::Up => SubDirection::Down,
            SubDirection::UpRight => SubDirection::DownLeft,
            SubDirection::Right => SubDirection::Left,
            SubDirection::DownRight => SubDirection::UpLeft,
            SubDirection::Down => SubDirection::Up,
            SubDirection::DownLeft => SubDirection::UpRight,
            SubDirection::Left => SubDirection::Right,
            SubDirection::UpLeft => SubDirection::DownRight,
        }
    }

    pub fn to_position(&self) -> Position {
        match self {
            SubDirection::Up => Position::new(0, 1),
            SubDirection::UpRight => Position::new(1, 1),
            SubDirection::Right => Position::new(1, 0),
            SubDirection::DownRight => Position::new(1, -1),
            SubDirection::Down => Position::new(0, -1),
            SubDirection::DownLeft => Position::new(-1, -1),
            SubDirection::Left => Position::new(-1, 0),
            SubDirection::UpLeft => Position::new(-1, 1),
        }
    }
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn add_direction(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => self.up(),
            Direction::Right => self.right(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
        }
    }

    pub fn add_subdirection(&self, subdirection: SubDirection) -> Self {
        match subdirection {
            SubDirection::Up => self.up(),
            SubDirection::UpRight => self.up().right(),
            SubDirection::Right => self.right(),
            SubDirection::DownRight => self.down().right(),
            SubDirection::Down => self.down(),
            SubDirection::DownLeft => self.down().left(),
            SubDirection::Left => self.left(),
            SubDirection::UpLeft => self.up().left(),
        }
    }

    pub fn length_sqr(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f32 {
        (self.length_sqr() as f32).sqrt()
    }

    pub fn jump_length(&self) -> i32 {
        self.x.abs().max(self.y.abs())
    }

    pub fn distance_sqr(&self, other: &Self) -> i32 {
        (self - other).length_sqr()
    }

    pub fn distance(&self, other: &Self) -> f32 {
        (self.distance_sqr(other) as f32).sqrt()
    }

    pub fn jump_distance(&self, other: &Self) -> i32 {
        let dpos = *self - *other;
        dpos.jump_length()
    }

    pub fn is_axial(&self) -> bool {
        self.x == 0 || self.y == 0
    }

    pub fn is_diagonal(&self) -> bool {
        self.x.abs() == self.y.abs()
    }

    pub fn is_subdir(&self) -> bool {
        self.is_axial() || self.is_diagonal()
    }

    pub fn is_axial_relative(&self, other: &Self) -> bool {
        let dpos = *self - *other;
        dpos.is_axial()
    }

    pub fn is_diagonal_relative(&self, other: &Self) -> bool {
        let dpos = *self - *other;
        dpos.is_diagonal()
    }

    pub fn is_subdir_relative(&self, other: &Self) -> bool {
        let dpos = *self - *other;
        dpos.is_subdir()
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, direction: Direction) -> Self {
        self.add_direction(direction)
    }
}

impl Add<SubDirection> for Position {
    type Output = Self;

    fn add(self, subdirection: SubDirection) -> Self {
        self.add_subdirection(subdirection)
    }
}

impl Add<Position> for Position {
    type Output = Self;

    fn add(self, other: Position) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<&Position> for Position {
    type Output = Self;

    fn add(self, other: &Position) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<&Position> for &Position {
    type Output = Position;

    fn add(self, other: &Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Direction> for Position {
    type Output = Self;

    fn sub(self, direction: Direction) -> Self {
        self.add_direction(direction.opposite())
    }
}

impl Sub<SubDirection> for Position {
    type Output = Self;

    fn sub(self, subdirection: SubDirection) -> Self {
        self.add_subdirection(subdirection.opposite())
    }
}

impl Sub<Position> for Position {
    type Output = Self;

    fn sub(self, other: Position) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<&Position> for Position {
    type Output = Self;

    fn sub(self, other: &Position) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<&Position> for &Position {
    type Output = Position;

    fn sub(self, other: &Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
