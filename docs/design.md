# xtrace UI Design Specification

`docs/design.md` 是 **UI 设计薄入口**。xtrace 仪表板在 `frontend/`（Vite + React + shadcn）。细则不够时先看现有 `frontend/src/components` 与 `frontend/src/pages`，再补章节，不要一次发明第二套视觉方言。

## Overview

密集运维台，不是营销站：近中性表面、一个主强调色、成功/警告/危险语义色分离。

## Hard rules

1. 产品页不用硬编码 `#hex` / `bg-red-*`；走语义 token（`primary` / `destructive` / `muted`）。
2. 不用原生 `<select>`；用共享 Select。
3. 弹窗：`DialogHeader` / `DialogFooter`；`max-h-[85vh]` + `overflow-y-auto`；长内容折叠或分页。
4. Overlay 点击遮罩或 Escape 关闭；删除用确认对话框，不用 `window.confirm`。
5. 全局配置只进设置页，不在头像菜单另开入口。
6. 图标按钮必须有 accessible name。
7. 列表选中用 `bg-primary/10`，不用彩色左边框。
8. 用户可见文案若做 i18n，中英对称；禁止中英混杂（标准术语 API Key / HTTP / JSON / OTEL / Trace 除外）。

## PR checklist

1. 颜色来自语义 token。
2. 弹窗不撑爆视口。
3. `cd frontend && npm run lint`（改 UI 时）。
4. 不把骨架功能写成已有能力。
