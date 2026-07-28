# THEME-SPEC v3.1 — Codex Dream Skin 单一发布包规范（写给 Agent 读）

> 读者设定：你是用户的 Codex / Claude Agent。用户提供一张参考图并说“给我的 Codex 做个这样的皮肤”。
>
> 你的任务是根据用户指定的中文名和参考图，生成一个可验证、可直接交给 CodexThemes.app 发布器的 `<theme-slug>.release.zip`。不要修改皮肤引擎源码。

## 0. 最重要的版本说明

当前主题格式使用 `schemaVersion: 1`。

旧版的 `meta`、`art`、`cards`、`composer`、`tokens` 以及 `--dream-*` CSS 变量格式已经停用，不得继续生成。

Dream Skin 运行时主题仍只包含两项核心内容：

1. `theme.json`：主题文案、图片引用和颜色配置。
2. `background.jpg`：清理后的主题背景图。

### 0.1 一次性交付是硬性要求

收到本规范、用户明确指定的主题中文名，以及原始背景图或参考图后，必须在同一次任务中完成：

1. 原样采用用户指定的主题中文名，并根据其语义生成英文 slug；
2. 生成或规范化无 UI 的 `background.jpg`；
3. 写入合法的 `theme.json`；
4. 创建 `<theme-slug>/` 运行时主题目录；
5. 生成只含运行时文件的 `<theme-slug>.zip`；
6. 生成三语言网站发布数据 `release.json`；
7. 使用图片模型生成呈现真实 Codex 主题应用效果的 `preview.webp`，再由它裁剪出缩略图和两种 Open Graph 图片；
8. 生成内部文件 SHA-256 清单 `checksums.json`；
9. 把全部发布文件封装为唯一交付物 `<theme-slug>.release.zip`；
10. 重新读取内外两层 ZIP，执行完整性、安全性和一致性验证；
11. 在最终回复中只把 `.release.zip` 作为主要发布交付物，并提供可点击绝对路径及其 SHA-256。

只展示 JSON、只生成背景图、只创建主题文件夹、只生成内层主题 ZIP，或要求用户自行整理发布文件，都不算完成。主题中文名是唯一不得由模型自行推断的必需输入；用户没有明确提供时必须先询问。颜色、构图、英文和日文展示名及短文案可根据中文名和图片语义生成一套克制、可用的结果。

不得把 ZIP 当作文本手写或伪造。必须先真实创建所有文件，再调用本机归档工具生成内层主题 ZIP 和外层发布 ZIP，并在交付前重新读取两个归档目录进行验证。

### 0.2 用户输入契约

用户应按以下格式提供任务，字段名称可以自然表达，但语义必须明确：

```text
主题中文名：金尘劲影
原始背景图：<附件或本地路径>
素材权利：<已确认可公开分发 / 尚未确认>
作者或来源：<可选；未知时写尚未提供>
```

命名规则：

* `theme.json.name` 和网站简体中文展示名必须逐字采用用户指定的中文名，不得润色、缩写、替换或根据图片重新命名。
* 模型只负责把中文名的语义转成适合 URL 的英文 slug，以及自然的英文、日文展示名。
* slug 使用 2～5 个简短英文语义词、小写 ASCII 和连字符，建议不超过 48 个字符。
* slug 应表达主题名称和视觉含义，例如 `金尘劲影` → `golden-dust-striker`。
* 不使用纯拼音，例如 `jin-chen-jin-ying`；不使用 `theme-01`、时间戳、随机数或无意义哈希。
* slug 中不重复加入 `codex`、`dream-skin`、`theme` 或 `preset`；`preset-` 只出现在 `theme.json.id`。
* slug 一经用于公开 URL 就必须保持稳定；重新发布同一主题不得擅自更名。
* 如果生成的 slug 与现有主题冲突，应增加一个有语义的英文限定词，而不是随机数字。
* 只有用户明确表示素材可以公开分发时，`release.json.rights.redistributionConfirmed` 才能写为 `true`；模型不得根据图片内容自行推断授权。

## 1. 交付目录格式

制作过程可以使用临时工作目录，但最终必须形成以下两层结构。

### 1.1 内层运行时主题 ZIP

