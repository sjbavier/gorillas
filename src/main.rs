mod audio;
mod city;
mod config;
mod entities;
mod game;
mod input;
mod physics;
mod render;

use minifb::{Scale, Window, WindowOptions};

use crate::{audio::Audio, config::GameConfig, game::GameState, render::Renderer};

fn main() -> Result<(), minifb::Error> {
    let config = GameConfig::default();
    let game = GameState::new(config);
    let audio = Audio::new();
    audio.play_intro();

    let mut window = Window::new(
        "Gorillas",
        config.screen_width,
        config.screen_height,
        WindowOptions {
            resize: false,
            scale: Scale::X1,
            ..WindowOptions::default()
        },
    )?;
    let mut renderer = Renderer::new(config.screen_width, config.screen_height);

    while !input::quit_requested(&window) {
        renderer.draw(&game);
        window.update_with_buffer(&renderer.buffer, config.screen_width, config.screen_height)?;
    }

    Ok(())
}
