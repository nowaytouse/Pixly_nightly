#!/usr/bin/env python3
"""
配置管理系统测试工具 - 全面验证配置管理功能

使用方法:
    python tools/test_config_system.py [options]

选项:
    --config-dir CONFIG_DIR    配置目录 (默认: test_config)
    --environment ENV          测试环境 (development/testing/production)
    --create-samples           创建示例配置文件
    --verbose                  详细输出
    --cleanup                  测试后清理文件
"""

import sys
import argparse
import tempfile
import shutil
from pathlib import Path
import json
import os
import time

# 添加项目根目录到Python路径
project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))

from core.python.config.config_manager import (
    ConfigManager, ConfigSection, ConfigValue, ConfigType, ConfigSource
)
from core.python.config.environment_manager import (
    EnvironmentManager, Environment, EnvironmentConfig
)
import logging


def setup_logging(verbose: bool = False):
    """设置日志"""
    level = logging.DEBUG if verbose else logging.INFO
    logging.basicConfig(
        level=level,
        format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
    )


def create_sample_configs(config_dir: Path):
    """创建示例配置文件"""
    config_dir.mkdir(parents=True, exist_ok=True)
    
    # 开发环境模型配置
    dev_models_config = {
        "lightgbm_model_path": {
            "value": "models/lightgbm_dev.pkl",
            "type": "string",
            "description": "LightGBM模型文件路径",
            "required": True
        },
        "ppo_enabled": {
            "value": True,
            "type": "boolean",
            "description": "启用PPO模型",
            "default": False,
            "required": False
        },
        "inference_timeout_ms": {
            "value": 5000,
            "type": "integer",
            "description": "推理超时时间",
            "default": 3000,
            "required": True
        }
    }
    
    with open(config_dir / "models_development.json", 'w') as f:
        json.dump(dev_models_config, f, indent=2)
    
    # 生产环境模型配置
    prod_models_config = {
        "lightgbm_model_path": {
            "value": "/opt/pixly/models/lightgbm_prod.pkl",
            "type": "string",
            "description": "生产LightGBM模型路径",
            "required": True
        },
        "ppo_enabled": {
            "value": False,
            "type": "boolean",
            "description": "生产环境禁用PPO",
            "default": False,
            "required": True
        },
        "inference_timeout_ms": {
            "value": 3000,
            "type": "integer",
            "description": "生产推理超时",
            "default": 3000,
            "required": True
        }
    }
    
    with open(config_dir / "models_production.json", 'w') as f:
        json.dump(prod_models_config, f, indent=2)
    
    # HTTP服务配置
    http_config = {
        "cors_origins": {
            "value": ["http://localhost:3000", "https://pixly.ai"],
            "type": "list",
            "description": "允许的CORS源",
            "required": False
        },
        "rate_limit": {
            "value": {"requests_per_minute": 100, "burst": 10},
            "type": "dict",
            "description": "速率限制配置",
            "required": False
        }
    }
    
    with open(config_dir / "http_development.json", 'w') as f:
        json.dump(http_config, f, indent=2)


