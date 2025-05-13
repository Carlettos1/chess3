use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Property {
    Life(u32),             // Defaults to 1
    CurrentLife(u32),      // Defaults to Life
    AttackDamage(u32),     // Defaults to 1
    TakeDamage(u32),       // Defaults to 1
    PieceList(Vec<Piece>), // Defaults to empty
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    Biologic,      // BIO
    Structure,     // STR
    Transportable, // TRANS
    Impenetrable,  // IMP
    Immune,        // IMM
    Heroic,        // HERO
    Demonic,       // DEM
    Dead,          // DEAD
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    // Classic Pieces
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,

    // Starting Pieces
    Archer,
    Balista,
    Builder,
    Cannon,
    Catapult,
    CrazyPawn,
    Magician,
    Paladin,
    Ram,
    ShieldBearer,
    Ship,
    SuperPawn,
    TeslaTower,
    Wall,
    Warlock,

    // Portal Units
    Portal,
    Basilisk,
    Dragon,
    Gargoyle,
    Golem,
    Imp,
    Mandragora,
    Mermaid,
    Necromancer,
    Ogre,
    Oni,
    Spider,
    SpiderEgg,
    Succubus,
    Witch,

    // Other Units
    Swamp,
    Leech,
}

impl PieceType {
    pub fn get_properties(&self) -> Vec<Property> {
        match self {
            // Classic Pieces
            PieceType::Pawn => vec![],
            PieceType::Bishop => vec![],
            PieceType::Knight => vec![],
            PieceType::Rook => vec![],
            PieceType::Queen => vec![],
            PieceType::King => vec![],

            // Starting Pieces
            PieceType::Archer => vec![],
            PieceType::Balista => vec![],
            PieceType::Builder => vec![],
            PieceType::Cannon => vec![],
            PieceType::Catapult => vec![],
            PieceType::CrazyPawn => vec![],
            PieceType::Magician => vec![],
            PieceType::Paladin => vec![],
            PieceType::Ram => vec![],
            PieceType::ShieldBearer => vec![],
            PieceType::Ship => vec![],
            PieceType::SuperPawn => vec![],
            PieceType::TeslaTower => vec![],
            PieceType::Wall => vec![],
            PieceType::Warlock => vec![],

            // Portal Units
            PieceType::Portal => vec![],
            PieceType::Basilisk => vec![],
            PieceType::Dragon => vec![],
            PieceType::Gargoyle => vec![],
            PieceType::Golem => vec![Property::Life(4)],
            PieceType::Imp => vec![],
            PieceType::Mandragora => vec![],
            PieceType::Mermaid => vec![],
            PieceType::Necromancer => vec![Property::PieceList(vec![])],
            PieceType::Ogre => vec![Property::Life(2)],
            PieceType::Oni => vec![],
            PieceType::Spider => vec![],
            PieceType::SpiderEgg => vec![],
            PieceType::Succubus => vec![],
            PieceType::Witch => vec![],

            // Other Units
            PieceType::Swamp => vec![],
            PieceType::Leech => vec![],
        }
    }

