#!/bin/bash
# Phase 47.20 (R-006): WASM构建脚本

echo "🚀 Building Pixly WASM..."

# 安装wasm-pack（如果未安装）
if ! command -v wasm-pack &> /dev/null; then
    echo "📦 Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# 构建WASM包
echo "🔨 Building WASM package..."
wasm-pack build --target web --out-dir pkg

# 优化WASM文件大小
if command -v wasm-opt &> /dev/null; then
    echo "⚡ Optimizing WASM size..."
    wasm-opt -Oz pkg/pixly_wasm_bg.wasm -o pkg/pixly_wasm_bg.wasm
fi

# 显示构建结果
echo "✅ Build complete!"
echo "📦 Output directory: pkg/"
ls -lh pkg/*.wasm 2>/dev/null

# 创建示例HTML
cat > pkg/index.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <title>Pixly WASM Demo</title>
    <style>
        body { font-family: Arial, sans-serif; padding: 20px; }
        .container { max-width: 800px; margin: 0 auto; }
        button { margin: 10px; padding: 10px 20px; }
        #output { margin-top: 20px; border: 1px solid #ccc; padding: 10px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🖼️ Pixly WASM Image Converter</h1>
        <input type="file" id="fileInput" accept="image/*">
        <br>
        <button onclick="convertToWebP()">Convert to WebP</button>
        <button onclick="convertToJPEG()">Convert to JPEG</button>
        <button onclick="resize()">Resize 50%</button>
        <div id="output"></div>
    </div>

    <script type="module">
        import init, { PixlyWasm, ConvertOptions } from './pixly_wasm.js';
        
        let pixly;
        let currentImageData;
        
        async function initWasm() {
            await init();
            pixly = new PixlyWasm();
            console.log('✅ WASM initialized');
        }
        
        document.getElementById('fileInput').addEventListener('change', async (e) => {
            const file = e.target.files[0];
            if (!file) return;
            
            const arrayBuffer = await file.arrayBuffer();
            currentImageData = new Uint8Array(arrayBuffer);
            
            pixly.load_from_bytes(currentImageData);
            document.getElementById('output').innerHTML = '✅ Image loaded';
        });
        
        window.convertToWebP = () => {
            if (!currentImageData) return;
            
            const options = new ConvertOptions();
            options.set_format('webp');
            options.set_quality(85);
            
            const result = pixly.convert(options);
            displayResult(result, 'image/webp');
        };
        
        window.convertToJPEG = () => {
            if (!currentImageData) return;
            
            const options = new ConvertOptions();
            options.set_format('jpeg');
            options.set_quality(90);
            
            const result = pixly.convert(options);
            displayResult(result, 'image/jpeg');
        };
        
        window.resize = () => {
            if (!currentImageData) return;
            
            const dims = pixly.get_dimensions();
            pixly.resize(Math.floor(dims[0] / 2), Math.floor(dims[1] / 2));
            document.getElementById('output').innerHTML = '✅ Resized to 50%';
        };
        
        function displayResult(data, mimeType) {
            const blob = new Blob([data], { type: mimeType });
            const url = URL.createObjectURL(blob);
            const img = document.createElement('img');
            img.src = url;
            img.style.maxWidth = '100%';
            document.getElementById('output').innerHTML = '';
            document.getElementById('output').appendChild(img);
        }
        
        initWasm();
    </script>
</body>
</html>
EOF

echo "📄 Demo page created: pkg/index.html"
echo "🌐 Run 'python3 -m http.server 8000 -d pkg' to test"
