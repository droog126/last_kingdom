use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin).add_startup_system(startup).add_system(step);
    }
}

fn startup(mut egui_ctx: ResMut<EguiContext>, mut commands: Commands) {}

fn step(mut commands: Commands, mut egui_ctx: ResMut<EguiContext>) {}
