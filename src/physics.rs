//! Rendering-independent banana trajectory physics.
//!
//! The original QBasic `PlotShot` routine animates shots with a fixed `t += 0.1`
//! loop and samples collision from pixels. This module keeps just the pure shot
//! math so future turn resolution can use it without depending on rendering or
//! local input.

#![allow(dead_code)]

use crate::{
    config::GameConfig,
    entities::{Gorilla, Point},
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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.001,
            "expected {actual} to be close to {expected}"
        );
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
}
