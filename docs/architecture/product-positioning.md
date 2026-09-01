# Xtrace 产品定位（商业 + OTEL + Xrouter）

> **地位**：产品与集成边界。已落地能力必须能对到代码；规划项不得写成既有实现。
> 代码锚点：OTLP traces/metrics ingest 见 `src/ingest/otlp.rs`；通用点表 `metrics`；trace/observation 见 `src/http/` 与 `src/ingest/`。

## 1. 一句话

Xtrace 是**独立售卖的 AI/LLM 可观测产品**：兼容 OTEL，比通用 Tempo/Jaeger 多一层模型调用账本。Xrouter 是金牌数据源，不是唯一数据源，更不是宿主进程。

和 Prometheus / Langfuse / APM 的错开、以及「报告 vs 仪表板」的冻表见 [`competitive-boundary.md`](competitive-boundary.md)。

## 2. 卖什么、不卖什么

通用 OTEL 后端已经能画 HTTP span。Xtrace 的差异化是 LLM 网关/推理专有事实：

- turn / run / attempt / fallback 时间线
- 路由、实例、排队、TTFT / token / 成本
- 租户：project（以及约定中的 org / api_key）

**卖点**：兼容 OTEL，但能回答「这次费用是哪一次 fallback 打出来的」。

| 放 Xtrace | 不放 Xtrace |
| --- | --- |
| OTLP traces / metrics ingest | 生产者进程内嵌追踪引擎 |
| 按 `trace_id` / `request_id` 查询与 UI | 调度、选路、配额、计费账本 |
| LLM semantic attributes（命名约定） | 为单一厂商固化 schema 列 |
| 再导出 OTLP 给客户已有 Tempo | 直连 Xrouter Postgres 读 `request_logs` |

Xrouter 卖治理与转发；Xtrace 卖解释与排障。License 可拆：网关 `ha` / 智能路由一套，可观测另一套。

## 3. 与 Xrouter 怎么连（不要嵌回去）

Xrouter v1.3.0 已删除进程内 xtrace。正确链接：

```text
客户端 (可选 traceparent)
  → Xrouter 数据面
       认/发 W3C traceparent
       span attributes: xrouter.request_id, route, provider, attempt
       上游 HTTP 继续传播；fallback = child span
       请求结束异步 OTLP（默认关闭，otlp_endpoint 空则旁路）
  → OTLP
  → Xtrace（也可被任意 OTEL collector 使用）
```

关联键（对外承诺，当作 API 冻住）：

| 键 | 谁生成 | 用途 |
| --- | --- | --- |
| `trace_id` / `traceparent` | 客户端或网关 | 分布式调用树 |
| `xrouter.request_id`（及 `X-Request-Id`） | 网关 | 对账网关 `request_logs` 这一票 |
| `session_id` / `turn_id` / `run_id` | 客户端或网关 | 对话与 Agent |
| `project_id` | ingest 鉴权 | Xtrace 租户隔离 |

套装体验只需控制面填 OTLP 地址。数据面仍是标准 OTLP：换成 Jaeger 也能工作。

**禁止**：Xtrace 直连 Xrouter 数据库；在 xtrace 核心为 `xrouter.*` 加专属表。领域字段进 span/metric **attributes**，走 OTel GenAI 约定 + 网关扩展名。

## 4. OTEL 兼容怎么才像产品

1. **主入口是 OTLP**（已有 `POST /api/public/otel/v1/traces` 与 `/metrics`）。Langfuse HTTP 是兼容面，不是战略主路径。
2. **属性字典对外文档化**：`gen_ai.request.model`、`gen_ai.usage.input_tokens`、`xrouter.route`、`xrouter.provider`、`xrouter.attempt_index`、`xrouter.is_fallback`、`xrouter.queue_wait_ms`。未实现的不要写成已有。
3. **Xrouter 金牌、非唯一**：vLLM / 业务服务只要发 OTEL，Xtrace 也能收。这样才能卖到没买网关的团队。
4. **能进能出**：规划再导出 OTLP；避免「进了 Xtrace 就出不来」。
5. **默认可关**：生产者侧 OTLP 必须 feature/配置门控，热路径零订阅者时零开销。

当前缺口（不得当已落地）：Xrouter 尚未发 `traceparent`、配置里 `otlp_endpoint` 未接线；Xtrace 尚无 OTLP logs。链接第一期是 **ID 级关联**，不是完整 mesh tracing。

分阶段实现、以及 **不得破坏的对外路径** 见 [`optimization-plan.md`](optimization-plan.md)。

## 5. 落地顺序

1. 冻本文关联键与 attribute 名（本文 + `docs/api.md`）。
2. Xrouter：W3C 传播 + 可选 OTLP span（另一仓库，feature 默认关）。
3. Xtrace：按 `trace_id` / `xrouter.request_id` 检索与 UI 互跳。
4. 订阅网关 Observation（异步，不挡响应）——差异化所在。
5. 多副本 Prometheus 仍按实例抓；不必等 Xtrace 聚合。

Xtrace 从第一天就是**另一个仓库、另一套发布**。这与 Xrouter 把智能路由放契约 crate、把 unigateway 放可选依赖是同一纪律。
