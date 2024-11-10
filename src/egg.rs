use bevy::prelude::*;
use avian2d::prelude::*;

use crate::GameState;

pub struct EggPlugin;

impl Plugin for EggPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), spawn_egg)
            .add_systems(Update, (jump, draw_path, trigger_game_over)
                .run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
struct Egg;

fn spawn_egg(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.init_resource::<CursorPosition>();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/egg.png"),
            transform: Transform::from_xyz(0.0, 305.,0.),
            ..default()
        },
        Egg,
        RigidBody::Dynamic,
        Collider::circle(16.),
        LockedAxes::ROTATION_LOCKED,
        Friction::new(1.0),
        Restitution::new(0.0),
        StateScoped(GameState::Playing),
    ));
}

#[derive(Resource, Default)]
struct CursorPosition {
    start: Vec2,
    current: Vec2,
}

fn jump(
    mut q_egg: Query<&mut ExternalImpulse, With<Egg>>,

    q_windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut cursor_position: ResMut<CursorPosition>,

    mouse: Res<ButtonInput<MouseButton>>,
) {
    let Ok(window) = q_windows.get_single() else { return };
    let Ok((camera, camera_transform)) = q_camera.get_single() else { return };
    let Ok(mut impulse) = q_egg.get_single_mut() else { return };

    if mouse.just_pressed(MouseButton::Left) {
        let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        else { return };

        cursor_position.start = world_position;
    }

    if mouse.pressed(MouseButton::Left) {
        let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        else { return };

        cursor_position.current = world_position;
    }

    if mouse.just_released(MouseButton::Left) {
        let difference = cursor_position.start - cursor_position.current;

        impulse.apply_impulse(difference * 2_000.).with_persistence(false);
    }
}

#[derive(Component)]
struct PathPoint(usize);

fn draw_path(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    q_egg: Query<Entity, With<Egg>>,
    mut q_path_point: Query<(&mut Transform, &PathPoint, Entity)>,

    cursor_position: Res<CursorPosition>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let Ok(egg_entity) = q_egg.get_single() else { return };

    if mouse.just_pressed(MouseButton::Left) {
        for i in 1..=10 {
            commands.entity(egg_entity).with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        texture: asset_server.load("textures/path_point.png"),
                        transform: Transform::from_scale(Vec3::splat(0.5)),
                        ..default()
                    },
                    PathPoint(i),
                ));
            });
        }
    }

    if mouse.pressed(MouseButton::Left) {
        for (mut point_transform, path_point, _) in q_path_point.iter_mut() {
            point_transform.translation = ((cursor_position.start - cursor_position.current) * (path_point.0 as f32/10.0)).extend(0.0);
        } 
    }

    if mouse.just_released(MouseButton::Left) {
        for (_, _, entity) in q_path_point.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn trigger_game_over(
    q_egg: Query<&Transform, With<Egg>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(egg_transform) = q_egg.get_single() else { return };

    if egg_transform.translation.y <= -325.0 {
        next_state.set(GameState::GameOver);
    }
}