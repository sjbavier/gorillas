//! Overall match state. Keep rules here instead of in rendering/input code.

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    city::City,
    config::GameConfig,
    entities::{ArmPose, Gorilla, Player, ShotResult, Sun, SunMood},
    physics::{self, ShotParams, ShotResolution, TrajectoryPoint, TIME_STEP},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScreenState {
    Intro,
}

#[derive(Clone, Debug)]
pub struct ActiveShot {
    pub thrower_index: usize,
    pub samples: Vec<TrajectoryPoint>,
    pub current_sample: usize,
    pub resolution: ShotResolution,
    post_impact_frames: u8,
}

impl ActiveShot {
    const IMPACT_HOLD_FRAMES: u8 = 20;

    pub fn visible_sample(&self) -> Option<TrajectoryPoint> {
        self.samples.get(self.current_sample).copied()
    }

    pub fn is_at_last_sample(&self) -> bool {
        self.current_sample + 1 >= self.samples.len()
    }
}

#[derive(Debug)]
pub struct GameState {
    pub config: GameConfig,
    pub players: [Player; 2],
    pub screen: ScreenState,
    pub city: City,
    pub gorillas: [Gorilla; 2],
    pub sun: Sun,
    pub current_turn: usize,
    pub active_shot: Option<ActiveShot>,
    pub last_shot: Option<ShotResolution>,
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
            current_turn: 0,
            active_shot: None,
            last_shot: None,
        }
    }

    /// Start a rendering-independent shot animation from a player command.
    ///
    /// This is the first bridge between pure projectile resolution and the local
    /// view. Later input/network code can call this with submitted shot commands
    /// instead of having rendering own turn rules.
    pub fn submit_shot(&mut self, player_index: usize, angle_degrees: f32, velocity: f32) -> bool {
        if player_index >= self.gorillas.len()
            || player_index != self.current_turn
            || self.active_shot.is_some()
        {
            return false;
        }

        self.sun.mood = SunMood::Happy;
        self.gorillas[player_index].pose = if player_index == 0 {
            ArmPose::RightUp
        } else {
            ArmPose::LeftUp
        };

        let start = physics::banana_spawn(&self.gorillas[player_index]);
        let world_angle = physics::world_angle_degrees(player_index, angle_degrees);
        let params = ShotParams::new(start, world_angle, velocity, self.city.wind, &self.config);
        let resolution = physics::resolve_shot(
            player_index,
            params,
            &self.config,
            &self.city,
            &self.gorillas,
            &self.sun,
        );
        let samples = samples_through_resolution(params, &self.config, resolution);

        self.last_shot = Some(resolution);
        self.active_shot = Some(ActiveShot {
            thrower_index: player_index,
            samples,
            current_sample: 0,
            resolution,
            post_impact_frames: 0,
        });
        true
    }

    pub fn update_animation(&mut self) {
        let Some(active_shot) = &mut self.active_shot else {
            return;
        };

        self.gorillas[active_shot.thrower_index].pose = ArmPose::Down;

        if let Some(sample) = active_shot.visible_sample() {
            if sample.position.y > 0.0 && self.sun.contains(sample.position) {
                self.sun.mood = SunMood::Shocked;
            }
        }

        if active_shot.is_at_last_sample() {
            active_shot.post_impact_frames += 1;
            if active_shot.post_impact_frames >= ActiveShot::IMPACT_HOLD_FRAMES {
                let thrower_index = active_shot.thrower_index;
                let resolution = active_shot.resolution;
                self.sun.mood = SunMood::Happy;
                self.current_turn = 1 - self.current_turn;
                self.active_shot = None;

                if self.apply_scoring_result(thrower_index, resolution.result) {
                    self.start_next_round();
                }
            }
        } else {
            active_shot.current_sample += 1;
        }
    }

    fn apply_scoring_result(&mut self, thrower_index: usize, result: ShotResult) -> bool {
        let Some(scoring_player) = scoring_player_for_result(thrower_index, result) else {
            return false;
        };

        self.players[scoring_player].score += 1;
        true
    }

    fn start_next_round(&mut self) {
        let mut rng = StdRng::from_entropy();
        self.city = City::generate(&self.config, &mut rng);
        self.gorillas = place_gorillas(&self.city, &mut rng);
        self.sun = Sun::new(self.config.screen_width);
    }
}

