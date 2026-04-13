mod cup;
mod particle;
mod physics;

use bevy::prelude::*;
use cup::CupPlugin;
use particle::ParticlePlugin;
use physics::PhysicsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CupPlugin, ParticlePlugin, PhysicsPlugin))
        .run();
}
