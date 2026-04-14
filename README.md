# DKDMD

<div align="center">

**どこでもドア（任意门）**

为 Claude Code、Codex 等本地 AI 工具配置自定义 API 端点

[![GitHub Release](https://img.shields.io/github/v/release/Maka314/DKDMD)](https://github.com/Maka314/DKDMD/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

</div>

---

## 这是什么

Claude Code、Codex 等 AI 编程工具默认只能连接官方 API。DKDMD 提供一个交互式菜单，让你方便地为这些工具配置自定义的 Base URL 和 API Key，从而接入任意兼容 OpenAI / Anthropic 协议的 API 服务（中转站、私有部署等）。

**支持的工具：**
- Claude Code（`@anthropic-ai/claude-code`）
- Codex（`@openai/codex`）

---

## 安装

```bash
curl -fsSL https://raw.githubusercontent.com/Maka314/DKDMD/main/install.sh | sudo bash
```

---

## 使用

```bash
DKDMD
```

启动后会显示交互菜单：

```
📋 Manage Models   — 添加/管理 API 模型（Base URL、API Key）
🛠️  Launch Tool    — 选择工具和模型并启动，未安装时会提示自动安装
⚙️  Show Config    — 查看当前配置
🗑️  Clear Config   — 清空所有配置
```

### 典型流程

1. **Manage Models → Add new model**：填入模型名、Base URL、API Key
2. **Launch Tool**：选择 Claude Code 或 Codex，再选刚添加的模型，即可启动

配置保存在 `~/.config/DKDMD/config.json`，下次打开无需重新输入。

---

## 卸载

```bash
sudo apt remove dkdmd
```

---

<div align="center">
Made with ❤️ by Mingchen
</div>
