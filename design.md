> 在AGENTS.md规则中首要概述中，新增需要通过 `docs/ai/.rules.md` 仓库项目规则来确定 `规则/架构/流程`（真相来源、阅读顺序、边界、工作规则）

## 优先生成真实来源

1. 先把项目 `设计/需求` 文档拆分到 `/docs` 中
2. 在 `设计/需求` 文档可能需要耗费很长一段时间，请你在生成 `设计/需求` 文档时，保持可观的目录结构，方便我/ai agent更快的阅读理解并通过ai agent编写代码
3. 在编写代码时，需要为当前项目实现graph并在 `docs/ai/.rules.md` 标记说明graph在哪里以便辅助ai agent开发
4. 你需要在编写代码的过程中，在 `docs/ai/.rules.md` 中同步长期自进化

## 目标方向

> Rust后端微内核架构设计，请你帮我在 `/docs` 蒸馏一份 `设计/需求` 文档

## 核心设计
```
应用依赖组装注入（依赖组装 + 生命周期），需要为每个应用实现（配置重载、初始化、中间件、统一启动、停止和资源关闭、等等）

=>

插件（插件的生命周期管理（加载、启动、销毁） 和 插件间的通信/共享协议（总线或接口））。
```

## 目录结构

> 目录封装不要同级化设计，如果单文件代码量过多，就不适合阅读，应将目录结构拆分层次设计避免单文件积压膨胀

### 核心结构
```bash
src/
├── boot/
├── kernel/               ← 控制中心（不变）
├── engine/               ← 插件执行引擎（新增核心）
├── plugins/              ← 自动化插件库（重点）
├── blueprints/           ← 结构模板（生成依据）
├── runtime/
├── domain/
├── infra/
└── shared/
```

```bash
boot/
├── main.rs
└── bootstrap/
    ├── mod.rs
    ├── run.rs
    ├── args.rs
    └── env.rs
```

```bash
1_kernel/
├── mod.rs
│
├── core/
│   ├── kernel.rs              ← Kernel 主体（控制中心）
│   ├── context.rs             ← 全局上下文（DI容器）
│   ├── builder.rs             ← Kernel构建器（graph构建）
│   ├── registry.rs            ← 服务注册中心
│   └── graph.rs               ← ★依赖图（核心）
│
├── lifecycle/
│   ├── init/
│   ├── mount/
│   ├── start/
│   ├── run/
│   ├── reload/
│   └── shutdown/
│
├── config/
│   ├── loader.rs
│   ├── watcher.rs
│   ├── diff.rs
│   └── schema.rs
│
├── resource/
│   ├── manager.rs
│   ├── trait.rs
│   ├── db.rs
│   ├── cache.rs
│   └── cleanup.rs
│
├── middleware/
│   ├── chain.rs
│   ├── auth.rs
│   ├── tracing.rs
│   ├── recovery.rs
│   └── metrics.rs
│
└── plugin/
    ├── trait.rs              ← Plugin抽象（类似“继承基类”）
    ├── manager.rs            ← 插件生命周期控制器
    ├── registry.rs
    └── loader.rs
```

```bash
engine/
├── mod.rs
│
├── executor/
│   ├── plugin_executor.rs      ← 执行插件
│   ├── pipeline.rs             ← 执行流水线
│   └── scheduler.rs            ← 自动任务调度
│
├── context/
│   ├── engine_context.rs       ← 当前工程上下文
│   ├── state.rs                ← 状态机
│   └── workspace.rs           ← 项目结构感知
│
├── planner/
│   ├── task_planner.rs         ← 自动拆任务
│   ├── diff_analyzer.rs        ← 差异分析（关键）
│   └── dependency_solver.rs    ← 依赖推导
│
└── registry/
    ├── plugin_registry.rs
    └── capability_registry.rs
```

## 新增通用库 `数据库应用` 示例流程： 

1. 为当前系统封装实现数据库应用为通用库
2. 为通用库提供默认配置接口（只需实现接口，应用系统本体会通过 `启用字段` 判断是否自动代理调用，需定义启用字段判断如果未配置当前应用配置是否启用默认配置覆盖未配置字段） 
3. 为通用库提供应用实例创建前接口（只需实现接口，应用系统本体会调用此钩子自动代理完成） 
4. 为通用库提供应用实例创建后接口（只需实现接口，应用系统本体会调用此钩子自动代理完成） 
5. 为通用库提供应用实例挂载前接口（只需实现接口，应用系统本体会调用此钩子自动代理完成）
6. 为通用库提供应用实例挂载后接口（只需实现接口，应用系统本体会调用此钩子自动代理完成）
7. 为通用库提供应用实例重载后接口（只需实现接口，应用系统本体会调用此钩子自动代理完成，附带以下动作：如果系统配置管理数据发生变化且变化的数据是当前 `数据库应用` 配置数据）
8. 为通用库提供应用实例停止前接口（只需实现接口，应用系统本体会调用此钩子自动代理完成）
9. 为通用库提供应用实例停止后接口（只需实现接口，应用系统本体会调用此钩子自动代理完成）

## 新增 `用户管理+权限管理插件` 注册示例流程：

```bash
plugins/user-management/
├── api/             # 接口层
├── model/           # 数据模型 (如有)
├── router/          # 路由层
├── service/         # 业务逻辑层
├── global/          # 全局变量 (如有)
└── main.rs          # 插件入口（必须）
```

...
