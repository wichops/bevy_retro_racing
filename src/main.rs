mod spawner;

mod prelude {
    pub use bevy::{prelude::*, time::FixedTimestep};
    pub use rand::prelude::*;

    pub const TOP_PADDING: f32 = 100.;
    pub const UI_WIDTH: f32 = 200.;
    pub const TILE_SIZE: f32 = 40.0;
    pub const HALF_TILE: f32 = TILE_SIZE / 2.;
    pub const COLUMN_SIZE: f32 = TILE_SIZE * 3.;

    pub const BORDER: usize = 1;
    pub const PADDING: usize = 1;

    pub const SCREEN_WIDTH: usize = 9 + BORDER * 2 + PADDING * 2;
    pub const SCREEN_HEIGHT: usize = 20;

    pub const WINDOW_PADDING: f32 = 20.0;
    pub const WINDOW_HEIGHT: f32 = TOP_PADDING + SCREEN_HEIGHT as f32 * TILE_SIZE + WINDOW_PADDING;
    pub const WINDOW_WIDTH: f32 = UI_WIDTH + SCREEN_WIDTH as f32 * TILE_SIZE + WINDOW_PADDING;

    pub const SCREEN_X: f32 = WINDOW_WIDTH as f32 / -2. + WINDOW_PADDING;
    pub const SCREEN_Y: f32 = WINDOW_HEIGHT as f32 / -2. + WINDOW_PADDING;

    #[derive(Component)]
    pub struct Player;

    #[derive(Component)]
    pub struct Enemy;

    pub use crate::spawner::*;
}

/*
 *
 * */
use prelude::*;

fn move_enemy(mut query: Query<&mut Transform, With<Enemy>>) {
    for mut enemy_transform in query.iter_mut() {
        enemy_transform.translation.y -= TILE_SIZE;
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.just_pressed(KeyCode::Left) {
        direction -= COLUMN_SIZE;
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        direction += COLUMN_SIZE;
    }

    player_transform.translation.x += direction;
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    println!("window height: {}", WINDOW_HEIGHT);
    println!("screen Y: {}", SCREEN_Y);

    let window = windows.primary_mut();
    window.center_window(MonitorSelection::Current);

    commands.spawn_bundle(Camera2dBundle::default());

    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.75, 0.75, 0.75),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        SCREEN_X + x as f32 * TILE_SIZE + HALF_TILE,
                        SCREEN_Y + y as f32 * TILE_SIZE + HALF_TILE,
                        0.0,
                    ),
                    ..default()
                },
                ..default()
            });
        }
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Carritos".to_string(),
            resizable: false,
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemy)
        .add_system(move_player)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(spawn_enemy),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.08))
                .with_system(move_enemy),
        )
        .run();
}
