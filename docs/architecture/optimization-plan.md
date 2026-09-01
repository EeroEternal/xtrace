# Xtrace 优化方案（报告层 + OTEL + 接口延续）

> **地位**：规划。未做的不得写成已有能力。
> 边界：[`competitive-boundary.md`](competitive-boundary.md)、[`product-positioning.md`](product-positioning.md)。
> 路由锚点：`src/app.rs` 的 `build_router`（2026-09 核对）。

目标：成为 **LLM 业务对账单**（到处收数 → 对上同一笔业务 → 出报告），同时 **不打断已经在用的接入方**（Xinference Langfuse 兼容、OTLP、`/v1/l/batch`、`/v1/metrics/batch`）。

## 0. 接口延续性（先于一切功能）

已有产品把 xtrace 当 Langfuse 后端或当 OTLP collector。优化 **默认只做加法**。

### 0.1 冻结契约（不得无版本地改）

| 路径 | 角色 | 已知用法 |
| --- | --- | --- |
| `POST /api/public/otel/v1/traces` | OTLP traces | 标准 OTEL SDK / collector |
| `POST /api/public/otel/v1/metrics` | OTLP metrics | 同上 |
| `POST /v1/l/batch` | Langfuse 风格 ingest | Xinference 等 |
| `POST /v1/metrics/batch` | 点指标 ingest | 自有客户端 / 网关侧脚本 |
| `GET /api/public/traces` | 列表（Langfuse 形） | Xinference 监控页、仪表板 |
| `GET /api/public/traces/{traceId}` | 详情（对象直接返回，无外层 data） | 同上 |
| `GET /api/public/metrics` | traces 概览兼容 | Xinference `/v1/l/metric/overview` |
| `GET /api/public/metrics/daily` | 日聚合 | 用量/成本 |
| `GET /api/public/metrics/query` | 时序 | 仪表板 |
| `GET /api/public/metrics/names` | 名称发现 | 自动完成 |
| `GET /api/public/projects` | 项目列表 | Langfuse 兼容 |
| `POST/GET/PATCH /api/public/media*` | 媒体 | Langfuse 兼容 |
| `GET /healthz` `GET /readyz` | 探针 | K8s / 托管探测 |
| Bearer / BasicAuth（`XTRACE_*` 与遗留 `LANGFUSE_*`） | 鉴权 | 已文档化 |

JSON 字段名、分页 `data`/`meta`、traces 详情无包裹、query 参数名（`sessionId`、`fromTimestamp` 等）均视为冻结。

### 0.2 允许的变更方式

| 变更 | 规则 |
| --- | --- |
| 新路径 | 可以，例如 `GET /api/public/reports` |
| 已有 GET 增加 **可选** query | 可以；缺省行为必须与今天完全一致 |
| 响应 **增加** 字段 | 可以；老客户端忽略未知字段 |
| 重命名/删除字段、改状态码语义、改默认分页 | **破坏性**，必须：changelog 专节、迁移指南、至少一轮 minor 预告；能双写则双写 |
| 鉴权变严 | 不得让现网 Xinference 突然 401；新策略用新 token 类型或 opt-in |
| 改 OTLP 映射 | 不得丢已接受的 Gauge/Sum/Histogram 派生点；新类型可跳过但要打日志（现状已如此） |

破坏性变更用新路径或明确 `Accept`/版本头，**不**默默改 `/api/public/traces`。没有存量客户迁移文档，禁止合入破坏性 PR。

### 0.3 给接入方的承诺

1. Langfuse 兼容面继续工作：换 `LANGFUSE_HOST` 的客户不因本方案改代码。
2. OTLP 路径继续工作：标准 protobuf/JSON/gzip 不改 URL。
3. 领域增强走 **metadata / OTEL attributes**（`session_id`、`xrouter.request_id`、`gen_ai.*`），不改顶层 Langfuse 字段名。
4. 若必须破坏：提前在 `docs/api.md` 写「破坏性变更」节，列出旧→新对照与停用时间。

## 1. 方案原则

1. Prometheus 仍看箱子；Xtrace 不出刮取器。
2. 报告是产品；仪表板是报告的一种视图。
3. 对账键：`trace_id` + `xrouter.request_id` + session/turn/run；metrics labels 禁止高基数 ID。
4. 成本口径：若有金额，用 **事件发生时的数**，不事后用现价重算。
5. Xrouter OTLP 出站在 **xrouter 仓库**，本仓库只收、只查、只报。

## 2. 分阶段

### P0 — 契约与文档（不改行为）

- 本文件 + `docs/api.md` 增加「兼容性」短节，链到 0.1 表。
- 属性字典（约定，非新列）：`gen_ai.request.model`、`gen_ai.usage.input_tokens`、`xrouter.request_id`、`xrouter.route`、`xrouter.provider`、`xrouter.attempt_index`、`xrouter.is_fallback`、`xrouter.queue_wait_ms`。
- 标明：网关侧 OTLP **尚未接线**。

**验收**：现有 `cargo test --workspace` 与 Xinference 换 HOST 路径零差异。

### P1 — 对账检索（只加可选参数）

在 `GET /api/public/traces` **增加可选**过滤，例如：

- `metadata.request_id` / `externalId`（若已有 `externalId` 则文档说明优先用哪个）
- 已有 `sessionId` 保持不变

缺省列表顺序、分页、字段集不变。UI：按 `request_id` 打开一条 trace。

**验收**：不带新参数的旧 curl 字节级语义不变（测黄金响应或契约测试）。

### P2 — 报表 API（全新路径）

新增，例如：

```text
GET /api/public/reports/usage?from=&to=&group_by=model,route
```

