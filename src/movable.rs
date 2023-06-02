use bevy::prelude::*;

pub struct MovablePlugin;

impl Plugin for MovablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Movable>().add_system(apply_movement);
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct Movable {
    pub acceleration: Vec3,
    pub velocity: Vec3,
}

impl Default for Movable {
    fn default() -> Self {
        Self {
            acceleration: Vec3::splat(1.0),
            velocity: Vec3::splat(0.0),
        }
    }
}

fn apply_movement(mut movables: Query<(&mut Movable, &mut Transform)>, time: Res<Time>) {
    for (mut movable, mut transform) in &mut movables {
        let a = movable.acceleration;
        let dt = time.delta_seconds();

        // Δx = ½aΔt² + v₀Δt
        transform.translation += 0.5 * a * dt.powi(2) + movable.velocity * dt;

        // Δv = aΔt
        movable.velocity += a * dt;
    }
}
