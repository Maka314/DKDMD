# DKDMD

<div align="center">

**どこでもドア (Dokodemo Doa) - 任意门**

让本地 AI 应用轻松接入任何 API 端点

</div>

## 📖 项目简介

DKDMD 源自日语"どこでもドア"（Dokodemo Doa）的罗马音缩写，意为"任意门"——哆啦A梦中那扇可以通往任何地方的神奇之门。

本项目旨在解决本地 AI 应用接入远程 API 的配置难题。通过 DKDMD，你可以让几乎所有运行在本地的 AI 应用无缝接入指定的 API 端口，而无需繁琐的配置过程。

## ✨ 核心特性

- 🚪 **零配置接入** - 让本地 AI 应用免于复杂的 API 配置
- 🔌 **统一端点** - 提供统一的 API 接入方式
- 🎯 **简单易用** - 开箱即用，快速上手
- 🔄 **兼容性强** - 支持多种本地 AI 应用
- ⚡ **高性能** - 基于 Rust 开发，性能卓越、内存安全

## 🛠️ 技术栈

- **开发语言**: Rust
- **特点**: 高性能、内存安全、并发友好

## 🎯 使用场景

DKDMD 适用于以下场景：

- 需要在本地运行 AI 应用，但想调用远程 API 服务
- 希望统一管理多个 AI 应用的 API 接入点
- 想要简化本地开发环境的 API 配置流程

## 🚀 快速开始

### 前置要求

- Rust 1.70 或更高版本
- Cargo（Rust 包管理器）

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/Maka314/DKDMD.git
cd DKDMD

# 构建项目
cargo build --release

# 运行
cargo run --release
```

## 📦 安装

### 方式一：从源码安装

```bash
cargo install --path .
```

### 方式二：使用预编译二进制

> 待发布后提供下载链接

## 💡 使用说明

### 基本用法

```bash
dkdmd [OPTIONS]
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

### 开发指南

1. Fork 本仓库
2. 创建您的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交您的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启一个 Pull Request

### 代码规范

```bash
# 格式化代码
cargo fmt

# 运行 linter
cargo clippy

# 运行测试
cargo test
```


---

<div align="center">
Made with ❤️ by Quaso
</div>
