# AGENTS.md — 代码代理协作规范

本文档是本仓库所有代码代理的**高密度薄入口**。目的是消灭幻觉、阻止夹带、保证 PR/commit 可复现。细则在 [`docs/ai/agents/`](docs/ai/agents/) 与 `.agents/skills/`；**不要默认把所有分章读进上下文**。

规范骨架来自 [rust-agentic-sekleton](https://github.com/EeroEternal/rust-agentic-sekleton)；产品边界见 [`docs/architecture/product-positioning.md`](docs/architecture/product-positioning.md)。

## 元知识分层与预算纪律

| 层级 | 放什么 | 入口 |
| --- | --- | --- |
| 站立约束 | 跨任务始终成立的禁止项 | 本文件「始终生效」；展开见 [`docs/ai/agents/`](docs/ai/agents/) |
| 可复用流程 | 领域流程与验证命令 | `.agents/skills/*/SKILL.md`（唯一权威） |
| 领域规范 | 产品定位 / UI / 模块边界 | [`docs/architecture.md`](docs/architecture.md)；[`docs/design.md`](docs/design.md) |

- **Token 预算**：本文件硬性上限 **80 行 / 1200 Tokens**；接近上限时**零和置换**。
- **反轶事**：新全局规则须 **≥ 2 个独立会话**重复出现，经 skill [`promote-lesson`](.agents/skills/promote-lesson/SKILL.md) 提炼并由人类审核。

## 按需阅读地图

| 任务信号 | 必读入口 |
| --- | --- |
| 产品定位 / 与 Xrouter 连接 / OTEL | [`product-positioning.md`](docs/architecture/product-positioning.md) |
| 优化方案 / 对外接口延续 | [`optimization-plan.md`](docs/architecture/optimization-plan.md) |
| 抽 crate / 跨模块 SQL join / 领域字段 | [`module-boundaries.md`](docs/architecture/module-boundaries.md) |
| 前端 UI / i18n | skill [`admin-ui-change`](.agents/skills/admin-ui-change/SKILL.md) → [`docs/design.md`](docs/design.md) |
| 新增 SQL 迁移 | skill [`add-sql-migration`](.agents/skills/add-sql-migration/SKILL.md) |
| `git stash` | skill [`git-stash-safe`](.agents/skills/git-stash-safe/SKILL.md) |
| 发版 / 打 tag | skill [`release`](.agents/skills/release/SKILL.md) |
| 代码评审 | skill [`review`](.agents/skills/review/SKILL.md) |
| Push 前门禁 | skill [`pre-push-local-gates`](.agents/skills/pre-push-local-gates/SKILL.md) |
| `tokio::spawn` / 夹带 / 管道退出码 | [`engineering.md`](docs/ai/agents/engineering.md) |
| Commit message | [`commit-style.md`](docs/ai/agents/commit-style.md) |
| 例行项目快照 | [`docs/project_status.md`](docs/project_status.md) |

## 始终生效

1. **禁止夹带**：commit/PR 不得夹带无关改动；整库 fmt、无说明的 `#[allow]`、跨模块顺手 refactor 违规则 `git reset --mixed HEAD~1`。
2. **禁止幻觉代码**：有定义必有调用；cache 必有 store；metric 正反双向；`TODO` 必挂 issue。设计文档不得把骨架当既有能力。
3. **安全 stash**：诚实命名；stash 前 `git diff --stat`；之后 `cargo check --tests`；严禁 stash `Cargo.toml` / `Cargo.lock` / 构建脚本。
4. **发布护栏**：本地闭环后向用户说明；**未经明确批准，严禁合并 main 或打 Tag**。
5. **UI 弹窗**：`max-h-[85vh]` + `overflow-y-auto`；长内容用折叠或标签；全局配置只进设置页。
6. **OTEL 优先、核心保持通用**：ingest 主入口是 OTLP；领域语义用 `gen_ai.*` / 约定 attributes，**禁止**为 Xrouter / Xinference 在核心 schema 加专属列或专属表。
7. **不嵌回生产者**：xtrace 是独立可观测产品，不得要求把追踪引擎编进 Xrouter 进程；连接只靠 `traceparent` + `xrouter.request_id`（见定位文档）。
8. **Push 前本地门禁**：禁止把 CI 当沙盒；按 [`pre-push-local-gates`](.agents/skills/pre-push-local-gates/SKILL.md) 全绿再推。

## Skills 索引

`.agents/skills/`：[`git-stash-safe`](.agents/skills/git-stash-safe/SKILL.md) · [`add-sql-migration`](.agents/skills/add-sql-migration/SKILL.md) · [`promote-lesson`](.agents/skills/promote-lesson/SKILL.md) · [`admin-ui-change`](.agents/skills/admin-ui-change/SKILL.md) · [`verify-design-doc`](.agents/skills/verify-design-doc/SKILL.md) · [`pre-push-local-gates`](.agents/skills/pre-push-local-gates/SKILL.md) · [`release`](.agents/skills/release/SKILL.md) · [`review`](.agents/skills/review/SKILL.md)