def test_config_manager(config_dir: Path, verbose: bool = False) -> bool:
    """测试配置管理器"""
    print("🔧 测试配置管理器...")
    
    try:
        # 创建配置管理器
        config_mgr = ConfigManager(str(config_dir), debug=verbose)
        
        # 测试基本配置获取
        port = config_mgr.get_value("http", "port", 8080)
        assert port == 8080, f"期望端口8080，实际{port}"
        print(f"  ✅ 基本配置获取: HTTP端口 = {port}")
        
        # 测试配置设置
        success = config_mgr.set_value("http", "port", 8081, persist=False)
        assert success, "配置设置失败"
        
        new_port = config_mgr.get_value("http", "port")
        assert new_port == 8081, f"期望端口8081，实际{new_port}"
        print(f"  ✅ 配置设置: 端口更新为 {new_port}")
        
        # 测试配置验证
        validation_results = config_mgr.validate_all()
        all_valid = all(
            all(results.values()) 
            for results in validation_results.values()
        )
        print(f"  ✅ 配置验证: {'通过' if all_valid else '有警告'}")
        
        # 测试配置段获取
        http_config = config_mgr.get_section_config("http")
        assert "port" in http_config, "HTTP配置段缺少端口配置"
        print(f"  ✅ 配置段获取: HTTP配置有{len(http_config)}项")
        
        # 测试配置回调
        callback_triggered = []
        def config_change_callback(section, key, old_value, new_value):
            callback_triggered.append((section, key, old_value, new_value))
        
        config_mgr.add_change_callback("http", "port", config_change_callback)
        config_mgr.set_value("http", "port", 8082)
        
        assert len(callback_triggered) == 1, "配置回调未触发"
        print(f"  ✅ 配置回调: 触发了{len(callback_triggered)}次")
        
        # 测试配置摘要
        summary = config_mgr.get_config_summary()
        assert summary['total_configs'] > 0, "配置摘要为空"
        print(f"  ✅ 配置摘要: 总计{summary['total_configs']}项配置")
        
        config_mgr.close()
        print("  ✅ 配置管理器测试完成")
        return True
        
    except Exception as e:
        print(f"  ❌ 配置管理器测试失败: {e}")
        return False


def test_environment_manager(config_dir: Path, verbose: bool = False) -> bool:
    """测试环境管理器"""
    print("🌍 测试环境管理器...")
    
    try:
        # 设置必需的环境变量
        os.environ["PIXLY_ENV"] = "development"
        os.environ["PIXLY_TEST_TOKEN"] = "test_token"
        os.environ["PIXLY_API_KEY"] = "test_api_key"
        os.environ["PIXLY_SECRET_KEY"] = "test_secret"
        
        # 创建环境管理器
        env_mgr = EnvironmentManager(str(config_dir), debug=verbose)
        
        # 测试当前环境
        current_env = env_mgr.get_current_environment_name()
        assert current_env in ["development", "testing", "production"], f"无效环境: {current_env}"
        print(f"  ✅ 当前环境: {current_env}")
        
        # 测试环境列表
        environments = env_mgr.list_environments()
        expected_envs = ["development", "testing", "staging", "production"]
        for env_name in expected_envs:
            assert env_name in environments, f"缺少环境: {env_name}"
        print(f"  ✅ 环境列表: {len(environments)}个环境")
        
        # 测试环境切换
        original_env = current_env
        success = env_mgr.activate_environment("testing")
        assert success, "环境切换失败"
        
        new_env = env_mgr.get_current_environment_name()
        assert new_env == "testing", f"期望testing环境，实际{new_env}"
        print(f"  ✅ 环境切换: {original_env} → {new_env}")
        
        # 测试环境信息
        env_info = env_mgr.get_environment_info("testing")
        assert env_info is not None, "环境信息获取失败"
        assert env_info["name"] == "testing", "环境信息不正确"
        print(f"  ✅ 环境信息: {env_info['description']}")
        
        # 测试环境验证
        validation = env_mgr.validate_current_environment()
        print(f"  ✅ 环境验证: {'通过' if validation['valid'] else '有问题'}")
        if not validation['valid']:
            print(f"    错误: {validation['errors']}")
        if validation.get('warnings'):
            print(f"    警告: {validation['warnings']}")
        
        # 测试服务配置
        http_config = env_mgr.get_service_config("http")
        assert "port" in http_config, "服务配置缺少端口"
        print(f"  ✅ 服务配置: HTTP端口{http_config['port']}")
        
        # 测试环境变量
        env_mgr.set_env_var("TEST_VAR", "test_value")
        test_value = env_mgr.get_env_var("TEST_VAR")
        assert test_value == "test_value", "环境变量设置失败"
        print(f"  ✅ 环境变量: TEST_VAR = {test_value}")
        
        # 测试环境检测函数
        is_dev = env_mgr.is_development()
        is_test = env_mgr.is_testing()
        is_prod = env_mgr.is_production()
        
        # 应该只有一个为True
        env_flags = sum([is_dev, is_test, is_prod])
        assert env_flags == 1, f"环境检测异常: dev={is_dev}, test={is_test}, prod={is_prod}"
        print(f"  ✅ 环境检测: 开发={is_dev}, 测试={is_test}, 生产={is_prod}")
        
        print("  ✅ 环境管理器测试完成")
        return True
        
    except Exception as e:
        print(f"  ❌ 环境管理器测试失败: {e}")
        return False


