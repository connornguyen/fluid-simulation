use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_line)
        .insert_resource(MouseState::default())
        .run();
}

#[derive(Resource, Default)]
struct MouseState {
    last_pos: Option<Vec2>,
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

fn draw_line(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut mouse_state: ResMut<MouseState>,
) {
    let window = window.single().unwrap();
    let (camera, camera_transform) = camera.single().unwrap();

    if mouse_button.pressed(MouseButton::Left) {
        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                if let Some(last_pos) = mouse_state.last_pos {
                    let delta = world_pos - last_pos;
                    let length = delta.length();
                    if length > 0.5 {
                        let mid = (last_pos + world_pos) / 2.0;
                        let angle = delta.y.atan2(delta.x);

                        commands.spawn((
                            Mesh2d(meshes.add(Circle::new(5.0))),
                            MeshMaterial2d(materials.add(Color::WHITE)),
                            Transform::from_xyz(mid.x, mid.y, 0.0)
                                .with_rotation(Quat::from_rotation_z(angle)),
                        ));
                    }
                }
                mouse_state.last_pos = Some(world_pos);
            }
        }
    } else {
        mouse_state.last_pos = None;
    }
}