    pub fn get_tags(&self) -> Vec<Tag> {
        match self {
            // Classic Pieces
            PieceType::Pawn => vec![Tag::Biologic, Tag::Transportable],
            PieceType::Bishop => vec![Tag::Biologic, Tag::Transportable],
            PieceType::Knight => vec![Tag::Biologic, Tag::Transportable],
            PieceType::Rook => vec![Tag::Structure],
            PieceType::Queen => vec![Tag::Biologic, Tag::Heroic],
            PieceType::King => vec![Tag::Biologic, Tag::Immune, Tag::Heroic],

            // Starting Pieces
            PieceType::Archer => vec![Tag::Biologic, Tag::Transportable],
            PieceType::Balista => vec![Tag::Structure],
            PieceType::Builder => vec![Tag::Biologic, Tag::Transportable],
            PieceType::Cannon => vec![Tag::Structure],
            PieceType::Catapult => vec![Tag::Structure],
            PieceType::CrazyPawn => vec![Tag::Biologic, Tag::Transportable],
            PieceType::Magician => {
                vec![Tag::Biologic, Tag::Heroic, Tag::Immune, Tag::Transportable]
            }
            PieceType::Paladin => vec![Tag::Heroic, Tag::Immune],
            PieceType::Ram => vec![Tag::Structure],
            PieceType::ShieldBearer => vec![Tag::Biologic, Tag::Transportable],
            PieceType::Ship => vec![Tag::Structure],
            PieceType::SuperPawn => vec![Tag::Biologic, Tag::Transportable],
            PieceType::TeslaTower => vec![Tag::Structure],
            PieceType::Wall => vec![Tag::Structure, Tag::Impenetrable],
            PieceType::Warlock => vec![Tag::Transportable, Tag::Demonic, Tag::Immune],

            // Portal Units
            PieceType::Portal => vec![Tag::Structure, Tag::Impenetrable, Tag::Immune, Tag::Heroic],
            PieceType::Basilisk => vec![Tag::Demonic, Tag::Immune],
            PieceType::Dragon => vec![Tag::Immune, Tag::Heroic, Tag::Biologic, Tag::Demonic],
            PieceType::Gargoyle => vec![Tag::Demonic],
            PieceType::Golem => vec![
                Tag::Heroic,
                Tag::Demonic,
                Tag::Immune,
                Tag::Structure,
                Tag::Impenetrable,
            ],
            PieceType::Imp => vec![Tag::Demonic, Tag::Biologic],
            PieceType::Mandragora => vec![Tag::Demonic, Tag::Biologic],
            PieceType::Mermaid => vec![Tag::Demonic, Tag::Biologic],
            PieceType::Necromancer => vec![Tag::Demonic],
            PieceType::Ogre => vec![Tag::Demonic, Tag::Impenetrable],
            PieceType::Oni => vec![Tag::Demonic, Tag::Biologic, Tag::Transportable],
            PieceType::Spider => vec![Tag::Demonic, Tag::Transportable, Tag::Biologic],
            PieceType::SpiderEgg => vec![Tag::Demonic, Tag::Biologic],
            PieceType::Succubus => vec![Tag::Biologic, Tag::Demonic, Tag::Transportable],
            PieceType::Witch => vec![Tag::Demonic],

            // Other Units
            PieceType::Swamp => vec![Tag::Structure],
            PieceType::Leech => vec![Tag::Biologic],
        }
    }