pub fn scoring_player_for_result(thrower_index: usize, result: ShotResult) -> Option<usize> {
    match result {
        ShotResult::Miss => None,
        ShotResult::HitPlayer(_) => Some(thrower_index),
        ShotResult::HitSelf => Some(1 - thrower_index),
    }
}

fn samples_through_resolution(
    params: ShotParams,
    config: &GameConfig,
    resolution: ShotResolution,
) -> Vec<TrajectoryPoint> {
    let end_time = resolution
        .impact
        .map(|impact| impact.sample.time)
        .unwrap_or_else(|| {
            physics::simulate_until_off_screen(params, config)
                .last()
                .map(|sample| sample.time)
                .unwrap_or(0.0)
        });

    let mut samples = Vec::new();
    let mut time = 0.0;
    while time <= end_time + f32::EPSILON {
        samples.push(physics::trajectory_at(params, time));
        time += TIME_STEP;
    }

    if samples.is_empty() {
        samples.push(physics::trajectory_at(params, 0.0));
    }
    samples
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

    #[test]
    fn submit_shot_creates_animation_and_records_resolution() {
        let config = GameConfig::default();
        let mut rng = StdRng::seed_from_u64(123);
        let mut state = GameState::new_with_rng(config, &mut rng);

        assert!(state.submit_shot(0, 45.0, 1.0));

        let shot = state.active_shot.as_ref().expect("active shot");
        assert_eq!(shot.thrower_index, 0);
        assert!(!shot.samples.is_empty());
        assert_eq!(state.last_shot, Some(shot.resolution));
    }

    #[test]
    fn score_mapping_matches_qbasic_update_scores() {
        assert_eq!(scoring_player_for_result(0, ShotResult::Miss), None);
        assert_eq!(
            scoring_player_for_result(0, ShotResult::HitPlayer(1)),
            Some(0)
        );
        assert_eq!(
            scoring_player_for_result(1, ShotResult::HitPlayer(0)),
            Some(1)
        );
        assert_eq!(scoring_player_for_result(0, ShotResult::HitSelf), Some(1));
        assert_eq!(scoring_player_for_result(1, ShotResult::HitSelf), Some(0));
    }

    #[test]
    fn scored_shot_updates_score_and_starts_fresh_round() {
        let config = GameConfig::default();
        let mut rng = StdRng::seed_from_u64(123);
        let mut state = GameState::new_with_rng(config, &mut rng);
        let original_city = state.city.clone();
        let params = ShotParams::new(
            crate::entities::Point::new(500.0, 290.0),
            0.0,
            30.0,
            0,
            &config,
        );
        let resolution =
            physics::resolve_shot(0, params, &config, &state.city, &state.gorillas, &state.sun);
        state.active_shot = Some(ActiveShot {
            thrower_index: 0,
            samples: vec![physics::trajectory_at(params, 0.0)],
            current_sample: 0,
            resolution: ShotResolution {
                result: ShotResult::HitPlayer(1),
                ..resolution
            },
            post_impact_frames: ActiveShot::IMPACT_HOLD_FRAMES - 1,
        });

        state.update_animation();

        assert_eq!(state.players[0].score, 1);
        assert_eq!(state.players[1].score, 0);
        assert!(state.active_shot.is_none());
        assert_ne!(state.city, original_city);
    }

    #[test]
    fn animation_shocks_and_resets_sun_after_shot_finishes() {
        let config = GameConfig::default();
        let mut rng = StdRng::seed_from_u64(123);
        let mut state = GameState::new_with_rng(config, &mut rng);
        let params = ShotParams::new(
            crate::entities::Point::new(state.sun.center.0 as f32, state.sun.center.1 as f32),
            0.0,
            10.0,
            0,
            &config,
        );
        let resolution =
            physics::resolve_shot(0, params, &config, &state.city, &state.gorillas, &state.sun);
        state.active_shot = Some(ActiveShot {
            thrower_index: 0,
            samples: samples_through_resolution(params, &config, resolution),
            current_sample: 0,
            resolution,
            post_impact_frames: 0,
        });

        state.update_animation();
        assert_eq!(state.sun.mood, SunMood::Shocked);

        for _ in 0..10_000 {
            if state.active_shot.is_none() {
                break;
            }
            state.update_animation();
        }

        assert!(state.active_shot.is_none());
        assert_eq!(state.sun.mood, SunMood::Happy);
    }
}
