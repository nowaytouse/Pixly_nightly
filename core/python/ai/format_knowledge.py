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
    ),
    
    "heic": FormatKnowledge(
        name="HEIC",
        full_name="High Efficiency Image Container",
        year=2015,
        category="modern",
        strengths=[
            "优秀的压缩效率（比JPEG小40-50%）",
            "Apple生态默认格式",
            "支持多图层和动画",
            "16-bit色深支持",
            "支持透明度"
        ],
        weaknesses=[
            "Windows/Android兼容性差",
            "需要专利授权",
            "浏览器不支持",
            "转换工具有限"
        ],
        use_cases=[
            "iPhone/iPad照片",
            "Apple设备间分享",
            "需要高效存储的场景"
        ],
        tech_specs={
            "max_resolution": "8,192 × 8,192 (typical)",
            "color_depth": "8/10/12/16-bit",
            "alpha_support": "yes",
            "animation": "yes",
            "metadata": "Exif, XMP"
        },
        compression_type="lossy",
        quality_range=QualityRange(min=50, max=100, default=85, lossless=False),
        recommended_for=["apple_ecosystem", "mobile", "storage"]
    ),
    
    "gif": FormatKnowledge(
        name="GIF",
        full_name="Graphics Interchange Format",
        year=1987,
        category="legacy",
        strengths=[
            "动画支持（广泛兼容）",
            "极简的文件结构",
            "通用支持（100%）",
            "适合简单动画",
            "支持透明度"
        ],
        weaknesses=[
            "仅256色限制",
            "文件体积大（动画）",
            "不适合照片",
            "压缩效率极低"
        ],
        use_cases=[
            "简单动画",
            "表情包",
            "Logo动画",
            "网页小动画"
        ],
        tech_specs={
            "max_resolution": "65,535 × 65,535",
            "color_depth": "1-8 bit (256 colors max)",
            "alpha_support": "yes (1-bit)",
            "animation": "yes",
            "metadata": "comments"
        },
        compression_type="lossless",
        quality_range=QualityRange(min=0, max=0, default=0, lossless=True, lossless_q_value=0),
        recommended_for=["animation", "legacy", "simple_graphics"]
    )
}


# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 高级评分系统 - 顶级价值功能 ⚡⚡⚡⚡⚡
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

# 格式评分矩阵 (0-100分)
FORMAT_SCORES = {
    "compression": {
        "jxl": 95, "avif": 98, "webp": 80, "heic": 85,
        "jpeg": 60, "png": 40, "gif": 20
    },
    "quality": {
        "jxl": 95, "avif": 90, "webp": 75, "heic": 85,
        "jpeg": 70, "png": 100, "gif": 30
    },
    "compatibility": {
        "jxl": 40, "avif": 75, "webp": 90, "heic": 50,
        "jpeg": 100, "png": 100, "gif": 100
    },
    "speed": {
        "jxl": 85, "avif": 50, "webp": 90, "heic": 70,
        "jpeg": 95, "png": 80, "gif": 95
    },
    "file_size": {
        "jxl": 95, "avif": 98, "webp": 85, "heic": 88,
        "jpeg": 75, "png": 30, "gif": 10
    }
}

# 场景权重配置
SCENARIO_WEIGHTS = {
    "web": {"compression": 0.3, "quality": 0.2, "compatibility": 0.4, "speed": 0.1},
    "archival": {"compression": 0.2, "quality": 0.6, "compatibility": 0.1, "speed": 0.1},
    "social_media": {"compression": 0.4, "quality": 0.2, "compatibility": 0.3, "speed": 0.1},
    "print": {"compression": 0.1, "quality": 0.8, "compatibility": 0.05, "speed": 0.05},
    "mobile": {"compression": 0.35, "quality": 0.25, "compatibility": 0.25, "speed": 0.15},
    "photography": {"compression": 0.2, "quality": 0.7, "compatibility": 0.05, "speed": 0.05}
}

