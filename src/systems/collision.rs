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
use rand::{thread_rng, Rng};

use crate::instance::{
    CollisionType, CollisionTypeValue, InstanceCamp, InstanceCampValue, InstanceType,
    InstanceTypeValue,
};
use broccoli::{
    axgeom::Rect,
    tree::{
        aabb_pin::AabbPin,
        bbox,
        node::{BBox, Num},
        rect,
    },
};
use dashmap::DashMap;

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

// 生产因子一般来源于需求，也就是消费者

pub enum CollisionShapeType {
    Rect,
}

#[derive(Component, Clone)]
pub struct CollisionInput {
    pub exclude: Option<fn(&InstanceType, &CollisionType, &InstanceCamp) -> bool>,
    pub receiveId: Entity,
    pub shape: CollisionShape,
}

#[derive(Component, Debug)]
pub struct CollisionResultArr {
    pub arr: Vec<CollisionResultItem>,
}
#[derive(Debug)]
pub struct CollisionResultItem {
    id: Entity,
    collisionType: CollisionType,
    instanceType: InstanceType,
    instanceCamp: InstanceCamp,
    shape: CollisionShape,
}

#[derive(Debug, Clone)]
pub struct CollisionShape {
    pub widthHalf: f32,
    pub heightHalf: f32,
    pub pos: Vec2,
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        // let map = Arc::new(Mutex::new(HashMap::new()));
        // app.insert_resource(CollisionEventMap { map: map.clone() });

        // app.insert_resource(Msaa { samples: 4 })
        //     // .add_event::<CollisionScopeEvent>()
        //     .add_plugin(ShapePlugin)
        //     .add_startup_system(startup);

        app.add_stage_before(CoreStage::Update, "collision", SystemStage::parallel())
            .add_system_to_stage("collision", collision_step.exclusive_system());
    }
}

fn startup(mut commands: Commands) {}

