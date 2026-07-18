# Codex Themes Desktop：Codex 桌面端主题管理器

<p align="center">
  <strong>发现、导入、切换与恢复 Codex 主题，让桌面工作空间更符合你的审美。</strong>
</p>

<p align="center">
  <a href="https://codexthemes.app/">官方主题商城</a> ·
  <a href="#下载-codex-themes-desktop">下载软件</a> ·
  <a href="#快速上手">快速上手</a> ·
  <a href="#为-codex-制作自己的主题">制作主题</a> ·
  <a href="#本地开发">本地开发</a>
</p>

> Codex Themes Desktop 是独立的开源社区项目，不隶属于 OpenAI，也未获得 OpenAI 的认可、赞助或背书。Codex 及相关商标归其权利人所有。

Codex Themes Desktop 是一款面向 macOS 的图形化 Codex 主题管理工具。它把主题商城、本地主题库、主题导入、主题切换和恢复官方外观集中到一个简单界面中。应用通过仅绑定本机回环地址的 Chrome DevTools Protocol（CDP）加载主题，不修改 Codex 官方应用包、`app.asar` 或代码签名。

## 下载 Codex Themes Desktop

> **macOS 下载入口已预留。** GitHub Release 上传对应的同名安装包后，以下按钮会自动指向最新版本，无需再次修改 README。

