use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Event {
    pub id: u32,
    pub player_id: u32,
    pub when: ChessTime,
    pub event_function: EventFunctionEnum,
}

#[derive(Debug, Clone)]
pub enum EventFunctionEnum {
    Summon(Piece),
    AddMana,
    AddMovement,
    ApplyEffect(Position),
}

impl Event {
    pub fn new(
        id: u32,
        player_id: u32,
        when: ChessTime,
        event_function: EventFunctionEnum,
    ) -> Self {
        Self {
            id,
            player_id,
            when,
            event_function,
        }
    }
}
