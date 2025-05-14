use derive_more::{Deref, DerefMut};

use crate::prelude::*;

#[derive(Debug, Deref, DerefMut, Clone, PartialEq, Eq)]
pub struct AppliedEffect {
    #[deref]
    #[deref_mut]
    pub effect: Effect,
    pub duration: ChessTime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    Fire,
    Ice,
    Deactivate,
    Invulnerable,
}

#[allow(clippy::single_match)]
impl Effect {
    /// Called when the effect expires
    pub fn on_expire(&self, board: &mut Board, piece: &Piece) {
        match self {
            Effect::Fire => {
                board.kill_piece(piece.id);
            }
            _ => {}
        }
    }

    /// Ticks every init of a turn
    pub fn on_tick(&self, board: &mut Board, piece: &Piece) {
        match self {
            Effect::Ice | Effect::Deactivate => {
                board.set_piece_moved(piece.id, true);
            }
            _ => {}
        }
    }

    /// Called when the effect is applied
    pub fn on_apply(&self, board: &mut Board, piece: &Piece) {
        match self {
            Effect::Ice if piece.is_str() => {
                board.halve_effect(piece.id, Effect::Ice);
            }
            Effect::Deactivate if !piece.is_str() => {
                board.halve_effect(piece.id, Effect::Deactivate);
            }
            _ => {}
        }
    }

    pub fn on_action_done(&self, _board: &mut Board, _piece: &Piece) {}
    pub fn on_action_received(&self, _board: &mut Board, _piece: &Piece) {}

    pub fn can_action_be_done(&self, _board: &Board, _piece: &Piece) -> bool {
        true
    }
    pub fn can_action_be_received(&self, _board: &Board, _piece: &Piece) -> bool {
        !matches!(self, Effect::Invulnerable)
    }
}
