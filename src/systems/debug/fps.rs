use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::time::FixedTimestep;

use crate::state::GameState;

use super::egui::DebugTable;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct GameStateText;

pub struct FpsPlugin;
impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(debug_assertions))]
        {
            app.add_startup_system(fps_text_startup);
            app.add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(0.5)).with_system(fps_show));
            app.add_system(fps_get);
        }

        #[cfg(debug_assertions)]
        {
            app.add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(0.5)).with_system(fps_get));
        }
    }
}

fn fps_get(diagnostics: Res<Diagnostics>, mut debugTable: ResMut<DebugTable>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            debugTable.fps = Some(average);
        }
    };
}

fn fps_text_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 放入函数里
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect { bottom: Val::Px(40.0), left: Val::Px(5.0), ..Default::default() },
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "appState".to_string(),
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
                position: UiRect { bottom: Val::Px(5.0), left: Val::Px(5.0), ..Default::default() },
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "fps\n fuck you".to_string(),
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

fn fps_show(
    diagnostics: Res<Diagnostics>,
    mut set: ParamSet<(Query<&mut Text, With<FpsText>>, Query<&mut Text, With<GameStateText>>)>,
    gameState: Res<State<GameState>>,
    debugTable: Res<DebugTable>,
    time: Res<Time>,
) {
    // if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
    //     if let Some(average) = fps.average() {
    //         for mut text in set.p0().iter_mut() {
    //             text.sections[0].value = format!("fps:{:.2}\n ", average);
    //         }
    //     }
    // };

    for mut text in set.p1().iter_mut() {
        // text.sections[0].value = format!("State:{:#?}", gameState)
        text.sections[0].value = format!("fps:{:.2}", 1.0 / time.delta_seconds())
    }
}
