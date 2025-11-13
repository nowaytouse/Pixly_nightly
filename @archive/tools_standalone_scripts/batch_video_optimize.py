#!/usr/bin/env python3
"""
PIXLY 视频批量优化脚本 v1.0
Phase 46: 批量视频智能转码

功能:
- 扫描目录下所有视频文件
- 调用AI预测获取最优参数
- 批量转码优化
- 进度追踪和统计
- 错误处理和重试

使用:
python3 tools/batch_video_optimize.py <input_dir> <output_dir> [--mode balanced] [--recursive] [--dry-run]
"""

import os
import sys
import json
import subprocess
import argparse
from pathlib import Path
from datetime import datetime
from typing import List, Dict, Optional
import shutil

# 支持的视频格式
VIDEO_EXTENSIONS = {
    '.mp4', '.mkv', '.avi', '.mov', '.webm', '.flv', '.wmv', 
    '.m4v', '.mpg', '.mpeg', '.3gp', '.ogv', '.ts', '.mts'
}

class BatchVideoOptimizer:
    def __init__(self, input_dir: str, output_dir: str, mode: str = 'balanced', 
                 recursive: bool = False, dry_run: bool = False):
        self.input_dir = Path(input_dir)
        self.output_dir = Path(output_dir)
        self.mode = mode
        self.recursive = recursive
        self.dry_run = dry_run
        
        # 统计信息
        self.total_files = 0
        self.processed_files = 0
        self.failed_files = 0
        self.skipped_files = 0
        self.total_size_before = 0
        self.total_size_after = 0
        
        # 查找必要工具
        self.ffmpeg_path = self._find_ffmpeg()
        self.predict_script = self._find_predict_script()
        
    def _find_ffmpeg(self) -> str:
        """查找ffmpeg路径"""
        for path in ['/opt/homebrew/bin/ffmpeg', '/usr/local/bin/ffmpeg', 
                     '/usr/bin/ffmpeg', 'ffmpeg']:
            result = subprocess.run(['which', path.split('/')[-1]], 
                                   capture_output=True, text=True)
            if result.returncode == 0:
                return result.stdout.strip()
        raise FileNotFoundError("❌ ffmpeg未找到")
    
    def _find_predict_script(self) -> Path:
        """查找AI预测脚本"""
        script_path = Path('tools/predict_video_params.py')
        if not script_path.exists():
            script_path = Path('predict_video_params.py')
        if not script_path.exists():
            raise FileNotFoundError("❌ predict_video_params.py未找到")
        return script_path
    
    def scan_videos(self) -> List[Path]:
        """扫描视频文件"""
        print(f"📂 扫描目录: {self.input_dir}")
        print(f"🔍 递归扫描: {'是' if self.recursive else '否'}")
        
        videos = []
        if self.recursive:
            for ext in VIDEO_EXTENSIONS:
                videos.extend(self.input_dir.rglob(f"*{ext}"))
        else:
            for ext in VIDEO_EXTENSIONS:
                videos.extend(self.input_dir.glob(f"*{ext}"))
        
        self.total_files = len(videos)
        print(f"✅ 找到 {self.total_files} 个视频文件")
        return sorted(videos)
    
    def predict_params(self, video_path: Path) -> Optional[Dict]:
        """调用AI预测获取最优参数"""
        try:
            cmd = [
                'python3',
                str(self.predict_script),
                str(video_path),
                self.mode,
                '{}'  # 空的options JSON
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
            
            if result.returncode != 0:
                print(f"⚠️  AI预测失败: {video_path.name}")
                return None
            
            # 解析JSON输出
            output = result.stdout.strip()
            data = json.loads(output)
            
            if not data.get('success'):
                print(f"⚠️  AI预测返回失败: {video_path.name}")
                return None
            
            return data
            
        except subprocess.TimeoutExpired:
            print(f"⏱️  AI预测超时: {video_path.name}")
            return None
        except json.JSONDecodeError:
            print(f"❌ AI预测输出解析失败: {video_path.name}")
            return None
        except Exception as e:
            print(f"❌ AI预测异常: {video_path.name} - {e}")
            return None
    
    def build_ffmpeg_command(self, input_path: Path, output_path: Path, 
                            params: Dict) -> List[str]:
        """构建ffmpeg命令"""
        video_params = params['params']
        encoder = video_params['encoder']
        crf = video_params['crf']
        preset = video_params['preset']
        
        # 映射编码器到ffmpeg名称
        encoder_map = {
            'h264': 'libx264',
            'h265': 'libx265',
            'av1': 'libsvtav1',
            'vp9': 'libvpx-vp9'
        }
        
        ffmpeg_encoder = encoder_map.get(encoder, 'libx265')
        
        cmd = [
            self.ffmpeg_path,
            '-i', str(input_path),
            '-c:v', ffmpeg_encoder,
            '-crf', str(crf),
            '-preset', preset,
            '-c:a', 'copy',  # 音频直接复制
        ]
        
        # 添加分辨率缩放
        if video_params.get('scale'):
            cmd.extend(['-vf', f"scale={video_params['scale']}"])
        
        # 添加帧率调整
        if video_params.get('fps'):
            cmd.extend(['-r', str(video_params['fps'])])
        
        # 添加码率限制
        if video_params.get('target_bitrate'):
            cmd.extend(['-b:v', f"{video_params['target_bitrate']}k"])
        
        if video_params.get('max_bitrate'):
            cmd.extend(['-maxrate', f"{video_params['max_bitrate']}k",
                       '-bufsize', f"{video_params['max_bitrate'] * 2}k"])
        
        # 两遍编码
        if video_params.get('two_pass'):
            # 这里简化处理，实际需要两次调用
            cmd.extend(['-pass', '1', '-f', 'null', '/dev/null', '&&',
                       self.ffmpeg_path, '-i', str(input_path),
                       '-c:v', ffmpeg_encoder, '-crf', str(crf),
                       '-preset', preset, '-pass', '2'])
        
        cmd.extend(['-y', str(output_path)])
        
        return cmd
    
    def optimize_video(self, video_path: Path) -> bool:
        """优化单个视频"""
        print(f"\n{'='*60}")
        print(f"📹 处理: {video_path.name}")
        
        # 获取原始文件大小
        original_size = video_path.stat().st_size
        self.total_size_before += original_size
        
        # 构建输出路径
        relative_path = video_path.relative_to(self.input_dir)
        output_path = self.output_dir / relative_path
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
        # 修改输出文件名（添加_optimized）
        output_path = output_path.with_stem(f"{output_path.stem}_optimized")
        
        # 检查是否已存在
        if output_path.exists():
            print(f"⏭️  跳过（已存在）: {output_path.name}")
            self.skipped_files += 1
            return True
        
        # AI预测参数
        print(f"🤖 AI预测参数...")
        params = self.predict_params(video_path)
        
        if not params:
            print(f"❌ 跳过（预测失败）: {video_path.name}")
            self.failed_files += 1
            return False
        
        # 输出预测结果
        video_params = params['params']
        print(f"📊 编码器: {video_params['encoder']}")
        print(f"📊 CRF: {video_params['crf']}")
        print(f"📊 Preset: {video_params['preset']}")
        
        if params.get('video_type'):
            vtype = params['video_type']
            print(f"🎭 类型: {vtype.get('type', 'unknown')} (置信度: {vtype.get('confidence', 0)*100:.1f}%)")
        
        if self.dry_run:
            print(f"🏃 Dry-run模式，跳过实际转码")
            self.processed_files += 1
            return True
        
        # 构建ffmpeg命令
        cmd = self.build_ffmpeg_command(video_path, output_path, params)
        
        # 执行转码
        print(f"⚙️  开始转码...")
        try:
            result = subprocess.run(
                cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                timeout=3600  # 1小时超时
            )
            
            if result.returncode != 0:
                print(f"❌ 转码失败: {video_path.name}")
                error_msg = result.stderr.decode('utf-8', errors='ignore')
                print(f"错误信息: {error_msg[:200]}")
                self.failed_files += 1
                return False
            
            # 获取输出文件大小
            if output_path.exists():
                output_size = output_path.stat().st_size
                self.total_size_after += output_size
                
                reduction = (1 - output_size / original_size) * 100
                print(f"✅ 转码成功")
                print(f"📊 原始大小: {original_size / 1024 / 1024:.2f} MB")
                print(f"📊 优化后: {output_size / 1024 / 1024:.2f} MB")
                print(f"📊 压缩率: {reduction:.1f}%")
                
                self.processed_files += 1
                return True
            else:
                print(f"❌ 输出文件不存在: {output_path}")
                self.failed_files += 1
                return False
                
        except subprocess.TimeoutExpired:
            print(f"⏱️  转码超时: {video_path.name}")
            self.failed_files += 1
            return False
        except Exception as e:
            print(f"❌ 转码异常: {video_path.name} - {e}")
            self.failed_files += 1
            return False
    
    def run(self):
        """执行批量优化"""
        print("\n" + "="*60)
        print("🎬 PIXLY 视频批量优化工具 v1.0")
        print("="*60)
        
        # 检查输入目录
        if not self.input_dir.exists():
            print(f"❌ 输入目录不存在: {self.input_dir}")
            return
        
        # 创建输出目录
        self.output_dir.mkdir(parents=True, exist_ok=True)
        
        # 扫描视频
        videos = self.scan_videos()
        
        if not videos:
            print("⚠️  未找到视频文件")
            return
        
        # 开始时间
        start_time = datetime.now()
        
        # 处理每个视频
        for i, video_path in enumerate(videos, 1):
            print(f"\n进度: [{i}/{self.total_files}]")
            self.optimize_video(video_path)
        
        # 结束时间
        end_time = datetime.now()
        duration = (end_time - start_time).total_seconds()
        
        # 输出统计信息
        print("\n" + "="*60)
        print("📊 批量优化完成")
        print("="*60)
        print(f"总文件数: {self.total_files}")
        print(f"成功处理: {self.processed_files}")
        print(f"失败: {self.failed_files}")
        print(f"跳过: {self.skipped_files}")
        print(f"总耗时: {duration:.1f}秒")
        
        if not self.dry_run and self.processed_files > 0:
            print(f"\n原始总大小: {self.total_size_before / 1024 / 1024 / 1024:.2f} GB")
            print(f"优化后总大小: {self.total_size_after / 1024 / 1024 / 1024:.2f} GB")
            total_reduction = (1 - self.total_size_after / self.total_size_before) * 100
            print(f"总压缩率: {total_reduction:.1f}%")
            print(f"节省空间: {(self.total_size_before - self.total_size_after) / 1024 / 1024 / 1024:.2f} GB")


def main():
    parser = argparse.ArgumentParser(
        description='PIXLY 视频批量优化工具',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
示例:
  python3 batch_video_optimize.py ~/Videos/input ~/Videos/output
  python3 batch_video_optimize.py ~/Videos/input ~/Videos/output --mode quality --recursive
  python3 batch_video_optimize.py ~/Videos/input ~/Videos/output --dry-run
        """
    )
    
    parser.add_argument('input_dir', help='输入目录')
    parser.add_argument('output_dir', help='输出目录')
    parser.add_argument('--mode', choices=['size', 'balanced', 'quality'], 
                       default='balanced', help='优化模式（默认: balanced）')
    parser.add_argument('--recursive', '-r', action='store_true', 
                       help='递归扫描子目录')
    parser.add_argument('--dry-run', action='store_true', 
                       help='仅预测参数，不实际转码')
    
    args = parser.parse_args()
    
    optimizer = BatchVideoOptimizer(
        input_dir=args.input_dir,
        output_dir=args.output_dir,
        mode=args.mode,
        recursive=args.recursive,
        dry_run=args.dry_run
    )
    
    try:
        optimizer.run()
    except KeyboardInterrupt:
        print("\n\n⚠️  用户中断")
        sys.exit(1)
    except Exception as e:
        print(f"\n\n❌ 异常: {e}")
        sys.exit(1)


if __name__ == '__main__':
    main()
