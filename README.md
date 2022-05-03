
# 进度
1. 输入系统
2. 动画系统
3. 移动
4. 相机跟随  相机debug


# todo
5. 地图编辑(等待1.0支持)

已经支持了，下一个todo

5.2 人物需要增加脚下阴影

5.1 4+9 动态地图加载
坍塌函数也没有看
这个研究还是太早了点，等联机ok了再搞这个



6. 碰撞
https://github.com/jcornaz/impacted

6.1 把碰撞形状实装父实体
6.2 把碰撞形状放进aabbs 输出碰撞事件 反馈给父实体

进度:
研究了下🥦碰撞库的效率，自己实装进去，没有优化的话，支持4w实例同时检测碰撞, 有75帧左右
下一把:
做下优化，看下是否能提升这个性能
目前  12w碰撞  60帧左右

动动碰撞完成了
下一步，静动碰撞
静态标签区分
静态实例生产
    随机宽高迭代器 
    实装位移逻辑
    
静态动态系统  可以附加在动动碰撞上

初步系统已经建成

7. ui试点
https://github.com/mvlabat/bevy_egui.git
7.1 按M呼出菜单
7.2 优化文字显示(done)
7.3 碰撞实例个数显示(done)

8. 怪物
8.1 素材
骷髅射手吧


8.2 ai
1. 随机移动
2. 看到玩家进行攻击
3. 血量过低，逃跑






9. pvp
10. 联网
11. debug 改成命令行
12. 开发工具
12.1 地形编辑器

### 实验室


1. g7的转点绳索
2. z轴


3. 曲线闭合相交判定
数组去建立区间
或者用库
https://parry.rs/
这个可以

矩形 内包裹不规则图形进行碰撞检测

### 记录
bevy_prototype_lyon 画图性能很烂
bevy_ecs_ldtk= "0.3.0" 性能也有点烂


### 源代码现存问题 
1. 相机抖动(done)
解决办法:镜头移动快速点 小于一定值直接等于

### P0
1. ui

# 启动
cargo run
cargo run --features bevy/trace_chrome


# debug

1.相机解锁 f3 DebugStatus.camera_debug
2.fps显示 f11 DebugStatus.fps_show
3.碰撞体积显示 f12   DebugStatus.collision_debug
4.放置怪物  f10  DebugStatus.instance_debug