    pub fn get_move_pattern(&self) -> Pattern {
        match self {
            // Classic Pieces
            PieceType::Pawn => Pattern::new(PatternEnum::PawnMovePattern(Direction::Up)),
            PieceType::Bishop => Pattern::new(PatternEnum::DiagonalPattern(-1)),
            PieceType::Knight => Pattern::new(PatternEnum::KnightPattern(2, 1)),
            PieceType::Rook => Pattern::new(PatternEnum::DirectionalPattern(-1)),
            PieceType::Queen => Pattern::new_many(vec![
                PatternEnum::DirectionalPattern(-1),
                PatternEnum::DiagonalPattern(-1),
            ]),
            PieceType::King => Pattern::new(PatternEnum::SubdirectionalPattern(1)),

            // Starting Pieces
            PieceType::Archer => Pattern::new_many(vec![
                PatternEnum::DirectionalPattern(2),
                PatternEnum::SubdirectionalPattern(1),
            ]),
            PieceType::Balista => Pattern::new(PatternEnum::DirectionalPattern(1)),
            PieceType::Builder => Pattern::new(PatternEnum::DirectionalPattern(2)),
            PieceType::Cannon => Pattern::new(PatternEnum::DirectionalPattern(1)),
            PieceType::Catapult => Pattern::new(PatternEnum::DirectionalPattern(1)),
            PieceType::CrazyPawn => Pattern::new(PatternEnum::RandomizablePattern(Box::new(
                PatternEnum::SubdirectionalPattern(2),
            ))),
            PieceType::Magician => Pattern::new(PatternEnum::DirectionalPattern(2)),
            PieceType::Paladin => Pattern::new_many(vec![
                PatternEnum::DirectionalPattern(-1),
                PatternEnum::DiagonalPattern(-1),
            ]),
            PieceType::Ram => Pattern::new(PatternEnum::DirectionalPattern(1)),
            PieceType::ShieldBearer => Pattern::new(PatternEnum::PawnMovePattern(Direction::Up)),
            PieceType::Ship => Pattern::new(PatternEnum::DirectionalPattern(2)),
            PieceType::SuperPawn => Pattern::new(PatternEnum::SuperPawnMovePattern(Direction::Up)),
            PieceType::TeslaTower => Pattern::new(PatternEnum::DirectionalPattern(2)),
            PieceType::Wall => Pattern::new_null(),
            PieceType::Warlock => Pattern::new(PatternEnum::DirectionalPattern(2)),

            // Portal Units
            PieceType::Portal => Pattern::new_null(),
            PieceType::Basilisk => Pattern::new(PatternEnum::DiagonalPattern(-1)),
            PieceType::Dragon => Pattern::new_many(vec![
                PatternEnum::KnightPattern(2, 1),
                PatternEnum::SubdirectionalPattern(1),
            ]),
            PieceType::Gargoyle => Pattern::new(PatternEnum::CirclePattern(5)),
            PieceType::Golem => Pattern::new_many(vec![
                PatternEnum::DirectionalPattern(2),
                PatternEnum::SubdirectionalPattern(1),
            ]),
            PieceType::Imp => Pattern::new(PatternEnum::KnightPattern(2, 1)),
            PieceType::Mandragora => Pattern::new(PatternEnum::SubdirectionalPattern(1)),
            PieceType::Mermaid => Pattern::new_many(vec![
                PatternEnum::KnightPattern(2, 1),
                PatternEnum::SubdirectionalPattern(1),
            ]),
            PieceType::Necromancer => Pattern::new(PatternEnum::DirectionalPattern(2)),
            PieceType::Ogre => Pattern::new_many(vec![
                PatternEnum::DirectionalPattern(2),
                PatternEnum::SubdirectionalPattern(1),
            ]),
            PieceType::Oni => Pattern::new_many(vec![
                PatternEnum::KnightPattern(2, 1),
                PatternEnum::DirectionalPattern(-1),
            ]),
            PieceType::Spider => Pattern::new(PatternEnum::KnightPattern(2, 1)),
            PieceType::SpiderEgg => Pattern::new_null(),
            PieceType::Succubus => Pattern::new(PatternEnum::DiagonalPattern(-1)),
            PieceType::Witch => Pattern::new_many(vec![
                PatternEnum::DirectionalPattern(-1),
                PatternEnum::SubdirectionalPattern(1),
            ]),

            // Other Units
            PieceType::Swamp => Pattern::new_null(),
            PieceType::Leech => Pattern::new(PatternEnum::SubdirectionalPattern(1)),
        }
    }

