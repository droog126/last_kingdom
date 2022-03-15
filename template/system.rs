use bevy::prelude::*;

#[derive(Component)]
pub struct InputIns;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {}
}
