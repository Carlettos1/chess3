use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternEnum {
    SubdirectionalPattern(i32), // King is 1
    DirectionalPattern(i32),    // Rook is -1, Structure is 1, Magician is 2, Balista is 6
    DiagonalPattern(i32),       // Bishop is -1, Leech is 1
    CirclePattern(i32),         // Archer is 4
    SquarePattern(i32),         // Cannon is 3
    KnightPattern(i32, i32),    // (dx = 2 & dy = 1) | (dx = 1 & dy = 2)

    // DirectionalPattern with Forward
    PawnMovePattern(Direction),      // 2 no Forward
    PawnTakePattern(Direction),      // 1 no ForwardDiagonals
    SuperPawnMovePattern(Direction), // 2 no Forward | ForwardDiagonals
    SuperPawnTakePattern(Direction), // 1 no Forward | ForwardDiagonals

    // RandomizablePattern
    RandomizablePattern(Box<PatternEnum>), // (pattern)
    // CrazyPawnPattern is RandomizablePattern with SubdirectionalPattern(2)

    // CompositePattern
    Composite(Vec<PatternEnum>),

    // AbilityPatterns
    PawnAbilityPattern(Direction), // is true when at the end of the board in that direction
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    patterns: Vec<PatternEnum>,
}

impl Pattern {
    pub fn new(pattern: PatternEnum) -> Self {
        Self {
            patterns: vec![pattern],
        }
    }

    pub fn new_pair(pattern1: PatternEnum, pattern2: PatternEnum) -> Self {
        Self {
            patterns: vec![pattern1, pattern2],
        }
    }

    pub fn new_many(patterns: Vec<PatternEnum>) -> Self {
        Self { patterns }
    }

    pub fn new_null() -> Self {
        Self { patterns: vec![] }
    }

    pub fn has_pattern(&self) -> bool {
        !self.patterns.is_empty()
    }

    pub fn is_null(&self) -> bool {
        self.patterns.is_empty()
    }

    pub fn matches(&self, start: Position, end: Position, board: &Board) -> bool {
        self.patterns.iter().any(|p| p.matches(start, end, board))
    }
}

impl PatternEnum {
    pub fn composite(patterns: Vec<PatternEnum>) -> Self {
        Self::Composite(patterns)
    }

    pub fn matches(&self, start: Position, end: Position, board: &Board) -> bool {
        match self {
            Self::SubdirectionalPattern(n) => {
                let dpos = end - start;
                let is_subdirectional = dpos.is_subdir();

                // If n is -1, allow infinite distance
                if *n == -1 {
                    is_subdirectional
                } else {
                    // Otherwise check if within distance n
                    is_subdirectional && dpos.jump_length() <= *n
                }
            }
            Self::DirectionalPattern(n) => {
                let dpos = end - start;

                // Check if movement is in a cardinal direction
                if !dpos.is_axial() {
                    return false;
                }

                // If n is -1, allow infinite distance
                if *n == -1 {
                    true
                } else {
                    // Otherwise check if within distance n
                    dpos.jump_length() <= *n
                }
            }
            Self::DiagonalPattern(n) => {
                let dpos = end - start;

                // Check if movement is diagonal
                if !dpos.is_diagonal() {
                    return false;
                }

                // If n is -1, allow infinite distance
                if *n == -1 {
                    true
                } else {
                    // Otherwise check if within distance n
                    dpos.jump_length() <= *n
                }
            }
            Self::CirclePattern(n) => {
                let dpos = end - start;
                dpos.length() <= *n as f32
            }
            Self::SquarePattern(n) => {
                let dpos = end - start;
                dpos.jump_length() <= *n
            }
            Self::KnightPattern(dx, dy) => {
                let dpos = end - start;
                let abs_dx = dpos.x.abs();
                let abs_dy = dpos.y.abs();

                // Check if movement matches either knight pattern
                (abs_dx == *dx && abs_dy == *dy) || (abs_dx == *dy && abs_dy == *dx)
            }
            Self::PawnMovePattern(forward) => {
                let dpos = end - start;
                dpos == forward.to_position()
            }
            Self::PawnTakePattern(forward) => {
                let dpos = end - start;
                let forward_pos = forward.to_position();
                let right_pos = Direction::Right.to_position();
                let left_pos = Direction::Left.to_position();

                dpos == (forward_pos + right_pos) || dpos == (forward_pos + left_pos)
            }
            Self::SuperPawnMovePattern(forward) => {
                let dpos = end - start;
                let forward_pos = forward.to_position();
                let right_pos = Direction::Right.to_position();
                let left_pos = Direction::Left.to_position();

                dpos == forward_pos
                    || dpos == (forward_pos + right_pos)
                    || dpos == (forward_pos + left_pos)
                    || dpos == (forward_pos + forward_pos)
                    || dpos == (forward_pos + forward_pos + right_pos)
                    || dpos == (forward_pos + forward_pos + left_pos)
            }
            Self::SuperPawnTakePattern(forward) => {
                let dpos = end - start;
                let forward_pos = forward.to_position();
                let right_pos = Direction::Right.to_position();
                let left_pos = Direction::Left.to_position();

                dpos == forward_pos
                    || dpos == (forward_pos + right_pos)
                    || dpos == (forward_pos + left_pos)
            }
            Self::RandomizablePattern(pattern) => {
                // The random numbers are between 0 and 1
                // and they generate a random direction, subdirection, or other stuff
                // the random numbers are obtained from the board state, so there is a random dir, subdir, etc for each ChessTime component (Movement, Turn, Round)
                // a subdirectional pattern will comply with the random subdirection
                // a directional pattern will comply with the random direction
                // a diagonal pattern will comply with the random diagonal
                // no other pattern is supported yet
                // For now, we'll just use the pattern's matching logic
                pattern.matches(start, end, board)
            }
            Self::Composite(patterns) => patterns.iter().any(|p| p.matches(start, end, board)),
            Self::PawnAbilityPattern(forward) => {
                // if end == start and forward is out of the board, return true
                if end == start {
                    let dpos = forward.to_position();
                    board.get_square(end + dpos).is_none()
                } else {
                    false
                }
            }
        }
    }
}

