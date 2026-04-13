use bevy::{prelude::*, window::PrimaryWindow};

const PARTICLE_RADIUS: f32 = 5.0;
const GROW_SPEED: f32 = 2.0;
const SHRINK_SPEED: f32 = 3.0;
const MOVE_THRESHOLD: f32 = 0.1;
const MIN_SCALE: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, pour_milk)
        .insert_resource(PourState::default())
        .run();
}

#[derive(Resource)]
struct PourState {
    last_pos: Option<Vec2>,
    active_circle: Option<Entity>,
    current_scale: f32,
}

impl Default for PourState {
    fn default() -> Self {
        Self {
            last_pos: None,
            active_circle: None,
            current_scale: MIN_SCALE,
        }
    }
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
        // Released — leave circle on screen, reset state but keep scale
        pour.active_circle = None;
        pour.last_pos = None;
        pour.current_scale = MIN_SCALE;
        return;
    }

    let Some(cursor_pos) = window.cursor_position() else { return };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return };

    // Spawn circle on first press
    if pour.active_circle.is_none() {
        let entity = commands.spawn((
            Mesh2d(meshes.add(Circle::new(PARTICLE_RADIUS))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz(world_pos.x, world_pos.y, 0.0)
                .with_scale(Vec3::splat(pour.current_scale)),
        )).id();
        pour.active_circle = Some(entity);
        pour.last_pos = Some(world_pos);
        return;
    }

    let moving = pour.last_pos
        .map(|last| world_pos.distance(last) >= MOVE_THRESHOLD)
        .unwrap_or(false);

    if moving {
        // Shrink scale, spawn new circle at cursor inheriting current scale
        pour.current_scale = (pour.current_scale - SHRINK_SPEED * time.delta_secs()).max(MIN_SCALE);

        let new_entity = commands.spawn((
            Mesh2d(meshes.add(Circle::new(PARTICLE_RADIUS))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz(world_pos.x, world_pos.y, 0.0)
                .with_scale(Vec3::splat(pour.current_scale)),
        )).id();
        pour.active_circle = Some(new_entity);
    } else if let Some(entity) = pour.active_circle {
        // Hold still — grow the circle
        pour.current_scale += GROW_SPEED * time.delta_secs();

        if let Ok(mut transform) = transforms.get_mut(entity) {
            transform.scale = Vec3::splat(pour.current_scale);
        }
    }

    pour.last_pos = Some(world_pos);
}
