mod board;

use bevy::{
    app::AppExit,
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    render::camera,
    window::{PrimaryWindow, WindowResolution},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board::*;
use clap::Parser;

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
    board: Vec<Vec<bool>>,
}

#[derive(Component, Debug)]
struct Cell {
    i: usize,
    j: usize,
    alive: bool,
}

fn setup(mut commands: Commands, mut game: ResMut<Game>) {
    game.board = create_board_with_gliders(game.width, game.height);
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

    for (j, line) in game.board.iter().enumerate() {
        let row = commands
            .spawn(SpatialBundle {
                transform: Transform::from_translation(Vec3::new(0., j as f32, 0.)),
                ..default()
            })
            .insert(Name::new(format!("Row {j}")))
            .id();
        commands.entity(board_id).push_children(&[row]);

        for (i, &alive) in line.iter().enumerate() {
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

            commands.entity(row).push_children(&[child]);
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
        game.board = create_board_empty(game.width, game.height);
        game.clear = false;
    }

    if !game.pause || game.step {
        game.board = update_board(&game.board);
        game.step = false;
    }

    if game.add_gliders > 0 {
        let count = game.add_gliders;
        let width = game.width;
        let height = game.height;

        add_gliders_to_board(&mut game.board, count, width, height);
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

            set_alive(&mut game.board, x, y);
        }
    }
    game.cursor_positions = vec![];

    for (mut sprite, mut cell) in &mut query {
        cell.alive = game.board[cell.j][cell.i];
        sprite.color = get_color(cell.alive)
    }
}

fn mouse_input_system(
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

fn keyboard_input_system(
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

fn get_color(alive: bool) -> Color {
    if alive {
        Color::rgb(112. / 255., 147. / 255., 204. / 255.)
    } else {
        Color::rgb(38. / 255., 82. / 255., 153. / 255.)
    }
}
