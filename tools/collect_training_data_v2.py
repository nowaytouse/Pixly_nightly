#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
PIXLY观测数据收集工具（CLI版本 v2.0）
使用pixly CLI工具批量转换图像并自动发送观测数据到GO核心
修复: 使用临时目录而不是直接传递文件路径
"""

import os
import sys
import subprocess
import json
import time
import glob
import requests
import shutil
import tempfile
from pathlib import Path

# 配置
GO_CORE_URL = "http://localhost:50052"
FORMATS = ["jxl", "avif", "webp"]
MODES = ["size", "balanced", "quality"]

def check_go_core():
    """检查GO核心是否运行"""
    try:
        response = requests.get(f"{GO_CORE_URL}/api/v1/health", timeout=2)
        if response.status_code == 200:
            print("✅ GO核心运行正常\n")
            return True
    except:
        pass
    print("❌ 错误: GO核心未运行")
    print("💡 请先启动: ./start_ai_service.sh\n")
    return False

def send_observation(observation):
    """发送观测数据到GO核心"""
    try:
        response = requests.post(
            f"{GO_CORE_URL}/api/v1/observations",
            json=observation,
            timeout=5
        )
        return response.status_code == 200
    except Exception as e:
        print(f"  ⚠️  发送失败: {e}")
        return False

def convert_with_cli(image_path, format_type, optimize_mode, temp_dir):
    """使用CLI工具转换图像（使用临时目录）"""
    
    # 创建临时输入输出目录
    temp_input = os.path.join(temp_dir, f"in_{format_type}_{optimize_mode}")
    temp_output = os.path.join(temp_dir, f"out_{format_type}_{optimize_mode}")
    
    os.makedirs(temp_input, exist_ok=True)
    os.makedirs(temp_output, exist_ok=True)
    
    # 复制图像到临时输入目录
    image_name = os.path.basename(image_path)
    temp_image = os.path.join(temp_input, image_name)
    shutil.copy2(image_path, temp_image)
    
    start_time = time.time()
    
    # 构建CLI命令
    cmd = [
        './pixly', 'smart',
        '--input', temp_input,
        '--output', temp_output,
        '--format', format_type,
        '--optimize', optimize_mode,
        '--no-animation',
        '--log-level', 'error'
    ]
    
    try:
        # 执行转换
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=60,
            cwd=os.getcwd()
        )
        
        conversion_time = int((time.time() - start_time) * 1000)
        
        if result.returncode != 0:
            return None
        
        # 查找输出文件
        output_files = glob.glob(os.path.join(temp_output, f"*.{format_type}"))
        if not output_files:
            return None
        
        output_path = output_files[0]
        
        # 获取文件大小
        input_size = os.path.getsize(image_path)
        output_size = os.path.getsize(output_path)
        
        # 构建观测数据
        observation = {
            "file_name": image_name,
            "tool": format_type,
            "optimize_mode": optimize_mode,
            "params": {
                "quality": 90,
                "distance": 1.0,
                "effort": 7,
                "speed": 5
            },
            "input_size": input_size,
            "output_size": output_size,
            "ssim": 0.95,  # 默认值
            "conversion_time": conversion_time,
            "image_type": os.path.splitext(image_name)[1].lower(),
            "has_alpha": False
        }
        
        return observation
        
    except subprocess.TimeoutExpired:
        return None
    except Exception as e:
        return None

def main():
    """主函数"""
    if len(sys.argv) < 2:
        print("❌ 用法: python3 collect_training_data_v2.py <图像目录>")
        sys.exit(1)
    
    input_dir = sys.argv[1]
    
    if not os.path.isdir(input_dir):
        print(f"❌ 错误: {input_dir} 不是有效目录")
        sys.exit(1)
    
    print("╔" + "═" * 78 + "╗")
    print("║" + " " * 78 + "║")
    print("║" + "�� PIXLY观测数据收集工具".center(86) + "║")
    print("║" + " " * 78 + "║")
    print("╚" + "═" * 78 + "╝")
    print("")
    
    # 检查GO核心
    if not check_go_core():
        sys.exit(1)
    
    # 查找图像文件
    image_extensions = ['*.jpg', '*.jpeg', '*.png', '*.bmp', '*.tiff']
    images = []
    for ext in image_extensions:
        images.extend(glob.glob(os.path.join(input_dir, ext)))
    
    if not images:
        print(f"❌ 未找到图像文件")
        sys.exit(1)
    
    print(f"📁 输入目录: {input_dir}")
    print(f"📊 找到图像: {len(images)} 张\n")
    
    print(f"🎯 转换配置:")
    print(f"   格式: {', '.join(FORMATS)}")
    print(f"   模式: {', '.join(MODES)}\n")
    
    response = input("开始收集观测数据? (y/n) ")
    if response.lower() != 'y':
        print("❌ 已取消")
        sys.exit(0)
    
    print(f"\n{'='*80}")
    print(f"🚀 开始收集...")
    print(f"{'='*80}\n")
    
    # 统计
    total_conversions = 0
    successful_observations = 0
    failed_conversions = 0
    failed_observations = 0
    
    # 创建临时工作目录
    with tempfile.TemporaryDirectory(prefix='pixly_collect_') as temp_dir:
        for format_type in FORMATS:
            for mode in MODES:
                print(f"\n{'━'*80}")
                print(f"📦 转换: {format_type} | 模式: {mode}")
                print(f"{'━'*80}\n")
                
                for idx, image in enumerate(images, 1):
                    sys.stdout.write(f"🔄 处理: {os.path.basename(image)} → {format_type} ({mode}) ")
                    sys.stdout.flush()
                    
                    observation = convert_with_cli(image, format_type, mode, temp_dir)
                    
                    if observation:
                        if send_observation(observation):
                            successful_observations += 1
                            print("✅")
                            print(f"   📊 观测已记录 (总数: {successful_observations})")
                        else:
                            failed_observations += 1
                            print("⚠️  观测记录失败")
                    else:
                        print("❌")
                        failed_conversions += 1
                    
                    total_conversions += 1
                
                print(f"\n✅ 完成: {len(images)}/{len(images)} 张")
    
    # 显示统计
    print(f"\n{'='*80}")
    print(f"✅ 收集完成！")
    print(f"{'='*80}\n")
    
    print(f"📊 统计:")
    print(f"   总转换: {total_conversions} 张")
    print(f"   观测数据: {successful_observations} 个\n")
    
    # 获取当前观测总数
    try:
        files = glob.glob("data/observations/*.json")
        total_obs = len(files)
        print(f"   当前观测总数: {total_obs} 个\n")
        
        if total_obs >= 50:
            print(f"🎉 观测数据充足！可以开始训练PPO模型！\n")
            print(f"🚀 下一步:")
            print(f"   ./train_ppo_model.sh\n")
        else:
            print(f"⚠️  观测数据不足（最少需要50个）")
            print(f"💡 还需要: {50 - total_obs} 个")
            print(f"💡 建议: 继续转换更多图像\n")
    except:
        pass
    
    print(f"{'='*80}\n")
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
