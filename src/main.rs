use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource, Default)]
struct Game {
    width: i8,
    height: i8,
}

fn setup(mut game: ResMut<Game>) {
    game.width = 32;
    game.height = 20;
}
