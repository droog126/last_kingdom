use crate::systems::{input::InsInput, stateMachine::InsState};
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};

// use bevy_editor_pls::EditorPlugin;

use bevy_egui::{egui, EguiContext};

pub struct DebugTable {
    pub fps: Option<f64>,
    pub collisionCount: Option<usize>,
    pub timeLine: Option<i32>,
}

pub struct EGuiPlugin;
impl Plugin for EGuiPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugin(EditorPlugin);
            .register_type::<InsInput>()
            // .register_type::<CollisionConfig>()
            .add_plugin(WorldInspectorPlugin::new())
            .add_system(debug_table_step);
    }
}

fn debug_table_step(mut egui_ctx: ResMut<EguiContext>, debugTable: Res<DebugTable>) {
    egui::Window::new("ebugTable").show(egui_ctx.ctx_mut(), |ui| {
        ui.label(format!("fps:{:.2}", debugTable.fps.unwrap_or(0.0)));
        ui.label(format!(
            "collisionCount:{:?}",
            debugTable.collisionCount.unwrap_or(0)
        ));
        ui.label(format!("timeLine:{:?}", debugTable.timeLine.unwrap_or(0)));
    });
}
