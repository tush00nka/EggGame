use bevy::{prelude::*, window::WindowResolution};
use avian2d::prelude::*;

mod camera;
use camera::CameraPlugin;

mod platform;
use platform::PlatformPlugin;

mod egg;
use egg::EggPlugin;

mod game_over_screen;
use game_over_screen::GameOverScreenPlugin;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
pub enum GameState {
    Menu,
    #[default]
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Egg Game))".to_string(),
                        resizable: false,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: WindowResolution::new(800., 600.),
                        ..default()
                    }),
                    ..default()
                })
            .set(ImagePlugin::default_nearest()))

        .add_plugins(PhysicsPlugins::new(FixedUpdate))
        .insert_resource(Gravity(Vec2::NEG_Y * 1_000.))

        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()

        .add_plugins((
            CameraPlugin,
            PlatformPlugin,
            EggPlugin,
            GameOverScreenPlugin,
        ))

        .run();
}
