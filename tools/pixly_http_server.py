#!/usr/bin/env python3
"""
PIXLY HTTP API Server - Python AI服务
替代Go HTTP网关，提供AI参数预测服务

Phase 47: 架构简化
- 移除Go依赖
- 双端架构（Rust + Python）
- 简化部署
"""

import os
import sys
import json
import logging
from pathlib import Path
from typing import Dict, Any, Optional
from datetime import datetime

# 添加tools目录到Python路径
TOOLS_DIR = Path(__file__).parent.resolve()
if str(TOOLS_DIR) not in sys.path:
    sys.path.insert(0, str(TOOLS_DIR))

# 使用Flask作为轻量级HTTP框架
try:
    from flask import Flask, request, jsonify
    from flask_cors import CORS
except ImportError:
    print("❌ Missing dependencies. Install: pip install flask flask-cors", file=sys.stderr)
    sys.exit(1)

# 导入现有的AI预测模块
try:
    from predict_params import AIPredictor
    from audio_predict import AudioPredictor  # 🆕 音频AI预测
except ImportError as e:
    print(f"❌ Cannot import AI modules: {e}", file=sys.stderr)
    sys.exit(1)

# 导入统一消息系统
try:
    from pixly_messaging import send_info, send_error, send_warning
except ImportError:
    print("⚠️  pixly_messaging not available, logging only to console", file=sys.stderr)
    send_info = lambda c, m: print(f"INFO [{c}]: {m}")
    send_error = lambda c, m, e: print(f"ERROR [{c}]: {m}")
    send_warning = lambda c, m: print(f"WARN [{c}]: {m}")

# 配置
SERVER_VERSION = "1.0.0"
DEFAULT_PORT = 50052  # 保持与Go服务相同端口
DEFAULT_HOST = "0.0.0.0"

# 创建Flask应用
app = Flask(__name__)

# 创建AI预测器实例（全局单例）
try:
    ai_predictor = AIPredictor()
    audio_predictor = AudioPredictor()  # 🆕 音频预测器
    send_info("Server", "AI Predictors initialized successfully (Image/Video/Audio)")
except Exception as e:
    print(f"⚠️  AI Predictor initialization failed: {e}", file=sys.stderr)
    ai_predictor = None
    audio_predictor = None
CORS(app)  # 启用跨域

# 配置日志
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger('pixly-http-server')


@app.route('/api/v1/health', methods=['GET'])
def health_check():
    """健康检查端点"""
    return jsonify({
        'status': 'healthy',
        'service': 'pixly-ai-service',
        'version': SERVER_VERSION,
        'language': 'python',
        'timestamp': datetime.now().isoformat()
    }), 200


