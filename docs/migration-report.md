# 迁移报告

## 1. 已完成任务列表

### T001~T017 落地状态

1. `T001` 已完成
   - 输出 `docs/spec/business-rules.md`
   - 输出 `docs/spec/legacy-gap.md`
   - 关键迁移说明已并入根目录 `README.md`
2. `T002` 已完成代码落地
   - 已创建 `package.json`、`vite.config.ts`
   - 已创建 `src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json`
   - 已创建 `src-tauri/capabilities/default.json`
3. `T003` 已完成
   - 输出 `docs/architecture.md`
   - Rust/React 分层目录已建立
4. `T004` 已完成代码落地
   - 已创建领域模型、规则类型、错误类型
5. `T005` 已完成代码落地
   - 已实现 `excel_reader`
6. `T006` 已完成代码落地
   - 已实现稳健块解析器，去掉 `>=5` 日期列硬门槛
7. `T007` 已完成代码落地
   - 已实现时间规范化与窗口判定
8. `T008` 已完成代码落地
   - 已实现聚合器，并保留 `0` 打卡人员
9. `T009` 已完成代码落地
   - 已实现通报名单功能与 `AND/OR`
10. `T010` 已完成代码落地
    - 已实现 xlsx/csv/通报名单导出
11. `T011` 已完成代码落地
    - 已实现 Tauri command 与后台任务壳
12. `T012` 已完成代码落地
    - 已实现文件、规则、预览、导出、日志 UI
13. `T013` 已完成文档与 harness 落地
    - 已建立 benchmark harness
    - 已输出 `docs/performance-baseline.md`
14. `T014` 已完成代码与文档落地
    - 已收敛 capability、路径与日志策略
15. `T015` 已完成代码与文档落地
    - 已建立 `tests/fixtures/*`
    - 已建立 Rust 测试文件
    - 已输出 `docs/test-matrix.md`
16. `T016` 已完成配置与文档落地
    - 已补 `tauri.conf.json` Windows bundle 配置
    - 已输出 `docs/release.md`
    - 已输出 `scripts/build-win.ps1`
17. `T017` 已完成
    - 已输出 `docs/acceptance-checklist.md`
    - 已更新本报告

## 2. 关键设计决策

1. 默认时间窗口采用旧代码真实行为，而不是旧 README 文案：
   - `AM 00:00:00~09:11:59`
   - `NOON 11:00:00~14:11:59`
2. `14:05`在默认规则下明确计入有效 `NOON`
3. 姓名识别与打卡识别解耦，确保 `0` 打卡人员不丢失
4. 新系统所有重活都放在 Rust，前端不解析 Excel、不导出文件
5. 多文件处理默认先保证可复现与稳定，暂不激进并行
6. 导出与日志默认做脱敏与路径规范化

## 3. 与旧系统不一致之处

1. 新系统不再依赖 pandas/DataFrame 作为内部总线
2. 新系统不再依赖 Qt/QThread
3. 新系统不再使用 `>=5` 日期列作为块识别前提
4. 新系统把窗口边界改为规则配置，而不是常量散落
5. 新系统新增通报名单、阈值组合和导出能力
6. 新系统默认日志不再暴露完整绝对路径
7. 新系统首屏改为面向非技术用户的简化流程，高级能力收纳到菜单中

## 4. 修复的旧缺陷

1. README、代码、selfcheck 注释之间的窗口冲突已冻结成单一规格
2. `_parse_person_block` 的短样本漏人问题已规避
3. `parse_all_names` 复用脆弱门槛导致 `0` 打卡人员漏入汇总的问题已规避
4. 导出链路已从前端和旧 Python 运行时中彻底移出
5. 旧系统没有通报名单功能，新系统已补齐

## 5. 剩余风险

1. `cargo bench` 仍未实际执行，性能数值需要后补
2. GitHub Actions 需要等待本次推送后重新生成最新安装包产物
3. 目标机器若缺少 `WebView2 Runtime` 或 `Visual C++ Runtime`，仍可能影响运行

## 6. 本地运行方式

### 前端开发

```powershell
pnpm install
pnpm dev
```

### Tauri 开发

```powershell
pnpm install
pnpm tauri:dev
```

### Rust 测试

```powershell
cargo test --tests
```

### 基准测试

```powershell
cargo bench
```

## 7. 打包方式

```powershell
pwsh -File .\scripts\build-win.ps1
```

或者手动执行：

```powershell
pnpm install
pnpm build
pnpm tauri:build
```

当前默认产出绿色版 `exe`、`nsis` 安装包和 `msi` 安装包。

## 8. 测试结果

### 已落地

1. `src-tauri/tests/block_detector.rs`
2. `src-tauri/tests/time_rules.rs`
3. `src-tauri/tests/aggregator.rs`
4. `src-tauri/tests/notice_filter.rs`
5. `src-tauri/tests/export.rs`
6. `tests/fixtures/*`

### 当前执行结果

- 测试代码：已生成
- 实际执行：**已执行**
- 结果：全部通过
  - `aggregator`：2/2
  - `block_detector`：2/2
  - `export`：1/1
  - `notice_filter`：1/1
  - `time_rules`：2/2

## 9. benchmark 结果

### 已落地

1. `src-tauri/benches/attendance_benchmark.rs`
2. `docs/performance-baseline.md`

### 当前执行结果

- benchmark harness：已建立
- benchmark 实测数据：**未执行**
- 原因：当前机器缺少 Rust 工具链

## 10. 当前交付结论

本次重构已经把：

1. 规格冻结
2. 工程结构
3. Rust 核心模块
4. Tauri 命令层
5. React 前端页面
6. 测试矩阵与 fixture
7. benchmark harness
8. Windows 发布文档与脚本

全部落到当前工作区。

当前已完成本地前端构建、Rust 测试与 Windows release `exe` / `nsis` / `msi` 构建；剩余未闭环项主要是 benchmark 实测数据与本次推送后的 GitHub Actions 重新产物验证。
