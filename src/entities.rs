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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bounds {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[allow(dead_code)]
impl Bounds {
    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn contains(self, point: Point) -> bool {
        point.x >= self.x as f32
            && point.x < (self.x + self.width) as f32
            && point.y >= self.y as f32
            && point.y < (self.y + self.height) as f32
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Gorilla {
    /// Top-left of the saved QBasic-style gorilla sprite area.
    pub position: Bounds,
    pub player_index: usize,
    pub pose: ArmPose,
}

impl Gorilla {
    pub const SPRITE_WIDTH: i32 = 30;
    pub const SPRITE_HEIGHT: i32 = 30;
    pub const X_ADJUST: i32 = 14;
    pub const Y_ADJUST: i32 = 30;

    pub const fn new(player_index: usize, x: i32, y: i32) -> Self {
        Self {
            position: Bounds::new(x, y, Self::SPRITE_WIDTH, Self::SPRITE_HEIGHT),
            player_index,
            pose: ArmPose::Down,
        }
    }

    /// Anchor used by the original `DrawGorilla(x, y, arms)` routine.
    pub const fn draw_anchor(self) -> (i32, i32) {
        (self.position.x + 15, self.position.y + 1)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Sun {
    pub center: (i32, i32),
    pub radius: i32,
    pub mood: SunMood,
}

impl Sun {
    pub const fn new(screen_width: usize) -> Self {
        Self {
            center: (screen_width as i32 / 2, 25),
            radius: 12,
            mood: SunMood::Happy,
        }
    }

    #[allow(dead_code)]
    pub fn contains(self, point: Point) -> bool {
        let dx = point.x - self.center.0 as f32;
        let dy = point.y - self.center.1 as f32;
        dx * dx + dy * dy <= (self.radius * self.radius) as f32
    }
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
