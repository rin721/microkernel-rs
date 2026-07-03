# 用户管理与权限管理插件注册流程 (User Auth Plugin)

本示例展示一个具体业务插件（用户与权限管理）是如何向内核注册，并通过**能力接口自动映射**来获取系统通用应用资源，彻底避免手动拉取的代码耦合问题。

## 1. 插件边界与职责

*   **职责**：处理用户的注册、登录，以及基于 RBAC/ABAC 等策略的权限鉴定。
*   **依赖**：依赖底层数据库应用和可能的缓存应用。必须使用依赖注入宏自动装配，不可直接访问内核注册表。
*   **提供服务**：向其他插件或 HTTP 网关层暴露 `AuthService` Trait。

## 2. 注册与装配流程 (自动映射式设计)

### 2.1 声明式依赖注入 (Load & 映射阶段)
系统采用基于宏的声明式注入（或者 `Context` 自动装配），插件无需使用 `kernel.get_service()`。在插件结构体定义时，直接通过 `#[inject]` 属性标记该字段为一个依赖，Host Proxy 会在实例化插件本体时**自动映射**并注入该能力。

```rust
use std::sync::Arc;
use crate::kernel::Plugin;
use crate::apps::database::DatabaseConnection;
use crate::apps::cache::CacheClient;

/// 用户权限插件本体
#[derive(Plugin)]
pub struct UserAuthPlugin {
    // 自动映射：内核识别到此字段，自动从依赖池注入通用数据库库的能力接口
    #[inject]
    db: Arc<dyn DatabaseConnection>,
    
    // 自动映射：自动注入缓存库能力接口
    #[inject]
    cache: Arc<dyn CacheClient>,
}
```
*如果微内核在组装依赖树时发现该插件所需的能力接口并未就绪或缺失，将直接在启动阶段（Load）抛出错误并阻止应用启动。*

### 2.2 启动过程 (Start 阶段)
当所有自动映射的接口都已被成功注入后，内核调用插件的 `start()` 方法：
1.  基于已注入的 `db` 和 `cache`，构建插件内部的具体服务实例。
2.  向事件总线订阅来自系统其他组件的生命周期事件（如 `RoleUpdated`）。
3.  向微内核发布自身的能力接口，将其对外的服务包装为 `Arc<dyn AuthService>` 供其他插件以相同的方式“自动映射”使用。

### 2.3 暴露接口与能力映射
正如当前插件自动获取 `DatabaseConnection` 一样，其他需要鉴权能力的插件（如订单服务、网关）也会在自身结构体中使用自动映射来获取 `AuthService`：

```rust
// 其他插件的代码示例：无须主动查询内核，只需在自己的结构体中声明 #[inject]
#[derive(Plugin)]
pub struct OrderPlugin {
    #[inject]
    auth_service: Arc<dyn AuthService>, 
}

impl OrderPlugin {
    async fn process_order(&self, user_id: u64) {
        // 直接使用已经被自动映射好的 auth_service，完全不感知 kernel 的存在
        if self.auth_service.check_permission(user_id, "create_order").await {
            // ...
        }
    }
}
```

### 2.4 资源销毁 (Destroy 阶段)
在系统终止时，内核调用 `destroy()` 方法：
*   取消订阅事件总线。
*   丢弃持有的所有注入引用 (`Arc` 计数减 1)。
*   数据库和缓存实例自身的销毁动作由系统本体基于通用库生命周期统一指挥（遵循 `pre_stop`、`post_stop`），插件无需操心底层通用库的生命周期。
