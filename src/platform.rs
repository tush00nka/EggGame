use bevy::prelude::*;
use avian2d::prelude::*;

use rand::Rng;

use crate::GameState;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), spawn_first_platform)
            .add_systems(Update, (spawn_platforms_over_time, despawn_platforms)
                .run_if(in_state(GameState::Playing)))
            .add_systems(FixedUpdate, move_platforms
                .run_if(in_state(GameState::Playing)));
    }
}

const PLATFORM_SPEED: f32 = 2000.0;

#[derive(Resource)]
struct PlatformSpawner {
    spawn_timer: Timer,
    last_spawned_x: i32,
}

impl Default for PlatformSpawner {
    fn default() -> Self {
        Self {
            spawn_timer: Timer::from_seconds(4., TimerMode::Repeating),
            last_spawned_x: 0,
        }
    }
}

#[derive(Component)]
struct Platform;

fn spawn_first_platform(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.init_resource::<PlatformSpawner>();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/platform.png"),
            transform: Transform::from_xyz(0.0, 275.0, 0.0),
            ..default()
        },
        Platform,
        RigidBody::Kinematic,
        Collider::rectangle(190., 24.),
        GravityScale(0.0),
        StateScoped(GameState::Playing)
    ));
}

fn spawn_platforms_over_time(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawner: ResMut<PlatformSpawner>,
    time: Res<Time>,
) {
    spawner.spawn_timer.tick(time.delta());

    let last_x = spawner.last_spawned_x;

    if spawner.spawn_timer.just_finished() {
        let mut rng = rand::thread_rng();

        let range: Vec<i32> = (-300..300).filter(|x| *x < (&last_x - 50) || *x > (&last_x + 50)).collect();
        let random_x = range[rng.gen_range(0..range.len())];

        spawner.last_spawned_x = random_x;

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("textures/platform.png"),
                transform: Transform::from_xyz(random_x as f32, 325.0, 0.0),
                ..default()
            },
            Platform,
            RigidBody::Kinematic,
            Collider::rectangle(190., 22.),
            GravityScale(0.0),
            StateScoped(GameState::Playing),
        ));
    }
}

fn move_platforms(
    mut q_platform: Query<&mut LinearVelocity, With<Platform>>,
    time: Res<Time>,
) { 
    for mut linvel in q_platform.iter_mut() {
        linvel.0 = Vec2::NEG_Y * PLATFORM_SPEED * time.delta_seconds();
    }
}

fn despawn_platforms(
    mut commands: Commands,
    q_platform: Query<(Entity, &Transform), With<Platform>>,
) {
    for (platform_entity, platform_transform) in q_platform.iter() {
        if platform_transform.translation.y <= -325.0 {
            commands.entity(platform_entity).despawn();
        }
    }
}