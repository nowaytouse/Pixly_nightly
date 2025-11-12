#!/usr/bin/env python3
"""
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 47.23: 格式知识库系统
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

功能：
- 所有现代格式完整知识（JXL/AVIF/WebP等）
- 格式特性、优缺点、适用场景
- 技术规格和质量范围
- 推荐使用场景

从Go代码提取：core/@deprecated/go_ai_service_2025_11_11/ai 2/format_knowledge.go
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"""

from dataclasses import dataclass, field
from typing import List, Dict, Optional


@dataclass
class QualityRange:
    """质量范围配置"""
    min: int
    max: int
    default: int
    lossless: bool
    lossless_q_value: int = 100


@dataclass
class FormatKnowledge:
    """格式知识数据结构"""
    name: str
    full_name: str
    year: int
    category: str  # modern, legacy, video
    
    strengths: List[str] = field(default_factory=list)
    weaknesses: List[str] = field(default_factory=list)
    use_cases: List[str] = field(default_factory=list)
    tech_specs: Dict[str, str] = field(default_factory=dict)
    
    compression_type: str = "lossy"  # lossy, lossless, hybrid
    quality_range: Optional[QualityRange] = None
    recommended_for: List[str] = field(default_factory=list)


# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 格式知识库
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

FORMAT_KNOWLEDGE_BASE: Dict[str, FormatKnowledge] = {
    "jxl": FormatKnowledge(
        name="JXL",
        full_name="JPEG XL",
        year=2021,
        category="modern",
        strengths=[
            "最佳的压缩效率（比JPEG小30-60%）",
            "支持无损压缩",
            "支持渐进式解码",
            "支持动画和多层",
            "HDR和广色域支持",
            "优秀的有损质量",
            "快速编解码",
            "可从JPEG无损转换"
        ],
        weaknesses=[
            "浏览器支持有限（Safari 17+, Firefox需手动启用）",
            "生态系统还在发展中",
            "部分老旧设备不支持"
        ],
        use_cases=[
            "网络图片分发（高质量+小体积）",
            "相册存档（无损压缩）",
            "HDR摄影作品",
            "需要渐进式加载的场景",
            "JPEG替代方案"
        ],
        tech_specs={
            "max_resolution": "1,073,741,823 × 1,073,741,823",
            "color_depth": "32-bit per channel",
            "alpha_support": "yes",
            "animation": "yes",
            "metadata": "Exif, XMP, JUMBF"
        },
        compression_type="hybrid",
        quality_range=QualityRange(min=60, max=100, default=90, lossless=True, lossless_q_value=100),
        recommended_for=["photography", "web", "archival", "hdr"]
    ),
    
    "avif": FormatKnowledge(
        name="AVIF",
        full_name="AV1 Image File Format",
        year=2019,
        category="modern",
        strengths=[
            "卓越的压缩率（比JPEG小50%+）",
            "优秀的细节保留",
            "广泛的浏览器支持",
            "HDR和广色域",
            "10-bit色深支持",
            "渐进式解码"
        ],
        weaknesses=[
            "编码速度慢（CPU密集）",
            "解码性能一般",
            "工具链不够成熟",
            "在某些场景可能出现色带"
        ],
        use_cases=[
            "现代网站（Chrome, Firefox, Safari支持）",
            "移动应用",
            "社交媒体图片",
            "需要最小文件体积的场景"
        ],
        tech_specs={
            "max_resolution": "65,536 × 65,536",
            "color_depth": "8/10/12-bit",
            "alpha_support": "yes",
            "animation": "yes",
            "metadata": "Exif, XMP"
        },
        compression_type="lossy",
        quality_range=QualityRange(min=20, max=100, default=75, lossless=False),
        recommended_for=["web", "mobile", "social"]
    ),
    
    "webp": FormatKnowledge(
        name="WebP",
        full_name="WebP",
        year=2010,
        category="modern",
        strengths=[
            "广泛的浏览器支持（95%+）",
            "良好的压缩效率",
            "同时支持有损和无损",
            "支持透明度",
            "支持动画",
            "编解码速度快"
        ],
        weaknesses=[
            "质量略逊于JXL和AVIF",
            "某些老旧浏览器不支持",
            "编辑工具支持不如JPEG/PNG"
        ],
        use_cases=[
            "网站图片（最佳兼容性）",
            "图片CDN",
            "电商产品图",
            "Web应用界面"
        ],
        tech_specs={
            "max_resolution": "16,383 × 16,383",
            "color_depth": "8-bit",
            "alpha_support": "yes",
            "animation": "yes",
            "metadata": "Exif, XMP, ICCP"
        },
        compression_type="hybrid",
        quality_range=QualityRange(min=0, max=100, default=80, lossless=True, lossless_q_value=100),
        recommended_for=["web", "cdn", "ecommerce"]
    ),
    
    "png": FormatKnowledge(
        name="PNG",
        full_name="Portable Network Graphics",
        year=1996,
        category="legacy",
        strengths=[
            "完全无损压缩",
            "完美的透明度支持",
            "广泛的软件支持",
            "简单可靠",
            "适合图形和图表"
        ],
        weaknesses=[
            "文件体积大（相比现代格式）",
            "不支持动画（除APNG）",
            "不适合照片",
            "压缩效率低"
        ],
        use_cases=[
            "需要透明度的图形",
            "图标和UI元素",
            "图表和截图",
            "需要无损质量的场景"
        ],
        tech_specs={
            "max_resolution": "2,147,483,647 × 2,147,483,647",
            "color_depth": "1/2/4/8/16-bit",
            "alpha_support": "yes",
            "animation": "no (APNG yes)",
            "metadata": "text chunks, Exif, XMP"
        },
        compression_type="lossless",
        quality_range=QualityRange(min=0, max=9, default=6, lossless=True, lossless_q_value=0),
        recommended_for=["graphics", "ui", "icons", "screenshots"]
    ),
    
    "jpeg": FormatKnowledge(
        name="JPEG",
        full_name="Joint Photographic Experts Group",
        year=1992,
        category="legacy",
        strengths=[
            "极广泛的支持（100%）",
            "成熟稳定的标准",
            "良好的照片压缩",
            "小文件体积",
            "快速编解码"
        ],
        weaknesses=[
            "有损压缩（质量损失）",
            "不支持透明度",
            "不适合图形和文字",
            "多次编辑会累积失真"
        ],
        use_cases=[
            "照片存储",
            "网络图片（传统）",
            "相机输出",
            "需要最大兼容性的场景"
        ],
        tech_specs={
            "max_resolution": "65,535 × 65,535",
            "color_depth": "8-bit",
            "alpha_support": "no",
            "animation": "no",
            "metadata": "Exif, IPTC, XMP"
        },
        compression_type="lossy",
        quality_range=QualityRange(min=1, max=100, default=85, lossless=False),
        recommended_for=["photography", "compatibility"]
    )
}


