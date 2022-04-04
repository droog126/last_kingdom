use crate::systems::{input::InsInput, stateMachine::InsState};
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};

use bevy_editor_pls::*;

pub struct EGuiPlugin;
impl Plugin for EGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EditorPlugin);
        // .register_inspectable::<InsInput>()
        // .register_type::<InsInput>()
        // .add_plugin(WorldInspectorPlugin::new());
    }
}
