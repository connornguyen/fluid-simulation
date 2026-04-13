use bevy::{prelude::*, window::PrimaryWindow};

const PARTICLE_RADIUS: f32 = 5.0;
const SPAWN_INTERVAL: f32 = 0.05; // spawn a particle every 50ms

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_particles)
        .insert_resource(MouseState::default())
        .run();
}

#[derive(Resource, Default)]
struct MouseState {
    timer: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2d);

    let window = window.single().unwrap();
    let radius = window.width() / 5.0; // diameter = 2/5 of screen width

    commands.spawn((
        Mesh2d(meshes.add(Annulus::new(radius - 5.0, radius))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut mouse_state: ResMut<MouseState>,
    time: Res<Time>,
) {
    let window = window.single().unwrap();
    let (camera, camera_transform) = camera.single().unwrap();

    if mouse_button.pressed(MouseButton::Left) {
        mouse_state.timer += time.delta_secs();

        if mouse_state.timer >= SPAWN_INTERVAL {
            mouse_state.timer = 0.0;

            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                    commands.spawn((
                        Mesh2d(meshes.add(Circle::new(PARTICLE_RADIUS))),
                        MeshMaterial2d(materials.add(Color::WHITE)),
                        Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
                    ));
                }
            }
        }
    } else {
        mouse_state.timer = 0.0;
    }
}
