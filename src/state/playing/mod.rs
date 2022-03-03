use crate::state::GameState;
use bevy::prelude::*;
pub struct PlayingPlugin;
impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(playing_enter))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(playing_setup))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(playing_exit));
    }
}

fn playing_enter() {
    println!("进入了游戏")
}

fn playing_setup() {
    // println!("我是游戏状态")
}

fn playing_exit() {}
