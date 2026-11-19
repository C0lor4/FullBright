# Fullbright

按 **G** 切换本地画面的全亮，单人和多人均可使用，默认开启。

适用于 Horizon 26.3 的 Nightly 2026-11-18（`6d765e0`）。在 **Options > Mods > Fullbright > Config** 可切换全亮，在 **Options > Controls** 可更改快捷键。

## 在 PowerShell 打包并安装

退出游戏，执行以下两行，再到 Horizon 启动器启用 Fullbright，点 **Build and play**。

```powershell
Set-Location 'F:\MC\mods\Fullbright'
py -3.11 .\package.py --install
```
