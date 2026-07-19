//! Game-wide configuration and constants for the Gorillas port.

pub const SPEED_CONST: u32 = 500;
#[allow(dead_code)]
pub const HIT_SELF: u8 = 1;

pub const SCREEN_WIDTH: usize = 640;
pub const SCREEN_HEIGHT: usize = 350;

pub type Color = u32;

// Original QBasic SCREEN 9 color attributes used by GORILLA.BAS.
pub const BACK_ATTR: u8 = 0;
#[allow(dead_code)]
pub const OBJECT_COLOR_ATTR: u8 = 1;
pub const EXPLOSION_COLOR_ATTR: u8 = 2;
pub const SUN_ATTR: u8 = 3;
#[allow(dead_code)]
pub const BUILDING_ATTRS: [u8; 3] = [5, 6, 7];
pub const UNLIT_WINDOW_ATTR: u8 = 8;
pub const WINDOW_COLOR_ATTR: u8 = 14;
pub const DISPLAY_COLOR_ATTR: u8 = 15;

#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub background: Color,
    pub object: Color,
    pub window: Color,
    pub sun: Color,
    pub explosion: Color,
    pub text: Color,
    pub dim_text: Color,
    pub prompt: Color,
    pub banana: Color,
    pub unlit_window: Color,
    pub building_colors: [Color; 3],
    pub black: Color,
}

impl Palette {
    pub const fn qbasic_ega() -> Self {
        // `GORILLA.BAS` calls SetScreen in EGA mode and remaps palette
        // attributes 0, 1, 2, 3, 5, 6, 7, and 9. Rather than expose raw
        // backend colors throughout the port, keep the QBasic attribute intent
        // here and let rendering/city generation consume semantic colors.
        Self {
            background: qbasic_screen9_color(BACK_ATTR),
            object: rgb(170, 85, 0),
            window: qbasic_screen9_color(WINDOW_COLOR_ATTR),
            sun: qbasic_screen9_color(SUN_ATTR),
            explosion: qbasic_screen9_color(EXPLOSION_COLOR_ATTR),
            text: qbasic_screen9_color(DISPLAY_COLOR_ATTR),
            dim_text: rgb(170, 170, 170),
            prompt: qbasic_screen9_color(WINDOW_COLOR_ATTR),
            banana: qbasic_screen9_color(WINDOW_COLOR_ATTR),
            unlit_window: qbasic_screen9_color(UNLIT_WINDOW_ATTR),
            building_colors: [rgb(170, 0, 0), rgb(170, 0, 170), rgb(170, 85, 0)],
            black: qbasic_screen9_color(BACK_ATTR),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GameConfig {
    pub screen_width: usize,
    pub screen_height: usize,
    pub gravity: f32,
    pub speed_const: u32,
    pub palette: Palette,
    /// Optional seed for deterministic city/wind/gorilla scene generation.
    ///
    /// The original QBasic code calls `RANDOMIZE (TIMER)` before each round.
    /// Keeping this as `None` preserves non-deterministic local play, while
    /// tests and debugging can opt into reproducible scenes with a fixed seed.
    pub random_seed: Option<u64>,
}

impl GameConfig {
    pub fn with_random_seed(mut self, seed: u64) -> Self {
        self.random_seed = Some(seed);
        self
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            screen_width: SCREEN_WIDTH,
            screen_height: SCREEN_HEIGHT,
            gravity: 9.8,
            speed_const: SPEED_CONST,
            palette: Palette::qbasic_ega(),
            random_seed: None,
        }
    }
}

pub const fn rgb(red: u8, green: u8, blue: u8) -> Color {
    ((red as Color) << 16) | ((green as Color) << 8) | blue as Color
}

pub const fn qbasic_screen9_color(attribute: u8) -> Color {
    match attribute {
        // This table is the familiar 16-color QBasic/EGA attribute palette for
        // SCREEN 9. The original also uses 64-color PALETTE remapping in
        // SetScreen; `Palette::qbasic_ega` above applies the few
        // gorillas-specific remaps that matter visually.
        0 => rgb(0, 0, 0),
        1 => rgb(0, 0, 170),
        2 => rgb(0, 170, 0),
        3 => rgb(0, 170, 170),
        4 => rgb(170, 0, 0),
        5 => rgb(170, 0, 170),
        6 => rgb(170, 85, 0),
        7 => rgb(170, 170, 170),
        8 => rgb(85, 85, 85),
        9 => rgb(85, 85, 255),
        10 => rgb(85, 255, 85),
        11 => rgb(85, 255, 255),
        12 => rgb(255, 85, 85),
        13 => rgb(255, 85, 255),
        14 => rgb(255, 255, 85),
        15 => rgb(255, 255, 255),
        _ => rgb(255, 255, 255),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qbasic_screen9_palette_maps_core_attributes() {
        assert_eq!(qbasic_screen9_color(0), rgb(0, 0, 0));
        assert_eq!(qbasic_screen9_color(1), rgb(0, 0, 170));
        assert_eq!(qbasic_screen9_color(14), rgb(255, 255, 85));
        assert_eq!(qbasic_screen9_color(15), rgb(255, 255, 255));
    }

    #[test]
    fn gorillas_palette_exposes_qbasic_semantic_colors() {
        let palette = Palette::qbasic_ega();
        assert_eq!(palette.window, qbasic_screen9_color(WINDOW_COLOR_ATTR));
        assert_eq!(palette.banana, palette.window);
        assert_eq!(
            palette.unlit_window,
            qbasic_screen9_color(UNLIT_WINDOW_ATTR)
        );
        assert_eq!(palette.building_colors.len(), BUILDING_ATTRS.len());
    }
}
