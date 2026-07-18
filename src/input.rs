//! Local input helpers. Future network play should feed commands into game logic without depending on this module.

use minifb::{Key, KeyRepeat, Window};

pub fn quit_requested(window: &Window) -> bool {
    !window.is_open() || window.is_key_down(Key::Escape)
}

pub fn demo_shot_requested(window: &Window) -> bool {
    window.is_key_pressed(Key::Space, KeyRepeat::No)
}
