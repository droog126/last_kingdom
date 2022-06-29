use crate::systems;
use crate::systems::instance::attack::attack_event_distribution_system;
use crate::systems::instance::instanceType::player::player_create;
use crate::systems::instance::instanceType::snake::snake_step;
use crate::systems::instance::z_depth_step;
use crate::{state::GameState, systems::instance::instanceType::player::player_step};
use bevy::core::FixedTimestep;
use bevy::prelude::*;

use super::loading::{ImageCenter, TextureAtlasCenter};

pub struct PlayingPlugin;
impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(systems::instance::collision::CollisionPlugin)
            .add_plugin(systems::instance::animation::AnimationPlugin);
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(playing_enter))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    // 运行系统
                    .with_system(playing_setup)
                    .with_system(attack_event_distribution_system)
                    .with_system(z_depth_step)
                    // 实体系统
                    .with_system(player_step)
                    .with_system(snake_step),
            )
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(playing_exit));
    }
}

fn playing_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    textureAtlasCenter: Res<TextureAtlasCenter>,
    imageCenter: Res<ImageCenter>,
) {
    println!("游戏开始");
    let texture: Handle<Image> = asset_server.load("title/firstUser/png/Level_0.png");
    commands.spawn_bundle(SpriteBundle { texture: texture.clone(), ..default() });
    // 暂时在这里创建instance
    player_create(&mut commands, &textureAtlasCenter, &imageCenter, 0.0, 0.0);
}

fn playing_setup() {
    // println!("游戏进行中")
}

fn playing_exit() {}
