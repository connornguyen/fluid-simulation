use bevy::{prelude::*, window::PrimaryWindow};

const PARTICLE_RADIUS: f32 = 5.0;
const GROW_SPEED: f32 = 2.0;
const MOVE_THRESHOLD: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, pour_milk)
        .insert_resource(PourState::default())
        .run();
}

#[derive(Resource, Default)]
struct PourState {
    last_pos: Option<Vec2>,
    active_circle: Option<Entity>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2d);

    let window = window.single().unwrap();
    let radius = window.width() / 5.0;

    commands.spawn((
        Mesh2d(meshes.add(Annulus::new(radius - 5.0, radius))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn pour_milk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut pour: ResMut<PourState>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
) {
    let window = window.single().unwrap();
    let (camera, camera_transform) = camera.single().unwrap();

    if !mouse_button.pressed(MouseButton::Left) {
        // Released — leave the circle on screen, reset state
        pour.active_circle = None;
        pour.last_pos = None;
        return;
    }

    let Some(cursor_pos) = window.cursor_position() else { return };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return };

    // Spawn circle on first press
    if pour.active_circle.is_none() {
        let entity = commands.spawn((
            Mesh2d(meshes.add(Circle::new(PARTICLE_RADIUS))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
        )).id();
        pour.active_circle = Some(entity);
        pour.last_pos = Some(world_pos);
        return;
    }

    let moving = pour.last_pos
        .map(|last| world_pos.distance(last) >= MOVE_THRESHOLD)
        .unwrap_or(false);

    if moving {
        // Deposit current circle where it is, spawn fresh small one at cursor
        let new_entity = commands.spawn((
            Mesh2d(meshes.add(Circle::new(PARTICLE_RADIUS))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
        )).id();
        pour.active_circle = Some(new_entity);
    } else if let Some(entity) = pour.active_circle {
        // Hold still — grow the circle
        if let Ok(mut transform) = transforms.get_mut(entity) {
            transform.scale += Vec3::splat(GROW_SPEED * time.delta_secs());
        }
    }

    pour.last_pos = Some(world_pos);
}
