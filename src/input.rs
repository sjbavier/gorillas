//! Local input helpers. Future network play should feed commands into game logic without depending on this module.

use minifb::{Key, KeyRepeat, Window};

use crate::entities::PlayerCommand;

const MAX_SHOT_INPUT: usize = 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SetupField {
    Player1,
    Player2,
    Rounds,
    Gravity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GameSetup {
    pub player1_name: String,
    pub player2_name: String,
    pub round_limit: u32,
    pub gravity: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetupInputState {
    pub player1_name: String,
    pub player2_name: String,
    pub rounds: String,
    pub gravity: String,
    pub active_field: SetupField,
}

impl SetupInputState {
    pub fn new() -> Self {
        Self {
            player1_name: String::new(),
            player2_name: String::new(),
            rounds: String::new(),
            gravity: String::new(),
            active_field: SetupField::Player1,
        }
    }

    fn active_value_mut(&mut self) -> &mut String {
        match self.active_field {
            SetupField::Player1 => &mut self.player1_name,
            SetupField::Player2 => &mut self.player2_name,
            SetupField::Rounds => &mut self.rounds,
            SetupField::Gravity => &mut self.gravity,
        }
    }

    fn advance_field(&mut self) -> Option<GameSetup> {
        self.active_field = match self.active_field {
            SetupField::Player1 => SetupField::Player2,
            SetupField::Player2 => SetupField::Rounds,
            SetupField::Rounds => SetupField::Gravity,
            SetupField::Gravity => return self.setup(),
        };
        None
    }

    pub fn setup(&self) -> Option<GameSetup> {
        let round_limit = if self.rounds.trim().is_empty() {
            3
        } else {
            let parsed: u32 = self.rounds.parse().ok()?;
            if parsed == 0 {
                return None;
            }
            parsed
        };
        let gravity = if self.gravity.trim().is_empty() {
            9.8
        } else {
            let parsed: f32 = self.gravity.parse().ok()?;
            if parsed <= 0.0 {
                return None;
            }
            parsed
        };
        Some(GameSetup {
            player1_name: setup_name(&self.player1_name, "Player 1"),
            player2_name: setup_name(&self.player2_name, "Player 2"),
            round_limit,
            gravity,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SetupInputEvent {
    Editing,
    Complete(GameSetup),
}

pub fn update_setup_input(window: &Window, state: &mut SetupInputState) -> SetupInputEvent {
    for key in window.get_keys_pressed(KeyRepeat::No) {
        match key {
            Key::Enter | Key::NumPadEnter => {
                if let Some(setup) = state.advance_field() {
                    return SetupInputEvent::Complete(setup);
                }
            }
            Key::Tab => {
                let _ = state.advance_field();
            }
            Key::Backspace | Key::Delete => {
                state.active_value_mut().pop();
            }
            key => {
                if let Some(ch) = key_to_setup_char(key) {
                    push_setup_char(state, ch);
                }
            }
        }
    }
    SetupInputEvent::Editing
}

pub fn any_continue_key_pressed(window: &Window) -> bool {
    !window.get_keys_pressed(KeyRepeat::No).is_empty()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuChoice {
    ViewIntro,
    PlayGame,
}

pub fn update_menu_input(window: &Window) -> Option<MenuChoice> {
    for key in window.get_keys_pressed(KeyRepeat::No) {
        match key {
            Key::V => return Some(MenuChoice::ViewIntro),
            Key::P | Key::Enter | Key::NumPadEnter => return Some(MenuChoice::PlayGame),
            _ => {}
        }
    }
    None
}

fn setup_name(value: &str, default: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        default.to_string()
    } else {
        trimmed.chars().take(10).collect()
    }
}

fn push_setup_char(state: &mut SetupInputState, ch: char) {
    match state.active_field {
        SetupField::Player1 | SetupField::Player2 => {
            let value = state.active_value_mut();
            if value.chars().count() < 10 {
                value.push(ch);
            }
        }
        SetupField::Rounds => {
            if ch.is_ascii_digit() && state.rounds.len() < 2 {
                let mut candidate = state.rounds.clone();
                candidate.push(ch);
                if candidate.parse::<u32>().is_ok_and(|n| n > 0) {
                    state.rounds.push(ch);
                }
            }
        }
        SetupField::Gravity => {
            let value = &mut state.gravity;
            if value.len() < 6 && (ch.is_ascii_digit() || ch == '.') {
                if ch == '.' && value.contains('.') {
                    return;
                }
                let mut candidate = value.clone();
                candidate.push(ch);
                if candidate == "." || candidate.parse::<f32>().is_ok_and(|n| n > 0.0) {
                    value.push(ch);
                }
            }
        }
    }
}

fn key_to_setup_char(key: Key) -> Option<char> {
    match key {
        Key::Space => Some(' '),
        Key::A => Some('A'),
        Key::B => Some('B'),
        Key::C => Some('C'),
        Key::D => Some('D'),
        Key::E => Some('E'),
        Key::F => Some('F'),
        Key::G => Some('G'),
        Key::H => Some('H'),
        Key::I => Some('I'),
        Key::J => Some('J'),
        Key::K => Some('K'),
        Key::L => Some('L'),
        Key::M => Some('M'),
        Key::N => Some('N'),
        Key::O => Some('O'),
        Key::P => Some('P'),
        Key::Q => Some('Q'),
        Key::R => Some('R'),
        Key::S => Some('S'),
        Key::T => Some('T'),
        Key::U => Some('U'),
        Key::V => Some('V'),
        Key::W => Some('W'),
        Key::X => Some('X'),
        Key::Y => Some('Y'),
        Key::Z => Some('Z'),
        other => key_to_numeric_char(other),
    }
}

pub fn quit_requested(window: &Window) -> bool {
    !window.is_open() || (window.is_key_down(QUIT_KEY) && key_requests_quit(QUIT_KEY))
}

pub const QUIT_KEY: Key = Key::Escape;

pub fn key_requests_quit(key: Key) -> bool {
    key == QUIT_KEY
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

    pub fn command(&self) -> Option<PlayerCommand> {
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

pub fn parse_shot_number(value: &str) -> Option<f32> {
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
    fn setup_defaults_and_limits_match_original_prompts() {
        let setup = SetupInputState::new().setup().expect("default setup");
        assert_eq!(setup.player1_name, "Player 1");
        assert_eq!(setup.player2_name, "Player 2");
        assert_eq!(setup.round_limit, 3);
        assert_eq!(setup.gravity, 9.8);

        let mut input = SetupInputState::new();
        input.player1_name = "LongPlayerName".into();
        input.player2_name = "".into();
        input.rounds = "12".into();
        input.gravity = "1.6".into();
        let setup = input.setup().expect("custom setup");
        assert_eq!(setup.player1_name, "LongPlayer");
        assert_eq!(setup.player2_name, "Player 2");
        assert_eq!(setup.round_limit, 12);
        assert_eq!(setup.gravity, 1.6);
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

    #[test]
    fn escape_key_is_the_global_quit_request() {
        assert!(key_requests_quit(Key::Escape));
        assert!(!key_requests_quit(Key::Enter));
    }
}