def test_config_file_loading(config_dir: Path, verbose: bool = False) -> bool:
    """测试配置文件加载"""
    print("📁 测试配置文件加载...")
    
    try:
        # 创建配置管理器
        config_mgr = ConfigManager(str(config_dir), debug=verbose)
        
        # 加载示例配置
        models_config_file = config_dir / "models_development.json"
        if models_config_file.exists():
            success = config_mgr.load_from_file(str(models_config_file))
            assert success, "模型配置文件加载失败"
            print(f"  ✅ 配置文件加载: {models_config_file.name}")
            
            # 验证加载的配置
            if "models" in config_mgr.sections:
                section = config_mgr.sections["models"]
                if "lightgbm_model_path" in section.values:
                    model_path = section.get_value("lightgbm_model_path")
                    print(f"    模型路径: {model_path}")
        
        # 测试配置保存
        success = config_mgr.save_section("http")
        assert success, "配置保存失败"
        print(f"  ✅ 配置保存: HTTP配置已保存")
        
        config_mgr.close()
        print("  ✅ 配置文件测试完成")
        return True
        
    except Exception as e:
        print(f"  ❌ 配置文件测试失败: {e}")
        return False


def test_integration(config_dir: Path, verbose: bool = False) -> bool:
    """集成测试"""
    print("🔗 集成测试...")
    
    try:
        # 确保必需的环境变量已设置
        os.environ["PIXLY_ENV"] = "development"
        os.environ["PIXLY_TEST_TOKEN"] = "test_token"
        os.environ["PIXLY_API_KEY"] = "test_api_key"
        os.environ["PIXLY_SECRET_KEY"] = "test_secret"
        
        # 同时使用配置管理器和环境管理器
        config_mgr = ConfigManager(str(config_dir), debug=verbose)
        env_mgr = EnvironmentManager(str(config_dir), debug=verbose)
        
        # 移除禁止的环境变量以测试生产环境
        os.environ.pop("PIXLY_DEBUG", None)
        os.environ.pop("DEBUG", None)
        
        # 切换到生产环境
        success = env_mgr.activate_environment("production")
        assert success, "生产环境激活失败"
        
        # 验证生产环境特定配置
        is_prod = env_mgr.is_production()
        assert is_prod, "生产环境检测失败"
        
        # 检查生产环境的HTTP配置
        http_config = env_mgr.get_service_config("http")
        assert http_config.get("port") == 80, "生产环境端口应为80"
        print(f"  ✅ 生产环境配置: 端口{http_config['port']}")
        
        # 检查环境变量设置
        pixly_env = os.getenv("PIXLY_ENV")
        assert pixly_env == "production", f"PIXLY_ENV应为production，实际{pixly_env}"
        print(f"  ✅ 环境变量: PIXLY_ENV = {pixly_env}")
        
        # 测试配置与环境的协同
        validation = env_mgr.validate_current_environment()
        config_validation = config_mgr.validate_all()
        
        overall_valid = validation['valid'] and all(
            all(results.values()) 
            for results in config_validation.values()
        )
        print(f"  ✅ 整体验证: {'通过' if overall_valid else '有问题'}")
        
        config_mgr.close()
        print("  ✅ 集成测试完成")
        return True
        
    except Exception as e:
        print(f"  ❌ 集成测试失败: {e}")
        return False


