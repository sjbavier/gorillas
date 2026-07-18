mod audio;
mod city;
mod config;
mod entities;
mod game;
mod input;
mod physics;
mod render;

use minifb::{Scale, Window, WindowOptions};

use crate::{
    audio::Audio,
    config::GameConfig,
    entities::PlayerCommand,
    game::GameState,
    input::{ShotInputEvent, ShotInputState},
    render::Renderer,
};

fn main() -> Result<(), minifb::Error> {
    let config = GameConfig::default();
    let mut game = GameState::new(config);
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
    let mut shot_input = ShotInputState::new(game.current_turn);

    while !input::quit_requested(&window) {
        if game.active_shot.is_none() {
            if shot_input.player_id != game.current_turn {
                shot_input.reset_for_player(game.current_turn);
            }
            if let ShotInputEvent::Submitted(PlayerCommand::SubmitShot {
                player_id,
                angle_degrees,
                velocity,
            }) = input::update_shot_input(&window, &mut shot_input)
            {
                if game.submit_shot(player_id, angle_degrees, velocity) {
                    shot_input.reset_for_player(game.current_turn);
                }
            }
        }
        game.update_animation();
        renderer.draw(&game, Some(&shot_input));
        window.update_with_buffer(&renderer.buffer, config.screen_width, config.screen_height)?;
    }

    Ok(())
}
