use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::{events::*, AppState};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2d).insert(Transform {
        translation: Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) && *app_state.get() != AppState::Game {
        next_app_state.set(AppState::Game);
        println!("Entered AppState::Game");
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) && *app_state.get() != AppState::MainMenu {
        next_app_state.set(AppState::MainMenu);
        println!("Entered AppState::MainMenu");
    }
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit::Success);
    }
}

pub fn handle_game_over(
    mut next_state: ResMut<NextState<AppState>>,
    mut game_over_event_reader: EventReader<GameOver>,
) {
    for event in game_over_event_reader.read() {
        next_state.set(AppState::GameOver);
        println!("Your final score is: {}", event.score);
    }
}
