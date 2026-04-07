# 团队打卡数据处理

纯本地离线的桌面端打卡数据处理工具。

当前版本采用以下技术栈：

- `Tauri v2`
- `Rust`
- `React + TypeScript`

核心业务、Excel 读取、规则判断、聚合与导出全部在 `Rust` 侧完成，前端只负责界面展示与命令调度，不接入远程 API，不保留 Python 运行时。

## 功能范围

- 多文件、多工作表读取
- 支持 `xls`、`xlsx`、`xlsm`
- 更稳健的“三行一人”解析
- AM / NOON 时间窗口配置
- 汇总表、明细表、需要打卡日、通报名单预览
- 按缺勤天数或缺勤次数生成通报名单
- 支持 `AND / OR` 组合逻辑
- 导出 `xlsx`
- 导出带 `UTF-8 BOM` 的 `csv`
- 默认日志脱敏，不输出完整个人数据和完整绝对路径

## 默认业务口径

默认窗口预设采用旧代码真实行为，不再采信旧 README 文案：

- `AM = 00:00:00 ~ 09:11:59`
- `NOON = 11:00:00 ~ 14:11:59`

因此默认规则下，`14:05`计入有效 `NOON`。

同时，新系统固定保证：

- 已识别姓名但 `0` 打卡的人员，仍进入汇总
- 已识别姓名但 `0` 打卡的人员，仍进入通报名单判断链路

## 目录结构

```text
.
├── docs/                文档与报告
├── scripts/             构建脚本
├── src/                 React + TypeScript 前端
├── src-tauri/           Tauri 与 Rust 后端
└── tests/               测试夹具
```

## 本地开发

前置环境：

- `Node.js 22+`
- `pnpm 10+`
- `Rust stable`
- Windows 下建议已安装 `WebView2 Runtime`

安装依赖：

```powershell
pnpm install
```

启动前端开发服务：

```powershell
pnpm dev
```

启动桌面开发模式：

```powershell
pnpm tauri:dev
```

## 测试与检查

前端构建检查：

```powershell
pnpm build
```

Rust 测试：

```powershell
cargo test --tests
```

基准测试：

```powershell
cargo bench
```

## Windows 打包

当前仓库默认只产出 `exe`，不再生成 `msi`、`nsis` 安装包。

本地构建：

```powershell
pwsh -File .\scripts\build-win.ps1
```

或直接执行：

```powershell
pnpm install
pnpm build
pnpm tauri build --no-bundle
```

构建完成后的可执行文件位于：

```text
src-tauri/target/release/*.exe
```

## GitHub Actions

仓库已配置 Windows 云端构建工作流：

- 工作流文件：`/.github/workflows/windows-build.yml`
- 产物类型：仅 `exe`

下载方式：

1. 打开仓库的 `Actions`
2. 进入最新一次 `build-windows-exe`
3. 在页面底部 `Artifacts` 下载 `team-attendance-exe-*`

## 隐私与安全

- 默认本地处理，不出网
- 不启用不必要的 shell / http / 任意文件系统权限
- 输入文件通过系统对话框选择
- 导出路径走受控保存路径
- 默认日志脱敏
- 仓库不保留真实打卡样本

## 关键文档

- 业务规则：[docs/spec/business-rules.md](./docs/spec/business-rules.md)
- 迁移报告：[docs/migration-report.md](./docs/migration-report.md)
- 测试矩阵：[docs/test-matrix.md](./docs/test-matrix.md)
- 发布说明：[docs/release.md](./docs/release.md)