```text
<theme-slug>/
  theme.json
  background.jpg

<theme-slug>.zip
```

内层 ZIP 是网站最终提供给用户下载和导入 Dream Skin 的文件。要求：

* `<theme-slug>` 必须使用 kebab-case，例如 `cream-cat-diary`。
* `theme.json` 中的 `id` 必须写成 `preset-<theme-slug>`。
* ZIP 解压后必须直接得到完整主题文件夹，不能多套一层无意义目录。
* ZIP 中只保留运行所需文件，不放原始截图、临时图、提示词或重复图片。
* 默认使用 `background.jpg`；若确需 PNG/WebP，`image` 字段必须与真实文件名完全一致。
* 网站发布器会从外层发布包中取出这个 ZIP，再上传为公开下载文件。

### 1.2 外层单一发布包

最终唯一主要交付物必须是：

```text
<theme-slug>.release.zip
├── release.json
├── checksums.json
├── packages/
│   └── <theme-slug>.zip
└── assets/
    ├── preview.webp
    ├── thumbnail.webp
    ├── og-wide.webp
    └── og-square.webp
```

外层 ZIP 解压后必须直接看到 `release.json`，不能再套 `<theme-slug>.release/` 目录。除上述文件外，不得包含主题源图、临时文件、提示词、`.DS_Store`、`__MACOSX`、日志或未声明文件。

图片要求：

| 文件 | 目标尺寸 | 用途 |
| --- | --- | --- |
| `preview.webp` | 1600 × 900 | 图片模型生成的 Codex 主题应用效果图；用于主题列表与详情页主预览 |
| `thumbnail.webp` | 480 × 270 | Hero 和紧凑卡片缩略图 |
| `og-wide.webp` | 1200 × 630 | Open Graph 横图 |
| `og-square.webp` | 1200 × 1200 | 方形社交预览 |

`preview.webp` 不能只是 `background.jpg` 的裁切图。必须把最终背景图作为主题素材，并参考 §5.4 指定的 Codex UI 效果图，通过图片模型生成一张能直观看出主题实际应用状态的完整效果图。`thumbnail.webp`、`og-wide.webp` 和 `og-square.webp` 必须只从最终 `preview.webp` 进行尺寸调整和内容感知裁切，不得再次调用图片模型重新生成，也不得重新叠加文字或改变主题内容。

生成的 `preview.webp` 是 Publisher Studio 的候选网站展示图，仍需管理员在发布前目视审核；Studio 可以接受管理员替换后的版本。它不能放入内层运行时主题 ZIP，也不能替代 `background.jpg`。若当前环境无法可靠输出 WebP，可以统一改用 JPEG，但必须同步修改文件扩展名、`release.json.assets` 和归档清单，禁止扩展名与真实格式不一致。

### 1.3 release.json 网站发布数据

必须生成以下结构的 `release.json`：

```json
{
  "schemaVersion": 1,
  "slug": "golden-dust-striker",
  "themeId": "preset-golden-dust-striker",
  "sourceName": "金尘劲影",
  "displayName": {
    "en": "Golden Dust Striker",
    "zh": "金尘劲影",
    "ja": "ゴールデンダスト・ストライカー"
  },
  "summary": {
    "en": "Golden light and swift motion shape a focused, powerful workspace.",
    "zh": "鎏金光影与迅捷动势，营造专注而有力量的工作空间。",
    "ja": "金色の光と俊敏な動きが、集中力と力強さのある空間をつくります。"
  },
  "description": {
    "en": "A dark cinematic theme built around warm gold, deep brown, and energetic motion.",
    "zh": "以暖金、深棕与利落动势构成的深色电影感主题。",
    "ja": "温かな金色と深いブラウン、躍動感のある構図を組み合わせたダークテーマです。"
  },
  "appearance": "dark",
  "categories": ["dark", "cinematic"],
  "tags": ["gold", "dynamic", "sport"],
  "featured": false,
  "package": "packages/golden-dust-striker.zip",
  "runtimeBackground": "background.jpg",
  "assets": {
    "preview": "assets/preview.webp",
    "thumbnail": "assets/thumbnail.webp",
    "ogWide": "assets/og-wide.webp",
    "ogSquare": "assets/og-square.webp"
  },
  "translationStatus": {
    "zh": "user-provided",
    "en": "generated-draft",
    "ja": "generated-draft"
  },
  "rights": {
    "author": "尚未提供",
    "source": "user-provided",
    "redistributionConfirmed": false,
    "reviewStatus": "pending"
  }
}
```

