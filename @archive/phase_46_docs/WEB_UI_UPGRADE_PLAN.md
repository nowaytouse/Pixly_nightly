# Pixly Web UI 升级计划 - 参考apps.apple.com架构

## 执行时间
2025-11-11 10:33

## apps.apple.com-main 架构分析

### 核心技术栈
- **框架**: Svelte (非常轻量、高性能)
- **语言**: TypeScript
- **架构模式**: Jet架构 (类似Flux/Redux)
- **组件化**: 高度模块化
- **国际化**: 完整i18n支持
- **路由**: 自定义路由系统
- **日志**: 结构化日志系统

### 关键设计理念

#### 1. 渐进式增强
```svelte
<!-- 优雅的骨架屏 -->
{#if webNavigation}
    <Navigation {webNavigation} />
{:else}
    <NavigationSkeleton />  <!-- 加载时显示骨架 -->
{/if}
```

**可学习**: 
- 异步加载时显示骨架屏
- 用户体验流畅
- 避免空白闪烁

#### 2. 响应式网格布局
```scss
.app-container {
    display: grid;
    grid-template-columns: 260px minmax(0, 1fr);  // 侧边栏 + 主内容
    
    @media (--sidebar-visible) {
        grid-template-columns: 260px minmax(0, 1fr);
    }
}
```

**可学习**:
- CSS Grid现代布局
- 响应式设计
- 移动端友好

#### 3. 图像优化系统
```typescript
// 支持多种尺寸和裁剪
export function getNaturalProfile(artwork: Artwork) {
    const aspectRatio = artwork.width / artwork.height;
    return [imageSizes, aspectRatio, artwork.crop];
}
```

**可学习**:
- 动态图像尺寸
- 响应式图像加载
- 懒加载支持

#### 4. 无障碍支持
```scss
/* Smart Invert 无障碍支持 */
@media (inverted-colors: inverted) {
    :global(.artwork-component img) {
        filter: invert(1);
    }
}
```

**可学习**:
- 无障碍优先
- 辅助功能完善

## Pixly Web UI 升级建议

### 🎯 优先级1: 基础架构搭建

#### 1.1 技术选型

**建议使用 Svelte + TypeScript**

**理由**:
- ✅ Svelte体积小（编译后无运行时）
- ✅ 性能极佳（无虚拟DOM）
- ✅ 学习曲线平缓
- ✅ TypeScript类型安全

**对比React**:
```
Svelte编译后: ~10KB
React运行时: ~130KB (React + ReactDOM)

Pixly Web UI必须轻量，因为要处理大量图像
```

#### 1.2 项目结构

```
Pixly_Nightly/
└── core/web/
    ├── public/
    │   ├── index.html
    │   └── assets/
    ├── src/
    │   ├── App.svelte                 # 根组件
    │   ├── main.ts                    # 入口文件
    │   ├── components/
    │   │   ├── layout/
    │   │   │   ├── Header.svelte
    │   │   │   ├── Sidebar.svelte
    │   │   │   └── Footer.svelte
    │   │   ├── converter/
    │   │   │   ├── ImageUploader.svelte
    │   │   │   ├── FormatSelector.svelte
    │   │   │   ├── ParameterPanel.svelte
    │   │   │   ├── PreviewPanel.svelte
    │   │   │   └── ProgressBar.svelte
    │   │   ├── ai/
    │   │   │   ├── AISuggestions.svelte
    │   │   │   ├── QualityPredictor.svelte
    │   │   │   └── OptimizationPath.svelte
    │   │   └── shared/
    │   │       ├── Button.svelte
    │   │       ├── Card.svelte
    │   │       └── Modal.svelte
    │   ├── stores/
    │   │   ├── images.ts              # 图像状态管理
    │   │   ├── conversion.ts          # 转换状态
    │   │   └── ai.ts                  # AI预测状态
    │   ├── workers/
    │   │   └── converter.worker.ts    # WebAssembly worker
    │   ├── api/
    │   │   ├── ai-client.ts           # Go AI服务客户端
    │   │   └── wasm-bridge.ts         # Rust WASM桥接
    │   └── utils/
    │       ├── file-size.ts
    │       ├── image-analysis.ts
    │       └── format-detection.ts
    ├── package.json
    ├── vite.config.ts
    └── tsconfig.json
```

