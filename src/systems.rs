pub use crate::prelude::*;
use std::cmp;

#[derive(Resource)]
pub struct ExplosionSound(Handle<AudioSource>);

#[derive(Resource)]
pub struct MotorSound(Handle<AudioSource>);

#[derive(Resource)]
pub struct MotorController(Handle<AudioSink>);

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_entity = commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(45.0)),
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
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Play",
                TextStyle {
                    font: asset_server.load("fonts/Calculator.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .id();
    commands.insert_resource(MenuData { button_entity });
}

pub fn menu(
    mut state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
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

    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::Playing).unwrap();
    }
}

pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
}

pub fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
    mut score_resource: ResMut<Scoreboard>,
) {
    let window = windows.primary_mut();
    window.center_window(MonitorSelection::Current);
    commands.spawn(Camera2dBundle::default());

    let explosion_sound = asset_server.load("sounds/explosion.ogg");
    commands.insert_resource(ExplosionSound(explosion_sound));

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
            .spawn(
                TextBundle::from_sections([
                    TextSection::new("SCORE\n", text_style.clone()),
                    TextSection::new(score_resource.score.to_string(), text_style.clone()),
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
            .spawn(
                TextBundle::from_sections([
                    TextSection::new("HISCORE\n", text_style.clone()),
                    TextSection::new(score_resource.highscore.to_string(), text_style),
                ])
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    justify_content: JustifyContent::FlexEnd,
                    position: UiRect {
                        top: Val::Px(120.),
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
            commands.spawn(SpriteBundle {
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

pub fn play_motor_sound(
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

pub fn play_explosion_sound(
    collision_events: EventReader<CollisionEvent>,
    audio: Res<Audio>,
    sound: Res<ExplosionSound>,
) {
    if !collision_events.is_empty() {
        collision_events.clear();
        audio.play(sound.0.clone());
    }
}

pub fn enemy_respawn(mut enemies_query: Query<&mut Transform, With<Enemy>>) {
    for mut enemy_transform in enemies_query.iter_mut() {
        if enemy_transform.translation.y < PLAYER_Y - TILE_SIZE * (CAR_SPACING - 1.0) {
            let mut rng = thread_rng();
            let column = rng.gen_range(0..3);

            let pos_y = SCREEN_Y + SCREEN_HEIGHT as f32 * TILE_SIZE + TILE_SIZE * CAR_SPACING;
            enemy_transform.translation.y = pos_y;

            enemy_transform.translation.x = TileScreen::column_to_coord(column);
        }
    }
}

pub fn wall_respawn(mut walls_query: Query<&mut Transform, With<Wall>>) {
    for mut wall_transform in walls_query.iter_mut() {
        if wall_transform.translation.y < PLAYER_Y - TILE_SIZE * (WALL_SPACING + 1.0) {
            let pos_y = SCREEN_Y + SCREEN_HEIGHT as f32 * TILE_SIZE + TILE_SIZE * WALL_SPACING;
            wall_transform.translation.y = pos_y;
        }
    }
}
pub fn accelerate(
    mut query: Query<&mut Transform, With<MoveY>>,
    mut game_data: ResMut<GameData>,
    timer: Res<Time>,
) {
    let delta = timer.delta();
    let boost_factor = game_data.boost_factor;
    let speed_factor = game_data.speed_factor;

    if game_data.is_boosting {
        game_data
            .move_timer
            .tick(delta.mul_f32(boost_factor * speed_factor));
    } else {
        game_data.move_timer.tick(delta.mul_f32(speed_factor));
    }

    if !game_data.move_timer.finished() {
        return;
    }

    for mut entity_transform in query.iter_mut() {
        entity_transform.translation.y -= TILE_SIZE;
    }
}

pub fn boost_player(keyboard_input: Res<Input<KeyCode>>, mut game_data: ResMut<GameData>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        game_data.is_boosting = true;
    }

    if keyboard_input.just_released(KeyCode::Space) {
        game_data.is_boosting = false;
    }
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Car, &mut Transform), With<Player>>,
    state: Res<State<GameState>>,
) {
    if *state.current() != GameState::Playing {
        return;
    }

    let (mut car, mut player_transform) = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.any_just_pressed([KeyCode::Left, KeyCode::A]) && car.column > 0 {
        direction -= COLUMN_SIZE;
        car.column -= 1;
    }

    if keyboard_input.any_just_pressed([KeyCode::Right, KeyCode::D]) && car.column < 2 {
        direction += COLUMN_SIZE;
        car.column += 1;
    }

    player_transform.translation.x += direction;
}

pub fn check_collisions(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemies_query: Query<(Entity, &Transform), With<Enemy>>,
    walls_query: Query<(Entity, &Transform), With<Wall>>,
    mut scoreboard: ResMut<Scoreboard>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut state: ResMut<State<GameState>>,
    mut game_data: ResMut<GameData>,
    audio_sinks: Res<Assets<AudioSink>>,
    motor_controller: Res<MotorController>,
) {
    let (player_entity, player_transform) = player_query.single();
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

            for (entity, _) in &walls_query {
                commands.entity(entity).despawn_recursive();
            }

            for (entity, _) in &enemies_query {
                commands.entity(entity).despawn_recursive();
            }

            commands.entity(player_entity).despawn_recursive();

            scoreboard.highscore = cmp::max(scoreboard.highscore, scoreboard.score);
            scoreboard.score = 0;
            game_data.speed_factor = 1.0;

            if let Some(sink) = audio_sinks.get(&motor_controller.0) {
                sink.pause();
                commands.remove_resource::<MotorController>();
            }

            state.set(GameState::Menu).unwrap();
        }
    }
}

pub fn update_scoreboard(score_resource: Res<Scoreboard>, mut score_query: Query<&mut Text>) {
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

pub fn increment_scoreboard(
    mut scoreboard: ResMut<Scoreboard>,
    mut game_data: ResMut<GameData>,
    state: Res<State<GameState>>,
) {
    if *state.current() != GameState::Playing {
        return;
    }
    scoreboard.score += 100;

    let factor = (scoreboard.score as f32 / 1000.0).floor() * 0.02 + 1.0;
    game_data.speed_factor = factor;
}
