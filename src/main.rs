use crate::pipe::PipePlugin;
use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use movable::MovablePlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use states::game::GameStatePlugin;

mod movable;
mod pipe;
mod player;
mod score;
mod states;

const BACKGROUND: Color = Color::rgb(0.12, 0.14, 0.15);
const RESOLUTION: Vec2 = Vec2::new(1280.0, 720.0);
const SCALE: f32 = 4.5;

// TODO: Parallax background
// TODO: Add enemies
// TODO: Collective coins

fn main() {
    App::new()
        // Plugins
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
        .add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::LShift)),
        )
        // END
        .insert_resource(ClearColor(BACKGROUND))
        .add_startup_system(spawn_camera)
        // Local Plugins
        .add_plugin(GameStatePlugin)
        .add_plugin(MovablePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PipePlugin)
        .add_plugin(ScorePlugin)
        // END
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(Name::new("Camera"));
}
