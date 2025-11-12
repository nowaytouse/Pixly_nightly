#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
PIXLY观测数据收集工具（CLI版本）
使用pixly CLI工具批量转换图像并自动发送观测数据到GO核心
"""

import os
import sys
import subprocess
import json
import time
import glob
import requests
from pathlib import Path
from typing import List, Dict, Optional
import shutil
import tempfile

# 配置
GO_CORE_URL = "http://localhost:50052"
FORMATS = ["jxl", "avif", "webp"]
MODES = ["size", "balanced", "quality"]
OUTPUT_BASE = "output_observations"


class ObservationCollector:
    """观测数据收集器"""
    
    def __init__(self, input_dir: str):
        self.input_dir = Path(input_dir)
        self.output_base = Path(OUTPUT_BASE)
        self.output_base.mkdir(exist_ok=True)
        
        # 检查GO核心
        self.check_go_core()
        
        # 统计图像
        self.images = self.find_images()
        
    def check_go_core(self):
        """检查GO核心是否运行"""
        try:
            resp = requests.get(f"{GO_CORE_URL}/api/v1/health", timeout=2)
            if resp.status_code == 200:
                print("✅ GO核心运行正常")
                return True
        except:
            pass
        
        print("❌ 错误: GO核心未运行")
        print("💡 请先启动: ./start_ai_service.sh")
        sys.exit(1)
    
    def find_images(self) -> List[Path]:
        """查找图像文件"""
        images = []
        for ext in ['*.jpg', '*.jpeg', '*.png', '*.webp']:
            images.extend(self.input_dir.glob(ext))
            images.extend(self.input_dir.glob(ext.upper()))
        
        print(f"📁 输入目录: {self.input_dir}")
        print(f"📊 找到图像: {len(images)} 张")
        
        if len(images) == 0:
            print("❌ 未找到图像文件")
            print("💡 支持格式: .jpg, .jpeg, .png, .webp")
            sys.exit(1)
        
        return images
    
    def convert_image(self, image_path: Path, format: str, mode: str) -> Tuple[bool, Dict]:
        """转换单张图像"""
        output_dir = self.output_base / f"{format}_{mode}"
        output_dir.mkdir(exist_ok=True)
        
        # 构建CLI命令
        cmd = [
            "./pixly", "smart",
            "--input", str(image_path),
            "--output", str(output_dir),
            "--format", format,
            "--optimize", mode,
            "--ssim-optimize",
            "--no-animation",
            "--no-banner"
        ]
        
        # 记录开始时间
        start_time = time.time()
        
        try:
            # 执行转换
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=60
            )
            
            # 记录结束时间
            end_time = time.time()
            conversion_time = int((end_time - start_time) * 1000)  # 毫秒
            
            if result.returncode != 0:
                print(f"⚠️  转换失败: {image_path.name}")
                return False, {}
            
            # 查找输出文件
            output_file = output_dir / f"{image_path.stem}.{format}"
            if not output_file.exists():
                # 尝试其他可能的扩展名
                possible_outputs = list(output_dir.glob(f"{image_path.stem}.*"))
                if possible_outputs:
                    output_file = possible_outputs[0]
                else:
                    return False, {}
            
            # 获取文件大小
            input_size = image_path.stat().st_size
            output_size = output_file.stat().st_size
            
            # 构建观测数据
            observation = {
                "file_name": image_path.name,
                "tool": format,
                "optimize_mode": mode,
                "params": self.get_default_params(format, mode),
                "input_size": input_size,
                "output_size": output_size,
                "ssim": 0.95,  # CLI工具没有返回SSIM，使用默认值
                "conversion_time": conversion_time,
                "image_type": image_path.suffix.lower(),
                "has_alpha": False
            }
            
            # 计算奖励
            compression_ratio = output_size / input_size
            speed_score = max(0, 1 - conversion_time / 5000)
            observation["reward"] = (
                0.5 * observation["ssim"] +
                0.3 * (1 - compression_ratio) +
                0.2 * speed_score
            )
            
            return True, observation
            
        except subprocess.TimeoutExpired:
            print(f"⏱️  转换超时: {image_path.name}")
            return False, {}
        except Exception as e:
            print(f"❌ 转换错误: {image_path.name} - {e}")
            return False, {}
    
    def get_default_params(self, format: str, mode: str) -> Dict:
        """获取默认参数（因为CLI没有返回）"""
        params_map = {
            "jxl": {
                "size": {"quality": 85, "distance": 1.5, "effort": 7, "speed": 6},
                "balanced": {"quality": 90, "distance": 0.8, "effort": 7, "speed": 5},
                "quality": {"quality": 95, "distance": 0.3, "effort": 9, "speed": 3}
            },
            "avif": {
                "size": {"quality": 50, "speed": 8, "effort": 5},
                "balanced": {"quality": 60, "speed": 6, "effort": 6},
                "quality": {"quality": 65, "speed": 4, "effort": 7}
            },
            "webp": {
                "size": {"quality": 75, "method": 4},
                "balanced": {"quality": 82, "method": 5},
                "quality": {"quality": 90, "method": 6}
            }
        }
        
        return params_map.get(format, {}).get(mode, {})
    
    def send_observation(self, observation: Dict) -> bool:
        """发送观测数据到GO核心"""
        try:
            resp = requests.post(
                f"{GO_CORE_URL}/api/v1/observations",
                json=observation,
                timeout=5
            )
            
            if resp.status_code == 200:
                data = resp.json()
                total = data.get("total_observations", 0)
                print(f"   📊 观测已记录 (总数: {total})")
                return True
            else:
                print(f"   ⚠️  记录失败: HTTP {resp.status_code}")
                return False
                
        except Exception as e:
            print(f"   ❌ 发送失败: {e}")
            return False
    
    def collect(self):
        """收集观测数据"""
        print("")
        print("🎯 转换配置:")
        print(f"   格式: {', '.join(FORMATS)}")
        print(f"   模式: {', '.join(MODES)}")
        print("")
        
        # 询问是否继续
        resp = input("开始收集观测数据? (y/n) ").strip().lower()
        if resp != 'y':
            print("❌ 取消")
            sys.exit(0)
        
        print("")
        print("=" * 80)
        print("🚀 开始收集...")
        print("=" * 80)
        print("")
        
        total_converted = 0
        total_observations = 0
        
        # 遍历所有组合
        for format in FORMATS:
            for mode in MODES:
                print("━" * 80)
                print(f"📦 转换: {format} | 模式: {mode}")
                print("━" * 80)
                print("")
                
                converted_count = 0
                
                for image in self.images:
                    print(f"🔄 处理: {image.name} → {format} ({mode})", end=" ")
                    
                    success, observation = self.convert_image(image, format, mode)
                    
                    if success:
                        # 发送观测数据
                        if self.send_observation(observation):
                            converted_count += 1
                            total_observations += 1
                        print("✅")
                    else:
                        print("❌")
                    
                    time.sleep(0.5)
                
                total_converted += converted_count
                print("")
                print(f"✅ 完成: {converted_count}/{len(self.images)} 张")
                print("")
        
        print("")
        print("=" * 80)
        print("✅ 收集完成！")
        print("=" * 80)
        print("")
        print("📊 统计:")
        print(f"   总转换: {total_converted} 张")
        print(f"   观测数据: {total_observations} 个")
        print("")
        
        # 检查是否足够训练
        obs_files = list(Path("data/observations").glob("*.json"))
        total_obs = len(obs_files)
        
        print(f"   当前观测总数: {total_obs} 个")
        print("")
        
        if total_obs >= 50:
            print("🎉 观测数据充足！可以开始训练PPO模型！")
            print("")
            print("🚀 下一步:")
            print("   ./train_ppo_model.sh")
            print("")
        else:
            needed = 50 - total_obs
            print(f"⚠️  观测数据不足（最少需要50个）")
            print(f"💡 还需要: {needed} 个")
            print(f"💡 建议: 继续转换更多图像")
            print("")
        
        print("=" * 80)


def convert_with_cli(
    image_path: str, 
    format_type: str, 
    optimize_mode: str,
    temp_dir: str,
    output_base: str
) -> Optional[Dict]:
    """使用CLI工具转换图像（使用临时目录）"""
    
    # 创建临时输入目录（每个格式+模式组合一个）
    temp_input = os.path.join(temp_dir, f"input_{format_type}_{optimize_mode}")
    temp_output = os.path.join(temp_dir, f"output_{format_type}_{optimize_mode}")
    
    os.makedirs(temp_input, exist_ok=True)
    os.makedirs(temp_output, exist_ok=True)
    
    # 复制图像到临时输入目录
    image_name = os.path.basename(image_path)
    temp_image = os.path.join(temp_input, image_name)
    shutil.copy2(image_path, temp_image)
    
    start_time = time.time()
    
    # 构建CLI命令（使用目录输入）
    cmd = [
        './pixly', 'smart',
        '--input', temp_input,
        '--output', temp_output,
        '--format', format_type,
        '--optimize', optimize_mode,
        '--no-animation',
        '--log-level', 'error'  # 减少日志输出
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
            print(f"❌ CLI转换失败: {result.stderr[:200]}")
            return None
        
        # 查找输出文件
        output_files = glob.glob(os.path.join(temp_output, f"*.{format_type}"))
        if not output_files:
            print(f"⚠️  未找到输出文件")
            return None
        
        output_path = output_files[0]
        
        # 获取文件大小
        input_size = os.path.getsize(image_path)
        output_size = os.path.getsize(output_path)
        
        # TODO: 实际计算SSIM（需要GO核心API或独立工具）
        ssim = 0.95  # 暂时使用默认值
        
        observation = {
            "file_name": image_name,
            "tool": format_type,
            "optimize_mode": optimize_mode,
            "params": {
                "quality": 90,  # CLI自动选择
                "distance": 1.0,
                "effort": 7,
                "speed": 5
            },
            "input_size": input_size,
            "output_size": output_size,
            "ssim": ssim,
            "conversion_time": conversion_time,
            "image_type": os.path.splitext(image_name)[1].lower(),
            "has_alpha": False  # TODO: 从图像元数据检测
        }
        
        return observation
        
    except subprocess.TimeoutExpired:
        print(f"⏱️  转换超时")
        return None
    except Exception as e:
        print(f"❌ 转换异常: {e}")
        return None


def main():
    """主函数"""
    if len(sys.argv) < 2:
        print("❌ 用法: python3 collect_training_data.py <图像目录>")
        sys.exit(1)
    
    input_dir = sys.argv[1]
    
    if not os.path.isdir(input_dir):
        print(f"❌ 错误: {input_dir} 不是有效目录")
        sys.exit(1)
    
    print("╔" + "═" * 78 + "╗")
    print("║" + " " * 78 + "║")
    print("║" + "              🤖 PIXLY观测数据收集工具".center(78) + "║")
    print("║" + " " * 78 + "║")
    print("╚" + "═" * 78 + "╝")
    print("")
    
    collector = ObservationCollector(input_dir)
    
    # 创建临时工作目录
    with tempfile.TemporaryDirectory(prefix='pixly_collect_') as temp_dir:
        print(f"📁 临时目录: {temp_dir}\n")
        
        for format_type in FORMATS:
            for mode in MODES:
                print(f"\n{'='*80}")
                print(f"📦 转换: {format_type} | 模式: {mode}")
                print(f"{'='*80}\n")
                
                for idx, image in enumerate(collector.images, 1):
                    sys.stdout.write(f"🔄 处理: {os.path.basename(image)} → {format_type} ({mode}) ")
                    sys.stdout.flush()
                    
                    observation = convert_with_cli(
                        image, 
                        format_type, 
                        mode,
                        temp_dir,
                        OUTPUT_BASE
                    )
                    
                    if observation:
                        if collector.send_observation(observation):
                            total_observations = 0 # This variable is not defined in the original code, assuming it's meant to be total_observations
                            successful_observations = 0 # This variable is not defined in the original code, assuming it's meant to be successful_observations
                            failed_observations = 0 # This variable is not defined in the original code, assuming it's meant to be failed_observations
                            failed_conversions = 0 # This variable is not defined in the original code, assuming it's meant to be failed_conversions
                            total_conversions = 0 # This variable is not defined in the original code, assuming it's meant to be total_conversions
                            total_observations += 1
                            successful_observations += 1
                            print("✅")
                            print(f"   📊 观测已记录 (总数: {total_observations})")
                        else:
                            failed_observations += 1
                            print("⚠️  观测记录失败")
                    else:
                        print("❌")
                        failed_conversions += 1
                    
                    total_conversions += 1
                
                print(f"\n✅ 完成: {idx}/{len(collector.images)} 张")
        
        print(f"\n{'='*80}")
        print(f"✅ 收集完成！")
        print(f"{'='*80}\n")
    
    return 0


if __name__ == '__main__':
    sys.exit(main())