@app.route('/api/v1/predict', methods=['POST'])
def predict_image():
    """
    图像AI参数预测
    
    Request:
    {
        "image_path": "/path/to/image.png",
        "target_format": "jxl",
        "mode": "balanced"
    }
    
    Response:
    {
        "recommended_format": "jxl",
        "quality": 92,
        "effort": 7,
        ...
    }
    """
    try:
        data = request.get_json()
        if not data:
            return jsonify({
                'error': 'Invalid JSON',
                'code': 'PIXLY-PYTHON-API-001'
            }), 400
        
        # 提取参数
        image_path = data.get('image_path')
        target_format = data.get('target_format', 'jxl')
        mode = data.get('mode', 'balanced')
        
        if not image_path:
            return jsonify({
                'error': 'Missing required field: image_path',
                'code': 'PIXLY-PYTHON-API-002'
            }), 400
        
        # 检查文件是否存在
        if not Path(image_path).exists():
            return jsonify({
                'error': f'Image file not found: {image_path}',
                'code': 'PIXLY-PYTHON-FILE-001'
            }), 404
        
        # 🔥 Phase 47.16: 添加文件大小检查
        file_size = Path(image_path).stat().st_size
        if file_size > 100 * 1024 * 1024:  # 100MB
            logger.warning(f"⚠️  Large file: {file_size / 1024 / 1024:.1f}MB - {image_path}")
            # 对于超大文件，返回默认参数避免卡死
            if file_size > 500 * 1024 * 1024:  # 500MB
                logger.error(f"❌ File too large (>500MB), returning default params")
                return jsonify({
                    'quality': 85,
                    'effort': 7,
                    'recommended_format': target_format,
                    'confidence': 0.5,
                    'warning': 'File too large for AI analysis, using defaults'
                }), 200
        
        logger.info(f"🔮 Predicting parameters for: {image_path}")
        logger.info(f"   Target format: {target_format}, Mode: {mode}")
        
        # 检查AI预测器是否可用
        if ai_predictor is None:
            return jsonify({
                'error': 'AI predictor not initialized',
                'code': 'PIXLY-PYTHON-AI-000'
            }), 503
        
        # 调用AI预测
        result = ai_predictor.predict(
            image_path=image_path,
            tool=target_format,
            target_quality=90,
            optimize_mode=mode
        )
        
        if result is None:
            return jsonify({
                'error': 'AI prediction failed',
                'code': 'PIXLY-PYTHON-AI-001'
            }), 500
        
        logger.info(f"✅ Prediction successful: format={result.get('recommended_format')}, quality={result.get('quality')}")
        
        return jsonify(result), 200
        
    except Exception as e:
        logger.error(f"❌ Error in /api/v1/predict: {e}")
        send_error("HTTPServer", f"Prediction error: {e}", "PIXLY-PYTHON-SYS-001")
        return jsonify({
            'error': str(e),
            'code': 'PIXLY-PYTHON-SYS-001'
        }), 500


@app.route('/api/v1/predict/video', methods=['POST'])
def predict_video():
    """
    视频AI参数预测
    
    Request:
    {
        "video_path": "/path/to/video.mp4",
        "mode": "balanced"
    }
    
    Response:
    {
        "encoder": "libx265",
        "crf": 23,
        "preset": "medium",
        "two_pass": false,
        ...
    }
    """
    try:
        data = request.get_json()
        if not data:
            return jsonify({
                'error': 'Invalid JSON',
                'code': 'PIXLY-PYTHON-API-001'
            }), 400
        
        # 提取参数
        video_path = data.get('video_path')
        mode = data.get('mode', 'balanced')
        
        if not video_path:
            return jsonify({
                'error': 'Missing required field: video_path',
                'code': 'PIXLY-PYTHON-API-002'
            }), 400
        
        # 检查文件是否存在
        if not Path(video_path).exists():
            return jsonify({
                'error': f'Video file not found: {video_path}',
                'code': 'PIXLY-PYTHON-FILE-002'
            }), 404
        
        logger.info(f"🎬 Predicting video parameters for: {video_path}")
        logger.info(f"   Mode: {mode}")
        
        # 检查AI预测器是否可用
        if ai_predictor is None:
            return jsonify({
                'error': 'AI predictor not initialized',
                'code': 'PIXLY-PYTHON-AI-000'
            }), 503
        
        # 调用AI预测
        result = ai_predictor.predict_video(
            video_path=video_path,
            optimize_mode=mode
        )
        
        if result is None:
            return jsonify({
                'error': 'Video AI prediction failed',
                'code': 'PIXLY-PYTHON-AI-002'
            }), 500
        
        logger.info(f"✅ Video prediction successful: encoder={result.get('encoder')}, crf={result.get('crf')}")
        
        return jsonify({
            'encoder': result.get('encoder'),
            'crf': result.get('crf'),
            'preset': result.get('preset'),
            'two_pass': result.get('two_pass'),
            'reasoning': result.get('reasoning', 'No reasoning'),
            'confidence': result.get('confidence', 0.0)
        }), 200
        
    except Exception as e:
        logger.error(f"❌ Error in /api/v1/predict/video: {e}")
        send_error("HTTPServer", f"Video prediction error: {e}", "PIXLY-PYTHON-SYS-002")
        return jsonify({
            'error': str(e),
            'code': 'PIXLY-PYTHON-SYS-002'
        }), 500


