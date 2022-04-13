use bevy::{core::FixedTimestep, prelude::*};
use bevy_prototype_lyon::{prelude::*, render::Shape};

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
                SystemSet::new().with_system(step), // .with_run_criteria(FixedTimestep::step(1.)),
            );
    }
}

fn startup(mut commands: Commands) {}

fn step(mut query: Query<(&GlobalTransform), With<CollisionTag>>) {
    // println!("start");

    // 1.转换shap->rect;
    let mut aabbs: Vec<_> = Vec::new();

    for (gloablTransform) in query.iter() {
        let target = bbox(
            rect(
                gloablTransform.translation.x - 5.,
                gloablTransform.translation.x + 5.,
                gloablTransform.translation.y - 5.,
                gloablTransform.translation.y + 5.,
            ),
            0,
        );
        aabbs.push(target);
    }

    println!("len: {:?}", aabbs.len());

    let mut tree = broccoli::tree::new(&mut aabbs);

    tree.colliding_pairs(|a, b| {
        *a.unpack_inner() += 1;
        *b.unpack_inner() += 1;
        // println!("碰撞了")
    });
    // println!("aabbs: {:?}", aabbs);
}
