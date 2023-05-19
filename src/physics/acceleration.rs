use bevy::prelude::*;

#[derive(Component)]
pub struct Acceleration {
    pub magnitude: Vec3,
    pub velocity: Vec3,
}

pub fn apply_acceleration(
    mut accelerations: Query<(&mut Acceleration, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut acceleration, mut transform) in &mut accelerations {
        // Δx = ½aΔt² + v₀t
        transform.translation += 0.5 * acceleration.magnitude * time.delta_seconds().powi(2)
            + acceleration.velocity * time.delta_seconds();

        // Δv = at
        let a = acceleration.magnitude;
        acceleration.velocity += a * time.delta_seconds();
    }
}
