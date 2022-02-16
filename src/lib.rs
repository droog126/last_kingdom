mod actions;
mod audio;
mod loading;
mod menu;
mod player;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::{app::App, diagnostic::Diagnostics};

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}


// fps start
pub struct FpsPlugin;

#[derive(Component)]
struct FpsText;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(fps_start_system);
        app.add_system(fps_setup_system);
    }
}
fn fps_start_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(TextBundle {
        text:Text{
            sections:vec![
                TextSection{
                    value:"fps".to_string(),
                    style:TextStyle{
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.0, 1.0, 0.0),
                    }
                }
            ],
            ..Default::default()
        },
        ..Default::default()
    }).insert(FpsText);
}

fn fps_setup_system(diagnostics: Res<Diagnostics>, mut query: Query<(&mut Text,&FpsText)>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            for (mut text, i) in query.iter_mut() {
                text.sections[0].value = format!("fps:{:.2}", average);
            }
        }
    };
}
// fps end