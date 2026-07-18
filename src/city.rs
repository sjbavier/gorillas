//! City skyline generation and rendering-independent scene data.
//!
//! This ports the structure of QBasic `MakeCityScape` while storing the
//! generated buildings/windows for later rendering and collision instead of
//! drawing directly as the original did.

use rand::Rng;

use crate::config::{rgb, Color, GameConfig};

pub const BOTTOM_LINE: i32 = 335;
const START_X: i32 = 2;
const HEIGHT_INCREMENT: i32 = 10;
const DEFAULT_BUILDING_WIDTH: i32 = 37;
const RANDOM_HEIGHT: i32 = 120;
const WINDOW_WIDTH: i32 = 3;
const WINDOW_HEIGHT: i32 = 6;
const WINDOW_SPACING_VERTICAL: i32 = 15;
const WINDOW_SPACING_HORIZONTAL: i32 = 10;
const MIN_GORILLA_CLEARANCE_Y: i32 = 65;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SlopePattern {
    Upward,
    Downward,
    VShape,
    InvertedV,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub color: Color,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Building {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub color: Color,
    pub windows: Vec<WindowRect>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct City {
    pub buildings: Vec<Building>,
    pub wind: i32,
    pub slope: SlopePattern,
    pub bottom_line: i32,
}

impl City {
    pub fn generate(config: &GameConfig, rng: &mut impl Rng) -> Self {
        let (slope, mut new_height) = match rng.gen_range(1..=6) {
            1 => (SlopePattern::Upward, 15),
            2 => (SlopePattern::Downward, 130),
            3..=5 => (SlopePattern::VShape, 15),
            6 => (SlopePattern::InvertedV, 130),
            _ => unreachable!(),
        };

        let mut x = START_X;
        let mut buildings = Vec::new();
        let screen_width = config.screen_width as i32;

        while x <= screen_width - HEIGHT_INCREMENT {
            match slope {
                SlopePattern::Upward => new_height += HEIGHT_INCREMENT,
                SlopePattern::Downward => new_height -= HEIGHT_INCREMENT,
                SlopePattern::VShape => {
                    if x > screen_width / 2 {
                        new_height -= 2 * HEIGHT_INCREMENT;
                    } else {
                        new_height += 2 * HEIGHT_INCREMENT;
                    }
                }
                SlopePattern::InvertedV => {
                    if x > screen_width / 2 {
                        new_height += 2 * HEIGHT_INCREMENT;
                    } else {
                        new_height -= 2 * HEIGHT_INCREMENT;
                    }
                }
            }

            let mut width = rng.gen_range(1..=DEFAULT_BUILDING_WIDTH) + DEFAULT_BUILDING_WIDTH;
            if x + width > screen_width {
                width = screen_width - x - 2;
            }
            if width <= 0 {
                break;
            }

            let mut height = rng.gen_range(1..=RANDOM_HEIGHT) + new_height;
            if height < HEIGHT_INCREMENT {
                height = HEIGHT_INCREMENT;
            }
            if BOTTOM_LINE - height <= MIN_GORILLA_CLEARANCE_Y {
                height = BOTTOM_LINE - MIN_GORILLA_CLEARANCE_Y - 5;
            }

            let y = BOTTOM_LINE - height;
            let color = building_color(rng.gen_range(1..=3));
            let windows = generate_windows(x, height, width, rng, config.palette.window);
            buildings.push(Building {
                x,
                y,
                width,
                height,
                color,
                windows,
            });

            x += width + 2;
        }

        Self {
            buildings,
            wind: generate_wind(rng),
            slope,
            bottom_line: BOTTOM_LINE,
        }
    }
}

fn generate_windows(
    building_x: i32,
    building_height: i32,
    building_width: i32,
    rng: &mut impl Rng,
    lit_color: Color,
) -> Vec<WindowRect> {
    let mut windows = Vec::new();
    let mut column_x = building_x + 3;

    while column_x < building_x + building_width - 3 {
        let mut i = building_height - 3;
        while i >= 7 {
            let color = if rng.gen_range(1..=4) == 1 {
                rgb(85, 85, 85)
            } else {
                lit_color
            };
            windows.push(WindowRect {
                x: column_x,
                y: BOTTOM_LINE - i,
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                color,
            });
            i -= WINDOW_SPACING_VERTICAL;
        }
        column_x += WINDOW_SPACING_HORIZONTAL;
    }

    windows
}

pub fn generate_wind(rng: &mut impl Rng) -> i32 {
    let mut wind = rng.gen_range(1..=10) - 5;
    if rng.gen_range(1..=3) == 1 {
        if wind > 0 {
            wind += rng.gen_range(1..=10);
        } else {
            wind -= rng.gen_range(1..=10);
        }
    }
    wind
}

fn building_color(choice: i32) -> Color {
    match choice {
        1 => rgb(170, 0, 0),
        2 => rgb(170, 0, 170),
        _ => rgb(170, 85, 0),
    }
}

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, SeedableRng};

    use super::*;

    #[test]
    fn generated_city_stays_within_screen_bounds() {
        let config = GameConfig::default();
        let mut rng = StdRng::seed_from_u64(42);
        let city = City::generate(&config, &mut rng);

        assert!(!city.buildings.is_empty());
        for building in city.buildings {
            assert!(building.x >= 0);
            assert!(building.y >= 0);
            assert!(building.x + building.width <= config.screen_width as i32);
            assert_eq!(building.y + building.height, BOTTOM_LINE);
            for window in building.windows {
                assert!(window.x >= building.x);
                assert!(window.x + window.width <= building.x + building.width);
                assert!(window.y >= building.y);
                assert!(window.y + window.height <= BOTTOM_LINE);
            }
        }
    }

    #[test]
    fn generated_wind_matches_original_possible_range() {
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..1_000 {
            let wind = generate_wind(&mut rng);
            assert!((-15..=15).contains(&wind));
        }
    }
}