// 需要在接受输入同步新位置后调用
// query拿出来的值需要回收的，你不能把他的子属性取出来，因为他也不知道你什么时候回收.
// 整体移入是所有权的转移  子属性移入可以是copy 或者 就是不能
// 感觉哈，只需要这里告诉我他们是否碰撞了, 然后他们自己在自己的step里面处理自己的碰撞就好了
// snake_step 清空自己的碰撞事件(消费)  collision_step 接受碰撞(生产)
// 1.我觉得应该在自己的step中同步自己的碰撞因子
pub fn collision_step(world: &mut World) {
    let mut debugTable = world.get_resource_mut::<DebugTable>();
    let mut query = world.query::<(
        Entity,
        &GlobalTransform,
        &InstanceTypeValue,
        &CollisionTypeValue,
        &InstanceCampValue,
        &mut CollisionInput,
        &mut CollisionResultArr,
    )>();
    let mut dynBots = vec![];

    for (
        entity,
        globalTransform,
        instanceType,
        collisionType,
        instanceCamp,
        mut collisionInput,
        mut collisionResultArr,
    ) in query.iter_mut(world)
    {
        collisionInput.shape.pos = globalTransform.translation.xy();
        let shape = &collisionInput.shape;
        let rect = Rect::new(
            shape.pos.x - shape.widthHalf,
            shape.pos.x + shape.widthHalf,
            shape.pos.y - shape.heightHalf,
            shape.pos.y + shape.heightHalf,
        );
        dynBots.push(bbox(
            rect,
            (
                entity,
                instanceType,
                collisionType,
                instanceCamp,
                collisionInput,
                collisionResultArr,
            ),
        ));
    }

    // 用于构建树
    // let mut queryPart1 = world.query::<(&GlobalTransform, &mut CollisionInput)>();

    let mut tree = broccoli::Tree::par_new(&mut dynBots);

    tree.par_find_colliding_pairs(|a, b| {
        let aBot = a.unpack_inner();
        let bBot = b.unpack_inner();
        let (
            aEntity,
            aInstanceTypeValue,
            aCollisionTypeValue,
            aInstanceCampValue,
            aCollisionInput,
            aCollisionResultArr,
        ) = aBot;

        let (
            bEntity,
            bInstanceTypeValue,
            bCollisionTypeValue,
            bInstanceCampValue,
            bCollisionInput,
            bCollisionResultArr,
        ) = bBot;

        // aCollisionResultArr.arr.push(CollisionResultItem {
        //     shape: bCollisionInput.shape.clone(),
        //     id: bEntity.clone(),
        //     collisionType: bCollisionTypeValue.value.clone(),
        //     instanceType: bInstanceTypeValue.value.clone(),
        //     instanceCamp: bInstanceCampValue.value.clone(),
        // });

        // aCollisionResultArr.arr.push(CollisionResultItem {
        //     shape: aCollisionInput.shape.clone(),
        //     id: aEntity.clone(),
        //     collisionType: aCollisionTypeValue.value.clone(),
        //     instanceType: aInstanceTypeValue.value.clone(),
        //     instanceCamp: aInstanceCampValue.value.clone(),
        // });
    });
    // let mut setResultQuery = world.query::<(&mut CollisionResultArr)>();
    // for (
    //     entity,
    //     globalTransform,
    //     instanceType,
    //     collisionType,
    //     InstanceCamp,
    //     collisionInput,
    //     mut collisionResult,
    // ) in query.iter_mut(&mut world)
    // {}
    // let mut newCollisionProductionFactor = collisionProductionFactor.clone();
    // newCollisionProductionFactor.pos = globalTransform.translation.xy();

    // mut query: Query<(&GlobalTransform, &CollisionProductionFactor)>,
    // mut debugTable: ResMut<DebugTable>,
    // mut collisionEventMapStruct: ResMut<CollisionEventMap>,
    // let mut collisionEventMap = &mut collisionEventMapStruct.map;
    // let mut collisionEventMap=HashMap::new();

    // let mut staBots: Vec<_> = vec![];

    // let mut dynBots: Vec<_> = vec![];
    // for (globalTransform, collisionProductionFactor) in query.iter() {
    //     let mut configWidth = collisionProductionFactor.width;
    //     let mut configHeight = collisionProductionFactor.height;
    //     let mut rect = Rect::new(
    //         globalTransform.translation.x - configWidth / 2.,
    //         globalTransform.translation.x + configWidth / 2.,
    //         globalTransform.translation.y - configHeight / 2.,
    //         globalTransform.translation.y + configHeight / 2.,
    //     );
    //     let collisionType = collisionProductionFactor.collisionType;
    //     match collisionType {
    //         CollisionType::Static => {
    //             staBots.push(rect);
    //         }
    //         _ => {
    //             let mut newCollisionProductionFactor = collisionProductionFactor.clone();
    //             newCollisionProductionFactor.pos = globalTransform.translation.xy();
    //             dynBots.push(bbox(rect, newCollisionProductionFactor));
    //         }
    //     }
    // }

    // #[cfg(debug_assertions)]
    // {
    //     debugTable.collisionCount = Some(dynBots.len());
    // }

    // let mut tree = broccoli::Tree::par_new(&mut dynBots);
    // // let map: Arc<DashMap<String, String>> = Arc::new(DashMap::new())
    // let map: DashMap<u16, String> = DashMap::new();
    // unsafe {
    //     // let counter = Arc::new(Mutex::new(1));
    //     tree.par_find_colliding_pairs(|a, b| {
    //         // let newMap = Arc::clone(&map);
    //         // println!("counter:{:?}", counter);
    //         // let newCounter = Arc::clone(&counter);
    //         // let mut newCounter = counter.lock().unwrap();
    //         // *newCounter += 1;
    //         let mut rng = thread_rng();

    //         map.insert(rng.gen::<u16>(), "hello".to_string());
    //     });
    // }

    // let mut collisionEventMap = tree.par_find_colliding_pairs_acc(
    //     HashMap::new(),
    //     |_| _,
    //     |a, b| {},
    //     |v, a, b| {
    //         let aBot = a.unpack_inner();
    //         let bBot = b.unpack_inner();
    //         add_collision_event(
    //             v,
    //             bBot.instanceType,
    //             bBot.id,
    //             CollisionDessert {
    //                 id: aBot.id,
    //                 collisionType: aBot.collisionType,
    //                 instanceType: aBot.instanceType,
    //                 pos: aBot.pos,
    //                 width: aBot.width,
    //                 height: aBot.height,
    //             },
    //         );
    //         add_collision_event(
    //             v,
    //             aBot.instanceType,
    //             aBot.id,
    //             CollisionDessert {
    //                 id: bBot.id,
    //                 collisionType: bBot.collisionType,
    //                 instanceType: bBot.instanceType,
    //                 pos: bBot.pos,
    //                 width: bBot.width,
    //                 height: bBot.height,
    //             },
    //         );
    //     },
    // );

    // for i in AabbPin::new(staBots.as_mut_slice()).iter_mut() {
    //     tree.find_all_intersect_rect(i, |r, mut a| {
    //         let (rect, bot) = a.destruct_mut();
    //         add_collision_event(
    //             &mut collisionEventMap,
    //             bot.instanceType,
    //             bot.id,
    //             CollisionDessert {
    //                 id: bot.id,
    //                 collisionType: CollisionType::Static,
    //                 instanceType: InstanceType::Wall,
    //                 pos: Vec2::new(
    //                     rect.x.start + rect.x.end / 2.,
    //                     rect.y.start + rect.y.end / 2.,
    //                 ),
    //                 width: rect.x.end - rect.x.start,
    //                 height: rect.y.end - rect.y.start,
    //             },
    //         );
    //     })
    // }

    // println!("count: {:?}", count);
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

// fn add_collision_event(
//     collisionEventMap: &mut HashMap<InstanceType, HashMap<Entity, Vec<CollisionDessert>>>,
//     instanceType: InstanceType,
//     id: Entity,
//     dessert: CollisionDessert,
// ) {
//     if let Some(curEventMap) = collisionEventMap.get_mut(&instanceType) {
//         if let Some(curEventVec) = curEventMap.get_mut(&id) {
//             curEventVec.push(dessert);
//         } else {
//             curEventMap.insert(id, vec![dessert]);
//         }
//     } else {
//         let mut curEventMap = HashMap::new();
//         collisionEventMap.insert(instanceType, curEventMap);
//         if let Some(curEventMap) = collisionEventMap.get_mut(&instanceType) {
//             if let Some(curEventVec) = curEventMap.get_mut(&id) {
//                 curEventVec.push(dessert);
//             } else {
//                 curEventMap.insert(id, vec![dessert]);
//             }
//         }
//     }
// }
