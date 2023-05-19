use crate::{RESOLUTION, SCALE};
use bevy::prelude::*;
use std::time::Duration;

const PIPE_SIZE: (f32, f32) = (16.0, 16.0);

const SPAWN_INTERVAL: u64 = 2000;
const SPEED: f32 = 200.0;

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::new(
            Duration::from_millis(SPAWN_INTERVAL),
            TimerMode::Repeating,
        )))
        .add_system(spawner)
        .add_system(movement);
    }
}

#[derive(Component)]
struct Pipe;

#[derive(Resource)]
struct SpawnTimer(Timer);

fn spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    spawn_timer.0.tick(time.delta());
    if !spawn_timer.0.just_finished() {
        return;
    }

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("pipe.png"),
            transform: Transform::from_xyz(RESOLUTION.0 / 2.0 + SCALE * PIPE_SIZE.0, 0.0, 0.0)
                .with_scale(Vec3::new(SCALE, SCALE, 1.0)),
            ..default()
        })
        .insert(Pipe);
}

fn movement(mut pipes: Query<&mut Transform, With<Pipe>>, time: Res<Time>) {
    for mut transform in &mut pipes {
        transform.translation.x -= SPEED * time.delta_seconds();
    }
}
