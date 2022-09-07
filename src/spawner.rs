use crate::prelude::*;

const CAR: [&str; 4] = ["_O_", "OOO", "_O_", "O_O"];

const LEFT_WALL_X: f32 = SCREEN_X + HALF_TILE;
const RIGHT_WALL_X: f32 = SCREEN_X + SCREEN_WIDTH as f32 * TILE_SIZE - HALF_TILE;

fn anchor_sprite(translation: Vec3) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(0.0, 0.0, 0.0, 0.0),
            ..default()
        },
        transform: Transform {
            translation,
            ..default()
        },
        ..default()
    }
}

fn position_in_screen(column: usize) -> (f32, f32) {
    let pos_x = SCREEN_X + (column as f32 * COLUMN_SIZE) + (HALF_TILE * 3.) + TILE_SIZE * 2.;
    let pos_y = SCREEN_Y + SCREEN_HEIGHT as f32 * TILE_SIZE;

    (pos_x, pos_y)
}

pub fn spawn_walls(mut commands: Commands) {
    commands
        .spawn()
        .insert(Wall)
        .with_children(|parent| {
            let sprite = Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                color: TILE_COLOR,
                ..default()
            };
            let scale = Vec3::new(0.75, 0.75, 0.0);

            for y in 0..3 {
                let pos_y = y as f32 * TILE_SIZE - HALF_TILE;

                parent.spawn_bundle(SpriteBundle {
                    sprite: sprite.clone(),
                    transform: Transform {
                        scale,
                        translation: Vec3::new(LEFT_WALL_X, pos_y, 0.0),
                        ..default()
                    },
                    ..default()
                });

                parent.spawn_bundle(SpriteBundle {
                    sprite: sprite.clone(),
                    transform: Transform {
                        scale,
                        translation: Vec3::new(RIGHT_WALL_X, pos_y, 0.0),
                        ..default()
                    },
                    ..default()
                });
            }
        })
        .insert_bundle(anchor_sprite(Vec3::new(
            0.0,
            SCREEN_Y + SCREEN_HEIGHT as f32 * TILE_SIZE,
            0.0,
        )));
}

pub fn spawn_enemy(mut commands: Commands) {
    let mut rng = thread_rng();
    let column = rng.gen_range(0..3);

    let (pos_x, pos_y) = position_in_screen(column);

    commands
        .spawn()
        .insert(Car { column })
        .insert(Enemy)
        .with_children(|parent| {
            let sprite = Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                color: TILE_COLOR,
                ..default()
            };
            let scale = Vec3::new(0.75, 0.75, 0.0);

            for (y, line) in CAR.iter().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    let pos_x = x as f32 * TILE_SIZE - TILE_SIZE;
                    let pos_y = y as f32 * -TILE_SIZE + TILE_SIZE + HALF_TILE;

                    if c == 'O' {
                        parent.spawn_bundle(SpriteBundle {
                            sprite: sprite.clone(),
                            transform: Transform {
                                scale,
                                translation: Vec3::new(pos_x, pos_y, 0.0),
                                ..default()
                            },
                            ..default()
                        });
                    }
                }
            }
        })
        .insert_bundle(anchor_sprite(Vec3::new(pos_x, pos_y, 0.0)));
}

pub fn spawn_player(mut commands: Commands) {
    let column = 1;
    let (pos_x, _) = position_in_screen(column);

    commands
        .spawn()
        .insert(Car { column })
        .insert(Player)
        .with_children(|parent| {
            let sprite = Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                color: TILE_COLOR,
                ..default()
            };
            let scale = Vec3::new(0.75, 0.75, 0.0);

            for (y, line) in CAR.iter().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    let pos_x = x as f32 * TILE_SIZE - TILE_SIZE;
                    let pos_y = y as f32 * -TILE_SIZE + TILE_SIZE + HALF_TILE;

                    if c == 'O' {
                        parent.spawn_bundle(SpriteBundle {
                            sprite: sprite.clone(),
                            transform: Transform {
                                scale,
                                translation: Vec3::new(pos_x, pos_y, 0.0),
                                ..default()
                            },
                            ..default()
                        });
                    }
                }
            }
        })
        .insert_bundle(anchor_sprite(Vec3::new(pos_x, PLAYER_Y, 0.0)));
}
