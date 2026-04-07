# 测试矩阵

## 覆盖目标

1. 旧自检样本
2. 两位员工都能被识别
3. 窗口边界
4. 全角字符
5. Excel 浮点时间
6. 空文件
7. 坏文件
8. 多 sheet
9. 多文件同名人员
10. 0 打卡人员纳入汇总
11. 通报名单阈值边界
12. 导出冒烟测试

## 当前测试设计

1. `src-tauri/tests/block_detector.rs`
   - 覆盖两位员工识别
   - 覆盖短样本表不再依赖`>=5`日期列
2. `src-tauri/tests/time_rules.rs`
   - 覆盖全角字符、紧凑数字、Excel 时间小数、`14:05`边界
3. `src-tauri/tests/aggregator.rs`
   - 覆盖重复打卡去重
   - 覆盖`0`打卡人员进入汇总
4. `src-tauri/tests/notice_filter.rs`
   - 覆盖阈值与`AND/OR`组合
5. `src-tauri/tests/export.rs`
   - 覆盖 xlsx/csv/通报名单导出冒烟

## fixture 目录

- `tests/fixtures/legacy_selfcheck_two_people.json`
- `tests/fixtures/short_sample_blocks.json`
- `tests/fixtures/time_tokens.json`

## 当前状态

- 测试代码与 fixture 已落地
- 受当前机器缺少 Rust 工具链影响，尚未实际执行 `cargo test`
- 一旦 Rust 环境补齐，优先执行：
  1. `cargo test --tests`
  2. `cargo test --test block_detector`
  3. `cargo test --test export`
