# 用户管理与权限管理插件注册流程 (User Auth Plugin)

本示例展示一个具体业务插件（用户与权限管理）是如何通过**静态环境绑定**来获取系统通用应用资源，彻底实现零成本抽象和编译期类型安全的解耦设计。

## 1. 插件边界与职责

*   **职责**：处理用户的注册、登录，以及基于 RBAC/ABAC 等策略的权限鉴定。
*   **依赖**：依赖底层数据库应用和可能的缓存应用。必须通过泛型声明环境约束（Environment Trait）获取，不可直接访问或假设底层具体实现。
*   **提供服务**：向其他插件或 HTTP 网关层暴露权限校验能力。

## 2. 注册与装配流程 (静态环境约束)

### 2.1 泛型环境绑定 (Load & 约束阶段)
系统采用基于关联类型和环境 Trait 的静态决议。插件结构体定义时带上环境泛型 `<E: SystemEnv>`，内核在实例化时传入具体的环境对象。

```rust
use crate::kernel::{Plugin, SystemEnv};

/// 用户权限插件，受到 SystemEnv 环境约束
#[derive(Plugin)]
pub struct UserAuthPlugin<E: SystemEnv> {
    // 插件持有对当前环境上下文的引用或拥有权
    env: E,
}

impl<E: SystemEnv> UserAuthPlugin<E> {
    pub fn new(env: E) -> Self {
        Self { env }
    }
}
```
*如果环境对象 `E` 无法满足 `SystemEnv` 中定义的所有关联类型（如数据库、缓存接口缺失），编译将直接失败，做到绝对的安全隔离。*

### 2.2 启动过程 (Start 阶段)
在应用的 `pre_start` 阶段，内核会调度插件启动逻辑：
1.  基于 `self.env` 中的 `db()` 和 `cache()` 获取连接进行预热。
2.  向事件总线订阅来自系统其他组件的生命周期事件（如 `RoleUpdated`）。

### 2.3 暴露接口与能力调用
当其他需要鉴权能力的插件（如订单服务）需要鉴权时，它们同样也是绑定在 `<E: SystemEnv>` 上。由于环境是全局约束，各插件可以直接在编译期静态调用所需能力，或通过环境传递组合服务：

```rust
// 其他插件代码示例
#[derive(Plugin)]
pub struct OrderPlugin<E: SystemEnv> {
    env: E,
}

impl<E: SystemEnv> OrderPlugin<E> {
    async fn process_order(&self, user_id: u64) {
        // 直接通过环境或具体的 auth 实例进行静态调用，无虚函数表开销
        // 此处假设 AuthService 是注册在环境上下文中的能力
        if self.env.auth().check_permission(user_id, "create_order").await {
            // ...
        }
    }
}
```

### 2.4 资源销毁 (Destroy 阶段)
在系统终止时，内核调用 `destroy()` 钩子：
*   取消订阅事件总线。
*   数据库和缓存实例自身的销毁动作由系统本体基于通用库生命周期统一指挥（遵循 `pre_stop`、`post_stop`），插件无需操心底层通用库的生命周期。
