#!/bin/bash

# 🔍 完整参数验证脚本
# 验证前端传递的所有CLI参数在Rust后端是否都有实现

echo "=========================================="
echo "🔍 Pixly 参数完整性验证"
echo "=========================================="
echo ""

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

TOTAL=0
IMPLEMENTED=0
MISSING=0

check_param() {
    local param=$1
    local rust_pattern=$2
    local description=$3
    
    TOTAL=$((TOTAL + 1))
    
    # 在Rust代码中搜索参数
    if grep -rq "$rust_pattern" src/ --include="*.rs"; then
        echo -e "${GREEN}✅${NC} $param - $description"
        IMPLEMENTED=$((IMPLEMENTED + 1))
    else
        echo -e "${RED}❌${NC} $param - $description ${RED}[空壳功能]${NC}"
        MISSING=$((MISSING + 1))
    fi
}

echo "📋 基础参数"
echo "----------------------------------------"
check_param "--quality" "quality.*u8\|quality.*u32" "质量参数"
check_param "--format" "format.*String\|target_format" "目标格式"
echo ""

echo "📋 JXL参数"
echo "----------------------------------------"
check_param "--effort" "jxl_effort\|effort.*Option" "JXL努力度"
check_param "--distance" "jxl_distance\|distance.*Option" "JXL距离"
check_param "--lossless" "lossless.*bool" "无损模式"
check_param "--jpeg-lossless" "jpeg_lossless\|jxl_jpeg_lossless" "JPEG无损转码"
check_param "--modular" "jxl_modular\|modular.*bool" "JXL模块化模式"
check_param "--progressive" "jxl_progressive\|progressive.*bool" "JXL渐进式"
check_param "--bit-depth" "bit_depth\|bitDepth" "位深度"
check_param "--color-space" "color_space\|colorSpace" "色彩空间"
echo ""

echo "📋 AVIF参数"
echo "----------------------------------------"
check_param "--speed" "avif.*speed\|speed.*Option" "AVIF速度"
check_param "--min-quantizer" "min_quantizer\|avif_min_quantizer" "AVIF最小量化器"
check_param "--max-quantizer" "max_quantizer\|avif_max_quantizer" "AVIF最大量化器"
check_param "--chroma" "avif.*chroma\|chroma.*Option" "AVIF色度采样"
check_param "--tiles" "avif.*tiles\|tiles.*Option" "AVIF分块"
echo ""

echo "📋 WebP参数"
echo "----------------------------------------"
check_param "--method" "webp_method\|method.*Option" "WebP方法"
check_param "--filter-strength" "filter_strength\|webp_filter_strength" "WebP滤镜强度"
check_param "--sharpness" "webp.*sharpness\|sharpness.*Option" "WebP锐化"
echo ""

echo "📋 HEIC参数"
echo "----------------------------------------"
check_param "--encoder" "heic_encoder\|encoder.*Option" "HEIC编码器"
check_param "--thumbnail" "heic_thumbnail\|thumbnail.*bool" "HEIC缩略图"
echo ""

echo "📋 快捷工具参数"
echo "----------------------------------------"
check_param "--merge-xmp" "merge_xmp\|xmp.*sidecar" "XMP合并"
check_param "--xmp-path" "xmp_path\|xmpPath" "XMP路径"
check_param "--normalize-filenames" "normalize_filenames\|normalizeFilenames" "文件名规范化"
check_param "--validate-file-type" "validate.*file.*type\|fileValidation" "文件类型验证"
check_param "--auto-correct-format" "auto.*correct.*format\|formatCorrection" "格式自动修正"
echo ""

echo "📋 AI智能参数"
echo "----------------------------------------"
check_param "--smart-quality" "smart_quality\|smartQuality" "智能质量预测"
check_param "--auto-optimize" "auto_optimize\|autoOptimize" "自动参数优化"
check_param "--ssim-validation" "ssim_validation\|ssimValidation" "SSIM质量验证"
check_param "--video-for-animation" "video.*animation\|videoForAnimation" "动图转视频推荐"
check_param "--smart-preprocess" "smart_preprocess\|smartPreprocess" "智能预处理"
echo ""

echo "📋 视频参数"
echo "----------------------------------------"
check_param "--crf" "crf.*u8\|crf.*u32\|crf.*Option" "CRF质量"
check_param "--gop" "gop.*u32\|gop.*Option" "GOP大小"
check_param "--bframes" "bframes.*u32\|bframes.*Option" "B帧数量"
check_param "--refs" "refs.*u32\|refs.*Option" "参考帧"
check_param "--rate-control" "rate_control\|rateControl" "码率控制"
check_param "--me-method" "me_method\|meMethod" "运动估计方法"
check_param "--pix-fmt" "pix_fmt\|pixFmt" "像素格式"
echo ""

echo "=========================================="
echo "📊 验证结果汇总"
echo "=========================================="
echo -e "总参数数: ${TOTAL}"
echo -e "${GREEN}已实现: ${IMPLEMENTED}${NC}"
echo -e "${RED}缺失: ${MISSING}${NC}"
echo ""

if [ $MISSING -eq 0 ]; then
    echo -e "${GREEN}🎉 所有参数都已实现！${NC}"
    exit 0
else
    PERCENTAGE=$(echo "scale=1; $MISSING * 100 / $TOTAL" | bc)
    echo -e "${RED}⚠️  发现 ${MISSING} 个空壳功能 (${PERCENTAGE}%)${NC}"
    echo ""
    echo "🔥 违反质量宣言："
    echo "   - 真实性原则：代码未真正实现声称的功能"
    echo "   - 反对摆设代码：UI控件存在但无实际功能"
    echo "   - 响亮的错误：参数被静默忽略"
    echo ""
    echo "📝 建议行动："
    echo "   1. 查看详细报告: docs/FRONTEND_BACKEND_GAP_INVESTIGATION.md"
    echo "   2. 选择修复方案: 实现功能 / 移除UI / 添加警告"
    echo "   3. 更新测试脚本"
    exit 1
fi
