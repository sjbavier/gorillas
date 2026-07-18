//! Game-wide configuration and constants for the Gorillas port.

pub const SPEED_CONST: u32 = 500;
#[allow(dead_code)]
pub const HIT_SELF: u8 = 1;

pub const SCREEN_WIDTH: usize = 640;
pub const SCREEN_HEIGHT: usize = 350;

pub type Color = u32;

#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub background: Color,
    pub object: Color,
    pub window: Color,
    pub sun: Color,
    pub text: Color,
}

#[derive(Clone, Copy, Debug)]
pub struct GameConfig {
    pub screen_width: usize,
    pub screen_height: usize,
    pub gravity: f32,
    pub speed_const: u32,
    pub palette: Palette,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            screen_width: SCREEN_WIDTH,
            screen_height: SCREEN_HEIGHT,
            gravity: 9.8,
            speed_const: SPEED_CONST,
            palette: Palette {
                background: rgb(0, 0, 170),
                object: rgb(170, 85, 0),
                window: rgb(255, 255, 85),
                sun: rgb(85, 255, 255),
                text: rgb(255, 255, 255),
            },
        }
    }
}

pub const fn rgb(red: u8, green: u8, blue: u8) -> Color {
    ((red as Color) << 16) | ((green as Color) << 8) | blue as Color
}
