use bevy::{prelude::*, window::PrimaryWindow};
use crate::cup::CupInnerRadius;

const PARTICLE_RADIUS: f32 = 5.0;
const GROW_SPEED: f32 = 3.0;
const SHRINK_SPEED: f32 = 2.0;
const MOVE_THRESHOLD: f32 = 0.01;
const MIN_SCALE: f32 = 1.0;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PourState::default())
            .add_systems(Update, pour_milk);
    }
}

#[derive(Component)]
pub struct Particle;

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
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(pos.x, pos.y, 0.0).with_scale(Vec3::splat(scale)),
    )).id()
}
