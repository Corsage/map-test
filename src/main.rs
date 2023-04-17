use std::collections::HashMap;

use bevy::{asset::LoadState, prelude::*};
use bevy_common_assets::json::JsonAssetPlugin;
use vectors::Vector2Int;

pub mod vectors;

const TILE_SIZE: f32 = 16.;
const TILE_Z: f32 = 0.;

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"] // <-- keep me unique
struct Scene {
    layers: Vec<Vec<usize>>, // Corresponds to width * height.
}

#[derive(Default, Resource)]
struct AssetList(pub Vec<HandleUntyped>);

#[derive(Resource)]
pub struct GraphicsAssets {
    pub sprite_texture: Handle<TextureAtlas>,
}

#[derive(Default, Resource)]
pub struct CurrentBoard {
    pub tiles: HashMap<Vector2Int, Entity>,
}

#[derive(Resource)]
struct SceneHandle(Handle<Scene>);

#[derive(Component)]
struct Position {
    pub v: Vector2Int,
}

#[derive(Component)]
struct Tile {
    pub i: usize,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .init_resource::<AssetList>()
        .init_resource::<CurrentBoard>()
        .add_plugin(JsonAssetPlugin::<Scene>::new(&["json"]))
        // Load assets.
        .add_startup_system(load_assets)
        // Load camera.
        .add_startup_system(spawn_camera)
        .add_system(check_asset_loading.in_set(OnUpdate(AppState::Loading)))
        // Load scene once assets are done loading.
        .add_system(load_scene.in_schedule(OnEnter(AppState::Game)))
        .add_system(spawn_scene_renderer)
        .run();
}

fn load_assets(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut atlas: ResMut<Assets<TextureAtlas>>,
    mut assets: ResMut<AssetList>,
) {
    let scene = server.load("data.json");
    let texture = server.load("tilemap_packed.png");

    assets.0.push(scene.clone_untyped());
    assets.0.push(texture.clone_untyped());

    let map = TextureAtlas::from_grid(texture, Vec2::splat(16.), 12, 11, None, None);
    let handle = atlas.add(map);

    // Add the graphic asset.
    commands.insert_resource(GraphicsAssets {
        sprite_texture: handle,
    });

    // Add the data asset.
    commands.insert_resource(SceneHandle(scene));
}

fn check_asset_loading(
    server: Res<AssetServer>,
    assets: Res<AssetList>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    match server.get_group_load_state(assets.0.iter().map(|a| a.id())) {
        LoadState::Loaded => {
            info!("Loaded {} assets.", assets.0.len());
            next_state.set(AppState::Game);
        }
        LoadState::Failed => {
            error!("Failed to load assets.");
        }
        _ => {}
    };
}

fn get_world_position(position: &Position, z: f32) -> Vec3 {
    Vec3::new(
        TILE_SIZE * position.v.x as f32,
        TILE_SIZE * position.v.y as f32,
        z,
    )
}

fn load_scene(
    mut commands: Commands,
    scene: Res<SceneHandle>,
    mut scenes: ResMut<Assets<Scene>>,
    mut current: ResMut<CurrentBoard>,
) {
    if let Some(scene) = scenes.remove(scene.0.id()) {
        info!("{:?}", scene);
        for (pos, i) in scene.layers[0].iter().enumerate() {
            info!("Got tile index: {}", i);

            // Calculate y from width.
            // Note: (0, 0) is actually centered.
            // In order to center the map which is 32 x 32
            // we have to start at (-16, -16).
            let index: i32 = (*i as i32) - 1;
            if index >= 0 {
                let v = Vector2Int::new(((pos as i32) % 32) - 16, (-(pos as i32) / 32) + 16);
                let tile = commands
                    .spawn((Position { v }, Tile { i: index as usize }))
                    .id(); // Offset by 1.
                current.tiles.insert(v, tile);
            }
        }
    }
}

fn spawn_scene_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Tile, &Position), Added<Tile>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, tile, position) in query.iter() {
        let mut sprite = TextureAtlasSprite::new(tile.i);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

        let v = get_world_position(&position, TILE_Z);

        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture_atlas: assets.sprite_texture.clone(),
            transform: Transform::from_translation(v),
            ..Default::default()
        });
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation = Vec3::new(
        4. * TILE_SIZE,
        4. * TILE_SIZE,
        camera.transform.translation.z,
    );
    commands.spawn(camera);
}