    pub fn get_take_pattern(&self) -> Pattern {
        match self {
            // Classic Pieces
            PieceType::Pawn => Pattern::new(PatternEnum::PawnTakePattern(Direction::Up)),
            PieceType::Bishop => Pattern::new(PatternEnum::DiagonalPattern(-1)),
            PieceType::Knight => Pattern::new(PatternEnum::KnightPattern(2, 1)),
            PieceType::Rook => Pattern::new(PatternEnum::DirectionalPattern(-1)),
            PieceType::Queen => Pattern::new_many(vec![
                PatternEnum::DirectionalPattern(-1),
                PatternEnum::DiagonalPattern(-1),
            ]),
            PieceType::King => Pattern::new(PatternEnum::SubdirectionalPattern(1)),

            // Starting Pieces
            PieceType::Archer => Pattern::new_null(),
            PieceType::Balista => Pattern::new_null(),
            PieceType::Builder => Pattern::new(PatternEnum::DiagonalPattern(1)),
            PieceType::Cannon => Pattern::new_null(),
            PieceType::Catapult => Pattern::new_null(),
            PieceType::CrazyPawn => Pattern::new(PatternEnum::RandomizablePattern(Box::new(
                PatternEnum::SubdirectionalPattern(2),
            ))),
            PieceType::Magician => Pattern::new_null(),
            PieceType::Paladin => Pattern::new_many(vec![
                PatternEnum::DirectionalPattern(-1),
                PatternEnum::DiagonalPattern(-1),
            ]),
            PieceType::Ram => Pattern::new_null(),
            PieceType::ShieldBearer => Pattern::new(PatternEnum::PawnTakePattern(Direction::Up)),
            PieceType::Ship => Pattern::new(PatternEnum::SubdirectionalPattern(1)),
            PieceType::SuperPawn => Pattern::new(PatternEnum::SuperPawnTakePattern(Direction::Up)),
            PieceType::TeslaTower => Pattern::new(PatternEnum::DirectionalPattern(1)),
            PieceType::Wall => Pattern::new_null(),
            PieceType::Warlock => Pattern::new_null(),

            // Portal Units
            PieceType::Portal => Pattern::new_null(),
            PieceType::Basilisk => Pattern::new(PatternEnum::DiagonalPattern(-1)),
            PieceType::Dragon => Pattern::new_many(vec![
                PatternEnum::KnightPattern(2, 1),
                PatternEnum::SubdirectionalPattern(1),
            ]),
            PieceType::Gargoyle => Pattern::new(PatternEnum::CirclePattern(5)),
            PieceType::Golem => Pattern::new_many(vec![
                PatternEnum::DirectionalPattern(2),
                PatternEnum::SubdirectionalPattern(1),
            ]),
            PieceType::Imp => Pattern::new_null(),
            PieceType::Mandragora => Pattern::new_null(),
            PieceType::Mermaid => Pattern::new_many(vec![
                PatternEnum::KnightPattern(2, 1),
                PatternEnum::SubdirectionalPattern(1),
            ]),
            PieceType::Necromancer => Pattern::new_null(),
            PieceType::Ogre => Pattern::new_many(vec![
                PatternEnum::DirectionalPattern(2),
                PatternEnum::SubdirectionalPattern(1),
            ]),
            PieceType::Oni => Pattern::new_many(vec![
                PatternEnum::KnightPattern(2, 1),
                PatternEnum::DirectionalPattern(-1),
            ]),
            PieceType::Spider => Pattern::new(PatternEnum::KnightPattern(2, 1)),
            PieceType::SpiderEgg => Pattern::new_null(),
            PieceType::Succubus => Pattern::new(PatternEnum::DiagonalPattern(-1)),
            PieceType::Witch => Pattern::new_many(vec![
                PatternEnum::DirectionalPattern(-1),
                PatternEnum::SubdirectionalPattern(1),
            ]),

            // Other Units
            PieceType::Swamp => Pattern::new_null(),
            PieceType::Leech => Pattern::new(PatternEnum::DiagonalPattern(1)),
        }
    }

