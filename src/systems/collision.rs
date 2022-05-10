use bevy::{core::FixedTimestep, math::Vec3Swizzles, prelude::*, utils::hashbrown::HashMap};
use bevy_prototype_lyon::{
    prelude::{tess::geom::Vector, *},
    render::Shape,
};
use duckduckgeo::{self, ErrTooClose};

use broccoli::{
    axgeom::Rect,
    tree::{
        aabb_pin::AabbPin,
        bbox,
        node::{BBox, Num},
        rect,
    },
};

use crate::instance::InstanceType;

use super::debug::egui::DebugTable;

#[derive(Component)]
pub struct CollisionID(pub Entity);

#[derive(Clone, Debug, PartialEq)]
pub enum CollisionInner {
    Static,
    Scope {
        parentId: Entity,
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

enum  CollisionInstanceType{
    Static,
    Scope,
    Instance,
}
// 生产因子一般来源于需求，也就是消费者
pub struct CollisionProductionFactor{
    pub id:Entity,
    pub _type:CollisionInstanceType,
    pub pos:Vec2,

    pub width:f32,
    pub height:f32,
    // pub is_accurate:bool
}

// 消费的是什么
// 1.排斥力  2.静止进入 3.是否碰撞  2和3 可以合起来  
// 共同点是 卧槽了，这不就是形状坐标吗? 不是点 不是点.

pub struct CollisionEventMap {
    pub map: HashMap<InstanceType, Vec<CollisionDessert>>,
}
pub struct CollisionDessert {
   pub id:Entity,
   // 后面这个需要改一下哈
   pub x:f32,
   pub y:f32,
   pub width:f32,
   pub height:f32,
} 


pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource()
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
// 感觉哈，只需要这里告诉我他们是否碰撞了, 然后他们自己在自己的step里面处理自己的碰撞就好了
// snake_step 清空自己的碰撞事件(消费)  collision_step 接受碰撞(生产)
// 
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
            globalTransform.translation.x - configWidth / 2.,
            globalTransform.translation.x + configWidth / 2.,
            globalTransform.translation.y - configHeight / 2.,
            globalTransform.translation.y + configHeight / 2.,
        );
        let collisionInner = &mut collisionBot.collisionInner;
        match collisionInner {
            CollisionInner::Static => {
                staBots.push(rect);
            }
            CollisionInner::Scope { other, parentId } => dynBots.push(bbox(rect, collisionBot)),
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

    let mut tree = broccoli::Tree::par_new(&mut dynBots);

    // 实体和墙碰撞  把碰撞的那边坐标传过去

    tree.find_colliding_pairs_with_iter(
        AabbPin::new(staBots.as_mut_slice()).iter_mut(),
        |mut bot, r| {
            let (rect, bot) = bot.destruct_mut();

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
        },
    );

    tree.par_find_colliding_pairs(|a, b| {
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
            // (CollisionInner::Scope { other }, CollisionInner::Scope { other: b_other }) => todo!(),
            (
                CollisionInner::Scope { other, parentId },
                CollisionInner::Instance {
                    pos,
                    force,
                    wall_move,
                },
            ) => {
                // println!("id:{:?}", b.id);
                other.push(b.id);
            }
            (
                CollisionInner::Instance {
                    pos,
                    force,
                    wall_move,
                },
                CollisionInner::Scope { other, parentId },
            ) => {
                other.push(a.id);
                // println!("id:{:?}", a.id);
            }
            _ => {}
        }
    });
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
