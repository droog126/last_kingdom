use crate::systems::input::InsInput;
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};

pub struct EGuiPlugin;
impl Plugin for EGuiPlugin {
    fn build(&self, app: &mut App) {
        app
            //.register_inspectable::<InsInput>()
            .register_type::<InsInput>()
            .add_plugin(WorldInspectorPlugin::new());
    }
}
