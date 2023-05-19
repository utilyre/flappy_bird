use crate::{RESOLUTION, SCALE};
use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

pub const PIPE_SPRITE_SIZE: (f32, f32) = (16.0, 16.0);
const COLUMN_SIZE: u32 = (RESOLUTION.1 / (SCALE * PIPE_SPRITE_SIZE.1)) as u32;
const EMPTY_COLUMNS: u32 = 3;

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
        .add_system(despawner)
        .add_system(movement);
    }
}

#[derive(Component)]
pub struct Pipe;

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

    let empty_idx = rand::thread_rng().gen_range(1..COLUMN_SIZE - EMPTY_COLUMNS);
    for i in 0..COLUMN_SIZE {
        if i >= empty_idx && i < empty_idx + EMPTY_COLUMNS {
            continue;
        }

        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("pipe.png"),
                transform: Transform::from_xyz(
                    (RESOLUTION.0 + SCALE * PIPE_SPRITE_SIZE.0) / 2.0,
                    -RESOLUTION.1 / 2.0 + SCALE * ((i as f32 + 0.5) * PIPE_SPRITE_SIZE.1),
                    0.0,
                )
                .with_scale(Vec3::new(SCALE, SCALE, 1.0)),
                ..default()
            })
            .insert(Pipe);
    }
}

fn despawner(mut commands: Commands, pipes: Query<(Entity, &GlobalTransform), With<Pipe>>) {
    for (entity, transform) in &pipes {
        if transform.translation().x < (-RESOLUTION.0 - SCALE * PIPE_SPRITE_SIZE.0) / 2.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn movement(mut pipes: Query<&mut Transform, With<Pipe>>, time: Res<Time>) {
    for mut transform in &mut pipes {
        transform.translation.x -= SPEED * time.delta_seconds();
    }
}