约束：

* `sourceName`、`displayName.zh` 和 `theme.json.name` 必须完全相同。
* 英文、日文展示名应让母语用户理解主题含义，不机械使用拼音。
* `summary` 每种语言只写一句简短描述；`description` 保持一至两句，不添加下载量、验证状态或兼容性等未经确认的信息。
* `appearance` 只能是 `light` 或 `dark`，根据主题颜色而不是背景图片亮度的局部区域判断。
* `categories` 和 `tags` 使用网站约定的英文机器值，用户可见文字由网站翻译。
* 机器生成的英文和日文默认标记为 `generated-draft`；经人工确认后才改为 `reviewed`。
* `featured` 默认是 `false`，不得让新主题自动占据首页 Hero。
* 所有路径必须是外层 ZIP 根目录下的 POSIX 相对路径，不能使用绝对路径、反斜杠或 `..`。
* 权利未确认时仍可生成结构完整的发布包，但 `rights.reviewStatus` 必须是 `pending`，后续发布器应阻止公开发布。
* 只有用户明确确认可公开分发，并提供必要作者或来源信息时，才可把 `redistributionConfirmed` 设为 `true`、`reviewStatus` 设为 `ready`。

### 1.4 checksums.json

`checksums.json` 必须记录外层发布包中除自身以外所有文件的 SHA-256：

```json
{
  "algorithm": "sha256",
  "files": {
    "release.json": "<sha256>",
    "packages/golden-dust-striker.zip": "<sha256>",
    "assets/preview.webp": "<sha256>",
    "assets/thumbnail.webp": "<sha256>",
    "assets/og-wide.webp": "<sha256>",
    "assets/og-square.webp": "<sha256>"
  }
}
```

`checksums.json` 不记录自身，也不记录外层 `.release.zip`，避免自引用。外层 ZIP 的 SHA-256 必须在压缩完成后单独计算并写入最终回复。

## 2. theme.json 标准格式

必须严格使用以下结构：

```json
{
  "schemaVersion": 1,
  "id": "preset-cream-cat-diary",
  "name": "奶油猫咪日记",
  "brandSubtitle": "CODEX DREAM SKIN",
  "tagline": "猫咪在旁，思路更稳，代码更美。",
  "projectPrefix": "选择项目 · ",
  "projectLabel": "♡  选择项目",
  "statusText": "CREAM CAT ONLINE",
  "quote": "CODE GENTLY, CREATE WONDERFULLY",
  "image": "background.jpg",
  "colors": {
    "background": "#fffaf3",
    "panel": "#fffdf9",
    "panelAlt": "#f8eee3",
    "accent": "#d77d7f",
    "accentAlt": "#eca09b",
    "secondary": "#d9aa76",
    "highlight": "#f4c7aa",
    "text": "#4b382e",
    "muted": "#9b7f70",
    "line": "rgba(215, 125, 127, .24)"
  },
  "promoTitle": "和奶油猫咪一起写代码",
  "promoSub": "CodexThemes.app",
  "promoUrl": "https://codexthemes.app"
}
```

除非引擎规范明确升级，否则不要增加示例之外的顶层字段或 `colors` 子字段。

## 3. 字段说明

### 3.1 基础标识

| 字段              | 类型     | 要求                                   |
| --------------- | ------ | ------------------------------------ |
| `schemaVersion` | number | 固定为 `1`                              |
| `id`            | string | `preset-<theme-slug>`，仅使用小写英文、数字和连字符 |
| `name`          | string | 用户可见的主题名称，建议 2～8 个中文字符               |
| `image`         | string | 背景图片文件名，默认 `background.jpg`          |

### 3.2 品牌与界面文案

