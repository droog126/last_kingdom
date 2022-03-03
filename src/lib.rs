#![allow(non_snake_case)]

mod actions;
mod audio;
mod loading;
mod menu;
mod player;

use std::fmt::Debug;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::core::FixedTimestep;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::{app::App, diagnostic::Diagnostics};
use bevy_inspector_egui::WorldInspectorPlugin;

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
            .add_plugin(PlayerPlugin)
            .add_plugin(DebugPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default());
            // .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}

// fps start
pub struct FpsPlugin;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct GameStateText;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(fps_start_system);
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(fps_setup_system),
        );
        app.add_system(fps_setup_system);
    }
}
fn fps_start_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 放入函数里
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(40.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "fps".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::rgb(0.0, 0.0, 0.0),
                    },
                }],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GameStateText);
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "fps/\n fuck you".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::rgb(0.0, 0.0, 0.0),
                    },
                }],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FpsText);
}

fn fps_setup_system(
    diagnostics: Res<Diagnostics>,
    mut queries: QuerySet<(
        QueryState<&mut Text, (With<FpsText>)>,
        QueryState<&mut Text, (With<GameStateText>)>,
    )>,
    gameState: Res<State<GameState>>,
) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            for mut text in queries.q0().iter_mut() {
                text.sections[0].value = format!("fps:{:.2}\n fuck you", average);
            }
        }
    };

    for mut text in queries.q1().iter_mut() {
        text.sections[0].value = format!("State:{:#?}", gameState)
    }
}
// fps end

// Egui start
pub struct EGuiPlugin;
impl Plugin for EGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new());
    }
}

// Egui end

// Debug control start
struct DebugPlugin;
struct DebugRes {
    env: bool,
}
impl FromWorld for DebugRes {
    fn from_world(world: &mut World) -> Self {
        DebugRes { env: true }
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugRes>();
        app.add_system_set(SystemSet::new().with_system(debug_system));
    }
}

fn debug_system(
    input: Res<Input<KeyCode>>,
    mut debug_res: ResMut<DebugRes>,
    mut query: Query<(&mut Visibility, With<FpsText>)>,
) {
    if (input.just_pressed(KeyCode::F11)) {
        debug_res.env = !debug_res.env;
        for (mut visibility, i) in query.iter_mut() {
            visibility.is_visible = !visibility.is_visible;
        }
    }
}

// Debug control end
