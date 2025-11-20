#!/usr/bin/env python3
"""
格式知识库 - Python端
用于ML训练的完整格式知识
包含图像、视频、音频格式的全面知识
"""

from dataclasses import dataclass
from typing import List, Dict, Optional
from enum import Enum


class Speed(Enum):
    VERY_SLOW = 1
    SLOW = 2
    MEDIUM = 3
    FAST = 4
    VERY_FAST = 5


class MediaType(Enum):
    IMAGE = "image"
    VIDEO = "video"
    AUDIO = "audio"


@dataclass
class FormatKnowledge:
    """完整的格式知识"""
    name: str
    full_name: str
    year_released: int
    media_type: MediaType
    
    # Capabilities
    supports_alpha: bool
    supports_animation: bool
    supports_lossless: bool
    supports_lossy: bool
    supports_hdr: bool
    max_dimensions: int
    bit_depth_max: int
    
    # Performance
    encode_speed: Speed
    decode_speed: Speed
    compression_ratio: float  # vs JPEG
    quality_retention: float  # 0-1
    
    # Browser support
    browser_support_percentage: float
    
    # Efficiency
    compression_efficiency: float  # 0-1
    
    # Use cases
    best_for: List[str]
    strengths: List[str]
    weaknesses: List[str]


# 格式知识库 - 2025年最新数据
# 包含图像、视频、音频三大类格式
FORMAT_KNOWLEDGE = {
    # ========== 图像格式 ==========
    "avif": FormatKnowledge(
        name="avif",
        full_name="AV1 Image File Format",
        year_released=2019,
        media_type=MediaType.IMAGE,
        supports_alpha=True,
        supports_animation=True,
        supports_lossless=True,
        supports_lossy=True,
        supports_hdr=True,
        max_dimensions=65536,
        bit_depth_max=12,
        encode_speed=Speed.SLOW,
        decode_speed=Speed.MEDIUM,
        compression_ratio=0.5,  # 50% of JPEG
        quality_retention=0.98,
        browser_support_percentage=85.0,
        compression_efficiency=0.95,
        best_for=["web_photography", "archival", "hdr_content"],
        strengths=[
            "Best compression efficiency",
            "HDR support",
            "Wide color gamut",
            "Royalty-free",
            "Growing browser support"
        ],
        weaknesses=[
            "Slow encoding",
            "Not universal yet (85%)"
        ]
    ),
    
    "jxl": FormatKnowledge(
        name="jxl",
        full_name="JPEG XL",
        year_released=2021,
        media_type=MediaType.IMAGE,
        supports_alpha=True,
        supports_animation=True,
        supports_lossless=True,
        supports_lossy=True,
        supports_hdr=True,
        max_dimensions=1073741824,
        bit_depth_max=32,
        encode_speed=Speed.MEDIUM,
        decode_speed=Speed.FAST,
        compression_ratio=0.45,  # 45% of JPEG
        quality_retention=0.99,
        browser_support_percentage=5.0,
        compression_efficiency=0.98,
        best_for=["archival", "printing", "scientific", "professional"],
        strengths=[
            "Best quality retention",
            "Fastest decode",
            "Progressive decoding",
            "Lossless JPEG recompression",
            "32-bit float support",
            "Best for high-res images"
        ],
        weaknesses=[
            "Very limited browser support",
            "Requires external tools",
            "Not web-ready yet"
        ]
    ),
    
    "webp": FormatKnowledge(
        name="webp",
        full_name="WebP",
        year_released=2010,
        media_type=MediaType.IMAGE,
        supports_alpha=True,
        supports_animation=True,
        supports_lossless=True,
        supports_lossy=True,
        supports_hdr=False,
        max_dimensions=16383,
        bit_depth_max=8,
        encode_speed=Speed.FAST,
        decode_speed=Speed.FAST,
        compression_ratio=0.7,  # 70% of JPEG
        quality_retention=0.95,
        browser_support_percentage=97.0,
        compression_efficiency=0.85,
        best_for=["web", "general_purpose", "animation"],
        strengths=[
            "Excellent browser support (97%)",
            "Fast encode/decode",
            "Good compression",
            "Animation support",
            "Mature ecosystem"
        ],
        weaknesses=[
            "No HDR",
            "8-bit only",
            "Not best for archival"
        ]
    ),
    
    "png": FormatKnowledge(
        name="png",
        full_name="Portable Network Graphics",
        year_released=1996,
        media_type=MediaType.IMAGE,
        supports_alpha=True,
        supports_animation=False,
        supports_lossless=True,
        supports_lossy=False,
        supports_hdr=False,
        max_dimensions=2147483647,
        bit_depth_max=16,
        encode_speed=Speed.FAST,
        decode_speed=Speed.VERY_FAST,
        compression_ratio=1.0,  # baseline
        quality_retention=1.0,
        browser_support_percentage=100.0,
        compression_efficiency=0.5,
        best_for=["graphics", "screenshots", "transparency"],
        strengths=[
            "Universal support",
            "Lossless",
            "Simple",
            "16-bit support"
        ],
        weaknesses=[
            "Large file sizes",
            "No animation",
            "Outdated compression"
        ]
    ),
    
    "jpeg": FormatKnowledge(
        name="jpeg",
        full_name="Joint Photographic Experts Group",
        year_released=1992,
        media_type=MediaType.IMAGE,
        supports_alpha=False,
        supports_animation=False,
        supports_lossless=False,
        supports_lossy=True,
        supports_hdr=False,
        max_dimensions=65535,
        bit_depth_max=8,
        encode_speed=Speed.VERY_FAST,
        decode_speed=Speed.VERY_FAST,
        compression_ratio=1.0,  # baseline
        quality_retention=0.90,
        browser_support_percentage=100.0,
        compression_efficiency=0.6,
        best_for=["photography", "compatibility"],
        strengths=[
            "Universal support",
            "Very fast",
            "Hardware acceleration",
            "Mature"
        ],
        weaknesses=[
            "No transparency",
            "Lossy only",
            "Compression artifacts",
            "Outdated"
        ]
    ),
    
    # ========== 视频格式 ==========
    "h266": FormatKnowledge(
        name="h266",
        full_name="Versatile Video Coding (VVC)",
        year_released=2020,
        media_type=MediaType.VIDEO,
        supports_alpha=True,
        supports_animation=True,
        supports_lossless=False,
        supports_lossy=True,
        supports_hdr=True,
        max_dimensions=16384,
        bit_depth_max=12,
        encode_speed=Speed.VERY_SLOW,
        decode_speed=Speed.SLOW,
        compression_ratio=0.35,  # 35% of H.264 - 最佳压缩
        quality_retention=0.99,
        browser_support_percentage=5.0,
        compression_efficiency=0.99,
        best_for=["archival", "ultra_high_quality", "future_proof"],
        strengths=[
            "Best compression efficiency (30-50% better than H.265)",
            "Highest quality retention",
            "HDR and wide color gamut",
            "Future-proof technology"
        ],
        weaknesses=[
            "Very slow encoding",
            "Limited hardware support",
            "New standard, limited adoption"
        ]
    ),
    
    "h265": FormatKnowledge(
        name="h265",
        full_name="High Efficiency Video Coding (HEVC)",
        year_released=2013,
        media_type=MediaType.VIDEO,
        supports_alpha=True,
        supports_animation=True,
        supports_lossless=False,
        supports_lossy=True,
        supports_hdr=True,
        max_dimensions=8192,
        bit_depth_max=10,
        encode_speed=Speed.SLOW,
        decode_speed=Speed.MEDIUM,
        compression_ratio=0.5,
        quality_retention=0.96,
        browser_support_percentage=75.0,
        compression_efficiency=0.92,
        best_for=["4k_video", "hdr_video", "streaming"],
        strengths=[
            "50% better compression than H.264",
            "HDR support",
            "4K/8K ready",
            "Wide device support"
        ],
        weaknesses=[
            "Slow encoding",
            "Patent licensing issues"
        ]
    ),
    
    "av1": FormatKnowledge(
        name="av1",
        full_name="AOMedia Video 1",
        year_released=2018,
        media_type=MediaType.VIDEO,
        supports_alpha=True,
        supports_animation=True,
        supports_lossless=False,
        supports_lossy=True,
        supports_hdr=True,
        max_dimensions=65536,
        bit_depth_max=12,
        encode_speed=Speed.VERY_SLOW,
        decode_speed=Speed.MEDIUM,
        compression_ratio=0.4,
        quality_retention=0.98,
        browser_support_percentage=70.0,
        compression_efficiency=0.96,
        best_for=["web_video", "streaming", "archival"],
        strengths=[
            "Best compression efficiency",
            "Royalty-free",
            "HDR and wide color gamut",
            "Growing browser support"
        ],
        weaknesses=[
            "Very slow encoding",
            "Limited hardware support"
        ]
    ),
    
    "vp9": FormatKnowledge(
        name="vp9",
        full_name="VP9",
        year_released=2013,
        media_type=MediaType.VIDEO,
        supports_alpha=True,
        supports_animation=True,
        supports_lossless=False,
        supports_lossy=True,
        supports_hdr=True,
        max_dimensions=16384,
        bit_depth_max=12,
        encode_speed=Speed.SLOW,
        decode_speed=Speed.MEDIUM,
        compression_ratio=0.55,
        quality_retention=0.95,
        browser_support_percentage=95.0,
        compression_efficiency=0.88,
        best_for=["web_video", "youtube"],
        strengths=[
            "Royalty-free",
            "Good compression",
            "Wide browser support"
        ],
        weaknesses=[
            "Slower than H.265",
            "Being replaced by AV1"
        ]
    ),
    
    "h264": FormatKnowledge(
        name="h264",
        full_name="Advanced Video Coding (AVC) - LEGACY",
        year_released=2003,
        media_type=MediaType.VIDEO,
        supports_alpha=False,
        supports_animation=True,
        supports_lossless=False,
        supports_lossy=True,
        supports_hdr=False,
        max_dimensions=4096,
        bit_depth_max=8,
        encode_speed=Speed.FAST,
        decode_speed=Speed.VERY_FAST,
        compression_ratio=1.0,
        quality_retention=0.92,
        browser_support_percentage=100.0,
        compression_efficiency=0.50,  # 降低效率分数，不推荐使用
        best_for=[],  # 不推荐用于任何场景
        strengths=[
            "Universal support (100%)",
            "Hardware acceleration everywhere",
            "Fast encode/decode"
        ],
        weaknesses=[
            "OUTDATED - Use H.266/AV1/H.265 instead",
            "Poor compression efficiency",
            "No HDR support",
            "No modern features",
            "Patent licensing issues"
        ]
    ),
    
    # ========== 音频格式 ==========
    "opus": FormatKnowledge(
        name="opus",
        full_name="Opus Interactive Audio Codec",
        year_released=2012,
        media_type=MediaType.AUDIO,
        supports_alpha=False,
        supports_animation=False,
        supports_lossless=False,
        supports_lossy=True,
        supports_hdr=False,
        max_dimensions=0,
        bit_depth_max=24,
        encode_speed=Speed.FAST,
        decode_speed=Speed.VERY_FAST,
        compression_ratio=0.6,
        quality_retention=0.98,
        browser_support_percentage=97.0,
        compression_efficiency=0.95,
        best_for=["web_audio", "voip", "streaming"],
        strengths=[
            "Best audio quality at low bitrates",
            "Royalty-free",
            "Low latency",
            "Wide bitrate range (6-510 kbps)"
        ],
        weaknesses=[
            "Limited hardware support",
            "Not supported in MP4 container"
        ]
    ),
    
    "aac": FormatKnowledge(
        name="aac",
        full_name="Advanced Audio Coding",
        year_released=1997,
        media_type=MediaType.AUDIO,
        supports_alpha=False,
        supports_animation=False,
        supports_lossless=False,
        supports_lossy=True,
        supports_hdr=False,
        max_dimensions=0,
        bit_depth_max=24,
        encode_speed=Speed.FAST,
        decode_speed=Speed.VERY_FAST,
        compression_ratio=1.0,
        quality_retention=0.95,
        browser_support_percentage=100.0,
        compression_efficiency=0.85,
        best_for=["general_audio", "mp4_video"],
        strengths=[
            "Universal support",
            "Hardware acceleration",
            "Good quality",
            "MP4 compatible"
        ],
        weaknesses=[
            "Patent licensing",
            "Not as efficient as Opus"
        ]
    ),
    
    "mp3": FormatKnowledge(
        name="mp3",
        full_name="MPEG-1 Audio Layer III",
        year_released=1993,
        media_type=MediaType.AUDIO,
        supports_alpha=False,
        supports_animation=False,
        supports_lossless=False,
        supports_lossy=True,
        supports_hdr=False,
        max_dimensions=0,
        bit_depth_max=16,
        encode_speed=Speed.VERY_FAST,
        decode_speed=Speed.VERY_FAST,
        compression_ratio=1.2,
        quality_retention=0.88,
        browser_support_percentage=100.0,
        compression_efficiency=0.65,
        best_for=["compatibility"],
        strengths=[
            "Universal support (100%)",
            "Very fast",
            "Simple"
        ],
        weaknesses=[
            "Outdated compression",
            "Lower quality than modern codecs"
        ]
    ),
    
    "flac": FormatKnowledge(
        name="flac",
        full_name="Free Lossless Audio Codec",
        year_released=2001,
        media_type=MediaType.AUDIO,
        supports_alpha=False,
        supports_animation=False,
        supports_lossless=True,
        supports_lossy=False,
        supports_hdr=False,
        max_dimensions=0,
        bit_depth_max=32,
        encode_speed=Speed.FAST,
        decode_speed=Speed.FAST,
        compression_ratio=0.5,
        quality_retention=1.0,
        browser_support_percentage=85.0,
        compression_efficiency=0.75,
        best_for=["archival", "high_res_audio"],
        strengths=[
            "Lossless compression",
            "Royalty-free",
            "Fast encode/decode",
            "High-res audio support"
        ],
        weaknesses=[
            "Large file sizes",
            "Limited browser support"
        ]
    ),
}