| 字段              | 用途          | 写作要求                          |
| --------------- | ----------- | ----------------------------- |
| `brandSubtitle` | 品牌副标题       | 默认 `CODEX DREAM SKIN`         |
| `tagline`       | 首页主题描述      | 一句话说明主题氛围及使用感受，避免夸张营销语        |
| `projectPrefix` | 已选择项目时的前缀   | 默认 `选择项目 · `，保留末尾空格           |
| `projectLabel`  | 未选择项目时的按钮文案 | 可使用一个与主题匹配的简单符号，不使用复杂 Emoji 串 |
| `statusText`    | 主题状态文案      | 简短大写英文，例如 `CREAM CAT ONLINE`  |
| `quote`         | 装饰性引语       | 简短大写英文，不超过一行                  |

文案不得包含换行、HTML、Markdown 或脚本内容。

### 3.3 推广信息

| 字段           | 用途      | 要求                   |
| ------------ | ------- | -------------------- |
| `promoTitle` | 推广区域主标题 | 应与主题或站点品牌相关          |
| `promoSub`   | 推广区域副标题 | 可填写品牌名或短域名           |
| `promoUrl`   | 点击链接    | 使用完整的 `https://` URL |

公开主题默认使用 CodexThemes.app 品牌信息。除非用户明确提供并授权，不得擅自加入第三方广告、邀请码或联盟链接。

## 4. colors 完整字段

`colors` 必须且只能包含以下 10 个字段：

| 字段           | 控制内容       | 取色建议                                |
| ------------ | ---------- | ----------------------------------- |
| `background` | 页面整体背景     | 整套主题最深或最浅的基础底色                      |
| `panel`      | 主面板、卡片底色   | 与背景有清晰但柔和的层级差                       |
| `panelAlt`   | 次级面板、悬停状态  | 位于 `background` 与 `panel` 之间或稍有色相变化 |
| `accent`     | 主强调色       | 从参考图最有识别度的颜色中提取                     |
| `accentAlt`  | 主强调色的亮/暗变体 | 与 `accent` 同色系，用于渐变或状态变化            |
| `secondary`  | 辅助强调色      | 从参考图第二视觉色中提取                        |
| `highlight`  | 高亮和光泽      | 比主强调色更亮，不能刺眼                        |
| `text`       | 主要文字       | 必须与 `background`、`panel` 保持高可读性     |
| `muted`      | 次要文字       | 弱于 `text`，但仍需清楚可读                   |
| `line`       | 边框和分隔线     | 建议使用低透明度 `rgba(...)`                |

### 4.1 浅色主题建议

* `background` 接近带主题色倾向的白色，而不是纯白。
* `panel` 可以略亮于或略深于背景，但必须看得出层级。
* `text` 使用带主题色相的深色，不推荐纯黑 `#000000`。
* `muted` 不能浅到难以阅读。
* `accent` 应控制饱和度，避免常见的廉价紫粉 AI 感。
* `line` 透明度通常在 `.16`～`.30` 之间。

### 4.2 深色主题建议

* `background` 使用带色相的近黑色，不推荐纯黑。
* `panel` 和 `panelAlt` 必须比背景稍亮。
* `text` 使用带暖/冷倾向的近白色，不推荐刺眼纯白。
* 高饱和强调色只用于按钮、状态和小面积装饰。
* 不能依赖背景图来保证文字可读性，颜色本身必须成立。

## 5. 背景图制作规则

### 5.1 不得直接使用完整 UI 截图

用户提供的参考图经常包含：

* 菜单栏、侧边栏和窗口边框；
* 按钮、输入框和卡片；
* 中文或英文标题；
* Logo、水印和状态图标。

这些内容不得直接出现在 `background.jpg` 中，否则会形成无法点击的假界面和文字鬼影。

正确做法是：

1. 把参考图当作美术方向，而不是最终背景。
2. 重新生成或清理为独立场景图。
3. 保留主题主体、材质、配色、光线和氛围。
4. 删除所有 UI、文字、Logo、水印、边框和按钮。
5. 为真实 Codex 文案与控件保留干净空间。

### 5.2 构图要求

