//! Core data types that are independent from rendering and local input.

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArmPose {
    RightUp,
    LeftUp,
    Down,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SunMood {
    Happy,
    Shocked,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShotResult {
    Miss,
    HitPlayer(usize),
    HitSelf,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum PlayerCommand {
    SubmitShot {
        player_id: usize,
        angle_degrees: f32,
        velocity: f32,
    },
}

#[derive(Clone, Debug)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub score: u32,
}

impl Player {
    pub fn new(id: usize, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            score: 0,
        }
    }
}
