# 发布说明

## 应用信息

- 应用名：团队打卡数据处理
- 版本号：0.1.0
- 桌面壳：Tauri v2
- 核心运行时：Rust
- 前端：React + TypeScript

## Windows Release 配置

1. Tauri 配置文件：`src-tauri/tauri.conf.json`
2. bundle 目标：
   - `nsis`
   - `msi`
3. Release 构建应使用：

```powershell
pnpm install
pnpm tauri:build
```

## 本地构建脚本

已提供：`scripts/build-win.ps1`

## 运行前提

1. Node.js 与 pnpm
2. Rust 工具链
3. WebView2 运行时（Windows 常见预装；若目标机器缺失，需要安装）
4. Visual C++ 运行库（若系统缺失，需补装）

## 发布建议

1. 先执行 `cargo test --tests`
2. 再执行 `cargo bench`
3. 最后执行 `pnpm tauri:build`

## 升级与回滚

1. 升级前备份旧导出目录
2. 新旧版本不要共用临时目录
3. 如需回滚，保留上一版安装包或便携版 `.exe`

## 已知限制

1. 当前仓库已落地 release 配置，但当前机器缺少 Rust 工具链，尚未实际完成 release 构建验证
2. 图标仍沿用现有 `logo.png` 资源，建议后续补齐 Windows `.ico`
