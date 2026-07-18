//! Local input helpers. Future network play should feed commands into game logic without depending on this module.

use minifb::{Key, KeyRepeat, Window};

use crate::entities::PlayerCommand;

const MAX_SHOT_INPUT: usize = 6;

pub fn quit_requested(window: &Window) -> bool {
    !window.is_open() || window.is_key_down(Key::Escape)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShotInputField {
    Angle,
    Velocity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShotInputState {
    pub player_id: usize,
    pub angle: String,
    pub velocity: String,
    pub active_field: ShotInputField,
}

impl ShotInputState {
    pub fn new(player_id: usize) -> Self {
        Self {
            player_id,
            angle: String::new(),
            velocity: String::new(),
            active_field: ShotInputField::Angle,
        }
    }

    pub fn reset_for_player(&mut self, player_id: usize) {
        *self = Self::new(player_id);
    }

    fn active_value_mut(&mut self) -> &mut String {
        match self.active_field {
            ShotInputField::Angle => &mut self.angle,
            ShotInputField::Velocity => &mut self.velocity,
        }
    }

    pub fn angle_value(&self) -> Option<f32> {
        parse_shot_number(&self.angle)
    }

    pub fn velocity_value(&self) -> Option<f32> {
        parse_shot_number(&self.velocity)
    }

    fn command(&self) -> Option<PlayerCommand> {
        Some(PlayerCommand::SubmitShot {
            player_id: self.player_id,
            angle_degrees: self.angle_value()?,
            velocity: self.velocity_value()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ShotInputEvent {
    Editing,
    Submitted(PlayerCommand),
}

pub fn update_shot_input(window: &Window, state: &mut ShotInputState) -> ShotInputEvent {
    for key in window.get_keys_pressed(KeyRepeat::No) {
        match key {
            Key::Enter | Key::NumPadEnter => {
                if state.active_field == ShotInputField::Angle {
                    state.active_field = ShotInputField::Velocity;
                } else if let Some(command) = state.command() {
                    return ShotInputEvent::Submitted(command);
                }
            }
            Key::Tab => {
                state.active_field = match state.active_field {
                    ShotInputField::Angle => ShotInputField::Velocity,
                    ShotInputField::Velocity => ShotInputField::Angle,
                };
            }
            Key::Backspace | Key::Delete => {
                state.active_value_mut().pop();
            }
            key => {
                if let Some(ch) = key_to_numeric_char(key) {
                    push_numeric_char(state.active_value_mut(), ch);
                }
            }
        }
    }

    ShotInputEvent::Editing
}

fn push_numeric_char(value: &mut String, ch: char) {
    if value.len() >= MAX_SHOT_INPUT {
        return;
    }
    if ch == '.' && value.contains('.') {
        return;
    }

    let mut candidate = value.clone();
    candidate.push(ch);
    if candidate == "." || parse_shot_number(&candidate).is_some() {
        value.push(ch);
    }
}

fn parse_shot_number(value: &str) -> Option<f32> {
    if value.is_empty() || value == "." {
        return None;
    }
    let parsed: f32 = value.parse().ok()?;
    if (0.0..=360.0).contains(&parsed) {
        Some(parsed)
    } else {
        None
    }
}

fn key_to_numeric_char(key: Key) -> Option<char> {
    match key {
        Key::Key0 | Key::NumPad0 => Some('0'),
        Key::Key1 | Key::NumPad1 => Some('1'),
        Key::Key2 | Key::NumPad2 => Some('2'),
        Key::Key3 | Key::NumPad3 => Some('3'),
        Key::Key4 | Key::NumPad4 => Some('4'),
        Key::Key5 | Key::NumPad5 => Some('5'),
        Key::Key6 | Key::NumPad6 => Some('6'),
        Key::Key7 | Key::NumPad7 => Some('7'),
        Key::Key8 | Key::NumPad8 => Some('8'),
        Key::Key9 | Key::NumPad9 => Some('9'),
        Key::Period | Key::NumPadDot => Some('.'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shot_number_validation_matches_expected_range_and_decimal_rules() {
        assert_eq!(parse_shot_number("45"), Some(45.0));
        assert_eq!(parse_shot_number("45.5"), Some(45.5));
        assert_eq!(parse_shot_number("360"), Some(360.0));
        assert_eq!(parse_shot_number(""), None);
        assert_eq!(parse_shot_number("."), None);
        assert_eq!(parse_shot_number("361"), None);
    }

    #[test]
    fn numeric_push_allows_only_one_decimal_and_capped_values() {
        let mut value = String::new();
        for ch in ['3', '6', '0', '.', '5'] {
            push_numeric_char(&mut value, ch);
        }
        assert_eq!(value, "360.");

        let mut decimal = String::new();
        for ch in ['4', '.', '5', '.'] {
            push_numeric_char(&mut decimal, ch);
        }
        assert_eq!(decimal, "4.5");
    }

    #[test]
    fn command_requires_both_numbers() {
        let mut input = ShotInputState::new(1);
        input.angle = "45".into();
        assert_eq!(input.command(), None);
        input.velocity = "80".into();
        assert_eq!(
            input.command(),
            Some(PlayerCommand::SubmitShot {
                player_id: 1,
                angle_degrees: 45.0,
                velocity: 80.0,
            })
        );
    }
}
