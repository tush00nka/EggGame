use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::hsl(20., 0.2, 0.5)))
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}