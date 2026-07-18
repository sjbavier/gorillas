//! Software rendering helpers for the minifb window backend.

use font8x8::{UnicodeFonts, BASIC_FONTS};

use std::f32::consts::PI;

use crate::{
    city::City,
    config::Color,
    entities::{ArmPose, Gorilla, Sun, SunMood},
    game::{ActiveShot, GameState, GorillaExplosion, ShotExplosion},
    input::{ShotInputField, ShotInputState},
};

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

    pub fn draw(&mut self, state: &GameState, shot_input: Option<&ShotInputState>) {
        self.draw_intro(state, shot_input);
    }

    pub fn draw_intro(&mut self, state: &GameState, shot_input: Option<&ShotInputState>) {
        let palette = state.config.palette;
        self.clear(palette.background);
        self.draw_sun(&state.sun, palette.sun, palette.background);
        self.draw_score_header(state);
        self.draw_city(&state.city, palette.background, palette.explosion);
        for gorilla in &state.gorillas {
            if state
                .gorilla_explosion
                .as_ref()
                .is_some_and(|explosion| explosion.victim_index == gorilla.player_index)
            {
                continue;
            }
            self.draw_gorilla(gorilla, palette.object, palette.background);
        }
        if let Some(explosion) = &state.gorilla_explosion {
            let victim = &state.gorillas[explosion.victim_index];
            self.draw_gorilla_explosion(explosion, victim, palette.explosion, palette.background);
        }
        if let Some(active_shot) = &state.active_shot {
            self.draw_active_shot(active_shot, palette.window, palette.explosion);
        }
        if let Some(explosion) = &state.shot_explosion {
            self.draw_shot_explosion(explosion, palette.explosion, palette.background);
        }

        self.draw_centered_text("Q B a s i c    G O R I L L A S", 44, 3, palette.text);
        self.draw_centered_text("Copyright (C) IBM Corporation 1991", 84, 2, 0xc0c0c0);
        self.draw_centered_text(
            "Your mission is to hit your opponent with the exploding",
            124,
            2,
            palette.text,
        );
        self.draw_centered_text(
            "banana by varying the angle and power of your throw, taking",
            148,
            2,
            palette.text,
        );
        self.draw_centered_text(
            "into account wind speed, gravity, and the city skyline.",
            172,
            2,
            palette.text,
        );
        self.draw_centered_text(
            "The wind speed is shown by a directional arrow at the bottom",
            196,
            2,
            palette.text,
        );
        self.draw_centered_text(
            "of the playing field, its length relative to its strength.",
            220,
            2,
            palette.text,
        );
        if let Some(shot_input) = shot_input {
            self.draw_shot_prompt(state, shot_input);
        }
        self.draw_centered_text("Enter angle and velocity - Esc quits", 326, 2, 0xffff55);
    }

    fn draw_score_header(&mut self, state: &GameState) {
        let palette = state.config.palette;
        let left = &state.players[0];
        let right = &state.players[1];
        self.fill_rect(0, 0, self.width, 8, palette.background);
        self.draw_text(&left.name, 2, 0, 1, palette.text);

        let right_x = self
            .width
            .saturating_sub(right.name.chars().count() * 8 + 2);
        self.draw_text(&right.name, right_x, 0, 1, palette.text);

        let score = format!("{} >Score< {}", left.score, right.score);
        self.draw_centered_text(&score, 308, 1, 0xffff55);
    }

    fn draw_shot_prompt(&mut self, state: &GameState, shot_input: &ShotInputState) {
        let palette = state.config.palette;
        let player = &state.players[shot_input.player_id];
        let prompt = format!(
            "{} angle: {}{}",
            player.name,
            shot_input.angle,
            if shot_input.active_field == ShotInputField::Angle {
                "_"
            } else {
                ""
            }
        );
        let velocity = format!(
            "velocity: {}{}",
            shot_input.velocity,
            if shot_input.active_field == ShotInputField::Velocity {
                "_"
            } else {
                ""
            }
        );
        let x = if shot_input.player_id == 0 {
            8
        } else {
            self.width.saturating_sub(230)
        };
        self.fill_rect(x, 8, 222, 28, palette.background);
        self.draw_text(&prompt, x, 8, 1, palette.text);
        self.draw_text(&velocity, x, 22, 1, palette.text);
    }

    fn draw_active_shot(
        &mut self,
        active_shot: &ActiveShot,
        banana_color: Color,
        impact_color: Color,
    ) {
        let Some(sample) = active_shot.visible_sample() else {
            return;
        };
        let x = sample.position.x.round() as i32;
        let y = sample.position.y.round() as i32;
        self.draw_banana(x, y, sample.rotation_frame, banana_color);

        if active_shot.is_at_last_sample() {
            self.draw_circle(x + 4, y + 4, 7, impact_color);
            self.draw_circle(x + 4, y + 4, 4, impact_color);
        }
    }

    fn draw_shot_explosion(&mut self, explosion: &ShotExplosion, color: Color, background: Color) {
        let x = explosion.position.x.round() as i32 + 4;
        let y = explosion.position.y.round() as i32 + 4;
        let frame = explosion.frame() as i32;
        let radius = if frame <= ShotExplosion::MAX_RADIUS {
            frame.max(1)
        } else {
            (ShotExplosion::TOTAL_FRAMES as i32 - frame).max(0)
        };

        // QBasic `DoExplosion` draws expanding colored rings, then erases them
        // with the background. Keep this renderer-only while core state owns the
        // deterministic timing and impact position.
        let ring_color = if frame <= ShotExplosion::MAX_RADIUS {
            color
        } else {
            background
        };
        for r in 1..=radius.max(1) {
            self.draw_circle(x, y, r, ring_color);
        }
    }

    fn draw_banana(&mut self, x: i32, y: i32, rotation_frame: u8, color: Color) {
        match rotation_frame % 4 {
            // Approximate the four small QBasic banana sprites: left, up, down, right.
            0 => {
                self.draw_line(x + 7, y + 1, x + 1, y + 4, color);
                self.draw_line(x + 1, y + 4, x + 7, y + 7, color);
            }
            1 => {
                self.draw_line(x + 1, y + 7, x + 4, y + 1, color);
                self.draw_line(x + 4, y + 1, x + 7, y + 7, color);
            }
            2 => {
                self.draw_line(x + 1, y + 1, x + 4, y + 7, color);
                self.draw_line(x + 4, y + 7, x + 7, y + 1, color);
            }
            _ => {
                self.draw_line(x + 1, y + 1, x + 7, y + 4, color);
                self.draw_line(x + 7, y + 4, x + 1, y + 7, color);
            }
        }
    }

    fn draw_sun(&mut self, sun: &Sun, color: Color, feature_color: Color) {
        let (x, y) = sun.center;
        self.draw_line(x - 20, y, x + 20, y, color);
        self.draw_line(x, y - 15, x, y + 15, color);
        self.draw_line(x - 15, y - 10, x + 15, y + 10, color);
        self.draw_line(x - 15, y + 10, x + 15, y - 10, color);
        self.draw_line(x - 8, y - 13, x + 8, y + 13, color);
        self.draw_line(x - 8, y + 13, x + 8, y - 13, color);
        self.draw_line(x - 18, y - 5, x + 18, y + 5, color);
        self.draw_line(x - 18, y + 5, x + 18, y - 5, color);
        self.fill_circle(x, y, sun.radius, color);
        self.draw_circle(x, y, sun.radius, color);

        match sun.mood {
            SunMood::Happy => self.draw_arc(x, y, 8, 210.0, 330.0, feature_color),
            SunMood::Shocked => {
                self.fill_circle(x, y + 5, 3, feature_color);
                self.draw_circle(x, y + 5, 3, feature_color);
            }
        }
        self.fill_circle(x - 3, y - 2, 1, feature_color);
        self.fill_circle(x + 3, y - 2, 1, feature_color);
    }

    fn draw_gorilla(&mut self, gorilla: &Gorilla, color: Color, feature_color: Color) {
        let (x, y) = gorilla.draw_anchor();

        // Head, brow, and nose.
        self.fill_rect_i32(x - 4, y, 8, 7, color);
        self.fill_rect_i32(x - 5, y + 2, 10, 3, color);
        self.draw_line(x - 3, y + 2, x + 2, y + 2, feature_color);
        self.set_pixel(x - 2, y + 4, feature_color);
        self.set_pixel(x - 1, y + 4, feature_color);
        self.set_pixel(x + 1, y + 4, feature_color);
        self.set_pixel(x + 2, y + 4, feature_color);

        // Neck/body.
        self.draw_line(x - 3, y + 7, x + 2, y + 7, color);
        self.fill_rect_i32(x - 8, y + 8, 16, 7, color);
        self.fill_rect_i32(x - 6, y + 15, 12, 6, color);

        // Arms as thick arcs similar to the QBasic vector sprite.
        match gorilla.pose {
            ArmPose::RightUp => {
                self.draw_thick_arc(x - 3, y + 14, 9, 135.0, 225.0, color, 5);
                self.draw_thick_arc(x + 2, y + 4, 9, 315.0, 405.0, color, 5);
            }
            ArmPose::LeftUp => {
                self.draw_thick_arc(x - 3, y + 4, 9, 135.0, 225.0, color, 5);
                self.draw_thick_arc(x + 2, y + 14, 9, 315.0, 405.0, color, 5);
            }
            ArmPose::Down => {
                self.draw_thick_arc(x - 3, y + 14, 9, 135.0, 225.0, color, 5);
                self.draw_thick_arc(x + 2, y + 14, 9, 315.0, 405.0, color, 5);
            }
        }

        // Legs and chest accents.
        self.draw_thick_arc(x + 2, y + 25, 10, 135.0, 202.5, color, 5);
        self.draw_thick_arc(x - 6, y + 25, 10, 337.5, 405.0, color, 5);
        self.draw_arc(x - 5, y + 10, 5, 270.0, 360.0, feature_color);
        self.draw_arc(x + 5, y + 10, 5, 180.0, 270.0, feature_color);
    }

    fn draw_gorilla_explosion(
        &mut self,
        explosion: &GorillaExplosion,
        gorilla: &Gorilla,
        color: Color,
        background: Color,
    ) {
        let (anchor_x, anchor_y) = gorilla.draw_anchor();
        let frame = explosion.frame() as i32;
        let grow_frames = GorillaExplosion::GROW_FRAMES as i32;
        let radius = if frame <= grow_frames {
            2 + frame
        } else {
            2 + grow_frames + (frame - grow_frames) / 2
        };
        let center_x = anchor_x + 8;
        let center_y = anchor_y + 12;

        // QBasic's ExplodeGorilla expands a half-circle around the hit gorilla,
        // then alternates colored rings before clearing them. Approximate that
        // with deterministic, renderer-only rings while core state owns timing.
        if frame < grow_frames {
            self.draw_arc(center_x, center_y, radius, 180.0, 360.0, color);
            self.draw_line(
                center_x - radius,
                center_y - frame / 2,
                center_x + radius,
                center_y - frame / 2,
                color,
            );
        } else if frame < GorillaExplosion::TOTAL_FRAMES as i32 - 8 {
            let ring_color = if frame % 2 == 0 { color } else { 0x55ff55 };
            self.draw_circle(center_x, center_y - 6, radius.min(24), ring_color);
            self.draw_circle(center_x, center_y - 6, (radius / 2).max(2), ring_color);
        } else {
            self.draw_circle(center_x, center_y - 6, radius.min(24), background);
        }
    }

    fn draw_city(&mut self, city: &City, background: Color, wind_color: Color) {
        for building in &city.buildings {
            self.draw_rect_outline(
                building.x - 1,
                city.bottom_line - building.height - 1,
                building.width + 3,
                building.height + 3,
                background,
            );
            self.fill_rect_i32(
                building.x,
                building.y,
                building.width + 1,
                building.height + 1,
                building.color,
            );
            for window in &building.windows {
                self.fill_rect_i32(
                    window.x,
                    window.y,
                    window.width + 1,
                    window.height + 1,
                    window.color,
                );
            }
        }
        self.draw_wind_arrow(city.wind, wind_color);
    }

    fn draw_wind_arrow(&mut self, wind: i32, color: Color) {
        if wind == 0 {
            return;
        }
        let start_x = (self.width / 2) as i32;
        let y = self.height as i32 - 5;
        let wind_line = wind * 3 * (self.width as i32 / 320);
        let end_x = start_x + wind_line;
        let arrow_dir = if wind > 0 { -2 } else { 2 };
        self.draw_line(start_x, y, end_x, y, color);
        self.draw_line(end_x, y, end_x + arrow_dir, y - 2, color);
        self.draw_line(end_x, y, end_x + arrow_dir, y + 2, color);
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

    fn draw_rect_outline(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        self.draw_line(x, y, x + width - 1, y, color);
        self.draw_line(x, y + height - 1, x + width - 1, y + height - 1, color);
        self.draw_line(x, y, x, y + height - 1, color);
        self.draw_line(x + width - 1, y, x + width - 1, y + height - 1, color);
    }

    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        let mut x0 = x0;
        let mut y0 = y0;
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.set_pixel(x0, y0, color);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
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

    fn fill_rect_i32(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        let start_x = x.max(0) as usize;
        let start_y = y.max(0) as usize;
        let end_x = (x + width).min(self.width as i32).max(0) as usize;
        let end_y = (y + height).min(self.height as i32).max(0) as usize;
        for py in start_y..end_y {
            for px in start_x..end_x {
                self.buffer[py * self.width + px] = color;
            }
        }
    }

    fn fill_circle(&mut self, center_x: i32, center_y: i32, radius: i32, color: Color) {
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx * dx + dy * dy <= radius * radius {
                    self.set_pixel(center_x + dx, center_y + dy, color);
                }
            }
        }
    }

    fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: i32, color: Color) {
        self.draw_arc(center_x, center_y, radius, 0.0, 360.0, color);
    }

    fn draw_thick_arc(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: i32,
        start_degrees: f32,
        end_degrees: f32,
        color: Color,
        thickness: i32,
    ) {
        for offset in 0..thickness {
            self.draw_arc(
                center_x,
                center_y,
                radius - offset / 2,
                start_degrees,
                end_degrees,
                color,
            );
        }
    }

    fn draw_arc(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: i32,
        start_degrees: f32,
        end_degrees: f32,
        color: Color,
    ) {
        let steps = ((end_degrees - start_degrees).abs() as i32).max(1) * 2;
        let mut previous = None;
        for step in 0..=steps {
            let degrees =
                start_degrees + (end_degrees - start_degrees) * step as f32 / steps as f32;
            let radians = degrees * PI / 180.0;
            let x = center_x + (radius as f32 * radians.cos()).round() as i32;
            let y = center_y + (radius as f32 * radians.sin()).round() as i32;
            if let Some((px, py)) = previous {
                self.draw_line(px, py, x, y, color);
            } else {
                self.set_pixel(x, y, color);
            }
            previous = Some((x, y));
        }
    }

    fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.buffer[y as usize * self.width + x as usize] = color;
        }
    }
}
