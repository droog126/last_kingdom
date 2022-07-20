use crate::systems::debug::egui::DebugTable;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use broccoli::{
    axgeom::Rect,
    tree::{bbox, node::ManySwappable},
};

use super::iType::{
    CollisionType, CollisionTypeValue, InstanceCamp, InstanceCampValue, InstanceType, InstanceTypeValue,
};

#[derive(Component)]
pub struct CollisionID(pub Entity);

// 生产因子一般来源于需求，也就是消费者
pub enum CollisionShapeType {
    Rect,
}

#[derive(Component, Clone)]
pub struct CollisionInput {
    pub exclude: Option<CollisionExcludeFunction>,
    pub receiveId: Entity,
    pub shape: CollisionShape,
}

// 我只接受的对象
pub type CollisionExcludeFunction = fn(&InstanceType, &CollisionType, &InstanceCamp) -> bool;

#[derive(Component, Debug)]
pub struct CollisionResultArr {
    pub arr: Vec<CollisionResultItem>,
}
#[derive(Debug, Clone)]
pub struct CollisionResultItem {
    pub id: Entity,
    collisionType: CollisionType,
    instanceType: InstanceType,
    instanceCamp: InstanceCamp,
    pub shape: CollisionShape,
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
        // app.insert_resource(Msaa { samples: 4 })
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
        dynBots.push(ManySwappable((
            rect,
            (
                entity,
                instanceType,
                collisionType,
                instanceCamp,
                collisionInput,
                collisionResultArr,
            ),
        )));
    }

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

        if let Some(exclude) = aCollisionInput.exclude {
            if !exclude(
                &bInstanceTypeValue.value,
                &bCollisionTypeValue.value,
                &bInstanceCampValue.value,
            ) {
                aCollisionResultArr.arr.push(CollisionResultItem {
                    shape: bCollisionInput.shape.clone(),
                    id: bEntity.clone(),
                    collisionType: bCollisionTypeValue.value.clone(),
                    instanceType: bInstanceTypeValue.value.clone(),
                    instanceCamp: bInstanceCampValue.value.clone(),
                });
            }
        }

        if let Some(exclude) = bCollisionInput.exclude {
            if !exclude(
                &aInstanceTypeValue.value,
                &aCollisionTypeValue.value,
                &aInstanceCampValue.value,
            ) {
                bCollisionResultArr.arr.push(CollisionResultItem {
                    shape: aCollisionInput.shape.clone(),
                    id: aEntity.clone(),
                    collisionType: aCollisionTypeValue.value.clone(),
                    instanceType: aInstanceTypeValue.value.clone(),
                    instanceCamp: aInstanceCampValue.value.clone(),
                });
            }
        }
    });

    #[cfg(debug_assertions)]
    {
        let len = dynBots.len();
        let mut collisionTable = world.resource_mut::<DebugTable>();
        collisionTable.collisionCount = Some(len);
    }
}

pub fn _repel(aPos: &Vec2, bPos: &Vec2, _closest: Option<f32>, _mag: Option<f32>) -> Vec2 {
    let mut closest = _closest.unwrap_or(0.001);
    let mut mag = _mag.unwrap_or(1.);

    let diff = Vec2::new(aPos.x - bPos.x, aPos.y - bPos.y);
    let len_sqr = diff.length();
    if len_sqr < closest {
        return Vec2::splat(0.0);
    }
    let len = len_sqr.sqrt();
    let mag = mag / len;
    let force = diff.normalize() * Vec2::splat(mag);

    force
}

// 实体和墙碰撞  把碰撞的那边坐标传过去
//  for i in AabbPin::new(staBots.as_mut_slice()).iter_mut() {
//     tree.for_all_intersect_rect_mut(i, |r, mut a| {
//         let (rect, bot) = a.destruct_mut();

//         let wallx = &r.x;
//         let wally = &r.y;
//         let ret = match duckduckgeo::collide_with_rect(&rect, &r).unwrap() {
//             duckduckgeo::WallSide::Above => [Some(1.), Some(wally.start)],
//             duckduckgeo::WallSide::Below => [Some(3.), Some(wally.end)],
//             duckduckgeo::WallSide::LeftOf => [Some(4.), Some(wallx.start)],
//             duckduckgeo::WallSide::RightOf => [Some(2.), Some(wallx.end)],
//         };

//         match &mut bot.collisionInner {
//             CollisionInner::Instance {
//                 pos,
//                 force,
//                 wall_move,
//             } => {
//                 wall_move[0] = ret[0];
//                 wall_move[1] = ret[1];
//             }
//             _ => {}
//         }
//     })
// }
