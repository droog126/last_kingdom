use bevy::{core::FixedTimestep, math::Vec3Swizzles, prelude::*};
use bevy_prototype_lyon::{prelude::*, render::Shape};
use duckduckgeo::{self, ErrTooClose};

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
pub struct CollisionDynTag;

#[derive(Component)]
pub struct CollisionConfig {
    pub width: i32,
    pub height: i32,
}

#[derive(Component)]
pub struct CollisionID(pub Entity);

pub struct CollisionEvent {
    pub entity: Option<Entity>,
}

#[derive(Debug, Clone)]
pub struct GlobalAabbs(Vec<BBox<i32, i32>>);

#[derive(Copy, Clone, Component, Debug)]
pub struct CollisionBot {
    pub pos: Vec2,
    pub vel: Vec2,
    pub force: Vec2,
    pub wall_move: [Option<(f32, f32)>; 2],
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
    }
}

fn startup(mut commands: Commands) {}

// 需要在接受输入同步新位置后调用
fn step(
    mut query: Query<
        (&mut Transform, &mut CollisionBot, Option<&CollisionConfig>),
        With<CollisionDynTag>,
    >,
) {
    // println!("start");

    // 1.转换shap->rect;
    let mut aabbs: Vec<BBox<f32, _>> = Vec::new();

    for (mut transform, mut collisionBot, collisionConfig) in query.iter_mut() {
        transform.translation.x += collisionBot.force.x;
        transform.translation.y += collisionBot.force.y;
        collisionBot.force = Vec2::new(0., 0.);

        collisionBot.pos = transform.translation.xy();
        let mut target = match collisionConfig {
            None => bbox(
                rect(
                    transform.translation.x - 5.,
                    transform.translation.x + 5.,
                    transform.translation.y - 5.,
                    transform.translation.y + 5.,
                ),
                collisionBot,
            ),
            Some(config) => bbox(
                rect(
                    transform.translation.x - config.width as f32,
                    transform.translation.x + config.width as f32,
                    transform.translation.y - config.height as f32,
                    transform.translation.y + config.height as f32,
                ),
                collisionBot,
            ),
        };
        aabbs.push(target);
    }

    println!("len: {:?}", aabbs.len());

    let mut tree = broccoli::tree::new_par(&mut aabbs);

    tree.colliding_pairs_builder(|a, b| {
        let a = a.unpack_inner();
        let b = b.unpack_inner();
        // println!("碰撞前 {:?}  {:?}", a, b);
        let _ = repel([(a.pos, &mut a.force), (b.pos, &mut b.force)], 0.001, 1.);

        // println!("碰撞后 {:?}  {:?}", a, b);
    })
    .build_par();
    // println!("aabbs: {:?}", aabbs);
}

pub fn repel(bots: [(Vec2, &mut Vec2); 2], closest: f32, mag: f32) -> Result<(), ErrTooClose> {
    let [(bot1_pos, bot1_force_buffer), (bot2_pos, bot2_force_buffer)] = bots;

    let diff = bot2_pos - bot1_pos;

    let len_sqr = diff.length();

    if len_sqr < closest {
        return Err(ErrTooClose);
    }

    let len = len_sqr.sqrt();
    let mag = mag / len;

    let force = diff.normalize() * Vec2::splat(mag);

    // println!("force: {:?}", force);

    *bot1_force_buffer -= force;
    *bot2_force_buffer += force;

    Ok(())
}
