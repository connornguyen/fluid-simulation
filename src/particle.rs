use bevy::{prelude::*, window::PrimaryWindow};
use crate::cup::CupInnerRadius;

const PARTICLE_RADIUS: f32 = 5.0;
const GROW_SPEED: f32 = 3.0;
const SHRINK_SPEED: f32 = 2.0;
const MOVE_THRESHOLD: f32 = 0.005;
const MIN_SCALE: f32 = 1.0;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PourState::default())
            .insert_resource(PourVolume { current: 0.0, max: 0.0 })
            .add_systems(Startup, spawn_ui)
            .add_systems(Update, (pour_milk, clear_particles, update_volume_text));
    }
}

#[derive(Component)]
pub struct Particle;

#[derive(Component)]
struct VolumeText;

#[derive(Resource)]
pub struct PourState {
    pub last_pos: Option<Vec2>,
    pub active: Option<Entity>,
    pub scale: f32,
}

impl Default for PourState {
    fn default() -> Self {
        Self { last_pos: None, active: None, scale: MIN_SCALE }
    }
}

#[derive(Resource)]
struct PourVolume {
    current: f32,
    max: f32,
}

fn spawn_ui(mut commands: Commands) {
    // Volume % above the cup (centered)
    commands.spawn((
        VolumeText,
        Text::new("0%"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(30.0),
            left: Val::Px(30.0),
            ..default()
        },
    ));

    // Hint text bottom left
    commands.spawn((
        Text::new("Press R to clear"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn update_volume_text(
    volume: Res<PourVolume>,
    mut text: Query<&mut Text, With<VolumeText>>,
) {
    if volume.max == 0.0 { return; }
    let pct = (volume.current / volume.max * 100.0).min(100.0) as u32;
    if let Ok(mut t) = text.single_mut() {
        **t = format!("{}%", pct);
    }
}

fn clear_particles(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    particles: Query<Entity, With<Particle>>,
    mut pour: ResMut<PourState>,
    mut volume: ResMut<PourVolume>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        for entity in &particles {
            commands.entity(entity).despawn();
        }
        *pour = PourState::default();
        volume.current = 0.0;
    }
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
    cup: Res<CupInnerRadius>,
    mut volume: ResMut<PourVolume>,
    time: Res<Time>,
) {
    // Set max volume lazily on first frame
    if volume.max == 0.0 {
        volume.max = (cup.0 / PARTICLE_RADIUS).powi(2);
    }

    if !mouse_button.pressed(MouseButton::Left) {
        *pour = PourState::default();
        return;
    }

    // Stop pouring when cup is full
    if volume.current >= volume.max {
        return;
    }

    let window = window.single().unwrap();
    let (camera, camera_transform) = camera.single().unwrap();
    let Some(cursor_pos) = window.cursor_position() else { return };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return };

    if world_pos.length() > cup.0 { return; }

    let max_scale = (cup.0 - world_pos.length()) / PARTICLE_RADIUS;

    let moving = pour.last_pos
        .map(|last| world_pos.distance(last) >= MOVE_THRESHOLD)
        .unwrap_or(false);

    if moving {
        pour.scale = (pour.scale - SHRINK_SPEED * time.delta_secs()).max(MIN_SCALE);
    } else {
        pour.scale = (pour.scale + GROW_SPEED * time.delta_secs()).min(max_scale);
    }

    if pour.active.is_none() || moving {
        volume.current += pour.scale * pour.scale;
        pour.active = Some(spawn_particle(
            &mut commands, &mut meshes, &mut materials,
            world_pos, pour.scale,
        ));
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
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.95, 0.8))),
        Transform::from_xyz(pos.x, pos.y, 0.5).with_scale(Vec3::splat(scale)),
    )).id()
}
