#![allow(non_snake_case)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

pub mod config;
mod instance;
mod state;
mod systems;
mod utils;

use state::loading::LoadingPlugin;
use state::menu::MenuPlugin;
use state::playing::PlayingPlugin;
use state::GameState;

use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::{app::App, diagnostic::FrameTimeDiagnosticsPlugin};
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(PlayingPlugin)
            // system
            .add_plugin(systems::input::InputPlugin)
            .add_plugin(systems::stateMachine::StateMachinePlugin)
            .add_plugin(systems::camera::CameraPlugin)
            // .add_plugin(systems::title::TitlePlugin)
            .add_plugin(systems::collision::CollisionPlugin)
            .add_plugin(systems::instance::shadow::ShadowPlugin);

        // .add_plugin(systems::render::RenderPlugin);

        // #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(systems::debug::egui::EGuiPlugin)
                .add_plugin(systems::debug::fps::FpsPlugin)
                .add_plugin(systems::debug::collision::CollisionDebugPlugin)
                .add_plugin(systems::debug::DebugPlugin);
        }
    }
}
