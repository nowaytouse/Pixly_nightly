#!/usr/bin/env python3
"""
Unified PPO Training Script
Supports all media types with configurable backends.

Usage:
    python ppo_train.py --mode all          # All media types with default backend
    python ppo_train.py --mode ffmpeg       # FFmpeg only backend
    python ppo_train.py --mode hybrid       # Hybrid: Rust CLI for images, FFmpeg for video/audio
    python ppo_train.py --mode image        # Images only
    python ppo_train.py --mode video        # Videos only
    python ppo_train.py --mode audio        # Audio only
"""

import os
import sys
import json
import subprocess
import argparse
import shutil
from pathlib import Path
from datetime import datetime
import random
from typing import Dict, List, Optional, Tuple

# Add project path
sys.path.insert(0, str(Path(__file__).parent.parent))

# ============================================================
# Configuration
# ============================================================

IMAGE_EXTENSIONS = {'.jpg', '.jpeg', '.png', '.webp', '.gif', '.bmp', '.tiff', '.avif', '.jxl', '.heic'}
VIDEO_EXTENSIONS = {'.mp4', '.webm', '.mkv', '.avi', '.mov', '.ts', '.m4v'}
AUDIO_EXTENSIONS = {'.mp3', '.wav', '.ogg', '.m4a', '.flac', '.aac', '.adts', '.opus'}

TARGET_FORMATS = {
    'image': ['webp', 'avif', 'jxl'],
    'video': ['webm', 'mp4'],
    'audio': ['opus', 'aac', 'mp3']
}

# ============================================================
# Utility Functions
# ============================================================

def check_dependencies(mode: str) -> bool:
    """Check required dependencies based on mode."""
    required = {'ffmpeg': shutil.which('ffmpeg')}

    if mode in ['hybrid', 'all']:
        # Check for Rust CLI
        rust_cli_paths = [
            Path(__file__).parent.parent / 'target' / 'release' / 'pixly-kernel',
            Path(__file__).parent.parent / 'plugin' / 'converter' / 'bin' / 'pixly-rust',
        ]
        rust_cli = None
        for path in rust_cli_paths:
            if path.exists():
                rust_cli = str(path)
                break
        required['rust_cli'] = rust_cli

    missing = [name for name, path in required.items() if not path]

    if missing:
        print(f"❌ Missing dependencies: {', '.join(missing)}", file=sys.stderr)
        return False

    return True


def find_media_files(data_dir: str) -> Dict[str, List[str]]:
    """Find all media files in directory."""
    media_files = {
        'images': [],
        'videos': [],
        'audio': []
    }

    data_path = Path(data_dir)
    if not data_path.exists():
        print(f"❌ Data directory not found: {data_dir}", file=sys.stderr)
        return media_files

    for file in data_path.rglob('*'):
        if not file.is_file():
            continue

        ext = file.suffix.lower()

        if ext in IMAGE_EXTENSIONS:
            media_files['images'].append(str(file))
        elif ext in VIDEO_EXTENSIONS:
            media_files['videos'].append(str(file))
        elif ext in AUDIO_EXTENSIONS:
            media_files['audio'].append(str(file))

    return media_files


def get_target_formats(media_type: str) -> List[str]:
    """Get target formats for media type."""
    return TARGET_FORMATS.get(media_type, [])


# ============================================================
# Conversion Backends
# ============================================================

