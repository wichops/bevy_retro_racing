mod spawner;

mod prelude {
    pub use bevy::{audio::AudioSink, ecs::schedule::ShouldRun, prelude::*, time::FixedTimestep};

    pub use rand::prelude::*;

    pub const BG_COLOR: &str = "8d9e7b";
    pub const FONT_SIZE: f32 = 36.0;

    pub const PLAYER_Y: f32 = SCREEN_Y + (HALF_TILE * 4.);
    pub const TILE_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.98);

    pub const UI_WIDTH: f32 = 260.;
    pub const TILE_SIZE: f32 = 32.0;
    pub const HALF_TILE: f32 = TILE_SIZE / 2.;
    pub const COLUMN_SIZE: f32 = TILE_SIZE * 3.;

    pub const BORDER: usize = 1;
    pub const PADDING: usize = 1;

    pub const SCREEN_WIDTH: usize = 9 + BORDER * 2 + PADDING * 2;
    pub const SCREEN_HEIGHT: usize = 20;

    pub const WINDOW_PADDING: f32 = 20.0;
    pub const WINDOW_HEIGHT: f32 = SCREEN_HEIGHT as f32 * TILE_SIZE + WINDOW_PADDING * 2.0;
    pub const WINDOW_WIDTH: f32 = UI_WIDTH + SCREEN_WIDTH as f32 * TILE_SIZE + WINDOW_PADDING;

    pub const SCREEN_X: f32 = WINDOW_WIDTH as f32 / -2. + WINDOW_PADDING;
    pub const SCREEN_Y: f32 = WINDOW_HEIGHT as f32 / -2. + WINDOW_PADDING;

    #[derive(Component)]
    pub struct Player;

    #[derive(Component)]
    pub struct Enemy;

    #[derive(Component)]
    pub struct Wall;

    #[derive(Component)]
    pub struct Car {
        pub column: usize,
    }

    #[derive(Default)]
    pub struct ScoreEntities {
        pub score: Option<Entity>,
        pub highscore: Option<Entity>,
    }

    #[derive(Default)]
    pub struct Scoreboard {
        pub score: usize,
        pub highscore: usize,
        pub entities: ScoreEntities,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub enum GameState {
        Menu,
        Playing,
    }

    pub struct MenuData {
        pub button_entity: Entity,
    }

    pub use crate::spawner::*;
}

use bevy::audio::AudioSink;
use prelude::*;
use std::cmp;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

/*
 * This is necessary because using with_run_criteria multiple times
 * overrides the existing SystemSet
 *
 * The fix was found in this thread:
 * https://github.com/bevyengine/bevy/issues/1839#issuecomment-835807108
*/
fn run_if_playing(In(input): In<ShouldRun>, state: Res<State<GameState>>) -> ShouldRun {
    if *state.current() == GameState::Playing {
        input
    } else {
        ShouldRun::No
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_entity = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                position: UiRect {
                    right: Val::Px((UI_WIDTH - WINDOW_PADDING) / 2.0),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Play",
                TextStyle {
                    font: asset_server.load("fonts/Calculator.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .id();
    commands.insert_resource(MenuData { button_entity });
}

fn menu(
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                state.set(GameState::Playing).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
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
        .init_resource::<Scoreboard>()
        .insert_resource(ClearColor(Color::hex(BG_COLOR).unwrap()))
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Menu)
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
        .add_system_set(SystemSet::on_update(GameState::Menu).with_system(menu))
        .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup_menu))
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(play_motor_sound)
                .with_system(spawn_player)
                .with_system(spawn_enemy)
                .with_system(spawn_walls),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(update_scoreboard)
                .with_system(move_player.before(check_collisions))
                .with_system(play_explosion_sound.after(check_collisions)),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.4).chain(run_if_playing))
                .with_system(spawn_walls),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(increment_scoreboard),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.72).chain(run_if_playing))
                .with_system(spawn_enemy),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.08).chain(run_if_playing))
                .with_system(check_collisions)
                .with_system(move_wall)
                .with_system(move_enemy.before(check_collisions)),
        )
        .run();
}

#[derive(Default)]
struct CollisionEvent;

#[derive(Default)]
struct Explosion(Handle<AudioSource>);

#[derive(Default)]
struct MotorSound(Handle<AudioSource>);

