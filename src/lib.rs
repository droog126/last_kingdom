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
        app.add_system_set(SystemSet::new()
        .with_run_criteria(FixedTimestep::step(0.5))
        .with_system(fps_setup_system));
        app.add_system(fps_setup_system);
    }

   
}
fn fps_start_system(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        .insert(FpsText);
}

fn fps_setup_system(diagnostics: Res<Diagnostics>, mut query: Query<(&mut Text, &FpsText)>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            for (mut text, i) in query.iter_mut() {
                text.sections[0].value = format!("fps:{:.2}", average);
            }
        }
    };
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
impl Plugin for DebugPlugin{
    fn build(&self, app: &mut App) {
        
    }
}

struct DebugRes {
    env:String
}
impl FromWorld for DebugRes{
    fn from_world(world: &mut World) -> Self {
        DebugRes{
            env:"DEV".to_string()
        }
    }
}

fn debug_system(actions:Res<Actions>,mut debug_res:Res<DEbugRes>){
    println!("action",actions)
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in player_query.iter_mut() {
        player_transform.translation += movement;
    }
}

// Debug control end