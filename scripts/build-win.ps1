param(
    [switch]$SkipInstall
)

$ErrorActionPreference = "Stop"

Write-Host "[1/4] 检查 pnpm"
pnpm --version | Out-Null

if (-not $SkipInstall) {
    Write-Host "[2/4] 安装前端依赖"
    pnpm install
}

Write-Host "[3/4] 运行前端构建"
pnpm build

Write-Host "[4/4] 运行 Tauri Windows Release 构建"
pnpm tauri:build