### 🎯 优先级2: 核心功能实现

#### 2.1 图像上传器 (参考Apple的Artwork组件)

```svelte
<!-- ImageUploader.svelte -->
<script lang="ts">
    import { writable } from 'svelte/store';
    
    export let onImagesSelected: (files: File[]) => void;
    
    let dragActive = false;
    
    function handleDrop(e: DragEvent) {
        e.preventDefault();
        dragActive = false;
        
        const files = Array.from(e.dataTransfer?.files || []);
        const imageFiles = files.filter(f => f.type.startsWith('image/'));
        
        onImagesSelected(imageFiles);
    }
    
    function handleFileInput(e: Event) {
        const input = e.target as HTMLInputElement;
        const files = Array.from(input.files || []);
        onImagesSelected(files);
    }
</script>

<div 
    class="uploader"
    class:drag-active={dragActive}
    on:drop={handleDrop}
    on:dragover|preventDefault={() => dragActive = true}
    on:dragleave={() => dragActive = false}
>
    <input 
        type="file" 
        multiple 
        accept="image/*"
        on:change={handleFileInput}
    />
    
    <div class="uploader-content">
        {#if dragActive}
            <p>释放以上传</p>
        {:else}
            <p>拖放图像或点击选择</p>
        {/if}
    </div>
</div>

<style>
    .uploader {
        border: 2px dashed #ccc;
        border-radius: 8px;
        padding: 40px;
        text-align: center;
        transition: all 0.3s ease;
    }
    
    .uploader.drag-active {
        border-color: #0071e3;
        background: rgba(0, 113, 227, 0.05);
    }
</style>
```

#### 2.2 AI建议面板 (新功能)

```svelte
<!-- AISuggestions.svelte -->
<script lang="ts">
    import { aiStore } from '~/stores/ai';
    
    export let imagePath: string;
    
    $: prediction = aiStore.getPrediction(imagePath);
</script>

<div class="ai-panel">
    <h3>🤖 AI 优化建议</h3>
    
    {#await prediction}
        <div class="skeleton">分析中...</div>
    {:then data}
        {#if data.success}
            <div class="suggestion-card">
                <div class="format">
                    <strong>推荐格式:</strong> {data.params.format}
                </div>
                <div class="quality">
                    <strong>推荐质量:</strong> {data.params.quality}
                </div>
                <div class="confidence">
                    <strong>置信度:</strong> {(data.confidence * 100).toFixed(1)}%
                </div>
                
                {#if data.preprocessing_steps?.length > 0}
                    <div class="preprocessing">
                        <strong>预处理建议:</strong>
                        <ul>
                            {#each data.preprocessing_steps as step}
                                <li>{step.reason}</li>
                            {/each}
                        </ul>
                    </div>
                {/if}
                
                <div class="optimization-path">
                    <p><em>{data.optimization_path}</em></p>
                </div>
            </div>
        {:else}
            <div class="error">AI 预测失败: {data.error}</div>
        {/if}
    {:catch error}
        <div class="error">错误: {error.message}</div>
    {/await}
</div>

<style>
    .ai-panel {
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        color: white;
        padding: 20px;
        border-radius: 12px;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    }
    
    .suggestion-card {
        background: rgba(255, 255, 255, 0.1);
        backdrop-filter: blur(10px);
        padding: 16px;
        border-radius: 8px;
        margin-top: 12px;
    }
</style>
```

#### 2.3 实时预览对比 (参考Apple的布局)

