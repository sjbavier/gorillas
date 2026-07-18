//! Software rendering helpers for the minifb window backend.

use font8x8::{UnicodeFonts, BASIC_FONTS};

use crate::{config::Color, game::GameState};

pub struct Renderer {
    width: usize,
    height: usize,
    pub buffer: Vec<Color>,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn draw_intro(&mut self, state: &GameState) {
        let palette = state.config.palette;
        self.clear(palette.background);

        self.draw_centered_text("Q B a s i c    G O R I L L A S", 64, 3, palette.text);
        self.draw_centered_text("Copyright (C) IBM Corporation 1991", 104, 2, 0xc0c0c0);
        self.draw_centered_text(
            "Your mission is to hit your opponent with the exploding",
            144,
            2,
            palette.text,
        );
        self.draw_centered_text(
            "banana by varying the angle and power of your throw, taking",
            168,
            2,
            palette.text,
        );
        self.draw_centered_text(
            "into account wind speed, gravity, and the city skyline.",
            192,
            2,
            palette.text,
        );
        self.draw_centered_text(
            "The wind speed is shown by a directional arrow at the bottom",
            216,
            2,
            palette.text,
        );
        self.draw_centered_text(
            "of the playing field, its length relative to its strength.",
            240,
            2,
            palette.text,
        );
        self.draw_centered_text("Press Esc to quit", 326, 2, 0xffff55);
    }

    fn clear(&mut self, color: Color) {
        self.buffer.fill(color);
    }

    fn draw_centered_text(&mut self, text: &str, y: usize, scale: usize, color: Color) {
        let width = text.chars().count() * 8 * scale;
        let x = self.width.saturating_sub(width) / 2;
        self.draw_text(text, x, y, scale, color);
    }

    fn draw_text(&mut self, text: &str, mut x: usize, y: usize, scale: usize, color: Color) {
        for ch in text.chars() {
            self.draw_char(ch, x, y, scale, color);
            x += 8 * scale;
        }
    }

    fn draw_char(&mut self, ch: char, x: usize, y: usize, scale: usize, color: Color) {
        if let Some(glyph) = BASIC_FONTS.get(ch) {
            for (row, bits) in glyph.iter().enumerate() {
                for col in 0..8 {
                    if bits & (1 << col) != 0 {
                        self.fill_rect(x + col * scale, y + row * scale, scale, scale, color);
                    }
                }
            }
        }
    }

    fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        for py in y..(y + height).min(self.height) {
            for px in x..(x + width).min(self.width) {
                self.buffer[py * self.width + px] = color;
            }
        }
    }
}
