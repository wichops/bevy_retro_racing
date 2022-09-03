use crate::prelude::*;

pub fn spawn_enemy(mut commands: Commands) {
    let mut rng = thread_rng();
    let column = rng.gen_range(0..3);

    let pos_x = SCREEN_X + (column as f32 * COLUMN_SIZE) + (HALF_TILE * 3.) + TILE_SIZE * 2.;
    let pos_y = SCREEN_Y + SCREEN_HEIGHT as f32 * TILE_SIZE - (HALF_TILE * 4.);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.75, 0.25, 0.25),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(TILE_SIZE * 3., TILE_SIZE * 4., 0.0),
                translation: Vec3::new(pos_x, pos_y, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Enemy);
}

pub fn spawn_player(mut commands: Commands) {
    let column = 1;

    let pos_x = SCREEN_X + (column as f32 * COLUMN_SIZE) + (HALF_TILE * 3.) + TILE_SIZE * 2.;
    let pos_y = SCREEN_Y + (HALF_TILE * 4.);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(TILE_SIZE * 3., TILE_SIZE * 4., 1.0),
                translation: Vec3::new(pos_x, pos_y, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Player);
}