```svelte
<!-- PreviewPanel.svelte -->
<script lang="ts">
    export let originalImage: string;
    export let convertedImage: string | null;
    export let originalSize: number;
    export let convertedSize: number | null;
    
    import { getFileSizeParts } from '~/utils/file-size';
    
    $: originalSizeParts = getFileSizeParts(originalSize);
    $: convertedSizeParts = convertedSize ? getFileSizeParts(convertedSize) : null;
    $: savings = convertedSize ? ((1 - convertedSize / originalSize) * 100).toFixed(1) : 0;
</script>

<div class="preview-grid">
    <!-- 原图 -->
    <div class="preview-card">
        <div class="preview-header">
            <h4>原图</h4>
            <span class="size">{originalSizeParts.count} {originalSizeParts.unit}</span>
        </div>
        <div class="preview-image">
            <img src={originalImage} alt="原图" />
        </div>
    </div>
    
    <!-- 转换后 -->
    <div class="preview-card">
        <div class="preview-header">
            <h4>转换后</h4>
            {#if convertedSizeParts}
                <span class="size">{convertedSizeParts.count} {convertedSizeParts.unit}</span>
                <span class="savings">↓ {savings}%</span>
            {:else}
                <span class="size">处理中...</span>
            {/if}
        </div>
        <div class="preview-image">
            {#if convertedImage}
                <img src={convertedImage} alt="转换后" />
            {:else}
                <div class="skeleton-image">处理中...</div>
            {/if}
        </div>
    </div>
</div>

<style>
    .preview-grid {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 20px;
        
        @media (max-width: 768px) {
            grid-template-columns: 1fr;
        }
    }
    
    .preview-card {
        border: 1px solid #e0e0e0;
        border-radius: 12px;
        overflow: hidden;
        background: white;
    }
    
    .savings {
        color: #34c759;
        font-weight: bold;
    }
</style>
```

### 🎯 优先级3: WebAssembly 集成

#### 3.1 Rust CLI 编译为 WASM

```toml
# Cargo.toml 添加
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = { version = "0.3", features = ["File", "Blob"] }
```

```rust
// src/wasm.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmConverter {
    // 内部状态
}

#[wasm_bindgen]
impl WasmConverter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }
    
    #[wasm_bindgen]
    pub fn convert_image(
        &self,
        input_data: &[u8],
        format: &str,
        quality: u8,
    ) -> Result<Vec<u8>, JsValue> {
        // 调用现有的Rust转换逻辑
        // 但是完全在浏览器中运行
        Ok(vec![])
    }
}
```

#### 3.2 Web Worker 集成

```typescript
// workers/converter.worker.ts
import init, { WasmConverter } from '../wasm/pixly_converter';

let converter: WasmConverter | null = null;

self.addEventListener('message', async (e) => {
    const { type, data } = e.data;
    
    if (type === 'init') {
        await init();
        converter = new WasmConverter();
        self.postMessage({ type: 'ready' });
    }
    
    if (type === 'convert' && converter) {
        const { imageData, format, quality } = data;
        
        try {
            const result = converter.convert_image(imageData, format, quality);
            self.postMessage({ 
                type: 'success', 
                data: result 
            });
        } catch (error) {
            self.postMessage({ 
                type: 'error', 
                error: error.message 
            });
        }
    }
});
```

### 🎯 优先级4: 高级特性

#### 4.1 批量处理队列 (参考Apple的异步模式)

```typescript
// stores/conversion.ts
import { writable, derived } from 'svelte/store';

interface ConversionTask {
    id: string;
    file: File;
    status: 'pending' | 'processing' | 'success' | 'error';
    progress: number;
    result?: Blob;
    error?: string;
}

function createConversionQueue() {
    const { subscribe, update } = writable<ConversionTask[]>([]);
    
    return {
        subscribe,
        add: (files: File[]) => {
            update(queue => [
                ...queue,
                ...files.map(file => ({
                    id: crypto.randomUUID(),
                    file,
                    status: 'pending' as const,
                    progress: 0,
                }))
            ]);
        },
        process: async (workers = 4) => {
            // 并行处理，最多4个worker
            // 类似XL Converter的并行编码
        },
    };
}

export const conversionQueue = createConversionQueue();
```

#### 4.2 实时性能监控

```svelte
<!-- PerformanceMonitor.svelte -->
<script lang="ts">
    import { onMount } from 'svelte';
    
    let metrics = {
        conversionSpeed: 0,  // MB/s
        avgQuality: 0,
        totalSaved: 0,       // bytes
    };
    
    onMount(() => {
        // 监控转换性能
        // 显示实时统计
    });
</script>

<div class="metrics-panel">
    <div class="metric">
        <span class="label">转换速度</span>
        <span class="value">{metrics.conversionSpeed.toFixed(2)} MB/s</span>
    </div>
    <div class="metric">
        <span class="label">总节省空间</span>
        <span class="value">{(metrics.totalSaved / 1024 / 1024).toFixed(2)} MB</span>
    </div>
</div>
```

