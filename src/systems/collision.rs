use std::{
    borrow::BorrowMut,
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

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

#[derive(Component, Clone, Debug)]
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

unsafe impl Send for CollisionEventMap {
    //其中 X 保证永远不会在自身之外克隆 Rc，也不让结构的用户直接访问 Rc，也不将 Rc 存储在某个 thread_local 变量中。
}
pub struct CollisionEventMap {
    // pub map: Arc<Mutex<HashMap<InstanceType, HashMap<Entity, Vec<CollisionDessert>>>>>,
    pub map: HashMap<InstanceType, HashMap<Entity, Vec<CollisionDessert>>>,
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        // let map = Arc::new(Mutex::new(HashMap::new()));
        // app.insert_resource(CollisionEventMap { map: map.clone() });

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
    // let mut collisionEventMap = &mut collisionEventMapStruct.map;
    // let mut collisionEventMap=HashMap::new();

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
            }
        }
    }

    #[cfg(debug_assertions)]
    {
        debugTable.collisionCount = Some(dynBots.len());
    }

    let mut tree = broccoli::Tree::par_new(&mut dynBots);

    let mut collisionEventMap = tree.par_find_colliding_pairs_acc(
        HashMap::new(),
        |_| HashMap::new(),
        |a, b| {},
        |v, a, b| {
            let aBot = a.unpack_inner();
            let bBot = b.unpack_inner();
            add_collision_event(
                v,
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
            add_collision_event(
                v,
                aBot.instanceType,
                aBot.id,
                CollisionDessert {
                    id: bBot.id,
                    collisionType: bBot.collisionType,
                    instanceType: bBot.instanceType,
                    pos: bBot.pos,
                    width: bBot.width,
                    height: bBot.height,
                },
            );
        },
    );

    for i in AabbPin::new(staBots.as_mut_slice()).iter_mut() {
        tree.find_all_intersect_rect(i, |r, mut a| {
            let (rect, bot) = a.destruct_mut();
            add_collision_event(
                &mut collisionEventMap,
                bot.instanceType,
                bot.id,
                CollisionDessert {
                    id: bot.id,
                    collisionType: CollisionType::Static,
                    instanceType: InstanceType::Wall,
                    pos: Vec2::new(
                        rect.x.start + rect.x.end / 2.,
                        rect.y.start + rect.y.end / 2.,
                    ),
                    width: rect.x.end - rect.x.start,
                    height: rect.y.end - rect.y.start,
                },
            );
        })
    }

    // println!("collisionEventMap: {:?}", collisionEventMap);
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
