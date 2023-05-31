use crate::{
    movable::Movable,
    pipe::{PipeBlock, PIPE_SPRITE_SIZE},
    states::GameState,
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
            .add_system(insert_movable.in_schedule(OnEnter(GameState::Playing)))
            .add_system(animate_sprite)
            .add_system(check_deadzone)
            .add_system(collide_with_pipe)
            .add_system(start_game)
            .add_system(handle_input);
    }
}

#[derive(Default, Reflect, Component)]
#[reflect(Component)]
pub struct Player;

#[derive(Default, Reflect, Component)]
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
        });
}

fn insert_movable(mut commands: Commands, player: Query<Entity, (With<Player>, Without<Movable>)>) {
    let Ok(entity) = player.get_single() else {
        return;
    };

    commands.entity(entity).insert(Movable {
        acceleration: SCALE * GRAVITY * Vec3::Y,
        velocity: JUMP_FORCE * Vec3::Y,
    });
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

fn check_deadzone(mut commands: Commands, player: Query<(Entity, &GlobalTransform), With<Player>>) {
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

fn collide_with_pipe(
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

fn start_game(keyboard: Res<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        game_state.set(GameState::Playing);
    }
}

fn handle_input(mut player: Query<&mut Movable, With<Player>>, keyboard: Res<Input<KeyCode>>) {
    let Ok(mut movable) = player.get_single_mut() else {
        return;
    };

    if keyboard.just_pressed(KeyCode::Space) {
        movable.velocity = JUMP_FORCE * Vec3::Y;
    }
}
