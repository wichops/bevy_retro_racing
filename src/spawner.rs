use crate::prelude::*;

const CAR: [&str; 4] = ["_O_", "OOO", "_O_", "O_O"];
const ENEMY_CAR: [&str; 4] = ["O_O", "_O_", "OOO", "_O_"];

fn position_in_screen(column: usize) -> (f32, f32) {
    let pos_x = SCREEN_X + (column as f32 * COLUMN_SIZE) + (HALF_TILE * 3.) + TILE_SIZE * 2.;
    let pos_y = SCREEN_Y + SCREEN_HEIGHT as f32 * TILE_SIZE;

    (pos_x, pos_y)
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
                color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            };
            let scale = Vec3::new(0.8, 0.8, 0.0);

            for (y, line) in ENEMY_CAR.iter().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    let pos_x = x as f32 * TILE_SIZE - TILE_SIZE;
                    let pos_y = y as f32 * -TILE_SIZE + TILE_SIZE + HALF_TILE;

                    // let pos_y = y as f32 * TILE_SIZE - TILE_SIZE - HALF_TILE;

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
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.75, 0.25, 0.25),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(pos_x, pos_y, 0.0),
                ..default()
            },
            ..default()
        });
}

pub fn spawn_player(mut commands: Commands) {
    let column = 1;
    let (pos_x, _) = position_in_screen(column);
    let pos_y = SCREEN_Y + (HALF_TILE * 4.);

    commands
        .spawn()
        .insert(Car { column })
        .insert(Player)
        .with_children(|parent| {
            let sprite = Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            };
            let scale = Vec3::new(0.8, 0.8, 0.0);

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
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(pos_x, pos_y, 0.0),
                ..default()
            },
            ..default()
        });
}
