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
    game::{GameState, ScreenState},
    input::{MenuChoice, SetupInputEvent, SetupInputState, ShotInputEvent, ShotInputState},
    render::Renderer,
};

fn main() -> Result<(), minifb::Error> {
    let config = GameConfig::default();
    let mut game = GameState::new(config);
    game.screen = ScreenState::Intro;
    game.setup_completed = false;
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
    let mut setup_input = SetupInputState::new();
    let mut shot_input = ShotInputState::new(game.current_turn);

    while !input::quit_requested(&window) {
        match game.screen {
            ScreenState::Intro => {
                if input::any_continue_key_pressed(&window) {
                    game.continue_intro();
                }
            }
            ScreenState::Setup => {
                if let SetupInputEvent::Complete(setup) =
                    input::update_setup_input(&window, &mut setup_input)
                {
                    game.apply_setup(
                        setup.player1_name,
                        setup.player2_name,
                        setup.round_limit,
                        setup.gravity,
                    );
                }
            }
            ScreenState::Menu => match input::update_menu_input(&window) {
                Some(MenuChoice::ViewIntro) => game.view_intro(),
                Some(MenuChoice::PlayGame) => {
                    game.start_match();
                    shot_input.reset_for_player(game.current_turn);
                }
                None => {}
            },
            ScreenState::Playing => {
                if game.accepts_shot_input() {
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
            }
            ScreenState::GameOver => {
                if input::any_continue_key_pressed(&window) {
                    game.continue_after_game_over();
                    setup_input = SetupInputState::new();
                }
            }
        }

        if game.screen == ScreenState::Setup {
            renderer.draw_setup(Some(&setup_input));
        } else {
            renderer.draw(&game, Some(&shot_input));
        }
        window.update_with_buffer(&renderer.buffer, config.screen_width, config.screen_height)?;
    }

    Ok(())
}
