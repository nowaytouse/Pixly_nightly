#!/usr/bin/env python3
"""
增强版Python AI HTTP服务器 - 支持批量并发预测
Phase 47.17: 性能优化版本
"""

import os
import sys
import json
import logging
from pathlib import Path
from datetime import datetime
from concurrent.futures import ThreadPoolExecutor, as_completed
import time

from flask import Flask, request, jsonify
from flask_cors import CORS

# 添加项目根目录到Python路径
project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))
sys.path.insert(0, str(project_root / 'tools'))

# 导入预测器
try:
    from predict_params import AIPredictor
    ai_predictor = AIPredictor()
except ImportError as e:
    print(f"❌ 无法导入AIPredictor: {e}")
    ai_predictor = None

# 初始化Flask
app = Flask(__name__)
CORS(app)

# 配置日志
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger('pixly-http-server')

# 🔥 Phase 47.17: 线程池用于并发处理
executor = ThreadPoolExecutor(max_workers=4)

@app.route('/api/v1/health', methods=['GET'])
def health():
    """健康检查端点"""
    return jsonify({
        'status': 'healthy',
        'timestamp': datetime.now().isoformat(),
        'service': 'pixly-ai-service-concurrent',
        'version': '2.0.0',
        'language': 'python',
        'features': {
            'concurrent': True,
            'cache': True,
            'resize': True
        }
    }), 200

@app.route('/api/v1/predict', methods=['POST'])
def predict_image():
    """单个图像预测（保持向后兼容）"""
    try:
        data = request.get_json()
        if not data:
            return jsonify({'error': 'Invalid JSON'}), 400
        
        image_path = data.get('image_path')
        if not image_path:
            return jsonify({'error': 'Missing image_path'}), 400
        
        # 调用预测
        result = _predict_single(data)
        return jsonify(result), 200
        
    except Exception as e:
        logger.error(f"Error in predict: {e}")
        return jsonify({'error': str(e)}), 500

@app.route('/api/v1/predict/batch', methods=['POST'])
def predict_batch():
    """
    批量并发预测端点
    
    Request:
    {
        "images": [
            {
                "image_path": "/path/to/image1.jpg",
                "target_format": "avif",
                "mode": "balanced"
            },
            {
                "image_path": "/path/to/image2.png",
                "target_format": "webp",
                "mode": "quality"
            }
        ],
        "concurrent": true  // 是否并发处理
    }
    
    Response:
    {
        "results": [...],
        "total": 2,
        "successful": 2,
        "failed": 0,
        "processing_time": 1.23
    }
    """
    try:
        data = request.get_json()
        if not data or 'images' not in data:
            return jsonify({'error': 'Invalid request, missing images'}), 400
        
        images = data.get('images', [])
        use_concurrent = data.get('concurrent', True)
        
        if not images:
            return jsonify({'error': 'Empty images list'}), 400
        
        start_time = time.time()
        results = []
        
        if use_concurrent and len(images) > 1:
            # 🔥 并发处理多个图像
            logger.info(f"Processing {len(images)} images concurrently")
            
            futures = []
            for img_data in images:
                future = executor.submit(_predict_single, img_data)
                futures.append((future, img_data.get('image_path', 'unknown')))
            
            # 收集结果
            for future, img_path in futures:
                try:
                    result = future.result(timeout=10)  # 10秒超时
                    results.append({
                        'image_path': img_path,
                        'status': 'success',
                        'prediction': result
                    })
                except Exception as e:
                    logger.error(f"Failed to predict {img_path}: {e}")
                    results.append({
                        'image_path': img_path,
                        'status': 'error',
                        'error': str(e)
                    })
        else:
            # 串行处理
            logger.info(f"Processing {len(images)} images sequentially")
            for img_data in images:
                try:
                    result = _predict_single(img_data)
                    results.append({
                        'image_path': img_data.get('image_path'),
                        'status': 'success',
                        'prediction': result
                    })
                except Exception as e:
                    results.append({
                        'image_path': img_data.get('image_path'),
                        'status': 'error',
                        'error': str(e)
                    })
        
        elapsed = time.time() - start_time
        successful = sum(1 for r in results if r['status'] == 'success')
        failed = len(results) - successful
        
        logger.info(f"Batch prediction completed: {successful}/{len(images)} successful in {elapsed:.2f}s")
        
        return jsonify({
            'results': results,
            'total': len(images),
            'successful': successful,
            'failed': failed,
            'processing_time': elapsed,
            'concurrent': use_concurrent
        }), 200
        
    except Exception as e:
        logger.error(f"Error in batch predict: {e}")
        return jsonify({'error': str(e)}), 500

def _predict_single(data):
    """内部函数：处理单个预测请求"""
    if not ai_predictor:
        raise ValueError("AI predictor not initialized")
    
    image_path = data.get('image_path')
    target_format = data.get('target_format', 'jxl')
    mode = data.get('mode', 'balanced')
    target_quality = data.get('target_quality', 90)
    
    # 检查文件
    if not Path(image_path).exists():
        raise FileNotFoundError(f"Image not found: {image_path}")
    
    # 文件大小检查
    file_size = Path(image_path).stat().st_size
    if file_size > 500 * 1024 * 1024:  # 500MB
        logger.warning(f"File too large: {file_size / 1024 / 1024:.1f}MB")
        return {
            'quality': 85,
            'effort': 7,
            'recommended_format': target_format,
            'confidence': 0.5,
            'warning': 'File too large, using defaults'
        }
    
    # 调用AI预测（会使用缓存和图像缩放优化）
    result = ai_predictor.predict(
        image_path=image_path,
        tool=target_format,
        target_quality=target_quality,
        optimize_mode=mode
    )
    
    return result

if __name__ == '__main__':
    port = int(os.environ.get('PIXLY_AI_PORT', '50052'))
    
    print("="*60)
    print(f"🚀 Pixly AI HTTP Server (Concurrent Version)")
    print(f"   Port: {port}")
    print(f"   AI Predictor: {'✅ Ready' if ai_predictor else '❌ Not initialized'}")
    print(f"   Concurrent: ✅ Enabled (4 workers)")
    print(f"   Cache: ✅ Enabled")
    print(f"   Image Resize: ✅ Enabled (max 1024x1024)")
    print("="*60)
    
    app.run(host='127.0.0.1', port=port, debug=False)
