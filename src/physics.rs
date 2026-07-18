//! Rendering-independent banana trajectory physics.
//!
//! The original QBasic `PlotShot` routine animates shots with a fixed `t += 0.1`
//! loop and samples collision from pixels. This module keeps pure shot math and
//! geometry-based collision helpers so future turn resolution can use it without
//! depending on rendering or local input.

#![allow(dead_code)]

use crate::{
    city::City,
    config::GameConfig,
    entities::{Gorilla, Point, ShotResult, Sun},
};

pub const TIME_STEP: f32 = 0.1;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShotParams {
    pub start: Point,
    pub angle_degrees: f32,
    pub velocity: f32,
    pub wind: i32,
    pub gravity: f32,
    pub screen_height: f32,
}

impl ShotParams {
    pub fn new(
        start: Point,
        angle_degrees: f32,
        velocity: f32,
        wind: i32,
        config: &GameConfig,
    ) -> Self {
        Self {
            start,
            angle_degrees,
            velocity,
            wind,
            gravity: config.gravity,
            screen_height: config.screen_height as f32,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrajectoryPoint {
    pub time: f32,
    pub position: Point,
    /// Banana rotation frame matching `(t * 10) MOD 4` from QBasic.
    pub rotation_frame: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollisionKind {
    Building(usize),
    Gorilla(usize),
    Sun,
    BottomOrEdge,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShotImpact {
    pub sample: TrajectoryPoint,
    pub kind: CollisionKind,
    pub result: ShotResult,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShotResolution {
    pub result: ShotResult,
    pub impact: Option<ShotImpact>,
    pub sun_was_hit: bool,
}

/// Convert local player input into the world-space firing angle used by QBasic.
///
/// Player 1 throws with the typed angle. Player 2 mirrors it with
/// `angle = 180 - angle` before simulation.
pub fn world_angle_degrees(player_index: usize, input_angle_degrees: f32) -> f32 {
    if player_index == 1 {
        180.0 - input_angle_degrees
    } else {
        input_angle_degrees
    }
}

/// Compute the QBasic banana spawn point from the gorilla sprite top-left.
///
/// In EGA mode `Scl(n) == n`, so this preserves:
/// - `StartYPos = StartY - 4 - 3`
/// - player 2 adds 25 pixels to `StartXPos`
pub fn banana_spawn(gorilla: &Gorilla) -> Point {
    let mut x = gorilla.position.x as f32;
    let y = (gorilla.position.y - 7) as f32;
    if gorilla.player_index == 1 {
        x += 25.0;
    }
    Point::new(x, y)
}

/// Position of the banana at elapsed shot time using the original formula.
pub fn trajectory_at(params: ShotParams, time: f32) -> TrajectoryPoint {
    let angle = params.angle_degrees.to_radians();
    let initial_x_velocity = angle.cos() * params.velocity;
    let initial_y_velocity = angle.sin() * params.velocity;

    let x = params.start.x
        + (initial_x_velocity * time)
        + (0.5 * (params.wind as f32 / 5.0) * time.powi(2));
    let y = params.start.y
        + ((-initial_y_velocity * time) + (0.5 * params.gravity * time.powi(2)))
            * (params.screen_height / 350.0);

    TrajectoryPoint {
        time,
        position: Point::new(x, y),
        rotation_frame: ((time * 10.0) as u32 % 4) as u8,
    }
}

pub fn is_off_screen(point: Point, config: &GameConfig) -> bool {
    point.x >= (config.screen_width as f32 - 10.0)
        || point.x <= 3.0
        || point.y >= (config.screen_height as f32 - 3.0)
}

/// Generate a finite list of on-screen trajectory samples for non-rendering tests
/// and future collision resolution. The first sample is at `t = 0.0`, matching
/// the QBasic loop's first computed position before `t` is incremented.
pub fn simulate_until_off_screen(params: ShotParams, config: &GameConfig) -> Vec<TrajectoryPoint> {
    let mut samples = Vec::new();
    let mut time = 0.0;

    for _ in 0..10_000 {
        let sample = trajectory_at(params, time);
        if is_off_screen(sample.position, config) {
            break;
        }
        samples.push(sample);
        time += TIME_STEP;
    }

    samples
}

/// Resolve a shot against scene geometry.
///
/// QBasic samples pixels from the drawn banana and scene. This Rust port uses a
/// documented geometry strategy for now: building rectangles, gorilla sprite
/// bounds, sun radius, and the same screen-edge/bottom threshold as `PlotShot`.
pub fn resolve_shot(
    thrower_index: usize,
    params: ShotParams,
    config: &GameConfig,
    city: &City,
    gorillas: &[Gorilla; 2],
    sun: &Sun,
) -> ShotResolution {
    if params.velocity < 2.0 {
        return ShotResolution {
            result: ShotResult::HitSelf,
            impact: Some(ShotImpact {
                sample: trajectory_at(params, 0.0),
                kind: CollisionKind::Gorilla(thrower_index),
                result: ShotResult::HitSelf,
            }),
            sun_was_hit: false,
        };
    }

    let mut sun_was_hit = false;
    let mut time = 0.0;

    for _ in 0..10_000 {
        let sample = trajectory_at(params, time);
        let position = sample.position;

        if is_off_screen(position, config) {
            let impact = ShotImpact {
                sample,
                kind: CollisionKind::BottomOrEdge,
                result: ShotResult::Miss,
            };
            return ShotResolution {
                result: ShotResult::Miss,
                impact: Some(impact),
                sun_was_hit,
            };
        }

        if position.y > 0.0 && sun.contains(position) {
            sun_was_hit = true;
        }

        if let Some((player_index, result)) = gorilla_hit(thrower_index, position, gorillas) {
            let impact = ShotImpact {
                sample,
                kind: CollisionKind::Gorilla(player_index),
                result,
            };
            return ShotResolution {
                result,
                impact: Some(impact),
                sun_was_hit,
            };
        }

        if let Some(building_index) = building_hit(position, city) {
            let impact = ShotImpact {
                sample,
                kind: CollisionKind::Building(building_index),
                result: ShotResult::Miss,
            };
            return ShotResolution {
                result: ShotResult::Miss,
                impact: Some(impact),
                sun_was_hit,
            };
        }

        time += TIME_STEP;
    }

    ShotResolution {
        result: ShotResult::Miss,
        impact: None,
        sun_was_hit,
    }
}

fn gorilla_hit(
    thrower_index: usize,
    position: Point,
    gorillas: &[Gorilla; 2],
) -> Option<(usize, ShotResult)> {
    gorillas
        .iter()
        .find(|gorilla| gorilla.position.contains(position))
        .map(|gorilla| {
            let result = if gorilla.player_index == thrower_index {
                ShotResult::HitSelf
            } else {
                ShotResult::HitPlayer(gorilla.player_index)
            };
            (gorilla.player_index, result)
        })
}

fn building_hit(position: Point, city: &City) -> Option<usize> {
    city.buildings.iter().position(|building| {
        position.x >= building.x as f32
            && position.x <= (building.x + building.width) as f32
            && position.y >= building.y as f32
            && position.y <= city.bottom_line as f32
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        city::{Building, City, SlopePattern},
        config::rgb,
    };

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.001,
            "expected {actual} to be close to {expected}"
        );
    }

    fn test_city() -> City {
        City {
            buildings: vec![Building {
                x: 100,
                y: 300,
                width: 60,
                height: 35,
                color: rgb(170, 0, 0),
                windows: Vec::new(),
            }],
            wind: 0,
            slope: SlopePattern::Upward,
            bottom_line: 335,
        }
    }

    fn test_gorillas() -> [Gorilla; 2] {
        [Gorilla::new(0, 20, 280), Gorilla::new(1, 500, 280)]
    }

    #[test]
    fn player_two_angle_is_mirrored_like_qbasic() {
        assert_close(world_angle_degrees(0, 45.0), 45.0);
        assert_close(world_angle_degrees(1, 45.0), 135.0);
    }

    #[test]
    fn trajectory_matches_original_formula_without_wind() {
        let params = ShotParams {
            start: Point::new(100.0, 200.0),
            angle_degrees: 45.0,
            velocity: 50.0,
            wind: 0,
            gravity: 9.8,
            screen_height: 350.0,
        };

        let sample = trajectory_at(params, 1.0);

        assert_close(sample.position.x, 135.35535);
        assert_close(sample.position.y, 169.54465);
        assert_eq!(sample.rotation_frame, 2);
    }

    #[test]
    fn wind_accelerates_x_position_by_original_wind_over_five_rule() {
        let no_wind = ShotParams {
            start: Point::new(100.0, 200.0),
            angle_degrees: 0.0,
            velocity: 10.0,
            wind: 0,
            gravity: 9.8,
            screen_height: 350.0,
        };
        let with_wind = ShotParams {
            wind: 10,
            ..no_wind
        };

        let still = trajectory_at(no_wind, 2.0);
        let windy = trajectory_at(with_wind, 2.0);

        assert_close(windy.position.x - still.position.x, 4.0);
    }

    #[test]
    fn player_specific_spawn_offsets_match_ega_qbasic_constants() {
        let left = Gorilla::new(0, 50, 120);
        let right = Gorilla::new(1, 500, 110);

        assert_eq!(banana_spawn(&left), Point::new(50.0, 113.0));
        assert_eq!(banana_spawn(&right), Point::new(525.0, 103.0));
    }

    #[test]
    fn simulation_stops_before_off_screen_sample() {
        let config = GameConfig::default();
        let params = ShotParams::new(Point::new(10.0, 100.0), 0.0, 300.0, 0, &config);

        let samples = simulate_until_off_screen(params, &config);

        assert!(!samples.is_empty());
        assert!(samples
            .iter()
            .all(|sample| !is_off_screen(sample.position, &config)));
    }

    #[test]
    fn low_velocity_resolves_as_self_hit() {
        let config = GameConfig::default();
        let gorillas = test_gorillas();
        let params = ShotParams::new(banana_spawn(&gorillas[0]), 45.0, 1.0, 0, &config);

        let resolution = resolve_shot(0, params, &config, &test_city(), &gorillas, &Sun::new(640));

        assert_eq!(resolution.result, ShotResult::HitSelf);
        assert_eq!(
            resolution.impact.map(|impact| impact.kind),
            Some(CollisionKind::Gorilla(0))
        );
    }

    #[test]
    fn shot_reports_building_collision() {
        let config = GameConfig::default();
        let gorillas = test_gorillas();
        let params = ShotParams::new(Point::new(90.0, 310.0), 0.0, 30.0, 0, &config);

        let resolution = resolve_shot(0, params, &config, &test_city(), &gorillas, &Sun::new(640));

        assert_eq!(resolution.result, ShotResult::Miss);
        assert_eq!(
            resolution.impact.map(|impact| impact.kind),
            Some(CollisionKind::Building(0))
        );
    }

    #[test]
    fn shot_reports_gorilla_collision() {
        let config = GameConfig::default();
        let gorillas = test_gorillas();
        let params = ShotParams::new(Point::new(490.0, 290.0), 0.0, 30.0, 0, &config);

        let resolution = resolve_shot(0, params, &config, &test_city(), &gorillas, &Sun::new(640));

        assert_eq!(resolution.result, ShotResult::HitPlayer(1));
        assert_eq!(
            resolution.impact.map(|impact| impact.kind),
            Some(CollisionKind::Gorilla(1))
        );
    }

    #[test]
    fn shot_records_sun_passage_without_stopping_trajectory() {
        let config = GameConfig::default();
        let gorillas = test_gorillas();
        let sun = Sun::new(640);
        let params = ShotParams::new(Point::new(320.0, 25.0), 90.0, 20.0, 0, &config);

        let resolution = resolve_shot(0, params, &config, &test_city(), &gorillas, &sun);

        assert!(resolution.sun_was_hit);
        assert_ne!(
            resolution.impact.map(|impact| impact.kind),
            Some(CollisionKind::Sun)
        );
    }

    #[test]
    fn shot_reports_edge_or_bottom_miss() {
        let config = GameConfig::default();
        let gorillas = test_gorillas();
        let params = ShotParams::new(Point::new(10.0, 50.0), 0.0, 300.0, 0, &config);

        let resolution = resolve_shot(0, params, &config, &test_city(), &gorillas, &Sun::new(640));

        assert_eq!(resolution.result, ShotResult::Miss);
        assert_eq!(
            resolution.impact.map(|impact| impact.kind),
            Some(CollisionKind::BottomOrEdge)
        );
    }
}
