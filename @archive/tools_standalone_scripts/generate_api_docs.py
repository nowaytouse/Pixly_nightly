#!/usr/bin/env python3
"""
API文档生成工具 - 自动生成完整API文档

使用方法:
    python tools/generate_api_docs.py [options]

选项:
    --output-dir OUTPUT_DIR    输出目录 (默认: docs/api)
    --formats FORMAT1,FORMAT2  文档格式 (markdown,openapi,postman)
    --config CONFIG_FILE       自定义配置文件
    --verbose                  详细输出
    --test                     生成测试用例
"""

import sys
import argparse
from pathlib import Path

# 添加项目根目录到Python路径
project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))

from core.python.docs.api_doc_generator import (
    APIDocGenerator, APIDocConfig, DocumentFormat,
    APIEndpoint, APIParameter, APIResponse, HTTPMethod
)
import json
import logging


def setup_logging(verbose: bool = False):
    """设置日志"""
    level = logging.DEBUG if verbose else logging.INFO
    logging.basicConfig(
        level=level,
        format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
    )


def load_custom_config(config_file: str) -> APIDocConfig:
    """加载自定义配置"""
    with open(config_file, 'r', encoding='utf-8') as f:
        config_data = json.load(f)
    
    return APIDocConfig(**config_data)


def enhance_api_endpoints(doc_gen: APIDocGenerator):
    """增强API端点定义（添加更多端点）"""
    
    # 视频预测API
    video_predict_endpoint = APIEndpoint(
        path="/api/v1/predict/video",
        method=HTTPMethod.POST,
        summary="视频AI参数预测",
        description="基于视频特征预测最佳压缩参数",
        tags=["prediction", "video"],
        parameters=[
            APIParameter("video_path", "string", "视频文件路径", True, example="/path/to/video.mp4"),
            APIParameter("target_codec", "string", "目标编解码器", False, "h265", example="h265", enum_values=["h264", "h265", "av1"]),
            APIParameter("target_crf", "integer", "目标CRF值", False, 23, example=23, min_value=0, max_value=51),
            APIParameter("preset", "string", "编码预设", False, "medium", example="medium", enum_values=["ultrafast", "fast", "medium", "slow", "veryslow"])
        ],
        responses=[
            APIResponse(200, "预测成功", example={
                "success": True,
                "params": {"crf": 23, "preset": "medium", "codec": "h265"},
                "confidence": 0.82,
                "estimated_size_mb": 125.6,
                "estimated_time_seconds": 45.2
            })
        ]
    )
    doc_gen.add_endpoint(video_predict_endpoint)
    
    # 训练状态API
    training_status_endpoint = APIEndpoint(
        path="/api/v1/training/status",
        method=HTTPMethod.GET,
        summary="获取训练状态",
        description="获取当前模型训练队列的状态",
        tags=["training"],
        auth_required=True,
        responses=[
            APIResponse(200, "训练状态", example={
                "is_running": True,
                "current_epoch": 15,
                "total_epochs": 100,
                "progress": 0.15,
                "eta_minutes": 42.5,
                "current_loss": 0.0234
            })
        ]
    )
    doc_gen.add_endpoint(training_status_endpoint)
    
    # 模型注册API
    model_register_endpoint = APIEndpoint(
        path="/api/v1/models/register",
        method=HTTPMethod.POST,
        summary="注册新模型",
        description="注册新的AI模型版本到系统",
        tags=["models"],
        auth_required=True,
        parameters=[
            APIParameter("name", "string", "模型名称", True, example="lightgbm"),
            APIParameter("version", "string", "模型版本", True, example="v2.0.0"),
            APIParameter("model_path", "string", "模型文件路径", True, example="/models/lightgbm_v2.pth"),
            APIParameter("description", "string", "模型描述", False, example="改进的LightGBM模型")
        ],
        responses=[
            APIResponse(201, "注册成功", example={
                "success": True,
                "model_id": "lightgbm_v2.0.0",
                "message": "模型注册成功"
            }),
            APIResponse(409, "模型已存在"),
            APIResponse(400, "注册参数错误")
        ]
    )
    doc_gen.add_endpoint(model_register_endpoint)
    
    # 反馈记录API
    feedback_endpoint = APIEndpoint(
        path="/api/v1/feedback/record",
        method=HTTPMethod.POST,
        summary="记录用户反馈",
        description="记录用户对AI预测结果的反馈",
        tags=["feedback"],
        parameters=[
            APIParameter("prediction_id", "string", "预测ID", True, example="pred_123456"),
            APIParameter("user_rating", "integer", "用户评分", True, example=4, min_value=1, max_value=5),
            APIParameter("actual_quality", "integer", "实际质量", False, example=87, min_value=0, max_value=100),
            APIParameter("comments", "string", "用户评论", False, example="预测很准确")
        ],
        responses=[
            APIResponse(200, "反馈记录成功", example={
                "success": True,
                "feedback_id": "fb_789012",
                "message": "感谢您的反馈"
            })
        ]
    )
    doc_gen.add_endpoint(feedback_endpoint)
    
    # 能力查询API
    capabilities_endpoint = APIEndpoint(
        path="/api/v1/capabilities",
        method=HTTPMethod.GET,
        summary="获取服务能力",
        description="获取AI服务支持的功能和格式列表",
        tags=["system"],
        responses=[
            APIResponse(200, "服务能力", example={
                "prediction": {
                    "image": True,
                    "video": True,
                    "supported_formats": ["jpg", "png", "webp", "avif", "jxl", "heic"]
                },
                "models": {
                    "lightgbm": True,
                    "ppo": True,
                    "ab_testing": True
                },
                "features": {
                    "online_learning": True,
                    "feedback": True,
                    "batch_processing": True
                }
            })
        ]
    )
    doc_gen.add_endpoint(capabilities_endpoint)