// Helper functions to create patterns
impl PatternEnum {
    // Classic Patterns
    pub fn get_king() -> Self {
        Self::SubdirectionalPattern(1)
    }

    pub fn get_rook() -> Self {
        Self::DirectionalPattern(-1)
    }

    pub fn get_bishop() -> Self {
        Self::DiagonalPattern(-1)
    }

    pub fn get_knight() -> Self {
        Self::KnightPattern(2, 1)
    }

    pub fn get_queen() -> Self {
        Self::composite(vec![Self::get_rook(), Self::get_bishop()])
    }

    pub fn get_pawn_move(forward: Direction) -> Self {
        Self::PawnMovePattern(forward)
    }

    pub fn get_pawn_take(forward: Direction) -> Self {
        Self::PawnTakePattern(forward)
    }

    // Starting Patterns
    pub fn get_structure_move() -> Self {
        Self::DirectionalPattern(1)
    }

    pub fn get_magician() -> Self {
        Self::DirectionalPattern(2)
    }

    pub fn get_balista_attack() -> Self {
        Self::DirectionalPattern(6)
    }

    pub fn get_leech_take() -> Self {
        Self::DiagonalPattern(1)
    }

    pub fn get_super_pawn_move(forward: Direction) -> Self {
        Self::SuperPawnMovePattern(forward)
    }

    pub fn get_super_pawn_take(forward: Direction) -> Self {
        Self::SuperPawnTakePattern(forward)
    }

    // Shape Patterns
    pub fn get_circle(n: i32) -> Self {
        Self::CirclePattern(n)
    }

    pub fn get_square(n: i32) -> Self {
        Self::SquarePattern(n)
    }

    pub fn get_archer_attack() -> Self {
        Self::get_circle(4)
    }

    pub fn get_gargoyle() -> Self {
        Self::get_circle(5)
    }

    pub fn get_cannon_attack() -> Self {
        Self::get_square(3)
    }

    // Other Patterns
    pub fn get_crazy_pawn() -> Self {
        Self::RandomizablePattern(Box::new(Self::SubdirectionalPattern(2)))
    }

    // Composite Patterns
    pub fn get_mermaid() -> Self {
        Self::composite(vec![Self::get_knight(), Self::get_king()])
    }

    pub fn get_oni() -> Self {
        Self::composite(vec![Self::get_knight(), Self::get_rook()])
    }

    pub fn get_witch() -> Self {
        Self::composite(vec![Self::get_rook(), Self::get_king()])
    }

    pub fn get_archer_move() -> Self {
        Self::composite(vec![Self::get_magician(), Self::get_king()])
    }

    pub fn get_pawn_ability(forward: Direction) -> Self {
        Self::PawnAbilityPattern(forward)
    }
}
