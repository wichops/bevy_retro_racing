mod entities;
mod spawner;
mod systems;

mod prelude {
    pub use bevy::{audio::AudioSink, ecs::schedule::ShouldRun, prelude::*, time::FixedTimestep};
    pub use rand::prelude::*;

    pub const BG_COLOR: &str = "8d9e7b";
    pub const FONT_SIZE: f32 = 48.0;

    pub const PLAYER_Y: f32 = SCREEN_Y + (HALF_TILE * 4.);
    pub const TILE_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.98);

    pub const UI_WIDTH: f32 = 240.0;
    pub const TILE_SIZE: f32 = 32.0;
    pub const HALF_TILE: f32 = TILE_SIZE / 2.;
    pub const COLUMN_SIZE: f32 = TILE_SIZE * 3.;

    pub const PADDING: usize = 2;

    pub const WINDOW_PADDING: f32 = 20.0;
    pub const WINDOW_HEIGHT: f32 = SCREEN_HEIGHT as f32 * TILE_SIZE + WINDOW_PADDING * 2.0;
    pub const WINDOW_WIDTH: f32 = UI_WIDTH + SCREEN_WIDTH as f32 * TILE_SIZE + WINDOW_PADDING;

    pub const SCREEN_X: f32 = WINDOW_WIDTH as f32 / -2. + WINDOW_PADDING;
    pub const SCREEN_Y: f32 = WINDOW_HEIGHT as f32 / -2. + WINDOW_PADDING;
    pub const SCREEN_WIDTH: usize = 9 + PADDING * 2;
    pub const SCREEN_HEIGHT: usize = 20;

    pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

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

    pub use crate::entities::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}

use prelude::*;

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
                .with_system(spawn_player),
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
                .with_system(accelerate.before(check_collisions)),
        )
        .run();
}

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
