use bevy::prelude::*;

use crate::{
    get_world_position, vectors::Vector3Int, AppState, GraphicsAssets, Position, TILE_SIZE,
};

pub const POSITION_TOLERANCE: f32 = 0.1;
pub const PLAYER_SPEED: f32 = 10.;

#[derive(Component)]
pub struct Player;

const DIR_KEY_MAPPING: [(KeyCode, Vector3Int); 4] = [
    (KeyCode::W, Vector3Int::UP),
    (KeyCode::S, Vector3Int::DOWN),
    (KeyCode::A, Vector3Int::LEFT),
    (KeyCode::D, Vector3Int::RIGHT),
];

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(load_player.in_schedule(OnEnter(AppState::Game)))
            .add_system(spawn_player_renderer)
            .add_system(player_position)
            .add_system(update_player_position)
            .add_system(camera_follow_player);
    }
}

fn load_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Position {
            v: Vector3Int::new(0, 0, 5), // Temp z-index.
        },
    ));
}

fn spawn_player_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Player>>,
    assets: Res<GraphicsAssets>,
) {
    let Ok((entity, position)) = query.get_single() else { return };

    let mut sprite = TextureAtlasSprite::new(95); // Temporary value.
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    let v = get_world_position(position);
    commands.entity(entity).insert(SpriteSheetBundle {
        sprite,
        texture_atlas: assets.sprite_texture.clone(),
        transform: Transform::from_translation(v),
        ..Default::default()
    });
}

fn player_position(keys: ResMut<Input<KeyCode>>, mut query: Query<&mut Position, With<Player>>) {
    let Ok(mut position) = query.get_single_mut() else { return };

    for (key, dir) in DIR_KEY_MAPPING {
        if keys.just_pressed(key) {
            position.v += dir;
        }
    }
}

fn update_player_position(
    mut query: Query<(&Position, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let Ok((position, mut transform)) = query.get_single_mut() else { return };
    let target = get_world_position(position);
    let d = (target - transform.translation).length();
    if d > POSITION_TOLERANCE {
        transform.translation = transform
            .translation
            .lerp(target, PLAYER_SPEED * time.delta_seconds());
    } else {
        transform.translation = target;
    }
}

fn camera_follow_player(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let Ok(mut c) = camera.get_single_mut() else { return };
    let Ok(p) = player.get_single() else { return };

    c.translation = p.translation;
}