* 必须输出 `1920 × 1080` 的 16:9 宽屏横图。
* 主体通常放在右侧 35%～45% 区域。
* 左侧至少保留约 50% 的低细节区域，供标题和项目控件使用。
* 不要在四角堆放高对比度小装饰。
* 不要生成碎片化贴纸、密集爪印、漂浮小图标等会影响操作的元素。
* 图片中不得出现任何可读文字。
* 背景本身应是一张完整艺术图，不能伪装成 Codex 界面截图。

### 5.3 从参考图生成背景时的提示词重点

提示词必须明确包含：

```text
只重建参考图的场景、主体、配色、材质和光线。
移除整个软件 UI、菜单、侧边栏、卡片、按钮、输入框、图标、边框、Logo、水印和全部文字。
输出独立的宽屏背景插画，不是 UI mockup。
主体位于右侧，左侧保留大面积干净留白，供真实界面内容叠加。
不得出现任何可读文字或假控件。
```

### 5.4 生成 Codex 主题应用效果图

背景图完成后，必须再调用图片模型独立生成 `preview.webp`。生成时同时提供以下两张输入图片，并明确它们不同的用途：

1. 最终 `background.jpg`：**主题内容参考**。保留其主体、构图识别度、配色和氛围，让效果图呈现这个主题，而不是重新设计另一个主题。
2. `https://assets.codexthemes.app/themes/theme-preview-demo.png`：**Codex UI 构图参考**。只参考窗口比例、侧栏、顶部工具栏、主内容区、功能卡片和底部输入区的层级与布局，不复制其中的星际主题、插画、名称、Logo、版本号或其他示例文案。

生成前必须下载并实际读取在线 UI 参考图，再把它和最终背景图一起传给图片模型。不得只在提示词中粘贴 URL 并假设模型已经读取图片。在线参考图只是生成过程的输入，不得放入内层或外层 ZIP。

效果图必须满足：

* 输出为 16:9，最终规范化到 `1600 × 900`。
* 呈现完整、正视、无透视畸变的 Codex 桌面应用界面，不添加设备外壳、桌面场景或浏览器边框。
* UI 结构参考示例图，但背景、面板、描边、高亮、按钮和图标颜色必须来自当前 `theme.json.colors`。
* 主内容区域使用最终 `background.jpg` 的主题视觉；不得退回示例图中的火箭、木屋、星空或金色探索主题，除非它们本来就是当前主题内容。
* 与主题相关的可见文字必须替换为当前主题的数据，优先使用 `name`、`brandSubtitle`、`tagline`、`projectLabel`、`statusText`、`quote`、`promoTitle` 和 `promoSub`；不得保留示例图的“星际探索主题皮肤”“探索者”等文字。
* 文字应简短、清晰、无乱码。图片模型不得自行添加版本号、下载量、评分、官方认证、品牌合作、用户数量或兼容性声明。
* 不复制 OpenAI 或其他第三方 Logo；只使用中性的界面图标和项目占位内容。
* 效果图应让用户第一眼能判断主题在 Codex 中的整体氛围、文字可读性和控件层级，而不是把背景图藏在装饰性 mockup 中。

建议使用以下图片模型提示结构：

```text
Use case: ui-mockup
Asset type: Codex theme website preview
Input image 1: final background.jpg, the current theme artwork and visual identity
Input image 2: theme-preview-demo.png, layout and UI hierarchy reference only
Primary request: Generate a polished front-facing Codex desktop app theme preview using the current theme.
Keep from input 1: subject identity, composition, palette, material, lighting and atmosphere.
Keep from input 2 only: window proportions and the hierarchy of sidebar, toolbar, main content, cards and composer.
Replace all theme-specific text and styling with the supplied current theme fields and colors.
Text (verbatim): use only the supplied short values from name, brandSubtitle, tagline, projectLabel, statusText, quote, promoTitle and promoSub.
Constraints: 16:9; complete straight-on app window; readable hierarchy; accurate theme identity; no perspective; no device frame.
Avoid: copied demo artwork or demo wording, gibberish, extra branding, OpenAI logo, version numbers, ratings, downloads, verification badges, watermark, browser chrome.
```

生成后必须目视检查文字、主题身份和 UI 完整性。若存在示例主题残留、明显乱码、截断窗口、错误主题名或虚构信息，必须针对错误重新生成；不得仅靠裁切隐藏问题。

