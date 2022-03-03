use crate::state::GameState;
use bevy::prelude::*;
pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(menu_enter))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(menu_setup))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(menu_exit));
    }
}

fn menu_enter(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    println!("进入了菜单")
}

fn menu_setup(mut gameState: ResMut<State<GameState>>) {
    gameState.set(GameState::Playing);
    println!("我是菜单状态");
}

fn menu_exit() {}