## 关键设计决策

### 1. 为什么选择 Svelte？

**✅ 优势**:
- 编译时框架，无运行时开销
- 性能接近原生JavaScript
- 体积极小（重要！因为要处理大图像）
- 响应式简单（无需useState/useEffect）

**对比**:
```
React应用:     130KB (框架) + 50KB (应用) = 180KB
Svelte应用:    0KB (框架) + 30KB (应用) = 30KB
```

### 2. 为什么使用 WebAssembly？

**✅ 优势**:
- 复用现有Rust代码
- 性能接近原生
- 完全本地处理（隐私）
- 无需服务器

**挑战**:
- [ ] 需要调研哪些依赖支持WASM
- [ ] 文件大小控制
- [ ] 加载时间优化

### 3. 架构图

```
用户浏览器
├── Svelte UI (30KB)
│   ├── 图像上传
│   ├── 参数调整
│   └── 预览对比
├── WebAssembly (Rust CLI编译)
│   ├── 图像转换
│   ├── 预处理管道
│   └── 格式检测
└── HTTP Client (连接Go AI服务)
    ├── AI参数预测
    ├── 预处理建议
    └── 优化路径
```

## 实施计划

### Phase 1: 基础搭建 (1周)
- [ ] 初始化Svelte + TypeScript + Vite项目
- [ ] 实现基础布局（Header/Sidebar/Main）
- [ ] 实现图像上传组件
- [ ] 连接Go AI服务（HTTP客户端）

### Phase 2: 核心功能 (2周)
- [ ] 实现格式选择器
- [ ] 实现参数面板
- [ ] 实现AI建议面板
- [ ] 实现预览对比

### Phase 3: WASM集成 (2周)
- [ ] 调研Rust WASM编译可行性
- [ ] 实现关键转换功能的WASM版本
- [ ] 实现Web Worker
- [ ] 性能优化

### Phase 4: 高级特性 (1周)
- [ ] 实现批量处理队列
- [ ] 实现拖放排序
- [ ] 实现性能监控
- [ ] 响应式设计优化

## 批判性分析

### 需要谨慎的点

1. **不要过度设计**
   - Apple的架构很复杂（Jet系统）
   - Pixly不需要那么复杂的路由
   - 保持简单直接

2. **WASM的权衡**
   - WASM文件可能很大（5-10MB）
   - 加载时间需要优化
   - 可能需要渐进式加载

3. **AI服务依赖**
   - Web UI依赖Go AI服务运行
   - 需要处理服务不可用的情况
   - 需要提供"离线模式"（使用默认参数）

4. **性能考虑**
   - 大图像在浏览器中处理可能卡顿
   - 需要限制并发数量
   - 需要内存管理

## 用户体验设计原则

### 1. 即时反馈 (参考Apple)
```svelte
<!-- 每个操作都有即时反馈 -->
<button 
    class:loading={isConverting}
    disabled={isConverting}
>
    {#if isConverting}
        <Spinner /> 转换中...
    {:else}
        开始转换
    {/if}
</button>
```

### 2. 优雅降级
```typescript
// 如果WASM不可用，回退到服务器处理
async function convertImage(file: File) {
    if (wasmSupported && wasmLoaded) {
        return convertWithWasm(file);
    } else {
        return convertWithServer(file);
    }
}
```

### 3. 无障碍优先
```svelte
<!-- 完整的ARIA标签 -->
<div role="region" aria-label="图像转换区域">
    <button aria-label="上传图像">
        上传
    </button>
</div>
```

## 总结

**核心改进方向**:
1. ✅ **Svelte架构** - 轻量高性能
2. ✅ **组件化设计** - 参考Apple的模块化
3. ✅ **WebAssembly** - 本地处理，保护隐私
4. ✅ **AI可视化** - 让AI建议更直观
5. ✅ **批量处理** - 提升效率

**保持的原则**:
- 简单 > 复杂
- 性能 > 功能
- 本地 > 云端
- 隐私 > 便利

**下一步**: 创建基础项目原型？
