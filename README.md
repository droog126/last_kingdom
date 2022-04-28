
# 进度
1. 输入系统
2. 动画系统
3. 移动
4. 相机跟随  相机debug


# todo
5. 地图编辑(等待1.0支持)

已经支持了，下一个todo

4+9 动态地图加载



6. 碰撞
https://github.com/jcornaz/impacted

6.1 把碰撞形状实装父实体
6.2 把碰撞形状放进aabbs 输出碰撞事件 反馈给父实体

进度:
研究了下🥦碰撞库的效率，自己实装进去，没有优化的话，支持4w实例同时检测碰撞, 有75帧左右
下一把:
做下优化，看下是否能提升这个性能
目前  7.7w碰撞  60帧左右

动动碰撞完成了


下一步，静动碰撞

静态标签区分
静态实例生产
    随机宽高迭代器 
    实装位移逻辑
    
静态动态系统  可以附加在动动碰撞上

初步系统已经建成

7. 怪物
8. pvp
9. 联网
10. debug 改成命令行

### 实验室


1. g7的转点绳索
2. z轴

### 记录
bevy_prototype_lyon 画图性能很烂




# 启动
cargo run
cargo run --features bevy/trace_chrome


# debug

1.相机解锁 f3 DebugStatus.camera_debug
2.fps显示 f11 DebugStatus.fps_show
3.碰撞体积显示 f12   DebugStatus.collision_debug



