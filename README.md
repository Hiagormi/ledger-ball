# 💰 记账助手 (Ledger Ball)

轻量级桌面记账应用，悬浮球形式，一键记录消费。

![preview](preview.png)

## ✨ 功能特性

- 🎯 **悬浮球设计** - 始终在桌面边缘，点击即可记账
- 🤖 **双模式解析** - 本地规则(免费) + AI智能理解(可选)
- 📊 **多周期统计** - 日/周/月/年统计一览
- 📁 **本地存储** - 所有数据在本地，隐私安全
- 📦 **轻量级** - 仅 5-10MB，无依赖

## 🚀 快速开始

### 1. 环境准备

需要安装：
- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/)
- pnpm/npm/yarn

### 2. 克隆并安装

```bash
git clone https://github.com/yourname/ledger-ball.git
cd ledger-ball
npm install
```

### 3. 开发模式

```bash
npm run tauri:dev
```

### 4. 构建发布版

```bash
npm run tauri:build
```

构建完成后，在 `src-tauri/target/release/bundle/` 目录下找到安装包。

## 🎮 使用方法

### 基本操作

1. **悬浮球** - 点击弹出记账面板，右键显示菜单
2. **输入记录** - 在输入框输入消费信息

### 支持的输入格式

**本地模式（无需配置）：**

```
餐饮 35            → 支出 | 餐饮 | ¥35
餐饮 午饭 35       → 支出 | 餐饮 | ¥35 | 备注:午饭
打车 25            → 支出 | 交通 | ¥25
收入 工资 +5000    → 收入 | ¥5000
3/15 购物 200      → 指定日期 | 购物 | ¥200
```

**AI模式（需配置API Key）：**

支持自然语言输入：
```
今天午饭花了35块
跟同事聚餐大概一百多
打滴滴回家花了25元
```

### 配置 AI 解析

右键悬浮球 → 设置：

1. 选择"AI解析"模式
2. 选择服务商（DeepSeek/阶跃AI/OpenAI/Claude）
3. 输入 API Key
4. 保存

推荐服务商：
| 服务商 | 价格 | 特点 |
|--------|------|------|
| DeepSeek | ¥0.001/次 | 最便宜，中文好 |
| 阶跃AI | 有免费额度 | 中文优秀 |
| Claude | $0.003/次 | 理解最强 |

## 📊 数据存储

账本数据存储在项目目录下的 `data/` 文件夹：

```
data/
├── ledger-2024-04.csv   # 2024年4月账本
├── ledger-2024-05.csv   # 2024年5月账本
├── config.json          # 用户配置
```

CSV 格式（可用 Excel 打开）：
```
日期,类型,分类,金额,备注
2024-04-01,expense,餐饮,35,午饭
2024-04-02,expense,交通,25,打车
2024-04-05,income,收入,5000,工资
```

## 🔧 开发

### 项目结构

```
ledger-ball/
├── src/                    # Vue 前端
│   ├── components/
│   │   ├── FloatingBall.vue    # 悬浮球
│   │   ├── InputPanel.vue      # 输入面板
│   │   ├── SettingsPanel.vue   # 设置面板
│   │   └── StatsPanel.vue      # 统计面板
│   └── main.ts
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   ├── main.rs             # 主程序
│   │   ├── commands.rs         # Tauri命令
│   │   ├── ledger.rs           # 账本操作
│   │   ├── parser.rs           # 输入解析
│   │   └── config.rs           # 配置管理
│   └── tauri.conf.json         # Tauri配置
├── data/                   # 数据目录
└── package.json
```

### 技术栈

- **前端**: Vue 3 + TypeScript + Vite
- **后端**: Tauri 2 + Rust
- **数据**: CSV 文件存储

## 📝 路线图

- [ ] 多账本支持（个人/家庭/工作）
- [ ] 数据导出（Excel/PDF）
- [ ] 图表可视化
- [ ] 预算提醒
- [ ] 分类自定义
- [ ] 快捷键唤起

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License

---

**开源记账，轻松管理每一笔消费 💰**