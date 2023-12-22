use crate::Game;
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    window::PrimaryWindow,
};

pub fn mouse_input_system(
    mut game: ResMut<Game>,
    mut button: EventReader<MouseButtonInput>,
    mut cursor: EventReader<CursorMoved>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    for event in button.read() {
        game.selecting = match event.state {
            ButtonState::Pressed => true,
            ButtonState::Released => false,
        };

        if game.selecting {
            if let Some(position) = q_windows.single().cursor_position() {
                game.cursor_positions.push(position);
            }
        }
    }

    if game.selecting {
        for event in cursor.read() {
            game.cursor_positions.push(event.position);
        }
    }
}
