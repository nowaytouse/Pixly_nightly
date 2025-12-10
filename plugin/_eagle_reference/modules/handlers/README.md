# Handler 架構使用說明

## 概述

這個目錄包含了重構後的格式轉換處理器架構，基於 `BaseHandler` 抽象類別實現，大幅減少了代碼重複並提高了維護性。

## 架構設計

### 核心架構

```
handlers/
├── core/                    # 核心抽象類別
│   ├── BaseHandler.js      # 統一轉換邏輯與錯誤處理
│   └── types.js            # 轉換上下文類型定義
├── [具體格式handlers]       # 各格式的具體實作
│   ├── jpg.js
│   ├── png.js
│   ├── gif.js
│   └── ...
```

### 轉換流程

1. **上下文建立**: 根據輸入/輸出格式和檔案特性建立轉換上下文
2. **策略選擇**: 根據上下文選擇合適的轉換器優先順序
3. **轉換執行**: 依序嘗試轉換器直到成功或全部失敗
4. **錯誤處理**: 統一的錯誤處理和 fallback 機制

## 使用 BaseHandler

### 基本繼承

最簡單的 handler 實作：

```javascript
const BaseHandler = require('./core/BaseHandler');
const { ConverterType } = require('./core/types');

class JPG extends BaseHandler {
    constructor() {
        super('jpg');
    }

    /**
     * 定義此格式的轉換策略
     */
    getDefaultPlan(ctx) {
        return [ConverterType.CANVAS, ConverterType.FFMPEG];
    }
}

module.exports = new JPG();
```

### 可覆寫的方法

#### 1. `getDefaultPlan(ctx)` - 預設轉換策略

定義此格式的預設轉換器優先順序：

```javascript
getDefaultPlan(ctx) {
    // 回傳轉換器優先順序陣列
    return [ConverterType.CANVAS, ConverterType.FFMPEG];
}
```

#### 2. `needsAnimationCheck()` - 動畫檢測

某些格式需要檢測是否為動畫檔案：

```javascript
needsAnimationCheck() {
    return true; // GIF, WebP 等格式需要
}
```

#### 3. `getPlan(ctx)` - 完整策略覆寫

如果需要複雜的轉換邏輯，可以完全覆寫：

```javascript
getPlan(ctx) {
    if (ctx.out.ext === 'webp') {
        return this.getPlanForWebP(ctx);
    }
    return this.getDefaultPlan(ctx);
}

getPlanForWebP(ctx) {
    if (ctx.in.animated) {
        return [ConverterType.FFMPEG, ConverterType.CANVAS];
    } else {
        return [ConverterType.CANVAS, ConverterType.FFMPEG];
    }
}
```

#### 4. `convert(src, dest, options)` - 完全自訂轉換

對於需要特殊處理的格式（如 SVG、BMP->HEIC）：

```javascript
async convert(src, dest, options) {
    if (options.format === 'heic') {
        return await this.convertToHeic(src, dest, options);
    }
    // 其他格式使用基礎轉換邏輯
    return await super.convert(src, dest, options);
}
```

## 轉換器類型

### 可用的轉換器

```javascript
const { ConverterType } = require('./core/types');

// 可用的轉換器常數
ConverterType.CANVAS  // 'canvas'  - 瀏覽器 Canvas API
ConverterType.FFMPEG  // 'ffmpeg'  - FFmpeg 命令行工具
ConverterType.HEIC    // 'heic'    - HEIC/HEIF 專用轉換器
```

### 轉換器特性

| 轉換器 | 特點 | 適用情況 |
|--------|------|----------|
| **Canvas** | 速度快、支援基本格式 | 常見格式轉換（JPG, PNG, WebP 等） |
| **FFmpeg** | 功能強大、支援動畫 | 動畫處理、特殊格式、HDR 等 |
| **HEIC** | 專用轉換器 | 只能讀取 HEIC/HEIF 輸入 |

## 上下文 (Context) 系統

### 上下文結構

```javascript
const ctx = createContext('png', 'jpg', false, true);
// 結果：
{
  in: { 
    ext: 'png',           // 輸入格式
    animated: false,      // 是否動畫
    hasAlpha: true        // 是否有透明度
  },
  out: { 
    ext: 'jpg',           // 輸出格式  
    wantAnimated: false   // 是否需要動畫輸出
  }
}
```

### 使用上下文

```javascript
getDefaultPlan(ctx) {
    // 根據輸入特性決策
    if (ctx.in.animated) {
        return [ConverterType.FFMPEG];
    }
    
    // 根據輸出需求決策
    if (ctx.out.wantAnimated) {
        return [ConverterType.FFMPEG, ConverterType.CANVAS];
    }
    
    return [ConverterType.CANVAS, ConverterType.FFMPEG];
}
```

## 實際範例

### 範例 1：簡單格式 Handler

```javascript
// handlers/png.js
const BaseHandler = require('./core/BaseHandler');
const { ConverterType } = require('./core/types');

class PNG extends BaseHandler {
    constructor() {
        super('png');
    }

    getDefaultPlan(ctx) {
        // PNG 優先使用 Canvas（速度快），失敗時使用 FFmpeg
        return [ConverterType.CANVAS, ConverterType.FFMPEG];
    }
}

module.exports = new PNG();
```

### 範例 2：需要動畫檢測的 Handler

```javascript
// handlers/webp.js
const BaseHandler = require('./core/BaseHandler');
const { ConverterType } = require('./core/types');

class WEBP extends BaseHandler {
    constructor() {
        super('webp');
    }

    needsAnimationCheck() {
        return true; // WebP 需要檢測是否動畫
    }

    getDefaultPlan(ctx) {
        // WebP 轉換策略：優先 Canvas，失敗時 FFmpeg
        return [ConverterType.CANVAS, ConverterType.FFMPEG];
    }
}

module.exports = new WEBP();
```

