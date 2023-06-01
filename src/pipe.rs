use crate::{movable::Movable, states::GameState, RESOLUTION, SCALE};
use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

pub const PIPE_SPRITE_SIZE: Vec2 = Vec2::new(16.0, 16.0);
const COLUMN_SIZE: u32 = (RESOLUTION.y / (SCALE * PIPE_SPRITE_SIZE.y)) as u32;
const EMPTY_COLUMNS: u32 = 3;

const SPAWN_INTERVAL: u64 = 2000;
const SPEED: f32 = 200.0;

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Pipe>()
            .register_type::<PipeBlock>()
            .register_type::<SpawnTimer>()
            .add_system(init_timer.in_schedule(OnEnter(GameState::Playing)))
            .add_system(spawn.in_set(OnUpdate(GameState::Playing)))
            .add_system(remove_movable.in_schedule(OnExit(GameState::Playing)))
            .add_system(despawn.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Default, Reflect, Component)]
#[reflect(Component)]
pub struct Pipe;

#[derive(Default, Reflect, Component)]
#[reflect(Component)]
pub struct PipeBlock;

#[derive(Reflect, Resource)]
#[reflect(Resource)]
struct SpawnTimer(Timer);

impl Default for SpawnTimer {
    fn default() -> Self {
        let mut timer = Timer::new(Duration::from_millis(SPAWN_INTERVAL), TimerMode::Repeating);
        timer.tick(timer.remaining());

        Self(timer)
    }
}

fn init_timer(mut commands: Commands) {
    commands.init_resource::<SpawnTimer>();
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    if spawn_timer.0.just_finished() {
        commands
            .spawn(SpriteBundle {
                // FIXME: a small default sprite appears in the center of the entity
                transform: Transform::from_xyz(
                    0.5 * (RESOLUTION.x + SCALE * PIPE_SPRITE_SIZE.x),
                    0.0,
                    0.0,
                ),
                ..default()
            })
            .insert(Name::new("Pipe"))
            .insert(Pipe)
            .insert(Movable {
                velocity: SPEED * Vec3::NEG_X,
                ..default()
            })
            .with_children(|builder| {
                let empty_idx = rand::thread_rng().gen_range(1..COLUMN_SIZE - EMPTY_COLUMNS);
                for i in 0..COLUMN_SIZE {
                    if i >= empty_idx && i < empty_idx + EMPTY_COLUMNS {
                        continue;
                    }

                    builder
                        .spawn(SpriteBundle {
                            texture: asset_server.load("sprites/pipe.png"),
                            transform: Transform::from_xyz(
                                0.0,
                                -0.5 * RESOLUTION.y
                                    + SCALE * ((i as f32 + 0.5) * PIPE_SPRITE_SIZE.y),
                                0.0,
                            )
                            .with_scale(Vec3::new(SCALE, SCALE, 1.0)),
                            ..default()
                        })
                        .insert(Name::new("Pipe Block"))
                        .insert(PipeBlock);
                }
            });
    }

    spawn_timer.0.tick(time.delta());
}

fn remove_movable(mut commands: Commands, pipes: Query<Entity, With<Pipe>>) {
    for entity in &pipes {
        commands.entity(entity).remove::<Movable>();
    }
}

fn despawn(mut commands: Commands, pipes: Query<(Entity, &GlobalTransform), With<Pipe>>) {
    for (entity, transform) in &pipes {
        if transform.translation().x < -0.5 * (RESOLUTION.x + SCALE * PIPE_SPRITE_SIZE.x) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
