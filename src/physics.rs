use bevy::prelude::*;
use crate::cup::CupInnerRadius;
use crate::particle::Particle;

const PARTICLE_RADIUS: f32 = 5.0;
const DAMPING: f32 = 0.92;
const SEPARATION_STRENGTH: f32 = 150.0;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (apply_separation, apply_physics));
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

fn apply_separation(
    positions: Query<(Entity, &Transform), With<Particle>>,
    mut velocities: Query<(Entity, &Transform, &mut Velocity), With<Particle>>,
) {
    let particles: Vec<(Entity, Vec2, f32)> = positions
        .iter()
        .map(|(e, t)| (e, t.translation.truncate(), t.scale.x))
        .collect();

    for (entity, transform, mut velocity) in &mut velocities {
        let pos = transform.translation.truncate();
        let radius = PARTICLE_RADIUS * transform.scale.x;

        for (other_entity, other_pos, other_scale) in &particles {
            if *other_entity == entity { continue; }

            let other_radius = PARTICLE_RADIUS * other_scale;
            let min_dist = radius + other_radius;
            let delta = pos - other_pos;
            let dist = delta.length();

            if dist < min_dist && dist > 0.001 {
                let push = delta.normalize() * (min_dist - dist);
                velocity.0 += push * SEPARATION_STRENGTH;
            }
        }
    }
}

fn apply_physics(
    mut query: Query<(&mut Velocity, &mut Transform), With<Particle>>,
    cup: Res<CupInnerRadius>,
    time: Res<Time>,
) {
    for (mut velocity, mut transform) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();

        // Cup boundary — reflect off inner wall
        let pos = transform.translation.truncate();
        let radius = PARTICLE_RADIUS * transform.scale.x;
        let dist = pos.length();
        if dist + radius > cup.0 {
            let normal = pos.normalize();
            transform.translation = ((cup.0 - radius) * normal).extend(0.0);
            let dot = velocity.0.dot(normal);
            velocity.0 -= 2.0 * dot * normal;
            velocity.0 *= 0.3;
        }

        velocity.0 *= DAMPING;
    }
}
