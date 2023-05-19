use crate::{
    pipe::{Pipe, PIPE_SPRITE_SIZE},
    RESOLUTION, SCALE,
};
use bevy::{prelude::*, sprite::collide_aabb::collide};

const PLAYER_SPRITE_SIZE: (f32, f32) = (16.0, 16.0);

const GRAVITY: f32 = -300.0;
const JUMP_FORCE: f32 = 500.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(dead_zone)
            .add_system(pipe_collision)
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
            magnitude: Vec3::new(0.0, SCALE * GRAVITY, 0.0),
            velocity: Vec3::ZERO,
        });
}

fn dead_zone(mut commands: Commands, player: Query<(Entity, &GlobalTransform), With<Player>>) {
    let Ok((entity, transform)) = player.get_single() else {
        return;
    };

    let Vec3 { y, .. } = transform.translation();
    if y <= (-RESOLUTION.1 + SCALE * PLAYER_SPRITE_SIZE.1) / 2.0 {
        // TODO: pause the game and show "You Lost!" UI
        commands.entity(entity).despawn_recursive();
    }
}

fn pipe_collision(
    mut commands: Commands,
    player: Query<(Entity, &GlobalTransform), With<Player>>,
    pipes: Query<&GlobalTransform, With<Pipe>>,
) {
    let Ok((player_entity, player_transform)) = player.get_single() else {
        return;
    };

    for pipe_transform in &pipes {
        let collision = collide(
            player_transform.translation(),
            SCALE * Vec2::from(PLAYER_SPRITE_SIZE),
            pipe_transform.translation(),
            SCALE * Vec2::from(PIPE_SPRITE_SIZE),
        );

        if collision.is_some() {
            // TODO: pause the game and show "You Lost!" UI
            commands.entity(player_entity).despawn_recursive();
            break;
        }
    }
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
        acceleration.velocity = Vec3::new(0.0, JUMP_FORCE, 0.0);
    }
}