def generate_postman_collection(doc_gen: APIDocGenerator, output_dir: Path):
    """生成Postman集合"""
    collection = {
        "info": {
            "name": doc_gen.config.title,
            "description": doc_gen.config.description,
            "version": doc_gen.config.version,
            "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
        },
        "item": []
    }
    
    # 按标签分组
    groups = {}
    for endpoint in doc_gen.endpoints:
        tag = endpoint.tags[0] if endpoint.tags else "default"
        if tag not in groups:
            groups[tag] = []
        groups[tag].append(endpoint)
    
    for tag, endpoints in groups.items():
        folder = {
            "name": tag.title(),
            "item": []
        }
        
        for endpoint in endpoints:
            item = {
                "name": endpoint.summary,
                "request": {
                    "method": endpoint.method.value,
                    "header": [
                        {"key": "Content-Type", "value": "application/json"}
                    ],
                    "url": {
                        "raw": f"{{{{base_url}}}}{endpoint.path}",
                        "host": ["{{base_url}}"],
                        "path": endpoint.path.strip("/").split("/")
                    },
                    "description": endpoint.description
                }
            }
            
            # 添加认证
            if endpoint.auth_required:
                item["request"]["header"].append({
                    "key": "X-API-Key", 
                    "value": "{{api_key}}"
                })
            
            # 添加参数示例
            if endpoint.parameters and endpoint.method == HTTPMethod.POST:
                body_example = {}
                for param in endpoint.parameters:
                    if param.example is not None:
                        body_example[param.name] = param.example
                
                if body_example:
                    item["request"]["body"] = {
                        "mode": "raw",
                        "raw": json.dumps(body_example, indent=2)
                    }
            
            folder["item"].append(item)
        
        collection["item"].append(folder)
    
    # 添加环境变量
    collection["variable"] = [
        {"key": "base_url", "value": doc_gen.config.base_url},
        {"key": "api_key", "value": "your_api_key_here"}
    ]
    
    # 保存Postman集合
    postman_file = output_dir / "postman_collection.json"
    postman_file.write_text(
        json.dumps(collection, indent=2, ensure_ascii=False),
        encoding='utf-8'
    )
    
    print(f"✅ Postman集合已保存: {postman_file}")


def main():
    """主函数"""
    parser = argparse.ArgumentParser(
        description="API文档生成工具",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__
    )
    
    parser.add_argument(
        '--output-dir',
        default='docs/api',
        help='输出目录 (默认: docs/api)'
    )
    
    parser.add_argument(
        '--formats',
        default='markdown,openapi',
        help='文档格式，逗号分隔 (默认: markdown,openapi)'
    )
    
    parser.add_argument(
        '--config',
        help='自定义配置文件路径'
    )
    
    parser.add_argument(
        '--verbose', '-v',
        action='store_true',
        help='详细输出'
    )
    
    parser.add_argument(
        '--test',
        action='store_true',
        help='生成测试用例'
    )
    
    args = parser.parse_args()
    
    # 设置日志
    setup_logging(args.verbose)
    logger = logging.getLogger(__name__)
    
    # 解析格式
    format_map = {
        'markdown': DocumentFormat.MARKDOWN,
        'openapi': DocumentFormat.OPENAPI,
        'html': DocumentFormat.HTML,
        'postman': DocumentFormat.POSTMAN
    }
    
    formats = []
    for fmt_name in args.formats.split(','):
        fmt_name = fmt_name.strip().lower()
        if fmt_name in format_map:
            formats.append(format_map[fmt_name])
        else:
            logger.warning(f"未知格式: {fmt_name}")
    
    # 加载配置
    config = None
    if args.config:
        try:
            config = load_custom_config(args.config)
            logger.info(f"已加载自定义配置: {args.config}")
        except Exception as e:
            logger.error(f"加载配置文件失败: {e}")
            return 1
    
    # 创建文档生成器
    logger.info("创建API文档生成器...")
    doc_gen = APIDocGenerator(config, debug=args.verbose)
    
    # 增强API端点
    enhance_api_endpoints(doc_gen)
    
    # 创建输出目录
    output_dir = Path(args.output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)
    
    logger.info(f"开始生成文档，输出目录: {output_dir}")
    
    try:
        # 生成标准格式文档
        if DocumentFormat.POSTMAN in formats:
            formats.remove(DocumentFormat.POSTMAN)
            generate_postman_collection(doc_gen, output_dir)
        
        if formats:
            doc_gen.save_documentation(str(output_dir), formats)
        
        # 生成测试用例
        if args.test:
            test_cases = doc_gen.generate_test_cases()
            test_file = output_dir / "test_cases.json"
            test_file.write_text(
                json.dumps(test_cases, indent=2, ensure_ascii=False),
                encoding='utf-8'
            )
            logger.info(f"测试用例已保存: {test_file}")
        
        # 生成摘要信息
        summary = doc_gen.get_api_summary()
        summary_file = output_dir / "api_summary.json"
        summary_file.write_text(
            json.dumps(summary, indent=2, ensure_ascii=False),
            encoding='utf-8'
        )
        logger.info(f"API摘要已保存: {summary_file}")
        
        # 打印摘要
        print("\n" + "="*60)
        print("📊 API文档生成完成!")
        print("="*60)
        print(f"总端点数: {summary['total_endpoints']}")
        print(f"按标签分布: {summary['by_tags']}")
        print(f"按HTTP方法分布: {summary['by_methods']}")
        print(f"输出目录: {output_dir}")
        print("="*60)
        
        return 0
        
    except Exception as e:
        logger.error(f"文档生成失败: {e}")
        return 1


if __name__ == "__main__":
    sys.exit(main())
