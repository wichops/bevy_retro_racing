use bevy::{prelude::*, time::FixedTimestep};
use rand::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people)
            .add_system(yes_or_what_people);
    }
}

fn yes_or_what_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    if timer.0.tick(time.delta()).finished() {
        for person in query.iter() {
            println!("Si o que {}", person.0);
        }
    }
}

fn add_people(mut commands: Commands) {
    let names = ["Pendejo", "Teraflu", "Mequin"];

    for name in names.iter() {
        commands
            .spawn()
            .insert(Person)
            .insert(Name(name.to_string()));
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

fn spawn_enemy(mut commands: Commands) {
    let mut rng = thread_rng();
    let column = rng.gen_range(0..3);

    let pos_x = column as f32 * 100.0 - 100.0;

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.75, 0.25, 0.25),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(100.0, 100.0, 0.0),
                translation: Vec3::new(pos_x, 200.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Enemy);
}

fn move_enemy(mut query: Query<&mut Transform, With<Enemy>>) {
    for mut enemy_transform in query.iter_mut() {
        enemy_transform.translation.y -= 10.0;
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.just_pressed(KeyCode::Left) {
        direction -= 100.0;
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        direction += 100.0;
    }

    player_transform.translation.x += direction;
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.25),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(100.0, 100.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Player);
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Carritos".to_string(),
            resizable: false,
            width: 400.,
            height: 600.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(2.0))
                .with_system(spawn_enemy),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.08))
                .with_system(move_enemy),
        )
        .run();
}