## 6. 从参考图映射主题的工作流程

### 第一步：识别视觉方向

从参考图中确定：

* 锁定用户指定的主题中文名，不根据图片重新命名；
* 根据中文名语义生成 SEO 友好的英文 slug；
* 明亮或深色模式；
* 主色、辅助色和高亮色；
* 背景主体及其适合的位置；
* 主题气质，例如治愈、复古、科幻、极简或电影感。

### 第二步：生成干净背景

* 删除参考图中的 UI 和文字。
* 保持主体识别度及主要氛围。
* 左侧预留真实界面区域。
* 导出为 `background.jpg`。

### 第三步：编写 theme.json

* 严格复制 §2 的字段结构。
* 替换主题 ID、名称、文案和颜色。
* `image` 与真实背景文件名一致。
* 不使用任何旧版 `--dream-*` token。

### 第四步：生成 release.json

* 按 §1.3 生成 `release.json`。
* 中文名严格使用用户输入。
* 英文、日文名应简洁自然，并与 slug 表达同一主题含义。
* 三种语言的 `summary` 保持一句话，`description` 保持一至两句，不机械重复主题名称。
* 未经用户明确确认，不得把权利状态或平台验证状态写成已通过。

### 第五步：生成网站图片

* 按 §5.4 将最终 `background.jpg` 与指定 UI 参考图一起传给图片模型，生成 Codex 主题应用效果图 `preview.webp`。
* 目视确认 `preview.webp` 使用当前主题的背景、颜色与文字，没有复制示例主题内容、乱码或虚构信息。
* 只从最终 `preview.webp` 裁剪和缩放生成 `thumbnail`、`og-wide` 和 `og-square`；三个派生文件不得再次交给图片模型重绘。
* 图片尺寸和真实格式必须符合 §1.2，并与 `release.json.assets` 一致。裁切时优先保留主题主体、主题名称和关键 Codex UI，不得拉伸画面。

### 第六步：运行时结构校验

至少检查：

```text
schemaVersion === 1
id === "preset-<theme-slug>"
所有必需顶层字段存在
colors 的 10 个字段完整
不存在多余的旧版字段
image 指向的文件真实存在
theme.json 是合法 JSON
promoUrl 是完整 https URL
```

### 第七步：生成内层主题 ZIP

ZIP 中应是：

```text
<theme-slug>/theme.json
<theme-slug>/background.jpg
```

不要把 ZIP 自己放进主题文件夹，也不要包含系统隐藏文件。

从包含 `<theme-slug>/` 的父目录执行打包。macOS / Linux 优先使用：

```bash
/usr/bin/zip -X -r "<theme-slug>.zip" "<theme-slug>" \
  -x "*/.DS_Store" "__MACOSX/*" "*/__MACOSX/*"
```

如果系统没有 `/usr/bin/zip`，可使用等价的标准 ZIP 工具，但归档内路径必须保持完全相同。不得使用会多套一层交付目录的压缩方式。

### 第八步：组装发布目录并生成 checksums.json

先创建一个临时发布目录，结构必须与 §1.2 一致：

```text
<release-root>/
├── release.json
├── packages/<theme-slug>.zip
└── assets/{preview,thumbnail,og-wide,og-square}.webp
```

在 `<release-root>` 的父目录执行以下命令生成校验清单：

```bash
node -e '
const crypto = require("crypto");
const fs = require("fs");
const path = require("path");
const root = path.resolve(process.argv[1]);
const release = JSON.parse(fs.readFileSync(path.join(root, "release.json"), "utf8"));
const files = ["release.json", release.package, ...Object.values(release.assets)];
const output = { algorithm: "sha256", files: {} };
for (const relative of files) {
  if (typeof relative !== "string" || path.isAbsolute(relative) || relative.split("/").includes("..")) throw new Error(`Unsafe release path: ${relative}`);
  const target = path.join(root, relative);
  if (!fs.statSync(target).isFile()) throw new Error(`Missing release file: ${relative}`);
  output.files[relative] = crypto.createHash("sha256").update(fs.readFileSync(target)).digest("hex");
}
fs.writeFileSync(path.join(root, "checksums.json"), `${JSON.stringify(output, null, 2)}\n`);
console.log(`Checksummed ${files.length} files`);
' "<release-root>"
```

