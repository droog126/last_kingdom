use bevy::{core::FixedTimestep, math::Vec3Swizzles, prelude::*};
use bevy_prototype_lyon::{prelude::*, render::Shape};
use duckduckgeo::{self, ErrTooClose};

use broccoli::{
    axgeom::Rect,
    prelude::*,
    queries::intersect_with::intersect_with_iter_mut,
    tree::{
        aabb_pin::AabbPin,
        bbox,
        node::{BBox, Num},
        rect,
    },
};

#[derive(Component)]
pub struct CollisionDynTag;

#[derive(Component)]
pub struct CollisionStaTag;

#[derive(Reflect, Component, Default, Debug)]
#[reflect(Component)]
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
    pub force: Vec2,
    pub wall_move: [Option<f32>; 2],
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
    mut set: ParamSet<(
        // 好像静态物体不需要bot
        Query<(&Transform, &mut CollisionBot, Option<&CollisionConfig>), With<CollisionStaTag>>,
        Query<(&mut Transform, &mut CollisionBot, Option<&CollisionConfig>), With<CollisionDynTag>>,
    )>,
) {
    // 1. 动静碰撞，先收集静态物体
    let mut staQuery = set.p0();
    let mut staVec = vec![];
    for (transform, mut collisionBot, collisionConfig) in staQuery.iter_mut() {
        let mut configWidth = 5.;
        let mut configHeight = 5.;
        if let Some(CollisionConfig { width, height }) = collisionConfig {
            configWidth = *width as f32;
            configHeight = *height as f32;
        }
        let mut target = Rect::new(
            transform.translation.x - configWidth / 2.,
            transform.translation.x + configWidth / 2.,
            transform.translation.y - configHeight / 2.,
            transform.translation.y + configHeight / 2.,
        );
        staVec.push(target);
    }

    let mut dynQuery = set.p1();
    let mut aabbs: Vec<BBox<f32, _>> = Vec::new();

    for (mut transform, mut collisionBot, collisionConfig) in dynQuery.iter_mut() {
        let mut configWidth = 5.;
        let mut configHeight = 5.;
        if let Some(CollisionConfig { width, height }) = collisionConfig {
            configWidth = *width as f32;
            configHeight = *height as f32;
        }

        // 动动碰撞影响
        transform.translation.x += collisionBot.force.x;
        transform.translation.y += collisionBot.force.y;
        collisionBot.force = Vec2::new(0., 0.);

        // 静静碰撞影响
        if let Some(dir) = collisionBot.wall_move[0] {
            if let Some(pos) = collisionBot.wall_move[1] {
                match dir {
                    1. => {
                        transform.translation.y = pos - configHeight / 2. - 0.1;
                    }
                    2. => {
                        transform.translation.x = pos + configWidth / 2. + 0.1;
                    }
                    3. => {
                        transform.translation.y = pos + configHeight / 2. + 0.1;
                    }
                    4. => {
                        transform.translation.x = pos - configWidth / 2. - 0.1;
                    }
                    _ => {}
                }
            }
        }

        collisionBot.wall_move[0] = None;
        collisionBot.wall_move[1] = None;
        collisionBot.pos = transform.translation.xy();

        let mut target = bbox(
            rect(
                transform.translation.x - configWidth / 2.,
                transform.translation.x + configWidth / 2.,
                transform.translation.y - configHeight / 2.,
                transform.translation.y + configHeight / 2.,
            ),
            collisionBot,
        );
        aabbs.push(target);
    }

    println!("len: {:?}", aabbs.len());

    let mut tree = broccoli::tree::new_par(&mut aabbs);

    // 动静碰撞  把碰撞的那边坐标传过去
    let mut hello = AabbPin::new(staVec.as_mut_slice());
    for i in AabbPin::new(staVec.as_mut_slice()).iter_mut() {
        tree.for_all_intersect_rect_mut(i, |r, mut a| {
            let (rect, bot) = a.destruct_mut();

            let wallx = &r.x;
            let wally = &r.y;
            let ret = match duckduckgeo::collide_with_rect(&rect, &r).unwrap() {
                duckduckgeo::WallSide::Above => [Some(1.), Some(wally.start)],
                duckduckgeo::WallSide::Below => [Some(3.), Some(wally.end)],
                duckduckgeo::WallSide::LeftOf => [Some(4.), Some(wallx.start)],
                duckduckgeo::WallSide::RightOf => [Some(2.), Some(wallx.end)],
            };
            bot.wall_move = ret;
        })
    }

    // 动动碰撞  根据pos 计算出force
    tree.colliding_pairs_builder(|a, b| {
        let a = a.unpack_inner();
        let b = b.unpack_inner();
        let _ = repel([(a.pos, &mut a.force), (b.pos, &mut b.force)], 0.001, 1.);
    })
    .build_par();
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
