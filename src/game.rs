//! Overall match state. Keep rules here instead of in rendering/input code.

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    city::City,
    config::GameConfig,
    entities::{ArmPose, Gorilla, Player, ShotResult, Sun, SunMood},
    physics::{self, CollisionKind, ShotParams, ShotResolution, TrajectoryPoint, TIME_STEP},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScreenState {
    Intro,
    Setup,
    Menu,
    Playing,
    GameOver,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AudioCue {
    Throw,
    Explosion,
    GorillaExplosion,
    Victory,
}

pub const ANIMATION_FRAME_SECONDS: f32 = 0.02;
const MAX_ANIMATION_STEPS_PER_UPDATE: u8 = 8;

#[derive(Clone, Debug)]
pub struct ActiveShot {
    pub thrower_index: usize,
    pub samples: Vec<TrajectoryPoint>,
    pub current_sample: usize,
    pub resolution: ShotResolution,
    post_impact_frames: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VictoryDance {
    pub winner_index: usize,
    frames_remaining: u8,
    frame: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GorillaExplosion {
    pub victim_index: usize,
    pub scoring_player_index: usize,
    frames_remaining: u8,
    frame: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShotExplosion {
    pub position: crate::entities::Point,
    frames_remaining: u8,
    frame: u8,
}

impl VictoryDance {
    const TOTAL_FRAMES: u8 = 48;
    const POSE_TOGGLE_FRAMES: u8 = 8;

    fn new(winner_index: usize) -> Self {
        Self {
            winner_index,
            frames_remaining: Self::TOTAL_FRAMES,
            frame: 0,
        }
    }

    fn pose(self) -> ArmPose {
        if (self.frame / Self::POSE_TOGGLE_FRAMES) % 2 == 0 {
            ArmPose::LeftUp
        } else {
            ArmPose::RightUp
        }
    }

    fn advance(&mut self) -> bool {
        self.frame = self.frame.saturating_add(1);
        self.frames_remaining = self.frames_remaining.saturating_sub(1);
        self.frames_remaining == 0
    }
}

impl GorillaExplosion {
    pub const TOTAL_FRAMES: u8 = 36;
    pub const GROW_FRAMES: u8 = 12;

    fn new(victim_index: usize, scoring_player_index: usize) -> Self {
        Self {
            victim_index,
            scoring_player_index,
            frames_remaining: Self::TOTAL_FRAMES,
            frame: 0,
        }
    }

    pub fn frame(self) -> u8 {
        self.frame
    }

    fn advance(&mut self) -> bool {
        self.frame = self.frame.saturating_add(1);
        self.frames_remaining = self.frames_remaining.saturating_sub(1);
        self.frames_remaining == 0
    }
}

impl ShotExplosion {
    pub const TOTAL_FRAMES: u8 = 18;
    pub const MAX_RADIUS: i32 = 7;

    fn new(position: crate::entities::Point) -> Self {
        Self {
            position,
            frames_remaining: Self::TOTAL_FRAMES,
            frame: 0,
        }
    }

    pub fn frame(self) -> u8 {
        self.frame
    }

    fn advance(&mut self) -> bool {
        self.frame = self.frame.saturating_add(1);
        self.frames_remaining = self.frames_remaining.saturating_sub(1);
        self.frames_remaining == 0
    }
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
    pub setup_completed: bool,
    pub round_limit: u32,
    pub completed_rounds: u32,
    pub city: City,
    pub gorillas: [Gorilla; 2],
    pub sun: Sun,
    pub current_turn: usize,
    pub active_shot: Option<ActiveShot>,
    pub victory_dance: Option<VictoryDance>,
    pub gorilla_explosion: Option<GorillaExplosion>,
    pub shot_explosion: Option<ShotExplosion>,
    pub last_shot: Option<ShotResolution>,
    animation_accumulator_seconds: f32,
    pending_audio_cues: Vec<AudioCue>,
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
            screen: ScreenState::Playing,
            setup_completed: true,
            round_limit: 3,
            completed_rounds: 0,
            city,
            gorillas,
            sun,
            current_turn: 0,
            active_shot: None,
            victory_dance: None,
            gorilla_explosion: None,
            shot_explosion: None,
            last_shot: None,
            animation_accumulator_seconds: 0.0,
            pending_audio_cues: Vec::new(),
        }
    }

    /// Start a rendering-independent shot animation from a player command.
    ///
    /// This is the first bridge between pure projectile resolution and the local
    /// view. Later input/network code can call this with submitted shot commands
    /// instead of having rendering own turn rules.
    pub fn drain_audio_cues(&mut self) -> Vec<AudioCue> {
        self.pending_audio_cues.drain(..).collect()
    }

    fn queue_audio(&mut self, cue: AudioCue) {
        self.pending_audio_cues.push(cue);
    }

    pub fn submit_shot(&mut self, player_index: usize, angle_degrees: f32, velocity: f32) -> bool {
        if self.screen != ScreenState::Playing
            || player_index >= self.gorillas.len()
            || player_index != self.current_turn
            || !self.accepts_shot_input()
        {
            return false;
        }

        self.animation_accumulator_seconds = 0.0;
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
        self.queue_audio(AudioCue::Throw);
        self.active_shot = Some(ActiveShot {
            thrower_index: player_index,
            samples,
            current_sample: 0,
            resolution,
            post_impact_frames: 0,
        });
        true
    }

    pub fn continue_intro(&mut self) {
        self.screen = if self.setup_completed {
            ScreenState::Menu
        } else {
            ScreenState::Setup
        };
    }

    pub fn apply_setup(
        &mut self,
        player1_name: impl Into<String>,
        player2_name: impl Into<String>,
        round_limit: u32,
        gravity: f32,
    ) {
        self.players[0].name = player1_name.into();
        self.players[1].name = player2_name.into();
        self.players[0].score = 0;
        self.players[1].score = 0;
        self.round_limit = round_limit.max(1);
        self.config.gravity = gravity;
        self.completed_rounds = 0;
        self.setup_completed = true;
        self.screen = ScreenState::Menu;
    }

    pub fn view_intro(&mut self) {
        if self.screen == ScreenState::Menu {
            self.screen = ScreenState::Intro;
        }
    }

    pub fn start_match(&mut self) {
        if self.screen != ScreenState::Menu {
            return;
        }
        self.players[0].score = 0;
        self.players[1].score = 0;
        self.completed_rounds = 0;
        self.current_turn = 0;
        self.active_shot = None;
        self.victory_dance = None;
        self.gorilla_explosion = None;
        self.shot_explosion = None;
        self.last_shot = None;
        self.animation_accumulator_seconds = 0.0;
        self.start_next_round();
    }

    pub fn accepts_shot_input(&self) -> bool {
        self.screen == ScreenState::Playing
            && self.active_shot.is_none()
            && self.victory_dance.is_none()
            && self.gorilla_explosion.is_none()
            && self.shot_explosion.is_none()
    }

    #[allow(dead_code)]
    pub fn update_animation(&mut self) {
        self.update_animation_with_delta(ANIMATION_FRAME_SECONDS);
    }

    pub fn update_animation_with_delta(&mut self, delta_seconds: f32) {
        if self.screen != ScreenState::Playing {
            self.animation_accumulator_seconds = 0.0;
            return;
        }

        self.animation_accumulator_seconds += delta_seconds.max(0.0);
        let mut steps = 0;
        while self.animation_accumulator_seconds >= ANIMATION_FRAME_SECONDS
            && steps < MAX_ANIMATION_STEPS_PER_UPDATE
        {
            self.animation_accumulator_seconds -= ANIMATION_FRAME_SECONDS;
            self.advance_animation_step();
            steps += 1;
            if self.screen != ScreenState::Playing || self.accepts_shot_input() {
                self.animation_accumulator_seconds = 0.0;
                break;
            }
        }
    }

    fn advance_animation_step(&mut self) {
        if let Some(victory_dance) = &mut self.victory_dance {
            let winner_index = victory_dance.winner_index;
            self.gorillas[winner_index].pose = victory_dance.pose();
            if victory_dance.advance() {
                self.victory_dance = None;
                self.start_next_round();
            }
            return;
        }

        if let Some(gorilla_explosion) = &mut self.gorilla_explosion {
            let scoring_player_index = gorilla_explosion.scoring_player_index;
            self.gorillas[gorilla_explosion.victim_index].pose = ArmPose::Down;
            if gorilla_explosion.advance() {
                self.gorilla_explosion = None;
                self.players[scoring_player_index].score += 1;
                self.completed_rounds += 1;
                self.begin_victory_dance(scoring_player_index);
            }
            return;
        }

        if let Some(shot_explosion) = &mut self.shot_explosion {
            if shot_explosion.advance() {
                self.shot_explosion = None;
            }
            return;
        }

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

                if let Some((victim_index, scoring_player_index)) =
                    gorilla_explosion_for_result(thrower_index, resolution.result)
                {
                    self.begin_gorilla_explosion(victim_index, scoring_player_index);
                } else if let Some(position) = shot_explosion_position(resolution) {
                    self.begin_shot_explosion(position);
                }
            }
        } else {
            active_shot.current_sample += 1;
        }
    }

    fn begin_gorilla_explosion(&mut self, victim_index: usize, scoring_player_index: usize) {
        self.gorillas[victim_index].pose = ArmPose::Down;
        self.queue_audio(AudioCue::GorillaExplosion);
        self.gorilla_explosion = Some(GorillaExplosion::new(victim_index, scoring_player_index));
    }

    fn begin_shot_explosion(&mut self, position: crate::entities::Point) {
        self.queue_audio(AudioCue::Explosion);
        self.shot_explosion = Some(ShotExplosion::new(position));
    }

    fn begin_victory_dance(&mut self, winner_index: usize) {
        self.queue_audio(AudioCue::Victory);
        self.gorillas[winner_index].pose = ArmPose::LeftUp;
        self.gorillas[1 - winner_index].pose = ArmPose::Down;
        self.victory_dance = Some(VictoryDance::new(winner_index));
    }

    fn start_next_round(&mut self) {
        if self.completed_rounds >= self.round_limit {
            self.screen = ScreenState::GameOver;
            return;
        }

        let mut rng = StdRng::from_entropy();
        self.city = City::generate(&self.config, &mut rng);
        self.gorillas = place_gorillas(&self.city, &mut rng);
        self.sun = Sun::new(self.config.screen_width);
        self.animation_accumulator_seconds = 0.0;
        self.gorillas
            .iter_mut()
            .for_each(|gorilla| gorilla.pose = ArmPose::Down);
        self.screen = ScreenState::Playing;
    }

    pub fn final_score_lines(&self) -> [String; 2] {
        [
            format!("{}        {}", self.players[0].name, self.players[0].score),
            format!("{}        {}", self.players[1].name, self.players[1].score),
        ]
    }

    pub fn continue_after_game_over(&mut self) {
        if self.screen != ScreenState::GameOver {
            return;
        }

        self.active_shot = None;
        self.victory_dance = None;
        self.gorilla_explosion = None;
        self.shot_explosion = None;
        self.last_shot = None;
        self.animation_accumulator_seconds = 0.0;
        self.completed_rounds = 0;
        self.setup_completed = false;
        self.screen = ScreenState::Setup;
    }
}

