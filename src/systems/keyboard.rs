use crate::Game;
use bevy::{prelude::*, app::AppExit};

pub fn keyboard_input_system(
    mut game: ResMut<Game>,
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        game.pause = !game.pause;
    }

    if keyboard_input.just_pressed(KeyCode::N) {
        game.step = true;
    }

    if keyboard_input.just_pressed(KeyCode::Q) || keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }

    if keyboard_input.just_pressed(KeyCode::Back) {
        game.clear = true;
    }

    let glider_keys = [
        KeyCode::Key0,
        KeyCode::Key1,
        KeyCode::Key2,
        KeyCode::Key3,
        KeyCode::Key4,
        KeyCode::Key5,
        KeyCode::Key6,
        KeyCode::Key7,
        KeyCode::Key8,
        KeyCode::Key9,
    ];

    for (count, &code) in glider_keys.iter().enumerate() {
        if keyboard_input.just_pressed(code) {
            game.add_gliders = count as i32;
        }
    }
}
