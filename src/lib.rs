#![allow(non_snake_case)]

mod actions;
mod audio;
mod instance;
mod state;
mod systems;
mod utils;

use state::loading::LoadingPlugin;
use state::menu::MenuPlugin;
use state::playing::PlayingPlugin;
use state::GameState;

#[cfg(debug_assertions)]
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
            .add_plugin(systems::input::InputPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(systems::debug::egui::EGuiPlugin)
                .add_plugin(systems::debug::fps::FpsPlugin)
                .add_plugin(systems::debug::DebugPlugin);
            // .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