| 系统与芯片 | 下载 | 状态 |
| --- | --- | --- |
| macOS · Apple Silicon（M1 / M2 / M3 / M4 及后续 Apple 芯片） | **[下载 Apple Silicon 版 DMG](https://github.com/NBchitu/CodexThemes-App/releases/latest/download/Codex-Themes-macOS-Apple-Silicon.dmg)** | Release 安装包待上传 |
| macOS · Intel 芯片 | **[下载 Intel 版 DMG](https://github.com/NBchitu/CodexThemes-App/releases/latest/download/Codex-Themes-macOS-Intel.dmg)** | Release 安装包待上传 |
| Windows · x64 | 暂未开放下载 | **极速开发中** |

不知道自己的 Mac 使用哪种芯片？点击屏幕左上角 ** → 关于本机**：看到“芯片 Apple M…”请选择 Apple Silicon 版；看到“处理器 Intel…”请选择 Intel 版。

也可以前往 [GitHub Releases](https://github.com/NBchitu/CodexThemes-App/releases) 查看全部版本、更新说明和文件校验信息。安装包正式签名、公证并上传前，请不要把上述预留链接视为已经发布。

## 界面预览

### 软件总览与在线主题商城

打开 Codex Themes Desktop 后，可以从统一侧边栏进入主题发现、主题管理、主题创建和设置页面，并一键前往在线主题商城。

![Codex Themes Desktop 软件总览与在线主题商城](https://raw.githubusercontent.com/NBchitu/CodexThemes-App/main/docs/screenshots/codex-themes-desktop-overview.png)

### 本地主题管理

“My Themes”集中展示内置主题和本地导入主题。用户可以打开主题目录、导入解压后的主题文件夹，并选择主题应用到 Codex。

![Codex Themes Desktop 本地主题管理界面](https://raw.githubusercontent.com/NBchitu/CodexThemes-App/main/docs/screenshots/codex-themes-theme-library.png)

### 创建主题引导

“Create”页面把主题制作拆成准备图片与指南、发送给 Codex、导入并应用三个步骤，即使不熟悉配置文件也能跟随完成。

![Codex Themes Desktop 创建 Codex 主题引导页面](https://raw.githubusercontent.com/NBchitu/CodexThemes-App/main/docs/screenshots/codex-themes-create-theme-guide.png)

## 它能做什么

- 从 [Codex Themes 官方主题商城](https://codexthemes.app/) 浏览主题。
- 导入已经解压的本地主题文件夹，并进行基础结构校验。
- 在“我的主题”中查看内置主题和导入主题。
- 一键应用主题，并在原生环境中验证主题是否真正生效。
- 在需要时恢复 Codex 官方外观，已下载和导入的主题会继续保留。
- 导出主题制作指南，配合 Codex 从喜欢的图片创建新主题。
- 在浅色、深色与跟随系统三种应用外观之间切换。

当前版本以 macOS 为主，并分别为 Apple Silicon 与 Intel 芯片准备发行入口。Windows 版本正在极速开发中；中文界面、日文界面、自动更新、签名与公证发行包仍在规划或发布准备阶段，详见[交付状态](docs/development/delivery-status.md)。

## 快速上手

### 1. 安装与打开

目前仓库提供源代码、开发构建流程和顶部的 Release 下载预留入口。普通用户请在 Releases 页面出现经过 Apple Developer ID 签名、公证的正式版本后再下载；不要运行来源不明的二次打包版本。

首次打开后，左侧会看到四个入口：

- **Discover**：打开官网主题商城并导入下载好的主题。
- **My Themes**：管理和使用本机主题。
- **Create**：按向导制作自己的主题。
- **Settings**：调整外观、启动偏好并恢复官方外观。

### 2. 从主题商城安装主题

这是最适合第一次使用的流程：

1. 打开 **Discover**，点击 **Browse theme gallery**。
2. 浏览器会打开 [https://codexthemes.app/](https://codexthemes.app/)。
3. 选择喜欢的主题，下载 ZIP 压缩包。
4. 在 Finder 中双击 ZIP，把它解压成一个普通文件夹。
5. 回到应用，点击 **Import extracted theme**。
6. 选择刚刚解压的主题文件夹。
7. 导入成功后进入 **My Themes**，打开主题详情并点击 **Apply**。

主题文件夹通常长这样：

```text
my-theme/
├── theme.json       # 主题配置，必需
├── background.jpg   # 背景图，必需
├── preview.jpg      # 商城或主题库预览图，推荐
└── README.md        # 作者说明，可选
```

如果导入失败，请先确认选择的是“解压后的文件夹”，而不是 ZIP 文件，也不要只选择其中的 `theme.json`。

### 3. 切换主题

1. 打开 **My Themes**。
2. 选择一个主题查看详情。
3. 点击 **Apply**。
4. 如果 Codex 需要重新启动，先保存尚未发送的输入，再按提示继续。
5. 等待应用完成验证；只有验证成功后，界面才会把该主题标记为当前主题。

主题切换失败时，应用会尽量保留上一个可用主题。请根据错误提示重试，不要把“浏览器预览模式”中的界面展示误认为主题已经应用到 Codex。

### 4. 恢复 Codex 官方外观

如果主题不合适或 Codex 更新后显示异常：

1. 打开 **Settings**。
2. 找到 **Original appearance**。
3. 点击恢复按钮并确认。
4. 应用会停止受管理的主题注入；必要时会重新启动 Codex。

恢复外观不会删除你的主题文件。之后仍可从 **My Themes** 再次使用它们。

## 为 Codex 制作自己的主题

不熟悉配置文件也没关系，应用内的 **Create** 页面把流程拆成了三步：

1. 准备一张 JPG、PNG 或 WebP 图片，并点击 **Save creation guide** 保存制作指南。
2. 在 Codex 对话中同时附上图片和指南，再粘贴应用提供的说明文字。
3. 下载 Codex 生成的主题 ZIP，解压后点击 **Import extracted theme**。

导入成功后，应用会把主题复制到受管理的主题目录并尝试应用。公开分享主题前，请确认你拥有背景图片、人物肖像、字体、Logo 和其他素材所需的使用与再分发权利。

更完整的字段与安全要求见[主题制作指南](docs/product/codex-theme-creation-guide.md)。

## 隐私与安全

- 主题运行使用本机回环地址，不应监听公网地址。
- 应用不会修改 Codex 官方安装包或代码签名。
- 导入主题前会检查清单和文件结构，并拒绝不受支持的可执行内容。
- 请只从可信来源下载主题，并在分享诊断信息前移除密钥、私人对话和本地路径。
- CDP 启用期间，不要运行来源不明的本机程序。

## 本地开发

### 环境要求

- macOS（原生主题桥当前仅支持 macOS）
- Node.js 20 或更高版本
- npm
- Rust stable 与 Cargo
- Xcode Command Line Tools
- Tauri 2 所需的 macOS 系统依赖

安装依赖并启动前端预览：

```bash
npm install
npm run dev
```

浏览器访问 `http://127.0.0.1:1420`。浏览器模式只用于查看和开发界面，不能导入、应用或恢复真实 Codex 主题。

启动 Tauri 原生开发应用：

```bash
npm run tauri -- dev
```

运行自动化检查：

```bash
npm test
npm run build
```

构建 macOS 应用：

```bash
npm run tauri -- build --bundles app
```

> 当前原生打包配置仍从上层工作区读取受管理的 macOS 主题引擎资源。单独克隆本仓库时，前端开发与测试可正常进行，但原生应用打包需要先完成资源独立化。这是当前开源版本的已知限制，不应把缺少资源的构建标记为可发布版本。

## 技术栈与项目结构

- Tauri 2 + Rust：原生窗口与受限平台命令
- React 19 + TypeScript：应用界面
- Vite 6：开发与前端构建
- Tailwind CSS 4：样式系统
- Zustand：本地应用状态
- Zod：主题清单校验
- Vitest：单元测试

```text
src/                  React 应用、主题领域模型与平台桥
src-tauri/            Tauri/Rust 原生宿主
resources/            随应用使用的指南资源
docs/product/         产品与主题制作规范
docs/development/     开发计划和交付状态
docs/design/          视觉约束
docs/screenshots/     README 截图及占位图
```

## 参与贡献

欢迎提交 Issue 和 Pull Request。开始修改前，请先阅读[功能规范](docs/product/functional-spec.md)与[开发计划](docs/development/development-plan.md)，并保持以下原则：

- 不伪造主题应用成功；必须由原生桥验证结果。
- 不修改 Codex 官方应用包、`app.asar` 或签名。
- 不把图片、密钥、对话内容或隐私数据加入日志和诊断报告。
- 新功能应包含对应测试，并确保 `npm test` 与 `npm run build` 通过。

## 致谢

感谢以下开源项目及其维护者为 Codex 主题与换肤工具生态所做的探索和贡献，也为本项目的设计与实现提供了宝贵参考：

- [Fei-Away/Codex-Dream-Skin](https://github.com/Fei-Away/Codex-Dream-Skin)
- [Finderchangchang/codex-autoskin](https://github.com/Finderchangchang/codex-autoskin)

请尊重这些项目各自的许可证、版权声明与使用边界。本致谢仅表示对开源贡献的认可，不代表上述项目与 Codex Themes Desktop 或 OpenAI 存在官方合作、认可或背书关系。

## 许可证与声明

源代码采用 [MIT License](LICENSE) 发布。第三方依赖及主题素材可能适用各自的许可证；MIT License 不自动授予任何图片、人物肖像、商标或品牌素材的使用权。

官方主题商城：[https://codexthemes.app/](https://codexthemes.app/)
