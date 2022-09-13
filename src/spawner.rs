use crate::prelude::*;

const CAR: [&str; 4] = ["_O_", "OOO", "_O_", "O_O"];

const LEFT_WALL_X: f32 = SCREEN_X + HALF_TILE;
const RIGHT_WALL_X: f32 = SCREEN_X + SCREEN_WIDTH as f32 * TILE_SIZE - HALF_TILE;

fn anchor_sprite(x: f32, y: f32) -> SpriteBundle {
    let pos = Vec2::new(x, y);
    SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(0.0, 0.0, 0.0, 0.0),
            ..default()
        },
        transform: Transform {
            translation: pos.extend(0.0),
            ..default()
        },
        ..default()
    }
}

fn draw_car(parent: &mut ChildBuilder) {
    let sprite = Sprite {
        custom_size: Some(Vec2::splat(TILE_SIZE)),
        color: TILE_COLOR,
        ..default()
    };

    for (y, line) in CAR.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Vec2::new(
                x as f32 * TILE_SIZE - TILE_SIZE,
                y as f32 * -TILE_SIZE + TILE_SIZE + HALF_TILE,
            );

            if c == 'O' {
                parent.spawn_bundle(SpriteBundle {
                    sprite: sprite.clone(),
                    transform: Transform {
                        scale: TileScreen::tile_scale(),
                        translation: Vec2::extend(pos, 0.0),
                        ..default()
                    },
                    ..default()
                });
            }
        }
    }
}

fn draw_walls(parent: &mut ChildBuilder) {
    let sprite = Sprite {
        custom_size: Some(Vec2::splat(TILE_SIZE)),
        color: TILE_COLOR,
        ..default()
    };

    for y in 0..3 {
        let pos_y = y as f32 * TILE_SIZE - HALF_TILE;

        parent.spawn_bundle(SpriteBundle {
            sprite: sprite.clone(),
            transform: Transform {
                scale: TileScreen::tile_scale(),
                translation: Vec3::new(LEFT_WALL_X, pos_y, 0.0),
                ..default()
            },
            ..default()
        });

        parent.spawn_bundle(SpriteBundle {
            sprite: sprite.clone(),
            transform: Transform {
                scale: TileScreen::tile_scale(),
                translation: Vec3::new(RIGHT_WALL_X, pos_y, 0.0),
                ..default()
            },
            ..default()
        });
    }
}

pub fn spawn_walls(mut commands: Commands) {
    for y in 0..6 {
        let pos_y = SCREEN_Y as f32 + TILE_SIZE;
        let y_distance = y as f32 * TILE_SIZE * WALL_SPACING;

        commands
            .spawn()
            .insert(Wall)
            .insert(MoveY)
            .with_children(draw_walls)
            .insert_bundle(anchor_sprite(0.0, pos_y + y_distance));
    }
}

pub fn spawn_enemies(mut commands: Commands /* asset_server: Res<AssetServer> */) {
    let mut rng = thread_rng();

    for y in 0..4 {
        let column = rng.gen_range(0..3);
        let pos_x = TileScreen::column_to_coord(column);
        let pos_y = SCREEN_Y + SCREEN_HEIGHT as f32 * TILE_SIZE + TILE_SIZE * CAR_SPACING;
        let y_distance = y as f32 * TILE_SIZE * CAR_SPACING;

        commands
            .spawn()
            .insert(Car { column })
            .insert(MoveY)
            .insert(Enemy)
            .with_children(draw_car)
            .insert_bundle(anchor_sprite(pos_x, pos_y + y_distance));

        // [Debug] Show car numbers
        //
        // let text_style = TextStyle {
        //     font: asset_server.load("fonts/Calculator.ttf"),
        //     font_size: 40.0,
        //     color: Color::WHITE,
        // };
        // commands
        //     .spawn_bundle(Text2dBundle {
        //         text: Text::from_section(format!("{}", y), text_style.clone()),
        //         transform: Transform {
        //             translation: Vec3::new(pos_x, pos_y + y_distance, 1.0),
        //             ..default()
        //         },
        //         ..default()
        //     })
        //     .insert(MoveY);
    }
}

pub fn spawn_player(mut commands: Commands) {
    let column = 1;
    let pos_x = TileScreen::column_to_coord(column);

    commands
        .spawn()
        .insert(Car { column })
        .insert(Player)
        .with_children(draw_car)
        .insert_bundle(anchor_sprite(pos_x, PLAYER_Y));
}
