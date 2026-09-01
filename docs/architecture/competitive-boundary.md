# 竞争与能力边界（冻）

> **地位**：产品边界。已落地必须对到代码；未做的不得对外当能力。
> 总定位见 [`product-positioning.md`](product-positioning.md)。优化阶段与 **API 冻结表** 见 [`optimization-plan.md`](optimization-plan.md)。

客户要的是 **到处收数 → 对上同一笔业务 → 出报告**。三层不可抢：

| 层 | 产品 | 回答的问题 |
| --- | --- | --- |
| 电表 | Prometheus（及 Xrouter `/metrics`） | 箱子活着吗、QPS、CPU |
| 公路 | OTEL | 各系统怎么用同一种话说话 |
| 对账单 | **Xtrace** | 这次调用贵在哪、失败在哪、哪个模型/路由/attempt |

## 1. 和谁错开

| 对手类型 | 代表 | 他们强 | Xtrace **不跟** | Xtrace **跟** |
| --- | --- | --- | --- | --- |
| Prompt / Eval 平台 | Langfuse、LangSmith、Braintrust、Phoenix | 实验、数据集、prompt 版本、人工评分 | 不做 Playground / 离线 eval 平台（可作未来第二产品线） | 生产 trace、成本、会话；Langfuse HTTP **仅兼容迁入** |
| LLM 代理日志 | Helicone、Portkey、LiteLLM 回调 | 拦在 API 前记账 | 不做成第二个网关 | 网关把事实 **OTLP 推过来**，观测独立卖 |
| 通用 APM | Datadog LLM、Tempo/Jaeger、OpenLLMetry | 全栈、OTEL 纯正 | 不刮 GPU、不抢告警 | LLM 语义 + 租户报告；Prom 继续看箱子 |
| Agent 专项 | AgentOps、OpenInference | 工具调用图 | 第一年不做华丽拓扑 | `session → turn → run → step/attempt` 时间线与费用 |

**对外一句话**：企业内网的 LLM 业务观测与报告层——讲 OTEL，吃网关事实，不绑框架，不出公有云。

不是「开源 Langfuse」。Langfuse 兼容是迁 Xinference / 存量客户的适配器，不是灵魂。

## 2. 放进 Xtrace / 留在别处

| 放进 Xtrace | 留在别处 |
| --- | --- |
| OTLP traces / metrics ingest（已有） | 生产者进程内嵌追踪引擎 |
| 按 `trace_id` / `request_id` 查询与 UI | 调度、选路、配额、**计费账本**（Xrouter） |
| 租户级 **报告**（成本/失败/TTFT，可导出） | Prometheus 刮取与告警 |
| session / turn / run / attempt 展示 | Prompt CMS、数据集、实验对比 |
| Langfuse HTTP 兼容面 | 直连 Xrouter / Xinference 数据库 |
| 约定 attributes：`gen_ai.*`、`xrouter.*` | 核心 schema 专属列、专属表 |

报告口径：成本用 **当时单价快照**，不事后用现价重算。点得开 `trace_id` / `xrouter.request_id` 的报告才算产品；点不开的只算仪表板。

## 3. 收数边界

**收**：Xrouter（规划 OTLP，默认关）、Xinference、业务 OTEL SDK。金牌数据源不是唯一数据源。

**不收**：去 scrape `/metrics` 替代 Prom；全量应用日志（OTLP logs 未做，第一期用 span/observation）。

Metrics **禁止**把 `user_id` / `request_id` 放进 labels（高基数）。请求级细节只在 trace/observation。

## 4. 已落地 vs 规划（防幻觉）

| 已落地（代码） | 规划（未做） |
| --- | --- |
| `POST /api/public/otel/v1/traces` 与 `/metrics` | Xrouter 发 `traceparent` / OTLP（另一仓库） |
| Langfuse 兼容 traces 查询 | OTLP logs；再导出 OTLP |
| session/turn/run 元数据约定 | 报表实体、CSV/定时邮件 |
| `metrics/daily`、`metrics/query` | 报表行 ↔ 网关日志一键互跳 |
| 自托管；project token 隔离 | Agent 图可视化；eval 平台 |

## 5. 套装怎么卖（不锁死）

- 有 Xrouter：流量在网关，故事在 Xtrace；中间只认 OTEL。
- 有 Xinference + Langfuse：换 HOST 即可，数据不出内网。
- 已有 Datadog/Prom：箱子仍归他们；模型生意归 Xtrace。
- 无网关：只要打 OTLP 也能用——否则 Xtrace 卖不进「没买网关」的团队。