class FormatKnowledgeDB:
    """格式知识库管理器 - 智能分析与推荐系统"""
    
    def __init__(self):
        """初始化知识库"""
        self.knowledge = FORMAT_KNOWLEDGE_BASE
        self.scores = FORMAT_SCORES
        self.scenario_weights = SCENARIO_WEIGHTS
    
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
    
    def calculate_format_score(self, format_name: str, scenario: str = "web") -> float:
        """
        计算格式在特定场景下的综合评分
        
        Args:
            format_name: 格式名称
            scenario: 使用场景
            
        Returns:
            综合评分 (0-100)
        """
        if format_name.lower() not in self.knowledge:
            return 0.0
        
        weights = self.scenario_weights.get(scenario, self.scenario_weights["web"])
        total_score = 0.0
        
        for aspect, weight in weights.items():
            aspect_score = self.scores.get(aspect, {}).get(format_name.lower(), 50)
            total_score += aspect_score * weight
        
        return round(total_score, 1)
    
    def get_recommended_quality(self, format_name: str, scenario: str) -> int:
        """
        根据格式和场景推荐质量参数
        
        Args:
            format_name: 格式名称  
            scenario: 使用场景
            
        Returns:
            推荐的质量值
        """
        knowledge = self.get(format_name)
        if not knowledge or not knowledge.quality_range:
            return 85  # 默认值
        
        base_quality = knowledge.quality_range.default
        
        # 场景调整
        adjustments = {
            "web": -5,          # 网络传输，稍低质量
            "archival": +10,    # 存档，最高质量
            "social_media": -10, # 社交媒体，更激进压缩
            "print": +5,       # 打印，高质量
            "mobile": -3,      # 移动设备，平衡
            "photography": +8   # 摄影，高质量
        }
        
        adjustment = adjustments.get(scenario, 0)
        result = base_quality + adjustment
        
        # 限制在有效范围内
        if knowledge.quality_range.min <= result <= knowledge.quality_range.max:
            return result
        else:
            return max(knowledge.quality_range.min, 
                      min(knowledge.quality_range.max, result))
    
    def intelligent_format_selection(self, 
                                   requirements: Dict[str, any],
                                   scenario: str = "web") -> List[Tuple[str, float, str]]:
        """
        智能格式选择算法
        
        Args:
            requirements: 需求字典 {
                'transparency': bool,  # 是否需要透明度
                'animation': bool,     # 是否需要动画
                'compatibility': str,  # 兼容性要求 (high/medium/low)
                'quality_priority': str, # 质量优先级 (high/medium/low)
                'size_priority': str,  # 文件大小优先级 (high/medium/low)
                'modern_only': bool    # 是否只考虑现代格式
            }
            scenario: 使用场景
            
        Returns:
            List[Tuple[format_name, score, reason]]: 按评分排序的格式推荐
        """
        candidates = []
        
        for name, knowledge in self.knowledge.items():
            # 过滤不符合要求的格式
            if requirements.get('modern_only', False) and knowledge.category != "modern":
                continue
            
            # 检查透明度支持
            if requirements.get('transparency', False):
                if knowledge.tech_specs.get('alpha_support', 'no') == 'no':
                    continue
            
            # 检查动画支持
            if requirements.get('animation', False):
                if knowledge.tech_specs.get('animation', 'no') == 'no':
                    continue
            
            # 计算基础分数
            base_score = self.calculate_format_score(name, scenario)
            
            # 应用优先级调整
            priority_bonus = 0
            
            # 兼容性优先级调整
            compatibility_req = requirements.get('compatibility', 'medium')
            compat_score = self.scores['compatibility'][name]
            if compatibility_req == 'high' and compat_score >= 90:
                priority_bonus += 10
            elif compatibility_req == 'high' and compat_score < 70:
                priority_bonus -= 20
            
            # 质量优先级调整
            quality_req = requirements.get('quality_priority', 'medium')
            quality_score = self.scores['quality'][name]
            if quality_req == 'high' and quality_score >= 90:
                priority_bonus += 15
            elif quality_req == 'low' and quality_score < 60:
                priority_bonus -= 10
            
            # 文件大小优先级调整
            size_req = requirements.get('size_priority', 'medium')
            size_score = self.scores['compression'][name]
            if size_req == 'high' and size_score >= 90:
                priority_bonus += 15
            elif size_req == 'low' and size_score < 50:
                priority_bonus -= 5
            
            final_score = base_score + priority_bonus
            
            # 生成推荐原因
            reason_parts = []
            if knowledge.category == "modern":
                reason_parts.append("现代格式")
            if compat_score >= 90:
                reason_parts.append("兼容性优秀")
            if quality_score >= 90:
                reason_parts.append("质量卓越")
            if size_score >= 90:
                reason_parts.append("压缩效率高")
            
            reason = ", ".join(reason_parts) if reason_parts else "基础推荐"
            
            candidates.append((name.upper(), round(final_score, 1), reason))
        
        # 按分数排序
        candidates.sort(key=lambda x: x[1], reverse=True)
        
        return candidates
    
    def get_conversion_recommendation(self, 
                                    source_format: str, 
                                    target_scenario: str = "web") -> Dict[str, any]:
        """
        获取格式转换建议
        
        Args:
            source_format: 源格式
            target_scenario: 目标场景
            
        Returns:
            转换建议字典
        """
        source_knowledge = self.get(source_format)
        if not source_knowledge:
            return {"error": f"不支持的源格式: {source_format}"}
        
        # 分析源格式特点
        has_alpha = source_knowledge.tech_specs.get('alpha_support', 'no') == 'yes'
        has_animation = source_knowledge.tech_specs.get('animation', 'no') == 'yes'
        is_modern = source_knowledge.category == "modern"
        
        # 构建需求
        requirements = {
            'transparency': has_alpha,
            'animation': has_animation,
            'compatibility': 'high' if target_scenario == 'web' else 'medium',
            'quality_priority': 'high' if target_scenario in ['photography', 'archival'] else 'medium',
            'size_priority': 'high' if target_scenario in ['web', 'mobile', 'social_media'] else 'medium',
            'modern_only': False
        }
        
        recommendations = self.intelligent_format_selection(requirements, target_scenario)
        
        # 过滤掉源格式本身
        filtered_recs = [rec for rec in recommendations if rec[0].lower() != source_format.lower()]
        
        return {
            "source_format": source_format.upper(),
            "source_category": source_knowledge.category,
            "target_scenario": target_scenario,
            "recommendations": filtered_recs[:5],  # 前5个推荐
            "best_choice": filtered_recs[0] if filtered_recs else None,
            "analysis": {
                "has_transparency": has_alpha,
                "has_animation": has_animation,
                "is_modern_format": is_modern,
                "source_score": self.calculate_format_score(source_format, target_scenario)
            }
        }
    
    def _determine_winner(self, k1: FormatKnowledge, k2: FormatKnowledge) -> str:
        """
        判断哪个格式更好（基于综合评分）
        
        Args:
            k1: 格式知识1
            k2: 格式知识2
            
        Returns:
            获胜格式名称
        """
        score1 = self.calculate_format_score(k1.name.lower(), "web")
        score2 = self.calculate_format_score(k2.name.lower(), "web")
        
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
    
    # 4. 格式评分测试
    print("\n4️⃣  格式综合评分（Web场景）:")
    for name in ["jxl", "avif", "webp", "jpeg"]:
        score = db.calculate_format_score(name, "web")
        print(f"   {name.upper():4s}: {score:5.1f}分")
    
    # 5. 智能格式选择测试
    print("\n5️⃣  智能格式选择（需要透明度+高兼容性）:")
    requirements = {
        'transparency': True,
        'compatibility': 'high',
        'quality_priority': 'medium',
        'modern_only': False
    }
    
    selections = db.intelligent_format_selection(requirements, "web")
    for i, (fmt, score, reason) in enumerate(selections[:3]):
        print(f"   {i+1}. {fmt} ({score}分) - {reason}")
    
    # 6. 转换建议测试
    print("\n6️⃣  JPEG→Web场景转换建议:")
    conversion = db.get_conversion_recommendation("jpeg", "web")
    if conversion.get("best_choice"):
        best = conversion["best_choice"]
        print(f"   最佳选择: {best[0]} ({best[1]}分) - {best[2]}")
        print(f"   源格式评分: {conversion['analysis']['source_score']:.1f}分")
    
    # 7. 质量参数推荐
    print("\n7️⃣  质量参数推荐:")
    for scenario in ["web", "archival", "social_media"]:
        quality = db.get_recommended_quality("jxl", scenario)
        print(f"   JXL {scenario:12s}: Q={quality}")
    
    # 8. 显示所有格式
    print("\n8️⃣  支持的格式 (按现代性排序):")
    formats_sorted = sorted(db.knowledge.items(), key=lambda x: (x[1].category == "modern", x[1].year), reverse=True)
    for name, knowledge in formats_sorted:
        icon = "🆕" if knowledge.category == "modern" else "📜"
        score = db.calculate_format_score(name, "web")
        print(f"   {icon} {knowledge.name:4s} - {knowledge.full_name} ({knowledge.year}) [Web评分: {score:4.1f}]")
    
    print("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print("✅ 格式知识库系统测试完成 - 顶级价值功能 ⚡⚡⚡⚡⚡")
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
