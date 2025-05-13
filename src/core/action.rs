use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    PieceAction(PieceAction),
    CardAction(CardAction),
    EffectAction(EffectAction),
    BoardAction(BoardAction),
    OtherAction(OtherAction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BasicAction {
    Move,
    Attack,
    Take,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PieceAction {
    Move(FromToAction),
    Attack(FromToAction),
    Take(FromToAction),
    Ability(AbilityAction),
    Die(Position),  // Position of the piece that dies
    Kill(Position), // Position of the piece that kills
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FromToAction {
    pub from: Position,
    pub to: Position,
    pub piece_id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityAction {
    pub piece_id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardAction {
    TakeCard(TakeCardAction),
    PutOnBoard(PlayCardAction),
    PlayCard(PlayCardAction),
    DiscardCard(DiscardCardAction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TakeCardAction {
    pub player_id: u32,
    pub card_id: u32,
    pub from_deck_id: u32,
    pub to_deck_id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayCardAction {
    pub player_id: u32,
    pub card_id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscardCardAction {
    pub player_id: u32,
    pub card_id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectAction {
    Tick,
    Expire,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoardAction {
    Movement,
    Turn,
    Round,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OtherAction {
    Summon(Position),
}