def convert_with_ffmpeg(input_file: str, output_format: str, quality: int, media_type: str) -> Optional[Dict]:
    """Convert using FFmpeg."""
    output_file = f"/tmp/ppo_test_{Path(input_file).stem}.{output_format}"

    try:
        cmd = ['ffmpeg', '-y', '-i', input_file]

        if media_type == 'image':
            if output_format == 'webp':
                cmd.extend(['-quality', str(quality)])
            elif output_format == 'avif':
                crf = int((100 - quality) * 0.63)
                cmd.extend(['-c:v', 'libaom-av1', '-crf', str(crf)])
            elif output_format == 'jxl':
                distance = (100 - quality) / 10.0
                cmd.extend(['-distance', str(distance)])
        elif media_type == 'video':
            crf = int((100 - quality) * 0.51)
            if output_format == 'webm':
                cmd.extend(['-c:v', 'libvpx-vp9', '-crf', str(crf), '-b:v', '0'])
            else:
                cmd.extend(['-c:v', 'libx264', '-crf', str(crf)])
        elif media_type == 'audio':
            bitrate = max(64, quality * 3)
            if output_format == 'opus':
                cmd.extend(['-c:a', 'libopus', '-b:a', f'{bitrate}k'])
            elif output_format == 'aac':
                cmd.extend(['-c:a', 'aac', '-b:a', f'{bitrate}k'])
            else:
                cmd.extend(['-b:a', f'{bitrate}k'])

        cmd.append(output_file)

        result = subprocess.run(cmd, capture_output=True, timeout=120)

        if result.returncode == 0 and Path(output_file).exists():
            original_size = Path(input_file).stat().st_size
            output_size = Path(output_file).stat().st_size
            compression_ratio = output_size / original_size if original_size > 0 else 1.0

            # Clean up temp file
            Path(output_file).unlink(missing_ok=True)

            return {
                'success': True,
                'original_size': original_size,
                'output_size': output_size,
                'compression_ratio': compression_ratio,
                'quality': quality,
                'format': output_format
            }

        return None

    except Exception as e:
        print(f"  ⚠️ FFmpeg error: {e}", file=sys.stderr)
        Path(output_file).unlink(missing_ok=True)
        return None


def convert_with_rust_cli(input_file: str, output_format: str, quality: int) -> Optional[Dict]:
    """Convert image using Rust CLI."""
    rust_cli_paths = [
        Path(__file__).parent.parent / 'target' / 'release' / 'pixly-kernel',
        Path(__file__).parent.parent / 'plugin' / 'converter' / 'bin' / 'pixly-rust',
    ]

    rust_cli = None
    for path in rust_cli_paths:
        if path.exists():
            rust_cli = str(path)
            break

    if not rust_cli:
        return convert_with_ffmpeg(input_file, output_format, quality, 'image')

    output_file = f"/tmp/ppo_test_{Path(input_file).stem}.{output_format}"

    try:
        cmd = [rust_cli, 'convert', input_file, '-o', output_file, '-q', str(quality), '-f', output_format]
        result = subprocess.run(cmd, capture_output=True, timeout=60)

        if result.returncode == 0 and Path(output_file).exists():
            original_size = Path(input_file).stat().st_size
            output_size = Path(output_file).stat().st_size
            compression_ratio = output_size / original_size if original_size > 0 else 1.0

            Path(output_file).unlink(missing_ok=True)

            return {
                'success': True,
                'original_size': original_size,
                'output_size': output_size,
                'compression_ratio': compression_ratio,
                'quality': quality,
                'format': output_format
            }

        # Fallback to FFmpeg
        return convert_with_ffmpeg(input_file, output_format, quality, 'image')

    except Exception as e:
        print(f"  ⚠️ Rust CLI error: {e}", file=sys.stderr)
        Path(output_file).unlink(missing_ok=True)
        return convert_with_ffmpeg(input_file, output_format, quality, 'image')


# ============================================================
# PPO Training Logic
# ============================================================

class PPOExperience:
    """Store PPO training experience."""

    def __init__(self, output_file: str = "ppo_experiences.json"):
        self.output_file = output_file
        self.experiences = []

    def add(self, state: Dict, action: Dict, reward: float, next_state: Dict):
        """Add experience to buffer."""
        self.experiences.append({
            'state': state,
            'action': action,
            'reward': reward,
            'next_state': next_state,
            'timestamp': datetime.now().isoformat()
        })

    def save(self):
        """Save experiences to file."""
        with open(self.output_file, 'w') as f:
            json.dump(self.experiences, f, indent=2)
        print(f"💾 Saved {len(self.experiences)} experiences to {self.output_file}")


def calculate_reward(result: Dict, target_ratio: float = 0.5) -> float:
    """Calculate reward based on conversion result."""
    if not result or not result.get('success'):
        return -1.0

    ratio = result['compression_ratio']

    # Reward for good compression (closer to target ratio)
    ratio_diff = abs(ratio - target_ratio)
    ratio_reward = max(0, 1 - ratio_diff * 2)

    # Bonus for significant compression
    if ratio < 0.5:
        ratio_reward += 0.3

    # Penalty for expansion
    if ratio > 1.0:
        ratio_reward -= 0.5

    return max(-1.0, min(1.0, ratio_reward))


