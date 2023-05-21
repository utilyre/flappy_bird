use bevy::prelude::*;

pub struct MovablePlugin;

impl Plugin for MovablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_movement);
    }
}

#[derive(Component)]
pub struct Movable {
    acceleration: Vec3,
    velocity: Vec3,
}

impl Movable {
    pub fn builder() -> MovableBuilder {
        MovableBuilder::new()
    }

    pub fn set_velocity(&mut self, value: Vec3) {
        self.velocity = value;
    }
}

#[derive(Default)]
pub struct MovableBuilder {
    acceleration: Vec3,
    velocity: Vec3,
}

impl MovableBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn acceleration(mut self, acceleration: Vec3) -> Self {
        self.acceleration = acceleration;
        self
    }

    pub fn velocity(mut self, velocity: Vec3) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn build(self) -> Movable {
        Movable {
            acceleration: self.acceleration,
            velocity: self.velocity,
        }
    }
}

fn apply_movement(mut movables: Query<(&mut Movable, &mut Transform)>, time: Res<Time>) {
    for (mut movable, mut transform) in &mut movables {
        let a = movable.acceleration;
        let dt = time.delta_seconds();

        // Δx = ½aΔt² + v₀t
        transform.translation += 0.5 * a * dt.powi(2) + movable.velocity * dt;

        // Δv = at
        movable.velocity += a * dt;
    }
}