def get_format_knowledge(format_name: str) -> Optional[FormatKnowledge]:
    """获取格式知识"""
    return FORMAT_KNOWLEDGE.get(format_name.lower())


def get_format_features_for_ml(format_name: str) -> List[float]:
    """
    将格式知识转换为ML特征向量 (32维)
    用于训练时的格式特征 - 完整喂给ML模型
    让ML完全理解每种格式的所有特性！
    """
    knowledge = get_format_knowledge(format_name)
    if not knowledge:
        return [0.0] * 32
    
    # 媒体类型编码
    media_type_value = {
        MediaType.IMAGE: 0.0,
        MediaType.VIDEO: 0.5,
        MediaType.AUDIO: 1.0,
    }[knowledge.media_type]
    
    return [
        # 基础能力 (8维)
        float(knowledge.supports_alpha),
        float(knowledge.supports_animation),
        float(knowledge.supports_lossless),
        float(knowledge.supports_lossy),
        float(knowledge.supports_hdr),
        knowledge.bit_depth_max / 32.0,
        (knowledge.max_dimensions ** 0.5) / 1000.0 if knowledge.max_dimensions > 0 else 0.0,
        media_type_value,
        
        # 性能特征 (8维)
        knowledge.encode_speed.value / 5.0,
        knowledge.decode_speed.value / 5.0,
        knowledge.compression_ratio,
        knowledge.quality_retention,
        knowledge.compression_efficiency,
        knowledge.browser_support_percentage / 100.0,
        (knowledge.year_released - 1990) / 35.0,  # 归一化年份
        1.0,  # Chrome支持 (简化)
        
        # 兼容性和生态 (8维)
        1.0,  # Firefox支持 (简化)
        1.0,  # Safari支持 (简化)
        1.0,  # Edge支持 (简化)
        len(knowledge.strengths) / 10.0,
        len(knowledge.weaknesses) / 10.0,
        len(knowledge.best_for) / 10.0,
        0.0,  # 保留
        0.0,  # 保留
        
        # 高级特征 (8维)
        1.0 if knowledge.year_released >= 2015 else 0.5,  # 现代性
        1.0 if knowledge.browser_support_percentage >= 95.0 else 0.5,  # 通用性
        knowledge.compression_efficiency * knowledge.quality_retention,  # 效率分数
        (knowledge.encode_speed.value + knowledge.decode_speed.value) / 10.0,  # 速度分数
        0.0,  # 保留
        0.0,  # 保留
        0.0,  # 保留
        0.0,  # 保留
    ]