struct MotorController(Handle<AudioSink>);

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
    mut score_resource: ResMut<Scoreboard>,
) {
    let window = windows.primary_mut();
    window.center_window(MonitorSelection::Current);
    commands.spawn_bundle(Camera2dBundle::default());

    let explosion_sound = asset_server.load("sounds/explosion.ogg");
    commands.insert_resource(Explosion(explosion_sound));

    let motor_sound = asset_server.load("sounds/motor.ogg");
    commands.insert_resource(MotorSound(motor_sound));

    let text_style = TextStyle {
        font: asset_server.load("fonts/Calculator.ttf"),
        font_size: FONT_SIZE,
        color: Color::BLACK,
    };

    let text_alignment = TextAlignment::CENTER_RIGHT;

    score_resource.entities.score = Some(
        commands
            .spawn_bundle(
                TextBundle::from_sections([
                    TextSection::new("SCORE\n", text_style.clone()),
                    TextSection::from_style(text_style.clone()),
                ])
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    justify_content: JustifyContent::FlexEnd,
                    position: UiRect {
                        top: Val::Px(60.),
                        right: Val::Px(20.),
                        ..default()
                    },
                    ..default()
                })
                .with_text_alignment(text_alignment),
            )
            .id(),
    );

    score_resource.entities.highscore = Some(
        commands
            .spawn_bundle(
                TextBundle::from_sections([
                    TextSection::new("HISCORE\n", text_style.clone()),
                    TextSection::from_style(text_style),
                ])
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    justify_content: JustifyContent::FlexEnd,
                    position: UiRect {
                        top: Val::Px(140.),
                        right: Val::Px(20.),
                        ..default()
                    },
                    ..default()
                })
                .with_text_alignment(text_alignment),
            )
            .id(),
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

fn play_motor_sound(
    mut commands: Commands,
    sound: Res<MotorSound>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let handle = audio_sinks.get_handle(
        audio.play_with_settings(sound.0.clone(), PlaybackSettings::LOOP.with_volume(0.9)),
    );

    commands.insert_resource(MotorController(handle));
}

fn play_explosion_sound(
    collision_events: EventReader<CollisionEvent>,
    audio: Res<Audio>,
    sound: Res<Explosion>,
) {
    if !collision_events.is_empty() {
        collision_events.clear();
        audio.play(sound.0.clone());
    }
}

fn move_enemy(mut commands: Commands, mut query: Query<(Entity, &mut Transform), With<Enemy>>) {
    for (entity, mut enemy_transform) in query.iter_mut() {
        enemy_transform.translation.y -= TILE_SIZE;
        if enemy_transform.translation.y < SCREEN_Y * 2.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn move_wall(mut commands: Commands, mut query: Query<(Entity, &mut Transform), With<Wall>>) {
    for (entity, mut wall_transform) in query.iter_mut() {
        wall_transform.translation.y -= TILE_SIZE;
        if wall_transform.translation.y < SCREEN_Y * 2.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Car, &mut Transform), With<Player>>,
    state: Res<State<GameState>>,
) {
    if *state.current() != GameState::Playing {
        return;
    }
    let (mut car, mut player_transform) = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.just_pressed(KeyCode::Left) && car.column > 0 {
        direction -= COLUMN_SIZE;
        car.column -= 1;
    }

    if keyboard_input.just_pressed(KeyCode::Right) && car.column < 2 {
        direction += COLUMN_SIZE;
        car.column += 1;
    }

    player_transform.translation.x += direction;
}

fn check_collisions(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    enemies_query: Query<(Entity, &Transform), With<Enemy>>,
    mut scoreboard: ResMut<Scoreboard>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let player_transform = player_query.single();
    let top_bound = PLAYER_Y + TILE_SIZE * 4.0;
    let bottom_bound = PLAYER_Y - TILE_SIZE * 4.0 - HALF_TILE;

    for (_, enemy_transform) in &enemies_query {
        let enemy_y = enemy_transform.translation.y;
        let enemy_x = enemy_transform.translation.x;

        if enemy_y < top_bound
            && enemy_y > bottom_bound
            && enemy_x == player_transform.translation.x
        {
            collision_events.send_default();

            for (entity, _) in &enemies_query {
                commands.entity(entity).despawn_recursive();
            }

            scoreboard.highscore = cmp::max(scoreboard.highscore, scoreboard.score);
            scoreboard.score = 0;
        }
    }
}

fn update_scoreboard(
    score_resource: Res<Scoreboard>,
    mut score_query: Query<&mut Text>,
    state: Res<State<GameState>>,
) {
    if *state.current() != GameState::Playing {
        return;
    }

    score_query
        .get_mut(score_resource.entities.score.unwrap())
        .unwrap()
        .sections[1]
        .value = score_resource.score.to_string();

    score_query
        .get_mut(score_resource.entities.highscore.unwrap())
        .unwrap()
        .sections[1]
        .value = score_resource.highscore.to_string();
}

fn increment_scoreboard(mut scoreboard: ResMut<Scoreboard>, state: Res<State<GameState>>) {
    if *state.current() != GameState::Playing {
        return;
    }
    scoreboard.score += 100;
}
