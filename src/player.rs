use crate::SCALE;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
    }
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            // TODO: cycle through bird_01.png and bird_02.png for animation
            texture: asset_server.load("bird_01.png"),
            transform: Transform::from_xyz(0.0, 0.0, 1.0).with_scale(Vec3::new(SCALE, SCALE, 1.0)),
            ..default()
        })
        .insert(Player);
}
