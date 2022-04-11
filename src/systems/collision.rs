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

// #[derive(Component)]
// pub struct MainCameraTag;

// pub struct CursorPosition {
//     x: f32,
//     y: f32,
// }

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub struct CollisionCircle {
    size: f32,
    // wall_move: [Option<(f32, f32)>; 2],
}

#[derive(Debug, Clone)]
pub struct GlobalAabbs(Vec<BBox<i32, i32>>);

impl CollisionCircle {
    fn update(&mut self) {
        // self.vel += self.force;
        // //non linear drag
        // self.vel *= 0.9;

        // self.pos += self.vel;

        // self.force = Vec2::splat(0.0);
    }
}

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

fn startup(mut commands: Commands) {
    commands.insert_resource(GlobalAabbs(
        [
            bbox(rect(00, 10, 00, 10), 0),
            bbox(rect(15, 20, 15, 20), 0),
            bbox(rect(05, 15, 05, 15), 0),
        ]
        .to_vec(),
    ));

    // //Create a layer of direction.
    // let mut ref_aabbs = aabbs.iter_mut().collect::<Vec<_>>();

    // //This will change the order of the elements in bboxes,
    // //but this is okay since we populated it with mutable references.
    // let mut tree = broccoli::tree::new(&mut ref_aabbs);

    // //Find all colliding aabbs.
    // tree.colliding_pairs(|a, b| {
    //     *a.unpack_inner() += 1;
    //     *b.unpack_inner() += 1;
    // });

    // assert_eq!(aabbs[0].inner, 1);
    // assert_eq!(aabbs[1].inner, 0);
    // assert_eq!(aabbs[2].inner, 1);

    // let radius = 5.0;

    // let mut bots = make_rand()
    //     .take(20)
    //     .map(|pos| CollisionCircle {
    //         pos: pos.into(),
    //         vel: Vec2::splat(0.0),
    //         force: Vec2::splat(0.0),
    //     })
    //     .collect::<Vec<_>>();

    // println!("lookme{:?}", bots);
}

fn step(
    mut commands: Commands,
    query: Query<(&mut Transform, &CollisionCircle)>,
    mut globalAabbs: ResMut<GlobalAabbs>,
) {
    //Create a layer of direction.
    let mut ref_aabbs = globalAabbs.0.clone();

    let mut tree = broccoli::tree::new(&mut ref_aabbs);

    // //This will change the order of the elements in bboxes,
    // //but this is okay since we populated it with mutable references.

    //Find all colliding aabbs.
    tree.colliding_pairs(|a, b| {
        *a.unpack_inner() += 1;
        *b.unpack_inner() += 1;
    });

    // println!("碰撞结果{:?}", ref_aabbs);
    // assert_eq!(ref_aabbs[0].inner, 1);
    // assert_eq!(ref_aabbs[1].inner, 0);
    // assert_eq!(ref_aabbs[2].inner, 1);
}
