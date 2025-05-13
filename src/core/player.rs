use std::mem;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: u32,
    pub max_movements: u32,
    pub movements: u32,
    pub max_mana: u32,
    pub mana: u32,
    pub starting_hand: Deck,
    pub starting_deck: Deck,
    pub current_hand: Deck,
    pub current_deck: Deck,
    pub central_deck: Deck,
    pub discard_pile: Deck,
}

impl Player {
    pub fn new(id: u32, deck_ids: [u32; 4]) -> Self {
        Self {
            id,
            max_movements: 1,
            movements: 1,
            max_mana: 2,
            mana: 2,
            starting_hand: Deck::new_without_id(),
            starting_deck: Deck::new_without_id(),
            current_hand: Deck::new(deck_ids[0]),
            current_deck: Deck::new(deck_ids[1]),
            central_deck: Deck::new(deck_ids[2]),
            discard_pile: Deck::new(deck_ids[3]),
        }
    }

    pub fn init_decks(&mut self) {
        mem::swap(&mut self.current_hand.cards, &mut self.starting_hand.cards);
        mem::swap(&mut self.current_deck.cards, &mut self.starting_deck.cards);
    }

    pub fn on_turn_start(&mut self) {
        self.movements = self.max_movements;
        if self.mana < self.max_mana {
            self.mana += 1;
        }
    }

    pub fn can_move(&self) -> bool {
        self.movements > 0
    }

    pub fn can_use_mana(&self, cost: u32) -> bool {
        self.mana >= cost
    }

    pub fn use_movement(&mut self) -> bool {
        if self.can_move() {
            self.movements -= 1;
            true
        } else {
            false
        }
    }

    pub fn use_mana(&mut self, cost: u32) -> bool {
        if self.can_use_mana(cost) {
            self.mana -= cost;
            true
        } else {
            false
        }
    }
}
