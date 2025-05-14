#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub id: u32,
    pub mana_cost: u32,
}

impl Card {
    pub fn new(id: u32, mana_cost: u32) -> Self {
        Self { id, mana_cost }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayedCard {
    pub card: Card,
    pub player_id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deck {
    pub id: u32,
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            cards: Vec::new(),
        }
    }

    /// Used for starting decks
    pub fn new_without_id() -> Self {
        Self {
            id: 0,
            cards: Vec::new(),
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn remove_card(&mut self, index: usize) -> Option<Card> {
        if index < self.cards.len() {
            Some(self.cards.remove(index))
        } else {
            None
        }
    }

    pub fn has_card(&self, card_id: u32) -> bool {
        self.cards.iter().any(|card| card.id == card_id)
    }
}