@app.route('/api/v1/predict/audio', methods=['POST'])
def predict_audio():
    """
    音频AI参数预测（统一策略）
    
    Request:
    {
        "audio_path": "/path/to/audio.mp3",
        "mode": "balanced",
        "options": {
            "aggressive_mode": false,
            "enable_magika": true
        }
    }
    
    Response:
    {
        "encoder": "opus",
        "bitrate": 128,
        "source_codec": "mp3",
        "recommended_codec": "opus",
        "codec_reason": "MP3→Opus upgrade",
        "preprocessing_steps": [],
        "magika_detection": {},
        "metadata": {}
    }
    """
    if not audio_predictor:
        return jsonify({
            'error': 'Audio predictor not initialized',
            'code': 'PIXLY-PYTHON-SYS-003'
        }), 503
    
    try:
        data = request.get_json()
        if not data:
            return jsonify({'error': 'No JSON data provided'}), 400
        
        audio_path = data.get('audio_path')
        mode = data.get('mode', 'balanced')
        options = data.get('options', {})
        
        if not audio_path:
            return jsonify({'error': 'audio_path is required'}), 400
        
        logger.info(f"🎵 Predicting audio parameters for: {audio_path}")
        logger.info(f"   Mode: {mode}")
        
        # 调用音频AI预测
        result = audio_predictor.predict_audio(
            audio_path=audio_path,
            optimize_mode=mode,
            options=options
        )
        
        if 'error' in result:
            return jsonify(result), 500
        
        return jsonify(result), 200
        
    except Exception as e:
        logger.error(f"❌ Error in /api/v1/predict/audio: {e}")
        send_error("HTTPServer", f"Audio prediction error: {e}", "PIXLY-PYTHON-SYS-003")
        return jsonify({
            'error': str(e),
            'code': 'PIXLY-PYTHON-SYS-003'
        }), 500


@app.errorhandler(404)
def not_found(error):
    """404错误处理"""
    return jsonify({
        'error': 'Endpoint not found',
        'code': 'PIXLY-PYTHON-API-404'
    }), 404


@app.errorhandler(500)
def internal_error(error):
    """500错误处理"""
    return jsonify({
        'error': 'Internal server error',
        'code': 'PIXLY-PYTHON-SYS-500'
    }), 500


def main():
    """启动HTTP服务器"""
    import argparse
    
    parser = argparse.ArgumentParser(description='PIXLY Python AI HTTP Server')
    parser.add_argument('--host', default=DEFAULT_HOST, help=f'Host to bind (default: {DEFAULT_HOST})')
    parser.add_argument('--port', type=int, default=DEFAULT_PORT, help=f'Port to bind (default: {DEFAULT_PORT})')
    parser.add_argument('--debug', action='store_true', help='Enable debug mode')
    
    args = parser.parse_args()
    
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print("🚀 PIXLY Python AI HTTP Server")
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print(f"   Version:     {SERVER_VERSION}")
    print(f"   Language:    Python {sys.version.split()[0]}")
    print(f"   Host:        {args.host}")
    print(f"   Port:        {args.port}")
    print(f"   Debug:       {args.debug}")
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print("")
    print("\n API Endpoints:")
    print(f"   GET  http://{args.host}:{args.port}/api/v1/health")
    print(f"   POST http://{args.host}:{args.port}/api/v1/predict")
    print(f"   POST http://{args.host}:{args.port}/api/v1/predict/video")
    print(f"   POST http://{args.host}:{args.port}/api/v1/predict/audio ")
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
    print("")
    
    send_info("HTTPServer", f"Starting server on {args.host}:{args.port}")
    
    # 启动Flask服务器
    app.run(
        host=args.host,
        port=args.port,
        debug=args.debug,
        threaded=True  # 启用多线程
    )


if __name__ == '__main__':
    main()
