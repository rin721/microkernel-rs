# 代码目录架构设计 (Codebase Directory Structure)

根据“**避免单文件代码积压膨胀，拆分层次设计，拒绝同级化封装**”的核心原则，本微内核架构的 Rust 源码目录树应该呈现严格的分层隔离结构。

## 整体目录分层图

```text
src/
├── domain/                  # 1. 领域与公共契约层 (无状态，零依赖)
│   ├── error/               # 结构化错误定义
│   └── models/              # 全局共享领域模型
│
├── kernel/                  # 2. 微内核核心层 (Host Proxy)
│   ├── config/              # 动态配置与分发
│   ├── lifecycle/           # 生命周期调度引擎
│   ├── registry/            # 依赖注入容器与插件服务注册表
│   └── event_bus/           # 异步事件总线 (Pub/Sub)
│
├── apps/                    # 3. 通用库/应用组件层 (Generic Apps)
│   ├── database/            # 数据库应用模块 (或独立 Crate)
│   │   ├── config.rs        # 默认配置接口实现
│   │   ├── hooks.rs         # 挂载/生命周期钩子实现
│   │   └── client.rs        # 数据库客户端核心逻辑
│   └── cache/               # 其他通用基础设施...
│
├── plugins/                 # 4. 业务插件层 (Business Plugins)
│   ├── user_auth/           # 用户管理与权限插件
│   │   ├── api/             # 暴露的通信 Trait (如 AuthService)
│   │   ├── service/         # 插件核心鉴权业务
│   │   └── registry.rs      # 声明依赖并向内核注册、订阅事件
│   └── [other_plugin]/      # 其它隔离的业务插件
│
└── main.rs                  # 5. 系统组装入口 (仅负责实例化内核并挂载各模块)
```

## 设计原则

1. **内聚与层次分明**：
   * `main.rs` 不应包含任何业务逻辑，只做纯粹的组装。
   * 每个大模块（如 `kernel`, `database`, `user_auth`）必须拆分成子文件夹进行管理，使用 `mod.rs`（或与文件夹同名的文件如 `kernel.rs`）对外暴露有限的公开接口（Pub APIs）。
2. **防腐与解耦**：
   * `plugins` 下的各个子目录之间**严禁**发生直接的代码层级引用。
   * 插件与插件之间，必须通过向 `kernel/registry` 申请对方暴露的 `Arc<dyn Trait>`，或者通过 `kernel/event_bus` 进行解耦通信。
3. **避免单文件膨胀**：
   * 单一 `.rs` 文件如果超过合理长度（如 300~500 行），必须将内部逻辑进一步按功能职责拆分成多个子模块（如将复杂的配置解析单独剥离到 `config.rs`，钩子实现剥离到 `hooks.rs`）。
