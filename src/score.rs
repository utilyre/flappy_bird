use crate::{pipe::Pipe, player::Player};
use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ScoreCounted>()
            .register_type::<ScoreText>()
            .register_type::<Score>()
            .init_resource::<Score>()
            .add_startup_system(spawn_text)
            .add_system(count)
            .add_system(update);
    }
}

#[derive(Default, Reflect, Component)]
#[reflect(Component)]
struct ScoreCounted;

#[derive(Default, Reflect, Component)]
#[reflect(Component)]
struct ScoreText;

#[derive(Default, Reflect, Resource)]
#[reflect(Resource)]
struct Score(u32);

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(
            TextBundle::from_section(
                "SCORE: 0",
                TextStyle {
                    font: asset_server.load("fonts/MinecraftRegular.otf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(16.0),
                    ..default()
                },
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    ..default()
                },
                ..default()
            }),
        )
        .insert(Name::new("Score Text"))
        .insert(ScoreText);
}

fn count(
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
    }
}

fn update(mut text: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    let Ok(mut text) = text.get_single_mut() else {
        return;
    };

    text.sections[0].value = format!("SCORE: {:03}", score.0);
}
