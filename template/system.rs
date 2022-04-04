use bevy::prelude::*;

// #[derive(Component)]
// pub struct MainCameraTag;

// pub struct CursorPosition {
//     x: f32,
//     y: f32,
// }

pub struct TitlePlugin;
impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup).add_system(step);

        // app.add_startup_system(camera_create)
        //     // .insert_resource(CursorPosition { x: 0.0, y: 0.0 })
        //     .add_system(camera_step);
    }
}
fn startup(mut commands: Commands) {}
fn step(mut commands: Commands) {}