#[allow(dead_code)]
pub fn scoring_player_for_result(thrower_index: usize, result: ShotResult) -> Option<usize> {
    gorilla_explosion_for_result(thrower_index, result)
        .map(|(_, scoring_player_index)| scoring_player_index)
}

#[allow(dead_code)]
pub fn gorilla_explosion_for_result(
    thrower_index: usize,
    result: ShotResult,
) -> Option<(usize, usize)> {
    match result {
        ShotResult::Miss => None,
        ShotResult::HitPlayer(victim_index) => Some((victim_index, thrower_index)),
        ShotResult::HitSelf => Some((thrower_index, 1 - thrower_index)),
    }
}

pub fn shot_explosion_position(resolution: ShotResolution) -> Option<crate::entities::Point> {
    let impact = resolution.impact?;
    if resolution.result == ShotResult::Miss && matches!(impact.kind, CollisionKind::Building(_)) {
        Some(impact.sample.position)
    } else {
        None
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
        assert_eq!(state.drain_audio_cues(), vec![AudioCue::Throw]);

        let shot = state.active_shot.as_ref().expect("active shot");
        assert_eq!(shot.thrower_index, 0);
        assert!(!shot.samples.is_empty());
        assert_eq!(state.last_shot, Some(shot.resolution));
    }

    #[test]
    fn building_miss_triggers_generic_explosion_and_blocks_input() {
        let config = GameConfig::default();
        let mut rng = StdRng::seed_from_u64(123);
        let mut state = GameState::new_with_rng(config, &mut rng);
        let impact_position = crate::entities::Point::new(90.0, 310.0);
        let sample = TrajectoryPoint {
            time: 0.0,
            position: impact_position,
            rotation_frame: 0,
        };
        let resolution = ShotResolution {
            result: ShotResult::Miss,
            impact: Some(crate::physics::ShotImpact {
                sample,
                kind: CollisionKind::Building(0),
                result: ShotResult::Miss,
            }),
            sun_was_hit: false,
        };

        assert_eq!(shot_explosion_position(resolution), Some(impact_position));
        state.active_shot = Some(ActiveShot {
            thrower_index: 0,
            samples: vec![sample],
            current_sample: 0,
            resolution,
            post_impact_frames: ActiveShot::IMPACT_HOLD_FRAMES - 1,
        });

        state.update_animation();

        assert!(state.active_shot.is_none());
        assert!(state.shot_explosion.is_some());
        assert_eq!(state.drain_audio_cues(), vec![AudioCue::Explosion]);
        assert!(!state.accepts_shot_input());
        assert!(!state.submit_shot(1, 45.0, 10.0));
        assert_eq!(state.current_turn, 1);

        for _ in 0..ShotExplosion::TOTAL_FRAMES {
            state.update_animation();
        }

        assert!(state.shot_explosion.is_none());
        assert!(state.accepts_shot_input());
    }

    #[test]
    fn edge_miss_does_not_trigger_generic_explosion() {
        let sample = TrajectoryPoint {
            time: 0.0,
            position: crate::entities::Point::new(3.0, 50.0),
            rotation_frame: 0,
        };
        let resolution = ShotResolution {
            result: ShotResult::Miss,
            impact: Some(crate::physics::ShotImpact {
                sample,
                kind: CollisionKind::BottomOrEdge,
                result: ShotResult::Miss,
            }),
            sun_was_hit: false,
        };

        assert_eq!(shot_explosion_position(resolution), None);
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
    fn scored_shot_triggers_gorilla_explosion_then_score_and_fresh_round() {
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

        assert_eq!(state.players[0].score, 0);
        assert_eq!(state.players[1].score, 0);
        assert!(state.active_shot.is_none());
        assert!(state.gorilla_explosion.is_some());
        assert_eq!(state.drain_audio_cues(), vec![AudioCue::GorillaExplosion]);
        assert!(state.victory_dance.is_none());
        assert_eq!(state.city, original_city);

        for _ in 0..GorillaExplosion::TOTAL_FRAMES {
            state.update_animation();
        }

        assert_eq!(state.players[0].score, 1);
        assert_eq!(state.players[1].score, 0);
        assert!(state.gorilla_explosion.is_none());
        assert!(state.victory_dance.is_some());
        assert_eq!(state.drain_audio_cues(), vec![AudioCue::Victory]);
        assert_eq!(state.city, original_city);

        for _ in 0..VictoryDance::TOTAL_FRAMES {
            state.update_animation();
        }

        assert!(state.victory_dance.is_none());
        assert_ne!(state.city, original_city);
    }

    #[test]
    fn setup_flow_applies_names_rounds_and_gravity_then_starts_match() {
        let config = GameConfig::default();
        let mut rng = StdRng::seed_from_u64(123);
        let mut state = GameState::new_with_rng(config, &mut rng);
        state.screen = ScreenState::Intro;
        state.setup_completed = false;

        state.continue_intro();
        assert_eq!(state.screen, ScreenState::Setup);

        state.apply_setup("Ada", "Grace", 5, 1.6);
        assert_eq!(state.screen, ScreenState::Menu);
        assert_eq!(state.players[0].name, "Ada");
        assert_eq!(state.players[1].name, "Grace");
        assert_eq!(state.round_limit, 5);
        assert_eq!(state.config.gravity, 1.6);

        state.view_intro();
        assert_eq!(state.screen, ScreenState::Intro);
        state.continue_intro();
        assert_eq!(state.screen, ScreenState::Menu);
        state.start_match();
        assert_eq!(state.screen, ScreenState::Playing);
        assert_eq!(state.completed_rounds, 0);
        assert_eq!(state.current_turn, 0);
    }

    #[test]
    fn fixed_round_limit_shows_game_over_after_configured_rounds() {
        let config = GameConfig::default();
        let mut rng = StdRng::seed_from_u64(123);
        let mut state = GameState::new_with_rng(config, &mut rng);
        state.round_limit = 1;

        state.begin_gorilla_explosion(1, 0);
        for _ in 0..GorillaExplosion::TOTAL_FRAMES {
            state.update_animation();
        }
        for _ in 0..VictoryDance::TOTAL_FRAMES {
            state.update_animation();
        }

        assert_eq!(state.players[0].score, 1);
        assert_eq!(state.completed_rounds, 1);
        assert_eq!(state.screen, ScreenState::GameOver);
        assert!(!state.accepts_shot_input());
        assert!(!state.submit_shot(0, 45.0, 10.0));
        assert_eq!(
            state.final_score_lines(),
            [
                "Player 1        1".to_string(),
                "Player 2        0".to_string()
            ]
        );

        state.continue_after_game_over();
        assert_eq!(state.screen, ScreenState::Setup);
        assert!(!state.setup_completed);
        assert_eq!(state.completed_rounds, 0);
        assert!(state.active_shot.is_none());
        assert!(state.victory_dance.is_none());
    }

    #[test]
    fn continue_after_game_over_only_applies_on_game_over_screen() {
        let config = GameConfig::default();
        let mut rng = StdRng::seed_from_u64(123);
        let mut state = GameState::new_with_rng(config, &mut rng);

        state.continue_after_game_over();
        assert_eq!(state.screen, ScreenState::Playing);
        assert!(state.setup_completed);

        state.screen = ScreenState::GameOver;
        state.completed_rounds = 2;
        state.continue_after_game_over();
        assert_eq!(state.screen, ScreenState::Setup);
        assert!(!state.setup_completed);
        assert_eq!(state.completed_rounds, 0);
    }

    #[test]
    fn gorilla_explosion_maps_victim_and_scorer_and_blocks_input() {
        assert_eq!(gorilla_explosion_for_result(0, ShotResult::Miss), None);
        assert_eq!(
            gorilla_explosion_for_result(0, ShotResult::HitPlayer(1)),
            Some((1, 0))
        );
        assert_eq!(
            gorilla_explosion_for_result(1, ShotResult::HitPlayer(0)),
            Some((0, 1))
        );
        assert_eq!(
            gorilla_explosion_for_result(0, ShotResult::HitSelf),
            Some((0, 1))
        );

        let config = GameConfig::default();
        let mut rng = StdRng::seed_from_u64(123);
        let mut state = GameState::new_with_rng(config, &mut rng);
        state.begin_gorilla_explosion(0, 1);
        assert_eq!(state.drain_audio_cues(), vec![AudioCue::GorillaExplosion]);

        assert!(!state.accepts_shot_input());
        assert!(!state.submit_shot(0, 45.0, 10.0));
        state.update_animation();
        assert_eq!(
            state.gorilla_explosion.map(|explosion| explosion.frame()),
            Some(1)
        );
    }

    #[test]
    fn victory_dance_alternates_winner_pose_and_blocks_new_shots() {
        let config = GameConfig::default();
        let mut rng = StdRng::seed_from_u64(123);
        let mut state = GameState::new_with_rng(config, &mut rng);

        state.begin_victory_dance(1);
        assert_eq!(state.drain_audio_cues(), vec![AudioCue::Victory]);
        assert!(!state.accepts_shot_input());
        assert!(!state.submit_shot(1, 45.0, 10.0));

        state.update_animation();
        assert_eq!(state.gorillas[1].pose, ArmPose::LeftUp);

        for _ in 0..VictoryDance::POSE_TOGGLE_FRAMES {
            state.update_animation();
        }
        assert_eq!(state.gorillas[1].pose, ArmPose::RightUp);
    }

    #[test]
    fn delta_animation_accumulates_time_and_caps_catch_up_steps() {
        let config = GameConfig::default();
        let mut rng = StdRng::seed_from_u64(123);
        let mut state = GameState::new_with_rng(config, &mut rng);
        assert!(state.submit_shot(0, 45.0, 10.0));
        state.drain_audio_cues();

        state.update_animation_with_delta(ANIMATION_FRAME_SECONDS / 2.0);
        assert_eq!(state.active_shot.as_ref().unwrap().current_sample, 0);

        state.update_animation_with_delta(ANIMATION_FRAME_SECONDS / 2.0);
        assert_eq!(state.active_shot.as_ref().unwrap().current_sample, 1);

        state.update_animation_with_delta(ANIMATION_FRAME_SECONDS * 20.0);
        assert_eq!(
            state.active_shot.as_ref().unwrap().current_sample,
            1 + MAX_ANIMATION_STEPS_PER_UPDATE as usize
        );
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
