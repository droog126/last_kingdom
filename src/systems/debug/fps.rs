use bevy::core::FixedTimestep;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

use crate::state::GameState;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct GameStateText;

pub struct FpsPlugin;
impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(fps_start_system);
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(fps_setup_system),
        );
    }
}
fn fps_start_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 放入函数里
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(40.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
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
                position: Rect {
                    bottom: Val::Px(5.0),
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
