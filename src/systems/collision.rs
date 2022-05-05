use bevy::{core::FixedTimestep, math::Vec3Swizzles, prelude::*};
use bevy_prototype_lyon::{
    prelude::{tess::geom::Vector, *},
    render::Shape,
};
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

use super::debug::egui::DebugTable;

#[derive(Component)]
pub struct CollisionID(pub Entity);

#[derive(Clone, Debug, PartialEq)]
pub enum CollisionInner {
    Static,
    Scope {
        other: Vec<Entity>,
    },
    Instance {
        pos: Vec2,
        force: Vec2,
        wall_move: [Option<f32>; 2],
    },
}
#[derive(Component)]
pub struct CollisionBot {
    pub id: Entity,
    pub collisionInner: CollisionInner,
    pub width: f32,
    pub height: f32,
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 4 })
            // .add_event::<CollisionScopeEvent>()
            .add_plugin(ShapePlugin)
            .add_startup_system(startup);
    }
}

fn startup(mut commands: Commands) {}

// 需要在接受输入同步新位置后调用
// query拿出来的值需要回收的，你不能把他的子属性取出来，因为他也不知道你什么时候回收.
// 整体移入是所有权的转移  子属性移入可以是copy 或者 就是不能
pub fn collision_step(
    mut query: Query<(&GlobalTransform, &mut Transform, &mut CollisionBot)>,
    mut debugTable: ResMut<DebugTable>,
) {
    let mut staBots: Vec<_> = vec![];

    let mut dynBots: Vec<_> = vec![];
    for (globalTransform, mut transform, mut collisionBot) in query.iter_mut() {
        let mut configWidth = collisionBot.width;
        let mut configHeight = collisionBot.height;
        let mut rect = Rect::new(
            transform.translation.x - configWidth / 2.,
            transform.translation.x + configWidth / 2.,
            transform.translation.y - configHeight / 2.,
            transform.translation.y + configHeight / 2.,
        );
        let collisionInner = &mut collisionBot.collisionInner;
        match collisionInner {
            CollisionInner::Static => {
                staBots.push(rect);
            }
            CollisionInner::Scope { other } => dynBots.push(bbox(rect, collisionBot)),
            CollisionInner::Instance {
                pos,
                force,
                wall_move,
            } => {
                transform.translation.x += force.x;
                transform.translation.y += force.y;
                force.x = 0.0;
                force.y = 0.0;
                // 静静碰撞影响
                if let Some(dir) = wall_move[0] {
                    if let Some(pos) = wall_move[1] {
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

                wall_move[0] = None;
                wall_move[1] = None;
                pos.x = transform.translation.x;
                pos.y = transform.translation.y;
                dynBots.push(bbox(rect, collisionBot))
            }
        }
    }

    #[cfg(debug_assertions)]
    {
        debugTable.collisionCount = Some(dynBots.len());
    }

    let mut tree = broccoli::tree::new_par(&mut dynBots);

    // 实体和墙碰撞  把碰撞的那边坐标传过去
    for i in AabbPin::new(staBots.as_mut_slice()).iter_mut() {
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

            match &mut bot.collisionInner {
                CollisionInner::Instance {
                    pos,
                    force,
                    wall_move,
                } => {
                    wall_move[0] = ret[0];
                    wall_move[1] = ret[1];
                }
                _ => {}
            }
        })
    }

    tree.colliding_pairs_builder(|a, b| {
        let a = a.unpack_inner();
        let b = b.unpack_inner();
        match (&mut a.collisionInner, &mut b.collisionInner) {
            (
                CollisionInner::Instance {
                    pos,
                    force,
                    wall_move,
                },
                CollisionInner::Instance {
                    pos: b_pos,
                    force: b_force,
                    wall_move: b_wall_move,
                },
            ) => {
                repel([(pos, force), (b_pos, b_force)], 0.001, 1.);
            }
            (CollisionInner::Scope { other }, CollisionInner::Scope { other: b_other }) => todo!(),
            (
                CollisionInner::Scope { other },
                CollisionInner::Instance {
                    pos,
                    force,
                    wall_move,
                },
            ) => {
                other.push(b.id);
            }
            (
                CollisionInner::Instance {
                    pos,
                    force,
                    wall_move,
                },
                CollisionInner::Scope { other },
            ) => other.push(a.id),
            _ => {}
        }
    })
    .build_par();
}

pub fn repel(bots: [(&mut Vec2, &mut Vec2); 2], closest: f32, mag: f32) -> Result<(), ErrTooClose> {
    // println!("{:?}", bots);
    let [(bot1_pos, bot1_force_buffer), (bot2_pos, bot2_force_buffer)] = bots;

    let diff = Vec2::new(bot2_pos.x - bot1_pos.x, bot2_pos.y - bot1_pos.y);

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

fn handle_collision(a: &mut Mut<CollisionBot>, b: &mut Mut<CollisionBot>) {}
