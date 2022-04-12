use bevy::{core::FixedTimestep, prelude::*};
use bevy_prototype_lyon::prelude::*;
use rand::prelude::*;

use broccoli::{
    axgeom::Rect,
    prelude::*,
    tree::{
        bbox,
        node::{BBox, Num},
        rect,
    },
};

#[derive(Component)]
pub struct CollisionTag;

#[derive(Component)]
pub struct CollisionID(pub Entity);

#[derive(Debug, Clone)]
pub struct GlobalAabbs(Vec<BBox<i32, i32>>);

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 4 })
            .add_plugin(ShapePlugin)
            .add_startup_system(startup)
            // .add_system(step)
            .add_system_set(
                SystemSet::new()
                    .with_system(step)
                    .with_run_criteria(FixedTimestep::step(1.)),
            );

        // app.add_startup_system(camera_create)
        //     // .insert_resource(CursorPosition { x: 0.0, y: 0.0 })
        //     .add_system(camera_step);
    }
}

use broccoli::*;
use rand::Rng;
pub fn point_to_rect_f32(a: axgeom::Vec2<f32>, radius: f32) -> Rect<f32> {
    Rect::from_point(a, axgeom::vec2same(radius))
}

pub fn distribute<X, T: Num>(
    inner: &mut [X],
    mut func: impl FnMut(&X) -> Rect<T>,
) -> Vec<BBox<T, &mut X>> {
    inner.iter_mut().map(|a| bbox(func(a), a)).collect()
}

pub fn make_rand() -> impl Iterator<Item = [f32; 2]> {
    let mut rng = thread_rng();

    std::iter::repeat_with(move || {
        let randx = rng.gen::<f32>() * 1000.0;
        let randy = rng.gen::<f32>() * 1000.0;
        [randx, randy]
    })
}

fn startup(mut commands: Commands) {}

fn step() {}
