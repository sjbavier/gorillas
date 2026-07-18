//! Overall match state. Keep rules here instead of in rendering/input code.

use crate::{config::GameConfig, entities::Player};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScreenState {
    Intro,
}

#[derive(Debug)]
pub struct GameState {
    pub config: GameConfig,
    pub players: [Player; 2],
    pub screen: ScreenState,
}

impl GameState {
    pub fn new(config: GameConfig) -> Self {
        Self {
            config,
            players: [Player::new(0, "Player 1"), Player::new(1, "Player 2")],
            screen: ScreenState::Intro,
        }
    }
}
