use bevy::{prelude::*, window::PrimaryWindow};

const CUP_RADIUS_RATIO: f32 = 5.0;
const CUP_THICKNESS: f32 = 5.0;

pub struct CupPlugin;

impl Plugin for CupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CupInnerRadius(0.0))
            .add_systems(Startup, setup);
    }
}

#[derive(Resource)]
pub struct CupInnerRadius(pub f32);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut cup: ResMut<CupInnerRadius>,
) {
    commands.spawn(Camera2d);

    let radius = window.single().unwrap().width() / CUP_RADIUS_RATIO;
    cup.0 = radius - CUP_THICKNESS;

    // Cup rim
    commands.spawn((
        Mesh2d(meshes.add(Annulus::new(cup.0, radius))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));

    // Coffee fill (z = 0, behind milk particles at z = 0.5)
    let coffee_color = Color::srgb(0.35, 0.18, 0.07);
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(cup.0))),
        MeshMaterial2d(materials.add(coffee_color)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
