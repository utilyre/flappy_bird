use crate::SCALE;
use bevy::prelude::*;

const GRAVITY: f32 = 9.8;
const ACCELERATION_COEFFICIENT: f32 = 25.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(apply_acceleration)
            .add_system(keyboard_input);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Acceleration {
    magnitude: Vec3,
    velocity: Vec3,
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            // TODO: cycle through bird_01.png and bird_02.png for animation
            texture: asset_server.load("bird_01.png"),
            transform: Transform::from_xyz(0.0, 0.0, 1.0).with_scale(Vec3::new(SCALE, SCALE, 1.0)),
            ..default()
        })
        .insert(Player)
        .insert(Acceleration {
            magnitude: Vec3::new(0.0, ACCELERATION_COEFFICIENT * SCALE * -GRAVITY, 0.0),
            velocity: Vec3::ZERO,
        });
}

fn apply_acceleration(
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

fn keyboard_input(
    mut player: Query<&mut Acceleration, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let Ok(mut acceleration) = player.get_single_mut() else {
        return;
    };

    if keyboard.just_pressed(KeyCode::Space) {
        acceleration.velocity = Vec3::new(0.0, 500.0, 0.0);
    }
}
