use crate::pipe::PipePlugin;
use bevy::prelude::*;
use movable::MovablePlugin;
use player::PlayerPlugin;

mod movable;
mod pipe;
mod player;

const BACKGROUND: (u8, u8, u8) = (30, 35, 37);
const RESOLUTION: Vec2 = Vec2::new(1280.0, 720.0);
const SCALE: f32 = 4.5;

// TODO: Add enemies
// TODO: Collective coins

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
        .add_startup_system(spawn_camera)
        // Plugins
        .add_plugin(MovablePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PipePlugin)
        //
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