def run_training(
    data_dir: str,
    mode: str,
    num_episodes: int = 100,
    media_types: Optional[List[str]] = None
):
    """Run PPO training."""
    print(f"🚀 Starting PPO training")
    print(f"   Mode: {mode}")
    print(f"   Episodes: {num_episodes}")
    print(f"   Data dir: {data_dir}")

    # Find media files
    media_files = find_media_files(data_dir)

    if media_types is None:
        media_types = ['image', 'video', 'audio']

    # Filter by requested types
    all_files = []
    for mtype in media_types:
        key = mtype + 's' if not mtype.endswith('s') else mtype
        if key == 'images':
            all_files.extend([(f, 'image') for f in media_files['images']])
        elif key == 'videos':
            all_files.extend([(f, 'video') for f in media_files['videos']])
        elif key == 'audio':
            all_files.extend([(f, 'audio') for f in media_files['audio']])

    if not all_files:
        print("❌ No media files found", file=sys.stderr)
        return

    print(f"📊 Found {len(all_files)} media files")

    # Initialize experience buffer
    experience = PPOExperience(f"ppo_experiences_{mode}_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json")

    # Training loop
    for episode in range(num_episodes):
        # Random sample
        file_path, media_type = random.choice(all_files)
        target_format = random.choice(get_target_formats(media_type))
        quality = random.randint(60, 95)

        print(f"\n📍 Episode {episode + 1}/{num_episodes}")
        print(f"   File: {Path(file_path).name}")
        print(f"   Type: {media_type} -> {target_format}")
        print(f"   Quality: {quality}")

        # State
        state = {
            'file_size': Path(file_path).stat().st_size,
            'media_type': media_type,
            'target_format': target_format
        }

        # Action
        action = {'quality': quality, 'format': target_format}

        # Execute conversion based on mode
        if mode == 'ffmpeg':
            result = convert_with_ffmpeg(file_path, target_format, quality, media_type)
        elif mode == 'hybrid':
            if media_type == 'image':
                result = convert_with_rust_cli(file_path, target_format, quality)
            else:
                result = convert_with_ffmpeg(file_path, target_format, quality, media_type)
        else:  # 'all' or default
            if media_type == 'image':
                result = convert_with_rust_cli(file_path, target_format, quality)
            else:
                result = convert_with_ffmpeg(file_path, target_format, quality, media_type)

        # Calculate reward
        reward = calculate_reward(result)

        # Next state
        next_state = {
            **state,
            'compression_ratio': result['compression_ratio'] if result else 1.0
        }

        # Store experience
        experience.add(state, action, reward, next_state)

        if result:
            print(f"   ✅ Ratio: {result['compression_ratio']:.3f}, Reward: {reward:.3f}")
        else:
            print(f"   ❌ Conversion failed, Reward: {reward:.3f}")

    # Save experiences
    experience.save()
    print(f"\n🎉 Training completed!")


# ============================================================
# Main Entry Point
# ============================================================

def main():
    parser = argparse.ArgumentParser(description='Unified PPO Training Script')
    parser.add_argument('--mode', choices=['all', 'ffmpeg', 'hybrid', 'image', 'video', 'audio'],
                        default='all', help='Training mode')
    parser.add_argument('--data-dir', default='./data', help='Data directory')
    parser.add_argument('--episodes', type=int, default=100, help='Number of training episodes')

    args = parser.parse_args()

    # Check dependencies
    if not check_dependencies(args.mode):
        sys.exit(1)

    # Determine media types
    media_types = None
    if args.mode == 'image':
        media_types = ['image']
    elif args.mode == 'video':
        media_types = ['video']
    elif args.mode == 'audio':
        media_types = ['audio']

    # Run training
    run_training(
        data_dir=args.data_dir,
        mode=args.mode if args.mode in ['ffmpeg', 'hybrid', 'all'] else 'all',
        num_episodes=args.episodes,
        media_types=media_types
    )


if __name__ == '__main__':
    main()
