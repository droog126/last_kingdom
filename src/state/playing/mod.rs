use crate::instance::player::{player_create, player_step};
use crate::state::GameState;
use crate::systems::attack::attack_event_distribution_system;
use bevy::core::FixedTimestep;
use bevy::prelude::*;

pub struct PlayingPlugin;
impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(playing_enter))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(playing_setup)
                    .with_system(attack_event_distribution_system),
            )
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(playing_exit));
    }
}

fn playing_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("游戏开始");
    let texture: Handle<Image> = asset_server.load("title/firstUser/png/Level_0.png");
    commands.spawn_bundle(SpriteBundle {
        texture: texture.clone(),
        ..default()
    });
    // 暂时在这里创建instance
}

fn playing_setup() {
    // println!("游戏进行中")
}

fn playing_exit() {}
