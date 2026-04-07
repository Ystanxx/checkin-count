# 性能基线

## 基准目标

1. 解析人员块
2. 逐日聚合
3. 通报名单过滤
4. xlsx/csv 导出

## 已落地内容

1. `src-tauri/benches/attendance_benchmark.rs`
   - `aggregate_records_200_people`
   - `build_notice_rows_200_people`
2. 前端表格采用分页渲染，避免一次性绘制全部行
3. Rust 命令通过后台任务执行，进度事件单独传输

## 计划执行命令

```powershell
cargo bench
```

## 当前结果

- benchmark harness：已建立
- 实际 benchmark 数值：**未执行**
- 原因：当前机器缺少 Rust 工具链，无法运行 `cargo bench`

## 后续补充口径

Rust 环境补齐后，至少补录 3 组数据：

1. 50 人 / 1 文件 / 1 sheet
2. 200 人 / 2 文件 / 4 sheet
3. 500 人 / 多文件混合 sheet

并记录：

- 解析耗时
- 聚合耗时
- 通报名单耗时
- 导出耗时
- 峰值内存观察
