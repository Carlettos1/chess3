use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Square {
    pub magic: bool,
    pub pos: Position,
    pub piece: Option<Piece>,
}

impl Square {
    pub fn new(pos: Position) -> Self {
        Self {
            magic: false,
            pos,
            piece: None,
        }
    }
}

#[derive(Debug)]
pub struct Board {
    pub board: Vec<Vec<Square>>,
    pub death_pile: Vec<Piece>,
    pub common_deck: Vec<PlayedCard>,
    pub cards_on_board: Vec<PlayedCard>,
    pub players: Vec<Player>,
    pub directions: Vec<Direction>,
    pub time: ChessTime,
    pub events: Vec<Event>,
    pub player_id_generator: u32,
    pub card_id_generator: u32,
    pub piece_id_generator: u32,
    pub event_id_generator: u32,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let mut board = Vec::with_capacity(height);
        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                row.push(Square::new(Position::new(x as i32, y as i32)));
            }
            board.push(row);
        }

        Self {
            board,
            death_pile: Vec::new(),
            common_deck: Vec::new(),
            cards_on_board: Vec::new(),
            players: Vec::new(),
            directions: Vec::new(),
            time: ChessTime::new(),
            events: Vec::new(),
            player_id_generator: 0,
            card_id_generator: 0,
            piece_id_generator: 0,
            event_id_generator: 0,
        }
    }

    pub fn get_square(&self, pos: Position) -> Option<&Square> {
        if pos.x >= 0 && pos.y >= 0 {
            self.board.get(pos.y as usize)?.get(pos.x as usize)
        } else {
            None
        }
    }

    pub fn get_square_mut(&mut self, pos: Position) -> Option<&mut Square> {
        if pos.x >= 0 && pos.y >= 0 {
            self.board.get_mut(pos.y as usize)?.get_mut(pos.x as usize)
        } else {
            None
        }
    }

    // id stuff
    pub fn generate_player_id(&mut self) -> u32 {
        self.player_id_generator += 1;
        self.player_id_generator
    }

    pub fn generate_card_id(&mut self) -> u32 {
        self.card_id_generator += 1;
        self.card_id_generator
    }

    pub fn generate_piece_id(&mut self) -> u32 {
        self.piece_id_generator += 1;
        self.piece_id_generator
    }

    pub fn generate_event_id(&mut self) -> u32 {
        self.event_id_generator += 1;
        self.event_id_generator
    }

    pub fn get_player_mut(&mut self, id: u32) -> Option<&mut Player> {
        self.players.iter_mut().find(|player| player.id == id)
    }

    pub fn get_played_card(&self, id: u32) -> Option<&PlayedCard> {
        self.common_deck.iter().find(|card| card.card.id == id)
    }

    pub fn get_played_card_mut(&mut self, id: u32) -> Option<&mut PlayedCard> {
        self.common_deck.iter_mut().find(|card| card.card.id == id)
    }

    pub fn get_piece(&self, id: u32) -> Option<&Piece> {
        self.death_pile.iter().find(|piece| piece.id == id)
    }

    pub fn get_piece_mut(&mut self, id: u32) -> Option<&mut Piece> {
        self.death_pile.iter_mut().find(|piece| piece.id == id)
    }

    pub fn get_event(&self, id: u32) -> Option<&Event> {
        self.events.iter().find(|event| event.id == id)
    }

    pub fn get_event_mut(&mut self, id: u32) -> Option<&mut Event> {
        self.events.iter_mut().find(|event| event.id == id)
    }

    // pattern stuff
    #[inline]
    pub fn get_squares(&self, from: Position, pattern: &Pattern) -> Vec<&Square> {
        self.board
            .iter()
            .flatten()
            .filter(|square| pattern.matches(from, square.pos, self))
            .collect()
    }

    #[inline]
    pub fn get_positions(&self, from: Position, pattern: &Pattern) -> Vec<Position> {
        self.get_squares(from, pattern)
            .iter()
            .map(|square| square.pos)
            .collect()
    }

    #[inline]
    pub fn get_squares_with_action(
        &self,
        from: Position,
        pattern: &Pattern,
        action: BasicAction,
    ) -> Vec<&Square> {
        self.get_squares(from, pattern)
            .into_iter()
            .filter(|s| match action {
                BasicAction::Move => s.piece.is_none(),
                BasicAction::Attack => s.piece.is_some(),
                BasicAction::Take => s.piece.is_some(),
                BasicAction::Ability => true,
            })
            .collect()
    }

    #[inline]
    pub fn get_positions_with_action(
        &self,
        from: Position,
        pattern: &Pattern,
        action: BasicAction,
    ) -> Vec<Position> {
        self.get_squares_with_action(from, pattern, action)
            .iter()
            .map(|s| s.pos)
            .collect()
    }

    // Piece stuff
    pub fn kill_piece(&mut self, id: u32) {
        if let Some(piece) = self.get_piece_mut(id) {
            piece.alive = false;
        }
    }

    pub fn set_piece_moved(&mut self, id: u32, moved: bool) {
        if let Some(piece) = self.get_piece_mut(id) {
            piece.moved = moved;
        }
    }

    pub fn halve_effect(&mut self, id: u32, effect: Effect) {
        if let Some(piece) = self.get_piece_mut(id) {
            if let Some(effect) = piece.effects.iter_mut().find(|e| e.effect == effect) {
                effect.duration /= 2;
            }
        }
    }

    pub fn double_effect(&mut self, id: u32, effect: Effect) {
        if let Some(piece) = self.get_piece_mut(id) {
            if let Some(effect) = piece.effects.iter_mut().find(|e| e.effect == effect) {
                effect.duration *= 2;
            }
        }
    }

    pub fn add_effect(&mut self, id: u32, effect: Effect, duration: ChessTime) {
        if let Some(piece) = self.get_piece_mut(id) {
            piece.effects.push(AppliedEffect { effect, duration });
        }
    }

    pub fn remove_effect(&mut self, id: u32, effect: Effect) {
        if let Some(piece) = self.get_piece_mut(id) {
            piece.effects.retain(|e| e.effect != effect);
        }
    }

    // Player stuff
    pub fn add_mana(&mut self, player_id: u32, amount: u32) {
        if let Some(player) = self.get_player_mut(player_id) {
            player.mana += amount;
        }
    }

    pub fn add_mana_to_color(&mut self, color: Color, amount: u32) {
        if let Some(player) = self.get_player_of_color_mut(color) {
            player.mana += amount;
        }
    }

    pub fn remove_mana(&mut self, player_id: u32, amount: u32) {
        if let Some(player) = self.get_player_mut(player_id) {
            player.mana -= amount;
        }
    }

    pub fn remove_mana_from_color(&mut self, color: Color, amount: u32) {
        if let Some(player) = self.get_player_of_color_mut(color) {
            player.mana -= amount;
        }
    }

    pub fn add_movement(&mut self, player_id: u32, amount: u32) {
        if let Some(player) = self.get_player_mut(player_id) {
            player.movements += amount;
        }
    }

    pub fn add_movement_to_color(&mut self, color: Color, amount: u32) {
        if let Some(player) = self.get_player_of_color_mut(color) {
            player.movements += amount;
        }
    }

    pub fn remove_movement(&mut self, player_id: u32, amount: u32) {
        if let Some(player) = self.get_player_mut(player_id) {
            player.movements -= amount;
        }
    }

    pub fn remove_movement_from_color(&mut self, color: Color, amount: u32) {
        if let Some(player) = self.get_player_of_color_mut(color) {
            player.movements -= amount;
        }
    }

    pub fn get_player_of_color(&self, color: Color) -> Option<&Player> {
        self.players.iter().find(|player| player.color == color)
    }

    pub fn get_player_of_color_mut(&mut self, color: Color) -> Option<&mut Player> {
        self.players.iter_mut().find(|player| player.color == color)
    }
}
