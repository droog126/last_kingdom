use crate::instance::player::{player_create, player_step};
use crate::state::GameState;
use bevy::core::FixedTimestep;
use bevy::prelude::*;

pub struct PlayingPlugin;
impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(playing_enter)
                .with_system(player_create.config(|params| {
                    params.0 = Some(true);
                })),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(playing_setup))
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::step(0.015))
                .with_system(player_step),
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(playing_exit));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn playing_enter(mut commands: Commands) {
    println!("游戏开始")
}

fn playing_setup() {
    // println!("游戏进行中")
}

fn playing_exit() {}
