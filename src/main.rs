use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (update, render))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Resource, Default)]
struct Game {
    width: usize,
    height: usize,
    board: Vec<Vec<Cell>>,
}

struct Cell {
    alive: bool,
}

fn setup(mut game: ResMut<Game>) {
    game.width = 32;
    game.height = 20;

    game.board = (0..game.height)
        .map(|j| {
            (0..game.width)
            .map(|i| {
                let alive = (i + j) % 2 == 0;
                Cell { alive }
            })
            .collect()
        })
        .collect()
}

fn update(mut game: ResMut<Game>) {
    let previous = &game.board;

    game.board = (0..game.height)
    .map(|j| {
        (0..game.width)
        .map(|i| {
            let alive = !previous[j][i].alive;
            Cell { alive }
        })
        .collect()
    })
    .collect()
}

fn render(game: Res<Game>) {
    println!("Value @ (5, 3) = {}", game.board[5][3].alive)
}
