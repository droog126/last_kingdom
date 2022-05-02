use std::collections::HashMap;

use crate::state::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

pub struct LoadingPlugin;

pub struct SpriteCenter(pub HashMap<String, Handle<TextureAtlas>>);
impl FromWorld for SpriteCenter {
    fn from_world(world: &mut World) -> Self {
        let mut spriteCenter: HashMap<String, Handle<TextureAtlas>> = HashMap::new();
        SpriteCenter(spriteCenter)
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
        app.init_resource::<SpriteCenter>()
            .add_startup_system(startup);
    }
}

//这些都在Res里面
#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

fn startup(
    mut spriteCenter: ResMut<SpriteCenter>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprite/snake_sheet.png");
    let sprite_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle.clone(),
        Vec2::new(32.0, 32.0),
        8,
        5,
        Vec2::new(0.0, 0.0),
    );

    let sprite_handle = texture_atlases.add(sprite_atlas);
    spriteCenter.0.insert("snake".to_string(), sprite_handle);
}
