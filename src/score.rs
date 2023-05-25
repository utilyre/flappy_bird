use crate::{pipe::Pipe, player::Player};
use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0)).add_system(counter);
    }
}

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
struct ScoreCounted;

#[derive(Default, Resource, Reflect)]
#[reflect(Resource)]
struct Score(u32);

fn counter(
    mut commands: Commands,
    player: Query<&GlobalTransform, With<Player>>,
    pipes: Query<(Entity, &GlobalTransform, Option<&ScoreCounted>), With<Pipe>>,
    mut score: ResMut<Score>,
) {
    let Ok(player_transform) = player.get_single() else {
        return;
    };

    for (pipe_entity, pipe_transform, pipe_score_counted) in &pipes {
        if pipe_score_counted.is_some() {
            continue;
        }
        if pipe_transform.translation().x > player_transform.translation().x {
            continue;
        }

        score.0 += 1;
        commands.entity(pipe_entity).insert(ScoreCounted);

        println!("Score = {}", score.0);
    }
}