### 第九步：生成外层单一发布包

从 `<release-root>` 内执行归档，确保外层 ZIP 没有多套目录：

```bash
(cd "<release-root>" && /usr/bin/zip -X -r "../<theme-slug>.release.zip" \
  release.json checksums.json packages assets \
  -x "*/.DS_Store" "__MACOSX/*" "*/__MACOSX/*")
```

### 第十步：交付前自动验证

完成内外两层打包后必须实际执行检查，不能只声称已验证。先验证工作目录中的配置关系：

```bash
node -e '
const fs = require("fs");
const path = require("path");
const slug = process.argv[1];
const dir = path.resolve(slug);
const configPath = path.join(dir, "theme.json");
const theme = JSON.parse(fs.readFileSync(configPath, "utf8"));
const releaseRoot = path.resolve(process.argv[2]);
const release = JSON.parse(fs.readFileSync(path.join(releaseRoot, "release.json"), "utf8"));
const required = ["schemaVersion", "id", "name", "brandSubtitle", "tagline", "projectPrefix", "projectLabel", "statusText", "quote", "image", "colors", "promoTitle", "promoSub", "promoUrl"];
const colorKeys = ["background", "panel", "panelAlt", "accent", "accentAlt", "secondary", "highlight", "text", "muted", "line"];
for (const key of required) if (!(key in theme)) throw new Error(`Missing field: ${key}`);
if (theme.schemaVersion !== 1) throw new Error("schemaVersion must be 1");
if (theme.id !== `preset-${slug}`) throw new Error("Theme id does not match slug");
if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(slug)) throw new Error("Invalid kebab-case slug");
if (!theme.colors || Object.keys(theme.colors).sort().join(",") !== colorKeys.sort().join(",")) throw new Error("colors must contain exactly 10 supported keys");
if (!fs.existsSync(path.join(dir, theme.image))) throw new Error("Referenced background image is missing");
if (!/^https:\/\//.test(theme.promoUrl)) throw new Error("promoUrl must use https");
if (release.schemaVersion !== 1 || release.slug !== slug || release.themeId !== theme.id) throw new Error("Release identity does not match theme package");
if (release.sourceName !== theme.name || release.displayName?.zh !== theme.name) throw new Error("User-provided Chinese name was changed");
if (!release.displayName?.en || !release.displayName?.ja) throw new Error("Localized display names are missing");
if (release.package !== `packages/${slug}.zip` || release.runtimeBackground !== theme.image) throw new Error("Release file references do not match package");
for (const locale of ["en", "zh", "ja"]) {
  if (!release.summary?.[locale] || !release.description?.[locale]) throw new Error(`Missing ${locale} website copy`);
}
console.log(`Validated ${theme.id}`);
' "<theme-slug>" "<release-root>"

unzip -Z -1 "<theme-slug>.zip"
unzip -Z -1 "<theme-slug>.release.zip"
shasum -a 256 "<theme-slug>.release.zip"
```

内层归档清单必须只出现以下三个条目（图片扩展名按实际情况替换）：

```text
<theme-slug>/
<theme-slug>/theme.json
<theme-slug>/background.jpg
```

外层归档清单必须且只能对应以下内容：

```text
release.json
checksums.json
packages/
packages/<theme-slug>.zip
assets/
assets/preview.webp
assets/thumbnail.webp
assets/og-wide.webp
assets/og-square.webp
```

最后必须把外层 ZIP 解压到新的临时目录，重新计算 `checksums.json` 中列出的每个 SHA-256，并确认全部一致。只验证制作工作目录、不重新读取最终 ZIP，不算完成。

如果检查失败，必须修复并重新打包；不得交付带警告的半成品。

## 7. 验收清单

交付前逐项确认：

