use bevy::prelude::*;

#[derive(Deref, DerefMut)]
pub struct ShadowAsset(Handle<Image>);
pub struct Instance;
pub struct ShadowPlugin;
impl Plugin for ShadowPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup).add_system(step);
    }
}
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("shadow/shadow.png");
    commands.insert_resource(ShadowAsset(texture));
}
fn step(mut commands: Commands) {}
