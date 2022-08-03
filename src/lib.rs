#![allow(non_snake_case)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

pub mod config;
mod state;
mod systems;
mod utils;

use bevy::render::texture::ImageSettings;
use state::loading::loading_start;
use state::GameState;

use bevy::app::App;
use bevy::app::Plugin;
use bevy::prelude::*;
use state::playing::playing_start;
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
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

        // 阶段
        app.add_state(GameState::Loading);
        loading_start(app);
        playing_start(app);
    }
}

#[macro_export]
macro_rules! add_events{
    ($a:ident,$b:ty)=>{
        {
            $a.add_event::<$b>()
        }
    };
    ($a:ident,$b:ty $(,$c:ty)*)=> {
        {
            add_events!($a, $b); add_events!($a $(,$c)*)
        }
    }
}
