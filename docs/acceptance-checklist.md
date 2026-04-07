# 验收清单

## 核心功能

- [x] 多文件、多 sheet 读取链路已设计并编码
- [x] 三行一人解析器已重写，移除`>=5`日期列硬门槛
- [x] 时间规范化支持全角、紧凑数字、Excel 浮点时间
- [x] AM/NOON 窗口已参数化，默认口径已冻结
- [x] 汇总、明细、需要打卡日数据结构已落地
- [x] 通报名单支持缺勤天数/次数和`AND/OR`
- [x] xlsx/csv/通报名单导出链路已编码
- [x] 前端已具备文件、规则、预览、导出、日志面板

## 工程与安全

- [x] 工程已切换到 Tauri v2 + Rust + React 结构
- [x] capability 已收敛到最小窗口与文件对话框能力
- [x] 默认声明本地处理、不出网
- [x] 日志与路径脱敏策略已写入代码与文档

## 测试与性能

- [x] `tests/fixtures/*` 已建立
- [x] Rust 单元/集成测试文件已建立
- [x] benchmark harness 已建立
- [x] 当前机器已完成 `cargo test`
- [ ] 当前机器已完成 `cargo bench`

## 打包与交付

- [x] Windows release 配置已写入 `tauri.conf.json`
- [x] 发布说明 `docs/release.md` 已生成
- [x] 构建脚本 `scripts/build-win.ps1` 已生成
- [x] 当前机器已产出 `.exe`

## 阻塞项

- [ ] `cargo bench` 尚未实际执行
- [ ] GitHub Actions 需在本次推送后重新生成最新 `exe` 产物