- 租户 = 当前 token 的 `project_id`（与现 ingest 隔离一致）
- `group_by` 白名单：`model`、`name`、低基数 label；禁止 `userId` 当时序维度除非单独分页明细
- 导出：`Accept: text/csv` 或 `?format=csv`
- **不**修改 `/api/public/metrics/daily` 的现有 JSON；日报可以内部复用查询，但对外路径分开，避免 Xinference 概览被改形

**验收**：旧 `metrics/daily` 与 `metrics` overview 测试仍绿；新测试只打 `/reports/*`。

### P3 — 与 Xrouter 套装（跨仓库）

Xrouter：可选 OTLP span（默认关），attributes 用 P0 字典。  
Xtrace：P1 能按 `xrouter.request_id` 找到 trace。  
两边 Admin 互跳可以后做。

**验收**：网关关 OTLP 时热路径无导出调用；打开后 xtrace 能按 request_id 查到。

### P4 — 报告产品化

- 定时任务 / 只读报表 token（**新** token 类型，不改变现有写 token 权限）
- 周报邮件可选
- 仪表板改走 reports API，旧 metrics 查询保留

### P5 — 后置

OTLP logs、再导出 OTLP、Agent 图、eval。不挡 P0–P3。

## 3. 明确不做（本方案周期内）

- 替代 Prometheus；scrape Xrouter `/metrics`
- 嵌回 Xrouter 进程；join 网关 Postgres
- Prompt CMS / 数据集 / 实验平台
- 把 `request_id` 打进 metric labels

## 4. 风险

| 风险 | 缓解 |
| --- | --- |
| 改 traces JSON 弄坏 Xinference | P1 只加可选 query；契约测试钉死无参响应形状 |
| 报表打爆高基数 | group_by 白名单 + series cap（沿用 metrics query 的 50 series / 1000 points） |
| 客户以为 OTEL 网关已通 | 文档写「规划」；README 不写未接线能力 |
| Langfuse 兼容与 OTEL 双模型漂移 | 同一 observation 两入口写入，查询只暴露一套冻结字段 |

## 5. 建议顺序（能卖）

P0 文档 → P1 可选检索 → P2 报表路径 → P3 网关 OTLP。P2 甚至可在网关接线前用现有 traces/metrics 先出「按 model 的周成本」——只要金额/token 已经在 observation 里。

## 6. 性能：PostgreSQL 够不够？

**结论：以本产品定位，PostgreSQL 就是主存储，不要为「看起来能扛 scraped metrics」上 ClickHouse。** 电表在 Prometheus；Xtrace 存的是业务事实（每次调用一条/数条 observation + 少量低基数点），量级比 Prom 刮取低一到两个数量级。

现状（代码）：ingest / metrics 均为 `try_send` 队列（满则 429）+ 50ms 微批最多 200 包写入；`metrics(project_id,name,timestamp)` + labels GIN；traces `(project_id, timestamp DESC)`；查询侧已有 series/points cap。这些对「对账单」是对的形状。

### 6.1 PG 能撑到什么规模（经验量级，非 SLA）

| 负载 | 单机 PG 16 + 本架构 | 说明 |
| --- | --- | --- |
| 每秒几十～几百条 trace/observation | 舒适 | 典型一个网关集群的业务调用 |
| 日千万级 metric 点（低基数 label） | 需要保留 + 降采样（已有 `METRICS_RETENTION_DAYS` / `DOWNSAMPLE`） | 不要把每请求当 metric 点 |
| 把 `/metrics` 刮取量或 `request_id` 当 label | **会先把 PG 打死** | 产品边界禁止 |

### 6.2 必须做的性能纪律（比换库更重要）

1. **写入永不挡热路径**：继续队列 + 满员 429；生产者（Xrouter）必须 degrade，禁止同步直写。队列深度做成可配，但默认保持「宁可丢观测不可拖网关」。
2. **高基数不出 metrics 表**：`request_id` / `user_id` 只进 traces/observations。报表 `group_by` 白名单。
3. **大 JSON 冷热分离**：`traces.input` / `output` 是 Langfuse 兼容所需，也是 PG 膨胀主因。列表查询必须继续 `fields=core` 不带大字段；长期可把 payload 外置对象存储，表内只留摘要（**加法**，不改现有详情 JSON 形状）。
4. **按时间砍数据**：保留天数是产品不是运维彩蛋；报告走日/小时 rollup，不扫热表做「全年每一跳」。
5. **索引跟查询走**：P1 若按 `metadata.request_id` 查，用表达式索引或把 `request_id` 提升为**可空生成列**（旧行可空，不改 JSON API）。禁止无索引扫 JSONB。
6. **单租户隔离先于跨租户扫**：所有热查询带 `project_id` 前缀（现有索引已是这个形状）。
7. **观测自己**：`ingest_stats` / 队列拒绝数是容量信号；拒绝升高先扩 worker/批大小，再谈换库。

### 6.3 什么时候才考虑「PG 不够」

按顺序，都还在 PG 生态内：

1. `traces` / `metrics` **按月 RANGE 分区**（Xrouter `request_logs` 已走这条路）——保留变便宜，查询带时间窗。
2. 报表物化表 / 定时 rollup（P2 报表读物化，不读热 observation）。
3. 仍不够再评估 **Timescale**（还是 PG 协议，迁移面小）或把 **纯时序** 拆到客户已有 Prom。
4. ClickHouse 只在「Xtrace 要当第二套 scraped TSDB」时才有意义——那是定位失败，不是性能胜利。

无 PG 的 JSON/内存模式继续留给边车和试用，不当生产报告库。

### 6.4 和接口延续的关系

性能优化不得改冻结 JSON。分区、生成列、外置 blob、物化报表都是存储实现，对外路径仍是 §0.1。