### 範例 3：複雜邏輯 Handler

```javascript
// handlers/gif.js
const BaseHandler = require('./core/BaseHandler');
const { ConverterType } = require('./core/types');

class GIF extends BaseHandler {
    constructor() {
        super('gif');
    }

    needsAnimationCheck() {
        return true;
    }

    getPlan(ctx) {
        if (ctx.out.ext === 'webp') {
            return this.getPlanForWebP(ctx);
        }
        return this.getDefaultPlan(ctx);
    }

    getPlanForWebP(ctx) {
        if (ctx.in.animated) {
            // 動畫 GIF -> WebP：優先 FFmpeg
            return [ConverterType.FFMPEG, ConverterType.CANVAS];
        } else {
            // 靜態 GIF：優先 Canvas
            return [ConverterType.CANVAS, ConverterType.FFMPEG];
        }
    }

    getDefaultPlan(ctx) {
        if (ctx.in.animated) {
            return [ConverterType.FFMPEG, ConverterType.CANVAS]; // 動畫轉換，附 fallback
        } else {
            return [ConverterType.CANVAS, ConverterType.FFMPEG];
        }
    }
}

module.exports = new GIF();
```

### 範例 4：完全自訂轉換的 Handler

```javascript
// handlers/svg.js
const BaseHandler = require('./core/BaseHandler');
const File = require('../utils/file');
const fs = require('fs').promises;
const path = require('path');

class SVG extends BaseHandler {
    constructor() {
        super('svg');
        this.tempDir = `${__dirname}/temp`;
    }

    // 完全覆寫轉換方法
    async convert(src, dest, options) {
        if (this.canvasConverter.isSupportedFormat(options.format)) {
            // Canvas 支援：直接轉換
            await this.canvasConverter.convert(src, dest, options);
        } else {
            // Canvas 不支援：透過 PNG 中間格式
            await this.convertViaPng(src, dest, options);
        }
    }

    async convertViaPng(src, dest, options) {
        const tempFilePath = File.generateTempFilePath(this.tempDir, '.png');
        // ... 兩步驟轉換邏輯
    }
}

module.exports = new SVG();
```

## 錯誤處理

### 自動錯誤處理

BaseHandler 提供統一的錯誤處理：

- 轉換器失敗時自動嘗試下一個
- 輸出 `console.warn` 錯誤訊息
- 所有轉換器失敗時拋出最終錯誤

### 特殊錯誤情況

```javascript
// 禁用的格式會拋出明確錯誤
getPlan(ctx) {
    if (ctx.out.ext === 'heic' || ctx.out.ext === 'heif') {
        throw new Error(`輸出到 ${ctx.out.ext.toUpperCase()} 格式已被停用`);
    }
    // ...
}
```

## 最佳實踐

### 1. 命名規範

- Handler 類別名稱使用大寫：`PNG`, `JPG`, `WEBP`
- 檔案名稱使用小寫：`png.js`, `jpg.js`, `webp.js`
- 必須匯出實例：`module.exports = new PNG();`

### 2. 轉換策略設計

```javascript
// ✅ 好的做法：根據格式特性選擇
getDefaultPlan(ctx) {
    if (ctx.out.ext === 'gif') {
        return [ConverterType.FFMPEG, ConverterType.CANVAS];
    }
    return [ConverterType.CANVAS, ConverterType.FFMPEG];
}

// ❌ 避免：硬編碼複雜邏輯
getDefaultPlan(ctx) {
    if (ctx.in.ext === 'png' && ctx.out.ext === 'jpg' && ctx.in.hasAlpha) {
        return [ConverterType.CANVAS];
    }
    // ... 複雜的 if-else 邏輯
}
```

### 3. 效能考量

- Canvas 轉換器通常比 FFmpeg 快，優先使用
- FFmpeg 適合處理動畫和特殊格式
- HEIC 轉換器只能處理 HEIC 輸入

### 4. 向後相容性

- 必須保持 `convert(src, dest, options)` API 簽名
- 必須保持 `module.exports = new Handler()` 匯出模式
- 必須支援 `killProcess` 特殊回傳值

## 故障排除

### 常見問題

1. **轉換器選擇錯誤**
   - 檢查 `canConverterHandle()` 邏輯
   - 確認轉換器支援的格式組合

2. **動畫檢測失敗**
   - 確保 `needsAnimationCheck()` 回傳 `true`
   - 檢查 `ffmpegConverter.isAnimatedImage()` 調用

3. **錯誤處理不當**
   - 確保拋出的是 Error 物件
   - 檢查錯誤訊息是否清楚

### 偵錯技巧

```javascript
// 加入偵錯日誌
getPlan(ctx) {
    console.log(`Planning conversion: ${ctx.in.ext} -> ${ctx.out.ext}`, ctx);
    const plan = this.getDefaultPlan(ctx);
    console.log(`Selected plan:`, plan);
    return plan;
}
```

## 總結

這個 BaseHandler 架構提供了：

- **統一的轉換邏輯**：消除重複代碼
- **靈活的策略系統**：支援複雜的轉換決策
- **強大的錯誤處理**：自動 fallback 機制
- **易於維護**：新增格式只需要定義策略
- **向後相容**：保持現有 API 不變

透過這個架構，每個 handler 的核心邏輯從平均 40-50 行縮減到 5-15 行，大幅提高了代碼的可維護性和一致性。