def run_performance_test(config_dir: Path, iterations: int = 1000) -> bool:
    """性能测试"""
    print(f"⚡ 性能测试 ({iterations}次迭代)...")
    
    try:
        config_mgr = ConfigManager(str(config_dir))
        
        # 测试配置读取性能
        start_time = time.time()
        for i in range(iterations):
            port = config_mgr.get_value("http", "port", 8080)
        read_time = time.time() - start_time
        
        read_rate = iterations / read_time
        print(f"  ✅ 读取性能: {read_rate:.1f} 读取/秒")
        
        # 测试配置写入性能
        start_time = time.time()
        for i in range(iterations // 10):  # 写入测试较少次数
            config_mgr.set_value("http", "port", 8080 + i, persist=False)
        write_time = time.time() - start_time
        
        write_rate = (iterations // 10) / write_time
        print(f"  ✅ 写入性能: {write_rate:.1f} 写入/秒")
        
        config_mgr.close()
        
        # 性能要求检查
        assert read_rate > 1000, f"读取性能不足: {read_rate:.1f}/秒 < 1000/秒"
        assert write_rate > 100, f"写入性能不足: {write_rate:.1f}/秒 < 100/秒"
        
        print("  ✅ 性能测试完成")
        return True
        
    except Exception as e:
        print(f"  ❌ 性能测试失败: {e}")
        return False


def main():
    """主函数"""
    parser = argparse.ArgumentParser(
        description="配置管理系统测试工具",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__
    )
    
    parser.add_argument(
        '--config-dir',
        default='test_config',
        help='配置目录 (默认: test_config)'
    )
    
    parser.add_argument(
        '--environment',
        choices=['development', 'testing', 'production'],
        default='development',
        help='测试环境 (默认: development)'
    )
    
    parser.add_argument(
        '--create-samples',
        action='store_true',
        help='创建示例配置文件'
    )
    
    parser.add_argument(
        '--verbose', '-v',
        action='store_true',
        help='详细输出'
    )
    
    parser.add_argument(
        '--cleanup',
        action='store_true',
        help='测试后清理文件'
    )
    
    parser.add_argument(
        '--performance',
        action='store_true',
        help='运行性能测试'
    )
    
    args = parser.parse_args()
    
    # 设置日志
    setup_logging(args.verbose)
    
    # 创建测试目录
    if args.config_dir == 'test_config':
        # 使用临时目录
        test_dir = Path(tempfile.mkdtemp(prefix="pixly_config_test_"))
        config_dir = test_dir / "config"
    else:
        config_dir = Path(args.config_dir)
    
    try:
        print("🧪 配置管理系统全面测试")
        print("=" * 50)
        print(f"配置目录: {config_dir}")
        print(f"测试环境: {args.environment}")
        print()
        
        # 创建示例配置
        if args.create_samples:
            print("📝 创建示例配置文件...")
            create_sample_configs(config_dir)
            print("  ✅ 示例配置创建完成")
            print()
        
        # 运行测试
        test_results = []
        
        # 配置管理器测试
        test_results.append(test_config_manager(config_dir, args.verbose))
        print()
        
        # 环境管理器测试
        test_results.append(test_environment_manager(config_dir, args.verbose))
        print()
        
        # 配置文件加载测试
        if args.create_samples:
            test_results.append(test_config_file_loading(config_dir, args.verbose))
            print()
        
        # 集成测试
        test_results.append(test_integration(config_dir, args.verbose))
        print()
        
        # 性能测试
        if args.performance:
            test_results.append(run_performance_test(config_dir))
            print()
        
        # 测试结果汇总
        passed = sum(test_results)
        total = len(test_results)
        
        print("📊 测试结果汇总")
        print("=" * 50)
        print(f"总测试: {total}")
        print(f"通过: {passed}")
        print(f"失败: {total - passed}")
        print(f"成功率: {passed/total*100:.1f}%")
        
        if passed == total:
            print("🎉 所有测试通过！")
            exit_code = 0
        else:
            print("💥 有测试失败！")
            exit_code = 1
        
    finally:
        # 清理
        if args.cleanup and args.config_dir == 'test_config':
            print(f"\n🧹 清理测试目录: {test_dir}")
            shutil.rmtree(test_dir, ignore_errors=True)
    
    return exit_code


if __name__ == "__main__":
    sys.exit(main())
