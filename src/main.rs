mod board;

use bevy::prelude::*;
use board::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, update)
        .insert_resource(Time::<Fixed>::from_seconds(1. / 15.))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Resource, Default)]
struct Game {
    width: usize,
    height: usize,
    board: Vec<Vec<bool>>,
}

#[derive(Component, Debug)]
struct Cell {
    i: usize,
    j: usize,
    alive: bool,
}

fn setup(mut commands: Commands, mut game: ResMut<Game>) {
    commands.spawn(Camera2dBundle::default());

    game.width = 32;
    game.height = 20;

    game.board = create_board_with_glider(game.width, game.height);

    for (j, line) in game.board.iter().enumerate() {
        for (i, &alive) in line.iter().enumerate() {
            let color = get_color(alive);

            let x = 10. * i as f32;
            let y = 10. * j as f32;

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                    ..default()
                })
                .insert(Cell { i, j, alive });
        }
    }
}

fn update(mut game: ResMut<Game>, mut query: Query<(&mut Sprite, &mut Cell)>) {
    game.board = update_board(&game.board);

    for (mut sprite, mut cell) in &mut query {
        cell.alive = game.board[cell.j][cell.i];
        sprite.color = get_color(cell.alive)
    }
}

fn get_color(alive: bool) -> Color {
    if alive {
        Color::rgb(112. / 255., 147. / 255., 204. / 255.)
    } else {
        Color::rgb(38. / 255., 82. / 255., 153. / 255.)
    }
}
