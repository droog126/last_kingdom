#![allow(non_snake_case)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

pub mod config;
mod state;
mod systems;
mod utils;

use state::loading::LoadingPlugin;
use state::menu::MenuPlugin;
use state::playing::PlayingPlugin;
use state::GameState;

use bevy::app::App;
use bevy::app::Plugin;
use bevy::prelude::*;
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // 阶段
            .add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(PlayingPlugin)
            // 总线系统
            .add_stage_before(CoreStage::Update, "origin", SystemStage::parallel())
            .add_system_to_stage("origin", state::origin::exclusive_system.exclusive_system())
            // 实例系统
            // system
            .add_plugin(systems::input::InputPlugin)
            .add_plugin(systems::camera::CameraPlugin)
            // .add_plugin(systems::title::TitlePlugin)
            .add_plugin(systems::ui::UiPlugin)
            .add_plugin(systems::timeLine::TimeLinePlugin)
            .add_plugin(systems::debug::DebugPlugin);

        // 总线系统
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
