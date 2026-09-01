# Architecture

薄入口。细节按主题拆在 [`docs/architecture/`](architecture/)。

| 主题 | 文件 |
| --- | --- |
| 产品定位、商业边界、与 Xrouter / OTEL | [`product-positioning.md`](architecture/product-positioning.md) |
| 竞争边界：Prom / OTEL / Langfuse / 报告 | [`competitive-boundary.md`](architecture/competitive-boundary.md) |
| 优化方案与接口延续 | [`optimization-plan.md`](architecture/optimization-plan.md) |
| 模块边界、端口、禁止跨表 join | [`module-boundaries.md`](architecture/module-boundaries.md) |

运行与 HTTP 契约仍以根 [`README.md`](../README.md)、[`docs/api.md`](api.md)、[`docs/dev.md`](dev.md) 为准。
