use bevy::{prelude::*, window::PrimaryWindow};

// --- Cup ---
const CUP_RADIUS_RATIO: f32 = 5.0; // cup diameter = 2/5 of screen width
const CUP_THICKNESS: f32 = 5.0;

// --- Milk particles ---
const PARTICLE_RADIUS: f32 = 5.0;
const GROW_SPEED: f32 = 2.0;
const SHRINK_SPEED: f32 = 3.0;
const MOVE_THRESHOLD: f32 = 0.05;
const MIN_SCALE: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, pour_milk)
        .insert_resource(PourState::default())
        .run();
}

// Marks an entity as a milk particle (useful for future systems)
#[derive(Component)]
struct Particle;

#[derive(Resource)]
struct PourState {
    last_pos: Option<Vec2>,
    active: Option<Entity>,
    scale: f32,
}

impl Default for PourState {
    fn default() -> Self {
        Self { last_pos: None, active: None, scale: MIN_SCALE }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2d);

    let radius = window.single().unwrap().width() / CUP_RADIUS_RATIO;
    commands.spawn((
        Mesh2d(meshes.add(Annulus::new(radius - CUP_THICKNESS, radius))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::default(),
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
    if !mouse_button.pressed(MouseButton::Left) {
        *pour = PourState::default();
        return;
    }

    let window = window.single().unwrap();
    let (camera, camera_transform) = camera.single().unwrap();
    let Some(cursor_pos) = window.cursor_position() else { return };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return };

    let moving = pour.last_pos
        .map(|last| world_pos.distance(last) >= MOVE_THRESHOLD)
        .unwrap_or(false);

    // Update scale based on movement
    if moving {
        pour.scale = (pour.scale - SHRINK_SPEED * time.delta_secs()).max(MIN_SCALE);
    } else {
        pour.scale += GROW_SPEED * time.delta_secs();
    }

    // Spawn a new particle or update the existing one
    if pour.active.is_none() || moving {
        pour.active = Some(spawn_particle(&mut commands, &mut meshes, &mut materials, world_pos, pour.scale));
    } else if let Some(entity) = pour.active {
        if let Ok(mut transform) = transforms.get_mut(entity) {
            transform.scale = Vec3::splat(pour.scale);
        }
    }

    pour.last_pos = Some(world_pos);
}

fn spawn_particle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    pos: Vec2,
    scale: f32,
) -> Entity {
    commands.spawn((
        Particle,
        Mesh2d(meshes.add(Circle::new(PARTICLE_RADIUS))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(pos.x, pos.y, 0.0).with_scale(Vec3::splat(scale)),
    )).id()
}