class FormatKnowledgeDB:
    """格式知识库管理器"""
    
    def __init__(self):
        """初始化知识库"""
        self.knowledge = FORMAT_KNOWLEDGE_BASE
    
    def get(self, format_name: str) -> Optional[FormatKnowledge]:
        """
        获取格式知识
        
        Args:
            format_name: 格式名称（如jxl, avif, webp等）
            
        Returns:
            格式知识对象，不存在返回None
        """
        return self.knowledge.get(format_name.lower())
    
    def recommend_format(self, use_case: str, modern_only: bool = True) -> List[str]:
        """
        根据使用场景推荐格式
        
        Args:
            use_case: 使用场景（如web, photography, archival等）
            modern_only: 是否只推荐现代格式
            
        Returns:
            推荐的格式列表
        """
        recommendations = []
        
        for name, knowledge in self.knowledge.items():
            # 过滤类别
            if modern_only and knowledge.category != "modern":
                continue
            
            # 检查是否推荐用于该场景
            if use_case in knowledge.recommended_for or use_case in [uc.lower() for uc in knowledge.use_cases]:
                recommendations.append(name)
        
        return recommendations
    
    def compare_formats(self, format1: str, format2: str) -> Dict[str, any]:
        """
        比较两种格式
        
        Args:
            format1: 格式1
            format2: 格式2
            
        Returns:
            比较结果字典
        """
        k1 = self.get(format1)
        k2 = self.get(format2)
        
        if not k1 or not k2:
            return {"error": "格式不存在"}
        
        return {
            "format1": k1.name,
            "format2": k2.name,
            "compression": {
                format1: k1.compression_type,
                format2: k2.compression_type
            },
            "year_diff": k1.year - k2.year,
            "both_modern": k1.category == "modern" and k2.category == "modern",
            "winner": self._determine_winner(k1, k2)
        }
    
    def _determine_winner(self, k1: FormatKnowledge, k2: FormatKnowledge) -> str:
        """
        简单判断哪个格式更好（基于现代性和特性）
        
        Args:
            k1: 格式知识1
            k2: 格式知识2
            
        Returns:
            获胜格式名称
        """
        score1 = len(k1.strengths) - len(k1.weaknesses)
        score2 = len(k2.strengths) - len(k2.weaknesses)
        
        if k1.category == "modern" and k2.category != "modern":
            score1 += 5
        elif k2.category == "modern" and k1.category != "modern":
            score2 += 5
        
        return k1.name if score1 > score2 else k2.name


# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 测试和演示
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

if __name__ == "__main__":
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print("Phase 47.23: 格式知识库系统测试")
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
    
    db = FormatKnowledgeDB()
    
    # 1. 查询格式知识
    print("1️⃣  查询JXL格式知识...")
    jxl = db.get("jxl")
    if jxl:
        print(f"   名称: {jxl.full_name} ({jxl.name})")
        print(f"   年份: {jxl.year}")
        print(f"   类别: {jxl.category}")
        print(f"   压缩类型: {jxl.compression_type}")
        print(f"   优点: {len(jxl.strengths)} 个")
        print(f"   缺点: {len(jxl.weaknesses)} 个")
        print(f"   适用场景: {', '.join(jxl.use_cases[:3])}")
    
    # 2. 推荐格式
    print("\n2️⃣  根据使用场景推荐格式...")
    web_formats = db.recommend_format("web", modern_only=True)
    print(f"   Web场景推荐: {', '.join(web_formats).upper()}")
    
    photo_formats = db.recommend_format("photography", modern_only=False)
    print(f"   摄影场景推荐: {', '.join(photo_formats).upper()}")
    
    # 3. 比较格式
    print("\n3️⃣  比较JXL vs WebP...")
    comparison = db.compare_formats("jxl", "webp")
    print(f"   格式1: {comparison['format1']}")
    print(f"   格式2: {comparison['format2']}")
    print(f"   年份差: {comparison['year_diff']} 年")
    print(f"   都是现代格式: {comparison['both_modern']}")
    print(f"   推荐: {comparison['winner']}")
    
    # 4. 显示所有格式
    print("\n4️⃣  所有支持的格式:")
    for name, knowledge in db.knowledge.items():
        icon = "🆕" if knowledge.category == "modern" else "📜"
        print(f"   {icon} {knowledge.name:4s} - {knowledge.full_name} ({knowledge.year})")
    
    print("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print("✅ 格式知识库系统测试完成")
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
