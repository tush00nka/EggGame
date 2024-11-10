use bevy::prelude::*;
use avian2d::prelude::*;

pub struct EggPlugin;

impl Plugin for EggPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CursorPosition>()
            .add_systems(Startup, spawn_egg)
            .add_systems(Update, (jump, draw_path));
    }
}

#[derive(Component)]
struct Egg;

fn spawn_egg(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/egg.png"),
            transform: Transform::from_xyz(0.0, 300.,0.),
            ..default()
        },
        Egg,
        RigidBody::Dynamic,
        Collider::circle(16.),
        LockedAxes::ROTATION_LOCKED,
        Friction::new(1.0),
        Restitution::new(0.0),
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
        let difference = cursor_position.current - cursor_position.start;

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
        for i in 0..10 {
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
            point_transform.translation = ((cursor_position.current - cursor_position.start) * (path_point.0 as f32/10.0)).extend(0.0);
        } 
    }

    if mouse.just_released(MouseButton::Left) {
        for (_, _, entity) in q_path_point.iter() {
            commands.entity(entity).despawn();
        }
    }
}