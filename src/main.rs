mod board;
mod systems;

use bevy::{prelude::*, render::camera, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board::*;
use clap::Parser;
use systems::{keyboard::keyboard_input_system, mouse::mouse_input_system};

#[derive(Parser)]
#[command(author, version)]
#[command(about = "Using Bevy to simulate the Game of Life", long_about = None)]
struct Cli {
    /// Width of the simulation
    #[arg(short, long)]
    #[arg(default_value = "32")]
    width: usize,

    /// Height of the simulation
    #[arg(short = 'H', long)]
    #[arg(default_value = "20")]
    height: usize,

    /// Size of a cell in pixels
    #[arg(short, long)]
    #[arg(default_value = "40")]
    size: usize,

    /// Framerate of the simulation
    #[arg(short, long)]
    #[arg(default_value = "15")]
    framerate: usize,
}

fn main() {
    let cli = Cli::parse();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(
                    cli.width as f32 * cli.size as f32,
                    cli.height as f32 * cli.size as f32,
                ),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(Game {
            width: cli.width,
            height: cli.height,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, update)
        .insert_resource(Time::<Fixed>::from_seconds(1. / cli.framerate as f64))
        .add_systems(Update, (mouse_input_system, keyboard_input_system))
        .run();
}

#[derive(Resource, Default)]
struct Game {
    selecting: bool,
    cursor_positions: Vec<Vec2>,
    pause: bool,
    clear: bool,
    add_gliders: i32,
    step: bool,
    width: usize,
    height: usize,
    board: Board,
}

#[derive(Component, Debug)]
struct Cell {
    i: usize,
    j: usize,
    alive: bool,
}

fn setup(mut commands: Commands, mut game: ResMut<Game>) {
    game.board = Board::with_gliders(game.width, game.height);
    game.cursor_positions = vec![];

    let width = game.width as f32;
    let height = game.height as f32;

    let mut camera = Camera2dBundle {
        transform: Transform::from_xyz(width / 2.0, height / 2.0, 0.),
        ..Default::default()
    };
    camera.projection.scaling_mode = camera::ScalingMode::Fixed { width, height };
    commands.spawn(camera);

    let board_id = commands
        .spawn(SpatialBundle {
            transform: Transform::from_translation(Vec3::new(0.5, 0.5, 0.)),
            ..default()
        })
        .insert(Name::new("Board"))
        .id();

    for (j, row) in game.board.rows().enumerate() {
        let row_entity = commands
            .spawn(SpatialBundle {
                transform: Transform::from_translation(Vec3::new(0., j as f32, 0.)),
                ..default()
            })
            .insert(Name::new(format!("Row {j}")))
            .id();
        commands.entity(board_id).push_children(&[row_entity]);

        for (i, &alive) in row.iter().enumerate() {
            let child = commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: get_color(alive),
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., 0.)),
                    ..default()
                })
                .insert(Cell { i, j, alive })
                .insert(Name::new(format!("Cell ({i}, {j})")))
                .id();

            commands.entity(row_entity).push_children(&[child]);
        }
    }
}

fn update(
    mut game: ResMut<Game>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(&mut Sprite, &mut Cell)>,
) {
    let (camera, camera_transform) = camera_query.single();

    if game.clear {
        game.board = Board::empty(game.board.width(), game.board.height());
        game.clear = false;
    }

    if !game.pause || game.step {
        game.board = game.board.update();
        game.step = false;
    }

    if game.add_gliders > 0 {
        let count = game.add_gliders;
        game.board.add_gliders(count);
        game.add_gliders = 0;
    }

    for viewport_position in game.cursor_positions.clone() {
        let point = camera
            .viewport_to_world_2d(camera_transform, viewport_position)
            .unwrap();

        if point.x >= 0.0
            && point.y >= 0.0
            && point.x < game.width as f32
            && point.y < game.height as f32
        {
            let x = point.x as usize;
            let y = point.y as usize;

            game.board.set_alive(x, y);
        }
    }
    game.cursor_positions = vec![];

    for (mut sprite, mut cell) in &mut query {
        cell.alive = game.board.is_alive(cell.i, cell.j);
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
