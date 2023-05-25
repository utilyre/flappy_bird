use crate::{
    movable::Movable,
    pipe::{PipeBlock, PIPE_SPRITE_SIZE},
    RESOLUTION, SCALE,
};
use bevy::{prelude::*, sprite::collide_aabb::collide};
use std::time::Duration;

const PLAYER_SPRITE_SIZE: Vec2 = Vec2::new(16.0, 16.0);
const HITBOX_RATIO: f32 = 0.75;
const ANIMATION_INTERVAL: u64 = 200;
const ANIMATION_FRAMES: &[usize] = &[0, 1];

const GRAVITY: f32 = -300.0;
const JUMP_FORCE: f32 = 500.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .register_type::<Animation>()
            .add_startup_system(spawn)
            .add_system(animate_sprite)
            .add_system(dead_zone)
            .add_system(pipe_collision)
            .add_system(keyboard_input);
    }
}

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
struct Animation {
    timer: Timer,
    index: usize,
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("sprites/bird.png"),
        PLAYER_SPRITE_SIZE,
        ANIMATION_FRAMES.len(),
        1,
        None,
        None,
    );

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(texture_atlas),
            sprite: TextureAtlasSprite::new(ANIMATION_FRAMES[0]),
            transform: Transform::from_xyz(0.0, 0.0, 1.0).with_scale(Vec3::new(SCALE, SCALE, 1.0)),
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(Player)
        .insert(Animation {
            timer: Timer::new(
                Duration::from_millis(ANIMATION_INTERVAL),
                TimerMode::Repeating,
            ),
            ..default()
        })
        .insert(
            Movable::builder()
                .acceleration(Vec3::new(0.0, SCALE * GRAVITY, 0.0))
                .build(),
        );
}

fn animate_sprite(
    mut animations: Query<(&mut TextureAtlasSprite, &mut Animation)>,
    time: Res<Time>,
) {
    for (mut sprite, mut animation) in &mut animations {
        animation.timer.tick(time.delta());
        if !animation.timer.just_finished() {
            continue;
        }

        animation.index = (animation.index + 1) % ANIMATION_FRAMES.len();
        sprite.index = ANIMATION_FRAMES[animation.index];
    }
}

fn dead_zone(mut commands: Commands, player: Query<(Entity, &GlobalTransform), With<Player>>) {
    let Ok((entity, transform)) = player.get_single() else {
        return;
    };

    let Vec3 { y, .. } = transform.translation();
    if y <= -0.5 * (RESOLUTION.y + SCALE * PLAYER_SPRITE_SIZE.y)
        || y >= 0.5 * (RESOLUTION.y + SCALE * PLAYER_SPRITE_SIZE.y)
    {
        // TODO: pause the game and show "You Lost!" UI
        commands.entity(entity).despawn_recursive();
    }
}

fn pipe_collision(
    mut commands: Commands,
    player: Query<(Entity, &GlobalTransform), With<Player>>,
    pipes: Query<&GlobalTransform, With<PipeBlock>>,
) {
    let Ok((player_entity, player_transform)) = player.get_single() else {
        return;
    };

    for pipe_transform in &pipes {
        let collision = collide(
            player_transform.translation(),
            HITBOX_RATIO * SCALE * PLAYER_SPRITE_SIZE,
            pipe_transform.translation(),
            SCALE * PIPE_SPRITE_SIZE,
        );

        if collision.is_some() {
            // TODO: pause the game and show "You Lost!" UI
            commands.entity(player_entity).despawn_recursive();
            break;
        }
    }
}

fn keyboard_input(mut player: Query<&mut Movable, With<Player>>, keyboard: Res<Input<KeyCode>>) {
    let Ok(mut movable) = player.get_single_mut() else {
        return;
    };

    if keyboard.just_pressed(KeyCode::Space) {
        movable.set_velocity(Vec3::new(0.0, JUMP_FORCE, 0.0));
    }
}
