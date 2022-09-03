mod spawner;

mod prelude {
    pub use bevy::{prelude::*, time::FixedTimestep};
    pub use rand::prelude::*;

    pub const TOP_PADDING: f32 = 100.;
    pub const UI_WIDTH: f32 = 260.;
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

    pub struct Scoreboard {
        pub score: usize,
        pub highscore: usize,
    }

    #[derive(Default)]
    pub struct CollisionEvent;

    pub use crate::spawner::*;
}

/*
 *
 * */
use prelude::*;

fn move_enemy(mut commands: Commands, mut query: Query<(Entity, &mut Transform), With<Enemy>>) {
    for (entity, mut enemy_transform) in query.iter_mut() {
        enemy_transform.translation.y -= TILE_SIZE;
        if enemy_transform.translation.y < SCREEN_Y * 2.0 {
            commands.entity(entity).despawn();
        }
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

fn check_collisions(
    player_query: Query<&Transform, With<Player>>,
    enemies_query: Query<&Transform, With<Enemy>>,
    mut scoreboard: ResMut<Scoreboard>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let player_transform = player_query.single();
    let top_bound = -360.0 + TILE_SIZE * 4.0;
    let bottom_bound = -360.0 - TILE_SIZE * 4.0;

    for enemy_transform in &enemies_query {
        let enemy_y = enemy_transform.translation.y;
        let enemy_x = enemy_transform.translation.x;

        if enemy_y < top_bound
            && enemy_y > bottom_bound
            && enemy_x == player_transform.translation.x
        {
            collision_events.send_default();
            scoreboard.highscore = scoreboard.score;
            scoreboard.score = 0;
        }
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}

fn increment_scoreboard(mut scoreboard: ResMut<Scoreboard>) {
    scoreboard.score += 100;
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>, asset_server: Res<AssetServer>) {
    let window = windows.primary_mut();
    window.center_window(MonitorSelection::Current);
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/Terminess-Mono.ttf"),
                    font_size: 40.0,
                    color: Color::BLACK,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/Terminess-Mono.ttf"),
                font_size: 40.0,
                color: Color::BLACK,
            }),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/Terminess-Mono.ttf"),
                font_size: 40.0,
                color: Color::BLACK,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(100.),
                right: Val::Px(20.),
                ..default()
            },
            ..default()
        }),
    );

    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.0, 0.0, 0.0, 0.1),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        SCREEN_X + x as f32 * TILE_SIZE + HALF_TILE,
                        SCREEN_Y + y as f32 * TILE_SIZE + HALF_TILE,
                        0.0,
                    ),
                    scale: Vec3::new(0.8, 0.8, 1.0),
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
        .insert_resource(Scoreboard {
            score: 0,
            highscore: 0,
        })
        .insert_resource(ClearColor(Color::hex("899774").unwrap()))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemy)
        .add_event::<CollisionEvent>()
        .add_system(check_collisions)
        .add_system(update_scoreboard)
        .add_system(move_player.before(check_collisions))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(spawn_enemy)
                .with_system(increment_scoreboard),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.08))
                .with_system(move_enemy.before(check_collisions)),
        )
        .run();
}
