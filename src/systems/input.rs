use crate::utils::num::*;
use bevy::prelude::*;

#[derive(Reflect, Component, Default, Debug)]
#[reflect(Component)]
pub struct InsInput {
    pub dir: Vec2,
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ins_input_read);
    }
}

fn ins_input_read(keyboard: Res<Input<KeyCode>>, mut query: Query<&mut InsInput>) {
    for (mut insInput) in query.iter_mut() {
        let xx =
            bool_to_f32(keyboard.pressed(KeyCode::D)) - bool_to_f32(keyboard.pressed(KeyCode::A));
        let yy =
            bool_to_f32(keyboard.pressed(KeyCode::W)) - bool_to_f32(keyboard.pressed(KeyCode::S));

        let newDir = match (xx, yy) {
            (0.0, 0.0) => Vec2::new(0.0, 0.0),
            (x, y) => Vec2::new(x, y).normalize(),
            _ => Vec2::new(0.0, 0.0),
        };

        insInput.dir = newDir;
        // println!("input hello {:?}", insInput);
    }
}
