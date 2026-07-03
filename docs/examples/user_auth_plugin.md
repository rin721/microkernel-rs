# 用户管理与权限管理插件注册流程 (User Auth Plugin)

本示例展示一个具体业务插件（用户与权限管理）是如何向内核注册并运行的。

## 1. 插件边界与职责

*   **职责**：处理用户的注册、登录，以及基于 RBAC/ABAC 等策略的权限鉴定。
*   **依赖**：依赖底层数据库应用（如 PostgreSQL，参见 [数据库应用示例](./database_app.md)）和可能的缓存应用（如 Redis）。
*   **提供服务**：向其他插件或 HTTP 网关层暴露 `AuthService` Trait 进行调用。

## 2. 注册与装配流程

### 2.1 依赖声明 (Load 阶段)
在插件加载阶段，`UserAuthPlugin` 向内核（Host Proxy）声明其依赖：
*   需要一个实现了 `DatabaseConnection` 的实例。
*   需要一个实现了 `CacheClient` 的实例。

如果微内核在组装依赖树时发现缺失依赖，将直接报错并停止启动。

### 2.2 启动过程 (Start 阶段)
当所有依赖都已被成功注入后，内核调用插件的 `start()` 方法：
1.  构建并启动内部的服务实例（比如 `UserAuthServiceImpl`）。
2.  向事件总线（Event Bus）订阅 `UserCreated` 或 `RoleUpdated` 等可能来自其他插件的事件。
3.  将其对外的接口实现包装为 `Arc<dyn AuthService>`，并注册回内核的插件服务表（Service Registry）。

### 2.3 暴露接口与通信
其他插件可以通过微内核获取到该接口并调用：
```rust
// 伪代码：其他插件中调用
let auth_service = kernel.get_service::<dyn AuthService>().await;
let has_permission = auth_service.check_permission(user_id, "read_data").await;
```

### 2.4 资源销毁 (Destroy 阶段)
在系统终止时，内核调用 `destroy()` 方法：
*   取消订阅事件总线。
*   等待进行中的鉴权请求执行完毕。
*   释放所持有的所有内存结构。由于数据库和缓存客户端的生命周期由对应的通用库应用控制，插件自身只需确保丢弃 (Drop) 这些引用即可。