def should_use_same_format_optimization(
    source_format: str,
    target_format: Optional[str],
    format_conversion_enabled: bool
) -> bool:
    """
    判断是否应该使用同格式优化
    当用户禁用格式转换时，强制使用同格式优化！
    这是ML进行精细优化的关键功能！
    """
    # 如果禁用了格式转换，必须使用同格式优化
    if not format_conversion_enabled:
        return True
    
    # 如果没有指定目标格式，使用同格式优化
    if target_format is None:
        return True
    
    # 如果目标格式与源格式相同，使用同格式优化
    if target_format == source_format:
        return True
    
    return False


def get_formats_by_type(media_type: MediaType) -> List[str]:
    """获取特定媒体类型的所有格式"""
    return [
        name for name, knowledge in FORMAT_KNOWLEDGE.items()
        if knowledge.media_type == media_type
    ]


def suggest_format_upgrade(source_format: str) -> tuple[bool, List[str], str]:
    """
    建议格式升级 - 从旧格式升级到现代格式
    返回: (是否应该升级, 推荐的新格式列表, 升级原因)
    """
    source = get_format_knowledge(source_format)
    if not source:
        return (False, [], "")
    
    # 检查是否是过时格式
    is_outdated = source.year_released < 2010 or source.compression_efficiency < 0.70
    
    if not is_outdated:
        return (False, [], "")
    
    # 根据媒体类型推荐现代格式
    if source.media_type == MediaType.IMAGE:
        if source_format == "jpeg":
            return (
                True,
                ["avif", "jxl", "webp"],
                "JPEG is outdated. Modern formats offer 40-60% better compression with higher quality."
            )
        elif source_format == "png":
            return (
                True,
                ["avif", "jxl", "webp"],
                "PNG has poor compression. Modern formats offer lossless compression with 50-70% smaller files."
            )
    elif source.media_type == MediaType.VIDEO:
        if source_format == "h264":
            return (
                True,
                ["h266", "av1", "h265"],
                "H.264 is OUTDATED. Modern codecs offer 40-60% better compression with HDR support."
            )
    elif source.media_type == MediaType.AUDIO:
        if source_format == "mp3":
            return (
                True,
                ["opus", "aac"],
                "MP3 is outdated. Modern codecs offer 30-40% better compression with higher quality."
            )
    
    return (False, [], "")


