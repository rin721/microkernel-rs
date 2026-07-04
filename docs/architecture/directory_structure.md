# 代码目录架构设计 (Codebase Directory Structure)

根据“**绝对防腐、领域折叠、微观细化**”的核心原则，本开源 Admin 后端架构强制采用 **Cargo Workspaces 多包工作空间** 结合 **微模块极致拆分** 的双层防膨胀设计。

## 1. 宏观层：基于领域折叠的 Cargo Workspace

为彻底阻断同级目录越权调用底层库的可能（底层依赖绝对隔离），整个项目必须在根目录下组织为 `Cargo Workspaces`。严禁将所有应用和插件平铺在同一个目录下，必须按照“领域 (Domain)”进行深层嵌套切分。

```text
/ (Workspace Root)
├── Cargo.toml                      # 统一声明 members = ["kernel/*", "systems/*/*", "host"]
│
├── kernel/                         # 【系统底座区】
│   ├── core/                       # 核心微内核 (Host Proxy)，包含引擎、生命周期流转
│   ├── contracts/                  # 纯净零依赖的 Trait 抽象定义与全局错误码
│   └── macros/                     # 存放 #[derive(Plugin)] 等自研过程宏
│
├── systems/                        # 【业务与基建领域区】(严禁平铺，必须按域折叠)
│   ├── data_plane/                 # ── 数据面组
│   │   ├── database_app/           # 封装 sqlx/sea-orm 的通用库
│   │   ├── cache_app/              # 封装 redis 的通用库
│   │   └── storage_app/            # 封装 opendal 的通用库
│   │
│   ├── identity_access/            # ── 身份与访问权限组
│   │   ├── rbac_plugin/            # 具体鉴权业务插件
│   │   ├── mfa_plugin/             # 双因子认证业务插件
│   │   └── auth_plugin/            # 登录签发插件
│   │
│   └── observability/              # ── 可观测性组
│       ├── logger_app/             # 日志基建应用
│       └── metrics_plugin/         # 指标抓取业务插件
│
└── host/                           # 【宿主挂载区】
    ├── Cargo.toml                  # 负责将所有需要启用的子 crate 引入
    └── src/main.rs                 # 整个系统的唯一入口
```

## 2. 微观层：单包内部 (Intra-Crate) 极限拆解规范

为解决工程级项目中单包内部文件迅速膨胀（动辄上千行）的通病，**任何子包内部都不允许将逻辑堆砌在 `lib.rs` 中**。必须遵循以下的子域切分模板：

### 2.1 微内核本体 (`kernel/core`) 规范骨架
```text
kernel/core/src/
├── lib.rs                  # 枢纽：仅含 pub use 导出，禁写业务实现
├── config/                 # 配置子域
│   ├── loader.rs           # 仅负责从文件/系统环境读取
│   └── parser.rs           # 仅负责反序列化映射
├── registry/               # 注入枢纽子域
│   ├── container.rs        # 仅负责管理依赖池的存取状态
│   └── injector.rs         # 仅负责执行自动映射和注入运算
├── lifecycle/              # 流程编排子域
│   ├── bootstrap.rs        # 仅负责启动阶段 (Load -> Start) 的串行逻辑
│   └── teardown.rs         # 仅负责销毁阶段 (Destroy) 的释放逻辑
└── event_bus/              # 总线子域
    └── dispatcher.rs       # 仅负责分发通道消息
```

### 2.2 领域契约 (`kernel/contracts`) 规范骨架
```text
kernel/contracts/src/
├── lib.rs                  
├── lifecycle/              
│   ├── app_hook.rs         # 细化：集中定义 Archetype 的 8 大钩子 Trait
│   └── plugin_hook.rs      # 细化：定义 Plugin 的生命周期 Trait
└── errors/                 
    ├── app_error.rs        # 规范化：通用应用抛出的统一 Enum
    └── kernel_error.rs     # 规范化：内核崩溃/注入失败的统一 Enum
```

### 2.3 基础设施通用应用 (以 `storage_app` 为例) 规范骨架
```text
systems/data_plane/storage_app/src/
├── lib.rs
├── config/                 
│   └── options.rs          # 细化：云存储密钥、Bucket 配置结构体
├── lifecycle/              # 核心：必须按钩子拆分文件！
│   ├── pre_create.rs       # 细化：仅写配置连接校验
│   ├── post_mount.rs       # 细化：仅写挂载报告
│   └── pre_stop.rs         # 细化：仅写强制中断未决流的操作
├── provider/               # 隔离的底层调用层
│   ├── s3_impl.rs          # 细化：完全隔离 AWS S3 的特定 SDK 逻辑
│   └── local_impl.rs       # 细化：本地 FS 的特定逻辑
└── error/                  
    └── storage_error.rs    
```

### 2.4 业务插件 (以 `rbac_plugin` 为例) 规范骨架
```text
systems/identity_access/rbac_plugin/src/
├── lib.rs
├── plugin.rs               # 细化：使用 #[derive(Plugin)]，并书写 #[inject] 获取依赖
├── api/                    
│   └── rbac_service.rs     # 细化：定义向其他插件暴露的检查权限 Trait 契约
├── core/                   # 核心算法
│   ├── policy_eval.rs      # 细化：单一职责，只算策略是否通过，不碰 IO
│   └── role_mgr.rs         # 细化：管理角色组的增删改查
└── handlers/               
    └── event_handler.rs    # 细化：单文件专门负责处理 EventBus 发来的异步广播
```

## 3. 防膨胀开发红线 (Developer Guidelines)

1.  **枢纽禁飞区**：`src/lib.rs` 或 `mod.rs` 仅作为命名空间的导出索引，**绝对禁止**在其中编写 `impl` 方法体或具体业务逻辑。
2.  **代码行数警戒线**：单一功能函数超过 50 行，或单一 `.rs` 源文件超过 **300 行** 时，视为极度危险的代码异味，强制要求触发目录裂变，拆分出新的子域文件夹进行责任分摊。
3.  **绝对底层隔离**：除 Generic Apps 自身的 `Cargo.toml` 可引入底层驱动外，其余任何模块均只能依赖 `kernel/contracts`，实现物理层面的防腐解耦。
