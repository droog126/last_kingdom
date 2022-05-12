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

#[derive(Clone, Debug, PartialEq, Copy, Hash, std::cmp::Eq)]
pub enum CollisionType {
    Static,
    Scope,
    Instance,
}
// 生产因子一般来源于需求，也就是消费者

#[derive(Component, Clone)]
pub struct CollisionProductionFactor {
    pub id: Entity,
    pub collisionType: CollisionType,
    pub instanceType: InstanceType,
    pub pos: Vec2,

    pub width: f32,
    pub height: f32,
    // pub is_accurate:bool
}

#[derive(Component, Clone)]
pub struct CollisionDessert {
    pub id: Entity,
    // 后面这个需要改一下哈
    pub collisionType: CollisionType,
    pub instanceType: InstanceType,
    pub pos: Vec2,
    pub width: f32,
    pub height: f32,
}

// 消费的是什么
// 1.排斥力  2.静止进入 3.是否碰撞  2和3 可以合起来
// 共同点是 卧槽了，这不就是形状坐标吗? 不是点 不是点.

pub struct CollisionEventMap {
    pub map: HashMap<InstanceType, HashMap<Entity, Vec<CollisionDessert>>>,
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CollisionEventMap {
            map: HashMap::new(),
        });
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
// 1.我觉得应该在自己的step中同步自己的碰撞因子
pub fn collision_step(
    mut query: Query<(&GlobalTransform, &CollisionProductionFactor)>,
    mut debugTable: ResMut<DebugTable>,
    mut collisionEventMapStruct: ResMut<CollisionEventMap>,
) {
    let mut collisionEventMap = &mut collisionEventMapStruct.map;
    let mut staBots: Vec<_> = vec![];

    let mut dynBots: Vec<_> = vec![];
    for (globalTransform, collisionProductionFactor) in query.iter() {
        let mut configWidth = collisionProductionFactor.width;
        let mut configHeight = collisionProductionFactor.height;
        let mut rect = Rect::new(
            globalTransform.translation.x - configWidth / 2.,
            globalTransform.translation.x + configWidth / 2.,
            globalTransform.translation.y - configHeight / 2.,
            globalTransform.translation.y + configHeight / 2.,
        );
        let collisionType = collisionProductionFactor.collisionType;
        match collisionType {
            CollisionType::Static => {
                staBots.push(rect);
            }
            _ => {
                let mut newCollisionProductionFactor = collisionProductionFactor.clone();
                newCollisionProductionFactor.pos = globalTransform.translation.xy();
                dynBots.push(bbox(rect, newCollisionProductionFactor));
            } // CollisionType::Scope { other, parentId } => dynBots.push(bbox(rect, collisionBot)),
              // CollisionType::Instance {
              //     pos,
              //     force,
              //     wall_move,
              // } => {
              //     transform.translation.x += force.x;
              //     transform.translation.y += force.y;
              //     force.x = 0.0;
              //     force.y = 0.0;
              //     // 静静碰撞影响
              //     if let Some(dir) = wall_move[0] {
              //         if let Some(pos) = wall_move[1] {
              //             match dir {
              //                 1. => {
              //                     transform.translation.y = pos - configHeight / 2. - 0.1;
              //                 }
              //                 2. => {
              //                     transform.translation.x = pos + configWidth / 2. + 0.1;
              //                 }
              //                 3. => {
              //                     transform.translation.y = pos + configHeight / 2. + 0.1;
              //                 }
              //                 4. => {
              //                     transform.translation.x = pos - configWidth / 2. - 0.1;
              //                 }
              //                 _ => {}
              //             }
              //         }
              //     }

              //     wall_move[0] = None;
              //     wall_move[1] = None;
              //     pos.x = transform.translation.x;
              //     pos.y = transform.translation.y;
              //     dynBots.push(bbox(rect, collisionBot))
              // }
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
            // let curEventMap = collisionEventMap.get_mut(&bot.instanceType).unwrap();

            add_collision_event(
                collisionEventMap,
                bot.instanceType,
                bot.id,
                CollisionDessert {
                    id: bot.id,
                    pos: Vec2::new(
                        (rect.x.start + rect.x.end) / 2.,
                        (rect.y.start + rect.y.end) / 2.,
                    ),
                    width: rect.x.end - rect.x.start,
                    height: rect.y.end - rect.y.start,
                    collisionType: CollisionType::Static,
                    instanceType: bot.instanceType,
                },
            );
        },
    );
    //         // let curEventVec = curEventMap.get_mut(&bot.id).unwrap_or(&mut Vec::new());
    //         // curEventVec.push(CollisionDessert {
    //         //     id: bot.id,
    //         //     pos: bot.pos,
    //         //     width: bot.width,
    //         //     height: bot.height,
    //         //     collisionType: bot.collisionType,
    //         //     instanceType: bot.instanceType,
    //         // });
    //         // let wallx = &r.x;
    //         // let wally = &r.y;
    //         // let ret = match duckduckgeo::collide_with_rect(&rect, &r).unwrap() {
    //         //     duckduckgeo::WallSide::Above => [Some(1.), Some(wally.start)],
    //         //     duckduckgeo::WallSide::Below => [Some(3.), Some(wally.end)],
    //         //     duckduckgeo::WallSide::LeftOf => [Some(4.), Some(wallx.start)],
    //         //     duckduckgeo::WallSide::RightOf => [Some(2.), Some(wallx.end)],
    //         // };

    //         // match &mut bot.collisionInner {
    //         //     CollisionInner::Instance {
    //         //         pos,
    //         //         force,
    //         //         wall_move,
    //         //     } => {
    //         //         wall_move[0] = ret[0];
    //         //         wall_move[1] = ret[1];
    //         //     }
    //         //     _ => {}
    //         // }
    //     },
    // );

    // let mut handle = |a: AabbPin<&mut BBox<f32, CollisionProductionFactor>>,
    //                   b: AabbPin<&mut BBox<f32, CollisionProductionFactor>>| {
    //     let aBot = a.unpack_inner();
    //     let bBot = b.unpack_inner();
    //     // 这里有闭包，会报错。

    //     add_collision_event(
    //         collisionEventMap,
    //         aBot.instanceType.clone(),
    //         aBot.id.clone(),
    //         CollisionDessert {
    //             id: bBot.id.clone(),
    //             collisionType: bBot.collisionType.clone(),
    //             instanceType: bBot.instanceType.clone(),
    //             pos: bBot.pos,
    //             width: bBot.width,
    //             height: bBot.height,
    //         },
    //     );
    // };
    // tree.par_find_colliding_pairs(handle);
    tree.par_find_colliding_pairs(|a, b| {
        let aBot = a.unpack_inner();
        let bBot = b.unpack_inner();
        add_collision_event(
            collisionEventMap,
            bBot.instanceType,
            bBot.id,
            CollisionDessert {
                id: aBot.id,
                collisionType: aBot.collisionType,
                instanceType: aBot.instanceType,
                pos: aBot.pos,
                width: aBot.width,
                height: aBot.height,
            },
        );

        // let curEventMap = collisionEventMap
        //     .get_mut(&aBot.instanceType)s
        //     .unwrap_or(&mut HashMap::new());
        // let curEventVec = curEventMap.get_mut(&aBot.id).unwrap_or(&mut Vec::new());
        // curEventVec.push(CollisionDessert {
        //     id: bBot.id,
        //     pos: bBot.pos,
        //     width: bBot.width,
        //     height: bBot.height,
        //     collisionType: bBot.collisionType,
        //     instanceType: bBot.instanceType,
        // });

        // curEventVec.push(CollisionDessert {
        //     id: a.id,

        // match (&mut a.collisionInner, &mut b.collisionInner) {
        //     (
        //         CollisionInner::Instance {
        //             pos,
        //             force,
        //             wall_move,
        //         },
        //         CollisionInner::Instance {
        //             pos: b_pos,
        //             force: b_force,
        //             wall_move: b_wall_move,
        //         },
        //     ) => {
        //         repel([(pos, force), (b_pos, b_force)], 0.001, 1.);
        //     }
        //     // (CollisionInner::Scope { other }, CollisionInner::Scope { other: b_other }) => todo!(),
        //     (
        //         CollisionInner::Scope { other, parentId },
        //         CollisionInner::Instance {
        //             pos,
        //             force,
        //             wall_move,
        //         },
        //     ) => {
        //         // println!("id:{:?}", b.id);
        //         other.push(b.id);
        //     }
        //     (
        //         CollisionInner::Instance {
        //             pos,
        //             force,
        //             wall_move,
        //         },
        //         CollisionInner::Scope { other, parentId },
        //     ) => {
        //         other.push(a.id);
        //         // println!("id:{:?}", a.id);
        //     }
        //     _ => {}
        // }
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

// fn handle_collision(a: &mut Mut<CollisionBot>, b: &mut Mut<CollisionBot>) {}

fn add_collision_event(
    collisionEventMap: &mut HashMap<InstanceType, HashMap<Entity, Vec<CollisionDessert>>>,
    instanceType: InstanceType,
    id: Entity,
    dessert: CollisionDessert,
) {
    if let Some(curEventMap) = collisionEventMap.get_mut(&instanceType) {
        if let Some(curEventVec) = curEventMap.get_mut(&id) {
            curEventVec.push(dessert);
        } else {
            curEventMap.insert(id, vec![dessert]);
        }
    } else {
        let mut curEventMap = HashMap::new();
        collisionEventMap.insert(instanceType, curEventMap);
        if let Some(curEventMap) = collisionEventMap.get_mut(&instanceType) {
            if let Some(curEventVec) = curEventMap.get_mut(&id) {
                curEventVec.push(dessert);
            } else {
                curEventMap.insert(id, vec![dessert]);
            }
        }
    }
}