def get_modern_format_recommendations(
    media_type: MediaType,
    needs_hdr: bool = False,
    needs_alpha: bool = False
) -> List[str]:
    """
    获取推荐的现代格式（按优先级排序）
    完全基于格式知识，零硬编码规则
    """
    formats = [
        (name, knowledge) for name, knowledge in FORMAT_KNOWLEDGE.items()
        if knowledge.media_type == media_type
        and (not needs_hdr or knowledge.supports_hdr)
        and (not needs_alpha or knowledge.supports_alpha)
    ]
    
    # 按现代性和效率排序
    def score(item):
        name, knowledge = item
        modern_bonus = 1.2 if knowledge.year_released >= 2015 else 1.0
        return knowledge.compression_efficiency * modern_bonus
    
    formats.sort(key=score, reverse=True)
    
    return [name for name, _ in formats]


if __name__ == "__main__":
    print("🎓 Format Knowledge Base - 2025 Edition")
    print("=" * 80)
    print("Complete knowledge for ML training: Images, Videos, Audio")
    print("=" * 80)
    
    # 按媒体类型分组显示
    for media_type in [MediaType.IMAGE, MediaType.VIDEO, MediaType.AUDIO]:
        print(f"\n{'='*80}")
        print(f"📁 {media_type.value.upper()} FORMATS")
        print(f"{'='*80}")
        
        formats = get_formats_by_type(media_type)
        for name in formats:
            knowledge = FORMAT_KNOWLEDGE[name]
            print(f"\n{knowledge.full_name} ({name.upper()})")
            print(f"  📅 Released: {knowledge.year_released}")
            print(f"  📦 Compression: {knowledge.compression_ratio:.0%} of baseline")
            print(f"  ⭐ Quality: {knowledge.quality_retention:.0%}")
            print(f"  🌐 Browser support: {knowledge.browser_support_percentage:.0%}")
            print(f"  🚀 Efficiency score: {knowledge.compression_efficiency:.2f}")
            print(f"  🎯 Best for: {', '.join(knowledge.best_for)}")
            
            # 显示ML特征向量
            ml_features = get_format_features_for_ml(name)
            print(f"  🤖 ML features: {len(ml_features)} dimensions")
    
    print("\n" + "=" * 80)
    print("✅ Format knowledge base ready for ML training!")
    print(f"📊 Total formats: {len(FORMAT_KNOWLEDGE)}")
    print(f"   - Images: {len(get_formats_by_type(MediaType.IMAGE))}")
    print(f"   - Videos: {len(get_formats_by_type(MediaType.VIDEO))}")
    print(f"   - Audio: {len(get_formats_by_type(MediaType.AUDIO))}")
    print("=" * 80)
    
    # 测试同格式优化判断
    print("\n🔧 Testing same-format optimization logic:")
    test_cases = [
        ("png", None, True, "No target format specified"),
        ("png", "png", True, "Same format"),
        ("png", "webp", True, "Different format, conversion enabled"),
        ("png", "webp", False, "Different format, conversion DISABLED"),
    ]
    
    for source, target, enabled, desc in test_cases:
        result = should_use_same_format_optimization(source, target, enabled)
        status = "✅ USE" if result else "❌ SKIP"
        print(f"  {status} same-format optimization: {desc}")
    
    print("=" * 80)
