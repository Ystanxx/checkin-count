# 架构设计

## 1. 总体目标

新系统采用“Rust 重业务、React 轻展示、Tauri 负责桌面壳”的结构。
所有高成本计算、Excel 读取、规则判定、导出都在 Rust 后台完成，
前端只消费精简 DTO 和进度事件。

## 2. 分层结构

### 2.1 Rust 侧

```text
src-tauri/src/
├── commands/
├── application/
├── domain/
├── infrastructure/
├── state.rs
├── error.rs
└── main.rs
```

#### `domain`

负责纯业务规则与领域对象：

- `attendance_schema`
- `model`
- `rules`
- `block_detector`
- `time_normalizer`
- `window_classifier`
- `aggregator`
- `notice_filter`
- `error`

#### `application`

负责用例编排与 DTO：

- `app_service`
- `notice_service`
- `dto`

#### `infrastructure`

负责外部资源交互：

- `excel_reader`
- `export_xlsx`
- `export_csv`
- `logging`
- `security`

#### `commands`

负责 Tauri 对外命令与输入校验：

- `select_input_files`
- `parse_attendance_preview`
- `build_summary`
- `build_notice_list`
- `export_summary_xlsx`
- `export_summary_csv`
- `export_notice_list`

### 2.2 React 侧

```text
src/
├── pages/
├── components/
├── store/
├── types/
└── utils/
```

#### `pages`

- `FileSelectPage`
- `RulesPage`
- `PreviewPage`
- `ExportPage`

#### `components`

- `FilePickerPanel`
- `RuleForm`
- `PreviewTabs`
- `SummaryTable`
- `DetailTable`
- `NeedDaysTable`
- `NoticeTable`
- `LogPanel`
- `ProgressBar`

#### `store`

- `appStore`
- `taskStore`
- `rulesStore`

#### `types`

- 命令请求/响应 DTO
- 视图模型
- 表格行类型

## 3. 模块职责

### `attendance_schema`

定义输入表结构、块解析结果、标准化记录、汇总记录、通报记录等 DTO。

### `excel_reader`

读取多文件、多 sheet 的 Excel 数据，输出统一的工作表抽象，
并携带文件名、sheet 名、行列号等定位信息。

### `block_detector`

从工作表中探测人员块，输出：

- `name`
- `day_to_tokens`
- `provenance`

### `time_normalizer`

把原始 token 规范化为强类型时间值，处理：

- 常见时间文本
- 紧凑数字
- Excel 时间小数
- 全角字符

### `window_classifier`

从`AttendanceRules`读取窗口配置，判断命中`AM`、`NOON`或无效。

### `aggregator`

基于姓名全集与标准化记录，生成：

- 明细表
- 汇总表
- 需要打卡日表

### `notice_filter`

基于汇总表与通知规则，生成通报名单。

### `export_xlsx`与`export_csv`

负责 Rust 原生导出，不把导出职责泄露给前端。

### `app_service`

编排读取、解析、聚合、通报、导出等完整业务流程。

## 4. 关键 DTO

### 输入 DTO

- `ParsePreviewRequest`
  - `input_files`
  - `year`
  - `month`
  - `rest_days`
  - `rules`
  - `start_row_mode`
- `NoticeBuildRequest`
  - `notice_rules`
  - `summary_snapshot`
- `ExportRequest`
  - `output_path`
  - `include_detail`
  - `include_need_days`
  - `include_notice`

### 输出 DTO

- `PreviewResponse`
  - `summary_rows`
  - `detail_rows`
  - `need_days_rows`
  - `notice_rows`
  - `warnings`
  - `stats`
- `TaskProgressEvent`
  - `task_id`
  - `stage`
  - `percent`
  - `message`
- `UserVisibleError`
  - `code`
  - `message`

## 5. 调用链

```text
前端页面
  -> Tauri command
    -> application::app_service
      -> infrastructure::excel_reader
      -> domain::block_detector
      -> domain::time_normalizer
      -> domain::window_classifier
      -> domain::aggregator
      -> domain::notice_filter
      -> infrastructure::export_xlsx / export_csv
```

## 6. 线程模型

1. 所有重活由 Rust 后台任务执行。
2. Tauri command 不在 UI 主线程中做大文件解析。
3. 进度事件节流发送，避免前端过度重渲染。
4. 多文件处理默认串行，优先保证可复现和稳定；
   后续仅在隔离明确的阶段预留并行接口。

## 7. 错误边界

1. `domain error`
   - 业务规则违规
   - 时间 token 非法
   - 解析块不完整
2. `infrastructure error`
   - 文件不可读
   - 导出失败
   - Excel 格式损坏
3. `command error`
   - 参数非法
   - 未选择文件
   - 输出路径不合法

用户可见错误与内部调试错误必须分层。

## 8. 安全边界

1. 只允许系统文件对话框返回的路径参与读取或保存。
2. capability 最小化，不启用 HTTP、shell、任意 FS。
3. 默认日志脱敏，不输出完整绝对路径和完整个人数据。

## 9. 前端渲染策略

1. 汇总/明细/需要打卡日/通报名单使用 Tabs。
2. 大表格使用虚拟滚动或分页。
3. 前端保存精简快照，不重复持有大对象副本。