    pub fn get_attack_pattern(&self) -> Pattern {
        match self {
            // Classic Pieces
            PieceType::Pawn => Pattern::new_null(),
            PieceType::Bishop => Pattern::new_null(),
            PieceType::Knight => Pattern::new_null(),
            PieceType::Rook => Pattern::new_null(),
            PieceType::Queen => Pattern::new_null(),
            PieceType::King => Pattern::new_null(),

            // Starting Pieces
            PieceType::Archer => Pattern::new(PatternEnum::CirclePattern(4)),
            PieceType::Balista => Pattern::new(PatternEnum::DirectionalPattern(6)),
            PieceType::Builder => Pattern::new_null(),
            PieceType::Cannon => Pattern::new(PatternEnum::SquarePattern(3)),
            PieceType::Catapult => Pattern::new_null(),
            PieceType::CrazyPawn => Pattern::new(PatternEnum::RandomizablePattern(Box::new(
                PatternEnum::SubdirectionalPattern(2),
            ))),
            PieceType::Magician => Pattern::new_null(),
            PieceType::Paladin => Pattern::new_null(),
            PieceType::Ram => Pattern::new_null(),
            PieceType::ShieldBearer => Pattern::new_null(),
            PieceType::Ship => Pattern::new_null(),
            PieceType::SuperPawn => Pattern::new_null(),
            PieceType::TeslaTower => Pattern::new_null(),
            PieceType::Wall => Pattern::new_null(),
            PieceType::Warlock => Pattern::new_null(),

            // Portal Units
            PieceType::Portal => Pattern::new_null(),
            PieceType::Basilisk => Pattern::new_null(),
            PieceType::Dragon => Pattern::new_null(),
            PieceType::Gargoyle => Pattern::new_null(),
            PieceType::Golem => Pattern::new_null(),
            PieceType::Imp => Pattern::new_null(),
            PieceType::Mandragora => Pattern::new_null(),
            PieceType::Mermaid => Pattern::new_null(),
            PieceType::Necromancer => Pattern::new_null(),
            PieceType::Ogre => Pattern::new_null(),
            PieceType::Oni => Pattern::new_null(),
            PieceType::Spider => Pattern::new_null(),
            PieceType::SpiderEgg => Pattern::new_null(),
            PieceType::Succubus => Pattern::new_null(),
            PieceType::Witch => Pattern::new_null(),

            // Other Units
            PieceType::Swamp => Pattern::new_null(),
            PieceType::Leech => Pattern::new_null(),
        }
    }

