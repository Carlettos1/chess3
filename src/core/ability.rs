use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityData {
    pub mana_cost: u32,
    pub cooldown: u32,
    pub movement_cost: u32,
    pub cast_time: u32,
    pub pattern: Pattern,
}

impl AbilityData {
    pub fn new(
        mana_cost: u32,
        cooldown: u32,
        movement_cost: u32,
        cast_time: u32,
        pattern: Pattern,
    ) -> Self {
        Self {
            mana_cost,
            cooldown,
            movement_cost,
            cast_time,
            pattern,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ability {
    pub id: u32,
    pub ability_type: AbilityType,
    pub data: AbilityData,
}

impl Ability {
    pub fn new(id: u32, ability_type: AbilityType, data: AbilityData) -> Self {
        Self {
            id,
            ability_type,
            data,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbilityType {
    PawnAbility,
    BishopAbility,
    KnightAbility,
    RookAbility,
    QueenAbility,
    KingAbility,
}

// TODO the rest
