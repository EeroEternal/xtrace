# 模块边界

> 如何加厚 xtrace、如何隔离第二实现与生产者定制。不是能力清单。

## 1. 一句话

用 **端口 + 不可变事实** 养厚同进程领域；用 **OTLP 契约** 隔离生产者（Xrouter / Xinference / 任意 OTEL SDK）；不要为「看起来干净」抽没有第二消费者的 crate，也不要 join 生产者的活表。

## 2. 三个对象

| 对象 | 何时 | 可变？ | 谁读 |
| --- | --- | --- | --- |
| **Snapshot** | ingest 鉴权时印出的 project / token 视图 | 下一请求可换 | HTTP 层 |
| **Fact** | 一条 trace / observation / metric 点写入后 | 只追加 | 查询、UI、保留任务 |
| **Port** | OTLP ingest、查询 API | 签名稳定 | 生产者与仪表板 |

纪律：

1. 正在写入的请求不得改历史点。
2. 事实只陈述：延迟、token、错误码；质量分、成本解释是订阅方算的。
3. 一份 OTEL 事实，多家订阅（Xtrace UI、客户 Tempo、未来计费解释）。禁止各家再 parse 生产者 `request_logs`。
4. 跨模块只走端口。禁止 SQL join 生产者数据库。

## 3. 放哪

| 需求 | 放哪 |
| --- | --- |
| OTLP 解码、队列、落库 | xtrace 核心（`src/ingest/`、`src/http/`） |
| Langfuse HTTP 兼容 | 兼容面，不得反向污染 OTEL 主路径 |
| Xrouter 路由/配额/选路 | **Xrouter** |
| 领域属性 `gen_ai.*` / `xrouter.*` | **命名约定**，不是新列 |
| 前端仪表板 | `frontend/`，只读查询 API |
| 请求级怪癖（客户私有头） | 生产者插件，不进 xtrace 内核 |

抽 crate 门槛：真有第二个实现或必须独立 ABI。`crates/xtrace-client` 已是给生产者用的客户端，保持瘦。

## 4. 一次请求（生产者侧）

```text
Xrouter 准入与转发（本仓库看不见）
  → 可选 OTLP Export（traceparent + attributes）
  → xtrace ingest 队列
  → 异步落库
  → 查询 API / UI
```

ingest 失败不得拖垮生产者热路径（队列满 → 429，由生产者 degrade）。