    pub fn get_ability_data(&self) -> AbilityData {
        match self {
            // Classic Pieces
            PieceType::Pawn => AbilityData::new(
                0,
                0,
                1,
                0,
                Pattern::new(PatternEnum::PawnAbilityPattern(Direction::Up)),
            ),
            PieceType::Bishop => AbilityData::new(1, 0, 0, 2, Pattern::new_null()),
            PieceType::Knight => AbilityData::new(1, 1, 1, 10, Pattern::new_null()),
            PieceType::Rook => AbilityData::new(2, 0, 0, 10, Pattern::new_null()),
            PieceType::Queen => AbilityData::new(1, 0, 0, 2, Pattern::new_null()),
            PieceType::King => AbilityData::new(0, 0, 0, 1, Pattern::new_null()),

            // Starting Pieces
            PieceType::Archer => AbilityData::new(0, 0, 0, 0, Pattern::new_null()),
            PieceType::Balista => AbilityData::new(0, 0, 0, 0, Pattern::new_null()),
            PieceType::Builder => AbilityData::new(1, 0, 0, 10, Pattern::new_null()),
            PieceType::Cannon => AbilityData::new(0, 0, 0, 0, Pattern::new_null()),
            PieceType::Catapult => AbilityData::new(1, 0, 0, 4, Pattern::new_null()),
            PieceType::CrazyPawn => AbilityData::new(1, 0, 0, 0, Pattern::new_null()),
            PieceType::Magician => AbilityData::new(1, 2, 1, 6, Pattern::new_null()),
            PieceType::Paladin => AbilityData::new(1, 2, 1, 4, Pattern::new_null()),
            PieceType::Ram => AbilityData::new(1, 0, 0, 4, Pattern::new_null()),
            PieceType::ShieldBearer => AbilityData::new(1, 0, 0, 15, Pattern::new_null()),
            PieceType::Ship => AbilityData::new(1, 0, 0, 12, Pattern::new_null()),
            PieceType::SuperPawn => AbilityData::new(1, 0, 0, 10, Pattern::new_null()),
            PieceType::TeslaTower => AbilityData::new(2, 0, 0, 10, Pattern::new_null()),
            PieceType::Wall => AbilityData::new(0, 0, 0, 0, Pattern::new_null()),
            PieceType::Warlock => AbilityData::new(1, 3, 1, 5, Pattern::new_null()),

            // Portal Units
            PieceType::Portal => AbilityData::new(1, 0, 0, 0, Pattern::new_null()),
            PieceType::Basilisk => AbilityData::new(1, 0, 0, 20, Pattern::new_null()),
            PieceType::Dragon => AbilityData::new(2, 1, 0, 2, Pattern::new_null()),
            PieceType::Gargoyle => AbilityData::new(0, 0, 0, 0, Pattern::new_null()),
            PieceType::Golem => AbilityData::new(0, 0, 0, 0, Pattern::new_null()),
            PieceType::Imp => AbilityData::new(1, 2, 0, 5, Pattern::new_null()),
            PieceType::Mandragora => AbilityData::new(1, 0, 0, 10, Pattern::new_null()),
            PieceType::Mermaid => AbilityData::new(1, 2, 0, 5, Pattern::new_null()),
            PieceType::Necromancer => AbilityData::new(2, 1, 0, 4, Pattern::new_null()),
            PieceType::Ogre => AbilityData::new(1, 0, 0, 4, Pattern::new_null()),
            PieceType::Oni => AbilityData::new(1, 1, 0, 7, Pattern::new_null()),
            PieceType::Spider => AbilityData::new(1, 1, 0, 12, Pattern::new_null()),
            PieceType::SpiderEgg => AbilityData::new(0, 0, 0, 0, Pattern::new_null()),
            PieceType::Succubus => AbilityData::new(1, 1, 0, 10, Pattern::new_null()),
            PieceType::Witch => AbilityData::new(1, 2, 0, 8, Pattern::new_null()),

            // Other Units
            PieceType::Swamp => AbilityData::new(1, 0, 0, 5, Pattern::new_null()),
            PieceType::Leech => AbilityData::new(1, 2, 0, 8, Pattern::new_null()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    pub id: u32,
    pub piece_type: PieceType,
    pub properties: Vec<Property>,
    pub tags: Vec<Tag>,
    pub move_pattern: Pattern,
    pub take_pattern: Pattern,
    pub attack_pattern: Pattern,
    pub ability_data: AbilityData,
}

impl Piece {
    pub fn new(id: u32, piece_type: PieceType) -> Self {
        Self {
            id,
            piece_type,
            properties: piece_type.get_properties(),
            tags: piece_type.get_tags(),
            move_pattern: piece_type.get_move_pattern(),
            take_pattern: piece_type.get_take_pattern(),
            attack_pattern: piece_type.get_attack_pattern(),
            ability_data: piece_type.get_ability_data(),
        }
    }

    pub fn can_do_action(&self, action: &Action, board: &Board) -> bool {
        true // TODO, it should check if effects, tags, etc allow the action
    }

    pub fn can_receive_action(&self, action: &Action, board: &Board) -> bool {
        true // TODO, it should check if effects, tags, etc allow the action
    }

    pub fn on_action_done(&mut self, action: &Action, board: &Board) {
        // TODO, it should trigger effects, tags, etc when done the action
    }

    pub fn on_action_received(&mut self, action: &Action, board: &Board) {
        // TODO, it should trigger effects, tags, etc when received the action
    }

    pub fn on_tick(&mut self, board: &mut Board) {
        // TODO, it should trigger effects, tags, etc when the piece is on the board
    }

    pub fn on_expire(&mut self, board: &mut Board) {
        // TODO, it should trigger effects, tags, etc when the piece is expired
    }
}
