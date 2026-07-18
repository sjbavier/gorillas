//! Overall match state. Keep rules here instead of in rendering/input code.

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    city::City,
    config::GameConfig,
    entities::{Gorilla, Player, Sun},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScreenState {
    Intro,
}

#[derive(Debug)]
pub struct GameState {
    pub config: GameConfig,
    pub players: [Player; 2],
    pub screen: ScreenState,
    pub city: City,
    pub gorillas: [Gorilla; 2],
    pub sun: Sun,
}

impl GameState {
    pub fn new(config: GameConfig) -> Self {
        let mut rng = StdRng::from_entropy();
        Self::new_with_rng(config, &mut rng)
    }

    pub fn new_with_rng(config: GameConfig, rng: &mut impl Rng) -> Self {
        let city = City::generate(&config, rng);
        let gorillas = place_gorillas(&city, rng);
        let sun = Sun::new(config.screen_width);
        Self {
            config,
            players: [Player::new(0, "Player 1"), Player::new(1, "Player 2")],
            screen: ScreenState::Intro,
            city,
            gorillas,
            sun,
        }
    }
}

/// Place gorillas on the second or third building from each edge, matching the
/// original QBasic `PlaceGorillas` intent while storing deterministic scene data.
pub fn place_gorillas(city: &City, rng: &mut impl Rng) -> [Gorilla; 2] {
    assert!(
        city.buildings.len() >= 4,
        "city generation must produce enough buildings for gorilla placement"
    );

    let left_index = rng.gen_range(1..=2).min(city.buildings.len() - 1);
    let right_index = city
        .buildings
        .len()
        .saturating_sub(rng.gen_range(2..=3))
        .max(left_index + 1);

    [
        gorilla_on_building(0, &city.buildings[left_index]),
        gorilla_on_building(1, &city.buildings[right_index]),
    ]
}

fn gorilla_on_building(player_index: usize, building: &crate::city::Building) -> Gorilla {
    let x = building.x + building.width / 2 - Gorilla::X_ADJUST;
    let y = building.y - Gorilla::Y_ADJUST;
    Gorilla::new(player_index, x, y)
}

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, SeedableRng};

    use super::*;

    #[test]
    fn gorillas_are_placed_on_rooftops_near_opposite_edges() {
        let config = GameConfig::default();
        let mut rng = StdRng::seed_from_u64(123);
        let state = GameState::new_with_rng(config, &mut rng);

        assert!(state.city.buildings.len() >= 4);
        let left = state.gorillas[0];
        let right = state.gorillas[1];

        assert_eq!(left.player_index, 0);
        assert_eq!(right.player_index, 1);
        assert!(left.position.x < right.position.x);
        assert!(left.position.y >= 0);
        assert!(right.position.y >= 0);

        let left_on_expected_building = state.city.buildings[1..=2]
            .iter()
            .any(|building| left.position.y + Gorilla::Y_ADJUST == building.y);
        let last = state.city.buildings.len() - 1;
        let right_on_expected_building = state.city.buildings[last - 2..=last - 1]
            .iter()
            .any(|building| right.position.y + Gorilla::Y_ADJUST == building.y);

        assert!(left_on_expected_building);
        assert!(right_on_expected_building);
    }
}
