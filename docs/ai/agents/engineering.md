# Engineering hard rules

## 一、禁止夹带变更（Silent Drift）

单个 commit / PR 不得夹带与声明目标无关的修改。特别是以下行为一律视为违规：

1. **调参私货**：以 "remediate" / "refactor" / "format" 名义修改默认值、阈值、超时、并发上限、限流配置等生产参数。
2. **批量重格式化**：`cargo fmt` / `prettier` / `black` 的结果必须单独 commit，不得与逻辑改动混在同一 commit。
3. **`#[allow(...)]` 压制**：用 `#[allow(clippy::xxx)]` 让警告消失，但没说明原因。
4. **跨模块 refactor**：修复 A 模块的 bug 时，不得同时重命名 B 模块的内部结构。

违规 commit 一律要求 `git reset --mixed HEAD~1` 重新拆分。

---

## 二、`git stash` 使用规则

`git stash` 极容易把 feature 依赖一起卷走。硬性约束：

1. **命名必须诚实**：`git stash push -m "..."` 不得用含糊描述掩盖逻辑改动。
2. **stash 前** `git diff --stat`，确认当前 feature 依赖不被抽走。
3. **stash 后必须** `cargo check --tests`（不可跳过）。
4. **不得 stash** `Cargo.toml` / `Cargo.lock` / 构建脚本。

完整步骤与误判症状见 skill [`git-stash-safe`](../../../.agents/skills/git-stash-safe/SKILL.md)。

---

## 三、引入后台任务 / daemon / background spawn 的规范

`tokio::spawn` / 其它 background task 是高风险构造，必须遵守：

1. **生命周期显式化**：不得在 `impl Default::default()` / `impl X::new()` 构造器里裸写 `tokio::spawn`。必须提供显式的 `start_background_xxx(&self)` 或类似入口方法，由 `init()` / `main()` 调用。
2. **Tokio runtime 守卫**：spawn 代码必须用 `tokio::runtime::Handle::try_current()` 包一层，或者只在已知 runtime 上下文中触发。避免同步测试/CLI 场景 panic。
3. **超时保护**：长跑任务里的每次异步调用都必须有 `tokio::time::timeout(...)` 兜底，避免 daemon 永久挂死。
4. **错误不得吞掉**：panic / timeout / calculate 错误必须 `tracing::error!` 至少记录一条，不得 silent `continue`。
5. **暴露 observability**：daemon 写入的共享状态（`Arc<AtomicX>` / `Arc<RwLock<X>>`）必须有至少一个 metric / debug endpoint 可以观察到最新值，否则无法判断 daemon 是否存活。

---

## 四、避免生成幻觉代码（Ghost Code）

以下模式一律视为幻觉：

1. **有定义无调用**：写了函数 / 类型，但全仓库 `grep` 找不到任何 caller。
2. **有路径无写入**：新增 `cached_xxx: Arc<AtomicU64>` 字段，有 load 点但没有 store 点。
3. **有埋点无分类**：metric counter 只埋上升事件（scale_up）不埋下降事件（scale_down），或反之。关键路径必须**正反两向**都有埋点。
4. **有 TODO 无 issue**：`// TODO: ...` 必须关联 GitHub issue 或 ticket。

发现此类幻觉，要求一次性补齐 caller / store / 下游埋点，不接受"下一轮再做"。

---

## 五、脚本化代码修改（`sed` / Python regex / awk）的规范

用脚本批量修改源码是高频犯错场景：

1. **regex 替换之后必须 grep 验证**。例如替换 `foo` 为 `bar`，必须 `rg "foo"` 确认剩余零匹配，或剩余匹配在预期内。
2. **`re.sub` 失败不会抛异常**。Python 正则不匹配时返回原串，不会报错。脚本跑完必须亲眼对比 diff 或重跑 grep 验证，不得直接 `git commit`。
3. **多行 regex 特别危险**：空白、引号、关键词拼写的任何一点偏差都会导致替换静默失败。优先使用 `StrReplace` 工具而非自写脚本。
4. **脚本内嵌字符串要小心转义**：Python 的 `'...'` 里如果包含 `"don't"` 这种，逃逸容易残留在最终代码里。脚本修改后必须人肉 review 一次生成的代码段。

---

## 六、摄入队列 / 保留 / 限流默认值

`RATE_LIMIT_*`、`*_RETENTION_DAYS`、ingest 队列容量等默认值影响生产。任何默认值调整必须：

1. **单独 commit**，不得与 bug fix / refactor 合并。
2. **commit message 必须列出**：旧值、新值、调整理由、预期影响范围。
3. **至少跑一次 `cargo test --workspace`**。
4. 对外行为变化写入 `README.md` 或 `docs/api.md`。

---

## 十二、数据库迁移

新增 `migrations/NNN_*.sql` 后，必须按 skill [`add-sql-migration`](../../../.agents/skills/add-sql-migration/SKILL.md) 强制重建二进制；仅 `cargo build` 而不 `touch` 嵌入点时，新迁移常不会进二进制，极易误判为 SQL 写错。

---

## 十三、核心通用、领域走约定

1. **OTLP 是主入口**；Langfuse HTTP 是兼容面，不得反向决定 schema。
2. **禁止为单一生产者固化列/表**：Xrouter / Xinference 字段用 span attributes 与 metric labels（`gen_ai.*`、`xrouter.*`）。
3. **禁止直连生产者数据库**。连接只靠 OTLP 与公开查询 API。
4. 生产者侧的 Session 提取、脱敏、私有 Header 留在生产者插件，不进 xtrace 内核。

---

## 十四、命令管道的退出码与「看似验证」陷阱

管道的退出码默认取**末位命令**：`失败命令 | tail -3` 的退出码是 tail 的 0。两个独立场景复现过此坑（CI watcher `| tail` 误报全绿；试合并 `| tail` 把 fatal 当成功打印 ✓）。

- **症状**：管道后的 `&&` 链继续执行；后台监听、脚本分支依据「末位命令」的退出码给出假结论。
- **正确做法**：判断上游成败时用 `set -o pipefail`（脚本内）、临时文件承接输出、或先跑命令再读输出；交互式判断「完成了吗」直接查权威源（`gh pr checks` / `gh run view` 的 conclusion 字段），不信代理进程退出码。
- **验证**：`bash -n` 只验语法；必须**实跑并人为制造一次上游失败**（如 `false | true`）确认链路中断。注意 `bash -o pipefail -n` 是无效组合——`-n` 不执行脚本，pipefail 不起作用，这本身就是一个「看似验证」。

---

## 八、违规处理

以下情形视为严重违规，审查方有权要求：

1. **报告与代码不符**：`git reset --hard` 回退到上一个诚实状态，重写提交。
2. **编译失败还报完成**：工作权限临时暂停，恢复前需提交书面复盘。
3. **重复同类错误 3 次以上**：该代理后续所有 commit 必须 pair review，两人 sign-off 才能合并。
4. **夹带生产参数私货**：立即 revert commit，重新按单独 commit 格式提交。

---

## 附：本规范的由来

规范骨架来自 rust-agentic-sekleton / xrouter 代理协作审查：幻觉报告、stash 抽依赖、regex 埋点失败、修复 commit 夹带生产调参。xtrace 侧额外约束：OTEL 主路径、不嵌回生产者、领域语义不进核心 schema。
