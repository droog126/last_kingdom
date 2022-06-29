use bevy::utils::HashMap;

use crate::state::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

pub struct LoadingPlugin;

pub struct TextureAtlasCenter(pub HashMap<String, Handle<TextureAtlas>>);
impl FromWorld for TextureAtlasCenter {
    fn from_world(world: &mut World) -> Self {
        let mut assertCenter: HashMap<String, Handle<TextureAtlas>> = HashMap::new();
        TextureAtlasCenter(assertCenter)
    }
}

pub struct ImageCenter(pub HashMap<String, Handle<Image>>);
impl FromWorld for ImageCenter {
    fn from_world(world: &mut World) -> Self {
        let mut assertCenter: HashMap<String, Handle<Image>> = HashMap::new();
        ImageCenter(assertCenter)
    }
}

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .with_collection::<FontAssets>()
            .continue_to_state(GameState::Menu)
            .build(app);

        app.init_resource::<TextureAtlasCenter>().init_resource::<ImageCenter>().add_startup_system(startup);
    }
}

//这些都在Res里面
#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

fn startup(
    mut textureAtlasCenter: ResMut<TextureAtlasCenter>,
    mut imageCenter: ResMut<ImageCenter>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // snake
    let texture_handle = asset_server.load("sprite/snake_sheet.png");
    let sprite_atlas =
        TextureAtlas::from_grid_with_padding(texture_handle.clone(), Vec2::new(32.0, 32.0), 8, 5, Vec2::new(0.0, 0.0));
    let sprite_handle = texture_atlases.add(sprite_atlas);
    textureAtlasCenter.0.insert("snake".to_string(), sprite_handle);

    // player

    let texture_handle = asset_server.load("sprite/player_sheet.png");
    let sprite_atlas =
        TextureAtlas::from_grid_with_padding(texture_handle.clone(), Vec2::new(32.0, 50.0), 8, 2, Vec2::new(0.0, 0.0));
    let sprite_handle = texture_atlases.add(sprite_atlas);
    textureAtlasCenter.0.insert("player".to_string(), sprite_handle);

    // circle
    let mut imageHandle = asset_server.load("basicShape/circle.png");
    imageCenter.0.insert("circle".to_string(), imageHandle.clone());

    // shadow
    let mut imageHandle = asset_server.load("shadow/shadow.png");
    imageCenter.0.insert("shadow".to_string(), imageHandle.clone());
}
