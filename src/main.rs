use std::time::Duration;

use bevy::prelude::*;

// Window
const BACKGROUND: (u8, u8, u8) = (30, 35, 37);
const RESOLUTION: (f32, f32) = (1280.0, 720.0);
const SCALE: f32 = 5.0;

// Gameplay
const SPAWN_INTERVAL: u64 = 2000;
const SPEED: f32 = 200.0;

// Pipe
const PIPE_SIZE: (f32, f32) = (16.0, 16.0);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        resolution: RESOLUTION.into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::rgb_u8(
            BACKGROUND.0,
            BACKGROUND.1,
            BACKGROUND.2,
        )))
        .add_startup_system(setup_world)
        .add_system(pipe_spawner)
        .add_system(pipe_movement)
        .run();
}

#[derive(Component)]
struct Pipe;

#[derive(Resource)]
struct SpawnTimer(Timer);

fn setup_world(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(SpawnTimer(Timer::new(
        Duration::from_millis(SPAWN_INTERVAL),
        TimerMode::Repeating,
    )));
}

fn pipe_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
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

fn pipe_movement(mut pipes: Query<&mut Transform, With<Pipe>>, time: Res<Time>) {
    for mut transform in &mut pipes {
        transform.translation.x -= SPEED * time.delta_seconds();
    }
}