1. `theme.json` 能被标准 JSON 解析器读取。
2. `schemaVersion` 为数字 `1`，不是字符串 `"1"`。
3. `id` 使用 `preset-` 前缀并与文件夹 slug 对应。
4. 顶层字段与 §2 一致，没有旧版字段。
5. `colors` 恰好包含规定的 10 个字段。
6. 所有颜色值均为合法 CSS 颜色。
7. 主要文字与面板背景有足够对比度。
8. `image` 与背景文件名、大小写和扩展名完全一致。
9. 背景图无 UI、无文字、无水印、无假按钮。
10. 背景图尺寸为 `1920 × 1080`，主体没有遮挡真实标题、项目选择器和主要操作区域。
11. 内层 ZIP 路径正确，解压后可直接得到主题文件夹。
12. 内层 ZIP 中没有原始参考截图、网站元数据或无关文件。
13. 内层和外层 ZIP 均已真实生成，能够被 `unzip` 读取，不是空文件或仅改扩展名。
14. `release.json` 的包路径、图片路径、slug 和 theme ID 与实际文件一致。
15. `preview` 是图片模型生成的当前主题 Codex 应用效果图，不是背景图的简单裁切；其中没有示例主题残留、明显乱码、错误主题名或虚构信息。
16. `thumbnail`、`og-wide` 和 `og-square` 均从最终 `preview` 裁剪生成，没有重新绘制、拉伸或叠加新内容。
17. 四张网站图片均存在，尺寸、扩展名和真实格式一致。
18. `theme.json.name` 与用户指定中文名逐字一致。
19. slug 是简短的英文语义 kebab-case，不是拼音或随机编号。
20. `release.json` 中英文、中文、日文展示名、简介和描述齐全，且中文名未被模型改写。
21. `checksums.json` 覆盖除自身外的全部外层文件，最终 ZIP 解压后复算结果一致。
22. 权利信息没有被模型擅自写成已确认；缺失时正确标记为 `pending`。
23. 外层 `.release.zip` SHA-256 已计算并在最终回复中提供。
24. 最终回复包含 `.release.zip` 的可点击绝对路径，用户无需再整理或压缩文件。

如果可以在目标 Codex Dream Skin 环境中运行，还应实际加载主题，检查首页、项目选择、输入框、聊天页面和窗口缩放状态。

## 8. 禁止事项

* 不得生成旧版 `meta`、`art`、`cards`、`composer`、`tokens` 结构。
* 不得生成任何 `--dream-*` CSS 变量。
* 不得修改皮肤引擎、注入器或 Codex 原始文件。
* 不得把完整 UI 效果图直接作为背景。
* 不得在背景图中保留可读文字、按钮、菜单、输入框或窗口边框。
* 不得用 `background.jpg` 的简单裁切冒充 `preview`；Codex UI 只允许出现在外层发布包的网站预览资产中。
* 不得在 `preview` 中照搬 UI 参考图的主题名称、示例插画、品牌标识或版本信息。
* 不得添加未经用户授权的推广链接、邀请码或第三方广告。
* 不得为了视觉效果牺牲正文和代码的可读性。
* 不得交付未经 JSON、文件引用和 ZIP 路径校验的主题包。
* 不得把 `release.json`、`checksums.json` 或网站派生图放进内层 Dream Skin 运行时 ZIP。
* 不得在没有用户明确声明的情况下把 `redistributionConfirmed` 写成 `true`。
* 不得只交付内层 `<theme-slug>.zip` 而遗漏外层 `<theme-slug>.release.zip`。

## 9. 最终交付说明模板

```text
主题发布包已按 THEME-SPEC v3.1 制作完成。

- 主题 ID：preset-<theme-slug>
- 用户指定中文名：<主题中文名>
- 网站英文名：<English display name>
- 网站日文名：<日本語の表示名>
- SEO URL：https://codexthemes.app/themes/<theme-slug>
- 背景图片：background.jpg
- 公开下载包：packages/<theme-slug>.zip
- 网站素材：preview、thumbnail、og-wide、og-square
- 配色：<一句话说明主色、辅助色和明暗模式>
- 权利状态：<ready / pending；不得虚构>
- 校验：内层主题、发布数据、网站图片、checksums 和外层 ZIP 均已通过
- 发布包 SHA-256：<release-zip-sha256>

发布包：[<theme-slug>.release.zip](</absolute/path/to/<theme-slug>.release.zip>)
```
