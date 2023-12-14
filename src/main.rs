use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, update)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Resource, Default)]
struct Game {
    width: usize,
    height: usize,
    board: Vec<Vec<bool>>,
}

#[derive(Component)]
struct Cell {
    alive: bool,
}

fn setup(mut commands: Commands, mut game: ResMut<Game>) {
    commands.spawn(Camera2dBundle::default());

    game.width = 32;
    game.height = 20;

    game.board = (0..game.height)
        .map(|j| {
            (0..game.width)
            .map(|i| {
                (i + j) % 2 == 0
            })
            .collect()
        })
        .collect();

    for (j, line) in game.board.iter().enumerate() {
        for (i, alive) in line.iter().enumerate() {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(112. / 255., 147. / 255., 204. / 255.),
                    custom_size: Some(Vec2::new(50.0, 100.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
                ..default()
            })
            .insert(Cell { alive: *alive });
        }
    }

}

fn update(mut game: ResMut<Game>) {
    let previous = &game.board;

    game.board = (0..game.height)
    .map(|j| {
        (0..game.width)
        .map(|i| {
            !previous[j][i]
        })
        .collect()
    })
    .collect()
}
